fn hello() -> String {
    "Hello World!".to_string()
}

fn is_id(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

#[derive(PartialEq)]
#[derive(Debug)]
enum CharType {
    Id,
    Space,
    Other,
    Uninitialized,
}

fn get_char_type(c: char) -> CharType {
    if is_id(c) {
        return CharType::Id;
    }
    if c.is_whitespace() {
        return CharType::Space;
    }
    CharType::Other
}

fn split_into_words(s: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut token: Vec<char> = Vec::new();
    let mut token_type = CharType::Uninitialized;

    for c in s.chars() {
        if token.is_empty() {
            token.push(c)
        }
    }
    tokens
}

fn main() {
    println!("{:?}", split_into_words(&hello()));
}

mod tests {
    use super::*;

    #[test]
    fn test_is_id() {
        for c in "aA0-_".to_string().chars() {
            assert!(is_id(c));
        }
        for c in " \t!()&".to_string().chars() {
            assert!(!is_id(c));
        }
    }

    #[test]
    fn test_get_char_type() {
        for c in "aA0-_".to_string().chars() {
            assert_eq!(get_char_type(c), CharType::Id);
        }
        for c in " \t".to_string().chars() {
            assert_eq!(get_char_type(c), CharType::Space);
        }
        for c in "!()&".to_string().chars() {
            assert_eq!(get_char_type(c), CharType::Other);
        }
    }
}
