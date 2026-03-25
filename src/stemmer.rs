use crate::dictionary::Dictionary;
use crate::tokenizer::Tokenizer;
use crate::affixation::Affixation;

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

    pub fn stem_sentence(&self, sentence: &String) -> Vec<String> {
        let words = self.tokenizer.tokenize(sentence);
        let mut results : Vec<String> = Vec::new();
        
        for word in &mut words.iter() {
            let mut word = word.clone();
            self.stem_word(&mut word);
            results.push(word)
        }

        results
    }

    pub fn stem_word(&self, word: &mut String) {
        let original_word = word.clone();
        let mut _particle = String::from("");
        let mut _possesive = String::from("");
        let mut _suffix = String::from("");
        
        *word = word.to_lowercase();
        if word.chars().count() < 3 {
            return
        }

        if self.dictionary.find(word) {
            return
        }

        if self.affixation.prefix_first.is_match(word) {
            let (found, found_word) = self.affixation.remove_prefixes(word);
            *word = found_word;
            if found {
                return
            }

            let (found_particle, found_word) = self.affixation.remove_particle(word);
            _particle = found_particle;
            *word = found_word;
            if self.dictionary.find(word) {
                return
            }

            let (found_possesive, found_word) = self.affixation.remove_possesive(word);
            _possesive = found_possesive;
            *word = found_word;
            if self.dictionary.find(word) {
                return
            }

            let (found_suffix, found_word) = self.affixation.remove_suffix(word);
            _suffix = found_suffix;
            *word = found_word;
            if self.dictionary.find(word) {
                return
            }
        } else {
            let (found_particle, found_word) = self.affixation.remove_particle(word);
            _particle = found_particle;
            *word = found_word;
            if self.dictionary.find(word) {
                return
            }

            let (found_possesive, found_word) = self.affixation.remove_possesive(word);
            _possesive = found_possesive;
            *word = found_word;
            if self.dictionary.find(word) {
                return
            }

            let (found_suffix, found_word) = self.affixation.remove_suffix(word);
            _suffix = found_suffix;
            *word = found_word;
            if self.dictionary.find(word) {
                return
            }

            let (found, found_word) = self.affixation.remove_prefixes(word);
            *word = found_word;
            if found {
                return
            }
        }

        let mut removed_suffixes = vec![String::from(""), _suffix.clone(), _possesive.clone(), _particle.clone()];
        if _suffix == "kan" {
            removed_suffixes = vec![String::from(""), String::from("k"), String::from("an"), _possesive, _particle];
        }
        
        let (found, found_word) = self.affixation.pengembalian_akhir(&original_word, &removed_suffixes);
        *word = found_word;
        if found {
            return
        }

        *word = original_word
    }
}