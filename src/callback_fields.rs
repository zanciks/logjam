use std::path::Path;
use std::fs;
use roxmltree::Document;

#[derive(Debug)]
pub struct CallbackField {
    pub name: Option<String>,
    pub pattern: Option<String>,
    pub file_name: Option<String>,
}

impl Default for CallbackField {
    fn default() -> Self {
        CallbackField {
            name: None,
            pattern: None,
            file_name: None
        }
    }
}

pub fn list_all(plugin_path: &Path) -> Vec<CallbackField> {
    let mut callback_fields: Vec<CallbackField> = vec![];

    if plugin_path.join("callbackFields.xml").exists() {
        let xml = fs::read_to_string(&plugin_path.join("callbackFields.xml")).unwrap();
        let document = Document::parse(&xml).unwrap();
        for node in document.descendants() {
            if node.has_tag_name("callbackField") {
                let mut callback_field = CallbackField::default();

                for child in node.descendants() {
                    match child.tag_name().name() {
                        "name"     => { callback_field.name      = child.text().map(|s| s.to_string()) },
                        "pattern"  => { callback_field.pattern   = child.text().map(|s| s.to_string()) },
                        "fileName" => { callback_field.file_name = child.text().map(|s| s.to_string()) },
                        _ => ()
                    }
                }

                if callback_field.name.is_some() && callback_field.pattern.is_some() && callback_field.file_name.is_some() {
                    callback_fields.push(callback_field);
                }
            }
        }
    }

    return callback_fields;
}