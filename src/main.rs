use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use anyhow::Context;
use clap::clap_app;

use crate::heads::{de_config, se_config};

mod commands;
mod heads;

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
            (@arg AS: -a --as +takes_value "endpoint name")
        )
    )
    .get_matches();

    // getting environment
    let angopath: PathBuf = std::env::var_os("ANGO_PATH")
        .context("ANGO_PATH is not set")?
        .into();

    let config_path = angopath.join("ango.toml");
    let data_path = angopath.join("data");

    // getting ango.toml configs
    let (mut hashmap, mut hashset) = {
        let filecontents = read_to_string(&config_path).context("failed to read ango.toml")?;
        de_config(&filecontents)?
    };

    // add subcommand
    if let Some(add) = matches.subcommand_matches("add") {
        // getting file
        let fname = add.value_of("FILE").context("FILE arg was not provided")?;
        let epname = add.value_of("AS").unwrap_or(fname).to_string();
        commands::add(fname, epname, &mut hashset, &mut hashmap, &data_path)?;

        // writing ango.toml
        let hashmap = se_config(hashmap, hashset)?;
        write(&config_path, hashmap).context("failed to save ango.toml")?;
    }

    Ok(())
}
