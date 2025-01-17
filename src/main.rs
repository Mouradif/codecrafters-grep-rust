use std::env;
use std::io;
use std::process;
use codecrafters_grep::patterns::match_pattern;

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

#[cfg(test)]
mod tests {
    use codecrafters_grep::patterns::match_pattern;

    #[test]
    fn test_single_char() {
        assert!(match_pattern("dog", "d"));
        assert!(!match_pattern("dog", "f"));
    }

    #[test]
    fn test_digit() {
        assert!(match_pattern("123", "\\d"));
        assert!(!match_pattern("apple", "\\d"));
    }

    #[test]
    fn test_wordlike() {
        assert!(match_pattern("word", "\\w"));
        assert!(!match_pattern("$!?", "\\w"));
    }

    #[test]
    fn test_positive_group() {
        assert!(match_pattern("a", "[abcd]"));
        assert!(!match_pattern("efgh", "[abcd]"));
    }

    #[test]
    fn test_negative_group() {
        assert!(match_pattern("apple", "[^xyz]"));
        assert!(!match_pattern("banana", "[^anb]"));
    }

    #[test]
    fn test_combining() {
        assert!(match_pattern("sally has 3 apples", "\\d apple"));
        assert!(!match_pattern("sally has 1 orange", "\\d apple"));
        assert!(match_pattern("sally has 124 apples", "\\d\\d\\d apples"));
        assert!(!match_pattern("sally has 12 apples", "\\d\\d\\d apples"));
        assert!(match_pattern("sally has 3 dogs", "\\d \\w\\w\\ws"));
        assert!(match_pattern("sally has 4 dogs", "\\d \\w\\w\\ws"));
        assert!(!match_pattern("sally has 1 dog", "\\d \\w\\w\\ws"));
    }

    #[test]
    fn test_start_anchor() {
        assert!(match_pattern("log", "^log"));
        assert!(!match_pattern("slog", "^log"));
        assert!(match_pattern("Hey ^log", " ^log"));
        assert!(match_pattern("^log", "^^log"));
        assert!(match_pattern("^log", "\\^log"));
    }

    #[test]
    fn test_end_anchor() {
        assert!(match_pattern("cat", "cat$"));
        assert!(!match_pattern("cats", "cat$"));
        assert!(match_pattern("For $5 only!", "$\\d only!"));
        assert!(match_pattern("For $5 only!", "$\\d only!$"));
        assert!(match_pattern("For $5 only!", "[%?$]"));
    }

    #[test]
    fn test_one_or_more() {
        assert!(match_pattern("caaats", "ca+t"));
        assert!(match_pattern("cat", "ca+t"));
        assert!(!match_pattern("act", "ca+t"));
        assert!(!match_pattern("ca", "ca+t"));
    }

    #[test]
    fn test_zero_or_one() {
        assert!(match_pattern("cat", "ca?t"));
        assert!(match_pattern("act", "ca?t"));
        assert!(!match_pattern("dog", "ca?t"));
        assert!(!match_pattern("cag", "ca?t"));
    }

    #[test]
    fn test_wildcard() {
        assert!(match_pattern("cat", "c.t"));
        assert!(!match_pattern("car", "c.t"));
        assert!(match_pattern("goøö0Ogol", "g.+gol"));
        assert!(!match_pattern("gol", "g.+gol"));
    }

    #[test]
    fn test_choice() {
        assert!(match_pattern("a cat", "a (cat|dog)"));
        assert!(match_pattern("a dog and cats", "a (cat|dog) and (cat|dog)s"));
        assert!(match_pattern("a cat and dogs", "a (cat|dog) and (cat|dog)s"));
    }

    #[test]
    fn test_capture() {
        assert!(match_pattern("cat and cat", "(cat) and \\1"));
        assert!(!match_pattern("cat and dog", "(cat) and \\1"));
        assert!(match_pattern("grep 101 is doing grep 101 times", "(\\w\\w\\w\\w \\d\\d\\d) is doing \\1 times"));
        assert!(!match_pattern("$?! 101 is doing $?! 101 times", "(\\w\\w\\w\\w \\d\\d\\d) is doing \\1 times"));
        assert!(!match_pattern("grep yes is doing grep yes times", "(\\w\\w\\w\\w \\d\\d\\d) is doing \\1 times"));
        assert!(match_pattern("abcd is abcd, not efg", "([abcd]+) is \\1, not [^xyz]+"));
        assert!(!match_pattern("efgh is efgh, not efg", "([abcd]+) is \\1, not [^xyz]+"));
        assert!(!match_pattern("abcd is abcd, not xyz", "([abcd]+) is \\1, not [^xyz]+"));
        assert!(match_pattern("this starts and ends with this", "^(\\w+) starts and ends with \\1$"));
        assert!(!match_pattern("that starts and ends with this", "^(\\w+) starts and ends with \\1$"));
        assert!(!match_pattern("that starts and ends with this", "^(this) starts and ends with \\1$"));
        assert!(!match_pattern("this starts and ends with this?", "^(this) starts and ends with \\1$"));
        assert!(match_pattern("once a dreaaamer, always a dreaaamer", "once a (drea+mer), alwaysz? a \\1"));
        assert!(!match_pattern("once a dreaamer, always a dreaaamer", "once a (drea+mer), alwaysz? a \\1"));
        assert!(!match_pattern("once a dremer, always a dreaaamer", "once a (drea+mer), alwaysz? a \\1"));
        assert!(!match_pattern("once a dreaaamer, alwayszzz a dreaaamer", "once a (drea+mer), alwaysz? a \\1"));
        assert!(match_pattern("bugs here and bugs there", "(b..s|c..e) here and \\1 there"));
        assert!(!match_pattern("bugz here and bugs there", "(b..s|c..e) here and \\1 there"));
    }

    #[test]
    fn test_multi_captures() {
        assert!(match_pattern("3 red squares and 3 red circles", "(\\d+) (\\w+) squares and \\1 \\2 circles"));
        assert!(!match_pattern("3 red squares and 4 red circles", "(\\d+) (\\w+) squares and \\1 \\2 circles"));
        assert!(match_pattern("grep 101 is doing grep 101 times", "(\\w\\w\\w\\w) (\\d\\d\\d) is doing \\1 \\2 times"));
        assert!(!match_pattern("$?! 101 is doing $?! 101 times", "(\\w\\w\\w) (\\d\\d\\d) is doing \\1 \\2 times"));
        assert!(!match_pattern("grep yes is doing grep yes times", "(\\w\\w\\w\\w) (\\d\\d\\d) is doing \\1 \\2 times"));
        assert!(match_pattern("abc-def is abc-def, not efg", "([abc]+)-([def]+) is \\1-\\2, not [^xyz]+"));
        assert!(!match_pattern("efg-hij is efg-hij, not efg", "([abc]+)-([def]+) is \\1-\\2, not [^xyz]+"));
        assert!(!match_pattern("abc-def is abc-def, not xyz", "([abc]+)-([def]+) is \\1-\\2, not [^xyz]+"));
        assert!(match_pattern("apple pie, apple and pie", "^(\\w+) (\\w+), \\1 and \\2$"));
        assert!(!match_pattern("pineapple pie, pineapple and pie", "^(apple) (\\w+), \\1 and \\2$"));
        assert!(!match_pattern("apple pie, apple and pies", "^(\\w+) (pie), \\1 and \\2$"));
        assert!(match_pattern("howwdy hey there, howwdy hey", "(how+dy) (he?y) there, \\1 \\2"));
        assert!(!match_pattern("hody hey there, howwdy hey", "(how+dy) (he?y) there, \\1 \\2"));
        assert!(!match_pattern("howwdy heeey there, howwdy heeey", "(how+dy) (he?y) there, \\1 \\2"));
        assert!(match_pattern("cat and fish, cat with fish", "(c.t|d.g) and (f..h|b..d), \\1 with \\2"));
        assert!(!match_pattern("bat and fish, cat with fish", "(c.t|d.g) and (f..h|b..d), \\1 with \\2"));
    }
}