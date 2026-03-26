use fst::Set;
use std::sync::OnceLock;

/// A dictionary used for stemming and stopword lookups.
///
/// It uses a Finite State Transducer (FST) for storage, ensuring
/// minimal memory overhead and extremely fast lookups.
pub struct Dictionary {
    set: Set<&'static [u8]>,
}

static DEFAULT_SET: OnceLock<Set<&'static [u8]>> = OnceLock::new();
static STOPWORD_SET: OnceLock<Set<&'static [u8]>> = OnceLock::new();

impl Dictionary {
    /// Loads the default Indonesian dictionary (~26k words).
    ///
    /// The dictionary is compiled into the binary as an FST and
    /// lazily initialized upon the first call to this method.
    ///
    /// # Example
    /// ```
    /// use sastrawi::Dictionary;
    /// let dict = Dictionary::new();
    /// assert!(dict.find("pohon"));
    /// ```
    pub fn new() -> Dictionary {
        let set = DEFAULT_SET.get_or_init(|| {
            let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/dictionary.fst"));
            Set::new(&bytes[..]).expect("failed to load default fst")
        });
        Dictionary { set: set.clone() }
    }

    /// Loads the default Indonesian stopword list.
    ///
    /// Useful for filtering common words like "yang", "di", "dari".
    ///
    /// # Example
    /// ```
    /// use sastrawi::Dictionary;
    /// let dict = Dictionary::stopword();
    /// assert!(dict.find("yang"));
    /// ```
    pub fn stopword() -> Dictionary {
        let set = STOPWORD_SET.get_or_init(|| {
            let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/stopwords.fst"));
            Set::new(&bytes[..]).expect("failed to load stopword fst")
        });
        Dictionary { set: set.clone() }
    }

    /// Creates a custom dictionary from a word list.
    ///
    /// Words are automatically sorted and deduplicated before
    /// being compiled into an FST in memory.
    ///
    /// # Example
    /// ```
    /// use sastrawi::Dictionary;
    /// let dict = Dictionary::custom(&["adu", "ajar", "adu"]);
    /// assert_eq!(dict.length(), 2);
    /// assert!(dict.find("adu"));
    /// ```
    pub fn custom(words: &[&str]) -> Dictionary {
        let mut sorted_words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        sorted_words.sort();
        sorted_words.dedup();

        let mut build = fst::SetBuilder::memory();
        for word in sorted_words {
            build.insert(word).unwrap();
        }
        let bytes = build.into_inner().unwrap();
        // Since FST relies on bytes, we leak the boxed slice to ensure
        // the dictionary outlives the build process.
        let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        Dictionary {
            set: Set::new(leaked).unwrap(),
        }
    }

    /// Checks if a word exists in the dictionary.
    ///
    /// Case-insensitive. Returns true if found.
    pub fn find(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();
        self.set.contains(&word_lower)
    }

    /// Returns the number of words in the dictionary.
    pub fn length(&self) -> usize {
        self.set.len()
    }

    /// Adding words individually is not supported for FST dictionaries.
    /// Use `Dictionary::custom` to modify the word set.
    pub fn add<'a>(&'a mut self, _word: &str) {
        unimplemented!("FST dictionary is immutable. Use Dictionary::custom for modifications.")
    }

    /// Removal is not supported for FST dictionaries.
    pub fn remove<'a>(&'a mut self, _word: &str) {
        unimplemented!("FST dictionary is immutable.")
    }
}
