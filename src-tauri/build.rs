use std::env;
use std::fs;
use std::path::Path;

fn main() {
    tauri_build::build();

    let profile = env::var("PROFILE").unwrap();
    if let Ok(profile) = env::var("PROFILE") {
        if profile == "debug" {
            let build_path = Path::new(".").join("target").join(profile).join("logjam");
            let target_path = Path::new(".").join("portable");
            fs::copy(build_path, target_path).expect("Could not copy executable");
        }
    }
}
