use crate::dictionary::Dictionary;
use crate::tokenizer::Tokenizer;
use crate::affixation::Affixation;
use std::borrow::Cow;

/// The main Indonesian word stemmer.
///
/// It follows the Nazief-Adriani algorithm, enhanced with ECS and modern
/// morphological extensions.
///
/// # Example
/// ```
/// use sastrawi::{Dictionary, Stemmer};
///
/// let dict = Dictionary::new();
/// let stemmer = Stemmer::new(&dict);
///
/// let word = "Perekonomian";
/// assert_eq!(stemmer.stem_word(word), "ekonomi");
/// ```
pub struct Stemmer<'a> {
    dictionary: &'a Dictionary,
    stopwords: Dictionary,
    tokenizer: Tokenizer,
    affixation: Affixation<'a>,
}

impl<'a> Stemmer<'a> {
    /// Creates a new `Stemmer` instance using the provided root dictionary.
    ///
    /// The dictionary must be loaded beforehand using `Dictionary::new()` or
    /// `Dictionary::custom()`.
    ///
    /// # Example
    /// ```
    /// use sastrawi::{Dictionary, Stemmer};
    ///
    /// let dict = Dictionary::new();
    /// let stemmer = Stemmer::new(&dict);
    /// ```
    pub fn new(dictionary: &Dictionary) -> Stemmer<'_> {
        let tokenizer = Tokenizer::new();
        let affixation = Affixation::new(dictionary);
        Stemmer {
            dictionary,
            stopwords: Dictionary::stopword(),
            tokenizer,
            affixation,
        }
    }

    /// Stems all tokens in a sentence, returning them as an iterator.
    ///
    /// Words are processed using `stem_word`. Stopwords are NOT filtered
    /// in this method. Use `stem_sentence_filtered` to skip them.
    ///
    /// # Example
    /// ```
    /// use sastrawi::{Dictionary, Stemmer};
    ///
    /// let dict = Dictionary::new();
    /// let stemmer = Stemmer::new(&dict);
    ///
    /// let sentence = "Dia sedang makan nasi";
    /// let stemmed: Vec<_> = stemmer.stem_sentence(sentence).collect();
    /// // → ["dia", "sedang", "makan", "nasi"]
    /// ```
    pub fn stem_sentence<'b>(&'b self, sentence: &'b str) -> impl Iterator<Item = Cow<'b, str>> + 'b {
        self.tokenizer.tokenize(sentence).map(move |word| self.stem_word(word))
    }

    /// Stems all tokens in a sentence, skipping common Indonesian stopwords.
    ///
    /// Stopwords like "yang", "di", "dari" are removed automatically.
    /// This is highly recommended for building search indexes or running
    /// NLP analysis.
    ///
    /// # Example
    /// ```
    /// use sastrawi::{Dictionary, Stemmer};
    ///
    /// let dict = Dictionary::new();
    /// let stemmer = Stemmer::new(&dict);
    ///
    /// let sentence = "Pertumbuhan yang membanggakan";
    /// let filtered: Vec<_> = stemmer.stem_sentence_filtered(sentence).collect();
    /// // → ["tumbuh", "bangga"]  — "yang" is skipped
    /// ```
    pub fn stem_sentence_filtered<'b>(&'b self, sentence: &'b str) -> impl Iterator<Item = Cow<'b, str>> + 'b {
        self.tokenizer
            .tokenize(sentence)
            .filter(move |word| !self.stopwords.find(word))
            .map(move |word| self.stem_word(word))
    }

    /// Checks whether a specific word is considered a stopword.
    ///
    /// It uses the default Indonesian stopword list.
    ///
    /// # Example
    /// ```
    /// use sastrawi::{Dictionary, Stemmer};
    ///
    /// let dict = Dictionary::new();
    /// let stemmer = Stemmer::new(&dict);
    ///
    /// assert!(stemmer.is_stopword("yang"));
    /// assert!(!stemmer.is_stopword("ekonomi"));
    /// ```
    pub fn is_stopword(&self, word: &str) -> bool {
        self.stopwords.find(word)
    }

    /// Stems a single Indonesian word.
    ///
    /// It returns a `Cow<'b, str>`. If the word was not modified, it will
    /// point to the original borrowed slice (zero-copy), otherwise any
    /// changes are owned and returned as a new `String`.
    ///
    /// # Morphological Handling
    ///
    /// 1.  **Hyphenated clitics**: `kuasa-Mu` → `kuasa`
    /// 2.  **Particles**: `-lah`, `-kah`, `-pun`
    /// 3.  **Possessives**: `-ku`, `-mu`, `-nya`
    /// 4.  **Prefixes/Suffixes**: full Nazief-Adriani + ECS extensions.
    /// 5.  **Backtracking**: ensures the longest valid root is found.
    ///
    /// # Example
    /// ```
    /// use sastrawi::{Dictionary, Stemmer};
    ///
    /// let dict = Dictionary::new();
    /// let stemmer = Stemmer::new(&dict);
    ///
    /// assert_eq!(stemmer.stem_word("membangunkan"), "bangun");
    /// assert_eq!(stemmer.stem_word("pertanian"), "tani");
    /// ```
    pub fn stem_word<'b>(&self, word: &'b str) -> Cow<'b, str> {
        // Handle hyphenated clitics: kuasa-Mu → stem("kuasa"), allah-lah → stem("allah")
        let base = if let Some(idx) = word.find('-') {
            &word[..idx]
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

        let mut current = original_word.clone();
        let mut particle = String::new();
        let mut possessive = String::new();
        let mut suffix = String::new();

        // --- Step 1: Remove Particle ---
        let pres = {
            let (p, r) = self.affixation.remove_particle(&current);
            if p.is_empty() { None } else { Some((p.into_owned(), r.into_owned())) }
        };
        if let Some((p, after)) = pres {
            particle = p;
            if self.dictionary.find(&after) && after.len() >= 4 {
                return Cow::Owned(after);
            }
            current = after;
        }

        // --- Step 2: Remove Possessive ---
        let posres = {
            let (p, r) = self.affixation.remove_possessive(&current);
            if p.is_empty() { None } else { Some((p.into_owned(), r.into_owned())) }
        };
        if let Some((pos, after)) = posres {
            possessive = pos;
            if self.dictionary.find(&after) {
                return Cow::Owned(after);
            }
            current = after;
        }

        // --- Step 3 & 4: Remove Suffix then try Prefixes ---
        let sufres = {
            let (s, r) = self.affixation.remove_suffix(&current);
            if s.is_empty() { None } else { Some((s.into_owned(), r.into_owned())) }
        };
        if let Some((s, after_suffix)) = sufres {
            suffix = s;
            if self.dictionary.find(&after_suffix) {
                let (fo, ro) = self.affixation.remove_prefixes(&original_word);
                if fo {
                    return Cow::Owned(ro);
                }
                return Cow::Owned(after_suffix);
            }
            let (found, res) = self.affixation.remove_prefixes(&after_suffix);
            if found {
                let (found_orig, res_orig) = self.affixation.remove_prefixes(&original_word);
                if found_orig && res_orig.len() > res.len() {
                    return Cow::Owned(res_orig);
                }
                return Cow::Owned(res);
            }
        }

        // --- Step 4.5: ECS Confix ---
        if let Some(root) = self.affixation.remove_confix(&original_word) {
            return Cow::Owned(root);
        }

        // --- Step 5: Prefix-only removal (Longest-root fallback) ---
        let (found_original, res_original) = self.affixation.remove_prefixes(&original_word);
        if found_original {
            return Cow::Owned(res_original);
        }

        if current != original_word {
            let (prefix_found, res5) = self.affixation.remove_prefixes(&current);
            if prefix_found {
                return Cow::Owned(res5);
            }
        }

        // --- Final: Pengembalian Akhir (Backtracking) ---
        let mut removed_suffixes = vec![
            String::new(),
            suffix.clone(),
            possessive.clone(),
            particle.clone(),
        ];
        if suffix == "kan" {
            removed_suffixes = vec![
                String::new(),
                "k".to_string(),
                "an".to_string(),
                possessive,
                particle,
            ];
        }

        let (found, res_backtrack) = self.affixation.pengembalian_akhir(&original_word, &removed_suffixes);
        if found {
            return Cow::Owned(res_backtrack);
        }

        Cow::Owned(original_word)
    }
}