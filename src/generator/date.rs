use chrono::{Datelike, Local, Timelike};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn epoch_as_ms() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    return since_the_epoch.as_millis().to_string();
}

pub fn epoch_as_secs() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    return since_the_epoch.as_secs().to_string();
}

pub fn iso() -> String {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let seconds = now.second();

    return format!(
        "{}-{:02}-{:02}T{:02}:{:02}:{:02}.000",
        year, month, day, hour, minute, seconds
    );
}

pub fn pattern(pattern: &str) -> String {
    let now = Local::now();
    now.format(pattern).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        let result = pattern("%Y-%m-%d");
        println!("{}", result);
        assert_eq!(result.len(), 10);
        assert!(result.chars().nth(4) == Some('-'));
        assert!(result.chars().nth(7) == Some('-'));
    }
}
