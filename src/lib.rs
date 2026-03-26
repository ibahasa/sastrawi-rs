#![doc = include_str!("../README.md")]

pub use dictionary::Dictionary;
pub use morph_analyzer::{MorphAnalysis, MorphAnalyzer};
pub use stemmer::Stemmer;
pub use stopword::StopWord;
pub use tokenizer::Tokenizer;

pub mod javanese;

mod affix_rules;
mod affixation;
mod dictionary;
mod morph_analyzer;
mod stemmer;
mod stopword;
mod tokenizer;
