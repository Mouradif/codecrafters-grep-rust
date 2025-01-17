#[derive(Clone, Debug)]
enum Pattern {
    SingleCharacter(char),
    Repeating(Box<Pattern>),
    Optional(Box<Pattern>),
    Digit,
    WordLike,
    Any(Vec<Pattern>, bool),
    Wildcard,
    Choice(Vec<Vec<Pattern>>),
    BackReference(usize),
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

    pub fn choice(choices: Vec<Vec<Pattern>>) -> Self {
        Pattern::Choice(choices)
    }

    pub fn backreference(n: usize) -> Self {
        Pattern::BackReference(n)
    }

    pub fn matches(&self, haystack: &str, captures: &mut Vec<Vec<Pattern>>, max_distance_from_start: usize, match_end: bool) -> (Option<usize>, usize) {
        if let (Some(index), len) = find_match(self, haystack, captures, max_distance_from_start, match_end) {
            return if (index > max_distance_from_start + 1) || (match_end && index + len != haystack.len()) {
                (None, 0)
            } else {
                (Some(index), len)
            };
        }
        (None, 0)
    }
}

fn match_any(patterns: &Vec<Pattern>, haystack: &str, captures: &mut Vec<Vec<Pattern>>, max_distance_from_start: usize, match_end: bool) -> (Option<usize>, usize) {
    for pattern in patterns {
        if let (Some(index), _) = pattern.matches(haystack, captures, max_distance_from_start, match_end) {
            return (Some(index), 0)
        }
    }
    (None, 0)
}

fn match_none(patterns: &Vec<Pattern>, haystack: &str, captures: &mut Vec<Vec<Pattern>>, max_distance_from_start: usize, match_end: bool) -> (Option<usize>, usize) {
    for pattern in patterns {
        if pattern.matches(haystack, captures, max_distance_from_start, match_end).0.is_some() {
            return (None, 0)
        }
    }
    (Some(1), 1)
}

fn find_match(pattern: &Pattern, haystack: &str, captures: &mut Vec<Vec<Pattern>>, max_distance_from_start: usize, match_end: bool) -> (Option<usize>, usize) {
    match pattern {
        Pattern::SingleCharacter(c) => {
            if let Some(pos) = haystack.find(*c) {
                (Some(pos + 1), 0)
            } else {
                (None, 0)
            }
        },
        Pattern::Repeating(p) => {
            let mut found = false;
            let mut index = 0;
            for (i, _) in haystack.char_indices() {
                let (matches, _) = p.matches(&haystack[i..], captures, 0, false);
                if !found && !matches.is_none() {
                    found = true;
                    index = i;
                }
                if found && matches.is_none() {
                    return (Some(index + 1), i - index)
                }
            }
            if !found {
                (None, 0)
            } else {
                (Some(index + 1), haystack.char_indices().count() - index)
            }
        },
        Pattern::Digit => {
            if let Some(pos) = haystack.chars().position(|c| c.is_digit(10)) {
                (Some(pos + 1), 0)
            } else {
                (None, 0)
            }
        },
        Pattern::WordLike => {
            if let Some(pos) = haystack
                .chars()
                .position(|c| c.is_digit(10) || c.is_alphabetic() || c == '_') {
                (Some(pos + 1), 0)
            } else {
                (None, 0)
            }
        },
        Pattern::Any(patterns, is_negative) => {
            if *is_negative {
                match_none(patterns, haystack, captures, max_distance_from_start, match_end)
            } else {
                match_any(patterns, haystack, captures, max_distance_from_start, match_end)
            }
        },
        Pattern::Wildcard => (Some(1), 0),
        Pattern::Choice(pattern_lists) => {
            for patterns in pattern_lists {
                if let Some((pos, len)) = match_patterns(haystack, patterns.to_vec(), max_distance_from_start, match_end) {
                    let capture_str = &haystack[pos..len - 1];
                    let capture: Vec<Pattern> = capture_str.chars().map(|c| Pattern::SingleCharacter(c)).collect();
                    captures.push(capture);
                    return (Some(pos), len);
                }
            }
            (None, 0)
        },
        Pattern::Optional(pattern) => {
            if let (Some(index), len) = pattern.matches(haystack, captures, max_distance_from_start, match_end) {
                (Some(index), len)
            } else {
                (Some(0), 0)
            }
        },
        Pattern::BackReference(index) => {
            if *index >= captures.len() {
                (None, 0)
            } else {
                if let Some((pos, len)) = match_patterns(haystack, captures[*index].clone(), max_distance_from_start, match_end) {
                    (Some(pos), len)
                } else {
                    (None, 0)
                }
            }
        }
    }
}

fn parse_patterns(pattern: &str, max_len: usize) -> (Vec<Pattern>, usize, bool) {
    let mut patterns: Vec<Pattern> = vec![];
    let mut current_group: Option<Pattern> = None;
    let mut current_choices: Vec<Vec<Pattern>> = vec![];
    let mut is_escaping = false;
    let mut max_distance_from_start = max_len;
    let mut match_end = false;
    let mut last_pattern: Option<Pattern> = None;
    let mut is_adding_choice = false;

    let chars = pattern.chars();
    let count = chars.clone().count();

    for (index, char) in chars.enumerate() {
        match char {
            '\\' => {
                if is_escaping {
                    let p = Pattern::single_character(char);
                    if is_adding_choice {
                        current_choices.last_mut().unwrap().push(p);
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
                is_escaping = !is_escaping;
            },
            'd' => {
                let p: Pattern = if is_escaping {
                    Pattern::digit()
                } else {
                    Pattern::single_character(char)
                };
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    if is_adding_choice {
                        current_choices.last_mut().unwrap().push(p);
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            },
            'w' => {
                let p: Pattern = if is_escaping {
                    Pattern::word_like()
                } else {
                    Pattern::single_character(char)
                };
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    if is_adding_choice {
                        current_choices.last_mut().unwrap().push(p);
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            },
            '[' => {
                if !is_escaping && current_group.is_none() {
                    current_group = Some(Pattern::any());
                } else {
                    let p = Pattern::single_character(char);
                    if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                        group_chars.push(Pattern::single_character(char));
                    } else if is_adding_choice {
                        current_choices.last_mut().unwrap().push(Pattern::single_character(char));
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            },
            ']' => {
                if let Some(Pattern::Any(group_chars, is_negative)) = &mut current_group {
                    if is_escaping {
                        group_chars.push(Pattern::single_character(char));
                    } else if !group_chars.is_empty() {
                        let p = Pattern::Any(group_chars.clone(), *is_negative);
                        if is_adding_choice {
                            current_choices.last_mut().unwrap().push(p);
                        } else {
                            patterns.push(p.clone());
                            last_pattern = Some(p.clone());
                        }
                        current_group = None;
                    }
                } else {
                    let p = Pattern::single_character(char);
                    if is_adding_choice {
                        current_choices.last_mut().unwrap().push(p);
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            },
            '^' => {
                if index == 0 {
                    max_distance_from_start = 0;
                } else if let Some(Pattern::Any(group_chars, is_negative)) = &mut current_group {
                    if group_chars.is_empty() && !*is_negative {
                        *is_negative = true;
                    } else {
                        group_chars.push(Pattern::single_character(char));
                    }
                } else {
                    let p = Pattern::single_character(char);
                    if is_adding_choice {
                        current_choices.last_mut().unwrap().push(p);
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            },
            '$' => {
                if index == count - 1 && !is_escaping {
                    match_end = true;
                } else {
                    let p = Pattern::single_character(char);
                    if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                        group_chars.push(p);
                    } else {
                        if is_adding_choice {
                            current_choices.last_mut().unwrap().push(p);
                        } else {
                            patterns.push(p.clone());
                            last_pattern = Some(p.clone());
                        }
                    }
                }
            },
            '+' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else if is_adding_choice {
                    let last_choice = current_choices.last_mut().unwrap();
                    let p = if last_choice.last().is_none() {
                        Pattern::single_character(char)
                    } else {
                        let last_elem = last_choice.pop().unwrap();
                        if let Pattern::Repeating(p) = last_elem {
                            last_choice.push(Pattern::repeating(*p));
                            Pattern::single_character(char)
                        } else {
                            Pattern::repeating(last_elem)
                        }
                    };
                    last_choice.push(p);
                } else if let Some(Pattern::Repeating(_)) = last_pattern {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                } else if last_pattern.is_none() || is_escaping {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                } else {
                    let last = patterns.pop().unwrap();
                    patterns.push(Pattern::repeating(last));
                    last_pattern = None;
                }
            },
            '?' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else if is_adding_choice {
                    let last_choice = current_choices.last_mut().unwrap();
                    let p = if last_choice.last().is_none() {
                        Pattern::single_character(char)
                    } else {
                        let last_elem = last_choice.pop().unwrap();
                        if let Pattern::Optional(p) = last_elem {
                            last_choice.push(Pattern::optional(*p));
                            Pattern::single_character(char)
                        } else {
                            Pattern::optional(last_elem)
                        }
                    };
                    last_choice.push(p);
                } else if let Some(Pattern::Optional(_)) = last_pattern {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                } else if last_pattern.is_none() || is_escaping {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                } else {
                    let last = patterns.pop().unwrap();
                    patterns.push(Pattern::optional(last));
                    last_pattern = None;
                }
            },
            '.' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else {
                    let p = if is_escaping { Pattern::single_character(char) } else { Pattern::wildcard() };
                    if is_adding_choice {
                        current_choices.last_mut().unwrap().push(p);
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            },
            '(' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else if is_adding_choice {
                    current_choices.last_mut().unwrap().push(Pattern::single_character(char))
                } else if is_escaping {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                } else {
                    is_adding_choice = true;
                    current_choices.push(vec![]);
                }
            },
            '|' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else if !is_adding_choice {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                } else {
                    current_choices.push(vec![]);
                }
            },
            ')' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else  {
                    let p = if !is_adding_choice {
                        Pattern::single_character(char)
                    } else {
                        let p = Pattern::choice(current_choices.clone());
                        is_adding_choice = false;
                        current_choices.clear();
                        p
                    };
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                }
            },
            '1'..='9' => {
                let p = if !is_escaping { Pattern::single_character(char) } else {
                    let n: usize = char.to_string().parse().unwrap();
                    Pattern::backreference(n - 1)
                };
                patterns.push(p.clone());
                last_pattern = Some(p.clone());
            },
            _ => {
                let p = Pattern::single_character(char);
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    if is_adding_choice {
                        current_choices.last_mut().unwrap().push(p);
                    } else {
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            }
        }
        if is_escaping && char != '\\' {
            is_escaping = false;
        }
    }
    (patterns, max_distance_from_start, match_end)
}

fn match_patterns(input_line: &str, patterns: Vec<Pattern>, mut max_distance_from_start: usize, match_end: bool) -> Option<(usize, usize)> {
    let mut first_position = 0;
    let mut position = 0;
    let mut captures: Vec<Vec<Pattern>> = vec![];

    for (index, p) in patterns.iter().enumerate() {
        if position >= input_line.len() {
            return None;
        }
        if let (Some(found_at), len) = p.matches(
            &input_line[position..],
            &mut captures,
            max_distance_from_start,
            index == patterns.len() - 1 && match_end
        ) {
            if first_position == 0 {
                first_position = found_at;
            }
            max_distance_from_start = len;
            position += found_at;
            if let Pattern::Choice(_) = p {
                position += len - 1;
            }
            if let Pattern::BackReference(_) = p {
                position += len - 1;
            }
        } else {
            return None;
        }
    }
    Some((first_position - 1, position + max_distance_from_start))
}

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let (patterns, max_distance_from_start, match_end) = parse_patterns(pattern, input_line.len());
    match_patterns(input_line, patterns, max_distance_from_start, match_end).is_some()
}
