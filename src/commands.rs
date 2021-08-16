use std::{
    fs::{metadata, read, read_dir, write},
    path::Path,
};

use anyhow::Context;
use data_encoding::BASE32HEX_NOPAD;

use crate::angofile::{AngoContext, LinkType, TypedHash};

pub fn add(path: &str, epname: String, context: &mut AngoContext) -> anyhow::Result<()> {
    let hash = add_pathed(path, context)?;

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

fn add_pathed<P>(path: P, context: &mut AngoContext) -> anyhow::Result<TypedHash>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let meta = metadata(path)
        .with_context(|| format!("failed to get {} metadata", path.to_string_lossy()))?;

    if meta.is_file() {
        // getting file
        let contents =
            read(path).with_context(|| format!("failed to open {}", path.to_string_lossy()))?;

        let hash = add_object(&contents, context)
            .with_context(|| format!("failed to add {} as an object", path.to_string_lossy()))?;

        Ok(TypedHash {
            hash,
            ty: LinkType::File,
        })
    } else if meta.is_dir() {
        // getting the dir data
        let entries = read_dir(path)
            .with_context(|| format!("failed to read {} dir", path.to_string_lossy()))?;

        let mut entrylist = Vec::new();

        for entry in entries {
            let entry = entry
                .with_context(|| {
                    format!(
                        "failed to read one of {} dir entries",
                        path.to_string_lossy()
                    )
                })?
                .path();

            let epname = entry.strip_prefix(path).with_context(|| {
                format!(
                    "failed to get relative path of {} from {}",
                    entry.to_string_lossy(),
                    path.to_string_lossy()
                )
            })?;

            let hash = add_pathed(&entry, context)
                .with_context(|| format!("failed to add {}", entry.to_string_lossy()))?;

            entrylist.push((epname.to_owned(), hash));
        }

        let entrylist = toml::to_string(&entrylist)
            .with_context(|| format!("failed to serialize {} entries", path.to_string_lossy()))?;

        let hash = add_object(entrylist.as_bytes(), context).with_context(|| {
            format!("failed to add {} entrylist object", path.to_string_lossy())
        })?;

        Ok(TypedHash {
            hash,
            ty: LinkType::Folder,
        })
    } else {
        Err(anyhow::anyhow!("failed to open FILE as directory or file"))
    }
}
