use std::path::Path;
use std::fs;
use roxmltree::Document;

pub struct CallbackField {
    element_name: String,
    pattern: String,
    file_name: String,
}

// todo
// function Self::list(path: Path)
// if path contains a callbackFields.xml file, then
// // parse the XML and return a vector of Self 
// // based on what is in the file. 
// return the Vec<CallbackField>

pub fn list_all(plugin_path: &Path) -> Vec<CallbackField> {
    let callback_fields: Vec<CallbackField> = vec![];

    if plugin_path.join("callbackFields.xml").exists() {
        let xml = fs::read_to_string(&plugin_path.join("callbackFields.xml")).unwrap();
        let document = Document::parse(&xml).unwrap();
        for node in document.descendants() {
            if node.has_tag_name("element") { println!("test!") }
        }
    }

    return callback_fields;
}