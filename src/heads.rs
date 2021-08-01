use std::collections::{HashMap, HashSet};

use anyhow::Context;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum ObjectType {
    File,
    Folder,
    Chunk,
}

#[derive(Deserialize, Serialize)]
struct Object {
    ty: ObjectType,
    tags: Vec<String>,
    hash: String,
}

#[derive(Clone)]
pub struct TypedHash {
    pub ty: ObjectType,
    pub hash: String,
}

#[derive(Deserialize, Serialize)]
struct AngoFile {
    object: Option<Vec<Object>>,
}

pub fn de_config(contents: &str) -> anyhow::Result<(HashMap<String, TypedHash>, HashSet<String>)> {
    let file = toml::from_str::<AngoFile>(contents).context("failed to deserialize ango.toml")?;
    match file.object {
        Some(objects) => {
            let hashmap = {
                let mut hashmap = HashMap::new();
                for object in objects.iter() {
                    for tag in object.tags.iter() {
                        hashmap.insert(
                            tag.clone(),
                            TypedHash {
                                hash: object.hash.clone(),
                                ty: object.ty.clone(),
                            },
                        );
                    }
                }
                hashmap
            };
            let hashset = objects.into_iter().map(|object| object.hash).collect();
            Ok((hashmap, hashset))
        }
        None => Ok((HashMap::new(), HashSet::new())),
    }
}

pub fn se_config(
    hashmap: HashMap<String, TypedHash>,
    set: HashSet<String>,
) -> anyhow::Result<String> {
    let mut object = set
        .into_iter()
        .map(|hash| {
            (
                hash.clone(),
                Object {
                    tags: vec![],
                    hash,
                    ty: ObjectType::Chunk,
                },
            )
        })
        .collect::<HashMap<String, Object>>();
    for (tag, hash) in hashmap {
        if let Some(object) = object.get_mut(&hash.hash) {
            object.tags.push(tag);
            object.ty = hash.ty;
        } else {
            print!(
                "\x1b[33m[WARN]\x1b[0m Tag {} does point to a hash that is not in the hashset",
                tag
            );
        }
    }

    let object = Some(object.into_iter().map(|(_, object)| object).collect());

    toml::to_string(&AngoFile { object }).context("Failed to serialize ango.toml")
}
