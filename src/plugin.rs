#![deny(warnings)]
#![allow(dead_code)]

use std::fs;
use std::path::Path;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    info: PluginInfo,
    options: Options,
}

#[derive(Debug, Deserialize)]
struct PluginInfo {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct Options {
    log_file_location: String,
}

pub fn parse_plugin_toml() {
    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let path = Path::new("portable").join("manifest.toml");
    let toml: String = fs::read_to_string(&path).unwrap();
    println!("{}", toml);
    let decoded: Config = toml::from_str(&toml).unwrap();
    println!("{:#?}", decoded);
}