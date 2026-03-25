use std::borrow::Cow;

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'e' | 'o')
}

pub fn is_consonant(c: char) -> bool {
    c.is_alphabetic() && !is_vowel(c)
}

pub fn remove_particle<'a>(word: &'a str) -> (Cow<'a, str>, Cow<'a, str>) {
    for p in ["lah", "kah", "tah", "pun"] {
        if word.ends_with(p) && word.len() > p.len() + 2 {
            return (Cow::Borrowed(p), Cow::Borrowed(&word[..word.len() - p.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

pub fn remove_possessive<'a>(word: &'a str) -> (Cow<'a, str>, Cow<'a, str>) {
    for p in ["ku", "mu", "nya"] {
        if word.ends_with(p) && word.len() > p.len() + 2 {
            return (Cow::Borrowed(p), Cow::Borrowed(&word[..word.len() - p.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

pub fn remove_suffix<'a>(word: &'a str) -> (Cow<'a, str>, Cow<'a, str>) {
    for s in ["kan", "isme", "isasi", "an", "is", "i"] {
        if word.ends_with(s) && word.len() > s.len() + 2 {
             return (Cow::Borrowed(s), Cow::Borrowed(&word[..word.len() - s.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

pub fn remove_prefix_me<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word.len() < 4 || !word.starts_with("me") {
        return (Cow::Borrowed(word), vec![]);
    }

    let chars: Vec<char> = word.chars().take(7).collect();
    if chars.len() < 3 { return (Cow::Borrowed(word), vec![]); }

    // Pattern 1: me{l|r|w|y}V => me-{l|r|w|y}V
    if matches!(chars[2], 'l' | 'r' | 'w' | 'y') && chars.len() > 3 && is_vowel(chars[3]) {
        return (Cow::Borrowed(&word[2..]), vec![]);
    }

    // Pattern 2: mem{b|f|v} => mem-{b|f|v}
    if word.starts_with("mem") && chars.len() > 3 && matches!(chars[3], 'b' | 'f' | 'v') {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    // Pattern 3: mempe => mem-pe
    if word.starts_with("mempe") {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    // Pattern 4: mem{rV|V} => mem-{rV|V} OR me-p{rV|V}
    if word.starts_with("mem") && chars.len() > 3 && (is_vowel(chars[3]) || (chars[3] == 'r' && chars.len() > 4 && is_vowel(chars[4]))) {
        return (Cow::Borrowed(&word[3..]), vec!["m".to_string(), "p".to_string()]);
    }

    // Pattern 5: men{c|d|j|s|t|z} => men-{c|d|j|s|t|z}
    if word.starts_with("men") && chars.len() > 3 && matches!(chars[3], 'c' | 'd' | 'j' | 's' | 't' | 'z') {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    // Pattern 6: menV => nV OR tV
    if word.starts_with("men") && chars.len() > 3 && is_vowel(chars[3]) {
        return (Cow::Borrowed(&word[3..]), vec!["n".to_string(), "t".to_string()]);
    }

    // Pattern 7: meng{g|h|q|k} => meng-{g|h|q|k}
    if word.starts_with("meng") && chars.len() > 4 && matches!(chars[4], 'g' | 'h' | 'q' | 'k') {
        return (Cow::Borrowed(&word[4..]), vec![]);
    }

    // Pattern 8b: menge- (monosyllabic base: mengebom→bom, mengecat→cat)
    // Must be checked BEFORE Pattern 8 'e' vowel case to avoid mengebom→ebom
    if word.starts_with("menge") && chars.len() > 5 && is_consonant(chars[5]) {
        return (Cow::Borrowed(&word[5..]), vec![]);
    }

    // Pattern 8: mengV => meng-V OR meng-kV (V != 'e' handled above)
    if word.starts_with("meng") && chars.len() > 4 && is_vowel(chars[4]) {
        if chars[4] == 'e' {
            return (Cow::Borrowed(&word[4..]), vec![]);
        }
        return (Cow::Borrowed(&word[4..]), vec!["ng".to_string(), "k".to_string()]);
    }

    // Pattern 9: menyV => bare vowel stem + recodings [s, ny]
    // Affixation prepends each recoding to the bare stem: s+ala=sala, ny+ala=nyala
    if word.starts_with("meny") && chars.len() > 4 && is_vowel(chars[4]) {
        return (Cow::Borrowed(&word[4..]), vec!["s".to_string(), "ny".to_string()]);
    }

    // Pattern 10: mempV => mem-pA where A != 'e'
    if word.starts_with("mem") && chars.len() > 3 && chars[3] == 'p' && chars.len() > 4 && chars[4] != 'e' {
        return (Cow::Borrowed(&word[3..]), vec![]);
    }

    (Cow::Borrowed(word), vec![])
}

pub fn remove_prefix_pe<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word == "pelajar" {
        return (Cow::Borrowed("ajar"), vec![]);
    }
    
    if word.starts_with("pe") && word.len() > 4 {
        let chars: Vec<char> = word.chars().take(7).collect();

        // Check per-V Pattern first
        if word.starts_with("per") && chars.len() > 3 && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
        }
        
        // Pattern 1a/b: pe{l|w|y|t...}V
        // In Indonesian pe- + consonant often just removes pe- (like petarung)
        // Except for triggering ones.
        if !word.starts_with("per") && !word.starts_with("pem") && !word.starts_with("pen") && !word.starts_with("peng") && !word.starts_with("peny") {
             if is_consonant(chars[2]) && chars.len() > 3 && is_vowel(chars[3]) {
                 return (Cow::Borrowed(&word[2..]), vec![]);
             }
        }

        if word.starts_with("per") && chars.len() > 5 && is_consonant(chars[3]) && chars[3] != 'r' && chars[4] == 'e' && chars[5] == 'r' {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        if word.starts_with("per") && is_consonant(chars[3]) && chars[3] != 'r' {
            if chars.len() > 5 && chars[4] == 'e' && chars[5] == 'r' {
                // p3
            } else {
                return (Cow::Borrowed(&word[3..]), vec![]);
            }
        }
        // peC1erC2
        if chars[2] != 'r' && chars[2] != 'l' && chars.len() > 4 && chars[3] == 'e' && chars[4] == 'r' {
             return (Cow::Borrowed(&word[2..]), vec![]);
        }

        if word.starts_with("pem") && chars.len() > 3 && matches!(chars[3], 'b' | 'f' | 'v') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        if word.starts_with("pem") && chars.len() > 3 && (is_vowel(chars[3]) || (chars[3] == 'r' && chars.len() > 4 && is_vowel(chars[4]))) {
            return (Cow::Borrowed(&word[3..]), vec!["m".to_string(), "p".to_string()]);
        }
        if word.starts_with("pen") && chars.len() > 3 && matches!(chars[3], 'c' | 'd' | 'j' | 's' | 't' | 'z') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        if word.starts_with("pen") && chars.len() > 3 && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["n".to_string(), "t".to_string()]);
        }
        if word.starts_with("peng") && chars.len() > 4 && matches!(chars[4], 'g' | 'h' | 'q' | 'k') {
            return (Cow::Borrowed(&word[4..]), vec![]);
        }
        // penge- monosyllabic: pengebom→bom (must come before peng+e vowel check)
        if word.starts_with("penge") && chars.len() > 5 && is_consonant(chars[5]) {
            return (Cow::Borrowed(&word[5..]), vec![]);
        }
        if word.starts_with("peng") && chars.len() > 4 && is_vowel(chars[4]) {
            if chars[4] == 'e' {
                return (Cow::Borrowed(&word[4..]), vec![]);
            }
            return (Cow::Borrowed(&word[4..]), vec!["ng".to_string(), "k".to_string()]);
        }
        
        // penyV => bare vowel stem + recodings [s, ny]
        if word.starts_with("peny") && chars.len() > 4 && is_vowel(chars[4]) {
             return (Cow::Borrowed(&word[4..]), vec!["s".to_string(), "ny".to_string()]);
        }

        // penge- (monosyllabic: pengebom→bom)
        if word.starts_with("penge") && chars.len() > 5 && is_consonant(chars[5]) {
            return (Cow::Borrowed(&word[5..]), vec![]);
        }
    }
    
    (Cow::Borrowed(word), vec![])
}

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
                // p2
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
