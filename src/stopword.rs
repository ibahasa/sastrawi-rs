use crate::dictionary::Dictionary;
use crate::tokenizer::Tokenizer;

pub struct StopWord<'a> {
    dictionary: &'a Dictionary,
    tokenizer: Tokenizer,
}

impl<'a> StopWord<'a> {
    pub fn new(dictionary: &Dictionary) -> StopWord<'_> {
        let tokenizer = Tokenizer::new();
        StopWord{
            dictionary: dictionary,
            tokenizer: tokenizer,
        }
    }

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