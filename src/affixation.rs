use crate::affix_rules;
use crate::dictionary::Dictionary;
use std::borrow::Cow;

pub struct Affixation<'a> {
    dictionary: &'a Dictionary,
}

impl<'a> Affixation<'a> {
    pub fn new(dict: &Dictionary) -> Affixation<'_> {
        Affixation { dictionary: dict }
    }

    /// Try to remove up to 3 prefix layers, returning the first root found in the dictionary.
    pub fn remove_prefixes(&self, word: &str) -> (bool, String) {
        let mut mutable_word = word.to_string();
        let mut removed_prefix = String::new();

        for _ in 0..3 {
            if mutable_word.len() < 3 {
                return (false, word.to_string());
            }

            if removed_prefix == mutable_word[..2] {
                break;
            }

            let (prefix, result, recoding_char) = self.remove_prefix(&mutable_word);
            removed_prefix = prefix;
            mutable_word = result;

            if self.dictionary.find(&mutable_word) {
                return (true, mutable_word);
            }

            for character in recoding_char {
                let recoded = format!("{}{}", character, mutable_word);
                if self.dictionary.find(&recoded) {
                    return (true, recoded);
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

    /// ECS Confix stripping: strips prefix+suffix simultaneously.
    /// Returns (found, root) where root is validated against the dictionary.
    ///
    /// Handles: ke-an, per-an, ber-an, me-kan, pe-an, ter-kan, se-nya
    pub fn remove_confix(&self, word: &str) -> Option<String> {
        let candidate = affix_rules::remove_confix(word)?;
        // Direct dictionary hit
        if self.dictionary.find(&candidate) {
            return Some(candidate);
        }
        // Try prefix removal on the confix candidate
        let (found, root) = self.remove_prefixes(&candidate);
        if found {
            return Some(root);
        }
        None
    }

    /// Pengembalian Akhir — backtrack through suffix combinations.
    pub fn pengembalian_akhir(&self, original_word: &str, suffixes: &[String]) -> (bool, String) {
        let mut len_suffixes: usize = 0;
        for suffix in suffixes {
            len_suffixes += suffix.len();
        }

        let base = &original_word[..original_word.len() - len_suffixes];

        for i in 0..suffixes.len() {
            let mut word = base.to_string();
            for j in 0..i {
                word.push_str(suffixes.get(j).unwrap());
            }

            if self.dictionary.find(&word) {
                return (true, word);
            }

            let (found, res) = self.remove_prefixes(&word);
            if found {
                return (true, res);
            }
        }

        (false, original_word.to_owned())
    }

    fn remove_prefix(&self, word: &str) -> (String, String, Vec<String>) {
        // kau- pronoun prefix
        if word.starts_with("kau") {
            return ("kau".to_string(), word[3..].to_string(), vec![]);
        }

        if word.len() < 2 {
            return (String::new(), word.to_string(), vec![]);
        }

        let prefix: String = word[..2].to_string();
        let (result_word, recoding) = match prefix.as_str() {
            "di" | "ke" | "se" | "ku" => (Cow::Borrowed(&word[2..]), vec![]),
            "me" => {
                let res = affix_rules::remove_prefix_me(word);
                (res.0, res.1)
            }
            "pe" => {
                let res = affix_rules::remove_prefix_pe(word);
                (res.0, res.1)
            }
            "be" => {
                let res = affix_rules::remove_prefix_be(word);
                (res.0, res.1)
            }
            "te" => {
                let res = affix_rules::remove_prefix_te(word);
                (res.0, res.1)
            }
            // nge- informal prefix (2020-2026 research)
            "ng" if word.starts_with("nge") => {
                let res = affix_rules::remove_prefix_nge(word);
                (res.0, res.1)
            }
            _ => {
                let res = affix_rules::remove_infix(word);
                (res.0, res.1)
            }
        };

        (prefix, result_word.into_owned(), recoding)
    }
}
