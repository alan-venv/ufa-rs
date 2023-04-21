use chrono::{Datelike, Local, Timelike};

const DEFAULT_COLOR: &str = "\x1B[0m";
const RED_COLOR: &str = "\x1B[31m";
const GREEN_COLOR: &str = "\x1B[32m";
const YELLOW_COLOR: &str = "\x1B[33m";
const BLUE_COLOR: &str = "\x1B[34m";
const MAGENTA_COLOR: &str = "\x1B[35m";
const CYAN_COLOR: &str = "\x1B[36m";
const WHITE_COLOR: &str = "\x1B[37m";
const BOLD_TEXT: &str = "\x1B[1m";
const DIM_TEXT: &str = "\x1B[2m";
const ITALIC_TEXT: &str = "\x1B[3m";
const UNDERLINE_TEXT: &str = "\x1B[4m";
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

    pub fn blue(text: &str) -> String {
        return format!("{}{}{}", BLUE_COLOR, text, DEFAULT_COLOR);
    }

    pub fn magenta(text: &str) -> String {
        return format!("{}{}{}", MAGENTA_COLOR, text, DEFAULT_COLOR);
    }

    pub fn cyan(text: &str) -> String {
        return format!("{}{}{}", CYAN_COLOR, text, DEFAULT_COLOR);
    }

    pub fn white(text: &str) -> String {
        return format!("{}{}{}", WHITE_COLOR, text, DEFAULT_COLOR);
    }

    pub fn bold(text: &str) -> String {
        return format!("{}{}{}", BOLD_TEXT, text, DEFAULT_COLOR);
    }

    pub fn dim(text: &str) -> String {
        return format!("{}{}{}", DIM_TEXT, text, DEFAULT_COLOR);
    }

    pub fn italic(text: &str) -> String {
        return format!("{}{}{}", ITALIC_TEXT, text, DEFAULT_COLOR);
    }

    pub fn underline(text: &str) -> String {
        return format!("{}{}{}", UNDERLINE_TEXT, text, DEFAULT_COLOR);
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

    pub fn blue(text: &str) {
        println!("{}", Style::blue(text));
    }

    pub fn magenta(text: &str) {
        println!("{}", Style::magenta(text));
    }

    pub fn cyan(text: &str) {
        println!("{}", Style::cyan(text));
    }

    pub fn white(text: &str) {
        println!("{}", Style::white(text));
    }

    pub fn bold(text: &str) {
        println!("{}", Style::bold(text));
    }

    pub fn dim(text: &str) {
        println!("{}", Style::dim(text));
    }

    pub fn italic(text: &str) {
        println!("{}", Style::italic(text));
    }

    pub fn underline(text: &str) {
        println!("{}", Style::underline(text));
    }
}

fn get_log_prefix() -> String {
    let now = Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();

    return format!(
        "[{:02}/{:02}/{}] [{:02}:{:02}]",
        day, month, year, hour, minute
    );
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
        println!(
            "{} [{}] {}",
            get_log_prefix(),
            Style::yellow("WARNING"),
            text
        );
    }
}

#[cfg(windows)]
pub fn set_virtual_terminal(use_virtual: bool) -> Result<(), ()> {
    use winapi::{
        shared::minwindef::DWORD,
        um::{
            consoleapi::{GetConsoleMode, SetConsoleMode},
            processenv::GetStdHandle,
            winbase::STD_OUTPUT_HANDLE,
            wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        },
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut original_mode: DWORD = 0;
        GetConsoleMode(handle, &mut original_mode);

        let enabled = original_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING
            == ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        match (use_virtual, enabled) {
            // not enabled, should be enabled
            (true, false) => {
                SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING | original_mode)
            }
            // already enabled, should be disabled
            (false, true) => {
                SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING ^ original_mode)
            }
            _ => 0,
        };
    }

    Ok(())
}
