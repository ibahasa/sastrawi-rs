pub struct Tokenizer;

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer
    }

    pub fn tokenize<'a>(&self, sentence: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        // Simple but very fast zero-copy tokenizer
        sentence.split_whitespace().map(|s| {
            // Trim typical Indonesian punctuation
            s.trim_matches(|c: char| !c.is_alphanumeric())
        })
    }
}