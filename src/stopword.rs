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

    pub fn stop_word(&self, sentence: &mut String) {
        let words = self.tokenizer.tokenize(sentence);
        let mut results: Vec<String> = Vec::new();

        for word in words.iter() {
            if !self.dictionary.find(word) {
                results.push(word.clone());
            }
        }

        *sentence = results.join(" ");
    }
}