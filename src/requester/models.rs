use serde_json::Value;

pub struct Response {
    pub status: String,
    pub body: Value,
}
