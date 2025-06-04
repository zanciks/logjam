use regex::Regex;
use roxmltree::Document;
use std::{
    fs::{File, metadata},
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

pub struct CallbackField {
    pub name: String,
    pub pattern: Regex,
    pub file_name: String,

    pub hint_text: Option<String>,
    pub group: bool,

    pub result: Arc<Mutex<Option<String>>>,
    pub stop_signal: Arc<AtomicBool>,
}

impl CallbackField {
    fn new(
        name: Option<String>,
        pattern: Option<Regex>,
        file_name: Option<String>,
        hint_text: Option<String>,
        group: bool,
    ) -> Self {
        let name = name.unwrap_or_default();
        let pattern = pattern.unwrap();
        let file_name = file_name.unwrap_or_default();

        let result = Arc::new(Mutex::new(None));
        let stop_signal = Arc::new(AtomicBool::new(false));

        CallbackField {
            name,
            pattern,
            file_name,
            hint_text,
            group,
            result,
            stop_signal,
        }
    }
    pub fn get_callback_fields() -> Vec<CallbackField> {
        let mut callback_fields: Vec<CallbackField> = vec![];

        let exe_path = std::env::current_exe().expect("Can't find current exe path");
        let parent = exe_path.parent().expect("Couldn't get exe path's parent");

        let mut file = File::open(&parent.join("callback_fields.xml"))
            .or_else(|_| File::create(&parent.join("callback_fields.xml")))
            .expect("Could not open or create file to search!");

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't read callback fields file to string!");
        let document = Document::parse(&contents).expect("callback_fields.xml isn't valid XML!");

        for node in document.descendants() {
            if node.has_tag_name("callbackField") {
                log::info!("Registering callback field: {}", node.tag_name().name());
                callback_fields.push(Self::get_callback_field(node));
            }
        }

        return callback_fields;
    }
    fn get_callback_field(node: roxmltree::Node<'_, '_>) -> CallbackField {
        let mut name = None;
        let mut pattern = None;
        let mut file_name = None;
        let mut hint_text = None;
        let mut group: bool = false;

        for child in node.descendants() {
            match child.tag_name().name() {
                "name" => name = child.text().map(|s| s.to_string()),
                "pattern" => {
                    pattern = child
                        .text()
                        .map(|s| Regex::new(&s).expect("Failed to make regular expression!"))
                }
                "file_name" => file_name = child.text().map(|s| s.to_string()),
                "hint_text" => hint_text = child.text().map(|s| s.to_string()),
                "group" => group = child.text().map(|s| s == "true").unwrap_or(false),
                _ => (),
            }
        }

        if name.is_some() && pattern.is_some() && file_name.is_some() {
            return CallbackField::new(name, pattern, file_name, hint_text, group);
        }

        log::error!("Callback Field is missing one or more fields!");
        log::error!("name: {:?}", name);
        log::error!("pattern: {:?}", pattern);
        log::error!("file_name: {:?}", file_name);
        log::error!("hint_text: {:?}", hint_text);
        log::error!("group: {:?}", group);

        panic!("One or more callback fields is missing a required field!")
    }
    pub fn begin_search(&self, folder_path: &Option<PathBuf>) {
        if folder_path.is_none() {
            log::info!("No folder is present for callback field {}", &self.name);
            return;
        }

        let file_path = folder_path.clone().unwrap().join(self.file_name.clone());
        let re = self.pattern.clone();
        let should_group = self.group.clone();

        let stop_signal: Arc<AtomicBool> = Arc::clone(&self.stop_signal);
        let result = Arc::clone(&self.result);

        thread::spawn(move || {
            let mut last_line_matched = false;
            let full_path = Path::new(&file_path);
            let mut position: u64 = 0;

            loop {
                if stop_signal.load(Ordering::Relaxed) {
                    break;
                }
                let file = File::open(&full_path)
                    .or_else(|_| File::create(&full_path))
                    .expect("Could not open or create file to search!");

                let mut reader = BufReader::new(file);
                let _ = reader.seek(SeekFrom::Start(position));
                position = metadata(&full_path).map_or(0, |meta| meta.len());

                let mut lines = reader.lines();
                while let Some(Ok(line)) = lines.next() {
                    if let Some(captures) = re.captures(&line) {
                        if let Some(first) = captures.get(1) {
                            let output = first.as_str();
                            let mut res = result.lock().unwrap();

                            if last_line_matched && should_group {
                                if let Some(ref mut existing) = *res {
                                    existing.push_str(", ");
                                    existing.push_str(output);
                                } else {
                                    *res = Some(output.to_string())
                                }
                            } else {
                                *res = Some(output.to_string())
                            }

                            last_line_matched = true;
                        }
                    } else {
                        last_line_matched = false;
                    }
                }
            }
            thread::sleep(Duration::from_secs(1));
        });
    }
    pub fn stop_search(&self) {
        self.stop_signal.store(true, Ordering::Relaxed);
    }
    pub fn result(&self) -> String {
        self.result
            .lock()
            .unwrap()
            .as_deref()
            .unwrap_or_default()
            .to_string()
    }
}
