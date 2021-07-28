use std::{
    collections::{HashMap, HashSet},
    fs::{read, read_to_string, write},
    path::PathBuf,
};

use anyhow::Context;
use clap::clap_app;
use data_encoding::BASE32HEX_NOPAD;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // getting cli args
    let matches = clap_app!(ango =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "Merkle trees showoff")
        (@subcommand add =>
            (about: "Adds file to the tree")
            (@arg FILE: +required "input file")
        )
    )
    .get_matches();

    // getting environment
    let angopath: PathBuf = std::env::var_os("ANGO_PATH")
        .context("ANGO_PATH is not set")?
        .into();

    let angoyml_path = angopath.join("ango.yml");
    let data_path = angopath.join("data");

    // getting ango.yml configs
    let (mut hashmap, hashset) = {
        let filecontents = read_to_string(&angoyml_path).context("failed to read ango.yml")?;
        if filecontents != "" {
            let hashmap: HashMap<String, String> =
                serde_yaml::from_str(&filecontents).context("failed to parse ango.yml")?;
            let hashset: HashSet<String> = hashmap.iter().map(|v| v.1.clone()).collect();
            (hashmap, hashset)
        } else {
            (HashMap::new(), HashSet::new())
        }
    };

    // add subcommand
    if let Some(add) = matches.subcommand_matches("add") {
        // getting file
        let fname = add.value_of("FILE").context("FILE arg was not provided")?;
        let entryname = fname.to_string();
        let contents = read(fname).context("failed to open ./.gitignore")?;
        let hash = BASE32HEX_NOPAD.encode(blake3::hash(&contents).as_bytes());

        // checking for existence
        if !hashset.contains(&hash) {
            hashmap.insert(entryname.clone(), hash.clone());

            // writing file
            write(data_path.join(hash), contents)
                .with_context(|| format!("Failed to write {}", entryname))?;
        } else if !hashmap.contains_key(&entryname) {
            hashmap.insert(entryname.clone(), hash);
        }

        // writing ango.yml
        let hashmap = serde_yaml::to_string(&hashmap).context("failed to encode ango.yml")?;
        write(&angoyml_path, hashmap).context("failed to save ango.yml")?;
    }

    Ok(())
}
