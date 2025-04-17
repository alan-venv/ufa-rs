use std::fmt::{Display, Formatter, Result};

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
    fn red(&self) -> CustomString;
    fn green(&self) -> CustomString;
    fn yellow(&self) -> CustomString;
    fn blue(&self) -> CustomString;
    fn magenta(&self) -> CustomString;
    fn cyan(&self) -> CustomString;
    fn white(&self) -> CustomString;
    fn bold(&self) -> CustomString;
    fn dim(&self) -> CustomString;
    fn italic(&self) -> CustomString;
    fn underline(&self) -> CustomString;
    fn reverse(&self) -> CustomString;
    fn hidden(&self) -> CustomString;
}

impl CustomStr for str {
    fn red(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(RED_COLOR),
        };
    }
    fn green(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(GREEN_COLOR),
        };
    }
    fn yellow(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(YELLOW_COLOR),
        };
    }
    fn blue(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(BLUE_COLOR),
        };
    }
    fn magenta(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(MAGENTA_COLOR),
        };
    }
    fn cyan(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(CYAN_COLOR),
        };
    }
    fn white(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(WHITE_COLOR),
        };
    }

    fn bold(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(BOLD_TEXT),
        };
    }

    fn dim(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(DIM_TEXT),
        };
    }

    fn italic(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(ITALIC_TEXT),
        };
    }

    fn underline(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(UNDERLINE_TEXT),
        };
    }

    fn reverse(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(REVERSE_VIDEO),
        };
    }

    fn hidden(&self) -> CustomString {
        return CustomString {
            value: String::from(self),
            style: String::from(HIDDEN_TEXT),
        };
    }
}

pub struct CustomString {
    value: String,
    style: String,
}

impl CustomString {
    pub fn red(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(RED_COLOR),
        };
    }

    pub fn green(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(GREEN_COLOR),
        };
    }

    pub fn yellow(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(YELLOW_COLOR),
        };
    }

    pub fn blue(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(BLUE_COLOR),
        };
    }

    pub fn magenta(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(MAGENTA_COLOR),
        };
    }

    pub fn cyan(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(CYAN_COLOR),
        };
    }

    pub fn white(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(WHITE_COLOR),
        };
    }

    pub fn bold(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(BOLD_TEXT),
        };
    }

    pub fn dim(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(DIM_TEXT),
        };
    }

    pub fn italic(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(ITALIC_TEXT),
        };
    }

    pub fn underline(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(UNDERLINE_TEXT),
        };
    }

    pub fn reverse(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(REVERSE_VIDEO),
        };
    }

    pub fn hidden(&self) -> CustomString {
        return CustomString {
            value: format!("{}{}", self.style, self.value),
            style: String::from(HIDDEN_TEXT),
        };
    }
}

impl Display for CustomString {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}{}", self.style, self.value, DEFAULT_STYLE)
    }
}
