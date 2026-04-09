pub mod bash;
pub mod dotfile;
pub mod utils;

#[cfg(unix)]
pub mod unix;

#[cfg(feature = "regex")]
pub mod regex;

#[cfg(feature = "codec")]
pub mod codec;

#[cfg(feature = "generator")]
pub mod generator;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "logger")]
pub mod logger;

#[cfg(feature = "request")]
pub mod requester;
