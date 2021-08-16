use std::{
    fs::{metadata, read, write},
    path::PathBuf,
};

use anyhow::Context;
use data_encoding::BASE32HEX_NOPAD;

use crate::angofile::{AngoContext, LinkType, TypedHash};

pub fn add(path: &str, epname: String, context: &mut AngoContext) -> anyhow::Result<()> {
    let hash = add_pathed(path.into(), context)?;

    match add_link(epname.clone(), hash, context)? {
        LinkResult::AlreadyExists => {
            println!(
                "Link {} already exists to {}",
                epname,
                context.links.get(&epname).unwrap().hash
            );
        }
        _ => (),
    }

    Ok(())
}

enum LinkResult {
    Added,
    AlreadyExists,
}

fn add_link(
    epname: String,
    hash: TypedHash,
    context: &mut AngoContext,
) -> anyhow::Result<LinkResult> {
    if !context.links.contains_key(&epname) {
        context.links.insert(epname, hash);
        Ok(LinkResult::Added)
    } else {
        Ok(LinkResult::AlreadyExists)
    }
}

// returns true if the hash was not in the set
fn add_object(contents: &[u8], context: &mut AngoContext) -> anyhow::Result<String> {
    let hash = BASE32HEX_NOPAD.encode(blake3::hash(&contents).as_bytes());

    // checking for existence
    if !context.objects.contains(&hash) {
        context.objects.insert(hash.clone());

        // writing the file
        write(context.data_path().join(&hash), contents)
            .with_context(|| format!("failed to write {}", hash))?;
    }

    Ok(hash)
}

fn add_pathed(path: PathBuf, context: &mut AngoContext) -> anyhow::Result<TypedHash> {
    let meta = metadata(&path)
        .with_context(|| format!("failed to get {} metadata", path.to_string_lossy()))?;

    if meta.is_file() {
        // getting file
        let contents =
            read(&path).with_context(|| format!("failed to open {}", path.to_string_lossy()))?;

        let hash = add_object(&contents, context)
            .with_context(|| format!("failed to add {} as an object", path.to_string_lossy()))?;

        Ok(TypedHash {
            hash,
            ty: LinkType::File,
        })
    } else if meta.is_dir() {
        Err(anyhow::anyhow!("failed to open FILE as file"))
    } else {
        Err(anyhow::anyhow!("failed to open FILE as directory or file"))
    }
}
