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