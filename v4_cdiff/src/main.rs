
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
    is_same_word: fn(&str, char) -> bool,
    last_pos: usize,
}

impl<'a> Words<'a> {
    fn new(s: &str, is_same_word: fn(&str, char) -> bool) -> Words {
        let chars = Box::new(s.chars().enumerate());
        Words{src: s, chars: chars, is_same_word: is_same_word, last_pos: 0}
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
struct SizedFragments<'a> {
    src: &'a [&'a str],
    size: usize,
    curr_pos: usize,
}

impl<'a> SizedFragments<'a> {
    fn new<'b>(words: &'b [&'b str], size: usize) -> SizedFragments<'b> {
        SizedFragments{src: words, size: size, curr_pos: 0}
    }
}

impl<'a> Iterator for SizedFragments<'a> {
    type Item = &'a [&'a str];

    fn next(&mut self) -> Option<Self::Item> {
        let last = self.curr_pos + self.size;
        if last > self.src.len() {
            return None;
        }
        let fragment = &self.src[self.curr_pos..last];
        self.curr_pos += 1;
        Some(fragment)
    }
}

struct Fragments<'a> {
    src: &'a [&'a str],
    curr_size: Box<SizedFragments<'a>>,
}

impl<'a> Fragments<'a> {
    fn new<'b>(words: &'b [&'b str]) -> Fragments<'b> {
        let largest_fragment = Box::new(SizedFragments::new(words, words.len()));
        Fragments{src: words, curr_size: largest_fragment}
    }
}

impl<'a> Iterator for Fragments<'a> {
    type Item = &'a [&'a str];

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_size.next() {
            None => {
                if self.curr_size.size > 1 {
                    self.curr_size = Box::new(SizedFragments::new(self.src, self.curr_size.size - 1));
                    return self.curr_size.next();
                }
                None
            },
            Some(c) => Some(c),
        }
    }
}

fn contains<'a, 'b> (small: &[&str], big: &'a [&'b str]) -> Option<(&'a [&'b str], &'a [&'b str], &'a [&'b str])> {
    for i in 0..big.len()-small.len()+1 {
        let subset = &big[i..i+small.len()];
        if subset == small {
            let before = &big[..i];
            let after = &big[i+small.len()..];
            return Some((before, subset, after));
        }
    }
    None
}

// Diff
#[derive(PartialEq, Debug)]
struct DiffChunk<'a, 'b> {
    old: Vec<&'a str>,
    new: Vec<&'b str>
}

impl<'a, 'b> DiffChunk<'a, 'b> {
    fn is_same(&self) -> bool {
        self.old == self.new
    }

    fn is_diff(&self) -> bool {
        !self.old.is_empty() && !self.new.is_empty() && self.old != self.new
    }

    fn is_del(&self) -> bool {
        !self.old.is_empty() && self.new.is_empty()
    }

    fn is_add(&self) -> bool {
        self.old.is_empty() && !self.new.is_empty()
    }

}

fn diff<'a, 'b>(old: &'a str, new: &'b str) -> Vec<DiffChunk<'a, 'b>> {
    let old_words: Vec<&str> = Words::new(old, is_same_word).collect();
    let new_words: Vec<&str> = Words::new(new, is_same_word).collect();

    if old_words.len() > new_words.len() {
        let small = &new_words;
        let big = &old_words;
    }
    else {
        let small = &old_words;
        let big = &new_words;
    }

    //for f in Fragments::new(small) {
    //}
    vec!()
}

fn build_test_diff<'a>(s: &'a str) -> Vec<DiffChunk<'a, 'a>> {
    let mut chunks = Vec::new();
    for chunk in s.split('|') {
        if chunk.contains(">") {
            let fragments: Vec<&str> = chunk.split('>').collect();
            let old = Words::new(fragments[0], is_same_word).collect();
            let new = Words::new(fragments[1], is_same_word).collect();
            chunks.push(DiffChunk{old: old, new: new});
        }
        else {
            let same: Vec<&str> = Words::new(chunk, is_same_word).collect();
            chunks.push(DiffChunk{old: same.clone(), new: same});
        }
    }
    chunks
}

// Main
fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn test_words() {
        fn test(input: &str, expected: Vec<&str>) {
            let it = Words::new(input, is_same_word);
            let output: Vec<&str> = it.collect();
            assert_eq!(output, expected);
        }

        test("", vec!());
        test("Hello", vec!("Hello"));
        test("Hello World", vec!("Hello", " ", "World"));
        test("Hello   World", vec!("Hello", "   ", "World"));
        test("Hello&World", vec!("Hello", "&", "World"));
        test("Hello&!World", vec!("Hello", "&", "!", "World"));
    }

    #[test]
    fn test_sized_fragments() {
        fn test(input: Vec<u8>, size: usize, expected: Vec<Vec<u8>>) {
            let owned_input: Vec<String> = input.iter().map(|x| x.to_string()).collect();
            let it_input: Vec<&str> = owned_input.iter().map(|x| &x[..]).collect();
            let it = SizedFragments::new(&it_input, size);
            let it_output: Vec<Vec<u8>> = it.map(|c| c.iter().map(|i| i.parse::<u8>().unwrap()).collect()).collect();
            assert_eq!(it_output, expected);
        }

        test(vec!(1,2,3,4), 1, [vec!(1), vec!(2), vec!(3), vec!(4)].to_vec());
        test(vec!(1,2,3,4), 2, [vec!(1,2), vec!(2,3), vec!(3,4)].to_vec());
        test(vec!(1,2,3,4), 3, [vec!(1,2,3), vec!(2,3,4)].to_vec());
        test(vec!(1,2,3,4), 4, [vec!(1,2,3,4)].to_vec());
    }

    #[test]
    fn test_fragments() {
        fn test(input: Vec<u8>, expected: Vec<Vec<u8>>) {
            let owned_input: Vec<String> = input.iter().map(|x| x.to_string()).collect();
            let it_input: Vec<&str> = owned_input.iter().map(|x| &x[..]).collect();
            let it = Fragments::new(&it_input);
            let it_output: Vec<Vec<u8>> = it.map(|c| c.iter().map(|i| i.parse::<u8>().unwrap()).collect()).collect();
            assert_eq!(it_output, expected);
        }

        test(vec!(1,2,3,4), [
            vec!(1,2,3,4),
            vec!(1,2,3), vec!(2,3,4),
            vec!(1,2), vec!(2,3), vec!(3,4),
            vec!(1), vec!(2), vec!(3), vec!(4),
        ].to_vec());
    }

    #[test]
    fn test_contains() {
        assert_eq!(contains(&vec!("2")[..], &vec!("1", "2", "3")[..]), Some((&vec!("1")[..], &vec!("2")[..], &vec!("3")[..])));
        assert_eq!(contains(&vec!("1")[..], &vec!("1", "2", "3")[..]), Some((&vec!()[..], &vec!("1")[..], &vec!("2", "3")[..])));
        assert_eq!(contains(&vec!("3")[..], &vec!("1", "2", "3")[..]), Some((&vec!("1", "2")[..], &vec!("3")[..], &vec!()[..])));
        assert_eq!(contains(&vec!("4")[..], &vec!("1", "2", "3")[..]), None);
    }

    #[test]
    fn test_diff_chuncks() {
        assert!(DiffChunk{old:vec!("Hello"), new:vec!("Hello")}.is_same());
        assert!(DiffChunk{old:vec!("Hello"), new:vec!("World")}.is_diff());
        assert!(DiffChunk{old:vec!("Hello"), new:vec!()}.is_del());
        assert!(DiffChunk{old:vec!(), new:vec!("World")}.is_add());

        assert_eq!(build_test_diff("Hello"), vec!(DiffChunk{old: vec!("Hello"), new: vec!("Hello")}));
        assert_eq!(build_test_diff("Hello>World"), vec!(DiffChunk{old: vec!("Hello"), new: vec!("World")}));
        assert_eq!(build_test_diff("Hello>"), vec!(DiffChunk{old: vec!("Hello"), new: vec!()}));
        assert_eq!(build_test_diff(">Hello"), vec!(DiffChunk{old: vec!(), new: vec!("Hello")}));
        assert_eq!(build_test_diff("Hello>World|!"), vec!(DiffChunk{old: vec!("Hello"), new: vec!("World")}, DiffChunk{old: vec!("!"), new: vec!("!")}));
        assert_eq!(build_test_diff("Hello>World|!>"), vec!(DiffChunk{old: vec!("Hello"), new: vec!("World")}, DiffChunk{old: vec!("!"), new: vec!()}));
    }
}
