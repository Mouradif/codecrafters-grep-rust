use std::env;
use std::io;
use std::process;
use codecrafters_grep::matchers::*;
use codecrafters_grep::patterns::*;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        _ if is_empty(pattern) => false,
        _ if is_single_char(pattern) => match_char(input_line, pattern),
        _ if is_digit(pattern) => match_digit(input_line),
        _ if is_wordlike(pattern) => match_wordlike(input_line),
        _ if is_positive_group(pattern) => match_positive_group(input_line, &pattern[1..pattern.len() - 1]),
        _ => panic!("Unhandled pattern: {}", pattern),
    }
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
