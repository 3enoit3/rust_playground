
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

    let mut yield_token = |token: &[char]| {
        if !token.is_empty() {
            let token_str: String = token.iter().collect();
            tokens.push(token_str);
        }
    };

    for c in s.chars() {
        if token_type == CharType::Other {
            yield_token(&token);
            token.clear();
            token_type = CharType::Uninitialized;
        }

        let new_token_type = get_char_type(c);
        if new_token_type != token_type {
            yield_token(&token);
            token.clear();
            token_type = new_token_type;
        }
        token.push(c);
    }

    yield_token(&token);
    tokens
}

fn diff_words(s1: &[String], s2: &[String]) -> bool {
    if s1 != s2 {
        if s1.len() != s2.len() {
            println!("{:?} and {:?} have different size", s1, s2);
            return false;
        }
        for (w1, w2) in s1.iter().zip(s2) {
            if w1 != w2 {
                println!("{} and {} differ", w1, w2);
                return false;
            }
        }
    }
    s1 == s2
}

fn main() {
    assert!(diff_words(&split_into_words("Hello World"), &split_into_words("Hello World")));
    assert!(diff_words(&split_into_words(""), &split_into_words("")));
    assert!(!diff_words(&split_into_words("Hello World"), &split_into_words("Hello World!")));
    assert!(!diff_words(&split_into_words("Hello Wind"), &split_into_words("Hello World")));
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

    #[test]
    fn test_diff_words() {
        assert!(diff_words(&split_into_words("Hello World"), &split_into_words("Hello World")));
        assert!(diff_words(&split_into_words(""), &split_into_words("")));
        assert!(!diff_words(&split_into_words("Hello World"), &split_into_words("Hello World!")));
        assert!(!diff_words(&split_into_words("Hello Wind"), &split_into_words("Hello World")));
    }
}
