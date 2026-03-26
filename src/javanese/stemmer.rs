use crate::javanese::affixation::JavaneseAffixation;
use crate::javanese::dictionary::JavaneseDictionary;
use crate::tokenizer::Tokenizer;
use std::borrow::Cow;

/// The main Universal Javanese word stemmer (Ngoko, Krama Alus, Krama Inggil).
///
/// Implements the ECS and Nazief-Adriani modifications for Javanese morphology.
pub struct JavaneseStemmer<'a> {
    dictionary: &'a JavaneseDictionary,
    tokenizer: Tokenizer,
    affixation: JavaneseAffixation<'a>,
}

impl<'a> JavaneseStemmer<'a> {
    pub fn new(dictionary: &'a JavaneseDictionary) -> JavaneseStemmer<'a> {
        let tokenizer = Tokenizer::new();
        let affixation = JavaneseAffixation::new(dictionary);
        JavaneseStemmer {
            dictionary,
            tokenizer,
            affixation,
        }
    }

    /// Stems all tokens in a Javanese sentence, returning an iterator.
    pub fn stem_sentence<'b>(
        &'b self,
        sentence: &'b str,
    ) -> impl Iterator<Item = Cow<'b, str>> + 'b {
        self.tokenizer
            .tokenize(sentence)
            .map(move |word| self.stem_word(word))
    }

    /// Stems a single Javanese word returning a zero-copy string slice if unmodified.
    pub fn stem_word<'b>(&self, word: &'b str) -> Cow<'b, str> {
        let base = if let Some(idx) = word.find('-') {
            &word[..idx] // Handle standard Javanese reduplication partly, e.g. wira-wiri -> wira
        } else {
            word
        };

        let original_word = base.to_lowercase();

        if self.dictionary.find(&original_word) {
            return Cow::Owned(original_word);
        }

        if original_word.chars().count() < 3 {
            return Cow::Owned(original_word);
        }

        let mut current_matches = Vec::new();
        let mut possessives = self.affixation.remove_possessive(&original_word);
        possessives.push((String::new(), original_word.clone())); // Always include unmodified word path

        // Exhaustive matching queue: (possessive_suf, particle_suf, current_root)
        for (pos, pos_root) in possessives {
            if self.dictionary.find(&pos_root) && pos_root.len() >= 3 {
                return Cow::Owned(pos_root);
            }

            let mut particles = self.affixation.remove_particle(&pos_root);
            particles.push((String::new(), pos_root.clone())); // Always include unmodified word path

            for (part, final_root) in particles {
                if self.dictionary.find(&final_root) {
                    return Cow::Owned(final_root);
                }

                // Add to prefix testing queue
                current_matches.push((pos.clone(), part.clone(), final_root));
            }
        }

        // 3. Remove Prefixes on suffix-stripped variants
        for (_pos, _part, root) in &current_matches {
            let (found, res) = self.affixation.remove_prefixes(root);
            if found {
                return Cow::Owned(res);
            }
        }

        // 4. Backtracking / Pengembalian Akhir
        // We simulate the backtracking by passing combinations to pengembalian_akhir.
        for (pos, part, _) in &current_matches {
            let mut removed_suffixes = Vec::new();
            if !part.is_empty() {
                removed_suffixes.push(part.clone());
            }
            if !pos.is_empty() {
                removed_suffixes.push(pos.clone());
            }
            if !pos.is_empty() && !part.is_empty() {
                removed_suffixes.push(format!("{}{}", part, pos));
            } // part + pos

            let (found, res) = self
                .affixation
                .pengembalian_akhir(&original_word, &removed_suffixes);
            if found {
                return Cow::Owned(res);
            }
        }

        Cow::Owned(original_word)
    }
}
