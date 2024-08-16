#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{File, metadata};
use std::path::Path;
use std::io::{Seek, BufReader, BufRead, SeekFrom};
use regex::Regex;
use tauri::generate_handler;
use xml::reader::{EventReader, XmlEvent};
use std::thread;
use std::time::Duration;

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![parse_xml, begin_search])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn begin_search(name: String, re: String, file_name: String, folder_path: String, window: tauri::Window) {
    std::thread::spawn(move || {
        let full_path = Path::new(&folder_path).join(&file_name);
        let mut position: u64 = 0;

        loop {
            let file = File::open(&full_path).or_else(|_| File::create(&full_path))
                .expect("Could not open or create log file!");
            let mut reader = BufReader::new(file);
            let _ = reader.seek(SeekFrom::Start(position));
            position = metadata(&full_path).map_or(0, |meta| meta.len());
            println!("{}", position);
            let mut lines = reader.lines();
            match lines.next() {
                Some(Ok(line)) => {
                    let regular_expression = Regex::new(&re).expect("Couldn't make regular expression!");
                    if let Some(captures) = regular_expression.captures(&line) {
                        if let Some(first) = captures.get(1) {
                            let message = format!("[{}] | [{}]", name, first.as_str());
                            window.emit("search-event", message).expect("Failed to send message!");
                        }
                    }
                },
                _ => ()
            }
            thread::sleep(Duration::from_secs(5));
        }
    });
}

#[tauri::command]
fn parse_xml() -> Vec<(String, String, String)> {
    let parser = EventReader::new(BufReader::new(File::open("/home/zanciks/logjam/src-tauri/patterns.xml").expect("Could not find patterns.xml!")));

    let mut in_element = false;
    let mut current_tag = String::new();
    let mut elements: Vec<(String, String, String)> = vec![];
    let mut current_tuple: (String, String, String) = (String::new(), String::new(), String::new());

    for event in parser {
        match event.unwrap() {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == "element" {
                    in_element = true;
                    current_tuple = (String::new(), String::new(), String::new());
                } else if in_element {
                    current_tag = name.local_name.clone();
                }
            },
            XmlEvent::Characters( content ) => {
                match current_tag.as_str() {
                    "elementName" => { current_tuple.0 = content.clone() },
                    "pattern" => { current_tuple.1 = content.clone() },
                    "fileName" => { current_tuple.2 = content.clone() },
                    _ => ()
                }
            },
            XmlEvent::EndElement { name } => {
                if name.local_name == "element" {
                    in_element = false;
                    elements.push(current_tuple.clone());
                }
            },
            _ => ()
        }
    }

    return elements;
}
