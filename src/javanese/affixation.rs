use crate::javanese::affix_rules;
use crate::javanese::dictionary::JavaneseDictionary;

pub struct JavaneseAffixation<'a> {
    dictionary: &'a JavaneseDictionary,
}

impl<'a> JavaneseAffixation<'a> {
    pub fn new(dictionary: &'a JavaneseDictionary) -> JavaneseAffixation<'a> {
        JavaneseAffixation { dictionary }
    }

    pub fn remove_particle(&self, word: &str) -> Vec<(String, String)> {
        affix_rules::remove_particle(word)
    }

    pub fn remove_possessive(&self, word: &str) -> Vec<(String, String)> {
        affix_rules::remove_possessive(word)
    }

    /// Attempts to strip Javanese prefixes and validate root against the FST.
    ///
    /// It first evaluates general/Tripurusa prefixes, then evaluates the complex
    /// Anuswara mutations. The first validated root wins.
    pub fn remove_prefixes(&self, word: &str) -> (bool, String) {
        let std_results = affix_rules::remove_standard_prefixes(word);
        for root in std_results {
            if self.dictionary.find(&root) {
                return (true, root);
            }
        }

        let anu_results = affix_rules::remove_anuswara_prefixes(word);
        for root in anu_results {
            if self.dictionary.find(&root) {
                return (true, root);
            }
        }

        (false, word.to_string())
    }

    /// Tries combinations of prefixes when suffix stripping alone fails.
    /// This acts as the backtracking "pengembalian akhir" adapted for Javanese.
    pub fn pengembalian_akhir(
        &self,
        original_word: &str,
        removed_suffixes: &[String],
    ) -> (bool, String) {
        for suffix in removed_suffixes {
            if suffix.is_empty() {
                continue;
            }

            if original_word.ends_with(suffix) {
                let stripped_end = &original_word[..original_word.len() - suffix.len()];
                let (found, prefix_result) = self.remove_prefixes(stripped_end);

                if found {
                    return (true, prefix_result);
                }
            }
        }

        (false, original_word.to_string())
    }
}
