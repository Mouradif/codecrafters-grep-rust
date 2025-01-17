use std::env;
use std::io;
use std::process;
use codecrafters_grep::patterns::*;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut patterns: Vec<Pattern> = vec![];
    let mut current_group: Option<Pattern> = None;
    let mut is_escaping = false;
    let mut is_first = true;
    let mut match_start = false;

    for char in pattern.chars() {
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
                if is_first {
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
            _ => {
                let p = Pattern::single_character(char);
                if let Some(Pattern::Any(group_chars, _)) = &mut current_group {
                    group_chars.push(p);
                } else {
                    patterns.push(p);
                }
            }
        }
        if is_first && !is_escaping {
            is_first = false;
        }
    }
    let mut position = 0;
    for p in patterns {
        if position == 0 && !match_start {
            if let Some(found_at) = p.first_match(&input_line) {
                position += found_at;
            } else {
                return false;
            }
        } else if !p.matches(&input_line[position..]) {
            return false;
        }
        position += 1;
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
