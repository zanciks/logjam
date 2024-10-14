use regex::Regex;
use roxmltree::Document;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct CallbackField {
    pub name: String,
    pub pattern: Regex,
    pub file_name: String,
}

impl CallbackField {
    fn new(name: String, pattern: Regex, file_name: String) -> Self {
        Self {
            name,
            pattern,
            file_name,
        }
    }
    pub fn list_all(plugin_path: &Path) -> Vec<Self> {
        let mut callback_fields: Vec<CallbackField> = vec![];

        if plugin_path.join("callbackFields.xml").exists() {
            let xml = fs::read_to_string(&plugin_path.join("callbackFields.xml")).unwrap();
            let document = Document::parse(&xml).unwrap();
            for node in document.descendants() {
                if node.has_tag_name("callbackField") {
                    let mut name: Option<String> = None;
                    let mut pattern: Option<Regex> = None;
                    let mut file_name: Option<String> = None;

                    for child in node.descendants() {
                        match child.tag_name().name() {
                            "name" => name = child.text().map(|s| s.to_string()),
                            "fileName" => file_name = child.text().map(|s| s.to_string()),
                            "pattern" => {
                                if let Some(text) = child.text() {
                                    pattern = Regex::new(text).ok()
                                }
                            }
                            _ => (),
                        }
                    }

                    if name.is_some() && pattern.is_some() && file_name.is_some() {
                        callback_fields.push(CallbackField::new(
                            name.unwrap(),
                            pattern.unwrap(),
                            file_name.unwrap(),
                        ));
                    }
                }
            }
        }

        return callback_fields;
    }
}
