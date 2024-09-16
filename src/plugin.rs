use crate::callback_fields::CallbackField;
use crate::manifest::Manifest;

use std::fs::{self, metadata, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Plugin {
    manifest: Manifest,
    callback_fields: Vec<CallbackField>,
}

impl Plugin {
    pub fn new(plugin_path: &Path) -> Self {
        let manifest = Manifest::parse(plugin_path);
        let callback_fields = CallbackField::list_all(plugin_path);

        Self {
            manifest,
            callback_fields,
        }
    }
}

impl Plugin {
    // plugins are valid if, and _only_ if they are a folder and have a manifest.toml file
    // callback fields are optional
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

    pub fn begin_search(&self) {
        for callback_field in &self.callback_fields {
            let log_file_location = self.manifest.options.log_file_location.clone();
            let log_file_name = callback_field.file_name.clone();
            let regular_expression = callback_field.pattern.clone();

            thread::spawn(move || {
                let file_path = Path::new(&log_file_location).join(&log_file_name);
                let mut position: u64 = 0; // keeps track of the last-read byte for continuing

                loop {
                    let file = File::open(&file_path)
                        .or_else(|_| File::create(&file_path))
                        .expect("Could not open for create file to search!");
                    let mut reader = BufReader::new(file);
                    let _ = reader.seek(SeekFrom::Start(position));
                    position = metadata(&file_path).map_or(0, |meta| meta.len()); // move to end of file

                    let mut lines = reader.lines();
                    while let Some(Ok(line)) = lines.next() {
                        if let Some(captures) = regular_expression.captures(&line) {
                            if let Some(first) = captures.get(1) {
                                println!("{}", first.as_str());
                            }
                        }
                    }
                    thread::sleep(Duration::from_secs(5));
                }
            });
        }
    }
}
