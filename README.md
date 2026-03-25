# Sastrawi-rs

Modern Indonesian Stemmer (Sastrawi-rs). Reviving Indonesian NLP in Rust with memory-efficient structures and full PUEBI compatibility.

This library is a modernized Rust implementation of the [Sastrawi](https://github.com/sastrawi/sastrawi) algorithm, allowing for high-speed stemming and linguistic analysis of Bahasa Indonesia.

## Features
- **High Performance**: Optimized for modern Rust (2024+ standards) with minimal memory overhead.
- **Nazief-Adriani Algorithm**: Core implementation based on standard Indonesian stemming rules.
- **PUEBI Compatible**: Enhanced handling for modern formal Indonesian grammar.
- **Dictionary Driven**: Highly dependent on a base word dictionary for accurate stemming.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sastrawi = { path = "../sastrawi-rs" } # or version if published
```

### Basic Example

```rust
use sastrawi::*;

fn main() {
    let dict = Dictionary::new();
    let stemmer = Stemmer::new(&dict);
    
    let sentence = "Perekonomian Indonesia sedang dalam pertumbuhan";
    let stemmed_words = stemmer.stem_sentence(sentence);
    
    for word in stemmed_words {
        println!("{}", word);
    }
}
```

## Background & Credits
This project is a modernized fork of the original `rust-sastrawi`, based on the work by [Andy Librian](https://github.com/andylibrian) (PHP Sastrawi). It utilizes the Enhanced Confix Stripping Stemmer technique for Indonesian text retrieval.

## License
Distributed under the MIT License. Base dictionary data sourced from Kateglo (CC-BY-NC-SA 3.0).
