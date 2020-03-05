
// Chars
#[derive(PartialEq, Debug)]
enum CharType {
    Id,
    White,
    Other,
    Uninitialized,
}

fn is_id(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

fn get_char_type(c: char) -> CharType {
    if is_id(c) {
        return CharType::Id;
    }
    if c.is_whitespace() {
        return CharType::White;
    }
    CharType::Other
}

// Words
struct Words<'a> {
    src: &'a str,
    chars: Box<dyn Iterator<Item = (usize, char)> + 'a>,
    last_pos: usize,
    is_same_word: fn(&str, char) -> bool,
}

impl<'a> Words<'a> {
    fn new(s: & str, is_same_word: fn(&str, char) -> bool) -> Words<> {
        Words{src: s, chars: Box::new(s.chars().enumerate()), last_pos: 0, is_same_word: is_same_word}
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Next word
        while let Some((i, c)) = self.chars.next() {
            let current_word = &self.src[self.last_pos..i];
            if !(self.is_same_word)(&current_word, c) {
                self.last_pos = i;
                return Some(&current_word);
            }
        }

        // Last word
        if self.last_pos < self.src.len() {
            let current_word = &self.src[self.last_pos..];
            self.last_pos = self.src.len();
            return Some(&current_word);
        }

        // End
        None
    }
}

fn is_same_word(curr: &str, next: char) -> bool {
    match curr.chars().next() {
        Some(c) => (get_char_type(c) != CharType::Other) && (get_char_type(c) == get_char_type(next)),
        None => true,
    }
}

// Fragments

// Diff
#[derive(PartialEq, Debug)]
struct Diff<'a, 'b> {
    before: Vec<&'a str>,
    after: Vec<&'b str>
}

fn get_diff<'a, 'b>(s1: &'a str, s2: &'b str) -> Diff<'a, 'b> {
    Diff{before: vec![s1], after: vec![s2]}
}

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn test_words() {
        fn test(input: &str, output_ref: Vec<&str>) {
            let words = Words::new(input, is_same_word);
            let output: Vec<&str> = words.collect();
            assert_eq!(output, output_ref);
        }

        test("", [].to_vec());
        test("Hello", ["Hello"].to_vec());
        test("Hello World", ["Hello", " ", "World"].to_vec());
        test("Hello   World", ["Hello", "   ", "World"].to_vec());
        test("Hello&World", ["Hello", "&", "World"].to_vec());
        test("Hello&!World", ["Hello", "&", "!", "World"].to_vec());
    }

    #[test]
    fn test_get_diff() {
        assert_eq!(get_diff("Hello", "World"), Diff{before: ["Hello"].to_vec(), after: ["World"].to_vec()});
        // assert_eq!(get_diff("Hello World", "Hello world"), Diff{before: ["Hello", " ", "World"].to_vec(), after: ["Hello", " ", "world"].to_vec()});
    }
}
