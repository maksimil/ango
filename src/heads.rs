use std::collections::{HashMap, HashSet};

use anyhow::Context;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Object {
    tag: Option<String>,
    hash: String,
}

#[derive(Deserialize, Serialize)]
struct AngoFile {
    object: Option<Vec<Object>>,
}

pub fn de_config(contents: &str) -> anyhow::Result<(HashMap<String, String>, HashSet<String>)> {
    let file = toml::from_str::<AngoFile>(contents).context("failed to deserialize ango.toml")?;
    match file.object {
        Some(objects) => {
            let hashmap = objects
                .iter()
                .filter_map(|object| match &object.tag {
                    Some(tag) => Some((tag.clone(), object.hash.clone())),
                    None => None,
                })
                .collect();
            let hashset = objects.into_iter().map(|object| object.hash).collect();
            Ok((hashmap, hashset))
        }
        None => Ok((HashMap::new(), HashSet::new())),
    }
}

pub fn se_config(hashmap: HashMap<String, String>, set: HashSet<String>) -> anyhow::Result<String> {
    let mut object = set
        .into_iter()
        .map(|hash| (hash.clone(), Object { tag: None, hash }))
        .collect::<HashMap<String, Object>>();
    for (tag, hash) in hashmap {
        if let Some(object) = object.get_mut(&hash) {
            object.tag = Some(tag);
        } else {
            print!(
                "\x1b[33m[WARN]\x1b[0m Tag {} does point to a hash that is not in the hashset",
                tag
            )
        }
    }

    let object = Some(object.into_iter().map(|(_, object)| object).collect());

    toml::to_string(&AngoFile { object }).context("Failed to serialize ango.toml")
}
