use crate::dictionary::Dictionary;
use crate::tokenizer::Tokenizer;
use crate::affixation::Affixation;
use std::borrow::Cow;

pub struct Stemmer<'a> {
    dictionary: &'a Dictionary,
    tokenizer: Tokenizer,
    affixation: Affixation<'a>,
}

impl<'a> Stemmer<'a> {
    pub fn new(dictionary: &Dictionary) -> Stemmer<'_> {
        let tokenizer = Tokenizer::new();
        let affixation = Affixation::new(dictionary);
        Stemmer{
            dictionary: dictionary,
            tokenizer: tokenizer,
            affixation: affixation,
        }
    }

    pub fn stem_sentence<'b>(&'b self, sentence: &'b str) -> impl Iterator<Item = Cow<'b, str>> + 'b {
        self.tokenizer.tokenize(sentence).map(move |word| self.stem_word(word))
    }

    pub fn stem_word<'b>(&self, word: &'b str) -> Cow<'b, str> {
        // Fast path: if already in dictionary (case-insensitive find)
        if self.dictionary.find(word) {
            return Cow::Borrowed(word);
        }

        let mut current_word = word.to_lowercase();
        if current_word.chars().count() < 3 || self.dictionary.find(&current_word) {
            return Cow::Owned(current_word);
        }

        // Nazief-Adriani Algorithm logic
        // Step 1: Remove Particle (-lah, -kah, -tah, -pun)
        let (particle, res1) = self.affixation.remove_particle(&current_word);
        if !particle.is_empty() {
             let res_str = res1.into_owned();
             if self.dictionary.find(&res_str) {
                 return Cow::Owned(res_str);
             }
             current_word = res_str;
        }

        // Step 2: Remove Possessive (-ku, -mu, -nya)
        let (possessive, res2) = self.affixation.remove_possessive(&current_word);
        if !possessive.is_empty() {
            let res_str = res2.into_owned();
            if self.dictionary.find(&res_str) {
                return Cow::Owned(res_str);
            }
            current_word = res_str;
        }

        // Step 3: Remove Prefixes
        let (prefix_removed, res3) = self.affixation.remove_prefixes(&current_word);
        if prefix_removed {
            return Cow::Owned(res3);
        }

        Cow::Owned(current_word)
    }
}