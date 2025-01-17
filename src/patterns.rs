#[derive(Clone, Debug)]
pub enum Pattern {
    SingleCharacter(char),
    Repeating(Box<Pattern>),
    Optional(Box<Pattern>),
    Digit,
    WordLike,
    Any(Vec<Pattern>, bool),
}

impl Pattern {
    pub fn single_character(c: char) -> Self {
        Pattern::SingleCharacter(c)
    }

    pub fn repeating(p: Pattern) -> Self {
        Pattern::Repeating(Box::new(p))
    }

    pub fn optional(p: Pattern) -> Self {
        Pattern::Optional(Box::new(p))
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

    pub fn matches(&self, haystack: &str, match_start: bool, match_end: bool) -> Option<usize> {
        eprintln!("Trying to match {} with {:?}", haystack, self);
        if let Some(index) = match self {
            Pattern::SingleCharacter(c) => haystack.find(*c),
            Pattern::Repeating(p) => {
                let mut found = false;
                for i in 0..haystack.chars().count() {
                    if !found && !p.matches(&haystack[i..], true, false).is_none() {
                        found = true;
                    }
                    if found && p.matches(&haystack[i..], true, false).is_none() {
                        return Some(i - 1)
                    }
                }
                return None;
            },
            Pattern::Digit => haystack.chars().position(|c| c.is_digit(10)),
            Pattern::WordLike => haystack
                .chars()
                .position(|c| c.is_digit(10) || c.is_alphabetic() || c == '_'),
            Pattern::Any(patterns, is_negative) => {
                for p in patterns {
                    let matches = p.matches(haystack, match_start, match_end);
                    if *is_negative && !matches.is_none() {
                        return None;
                    }
                    if !*is_negative && !matches.is_none() {
                        return matches;
                    }
                }
                if *is_negative {
                    return Some(0);
                }
                return None
            },
            _ => None
        } {
            return if (match_start && index > 0) || (match_end && index != haystack.len() - 1) {
                None
            } else {
                Some(index)
            }
        }
        None
    }
}
