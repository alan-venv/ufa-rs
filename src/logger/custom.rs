const DEFAULT_STYLE: &str = "\x1B[0m";
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
const REVERSE_VIDEO: &str = "\x1B[7m";
const HIDDEN_TEXT: &str = "\x1B[8m";

pub trait CustomStr {
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;
    fn bold(&self) -> String;
    fn dim(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn reverse(&self) -> String;
    fn hidden(&self) -> String;
}

impl CustomStr for String {
    fn red(&self) -> String {
        return format!("{}{}{}", RED_COLOR, self, DEFAULT_STYLE);
    }

    fn green(&self) -> String {
        return format!("{}{}{}", GREEN_COLOR, self, DEFAULT_STYLE);
    }

    fn yellow(&self) -> String {
        return format!("{}{}{}", YELLOW_COLOR, self, DEFAULT_STYLE);
    }

    fn blue(&self) -> String {
        return format!("{}{}{}", BLUE_COLOR, self, DEFAULT_STYLE);
    }

    fn magenta(&self) -> String {
        return format!("{}{}{}", MAGENTA_COLOR, self, DEFAULT_STYLE);
    }

    fn cyan(&self) -> String {
        return format!("{}{}{}", CYAN_COLOR, self, DEFAULT_STYLE);
    }

    fn white(&self) -> String {
        return format!("{}{}{}", WHITE_COLOR, self, DEFAULT_STYLE);
    }

    fn bold(&self) -> String {
        return format!("{}{}{}", BOLD_TEXT, self, DEFAULT_STYLE);
    }

    fn dim(&self) -> String {
        return format!("{}{}{}", DIM_TEXT, self, DEFAULT_STYLE);
    }

    fn italic(&self) -> String {
        return format!("{}{}{}", ITALIC_TEXT, self, DEFAULT_STYLE);
    }

    fn underline(&self) -> String {
        return format!("{}{}{}", UNDERLINE_TEXT, self, DEFAULT_STYLE);
    }

    fn reverse(&self) -> String {
        return format!("{}{}{}", REVERSE_VIDEO, self, DEFAULT_STYLE);
    }

    fn hidden(&self) -> String {
        return format!("{}{}{}", HIDDEN_TEXT, self, DEFAULT_STYLE);
    }
}

impl CustomStr for str {
    fn red(&self) -> String {
        return format!("{}{}{}", RED_COLOR, self, DEFAULT_STYLE);
    }

    fn green(&self) -> String {
        return format!("{}{}{}", GREEN_COLOR, self, DEFAULT_STYLE);
    }

    fn yellow(&self) -> String {
        return format!("{}{}{}", YELLOW_COLOR, self, DEFAULT_STYLE);
    }

    fn blue(&self) -> String {
        return format!("{}{}{}", BLUE_COLOR, self, DEFAULT_STYLE);
    }

    fn magenta(&self) -> String {
        return format!("{}{}{}", MAGENTA_COLOR, self, DEFAULT_STYLE);
    }

    fn cyan(&self) -> String {
        return format!("{}{}{}", CYAN_COLOR, self, DEFAULT_STYLE);
    }

    fn white(&self) -> String {
        return format!("{}{}{}", WHITE_COLOR, self, DEFAULT_STYLE);
    }

    fn bold(&self) -> String {
        return format!("{}{}{}", BOLD_TEXT, self, DEFAULT_STYLE);
    }

    fn dim(&self) -> String {
        return format!("{}{}{}", DIM_TEXT, self, DEFAULT_STYLE);
    }

    fn italic(&self) -> String {
        return format!("{}{}{}", ITALIC_TEXT, self, DEFAULT_STYLE);
    }

    fn underline(&self) -> String {
        return format!("{}{}{}", UNDERLINE_TEXT, self, DEFAULT_STYLE);
    }

    fn reverse(&self) -> String {
        return format!("{}{}{}", REVERSE_VIDEO, self, DEFAULT_STYLE);
    }

    fn hidden(&self) -> String {
        return format!("{}{}{}", HIDDEN_TEXT, self, DEFAULT_STYLE);
    }
}
