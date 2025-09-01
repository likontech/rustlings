fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn uppercase_me(input: &str) -> String {
    // TODO: Convert the string to uppercase.
    input.to_uppercase()
}

fn lowercase_me(input: &str) -> String {
    // TODO: Convert the string to lowercase.
    input.to_lowercase()
}

fn contain_me(input: &str) -> bool {
    // TODO: Check if the string contains "balloons".
    input.contains("balloons")
}

fn start_with_me(input: &str) -> bool {
    // TODO: Check if the string starts with "Hello".
    input.starts_with("Hello")
}

fn end_with_me(input: &str) -> bool {
    // TODO: Check if the string ends with "world!".
    input.ends_with("world!")
}

fn split_me(input: &str, delimiter: &str) -> Vec<String> {
    // TODO: Split the string into a vector of strings at each space.
    input.split(delimiter).map(String::from).collect()
}

fn join_me(input: Vec<String>, separator: &str) -> String {
    // TODO: Join the vector of strings into a single string, separated by spaces.
    input.join(separator)
}

fn repeat_me(input: &str, times: usize) -> String {
    // TODO: Repeat string n times
    input.repeat(times)
}

fn reverse_me(input: &str) -> String {
    // TODO: Reverse string
    input.chars().rev().collect()
}

fn substring_me(input: &str, start: usize, end: usize) -> &str {
    // TODO: Get substring from start to end index
    &input[start..end]
}

fn char_count_me(input: &str) -> usize {
    // TODO: Count characters in string
    input.chars().count()
}

fn byte_count_me(input: &str) -> usize {
    // TODO: Count bytes in string
    input.len()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }

    #[test]
    fn uppercase_a_string() {
        assert_eq!(uppercase_me("Hello"), "HELLO");
        assert_eq!(uppercase_me("Goodbye"), "GOODBYE");
    }

    #[test]
    fn lowercase_a_string() {
        assert_eq!(lowercase_me("HELLO"), "hello");
        assert_eq!(lowercase_me("GOODBYE"), "goodbye");
    }

    #[test]
    fn contain_a_string() {
        assert!(contain_me("I think balloons are cool"));
        assert!(contain_me("I love to look at balloons"));
    }

    #[test]
    fn start_with_a_string() {
        assert!(start_with_me("Hello, world!"));
        assert!(!start_with_me("Goodbye, world!"));
    }

    #[test]
    fn end_with_a_string() {
        assert!(end_with_me("Hello, world!"));
        assert!(!end_with_me("Hello, Rust!"));
    }

    #[test]
    fn split_a_string() {
        assert_eq!(
            split_me("Hello,world!", ","),
            vec!["Hello", "world!"],
        );
        assert_eq!(
            split_me("I love to look at balloons", " "),
            vec!["I", "love", "to", "look", "at", "balloons"],
        );
    }

    #[test]
    fn join_a_string() {
        assert_eq!(join_me(vec!["Hello".to_string(), "world!".to_string()], ","), "Hello,world!");
        assert_eq!(
            join_me(vec!["I".to_string(), "love".to_string(), "to".to_string(), "look".to_string(), "at".to_string(), "balloons".to_string()], " "),
            "I love to look at balloons",
        );
    }

    #[test]
    fn repeat_a_string() {
        assert_eq!(repeat_me("Hello", 3), "HelloHelloHello");
        assert_eq!(repeat_me("Goodbye", 2), "GoodbyeGoodbye");
    }

    #[test]
    fn reverse_a_string() {
        assert_eq!(reverse_me("Hello"), "olleH");
        assert_eq!(reverse_me("Goodbye"), "eybdooG");
    }

    #[test]
    fn count_characters_in_a_string() {
        assert_eq!(char_count_me("Hello"), 5);
        assert_eq!(char_count_me("Goodbye"), 7);
    }

    #[test]
    fn count_bytes_in_a_string() {
        assert_eq!(byte_count_me("Hello"), 5);
        assert_eq!(byte_count_me("Goodbye"), 7);
    }

    #[test]
    fn substring_a_string() {
        assert_eq!(substring_me("Hello, world!", 7, 12), "world");
        assert_eq!(substring_me("I love to look at balloons", 7, 14), "to look");
    }
}
