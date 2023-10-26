use serde_json::Value;

pub struct Utils {}

impl Utils {
    pub fn json_to_string_pretty(json: Value) -> String {
        return serde_json::to_string_pretty(&json).unwrap();
    }
}
