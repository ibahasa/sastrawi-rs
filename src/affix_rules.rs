use std::borrow::Cow;

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'e' | 'o')
}

pub fn is_consonant(c: char) -> bool {
    c.is_alphabetic() && !is_vowel(c)
}

// ---------------------------------------------------------------------------
// Particles: -lah, -kah, -tah, -pun
// Also handles se-nya superlative: "selengkapnya" strips -nya here so
// the remainder "selengkap" can be further processed.
// ---------------------------------------------------------------------------
pub fn remove_particle<'a>(word: &'a str) -> (Cow<'a, str>, Cow<'a, str>) {
    for p in ["lah", "kah", "tah", "pun"] {
        if word.ends_with(p) && word.len() > p.len() + 2 {
            return (Cow::Borrowed(p), Cow::Borrowed(&word[..word.len() - p.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

// ---------------------------------------------------------------------------
// Possessives: -ku, -mu, -nya
// ---------------------------------------------------------------------------
pub fn remove_possessive<'a>(word: &'a str) -> (Cow<'a, str>, Cow<'a, str>) {
    for p in ["ku", "mu", "nya"] {
        if word.ends_with(p) && word.len() > p.len() + 2 {
            return (Cow::Borrowed(p), Cow::Borrowed(&word[..word.len() - p.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

// ---------------------------------------------------------------------------
// Derivational suffixes (longest-match first)
// Includes loanword suffixes from 2020-2026 research:
//   -isme, -isasi, -isir  (idealisasi→ideal, legalisir→legal)
// ---------------------------------------------------------------------------
pub fn remove_suffix<'a>(word: &'a str) -> (Cow<'a, str>, Cow<'a, str>) {
    for s in ["kan", "isme", "isasi", "isir", "an", "is", "i"] {
        if word.ends_with(s) && word.len() > s.len() + 2 {
             return (Cow::Borrowed(s), Cow::Borrowed(&word[..word.len() - s.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

// ---------------------------------------------------------------------------
// Confix stripping (ECS — Enhanced Confix Stripping, 2020-2026)
// Strips prefix AND suffix simultaneously for circumfix morphemes:
//   ke-an  → keamanan  → aman
//   per-an → pertanian → tani
//   ber-an → berhadapan → hadap
//   se-nya → selengkapnya → lengkap (after -nya stripped as particle)
//
// Returns (found: bool, root: String)
// ---------------------------------------------------------------------------
pub fn remove_confix(word: &str) -> Option<String> {
    // ke-an
    if word.starts_with("ke") && word.ends_with("an") && word.len() > 6 {
        let root = &word[2..word.len() - 2];
        if root.len() >= 2 {
            return Some(root.to_string());
        }
    }
    // per-an
    if word.starts_with("per") && word.ends_with("an") && word.len() > 7 {
        let root = &word[3..word.len() - 2];
        if root.len() >= 2 {
            return Some(root.to_string());
        }
    }
    // ber-an
    if word.starts_with("ber") && word.ends_with("an") && word.len() > 7 {
        let root = &word[3..word.len() - 2];
        if root.len() >= 2 {
            return Some(root.to_string());
        }
    }
    // me-kan
    if word.starts_with("me") && word.ends_with("kan") && word.len() > 7 {
        let root = &word[2..word.len() - 3];
        if root.len() >= 2 {
            return Some(root.to_string());
        }
    }
    // pe-an
    if word.starts_with("pe") && word.ends_with("an") && word.len() > 6 {
        let root = &word[2..word.len() - 2];
        if root.len() >= 2 {
            return Some(root.to_string());
        }
    }
    // ter-kan
    if word.starts_with("ter") && word.ends_with("kan") && word.len() > 8 {
        let root = &word[3..word.len() - 3];
        if root.len() >= 2 {
            return Some(root.to_string());
        }
    }
    // se-nya (superlative: selengkapnya→lengkap after nya stripped)
    if word.starts_with("se") && word.len() > 4 {
        let root = &word[2..];
        if root.len() >= 2 {
            return Some(root.to_string());
        }
    }
    None
}

// ---------------------------------------------------------------------------
// me- prefix patterns (Nazief-Adriani complete)
// ---------------------------------------------------------------------------
pub fn remove_prefix_me<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word.len() < 4 || !word.starts_with("me") {
        return (Cow::Borrowed(word), vec![]);
    }

    let chars: Vec<char> = word.chars().take(7).collect();
    if chars.len() < 3 { return (Cow::Borrowed(word), vec![]); }

    // Pattern 1: me{l|r|w|y}V
    if matches!(chars[2], 'l' | 'r' | 'w' | 'y') && chars.len() > 3 && is_vowel(chars[3]) {
        return (Cow::Borrowed(&word[2..]), vec![]);
    }

    // Pattern 2: mem{b|f|v}
    if word.starts_with("mem") && chars.len() > 3 && matches!(chars[3], 'b' | 'f' | 'v') {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    // Pattern 3: mempe
    if word.starts_with("mempe") {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    // Pattern 4: mem{rV|V}
    if word.starts_with("mem") && chars.len() > 3 && (is_vowel(chars[3]) || (chars[3] == 'r' && chars.len() > 4 && is_vowel(chars[4]))) {
        return (Cow::Borrowed(&word[3..]), vec!["m".to_string(), "p".to_string()]);
    }

    // Pattern 5: men{c|d|j|s|t|z}
    if word.starts_with("men") && chars.len() > 3 && matches!(chars[3], 'c' | 'd' | 'j' | 's' | 't' | 'z') {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    // Pattern 6: menV → nV OR tV
    if word.starts_with("men") && chars.len() > 3 && is_vowel(chars[3]) {
        return (Cow::Borrowed(&word[3..]), vec!["n".to_string(), "t".to_string()]);
    }

    // Pattern 7: meng{g|h|q|k}
    if word.starts_with("meng") && chars.len() > 4 && matches!(chars[4], 'g' | 'h' | 'q' | 'k') {
        return (Cow::Borrowed(&word[4..]), vec![]);
    }

    // Pattern 8b: menge- monosyllabic (mengebom→bom)
    // Must precede Pattern 8 'e' branch to avoid mengebom→ebom
    if word.starts_with("menge") && chars.len() > 5 && is_consonant(chars[5]) {
        return (Cow::Borrowed(&word[5..]), vec![]);
    }

    // Pattern 8: mengV (V≠e handled in 8b above)
    if word.starts_with("meng") && chars.len() > 4 && is_vowel(chars[4]) {
        if chars[4] == 'e' {
            return (Cow::Borrowed(&word[4..]), vec![]);
        }
        return (Cow::Borrowed(&word[4..]), vec!["ng".to_string(), "k".to_string()]);
    }

    // Pattern 9: menyV → bare vowel stem + recodings [s, ny]
    // Affixation prepends: s+ala→sala, ny+ala→nyala
    if word.starts_with("meny") && chars.len() > 4 && is_vowel(chars[4]) {
        return (Cow::Borrowed(&word[4..]), vec!["s".to_string(), "ny".to_string()]);
    }

    // Pattern 10: mempV (V≠e)
    if word.starts_with("mem") && chars.len() > 3 && chars[3] == 'p' && chars.len() > 4 && chars[4] != 'e' {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    (Cow::Borrowed(word), vec![])
}

// ---------------------------------------------------------------------------
// nge- informal prefix (2020-2026 research: MPStemmer, Aksara v1.2+)
// Colloquial/lisan form of menge-. Common in social media & Jakarta speech.
// Examples: ngecat→cat, ngegas→gas, ngerasain→rasain→rasa, ngelamar→lamar
// ---------------------------------------------------------------------------
pub fn remove_prefix_nge<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word.starts_with("nge") && word.len() > 4 {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }
    (Cow::Borrowed(word), vec![])
}

// ---------------------------------------------------------------------------
// pe- prefix patterns
// ---------------------------------------------------------------------------
pub fn remove_prefix_pe<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    // Exception: pelajar is always "ajar"
    if word == "pelajar" {
        return (Cow::Borrowed("ajar"), vec![]);
    }

    if word.starts_with("pe") && word.len() > 4 {
        let chars: Vec<char> = word.chars().take(7).collect();

        // per-V
        if word.starts_with("per") && chars.len() > 3 && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
        }

        // pe{C}V — simple pe- drop (petarung→tarung, pekerja→kerja)
        if !word.starts_with("per") && !word.starts_with("pem") && !word.starts_with("pen") && !word.starts_with("peng") && !word.starts_with("peny") {
             if is_consonant(chars[2]) && chars.len() > 3 && is_vowel(chars[3]) {
                 return (Cow::Borrowed(&word[2..]), vec![]);
             }
        }

        // per-C (not Cer pattern)
        if word.starts_with("per") && chars.len() > 5 && is_consonant(chars[3]) && chars[3] != 'r' && chars[4] == 'e' && chars[5] == 'r' {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        if word.starts_with("per") && is_consonant(chars[3]) && chars[3] != 'r' {
            if chars.len() > 5 && chars[4] == 'e' && chars[5] == 'r' {
                // p3 case — skip
            } else {
                return (Cow::Borrowed(&word[3..]), vec![]);
            }
        }

        // peC1erC2
        if chars[2] != 'r' && chars[2] != 'l' && chars.len() > 4 && chars[3] == 'e' && chars[4] == 'r' {
             return (Cow::Borrowed(&word[2..]), vec![]);
        }

        // pem{b|f|v}
        if word.starts_with("pem") && chars.len() > 3 && matches!(chars[3], 'b' | 'f' | 'v') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        // pem{rV|V}
        if word.starts_with("pem") && chars.len() > 3 && (is_vowel(chars[3]) || (chars[3] == 'r' && chars.len() > 4 && is_vowel(chars[4]))) {
            return (Cow::Borrowed(&word[3..]), vec!["m".to_string(), "p".to_string()]);
        }
        // pen{c|d|j|s|t|z}
        if word.starts_with("pen") && chars.len() > 3 && matches!(chars[3], 'c' | 'd' | 'j' | 's' | 't' | 'z') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        // penV → nV|tV
        if word.starts_with("pen") && chars.len() > 3 && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["n".to_string(), "t".to_string()]);
        }
        // peng{g|h|q|k}
        if word.starts_with("peng") && chars.len() > 4 && matches!(chars[4], 'g' | 'h' | 'q' | 'k') {
            return (Cow::Borrowed(&word[4..]), vec![]);
        }
        // penge- monosyllabic (pengebom→bom) — must precede peng+e
        if word.starts_with("penge") && chars.len() > 5 && is_consonant(chars[5]) {
            return (Cow::Borrowed(&word[5..]), vec![]);
        }
        // pengV
        if word.starts_with("peng") && chars.len() > 4 && is_vowel(chars[4]) {
            if chars[4] == 'e' {
                return (Cow::Borrowed(&word[4..]), vec![]);
            }
            return (Cow::Borrowed(&word[4..]), vec!["ng".to_string(), "k".to_string()]);
        }
        // penyV → bare vowel stem + recodings [s, ny]
        if word.starts_with("peny") && chars.len() > 4 && is_vowel(chars[4]) {
             return (Cow::Borrowed(&word[4..]), vec!["s".to_string(), "ny".to_string()]);
        }
    }

    (Cow::Borrowed(word), vec![])
}

// ---------------------------------------------------------------------------
// ber- prefix patterns
// ---------------------------------------------------------------------------
pub fn remove_prefix_be<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word == "belajar" {
        return (Cow::Borrowed("ajar"), vec![]);
    }
    if word.starts_with("ber") && word.len() > 3 {
        let chars: Vec<char> = word.chars().take(7).collect();
        if is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
        }
        if chars.len() > 5 && is_consonant(chars[3]) && chars[3] != 'r' && chars[4] == 'e' && chars[5] == 'r' {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        if is_consonant(chars[3]) && chars[3] != 'r' {
            if chars.len() > 5 && chars[4] == 'e' && chars[5] == 'r' {
                // p2 case — skip
            } else {
                return (Cow::Borrowed(&word[3..]), vec![]);
            }
        }
    } else if word.starts_with("be") && word.len() > 4 {
        let chars: Vec<char> = word.chars().take(7).collect();
        if chars[2] != 'r' && chars[2] != 'l' && chars[3] == 'e' && chars[4] == 'r' {
            return (Cow::Borrowed(&word[2..]), vec![]);
        }
    }
    (Cow::Borrowed(word), vec![])
}

// ---------------------------------------------------------------------------
// ter- prefix patterns
// ---------------------------------------------------------------------------
pub fn remove_prefix_te<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word.starts_with("ter") && word.len() > 3 {
         let chars: Vec<char> = word.chars().take(7).collect();
         if is_vowel(chars[3]) {
             return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
         }
         if chars.len() > 5 && is_consonant(chars[3]) && chars[3] != 'r' && chars[4] == 'e' && chars[5] == 'r' {
             return (Cow::Borrowed(&word[3..]), vec![]);
         }
         if is_consonant(chars[3]) && chars[3] != 'r' {
              return (Cow::Borrowed(&word[3..]), vec![]);
         }
    } else if word.starts_with("te") && word.len() > 4 {
         let chars: Vec<char> = word.chars().take(7).collect();
         if chars[2] != 'r' && chars[3] == 'e' && chars[4] == 'r' {
             return (Cow::Borrowed(&word[2..]), vec![]);
         }
    }
    (Cow::Borrowed(word), vec![])
}

// ---------------------------------------------------------------------------
// Infixes: -el-, -er-, -em-, -in-
// ---------------------------------------------------------------------------
pub fn remove_infix<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word.len() > 4 {
        let chars: Vec<char> = word.chars().collect();
        if is_consonant(chars[0]) && chars[1] == 'e' && matches!(chars[2], 'l' | 'r' | 'm') && is_vowel(chars[3]) {
             return (Cow::Borrowed(&word[3..]), vec![word[..3].to_string(), word[..1].to_string()]);
        }
        if is_consonant(chars[0]) && chars[1] == 'i' && chars[2] == 'n' && is_vowel(chars[3]) {
             return (Cow::Borrowed(&word[3..]), vec![word[..3].to_string(), word[..1].to_string()]);
        }
    }
    (Cow::Borrowed(word), vec![])
}
