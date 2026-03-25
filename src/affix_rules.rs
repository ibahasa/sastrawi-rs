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
    for s in ["kan", "an", "i"] {
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

    // Pattern 9: menyV => meny-sV OR me-nyV
    if word.starts_with("meny") && chars.len() > 4 && is_vowel(chars[4]) {
        if chars[4] == 'a' && !word.starts_with("menyanyi") {
             return (Cow::Borrowed(&word[2..]), vec![]);
        }
        let mut s = "s".to_string();
        s.push_str(&word[4..]);
        return (Cow::Owned(s), vec![]);
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
        let chars: Vec<char> = word.chars().take(6).collect();

        if matches!(chars[2], 'l' | 'r' | 'w' | 'y') && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[2..]), vec![]);
        }
        if word.starts_with("per") && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
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
        if word.starts_with("pem") && matches!(chars[3], 'b' | 'f' | 'v') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        if word.starts_with("pem") && (is_vowel(chars[3]) || (chars[3] == 'r' && chars.len() > 4 && is_vowel(chars[4]))) {
            return (Cow::Borrowed(&word[3..]), vec!["m".to_string(), "p".to_string()]);
        }
        if word.starts_with("pen") && matches!(chars[3], 'c' | 'd' | 'j' | 's' | 't' | 'z') {
            return (Cow::Borrowed(&word[3..]), vec![]);
        }
        if word.starts_with("pen") && is_vowel(chars[3]) {
            return (Cow::Borrowed(&word[3..]), vec!["n".to_string(), "t".to_string()]);
        }
        if word.starts_with("peng") && chars.len() > 4 && matches!(chars[4], 'g' | 'h' | 'q' | 'k') {
            return (Cow::Borrowed(&word[4..]), vec![]);
        }
        if word.starts_with("peng") && chars.len() > 4 && is_vowel(chars[4]) {
            if chars[4] == 'e' {
                return (Cow::Borrowed(&word[4..]), vec![]);
            }
            return (Cow::Borrowed(&word[4..]), vec!["ng".to_string(), "k".to_string()]);
        }
    }
    
    (Cow::Borrowed(word), vec![])
}

pub fn remove_prefix_be<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word == "belajar" {
        return (Cow::Borrowed("ajar"), vec![]);
    }
    if word.starts_with("ber") && word.len() > 3 {
        let chars: Vec<char> = word.chars().collect();
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
    }
    (Cow::Borrowed(word), vec![])
}

pub fn remove_prefix_te<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    if word.starts_with("ter") && word.len() > 3 {
         let chars: Vec<char> = word.chars().take(6).collect();
         if is_vowel(chars[3]) {
             return (Cow::Borrowed(&word[3..]), vec!["r".to_string()]);
         }
         if chars.len() > 5 && is_consonant(chars[3]) && chars[3] != 'r' && chars[4] == 'e' && chars[5] == 'r' {
             return (Cow::Borrowed(&word[3..]), vec![]);
         }
         if is_consonant(chars[3]) && chars[3] != 'r' {
              return (Cow::Borrowed(&word[3..]), vec![]);
         }
    }
    (Cow::Borrowed(word), vec![])
}

pub fn remove_infix<'a>(word: &'a str) -> (Cow<'a, str>, Vec<String>) {
    // Pattern: -el-, -er-, -em-, -in-
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
