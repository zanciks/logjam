use tauri::{Window, command};
use xml::reader::{EventReader, XmlEvent};
use regex::Regex;
use std::{
    fs::{File, metadata},
    path::Path,
    io::{BufReader, BufRead, Seek, SeekFrom},
    thread,
    time::Duration,
};

#[command]
pub fn parse_xml(plugin_name: String) -> Vec<(String, String, String)> {
    let mut elements: Vec<(String, String, String)> = vec![];

    let path = Path::new(".").join("plugins").join(plugin_name).join("patterns.xml");
    let file = File::open(path).expect("Couldn't find patterns.xml for a plugin!");
    let reader = BufReader::new(file);
    let xml_events = EventReader::new(reader);

    let mut in_xml_element = false;
    let mut current_tuple = (String::new(), String::new(), String::new());
    let mut current_tag = String::new();

    for event in xml_events {
        match event.unwrap() {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == "element" {
                    in_xml_element = true;
                    current_tuple = (String::new(), String::new(), String::new());
                } else if in_xml_element {
                    current_tag = name.local_name.clone();
                }
            },
            XmlEvent::Characters(content) => {
                match current_tag.as_str() {
                    "elementName" => { current_tuple.0 = content.clone() },
                    "pattern" => { current_tuple.1 = content.clone() },
                    "fileName" => { current_tuple.2 = content.clone() },
                    _ => ()
                }
            },
            XmlEvent::EndElement { name } => {
                if name.local_name == "element" {
                    in_xml_element = false;
                    elements.push(current_tuple.clone());
                }
            },
            _ => (),
        }
    }
    elements
}

#[command]
pub fn begin_search(name: String, re: String, file_name: String, folder_path: String, window: Window) {
    thread::spawn(move || {
        let full_path = Path::new(&folder_path).join(&file_name);
        let mut position: u64 = 0;

        loop {
            let file = File::open(&full_path).or_else(|_| File::create(&full_path))
                .expect("Could not open for create file to search!");
            let mut reader = BufReader::new(file);
            let _ = reader.seek(SeekFrom::Start(position));
            position = metadata(&full_path).map_or(0, |meta| meta.len());
            
            let mut lines = reader.lines();
            while let Some(Ok(line)) = lines.next() {
                let regular_expression = Regex::new(&re).expect("Couldn't make regular exression!");
                if let Some(captures) = regular_expression.captures(&line) {
                    if let Some(first) = captures.get(1) {
                        let message = format!("[{}] | [{}]", name, first.as_str());
                        window.emit("search-event", message).expect("Failed to send message to front-end!");
                    }
                }
            }
            thread::sleep(Duration::from_secs(5));
        }
    });
}