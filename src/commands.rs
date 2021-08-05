use std::{
    fs::{metadata, read, write},
    path::PathBuf,
};

use anyhow::Context;
use data_encoding::BASE32HEX_NOPAD;

use crate::angofile::{AngoContext, LinkType, TypedHash};

pub fn add(
    path: &str,
    epname: String,
    context: &mut AngoContext,
    data_path: &PathBuf,
) -> anyhow::Result<()> {
    let meta = metadata(path).context("failed to open FILE")?;
    if meta.is_file() {
        // getting file
        let contents = read(path).context("failed to open FILE")?;

        let (hash, added) = add_object(&contents, context, data_path)?;
        if added || !context.links.contains_key(&epname) {
            context.links.insert(
                epname.clone(),
                TypedHash {
                    ty: LinkType::File,
                    hash: hash.clone(),
                },
            );
        } else {
            println!(
                "\x1b[33m[WARN]\x1b[0m Link {} already exists to {}",
                epname, hash
            );
        }
        Ok(())
    } else if meta.is_dir() {
        Err(anyhow::anyhow!("failed to open FILE as file"))
    } else {
        Err(anyhow::anyhow!("failed to open FILE as directory or file"))
    }
}

// returns true if the hash was not in the set
fn add_object(
    contents: &[u8],
    context: &mut AngoContext,
    data_path: &PathBuf,
) -> anyhow::Result<(String, bool)> {
    let hash = BASE32HEX_NOPAD.encode(blake3::hash(&contents).as_bytes());

    // checking for existence
    if !context.objects.contains(&hash) {
        context.objects.insert(hash.clone());

        // writing the file
        write(data_path.join(&hash), contents)
            .with_context(|| format!("failed to write {}", hash))?;
        Ok((hash, true))
    } else {
        Ok((hash, false))
    }
}
