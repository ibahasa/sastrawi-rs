use std::sync::OnceLock;
use fst::Set;

pub struct Dictionary {
    set: Set<&'static [u8]>,
}

static DEFAULT_SET: OnceLock<Set<&'static [u8]>> = OnceLock::new();
static STOPWORD_SET: OnceLock<Set<&'static [u8]>> = OnceLock::new();

impl Dictionary {
    pub fn new() -> Dictionary {
        let set = DEFAULT_SET.get_or_init(|| {
            let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/dictionary.fst"));
            Set::new(&bytes[..]).expect("failed to load default fst")
        });
        Dictionary { set: set.clone() }
    }

    pub fn stopword() -> Dictionary {
        let set = STOPWORD_SET.get_or_init(|| {
            let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/stopwords.fst"));
            Set::new(&bytes[..]).expect("failed to load stopword fst")
        });
        Dictionary { set: set.clone() }
    }

    pub fn custom(words: &[&str]) -> Dictionary {
        let mut sorted_words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        sorted_words.sort();
        sorted_words.dedup();
        
        let mut build = fst::SetBuilder::memory();
        for word in sorted_words {
            build.insert(word).unwrap();
        }
        let bytes = build.into_inner().unwrap();
        // custom dictionary is NOT static, so we need a different approach for storage
        // for now let's just leak it to simplify the demo or fix later
        let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        Dictionary {
            set: Set::new(leaked).unwrap(),
        }
    }

    pub fn find(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();
        let found = self.set.contains(&word_lower);
        found
    }

    pub fn length(&self) -> usize {
        self.set.len()
    }

    pub fn add<'a>(&'a mut self, _word: &str) {
        // FST is immutable. For mutability, we should use a hybrid approach
        // but for Sastrawi core, the dictionary is usually static.
        unimplemented!("FST dictionary is immutable. Use Dictionary::custom for modifications.")
    }

    pub fn remove<'a>(&'a mut self, _word: &str) {
        unimplemented!("FST dictionary is immutable.")
    }
}