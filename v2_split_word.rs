fn hello() -> String {
    "Hello World!".to_string()
}

fn is_sep(c: char) -> bool {
    c.is_whitespace()
}

fn split_into_words(s: &str) -> Vec<&str> {
    s.split(is_sep).collect()
}

fn main() {
    println!("{:?}", split_into_words(&hello()));
}
