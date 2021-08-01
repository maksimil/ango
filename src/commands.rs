use std::{
    collections::{HashMap, HashSet},
    fs::{read, write},
    path::PathBuf,
};

use anyhow::Context;
use data_encoding::BASE32HEX_NOPAD;

pub fn add(
    fname: &str,
    epname: String,
    hashset: &mut HashSet<String>,
    hashmap: &mut HashMap<String, String>,
    data_path: &PathBuf,
) -> anyhow::Result<()> {
    // getting file
    let contents = read(fname).context("failed to open FILE")?;
    let hash = BASE32HEX_NOPAD.encode(blake3::hash(&contents).as_bytes());

    // checking for existence
    if !hashset.contains(&hash) {
        hashmap.insert(epname.clone(), hash.clone());
        hashset.insert(hash.clone());

        // writing file
        write(data_path.join(hash), contents)
            .with_context(|| format!("Failed to write {}", epname))?;
    } else if !hashmap.contains_key(&epname) {
        hashmap.insert(epname.clone(), hash);
    }

    Ok(())
}
