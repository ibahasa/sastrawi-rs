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

        let original_word = word.to_lowercase();
        if original_word.chars().count() < 3 || self.dictionary.find(&original_word) {
            return Cow::Owned(original_word);
        }

        let mut current_word = original_word.clone();
        let mut _particle = String::new();
        let mut _possessive = String::new();
        let mut _suffix = String::new();

        // Step 1: Remove Particle (-lah, -kah, -tah, -pun)
        let (p, res1) = self.affixation.remove_particle(&current_word);
        if !p.is_empty() {
             _particle = p.into_owned();
             current_word = res1.into_owned();
             if self.dictionary.find(&current_word) {
                 return Cow::Owned(current_word);
             }
        }

        // Step 2: Remove Possessive (-ku, -mu, -nya)
        let (pos, res2) = self.affixation.remove_possessive(&current_word);
        if !pos.is_empty() {
            _possessive = pos.into_owned();
            current_word = res2.into_owned();
            if self.dictionary.find(&current_word) {
                return Cow::Owned(current_word);
            }
        }

        // Step 3 & 4: Remove Suffix and Prefixes
        let (s, res3) = self.affixation.remove_suffix(&current_word);
        if !s.is_empty() {
            _suffix = s.into_owned();
            let word_before_suffix = current_word.clone();
            current_word = res3.into_owned();
            if self.dictionary.find(&current_word) {
                return Cow::Owned(current_word);
            }
            
            // Try removing prefixes after suffix
            let (prefix_removed, res4) = self.affixation.remove_prefixes(&current_word);
            if prefix_removed {
                return Cow::Owned(res4);
            }
            // If not found, revert suffix for now (Nazief-Adriani backtracking)
            // But usually we continue to pengembalian_akhir
            current_word = word_before_suffix;
        }

        // Step 5: Final Prefix removal (if suffix removal didn't yield base)
        let (prefix_removed, res5) = self.affixation.remove_prefixes(&current_word);
        if prefix_removed {
            return Cow::Owned(res5);
        }

        // Final Step: Pengembalian Akhir (Backtracking)
        let mut removed_suffixes = vec![String::from(""), _suffix.clone(), _possessive.clone(), _particle.clone()];
        if _suffix == "kan" {
            removed_suffixes = vec![String::from(""), String::from("k"), String::from("an"), _possessive, _particle];
        }
        
        let (found, res_backtrack) = self.affixation.pengembalian_akhir(&original_word, &removed_suffixes);
        if found {
            return Cow::Owned(res_backtrack);
        }

        Cow::Owned(original_word)
    }
}