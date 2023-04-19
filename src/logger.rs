use chrono::{Local, Datelike, Timelike};

const DEFAULT_COLOR: &str = "\x1B[0m";
const RED_COLOR: &str = "\x1B[31m";
const GREEN_COLOR: &str = "\x1B[32m";
const YELLOW_COLOR: &str = "\x1B[33m";
// const BLUE_COLOR: &str = "\x1B[34m";
// const MAGENTA_COLOR: &str = "\x1B[35m";
// const CYAN_COLOR: &str = "\x1B[36m";
// const WHITE_COLOR: &str = "\x1B[37m";
// const BOLD_TEXT: &str = "\x1B[1m";
// const DIM_TEXT: &str = "\x1B[2m";
// const ITALIC_TEXT: &str = "\x1B[3m";
// const UNDERLINE_TEXT: &str = "\x1B[4m";
// const REVERSE_VIDEO: &str = "\x1B[7m"; // (swap foreground and background colors).
// const HIDDEN_TEXT: &str = "\x1B[8m";

pub struct Style {}

#[allow(dead_code)]
impl Style {
    pub fn red(text: &str) -> String {
        return format!("{}{}{}", RED_COLOR, text, DEFAULT_COLOR);
    }

    pub fn green(text: &str) -> String {
        return format!("{}{}{}", GREEN_COLOR, text, DEFAULT_COLOR);
    }

    pub fn yellow(text: &str) -> String {
        return format!("{}{}{}", YELLOW_COLOR, text, DEFAULT_COLOR);
    }
}


pub struct Print {}

#[allow(dead_code)]
impl Print {
    pub fn red(text: &str) {
        println!("{}", Style::red(text));
    }

    pub fn green(text: &str) {
        println!("{}", Style::green(text));
    }

    pub fn yellow(text: &str) {
        println!("{}", Style::yellow(text));
    }
}

fn get_log_prefix() -> String {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();

    return format!("[{:02}/{:02}/{}] [{:02}:{:02}]", day, month, year, hour, minute);
}

pub struct Log {}

impl Log {
    pub fn info(text: &str) {
        println!("{} [{}] {}", get_log_prefix(), Style::green("INFO"), text);
    }

    pub fn error(text: &str) {
        println!("{} [{}] {}", get_log_prefix(), Style::red("ERROR"), text);
    }

    pub fn warning(text: &str) {
        println!("{} [{}] {}", get_log_prefix(), Style::yellow("WARNING"), text);
    }
}

