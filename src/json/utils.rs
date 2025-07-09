use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub fn serialize<T: Serialize>(object: &T) -> Value {
    return serde_json::to_value(&object).unwrap();
}

pub fn deserialize<T: DeserializeOwned>(object: Value) -> T {
    return serde_json::from_value(object).expect("Failed to deserialize");
}
