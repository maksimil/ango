use std::{
    collections::{HashMap, HashSet},
    fs::{metadata, read, write},
    path::PathBuf,
};

use anyhow::Context;
use data_encoding::BASE32HEX_NOPAD;

use crate::heads::{ObjectType, TypedHash};

pub fn add(
    path: &str,
    epname: String,
    hashset: &mut HashSet<String>,
    hashmap: &mut HashMap<String, TypedHash>,
    data_path: &PathBuf,
) -> anyhow::Result<()> {
    let meta = metadata(path).context("failed to open FILE")?;
    if meta.is_file() {
        // getting file
        let contents = read(path).context("failed to open FILE")?;
        let hash = BASE32HEX_NOPAD.encode(blake3::hash(&contents).as_bytes());

        // checking for existence
        if !hashset.contains(&hash) {
            hashmap.insert(
                epname.clone(),
                TypedHash {
                    ty: ObjectType::File,
                    hash: hash.clone(),
                },
            );
            hashset.insert(hash.clone());

            // writing file
            write(data_path.join(hash), contents)
                .with_context(|| format!("failed to write {}", epname))?;
        } else if !hashmap.contains_key(&epname) {
            // adding endpoint, since the file exists
            hashmap.insert(
                epname.clone(),
                TypedHash {
                    ty: ObjectType::File,
                    hash,
                },
            );
        }
        Ok(())
    } else if meta.is_dir() {
        Err(anyhow::anyhow!("failed to open FILE as file"))
    } else {
        Err(anyhow::anyhow!("failed to open FILE as directory or file"))
    }
}
