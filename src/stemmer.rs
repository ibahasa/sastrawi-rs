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
        Stemmer {
            dictionary,
            tokenizer,
            affixation,
        }
    }

    pub fn stem_sentence<'b>(&'b self, sentence: &'b str) -> impl Iterator<Item = Cow<'b, str>> + 'b {
        self.tokenizer.tokenize(sentence).map(move |word| self.stem_word(word))
    }

    pub fn stem_word<'b>(&self, word: &'b str) -> Cow<'b, str> {
        // Handle hyphenated clitics: kuasa-Mu → stem("kuasa"), allah-lah → stem("allah")
        // Strip everything from the hyphen onward before processing
        let base = if let Some(idx) = word.find('-') {
            &word[..idx]
        } else {
            word
        };

        // Always normalize to lowercase first
        let original_word = base.to_lowercase();

        if self.dictionary.find(&original_word) {
            return Cow::Owned(original_word);
        }

        if original_word.chars().count() < 3 {
            return Cow::Owned(original_word);
        }

        // We work with owned Strings from here so no borrow issues
        let mut current = original_word.clone();
        let mut particle = String::new();
        let mut possessive = String::new();
        let mut suffix = String::new();

        // --- Step 1: Remove Particle ---
        // Call in block to isolate borrow
        let pres = {
            let (p, r) = self.affixation.remove_particle(&current);
            if p.is_empty() { None } else { Some((p.into_owned(), r.into_owned())) }
        };
        if let Some((p, after)) = pres {
            particle = p;
            // Only accept if: (a) bare word is in dict AND (b) it's at least 3 chars
            // This prevents false positives like "seko" from "bersekolah-lah"→"berseko"
            // The real check: don't short-circuit if the bare word is suspiciously short
            // and the FULL word (without particle only) is also a valid base after prefix stripping.
            // Simple heuristic: accept if found in dict and len >= 4.
            if self.dictionary.find(&after) && after.len() >= 4 {
                return Cow::Owned(after);
            }
            // Continue Nazief-Adriani with particle removed
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
                // Guard against over-stemming: if prefix-only on original ALSO succeeds,
                // the original path is the true root (fewer morphemes removed).
                // e.g. petani → suffix-i → petan (in dict), but petani → pe- → tani is better.
                let (fo, ro) = self.affixation.remove_prefixes(&original_word);
                if fo {
                    return Cow::Owned(ro);
                }
                return Cow::Owned(after_suffix);
            }
            let (found, res) = self.affixation.remove_prefixes(&after_suffix);
            if found {
                // Longest Root check: prefer prefix-only on original if it yields longer root
                let (found_orig, res_orig) = self.affixation.remove_prefixes(&original_word);
                if found_orig && res_orig.len() > res.len() {
                    return Cow::Owned(res_orig);
                }
                return Cow::Owned(res);
            }
            // Suffix didn't help — keep suffix captured for final backtrack
        }

        // --- Step 4.5: ECS Confix — simultaneous prefix+suffix strip ---
        // Handles ke-an, per-an, ber-an, me-kan, pe-an, ter-kan, se-nya.
        // Based on Enhanced Confix Stripping research (outperforms plain Nazief-Adriani).
        if let Some(root) = self.affixation.remove_confix(&original_word) {
            return Cow::Owned(root);
        }

        // --- Step 5: Prefix-only removal ---
        // Best practice (Longest Root / Conservative Stemming):
        // Try prefix stripping on ORIGINAL word first. If it yields a valid root,
        // prefer it over stripping from a particle-modified `current`. This prevents
        // over-stemming like bersekolah→berseko(-lah)→seko instead of bersekolah→sekolah.
        let (found_original, res_original) = self.affixation.remove_prefixes(&original_word);
        if found_original {
            return Cow::Owned(res_original);
        }

        // Fallback: try prefix stripping on current (particle/possessive may have been removed)
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