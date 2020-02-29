fn hello() -> String {
    "Hello World!".to_string()
}

fn is_id(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

#[derive(PartialEq, Debug)]
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

fn split_into_words(s: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut token: Vec<char> = Vec::new();
    let mut token_type = CharType::Uninitialized;

    fn yield_token(token: &mut Vec<char>, tokens: &mut Vec<String>) {
        if !token.is_empty() {
            let token_str: String = token.iter().collect();
            tokens.push(token_str);
            token.clear();
        }
    }

    for c in s.chars() {
        if token_type == CharType::Other {
            yield_token(&mut token, &mut tokens);
            token_type = CharType::Uninitialized;
        }

        let new_token_type = get_char_type(c);
        if new_token_type != token_type {
            yield_token(&mut token, &mut tokens);
            token_type = new_token_type;
        }
        token.push(c);
    }

    yield_token(&mut token, &mut tokens);
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

    #[test]
    fn test_split_words() {
        assert_eq!(split_into_words("Hello World"), vec!["Hello", " ", "World"]);
        assert_eq!(
            split_into_words("Hello   World!!!"),
            vec!["Hello", "   ", "World", "!", "!", "!"]
        );
        assert_eq!(
            split_into_words("assert!(is_id(c))"),
            vec!["assert", "!", "(", "is_id", "(", "c", ")", ")"]
        );
        assert_eq!(
            split_into_words("if true {\n\treturn true;\n}"),
            vec!["if", " ", "true", " ", "{", "\n\t", "return", " ", "true", ";", "\n", "}"]
        );
    }
}
