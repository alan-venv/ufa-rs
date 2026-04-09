use regex::Regex;

pub fn check_regex(content: &str, regex: &str) -> bool {
    let regex = Regex::new(regex).unwrap();
    return regex.is_match(content);
}

pub fn check_regexes(content: &str, regexes: Vec<&str>) -> bool {
    for regex in regexes {
        let regex = Regex::new(regex).unwrap();
        if regex.is_match(content) {
            return true;
        }
    }
    return false;
}
