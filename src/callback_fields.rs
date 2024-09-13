use std::path::Path;
use std::fs;
use roxmltree::Document;

pub struct CallbackField {
    pub name: String,
    pub pattern: String,
    pub file_name: String,
}

impl Default for CallbackField {
    fn default() -> Self {
        CallbackField {
            name: String::new(),
            pattern: String::new(),
            file_name: String::new()
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
                        "name"     => { callback_field.name = child.tag_name().name().to_string() },
                        "pattern"  => { callback_field.pattern      = child.tag_name().name().to_string() },
                        "fileName" => { callback_field.file_name    = child.tag_name().name().to_string() },
                        _ => ()
                    }
                }

                if callback_field.name != String::new() 
                    && callback_field.pattern != String::new() 
                    && callback_field.file_name != String::new() {
                        callback_fields.push(callback_field);
                    }
            }
        }
    }

    return callback_fields;
}