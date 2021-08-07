use anyhow::Context;
use clap::clap_app;

use crate::angofile::{get_context, save_context};

mod angofile;
mod commands;

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

    let mut context = get_context()?;

    // add subcommand
    if let Some(add) = matches.subcommand_matches("add") {
        // getting file
        let fname = add.value_of("FILE").context("FILE arg was not provided")?;
        let epname = add.value_of("AS").unwrap_or(fname).to_string();
        commands::add(fname, epname, &mut context)?;

        save_context(context)?;
    }

    Ok(())
}
