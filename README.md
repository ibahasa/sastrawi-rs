# sastrawi-rs

> **High-performance Indonesian stemmer for Rust — Zero-regex, zero-copy, FST-powered.**

[![Rust 2024](https://img.shields.io/badge/Rust-2024%20Edition-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/sastrawi-rs)](https://crates.io/crates/sastrawi-rs)
[![docs.rs](https://img.shields.io/docsrs/sastrawi-rs)](https://docs.rs/sastrawi-rs)
[![Downloads](https://img.shields.io/crates/d/sastrawi-rs)](https://crates.io/crates/sastrawi-rs)

A fully modernized Rust 2024 implementation of the **Nazief-Adriani / Enhanced Confix Stripping (ECS)** stemmer for Bahasa Indonesia. Fork of [iDevoid/rust-sastrawi](https://github.com/iDevoid/rust-sastrawi), itself a Rust port of [PHP Sastrawi](https://github.com/sastrawi/sastrawi) by **Andy Librian**.

> **Note:** The crate is published as `sastrawi-rs` on crates.io but imported as `sastrawi` in Rust code (hyphens become underscores per Rust convention).

---

## What's New (vs. original rust-sastrawi)

| Feature                | Old                     | New                                                                 |
| ---------------------- | ----------------------- | ------------------------------------------------------------------- |
| **Engine**             | Regex-based rules       | Zero-regex manual string slicing                                    |
| **Dictionary**         | HashMap on every call   | FST (Finite State Transducer) with `OnceLock`                       |
| **Allocation**         | Heap strings everywhere | `Cow<'a, str>` zero-copy API                                        |
| **Prefix rules**       | Basic me-/ber-          | Full Nazief-Adriani: me-, pe-, ber-, ter-, se-, di-, ke-, ku-, kau- |
| **`menge-`/`penge-`**  | ❌                      | ✅ Monosyllabic base words (mengebom→bom)                           |
| **`nge-`**             | ❌                      | ✅ Informal/colloquial prefix (ngecat→cat)                          |
| **Confix**             | ❌                      | ✅ ke-an, per-an, ber-an, se-nya simultaneous strip                 |
| **Loanword suffixes**  | ❌                      | ✅ -isme, -isasi, -isir, -is                                        |
| **Hyphenated clitics** | ❌                      | ✅ kuasa-Mu, allah-lah, nikmat-Ku                                   |
| **Stopword filter**    | ❌                      | ✅ `stem_sentence_filtered` + `is_stopword`                         |
| **Backtracking**       | Partial                 | Full Longest-Root / Conservative Stemming                           |
| **Edition**            | Rust 2018               | **Rust 2024**                                                       |

---

## 🚀 Usage

Add to `Cargo.toml`:

```toml
[dependencies]
sastrawi-rs = "0.3"
```

### Stem a Sentence

```rust
use sastrawi::{Dictionary, Stemmer};

fn main() {
    let dict = Dictionary::new();
    let stemmer = Stemmer::new(&dict);

    let sentence = "Perekonomian Indonesia sedang dalam pertumbuhan yang membanggakan";
    for word in stemmer.stem_sentence(sentence) {
        print!("{} ", word); // ekonomi indonesia sedang dalam tumbuh yang bangga
    }
}
```

### Stem a Single Word

```rust
use sastrawi::{Dictionary, Stemmer};

let dict = Dictionary::new();
let stemmer = Stemmer::new(&dict);

assert_eq!(stemmer.stem_word("membangunkan").as_ref(), "bangun");
assert_eq!(stemmer.stem_word("keberuntunganmu").as_ref(), "untung");
assert_eq!(stemmer.stem_word("mengebom").as_ref(), "bom");       // menge-
assert_eq!(stemmer.stem_word("ngecat").as_ref(), "cat");          // nge- informal
assert_eq!(stemmer.stem_word("keamanan").as_ref(), "aman");       // ke-an confix
assert_eq!(stemmer.stem_word("pertanian").as_ref(), "tani");      // per-an confix
assert_eq!(stemmer.stem_word("idealisasi").as_ref(), "ideal");    // -isasi loanword
assert_eq!(stemmer.stem_word("kuasa-Mu").as_ref(), "kuasa");      // hyphen clitic
```

### Stopword Filtering

Common function words (_yang, di, dari, dalam, dengan, …_) carry no semantic value for
indexing or NLP analysis. `stem_sentence_filtered` removes them automatically:

```rust
use sastrawi::{Dictionary, Stemmer};

let dict = Dictionary::new();
let stemmer = Stemmer::new(&dict);

let sentence = "Perekonomian Indonesia sedang dalam pertumbuhan yang membanggakan";

// Without filter — all tokens included
let all: Vec<_> = stemmer.stem_sentence(sentence).collect();
// ["ekonomi", "indonesia", "sedang", "dalam", "tumbuh", "yang", "bangga"]

// With stopword filter — function words removed
let filtered: Vec<_> = stemmer.stem_sentence_filtered(sentence).collect();
// ["ekonomi", "indonesia", "tumbuh", "bangga"]

// Check individual words
assert!(stemmer.is_stopword("yang"));     // true
assert!(!stemmer.is_stopword("ekonomi")); // false
```

### Custom Dictionary

```rust
use sastrawi::{Dictionary, Stemmer};

let words = &["aman", "tani", "bangun", "bom"];
let dict = Dictionary::custom(words);
let stemmer = Stemmer::new(&dict);

assert_eq!(stemmer.stem_word("keamanan").as_ref(), "aman");
```

---

## 📦 API Reference

```rust
// Initialization
let dict = Dictionary::new();                   // bundled dictionary (~26k words)
let dict = Dictionary::custom(&["word", ...]); // custom word list
let stemmer = Stemmer::new(&dict);

// Stemming
stemmer.stem_word(word)               // → Cow<'_, str>  (zero-copy when unchanged)
stemmer.stem_sentence(sentence)       // → impl Iterator<Item = Cow<str>>
stemmer.stem_sentence_filtered(sent)  // → Iterator with stopwords removed

// Utilities
stemmer.is_stopword(word)             // → bool
```

---

## 🏗 Architecture

```
sastrawi-rs/
├── src/
│   ├── lib.rs           # Public API re-exports
│   ├── stemmer.rs       # Main engine: Nazief-Adriani pipeline + backtracking
│   ├── affixation.rs    # Prefix/suffix/confix orchestration
│   ├── affix_rules.rs   # Zero-regex morphological rules
│   ├── dictionary.rs    # FST-based dictionary with OnceLock lazy init
│   ├── tokenizer.rs     # Zero-copy &str tokenizer
│   └── stopword.rs      # FST stopword filter
├── data/
│   ├── words.txt        # ~26k root words (Kateglo, CC-BY-NC-SA 3.0)
│   └── stopwords.txt    # Common Indonesian stopwords
├── build.rs             # Compiles word lists → FST at build time
└── tests/test.rs        # 200+ integration test cases across 6 suites
```

### Stemming Pipeline (Nazief-Adriani + ECS)

```
Input word
  │
  ├─ 0. Lowercase + hyphen-clitic strip  (kuasa-Mu → kuasa)
  ├─ 1. Dictionary lookup                → return if found
  ├─ 2. Remove Particle                  (-lah, -kah, -tah, -pun)
  ├─ 3. Remove Possessive                (-ku, -mu, -nya)
  ├─ 4. Remove Suffix + Prefix           (-kan/-an/-i + me-/pe-/ber-/ter-…)
  ├─ 5. ECS Confix                       (ke-an, per-an, ber-an simultaneously)
  ├─ 6. Prefix-only                      (Longest Root preference on original word)
  └─ 7. Pengembalian Akhir               (backtracking over suffix combinations)
```

### Longest-Root / Conservative Stemming

When multiple valid roots exist, we always prefer the **longest** (least-stemmed) result.

```
bersekolah → sekolah  ✓  (not seko — fewer morphemes removed = better fidelity)
```

---

## 🆕 Extensions (2020–2026 Research)

Based on recent Indonesian NLP research (ECS [¹], IndoMorph [²], Aksara v1.5 [³]), we added these modern morphological patterns:

### A. `nge-` Informal Prefix
Colloquial prefix, mirror of `menge-`. Common in Jakarta informal speech and social media (e.g. MPStemmer [⁴]).

```
ngecat    → cat
ngegas    → gas
ngelamar  → lamar
ngelepas  → lepas
```

### B. Confixes — ECS (Enhanced Confix Stripping)
Simultaneous prefix+suffix removal, proven to outperform plain Nazief-Adriani in accuracy [¹].

```
keamanan    → aman    (ke-…-an)
pertanian   → tani    (per-…-an)
berhadapan  → hadap   (ber-…-an)
```

### C. Superlative `se-nya` Particle
```
selengkapnya  → lengkap
seberhasilnya → hasil
```

### D. Loanword Suffixes
```
idealisasi → ideal   (-isasi)
legalisir  → legal   (-isir)
idealisme  → ideal   (-isme)
idealis    → ideal   (-is)
```

---

## 📊 Performance

| Operation         | Old (regex)              | New (zero-regex FST)           |
| ----------------- | ------------------------ | ------------------------------ |
| Dictionary lookup | O(n) HashMap             | O(k) FST where k = key length  |
| Prefix stripping  | Regex compile + match    | Direct string slice comparison |
| Memory            | Regex DFA state machines | FST bytes + OnceLock           |

---

## 🧪 Testing

```bash
cargo test --release
# 6 test suites, 200+ word cases
```

| Suite | Coverage |
|---|---|
| `test_stem_word` | 160+ Nazief-Adriani morphological cases |
| `test_stem_sentence` | Full sentence pipeline |
| `test_nge_informal_prefix` | `ngecat`, `ngegas`, `ngelepas` |
| `test_ecs_confixes` | `keamanan`, `pertanian`, `berhadapan` |
| `test_loanword_suffixes` | `-isasi`, `-isir`, `-isme`, `-is` |
| `test_stopword_filter` | `stem_sentence_filtered`, `is_stopword` |

---

## 📚 References & Credits

- [¹] **ECS**: Arifin, A., Mahendra, P., & Ciptaningtyas, H. T. (2009). _Enhanced Confix Stripping Stemmer and Ants Algorithm for Classifying News Document in Indonesian Language_.
- [²] **IndoMorph**: Kamajaya, I., & Moeljadi, D. (2025). _IndoMorph: a Morphology Engine for Indonesian_. [ACL Anthology](https://aclanthology.org/2025.sealp-1.7/).
- [³] **Aksara**: Universitas Indonesia (2023). _Aksara v1.5: Indonesian NLP tool conforming to UD v2 guidelines_. [GitHub](https://github.com/ir-nlp-csui/aksara).
- [⁴] **MPStemmer**: Prabono, A. G. (2020). _Mpstemmer: a multi-phase stemmer for standard and nonstandard Indonesian words_. [GitHub](https://github.com/ariaghora/mpstemmer).
- **Algorithm**: Nazief & Adriani (1996, 2007) — _"Confix Stripping: Approach to Stemming Algorithm for Bahasa Indonesia"_
- **PHP Sastrawi**: [Andy Librian](https://github.com/andylibrian) — original PHP implementation
- **rust-sastrawi**: [iDevoid](https://github.com/iDevoid/rust-sastrawi) — original Rust port (2019)
- **sastrawi-rs**: [ibahasa Team](https://github.com/ibahasa/sastrawi-rs) — this modernized fork (2026)
- **Dictionary**: [Kateglo](http://kateglo.com/) by Ivan Lanin (CC-BY-NC-SA 3.0)

---

## 📄 License

MIT — see [LICENSE](LICENSE).
Dictionary data: Kateglo (CC-BY-NC-SA 3.0) — non-commercial use only for the bundled word list.
