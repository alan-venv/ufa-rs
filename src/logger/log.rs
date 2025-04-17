use crate::generator::date::pattern;

use super::custom::{CustomStr, CustomString};

pub fn info(text: &str) {
    let info = format!("{}{}{}", "[".white(), "INFO".green(), "]".white());
    println!("{} {} {}", log_prefix(), info, text.white());
}

pub fn error(text: &str) {
    let error = format!("{}{}{}", "[".white(), "ERROR".red(), "]".white());
    println!("{} {} {}", log_prefix(), error, text.white());
}

pub fn warning(text: &str) {
    let warning = format!("{}{}{}", "[".white(), "WARNING".yellow(), "]".white());
    println!("{} {} {}", log_prefix(), warning, text.white());
}

fn log_prefix() -> CustomString {
    return pattern("[%d/%m/%Y] [%H:%M:%S]").as_str().white();
}
