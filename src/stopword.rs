use crate::dictionary::Dictionary;
use crate::tokenizer::Tokenizer;

/// Legacy stopword filter used for stripping common function words from sentences.
///
/// It's generally recommended to use `Stemmer::stem_sentence_filtered` which
/// combines both stemming and stopword filtering in a single pass.
pub struct StopWord<'a> {
    dictionary: &'a Dictionary,
    tokenizer: Tokenizer,
}

impl<'a> StopWord<'a> {
    /// Creates a new `StopWord` instance with the provided stopword dictionary.
    pub fn new(dictionary: &Dictionary) -> StopWord<'_> {
        let tokenizer = Tokenizer::new();
        StopWord {
            dictionary: dictionary,
            tokenizer: tokenizer,
        }
    }

    /// Filters and joins words from a sentence by removing anything
    /// found in the stopword dictionary.
    ///
    /// # Example
    /// ```
    /// use sastrawi::{Dictionary, StopWord};
    ///
    /// let dict = Dictionary::stopword();
    /// let stopword = StopWord::new(&dict);
    ///
    /// let sentence = "Pertumbuhan yang membanggakan";
    /// let filtered = stopword.stop_word(sentence);
    /// // → "Pertumbuhan membanggakan"
    /// ```
    pub fn stop_word(&self, sentence: &str) -> String {
        let words = self.tokenizer.tokenize(sentence);
        let mut results: Vec<String> = Vec::new();

        for word in words {
            if !self.dictionary.find(word) {
                results.push(word.to_string());
            }
        }

        results.join(" ")
    }
}
