pub use serde_json::{json, Value};

use base64::{engine::general_purpose, Engine as _};
use std::fs::{self, File};
use std::io::prelude::*;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::time::Duration;
use std::{process, thread};

pub fn json_to_pretty_string(json: Value) -> String {
    return serde_json::to_string_pretty(&json).unwrap();
}

pub fn sleep(seconds: u64) {
    let duration = Duration::from_secs(seconds);
    thread::sleep(duration)
}

#[cfg(unix)]
pub fn verify_root() {
    if let Ok(val) = fs::metadata("/proc/self").map(|m| m.uid()) {
        if val != 0 {
            println!("User isn't root");
            process::exit(1);
        }
    }
}

pub fn base64_to_file(b64: &str, path: &str, file: &str) {
    fs::create_dir_all(path).expect("Error creating directory!");
    let mut file = File::create(format!("{0}{1}", path, file)).expect("Error creating file!");
    let bytes = general_purpose::STANDARD
        .decode(b64)
        .expect("Error decoding base64!");
    file.write_all(&bytes).expect("Error writing to file!");
}
