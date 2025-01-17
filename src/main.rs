use std::env;
use std::io;
use std::process;
use codecrafters_grep::patterns::*;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut patterns: Vec<Pattern> = vec![];
    let mut current_group: Option<Pattern> = None;
    let mut is_escaping = false;
    let mut match_start = false;
    let mut match_end = false;

    let chars = pattern.chars();
    let count = chars.clone().count();

    for (index, char) in chars.enumerate() {
        match char {
            '\\' => {
                if is_escaping {
                    patterns.push(Pattern::single_character(char));
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
                    patterns.push(p);
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
                    patterns.push(p);
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
                        patterns.push(Pattern::Any(group_chars.clone(), *is_negative));
                        current_group = None;
                    }
                } else {
                    patterns.push(Pattern::single_character(char));
                }
            },
            '^' => {
                if index == 0 {
                    match_start = true;
                } else if let Some(Pattern::Any(group_chars, is_negative)) = &mut current_group {
                    if group_chars.is_empty() && !*is_negative {
                        *is_negative = true;
                    } else {
                        group_chars.push(Pattern::single_character(char));
                    }
                } else {
                    patterns.push(Pattern::single_character(char));
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
                        patterns.push(p);
                    }
                }
            },
            '+' => {
                if patterns.len() == 0 || is_escaping {
                    let p = Pattern::single_character(char);
                    if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                        group_chars.push(p);
                    } else {
                        patterns.push(p);
                    }
                } else {
                    patterns.push(Pattern::repeating_character());
                }
            },
            _ => {
                let p = Pattern::single_character(char);
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    patterns.push(p);
                }
            }
        }
        if is_escaping && char != '\\' {
            is_escaping = false;
        }
    }
    let mut position = 0;
    for (index, p) in patterns.iter().enumerate() {
        let previous = if index == 0 { None } else { Some(patterns[index - 1].clone()) };
        if let Some(found_at) = p.matches(
            previous,
            &input_line[position..],
            index == 0 && match_start,
            index == patterns.len() - 1 && match_end
        ) {
            position += found_at + 1;
        } else {
            match p {
                Pattern::RepeatingLast => continue,
                _ => return false,
            }
        }
    }
    return true;
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
