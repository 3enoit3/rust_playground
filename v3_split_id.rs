fn hello() -> String {
    "Hello World!".to_string()
}

fn is_id(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

fn split_into_words(s: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut token: Vec<char> = Vec::new();
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
