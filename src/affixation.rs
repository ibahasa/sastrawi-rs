extern crate regex;

use crate::dictionary::Dictionary;
use crate::affixation_regex::*;

pub struct Affixation<'a> {
    dictionary: &'a Dictionary,
    // regex for stemmer
    pub prefix_first: regex::Regex,
    particle: regex::Regex,
    possesive: regex::Regex,
    suffix: regex::Regex,

    prefix_me_regex: RegexPrefixMe,
    prefix_pe_regex: [(regex::Regex, Vec<String>); 15],
    prefix_be_regex: [(regex::Regex, Vec<String>); 5],
    prefix_te_regex: [(regex::Regex, Vec<String>); 5],
    infix_regex: [regex::Regex; 2],
}

impl<'a> Affixation<'a> {
    pub fn new(dict: &Dictionary) -> Affixation<'_> {
        Affixation{
            dictionary: dict,
            prefix_first: regex::Regex::new(r"^(be.+lah|be.+an|me.+i|di.+i|pe.+i|ter.+i)$").unwrap(),
            particle: regex::Regex::new(r"-*(lah|kah|tah|pun)$").unwrap(),
            possesive: regex::Regex::new(r"-*(ku|mu|nya)$").unwrap(),
            suffix: regex::Regex::new(r"-*(is|isme|isasi|i|kan|an)$").unwrap(),
            prefix_me_regex: assign_regex_prefix_me(),
            prefix_pe_regex: assign_regex_prefix_pe(),
            prefix_be_regex: assign_regex_prefix_be(),
            prefix_te_regex: assign_regex_prefix_te(),
            infix_regex: assign_regex_infix(),
        }
    }

    pub fn remove_prefixes(&self, word: &String) -> (bool, String) {
        let original_word = word.clone();
        let mut mutable_word = word.clone();
        let mut removed_prefix = String::from("");
        let mut recoding_char: Vec<String>;

        for _ in 0..3 {
            if mutable_word.len() < 3 {
                return (false, original_word)
            }

            if removed_prefix == mutable_word[..2] {
                break
            }

            let prefixes = self.remove_prefix(&mutable_word);
            removed_prefix = prefixes.0;
            mutable_word = prefixes.1;
            recoding_char = prefixes.2;
            if self.dictionary.find(&mutable_word) {
                return (true, mutable_word.to_owned())
            }

            for character in recoding_char {
                let mut char_word = character.to_string();
                char_word.push_str(&mutable_word);
                if self.dictionary.find(&char_word) {
                    return (true, char_word)
                }
            }
        }

        (false, mutable_word.to_owned())
    }

    pub fn remove_particle(&self, word: &String) -> (String, String) {
        let result = self.particle.replace_all(word, "").to_string();
        let particle = word.replace(&result, "");
        (particle, result)
    }

    pub fn remove_possesive(&self, word: &String) -> (String, String) {
        let result = self.possesive.replace_all(word, "").to_string();
        let possesive = word.replace(&result, "");
        (possesive, result)
    }

    pub fn remove_suffix(&self, word: &String) -> (String, String) {
        let result = self.suffix.replace_all(word, "").to_string();
        let suffix = word.replace(&result, "");
        (suffix, result)
    }

    pub fn pengembalian_akhir(&self, original_word: &String, suffixes: &Vec<String>) -> (bool, String) {
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

            let (found, word) = self.remove_prefixes(&word);
            if found {
                return (true, word)
            }
        }

        (false, original_word.to_owned())
    }

    fn remove_prefix(&self, word: &String) -> (String, String, Vec<String>) {
        let mut recoding: Vec<String> = vec![];

        if word.starts_with("kau") {
            return ("kau".to_string(), word[3..].to_string(), recoding)
        }
        
        let prefix: String = word[..2].to_string();
        let mut _result: String = String::from("");
        
        match prefix.as_str() {
            "di" | "ke" | "se" | "ku" => {
                _result = word[2..].to_string();
            },
            "me" => {
                let me = self.remove_prefix_me(word);
                _result = me.0;
                recoding = me.1;
            },
            "pe" => {
                let pe = self.remove_prefix_pe(word);
                _result = pe.0;
                recoding = pe.1;
            },
            "be" => {
                let be = self.remove_prefix_be(word);
                _result = be.0;
                recoding = be.1;
            },
            "te" => {
                let te = self.remove_prefix_te(word);
                _result = te.0;
                recoding = te.1;
            },
            _ => {
                let infix = self.remove_infix(word);
                _result = infix.0;
                recoding = infix.1;
            }
        }

        (prefix, _result, recoding)
    }

    fn remove_prefix_me(&self, word: &String) -> (String, Vec<String>) {
        // Pattern 1
        // me{l|r|w|y}V => me-{l|r|w|y}V
        let mut captured = self.prefix_me_regex.pattern1.captures(word);
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![])
        }

        // Pattern 2
	    // mem{b|f|v} => mem-{b|f|v}
        captured = self.prefix_me_regex.pattern2.captures(word);
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![])
        }

        // Pattern 3
        // mempe => mem-pe
        captured = self.prefix_me_regex.pattern3.captures(word); 
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![])
        }

        // Pattern 4
        // mem{rV|V} => mem-{rV|V} OR me-p{rV|V}
        captured = self.prefix_me_regex.pattern4.captures(word);
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![String::from("m"), String::from("p")])
        }

        // Pattern 5
        // men{c|d|j|s|t|z} => men-{c|d|j|s|t|z}
        captured = self.prefix_me_regex.pattern5.captures(word);
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![])
        }

        // Pattern 6
	    // menV => nV OR tV
        captured = self.prefix_me_regex.pattern6.captures(word);
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![String::from("n"), String::from("t")])
        }

        // Pattern 7
        // meng{g|h|q|k} => meng-{g|h|q|k}
        captured = self.prefix_me_regex.pattern7.captures(word);
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![])
        }

        // Pattern 8
        // mengV => meng-V OR meng-kV OR me-ngV OR mengV- where V = 'e'
        captured = self.prefix_me_regex.pattern8.captures(word);
        if captured.is_some() {
            let unwrapped = captured.unwrap();
            if &unwrapped.get(2).map_or(String::from(""), |m| m.as_str().to_string()) == "e" {
                return (unwrapped.get(3).map_or(String::from(""), |m| m.as_str().to_string()), vec![])
            }
            return (unwrapped.get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![String::from("ng"), String::from("k")])
        }

        // Pattern 9
        // menyV => meny-sV OR me-nyV to stem menyala
        captured = self.prefix_me_regex.pattern9.captures(word);
        if captured.is_some() {
            let mut result = String::from("s");
            let unwrapped = captured.unwrap();
            if &unwrapped.get(2).map_or(String::from(""), |m| m.as_str().to_string()) == "a" {
                result = String::from("ny");
            }
            result.push_str(&unwrapped.get(1).map_or(String::from(""), |m| m.as_str().to_string()));

            return (result, vec![])
        }

        // Pattern 10
        // mempV => mem-pA where A != 'e'
        captured = self.prefix_me_regex.pattern10.captures(word);
        if captured.is_some() {
            return (captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()), vec![])
        }

        (word.to_owned(), vec![])
    }

    fn remove_prefix_pe(&self, word: &String) -> (String, Vec<String>) {
        if word == "pelajar" {
            return (String::from("ajar"), vec![])
        }
        for (rx, recoding) in self.prefix_pe_regex.iter() {
            let _captured = rx.captures(word);
            if _captured.is_some() {
                let _unwrapped = _captured.unwrap();
                // this should only be captured by pattern 10
                if &_unwrapped.get(2).map_or(String::from(""), |m| m.as_str().to_string()) == "e" {
                    return (
                        _unwrapped.get(3).map_or(String::from(""), |m| m.as_str().to_string()), 
                        vec![]
                    )
                }

                return (
                    _unwrapped.get(1).map_or(String::from(""), |m| m.as_str().to_string()),
                    recoding.to_owned()
                )
            }
        }
        (word.to_owned(), vec![])
    }

    fn remove_prefix_be(&self, word: &String) -> (String, Vec<String>) {
        for (rx, recoding) in self.prefix_be_regex.iter() {
            let _captured = rx.captures(word);
            if _captured.is_some() {
                return (
                    _captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()),
                    recoding.to_owned()
                )
            }
        }
        (word.to_owned(), vec![])
    }

    fn remove_prefix_te(&self, word: &String) -> (String, Vec<String>) {
        for (rx, recoding) in self.prefix_te_regex.iter() {
            let _captured = rx.captures(word);
            if _captured.is_some() {
                return (
                    _captured.unwrap().get(1).map_or(String::from(""), |m| m.as_str().to_string()),
                    recoding.to_owned()
                )
            }
        }
        (word.to_owned(), vec![])
    }

    fn remove_infix(&self, word: &String) -> (String, Vec<String>) {
        for rx in self.infix_regex.iter() {
            let _captured = rx.captures(word);
            if _captured.is_some() {
                let unwrapped = _captured.unwrap();
                return (
                    unwrapped.get(3).map_or(String::from(""), |m| m.as_str().to_string()), 
                    vec![
                        unwrapped.get(1).map_or(String::from(""), |m| m.as_str().to_string()), 
                        unwrapped.get(2).map_or(String::from(""), |m| m.as_str().to_string())
                    ]
                )
            }
        }
        (word.to_owned(), vec![])
    }
}

struct RegexPrefixMe {
    pattern1: regex::Regex,
    pattern2: regex::Regex,
    pattern3: regex::Regex,
    pattern4: regex::Regex,
    pattern5: regex::Regex,
    pattern6: regex::Regex,
    pattern7: regex::Regex,
    pattern8: regex::Regex,
    pattern9: regex::Regex,
    pattern10: regex::Regex,
}

fn assign_regex_prefix_me() -> RegexPrefixMe {
    RegexPrefixMe{
        pattern1: regex::Regex::new(r"^me([lrwy][aiueo].*)$").unwrap(),
        pattern2: regex::Regex::new(r"^mem([bfv].*)$").unwrap(),
        pattern3: regex::Regex::new(r"^mem(pe.*)$").unwrap(),
        pattern4: regex::Regex::new(r"^mem(r?[aiueo].*)$").unwrap(),
        pattern5: regex::Regex::new(r"^men([cdjstz].*)$").unwrap(),
        pattern6: regex::Regex::new(r"^men([aiueo].*)$").unwrap(),
        pattern7: regex::Regex::new(r"^meng([ghqk].*)$").unwrap(),
        pattern8: regex::Regex::new(r"^meng(([aiueo])(.*))$").unwrap(),
        pattern9: regex::Regex::new(r"^meny(([aiueo])(.*))$").unwrap(),
        pattern10: regex::Regex::new(r"^mem(p[^e].*)$").unwrap(),
    }
}