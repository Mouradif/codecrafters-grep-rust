pub fn is_empty(pattern: &str) -> bool {
    pattern.len() == 0
}

pub fn is_single_char(pattern: &str) -> bool {
    pattern.len() == 1
}

pub fn is_digit(pattern: &str) -> bool {
    pattern == "\\d"
}

pub fn is_wordlike(pattern: &str) -> bool {
    pattern == "\\w"
}

pub fn is_positive_group(pattern: &str) -> bool {
    let mut chars_iter = pattern.chars().into_iter();
    let first = chars_iter.next().unwrap();
    let last = chars_iter.last().unwrap();
    first == '[' && last == ']'
}