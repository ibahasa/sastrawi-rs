use fst::Set;
use std::sync::OnceLock;

/// A dictionary used for Javanese stemming and stopword lookups.
///
/// It uses a Finite State Transducer (FST) for storage, ensuring
/// minimal memory overhead and extremely fast lookups.
pub struct JavaneseDictionary {
    set: Set<&'static [u8]>,
}

static JAVANESE_SET: OnceLock<Set<&'static [u8]>> = OnceLock::new();

impl JavaneseDictionary {
    /// Loads the default Javanese dictionary (Ngoko, Krama Alus, Krama Inggil).
    ///
    /// The dictionary is compiled into the binary as an FST and
    /// lazily initialized upon the first call to this method.
    ///
    /// # Example
    /// ```
    /// use sastrawi::javanese::JavaneseDictionary;
    /// let dict = JavaneseDictionary::new();
    /// assert!(dict.find("mangan") || dict.find("turu"));
    /// ```
    pub fn new() -> JavaneseDictionary {
        let set = JAVANESE_SET.get_or_init(|| {
            let bytes = include_bytes!(concat!(env!("OUT_DIR"), "/javanese.fst"));
            Set::new(&bytes[..]).expect("failed to load default javanese fst")
        });
        JavaneseDictionary { set: set.clone() }
    }

    /// Creates a custom dictionary from a word list.
    ///
    /// Words are automatically sorted and deduplicated before
    /// being compiled into an FST in memory.
    pub fn custom(words: &[&str]) -> JavaneseDictionary {
        let mut sorted_words: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        sorted_words.sort();
        sorted_words.dedup();

        let mut build = fst::SetBuilder::memory();
        for word in sorted_words {
            build.insert(word).unwrap();
        }
        let bytes = build.into_inner().unwrap();
        let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        JavaneseDictionary {
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
}
