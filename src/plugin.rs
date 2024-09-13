use crate::manifest::Manifest;
use crate::callback_fields::{self, CallbackField};

use std::fs;
use std::path::Path;

pub struct Plugin {
    manifest: Manifest,
    callback_fields: Vec<CallbackField>,
}

impl Plugin {
    pub fn new(plugin_path: &Path) -> Self {
        let manifest = Manifest::parse(plugin_path);
        let callback_fields = callback_fields::list_all(plugin_path);

        Self { manifest, callback_fields }
    } 
}

// plugins are valid if, and _only_ if they are a folder and have a manifest.toml file
pub fn get_all_plugins() -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = vec![];
    let path = Path::new("plugins");

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() && entry.path().join("manifest.toml").exists() {
                    plugins.push(Plugin::new(&entry.path()));
                }
            }
        }
    }

    return plugins;
}