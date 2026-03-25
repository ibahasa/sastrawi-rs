use std::borrow::Cow;

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'i' | 'u' | 'e' | 'o')
}

pub fn is_consonant(c: char) -> bool {
    c.is_alphabetic() && !is_vowel(c)
}

pub fn remove_particle(word: &str) -> (Cow<str>, Cow<str>) {
    for p in ["lah", "kah", "tah", "pun"] {
        if word.ends_with(p) && word.len() > p.len() + 2 {
            return (Cow::Borrowed(p), Cow::Borrowed(&word[..word.len() - p.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

pub fn remove_possessive(word: &str) -> (Cow<str>, Cow<str>) {
    for p in ["ku", "mu", "nya"] {
        if word.ends_with(p) && word.len() > p.len() + 2 {
            return (Cow::Borrowed(p), Cow::Borrowed(&word[..word.len() - p.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

pub fn remove_suffix(word: &str) -> (Cow<str>, Cow<str>) {
    // Suffixes: -kan, -an, -i
    // Also handling modern suffixes if needed, but keeping original Sastrawi for now
    for s in ["kan", "an", "i"] {
        if word.ends_with(s) && word.len() > s.len() + 2 {
             // In Indonesian stemming, some suffixes have priorities.
             // Nazief-Adriani handles this in a specific loop.
             return (Cow::Borrowed(s), Cow::Borrowed(&word[..word.len() - s.len()]));
        }
    }
    (Cow::Borrowed(""), Cow::Borrowed(word))
}

pub fn remove_prefix_me(word: &str) -> (Cow<str>, Vec<String>) {
    if word.len() < 4 {
        return (Cow::Borrowed(word), vec![]);
    }

    if word.starts_with("me") {
        let chars: Vec<char> = word.chars().take(6).collect();
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

        // Pattern 8: mengV => meng-V OR meng-kV OR me-ngV OR mengV- where V = 'e'
        if word.starts_with("meng") && chars.len() > 4 && is_vowel(chars[4]) {
            if chars[4] == 'e' {
                return (Cow::Borrowed(&word[4..]), vec![]);
            }
            return (Cow::Borrowed(&word[4..]), vec!["ng".to_string(), "k".to_string()]);
        }

        // Pattern 9: menyV => meny-sV OR me-nyV to stem menyala
        if word.starts_with("meny") && chars.len() > 4 && is_vowel(chars[4]) {
            if chars[4] == 'a' && !word.starts_with("menyanyi") {
                 // menyala -> ala (checked against dict later)
                 return (Cow::Borrowed(&word[2..]), vec![]);
            }
            // menyikat -> sikat
            let mut s = "s".to_string();
            s.push_str(&word[4..]);
            return (Cow::Owned(s), vec![]);
        }

        // Pattern 10: mempV => mem-pA where A != 'e'
        if word.starts_with("mem") && chars.len() > 3 && chars[3] == 'p' && chars.len() > 4 && chars[4] != 'e' {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
    }

    (Cow::Borrowed(word), vec![])
}

pub fn remove_prefix_pe(word: &str) -> (Cow<str>, Vec<String>) {
    if word == "pelajar" {
        return (Cow::Borrowed("ajar"), vec![]);
    }
    
    if word.starts_with("pe") && word.len() > 4 {
        let chars: Vec<char> = word.chars().take(6).collect();

        // Pattern 1: pe{l|r|w|y}V => pe-{l|r|w|y}V
        if matches!(chars[2], 'l' | 'r' | 'w' | 'y') && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[2..]), vec![]);
        }

        // Pattern 2: perV => per-V OR pe-rV
        if word.starts_with("per") && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
        }

        // Pattern 3: perCerV => per-CerV where C != 'r'
        if word.starts_with("per") && chars.len() > 5 && is_consonant(chars[3]) && chars[3] != 'r' && chars[4] == 'e' && chars[5] == 'r' {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }

        // Pattern 4: perCP => per-CP where C != 'r' and P != 'er'
        if word.starts_with("per") && is_consonant(chars[3]) && chars[3] != 'r' {
            if chars.len() > 5 && chars[4] == 'e' && chars[5] == 'r' {
                // handled by pattern 3
            } else {
                return (Cow::Borrowed(&word[3..]), vec![]);
            }
        }

        // Pattern 5: pem{b|f|v} => pem-{b|f|v}
        if word.starts_with("pem") && matches!(chars[3], 'b' | 'f' | 'v') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }

        // Pattern 6: pem{rV|V} => pem-{rV|V} OR pe-p{rV|V}
        if word.starts_with("pem") && (is_vowel(chars[3]) || (chars[3] == 'r' && chars.len() > 4 && is_vowel(chars[4]))) {
            return (Cow::Borrowed(&word[3..]), vec!["m".to_string(), "p".to_string()]);
        }

        // Pattern 7: pen{c|d|j|s|t|z} => pen-{c|d|j|s|t|z}
        if word.starts_with("pen") && matches!(chars[3], 'c' | 'd' | 'j' | 's' | 't' | 'z') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }

        // Pattern 8: penV => nV OR tV
        if word.starts_with("pen") && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["n".to_string(), "t".to_string()]);
        }

        // Pattern 9: peng{g|h|q|k} => peng-{g|h|q|k}
        if word.starts_with("peng") && chars.len() > 4 && matches!(chars[4], 'g' | 'h' | 'q' | 'k') {
            return (Cow::Borrowed(&word[4..]), vec![]);
        }

        // Pattern 10: pengV => peng-V OR peng-kV OR pe-ngV OR pengV- where V = 'e'
        if word.starts_with("peng") && chars.len() > 4 && is_vowel(chars[4]) {
            if chars[4] == 'e' {
                return (Cow::Borrowed(&word[4..]), vec![]);
            }
            return (Cow::Borrowed(&word[4..]), vec!["ng".to_string(), "k".to_string()]);
        }

        // Pattern 11: penyV => peny-sV OR pe-nyV
        if word.starts_with("meny") && chars.len() > 4 && is_vowel(chars[4]) {
             // peny- is actually handled by me- mostly, but pe- has similar logic for nominals
             let mut s = "s".to_string();
             s.push_str(&word[4..]);
             return (Cow::Owned(s), vec![]);
        }
    }
    
    (Cow::Borrowed(word), vec![])
}

pub fn remove_prefix_be(word: &str) -> (Cow<str>, Vec<String>) {
    if word == "belajar" {
        return (Cow::Borrowed("ajar"), vec![]);
    }
    if word.starts_with("ber") && word.len() > 3 {
        let chars: Vec<char> = word.chars().collect();
        // Pattern 1: berV => ber-V OR be-rV
        if is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
        }
        // Pattern 2: berCerV => ber-CerV where C != 'r'
        if chars.len() > 5 && is_consonant(chars[3]) && chars[3] != 'r' && chars[4] == 'e' && chars[5] == 'r' {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        // Pattern 3: berCP => ber-CP where C != 'r' and P != 'er'
        if is_consonant(chars[3]) && chars[3] != 'r' {
            if chars.len() > 5 && chars[4] == 'e' && chars[5] == 'r' {
                // handled by p2
            } else {
                return (Cow::Borrowed(&word[3..]), vec![]);
            }
        }
    }
    (Cow::Borrowed(word), vec![])
}
