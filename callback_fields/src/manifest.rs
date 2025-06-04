use regex::{Captures, Regex};
use serde_json;
use std::io::Read;

use crate::callback_field::CallbackField;

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
        let file_path = std::env::current_exe()?
            .parent()
            .unwrap()
            .join("manifest.json");

        let mut contents = String::new();
        let mut file = std::fs::File::open(&file_path)?;
        file.read_to_string(&mut contents)?;

        let json_manifests: Vec<serde_json::Value> =
            serde_json::from_str(&contents).unwrap_or_default();

        for field in &json_manifests {
            if let Some(obj) = field.as_object() {
                let mut manifest = Manifest::default();
                if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
                    manifest.title = title.to_string();
                }
                if let Some(description) = obj.get("description").and_then(|v| v.as_array()) {
                    for item in description {
                        if let Some(text) = item.get("text").and_then(|v| v.as_str()) {
                            manifest.description.push_str(text);
                        }
                    }
                }
                log::info!("Manifest loaded: {}", manifest.title);
                manifests.push(manifest);
            }
        }

        return Ok(manifests);
    }
    pub fn to_plain_text(&self, callback_fields: &Vec<CallbackField>) -> String {
        let mut text = insert_callback_results(&self.description, callback_fields);

        let patterns = [
            r"\*(\S(?:.*?\S)?)\*", // remove bolds
            r"\_(\S(?:.*?\S)?)\_", // remove italics
            r"\# (.*)",            // remove numbered lists
            r"\* (.*)",            // remove unordered lists
        ];
        for pattern in patterns {
            let re = Regex::new(pattern).unwrap();
            text = re
                .replace_all(&text, |caps: &Captures| format!("{}", caps[1].to_string()))
                .to_string();
        }

        return text;
    }
    pub fn to_hansoft(&self, callback_fields: &Vec<CallbackField>) -> String {
        let mut text = self.description.to_owned();

        text = Regex::new(r"(?m)((?:^\# .+\n?)+)")
            .unwrap()
            .replace_all(&text, "<OL>$1</OL>")
            .into();

        text = Regex::new(r"(?m)((?:^\* .+\n?)+)")
            .unwrap()
            .replace_all(&text, "<UL>$1</UL>")
            .into();

        let replacements = [
            (r"(?m)\*([^ ].*?)\*", "<BOLD>$1</BOLD>"), // bold tags
            (r"(?m)\# (.*? \n)", "<LI>$1</LI>"),       // used for numbered lists
            (r"(?m)\* (.*? \n)", "<LI>$1</LI>"),       // bullet points
        ];

        for (pattern, replacement) in replacements {
            let re = Regex::new(pattern).unwrap();
            text = re.replace_all(&text, replacement).to_string();
        }

        text = insert_callback_results(&text, callback_fields);

        return text;
    }
}

pub fn insert_callback_results(text: &str, callback_fields: &Vec<CallbackField>) -> String {
    Regex::new(r"\{(.*?)\}")
        .unwrap()
        .replace_all(text, |caps: &Captures| {
            if let Some(matching_field) =
                callback_fields.iter().find(|&field| field.name == caps[1])
            {
                matching_field
                    .result
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .clone()
            } else {
                caps[0].to_string()
            }
        })
        .to_string()
}
