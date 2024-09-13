use std::fs;
use std::path::Path;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub info: PluginInfo,
    pub options: Options,
}

#[derive(Debug, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Options {
    pub log_file_location: String,
}

impl Manifest {
    pub fn parse(plugin_path: &Path) -> Self  {
        let path = plugin_path.join("manifest.toml");
        let toml: String = fs::read_to_string(&path).unwrap();
        let plugin: Manifest = toml::from_str(&toml).unwrap();
        return plugin;
    }
}