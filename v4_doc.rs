#![deny(missing_docs)]
//! A test

/// Simple string generator
fn hello() -> String {
    "Hello World!".to_string()
}

fn main() {
    println!("{}", hello());
}
