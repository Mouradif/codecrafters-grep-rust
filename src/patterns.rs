#[derive(Clone, Debug)]
pub enum Pattern {
    SingleCharacter(char),
    Digit,
    WordLike,
    Any(Vec<Pattern>, bool),
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

    pub fn any() -> Self {
        Pattern::Any(vec![], false)
    }

    pub fn first_match(&self, haystack: &str) -> Option<usize> {
        match self {
            Pattern::SingleCharacter(c) => haystack.find(*c),
            Pattern::Digit => haystack.chars().position(|c| c.is_digit(10)),
            Pattern::WordLike => haystack
                .chars()
                .position(|c| c.is_digit(10) || c.is_alphabetic() || c == '_'),
            Pattern::Any(patterns, is_negative) => patterns.iter().position(|p| {
                p.first_match(haystack).is_none() == *is_negative
            }),
        }
    }

    pub fn matches(&self, haystack: &str) -> bool {
        let first_char = haystack.chars().next();
        if first_char.is_none() {
            return false;
        }
        let f = first_char.unwrap();
        match self {
            Pattern::SingleCharacter(c) => f == *c,
            Pattern::Digit => f.is_digit(10),
            Pattern::WordLike => f.is_digit(10) || f.is_alphabetic() || f == '_',
            Pattern::Any(patterns, is_negative) => patterns.iter().any(|p| {
                p.matches(haystack) == !*is_negative
            })
        }
    }
}
