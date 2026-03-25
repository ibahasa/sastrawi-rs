# sastrawi-rs

> **High-performance Indonesian stemmer for Rust — Zero-regex, zero-copy, FST-powered.**

[![Rust 2024](https://img.shields.io/badge/Rust-2024%20Edition-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-sastrawi-blue)](https://crates.io/crates/sastrawi)

A fully modernized Rust 2024 implementation of the **Nazief-Adriani / Enhanced Confix Stripping (ECS)** stemmer for Bahasa Indonesia. Fork of [iDevoid/rust-sastrawi](https://github.com/iDevoid/rust-sastrawi), itself a Rust port of [PHP Sastrawi](https://github.com/sastrawi/sastrawi) by **Andy Librian**.

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
| **Backtracking**       | Partial                 | Full Longest-Root / Conservative Stemming                           |
| **Edition**            | Rust 2018               | **Rust 2024**                                                       |

---

## 🚀 Usage

Add to `Cargo.toml`:

```toml
[dependencies]
sastrawi = { git = "https://github.com/ibahasa/sastrawi-rs" }
```

### Stem a Sentence

```rust
use sastrawi::{Dictionary, Stemmer};

fn main() {
    let dict = Dictionary::new(); // loads FST dictionary
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
assert_eq!(stemmer.stem_word("mengebom").as_ref(), "bom");      // menge-
assert_eq!(stemmer.stem_word("ngecat").as_ref(), "cat");         // nge- informal
assert_eq!(stemmer.stem_word("keamanan").as_ref(), "aman");      // ke-an confix
assert_eq!(stemmer.stem_word("pertanian").as_ref(), "tani");     // per-an confix
assert_eq!(stemmer.stem_word("idealisasi").as_ref(), "ideal");   // -isasi loanword
assert_eq!(stemmer.stem_word("kuasa-Mu").as_ref(), "kuasa");     // hyphen clitic
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

## 🏗 Architecture

```
sastrawi-rs/
├── src/
│   ├── lib.rs           # Public API re-exports
│   ├── stemmer.rs       # Main engine: Nazief-Adriani 5-step pipeline + backtracking
│   ├── affixation.rs    # Prefix/suffix/confix orchestration + pengembalian_akhir
│   ├── affix_rules.rs   # Zero-regex morphological rules (all prefix/suffix patterns)
│   ├── dictionary.rs    # FST-based dictionary with OnceLock lazy init
│   ├── tokenizer.rs     # Zero-copy &str tokenizer
│   └── stopword.rs      # FST stopword filter
├── data/
│   ├── words.txt        # ~26k root words (Kateglo, CC-BY-NC-SA 3.0)
│   └── stopwords.txt    # Common Indonesian stopwords
├── build.rs             # Compiles words.txt → dictionary.fst at build time
└── tests/test.rs        # 170+ integration test cases
```

### Stemming Pipeline (Nazief-Adriani + ECS)

```
Input word
  │
  ├─ 0. Lowercase + hyphen-clitic strip (kuasa-Mu → kuasa)
  ├─ 1. Dictionary lookup  → return if found
  ├─ 2. Remove Particle    (-lah, -kah, -tah, -pun, -se-nya)
  ├─ 3. Remove Possessive  (-ku, -mu, -nya)
  ├─ 4. Remove Suffix + Prefix  (-kan/-an/-i + me-/pe-/ber-/ter-...)
  ├─ 5. Remove Confix      (ke-an, per-an, ber-an simultaneously)
  ├─ 6. Prefix-only (Longest Root preference on original word)
  └─ 7. Pengembalian Akhir (backtracking over suffix combinations)
```

### Longest-Root / Conservative Stemming

When multiple valid roots exist, we always prefer the **longest** (least-stemmed) result.
Example: `bersekolah` could produce `seko` (via ber-+seko) or `sekolah` (via ber-).
We prefer `sekolah` — fewer morphemes removed = better fidelity.

---

## 🆕 Extensions (2020–2026 Research)

Based on recent Indonesian NLP research (ECS, IndoMorph, Aksara), we added:

### A. `nge-` Informal Prefix

Colloquial/lisan prefix, mirror of `menge-`. Common in Jakarta informal speech and social media.

```
ngecat     → cat
ngegas     → gas
ngerasain  → rasa
ngelamar   → lamar
```

### B. Confixes (ECS — Enhanced Confix Stripping)

Simultaneous prefix+suffix removal, proven to outperform plain Nazief-Adriani.

```
keamanan    → aman     (ke- + -an)
pertanian   → tani     (per- + -an)
berhadapan  → hadap    (ber- + -an)
```

### C. Superlative `se-nya` Particle

```
selengkapnya  → lengkap
seberhasilnya → berhasil → hasil
```

### D. Loanword Suffixes

```
idealisasi  → ideal   (-isasi)
legalisir   → legal   (-isir)
idealisme   → ideal   (-isme) [already in original]
idealis     → ideal   (-is)   [already in original]
```

---

## 📊 Performance

The zero-regex FST engine is significantly faster than the legacy implementation:

| Operation         | Old (regex)              | New (zero-regex FST)           |
| ----------------- | ------------------------ | ------------------------------ |
| Dictionary lookup | O(n) HashMap             | O(k) FST where k=key length    |
| Prefix stripping  | Regex compile + match    | Direct string slice comparison |
| Memory            | Regex DFA state machines | Minimal — FST bytes + OnceLock |

---

## 🧪 Testing

```bash
cargo test --release
# 2 integration test suites, 170+ word cases
```

---

## 📚 References & Credits

- **Algorithm**: Nazief & Adriani (1996, 2007) — _"Confix Stripping: Approach to Stemming Algorithm for Bahasa Indonesia"_
- **ECS**: A. Larasati et al. — _Enhanced Confix Stripping Stemmer_
- **PHP Sastrawi**: [Andy Librian](https://github.com/andylibrian) — original PHP implementation
- **rust-sastrawi**: [iDevoid](https://github.com/iDevoid/rust-sastrawi) — original Rust port (2019)
- **sastrawi-rs**: [ibahasa Team](https://github.com/ibahasa/sastrawi-rs) — this modernized fork (2026)
- **Dictionary**: [Kateglo](http://kateglo.com/) by Ivan Lanin (CC-BY-NC-SA 3.0)
- **Research consulted**: IndoMorph (ACL 2025), Aksara v1.5 (UI-NLP, 2023), MPStemmer (2020)

---

## 📄 License

MIT — see [LICENSE](LICENSE).
Dictionary data: Kateglo (CC-BY-NC-SA 3.0) — non-commercial use only for the bundled word list.
