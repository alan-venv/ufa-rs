use std::fs;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub fn serialize<T: Serialize>(object: &T) -> Value {
    return serde_json::to_value(&object).expect("Failed to serialize");
}

pub fn deserialize<T: DeserializeOwned>(object: Value) -> T {
    return serde_json::from_value(object).expect("Failed to deserialize");
}

pub fn parse_str(content: &str) -> Value {
    return serde_json::from_str(content).expect("Failed to parse_str");
}

pub fn parse_file(path: &str) -> Value {
    let content = fs::read_to_string(path).expect("Failed to parse_file");
    return parse_str(&content);
}
