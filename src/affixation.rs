use crate::dictionary::Dictionary;
use crate::affix_rules;
use std::borrow::Cow;

pub struct Affixation<'a> {
    dictionary: &'a Dictionary,
}

impl<'a> Affixation<'a> {
    pub fn new(dict: &Dictionary) -> Affixation<'_> {
        Affixation {
            dictionary: dict,
        }
    }

    pub fn remove_prefixes(&self, word: &str) -> (bool, String) {
        let mut mutable_word = word.to_string();
        let mut removed_prefix = String::from("");

        for _ in 0..3 {
            if mutable_word.len() < 3 {
                return (false, word.to_string())
            }

            if removed_prefix == mutable_word[..2] {
                break
            }

            let (prefix, result, recoding_char) = self.remove_prefix(&mutable_word);
            removed_prefix = prefix;
            mutable_word = result;

            if self.dictionary.find(&mutable_word) {
                return (true, mutable_word)
            }

            for character in recoding_char {
                let mut char_word = character.to_string();
                char_word.push_str(&mutable_word);
                if self.dictionary.find(&char_word) {
                    return (true, char_word)
                }
            }
        }

        (false, mutable_word)
    }

    pub fn remove_particle<'b>(&self, word: &'b str) -> (Cow<'b, str>, Cow<'b, str>) {
        affix_rules::remove_particle(word)
    }

    pub fn remove_possessive<'b>(&self, word: &'b str) -> (Cow<'b, str>, Cow<'b, str>) {
        affix_rules::remove_possessive(word)
    }

    pub fn remove_suffix<'b>(&self, word: &'b str) -> (Cow<'b, str>, Cow<'b, str>) {
        affix_rules::remove_suffix(word)
    }

    pub fn pengembalian_akhir(&self, original_word: &str, suffixes: &[String]) -> (bool, String) {
        let mut len_suffixes: usize = 0;
        for suffix in suffixes {
            len_suffixes += suffix.len();
        }
        
        let mut word = original_word[..original_word.len()-len_suffixes].to_string();

        for i in 0..suffixes.len() {
            let mut suffix_combination = String::from("");
            for j in 0..i {
                suffix_combination.push_str(suffixes.get(j).unwrap());
            }

            word.push_str(&suffix_combination);
            if self.dictionary.find(&word) {
                return (true, word)
            }

            let (found, res) = self.remove_prefixes(&word);
            if found {
                return (true, res)
            }
        }

        (false, original_word.to_owned())
    }

    fn remove_prefix(&self, word: &str) -> (String, String, Vec<String>) {
        if word.starts_with("kau") {
            return ("kau".to_string(), word[3..].to_string(), vec![])
        }
        
        if word.len() < 2 {
            return ("".to_string(), word.to_string(), vec![]);
        }

        let prefix: String = word[..2].to_string();
        let mut result_word: Cow<str> = Cow::Borrowed(word);
        let mut recoding: Vec<String> = vec![];
        
        match prefix.as_str() {
            "di" | "ke" | "se" | "ku" => {
                result_word = Cow::Borrowed(&word[2..]);
            },
            "me" => {
                let res = affix_rules::remove_prefix_me(word);
                result_word = res.0;
                recoding = res.1;
            },
            "pe" => {
                let res = affix_rules::remove_prefix_pe(word);
                result_word = res.0;
                recoding = res.1;
            },
            "be" => {
                let res = affix_rules::remove_prefix_be(word);
                result_word = res.0;
                recoding = res.1;
            },
            "te" => {
                let res = affix_rules::remove_prefix_te(word);
                result_word = res.0;
                recoding = res.1;
            },
            _ => {
                let res = affix_rules::remove_infix(word);
                result_word = res.0;
                recoding = res.1;
            }
        }

        (prefix, result_word.into_owned(), recoding)
    }
}