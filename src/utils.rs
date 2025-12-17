use std::fs::{self, File};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use serde::ser::Serialize;
use serde_json::Value;

pub fn check_regex(content: &str, regex: &str) -> bool {
    let regex = Regex::new(regex).unwrap();
    return regex.is_match(content);
}

pub fn check_regexes(content: &str, regexes: Vec<&str>) -> bool {
    for regex in regexes {
        let regex = Regex::new(regex).unwrap();
        if regex.is_match(content) {
            return true;
        }
    }
    return false;
}

pub fn struct_to_string<T: Serialize>(value: &T) -> String {
    return serde_json::to_string(&value).unwrap();
}

pub fn json_to_pretty_string(json: Value) -> String {
    return serde_json::to_string_pretty(&json).unwrap();
}

pub fn sleep(seconds: u64) {
    let duration = Duration::from_secs(seconds);
    thread::sleep(duration)
}

pub fn base64_to_file(b64: &str, path: &str, file: &str) {
    fs::create_dir_all(path).expect("Error creating directory!");
    let mut file = File::create(format!("{0}{1}", path, file)).expect("Error creating file!");
    let bytes = general_purpose::STANDARD
        .decode(b64)
        .expect("Error decoding base64!");
    file.write_all(&bytes).expect("Error writing to file!");
}

pub fn text_to_base64(content: &str) -> String {
    return general_purpose::STANDARD.encode(content.as_bytes());
}
