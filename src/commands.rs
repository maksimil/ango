use serde_derive::{Deserialize, Serialize};

use crate::angofile::LinkType;

mod add;

pub use add::add;

#[derive(Deserialize, Serialize)]
struct EntryHash {
    name: String,
    ty: LinkType,
    hash: String,
}

#[derive(Deserialize, Serialize)]
struct EntryList {
    entries: Vec<EntryHash>,
}
