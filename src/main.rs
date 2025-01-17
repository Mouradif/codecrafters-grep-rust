use std::env;
use std::io;
use std::process;
use codecrafters_grep::patterns::*;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut patterns: Vec<Pattern> = vec![];
    let mut current_group: Option<Pattern> = None;
    let mut is_escaping = false;
    let mut max_distance_from_start = input_line.len();
    let mut match_end = false;
    let mut last_pattern: Option<Pattern> = None;

    let chars = pattern.chars();
    let count = chars.clone().count();

    for (index, char) in chars.enumerate() {
        match char {
            '\\' => {
                if is_escaping {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                }
                is_escaping = !is_escaping;
            },
            'd' => {
                let p: Pattern = if is_escaping {
                    is_escaping = false;
                    Pattern::digit()
                } else {
                    Pattern::single_character(char)
                };
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                }
            },
            'w' => {
                let p: Pattern = if is_escaping {
                    is_escaping = false;
                    Pattern::word_like()
                } else {
                    Pattern::single_character(char)
                };
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                }
            },
            '[' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else {
                    current_group = Some(Pattern::any());
                }
            },
            ']' => {
                if let Some(Pattern::Any(group_chars, is_negative)) = &mut current_group {
                    if is_escaping {
                        group_chars.push(Pattern::single_character(char));
                    } else if !group_chars.is_empty() {
                        let p = Pattern::Any(group_chars.clone(), *is_negative);
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                        current_group = None;
                    }
                } else {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
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
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
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
                        patterns.push(p.clone());
                        last_pattern = Some(p.clone());
                    }
                }
            },
            '+' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
                } else {
                    if let Some(Pattern::Repeating(_)) = last_pattern {
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
                }
            },
            '?' => {
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(Pattern::single_character(char));
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
                } else if is_escaping {
                    let p = Pattern::single_character(char);
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                } else {
                    let p = Pattern::wildcard();
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                }
            },
            _ => {
                let p = Pattern::single_character(char);
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    patterns.push(p.clone());
                    last_pattern = Some(p.clone());
                }
            }
        }
        if is_escaping && char != '\\' {
            is_escaping = false;
        }
    }
    let mut position = 0;
    for (index, p) in patterns.iter().enumerate() {
        if let Pattern::Optional(pat) = p {
            if let (Some(found_at), _) = pat.matches(
                &input_line[position..],
                max_distance_from_start,
                match_end
            ) {
                position += found_at + 1;
                max_distance_from_start = 0;
            }
        } else if let Pattern::Repeating(_) = p {
            let (maybe_found, len) = p.matches(
                &input_line[position..],
                max_distance_from_start,
                index == patterns.len() - 1 && match_end
            );
            if maybe_found.is_none() {
                return false;
            }
            max_distance_from_start = len;
            position += 1;
        } else if let (Some(found_at), _) = p.matches(
            &input_line[position..],
            max_distance_from_start,
            index == patterns.len() - 1 && match_end
        ) {
            max_distance_from_start = 0;
            position += found_at + 1;
        } else {
            return false;
        }
    }
    true
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
