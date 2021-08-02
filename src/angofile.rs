use std::collections::{HashMap, HashSet};

use anyhow::Context;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct AngoFile {
    objects: HashSet<String>,
    links: Vec<Link>,
}

#[derive(Deserialize, Serialize)]
struct Link {
    name: String,
    ty: LinkType,
    hash: String,
}

#[derive(Deserialize, Serialize)]
pub enum LinkType {
    File,
    Folder,
    Chunk,
}

pub struct TypedHash {
    pub ty: LinkType,
    pub hash: String,
}

pub struct AngoContext {
    pub objects: HashSet<String>,
    pub links: HashMap<String, TypedHash>,
}

pub fn de_config(data: &str) -> anyhow::Result<AngoContext> {
    let AngoFile { objects, links } =
        toml::from_str::<AngoFile>(data).context("failed to deserialize ango.toml")?;

    let links = links
        .into_iter()
        .map(|Link { name, ty, hash }| (name, TypedHash { ty, hash }))
        .collect();

    Ok(AngoContext { objects, links })
}

pub fn se_config(AngoContext { objects, links }: AngoContext) -> anyhow::Result<String> {
    let links = links
        .into_iter()
        .map(|(name, TypedHash { ty, hash })| Link { name, ty, hash })
        .collect();
    Ok(toml::to_string(&AngoFile { links, objects }).context("failed to serialize ango.toml")?)
}
