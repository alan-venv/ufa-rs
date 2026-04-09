use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn base64_decode(content: impl AsRef<str>) -> String {
    return STANDARD
        .decode(content.as_ref())
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .expect("Error decoding base64!");
}

pub fn base64_encode(content: impl AsRef<str>) -> String {
    return STANDARD.encode(content.as_ref().as_bytes());
}
