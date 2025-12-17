use std::env;
use std::fs;

pub fn load() {
    load_file(".env");
}

pub fn load_file(path: &str) {
    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                unsafe { env::set_var(key.trim(), value.trim()) };
            }
        }
    }
}

pub fn load_content(content: &str) {
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            unsafe { env::set_var(key.trim(), value.trim()) };
        }
    }
}

pub fn get(key: &str) -> String {
    return env::var(key).unwrap_or_else(|_| {
        println!("Environment variable \"{}\" not found.", key);
        std::process::exit(0);
    });
}
