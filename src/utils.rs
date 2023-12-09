pub use serde_json::{json, Value};

use base64::{engine::general_purpose, Engine as _};
use std::io::prelude::*;
use std::os::unix::fs::MetadataExt;
use std::str;
use std::{process, thread, time};

pub struct Utils {}

impl Utils {
    pub fn json_to_string_pretty(json: Value) -> String {
        return serde_json::to_string_pretty(&json).unwrap();
    }

    pub fn bash(command: &str) {
        process::Command::new("bash")
            .arg("-c")
            .arg(command)
            .status()
            .expect("Error!");
    }

    pub fn sleep(seconds: u64) {
        let duration = time::Duration::from_secs(seconds);
        thread::sleep(duration)
    }

    #[cfg(unix)]
    pub fn verify_root() {
        if let Ok(val) = std::fs::metadata("/proc/self").map(|m| m.uid()) {
            if val != 0 {
                println!("User isn't root");
                process::exit(1);
            }
        }
    }

    pub fn base64_to_file(b64: &str, path: &str, file: &str) {
        std::fs::create_dir_all(path).expect("Error creating directory!");
        let mut file =
            std::fs::File::create(format!("{0}{1}", path, file)).expect("Error creating file!");
        let bytes = general_purpose::STANDARD
            .decode(b64)
            .expect("Error decoding base64!");
        file.write_all(&bytes).expect("Error writing to file!");
    }
}
