#[derive(Clone, Debug)]
pub enum Pattern {
    SingleCharacter(char),
    Repeating(Box<Pattern>),
    Optional(Box<Pattern>),
    Digit,
    WordLike,
    Any(Vec<Pattern>, bool),
    Wildcard,
}

fn find_match(pattern: &Pattern, haystack: &str, max_distance_from_start: usize, match_end: bool) -> (Option<usize>, usize){
    match pattern {
        Pattern::SingleCharacter(c) => (haystack.find(*c), 1),
        Pattern::Repeating(p) => {
            let mut found = false;
            for (i, _) in haystack.char_indices() {
                let (matches, _) = p.matches(&haystack[i..], 0, false);
                if !found && !matches.is_none() {
                    found = true;
                }
                if found && matches.is_none() {
                    return (Some(0), i)
                }
            }
            if !found {
                (None, 0)
            } else {
                (Some(0), haystack.char_indices().count())
            }
        },
        Pattern::Digit => (haystack.chars().position(|c| c.is_digit(10)), 1),
        Pattern::WordLike => (
            haystack
                .chars()
                .position(|c| c.is_digit(10) || c.is_alphabetic() || c == '_'),
            1
        ),
        Pattern::Any(patterns, is_negative) => {
            for p in patterns {
                let (matches, _) = p.matches(haystack, max_distance_from_start, match_end);
                if *is_negative && !matches.is_none() {
                    return (None, 0);
                }
                if !*is_negative && !matches.is_none() {
                    return (matches, 1);
                }
            }
            if *is_negative {
                return (Some(0), 1);
            }
            return (None, 0)
        },
        Pattern::Wildcard => (Some(0), 1),
        _ => (None, 0)
    }
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

    pub fn wildcard() -> Self {
        Pattern::Wildcard
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

    pub fn matches(&self, haystack: &str, max_distance_from_start: usize, match_end: bool) -> (Option<usize>, usize) {
        if let (Some(index), len) = find_match(self, haystack, max_distance_from_start, match_end) {
            return if (index > max_distance_from_start) || (match_end && index != haystack.len() - 1) {
                (None, 0)
            } else {
                (Some(index), len)
            }
        }
        (None, 0)
    }
}
