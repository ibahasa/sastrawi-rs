/// A zero-copy tokenizer for Indonesian text.
///
/// It splits text into individual words while cleaning up common punctuation marks.
pub struct Tokenizer;

impl Tokenizer {
    /// Creates a new `Tokenizer` instance.
    pub fn new() -> Tokenizer {
        Tokenizer
    }

    /// Tokenizes a sentence into individual word slices.
    ///
    /// It returns an iterator over `&str` slices, avoiding unnecessary
    /// memory allocations.
    ///
    /// # Example
    /// ```
    /// use sastrawi::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new();
    /// let tokens: Vec<_> = tokenizer.tokenize("Saya Makan: Nasi!").collect();
    /// assert_eq!(tokens, vec!["Saya", "Makan", "Nasi"]);
    /// ```
    pub fn tokenize<'a>(&self, sentence: &'a str) -> impl Iterator<Item = &'a str> {
        sentence.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
                .filter(|s| !s.is_empty())
    }
}