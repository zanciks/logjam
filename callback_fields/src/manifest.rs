use std::io::Read;

use serde_json;

pub struct Manifest {
    pub title: String,
    pub description: String,
}

impl Default for Manifest {
    fn default() -> Manifest {
        Manifest {
            title: String::new(),
            description: String::new(),
        }
    }
}

impl Manifest {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}

impl Manifest {
    pub fn load_manifests() -> Result<Vec<Manifest>, Box<dyn std::error::Error>> {
        let mut manifests: Vec<Manifest> = vec![];
        let file_path = std::env::current_exe()?.parent().unwrap().join("manifest.json");
        
        let mut contents = String::new();
        let mut file = std::fs::File::open(&file_path)?;
        file.read_to_string(&mut contents)?;

        let json_manifests: Vec<serde_json::Value> = serde_json::from_str(&contents)
            .unwrap_or_default();

        for field in &json_manifests {
            if let Some(obj) = field.as_object() {
                let mut manifest = Manifest::default();
                if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
                    manifest.title = title.to_string();
                }
                if let Some(description) = obj.get("description").and_then(|v| v.as_str()) {
                    manifest.description = description.to_string();
                }
                manifests.push(manifest);
            }
        }

        return Ok(manifests);
    }
}