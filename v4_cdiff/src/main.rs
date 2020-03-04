
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
struct WordGen<'a> {
    src: &'a str,
    curr_pos: usize,
    curr_word_pos: usize,
}

fn can_accumulate(curr: &[char], next: char) -> bool {
    match curr.get(0) {
        Some(c) => (get_char_type(*c) != CharType::Other) || (get_char_type(*c) != get_char_type(next)),
        None => true,
    }
}

impl WordGen<'_> {
    fn new<'a>(src: &'a str) -> WordGen<'a> {
        WordGen{src: src, curr_pos: 0, curr_word_pos: 0}
    }

    fn get<'a>(&'a self) -> &'a str {
        &self.src[self.curr_word_pos..self.curr_pos]
    }

    fn next(&mut self) {
        self.curr_word_pos = self.curr_pos;
        while !self.is_done() {
            match self.src.get(self.curr_pos) {
                Some(c) => if !can_accumulate(&self.src[self.curr_word_pos..self.curr_pos], *c) { return; },
                None => return,
            }
            self.curr += 1;
        }
    }

    fn is_done(&self) -> bool {
        self.curr_pos >= self.src.len()
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
    fn test_get_diff() {
        assert_eq!(get_diff("Hello", "World"), Diff{before: ["Hello"].to_vec(), after: ["World"].to_vec()});
        // assert_eq!(get_diff("Hello World", "Hello world"), Diff{before: ["Hello", " ", "World"].to_vec(), after: ["Hello", " ", "world"].to_vec()});
    }
}
