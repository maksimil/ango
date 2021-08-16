use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, write},
    path::PathBuf,
};

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

#[derive(Deserialize, Serialize)]
pub struct TypedHash {
    pub ty: LinkType,
    pub hash: String,
}

impl TypedHash {
    pub fn file(hash: String) -> TypedHash {
        TypedHash {
            hash,
            ty: LinkType::File,
        }
    }

    pub fn folder(hash: String) -> TypedHash {
        TypedHash {
            hash,
            ty: LinkType::Folder,
        }
    }
}

pub struct AngoContext {
    pub objects: HashSet<String>,
    pub links: HashMap<String, TypedHash>,
    pub ango_path: PathBuf,
}

impl AngoContext {
    pub fn data_path(&self) -> PathBuf {
        self.ango_path.join("data")
    }
}

pub fn get_context() -> anyhow::Result<AngoContext> {
    // getting environment
    let ango_path: PathBuf = std::env::var_os("ANGO_PATH")
        .context("ANGO_PATH is not set")?
        .into();

    let contents =
        read_to_string(&ango_path.join("ango.toml")).context("failed to read ango.toml")?;

    let AngoFile { objects, links } =
        toml::from_str::<AngoFile>(&contents).context("failed to deserialize ango.toml")?;

    let links = links
        .into_iter()
        .map(|Link { name, ty, hash }| (name, TypedHash { ty, hash }))
        .collect();

    Ok(AngoContext {
        objects,
        links,
        ango_path,
    })
}

pub fn save_context(context: AngoContext) -> anyhow::Result<()> {
    let AngoContext {
        links,
        objects,
        ango_path,
    } = context;

    let links = links
        .into_iter()
        .map(|(name, TypedHash { ty, hash })| Link { name, ty, hash })
        .collect();

    let contents =
        toml::to_string(&AngoFile { links, objects }).context("failed to serialize ango.toml")?;
    write(ango_path.join("ango.toml"), contents).context("failed to save ango.toml")?;

    Ok(())
}
