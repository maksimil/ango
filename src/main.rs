use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    path::PathBuf,
};

use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let angopath: PathBuf = std::env::var_os("ANGO_PATH")
        .context("ANGO_PATH is not set")?
        .into();

    let angoyml_path = angopath.join("ango.yml");
    let data_path = angopath.join("data");

    // getting ango.yml configs
    let (hashmap, hashset) = {
        let filecontents = read_to_string(angoyml_path).context("Was not able to read ango.yml")?;
        if filecontents != "" {
            let hashmap: HashMap<String, String> =
                serde_yaml::from_str(&filecontents).context("failed to parse ango.yml")?;
            let hashset: HashSet<String> = hashmap.iter().map(|v| v.1.clone()).collect();
            (hashmap, hashset)
        } else {
            (HashMap::new(), HashSet::new())
        }
    };

    dbg!(data_path, hashmap, hashset);

    Ok(())
}
