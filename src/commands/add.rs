use std::{
    fs::{metadata, read, read_dir, write},
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Context;
use data_encoding::BASE32HEX_NOPAD;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    angofile::{AngoContext, TypedHash},
    commands::{EntryHash, EntryList},
};

pub fn add(path: &str, epname: String, context: Arc<Mutex<AngoContext>>) -> anyhow::Result<()> {
    let hash = add_pathed(path, context.clone())?;

    match add_link(epname.clone(), hash, context.clone())? {
        LinkResult::AlreadyExists => {
            let context = context
                .lock()
                .map_err(|_| anyhow::anyhow!("failed to lock context"))?;
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
    context: Arc<Mutex<AngoContext>>,
) -> anyhow::Result<LinkResult> {
    let mut context = context
        .lock()
        .map_err(|_| anyhow::anyhow!("failed to lock context"))?;
    if !context.links.contains_key(&epname) {
        context.links.insert(epname, hash);
        Ok(LinkResult::Added)
    } else {
        Ok(LinkResult::AlreadyExists)
    }
}

// returns true if the hash was not in the set
fn add_object(contents: &[u8], context: Arc<Mutex<AngoContext>>) -> anyhow::Result<String> {
    let hash = BASE32HEX_NOPAD.encode(blake3::hash(&contents).as_bytes());

    let mut context = context
        .lock()
        .map_err(|_| anyhow::anyhow!("failed to lock context"))?;
    // checking for existence
    if !context.objects.contains(&hash) {
        context.objects.insert(hash.clone());

        // writing the file
        write(context.data_path().join(&hash), contents)
            .with_context(|| format!("failed to write {}", hash))?;
    }

    Ok(hash)
}

fn add_pathed<P>(path: P, context: Arc<Mutex<AngoContext>>) -> anyhow::Result<TypedHash>
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

        Ok(TypedHash::file(hash))
    } else if meta.is_dir() {
        // getting the dir data
        let entries = read_dir(path)
            .with_context(|| format!("failed to read {} dir", path.to_string_lossy()))?
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|entry| {
                let entry = entry
                    .with_context(|| {
                        format!(
                            "failed to read one of {} dir entries",
                            path.to_string_lossy()
                        )
                    })?
                    .path();

                let name = entry
                    .strip_prefix(path)
                    .with_context(|| {
                        format!(
                            "failed to get relative path of {} from {}",
                            entry.to_string_lossy(),
                            path.to_string_lossy()
                        )
                    })?
                    .to_string_lossy()
                    .into_owned();

                let hash = add_pathed(&entry, context.clone())
                    .with_context(|| format!("failed to add {}", entry.to_string_lossy()))?;

                Ok(EntryHash {
                    ty: hash.ty,
                    name,
                    hash: hash.hash,
                }) as anyhow::Result<EntryHash>
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("failed to hash {} dir entries", path.to_string_lossy()))?;

        let entrylist = toml::to_string(&EntryList { entries })
            .with_context(|| format!("failed to serialize {} entries", path.to_string_lossy()))?;

        let hash = add_object(entrylist.as_bytes(), context).with_context(|| {
            format!("failed to add {} entrylist object", path.to_string_lossy())
        })?;

        Ok(TypedHash::folder(hash))
    } else {
        Err(anyhow::anyhow!("failed to open FILE as directory or file"))
    }
}
