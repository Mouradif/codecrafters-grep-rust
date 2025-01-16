#[derive(Clone)]
pub enum Pattern {
    SingleCharacter(char),
    Digit,
    WordLike,
    Group(Vec<Pattern>, bool),
}

impl Pattern {
    pub fn single_character(c: char) -> Self {
        Pattern::SingleCharacter(c)
    }

    pub fn digit() -> Self {
        Pattern::Digit
    }

    pub fn word_like() -> Self {
        Pattern::WordLike
    }

    pub fn group(chars: Vec<Pattern>, is_negative: bool) -> Self {
        Pattern::Group(chars, is_negative)
    }

    pub fn first_match(&self, haystack: &str) -> Option<usize> {
        match self {
            Pattern::SingleCharacter(c) => haystack.find(*c),
            Pattern::Digit => haystack.chars().position(|c| c.is_digit(10)),
            Pattern::WordLike => haystack
                .chars()
                .position(|c| c.is_digit(10) || c.is_alphabetic() || c == '_'),
            Pattern::Group(patterns, is_negative) => patterns.iter().position(|p| {
                p.first_match(haystack).is_none() == *is_negative
            }),
        }
    }
}
