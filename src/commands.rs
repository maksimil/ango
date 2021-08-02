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
        let hash = BASE32HEX_NOPAD.encode(blake3::hash(&contents).as_bytes());

        // checking for existence
        if !context.objects.contains(&hash) {
            context.links.insert(
                epname.clone(),
                TypedHash {
                    ty: LinkType::File,
                    hash: hash.clone(),
                },
            );
            context.objects.insert(hash.clone());

            // writing file
            write(data_path.join(hash), contents)
                .with_context(|| format!("failed to write {}", epname))?;
        } else if !context.links.contains_key(&epname) {
            // adding endpoint, since the file exists
            context.links.insert(
                epname.clone(),
                TypedHash {
                    ty: LinkType::File,
                    hash,
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
