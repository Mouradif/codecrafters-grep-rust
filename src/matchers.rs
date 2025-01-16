pub fn match_char(input_line: &str, char: &str) -> bool {
    return input_line.contains(char);
}

pub fn match_digit(input_line: &str) -> bool {
    return input_line.chars().any(|c| c.is_digit(10));
}

pub fn match_wordlike(input_line: &str) -> bool {
    return input_line.chars().any(|c| c.is_digit(10) || c.is_alphabetic() || c == '_');
}

pub fn match_positive_group(input_line: &str, chars: &str) -> bool {
    for char in chars.chars() {
        if input_line.contains(char) {
            return true;
        }
    }
    false
}