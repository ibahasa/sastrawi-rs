# sastrawi-rs

> **High-performance Indonesian & Javanese stemmer for Rust — Zero-regex, zero-copy, FST-powered.**

[![Rust 2024](https://img.shields.io/badge/Rust-2024%20Edition-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/sastrawi-rs)](https://crates.io/crates/sastrawi-rs)
[![docs.rs](https://img.shields.io/docsrs/sastrawi-rs)](https://docs.rs/sastrawi-rs)
[![Downloads](https://img.shields.io/crates/d/sastrawi-rs)](https://crates.io/crates/sastrawi-rs)

A fully modernized Rust 2024 implementation of stemming algorithms for Bahasa Indonesia and Bahasa Jawa. Based on the **Nazief-Adriani / Enhanced Confix Stripping (ECS)** algorithm for Indonesian, and a dedicated multi-strata engine (Ngoko, Krama Alus, Krama Inggil) for Javanese. Fork of [iDevoid/rust-sastrawi](https://github.com/iDevoid/rust-sastrawi), itself a Rust port of [PHP Sastrawi](https://github.com/sastrawi/sastrawi) by **Andy Librian**.

> **Note:** The crate is published as `sastrawi-rs` on crates.io but imported as `sastrawi` in Rust code (hyphens become underscores per Rust convention).

---

## Quick Start

Add to `Cargo.toml`:

```toml
[dependencies]
sastrawi-rs = "0.5"
```

This crate provides **two independent stemmers** that share the same zero-copy, FST-based architecture:

| Stemmer | Import Path | Dictionary | Language |
|---|---|---|---|
| **Indonesian** | `sastrawi::{Dictionary, Stemmer}` | ~26k root words | Bahasa Indonesia |
| **Javanese** | `sastrawi::javanese::{JavaneseDictionary, JavaneseStemmer}` | ~1.3k root words | Bahasa Jawa (Ngoko/Krama) |

They are **completely independent** — changing one has zero effect on the other.

---

## 🇮🇩 Indonesian Stemmer

### What's New (vs. original rust-sastrawi)

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

### Usage

```rust,ignore
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

```rust,ignore
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

```rust,ignore
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

assert!(stemmer.is_stopword("yang"));     // true
assert!(!stemmer.is_stopword("ekonomi")); // false
```

### Custom Dictionary

```rust,ignore
use sastrawi::{Dictionary, Stemmer};

let words = &["aman", "tani", "bangun", "bom"];
let dict = Dictionary::custom(words);
let stemmer = Stemmer::new(&dict);

assert_eq!(stemmer.stem_word("keamanan").as_ref(), "aman");
```

### Indonesian API Reference

```rust,ignore
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

### Indonesian Stemming Pipeline (Nazief-Adriani + ECS)

```text
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

---

## 🫙 Javanese Stemmer (Bahasa Jawa) 🆕

`v0.4.0` introduces a dedicated Universal Javanese stemmer based on adaptations of the Nazief-Adriani algorithm for Javanese [⁵][⁶]. It is fully isolated from the Indonesian stemmer — different dictionary, different pipeline, different module.

### Usage

```rust,ignore
use sastrawi::javanese::{JavaneseDictionary, JavaneseStemmer};

// Uses the bundled Javanese dictionary (~1.3k pure root words)
let jv_dict = JavaneseDictionary::new();
let jv_stemmer = JavaneseStemmer::new(&jv_dict);

// Ngoko — Anuswara Meluluhkan
assert_eq!(jv_stemmer.stem_word("mangan").as_ref(), "pangan");    // m- + pangan
assert_eq!(jv_stemmer.stem_word("nulis").as_ref(), "tulis");       // n- + tulis

// Ngoko — Anuswara Menempel
assert_eq!(jv_stemmer.stem_word("ndawuhi").as_ref(), "dawuh");     // n- attaches to d
assert_eq!(jv_stemmer.stem_word("mbalang").as_ref(), "balang");    // m- attaches to b

// Krama Passives + Causatives
assert_eq!(jv_stemmer.stem_word("dipunjupuk").as_ref(), "jupuk");
assert_eq!(jv_stemmer.stem_word("lampahaken").as_ref(), "lampah");

// Circumfix Backtracking
assert_eq!(jv_stemmer.stem_word("nglebetaken").as_ref(), "lebet"); // ng-…-aken

// Monosyllabic root (nge-)
assert_eq!(jv_stemmer.stem_word("ngecet").as_ref(), "cet");
```

### Custom Dictionary

```rust,ignore
use sastrawi::javanese::{JavaneseDictionary, JavaneseStemmer};

let roots = &["gawa", "jupuk", "tulis", "dawuh"];
let jv_dict = JavaneseDictionary::custom(roots);
let jv_stemmer = JavaneseStemmer::new(&jv_dict);

assert_eq!(jv_stemmer.stem_word("nggawa").as_ref(), "gawa");
assert_eq!(jv_stemmer.stem_word("ndawuhi").as_ref(), "dawuh");
```

### Javanese Affix Rules Reference

All rules are derived from academic paper adaptations of Nazief-Adriani for Javanese [⁵][⁶][⁷].

#### 1. Ater-ater Anuswara (Nasalization) [⁶]

The most complex aspect of Javanese morphology. Rules differ by whether the nasal *replaces* the initial consonant (*Meluluhkan*) or simply *prepends* to it (*Menempel*).

| Prefix | Meluluhkan (replaces) | Menempel (attaches to) | Vowel-initial |
|---|---|---|---|
| `m-` | `p → m` (*macul←pacul*), `w → m` (*maca←waca*) | `b` (*mbalang←balang*) | ✅ (*munggah←unggah*) |
| `n-` | `t → n` (*nulis←tulis*), `th → n` (*nuthuk←thuthuk*) | `d`, `dh`, `j` (*ndawuh←dawuh*, *njupuk←jupuk*) | ✅ |
| `ng-` | `k → ng` (*ngirim←kirim*) | `g` (*ngguyu←guyu*) | ✅ (*ngombe←ombe*) |
| `ny-` | `s → ny` (*nyapu←sapu*), `c → ny` (*nyekel←cekel*) | — | — |
| `nge-` | Monosyllabic roots (*ngecet←cet*, *ngecat←cat*) [special] | — | — |

#### 2. Ater-ater Tripurusa & General Prefixes

| Category | Prefixes |
|---|---|
| Krama Passive | `dipun-` |
| Tripurusa Ngoko | `di-`, `dak-`, `tak-`, `kok-`, `ko-` |
| Nominal Derivation [⁷] | `pan-`, `pam-`, `pang-` (allomorphs before labial/velar) |
| General | `pa-`, `pi-`, `ka-`, `sa-`, `ma-`, `ke-`, `pra-` |
| Formal/Literary [⁷] | `kuma-`, `kapi-`, `we-`, `a-`, `ben-` |
| Dialectal (Jawa Timur) [⁷] | `tar-`, `tok-` |

#### 3. Panambang (Suffixes) & Allomorphs

| Category | Suffixes |
|---|---|
| Particles | `-a`, `-i`, `-e`, `-en`, `-an`, `-na`, `-no` (Dialect) |
| Causative | `-ake` (Ngoko), `-aken` (Krama) |
| Possessives | `-ku`, `-mu`, `-ne`, `-ane` (allomorph), `-ipun` (Krama) |
| Vowel Sandhi Allomorphs [⁶] | `-kake`, `-kaken` (*gunakake←guna*), `-ni` (*larani←lara*), `-nan` |
| Complex suffixes | `-ana`, `-nan` |

#### 4. Circumfix Backtracking (Confiks)

The pipeline performs exhaustive suffix-then-prefix stripping, meaning all circumfix combinations are resolved automatically without hardcoded confix rules. Examples:

```text
dipunlampahaken → lampah   (dipun- … -aken)
nggunakake      → guna     (ng- … -kake allomorph)
nglebetaken     → lebet    (ng- … -aken)
```

### Javanese API Reference

```rust,ignore
// Initialization
let jv_dict = JavaneseDictionary::new();                 // bundled (~1.3k root words)
let jv_dict = JavaneseDictionary::custom(&["w1", ...]); // custom list (for testing)
let jv_stemmer = JavaneseStemmer::new(&jv_dict);

// Identical API surface as Indonesian stemmer
jv_stemmer.stem_word(word)          // → Cow<'_, str>
jv_stemmer.stem_sentence(sentence)  // → impl Iterator<Item = Cow<str>>
```

### Javanese Stemming Pipeline

```text
Input word
  │
  ├─ 0. Lowercase + hyphen strip (ngisin-isini → isin via first segment)
  ├─ 1. Dictionary lookup        → return if found
  ├─ 2. Min length guard (< 3)   → return as-is
  ├─ 3. Exhaustive Suffix scan   → try ALL possessives × ALL particles
  │      └─ Dictionary check at each combination → return if found
  ├─ 4. Prefix removal on each suffix-stripped candidate
  │      └─ Anuswara (m-/n-/ng-/ny-/nge-) + Standard (di-/dipun-/pan-/…)
  └─ 5. Backtracking (Pengembalian Akhir) with known suffix combinations
```

> **Note on Infixes (Seselan):** Javanese infixes `-um-`, `-in-`, `-el-`, `-er-` are intentionally **not implemented** in v0.4.0 as they require character-level mid-word insertion detection that conflicts with the zero-regex philosophy. Planned for v0.5.0 with an Aho-Corasick approach.

---

## 🔬 MorphAnalyzer — Dictionary-Free Morphological Analyzer

`MorphAnalyzer` is a **zero-dependency, no-dictionary** morphological analyzer for Indonesian. It detects affix patterns and returns candidate roots **without validating** them against any dictionary.

> **Honest caveat:** Because there is no dictionary, `MorphAnalyzer` cannot resolve all morphophonemic mutations (e.g. `men-` + `tulis` = `menulis`, but stripping `men-` yields `ulis` not `tulis` without knowing the root starts with `t`). It is accurate for **affix detection** and **candidate generation**, but not as a standalone stemmer.

### When to use `MorphAnalyzer` vs `Stemmer`

| Need | Use |
|---|---|
| Single validated root from a word | `Stemmer` (requires dictionary) |
| Does this word have any affix? | `MorphAnalyzer` |
| What prefix/suffix does this word have? | `MorphAnalyzer` |
| Generate candidate roots for autocomplete or admin UI | `MorphAnalyzer` |
| Validate that a game submission is morphologically plausible | `MorphAnalyzer` |
| ML feature: prefix/suffix signals | `MorphAnalyzer` |

### Usage

```rust,ignore
use sastrawi::{MorphAnalyzer, MorphAnalysis};

let ma = MorphAnalyzer::new(); // Zero allocation — no dictionary loaded

// --- Affix detection ---
let r = ma.analyze("membangunkan");
assert!(r.has_affix);
assert_eq!(r.prefix.as_deref(), Some("me"));
assert_eq!(r.suffix.as_deref(), Some("kan"));
// candidate_roots may contain ["mbangun"] — final resolution needs Stemmer+dictionary

// --- Plain word (no affix) ---
let r = ma.analyze("buku");
assert!(!r.has_affix);
assert!(r.candidate_roots.is_empty());

// --- Confix (ke-an, per-an, ber-an) ---
let r = ma.analyze("keamanan");
assert_eq!(r.prefix.as_deref(), Some("ke"));
assert_eq!(r.suffix.as_deref(), Some("an"));
assert!(r.candidate_roots.contains(&"aman".to_string()));

// --- nge- informal prefix ---
let r = ma.analyze("ngecat");
assert_eq!(r.prefix.as_deref(), Some("nge"));
assert!(r.candidate_roots.contains(&"cat".to_string()));

// --- Possessive suffix ---
let r = ma.analyze("rumahnya");
assert_eq!(r.suffix.as_deref(), Some("nya"));
assert!(r.candidate_roots.contains(&"rumah".to_string()));

// --- Hyphen-clitic stripped before analysis ---
let r = ma.analyze("kuasa-Mu");
assert_eq!(r.word, "kuasa"); // clitic segment after hyphen is dropped
```

### `MorphAnalysis` struct

```rust,ignore
pub struct MorphAnalysis {
    pub word: String,                  // normalized (lowercased, hyphen-stripped) input
    pub prefix: Option<String>,        // detected prefix ("me", "ber", "nge", "ke", …)
    pub suffix: Option<String>,        // detected suffix ("kan", "an", "lah", "ku", …)
    pub candidate_roots: Vec<String>,  // plausible roots (may be ambiguous — see caveats)
    pub has_affix: bool,               // true if prefix OR suffix detected
}
```

### Detection pipeline (priority order)

```text
Input
  │
  ├─ 0. Lowercase + hyphen-clitic strip
  ├─ 1. Guard: word < 4 chars → return as-is (no analysis)
  ├─ 2. CONFIX (ke-an, per-an, ber-an, me-kan)  ← highest precision, both sides locked
  ├─ 3. PREFIX (me-/ber-/ter-/pe-/nge-/di-/se-/ke-/ku-/kau-)
  ├─ 4. PARTICLE (-lah, -kah, -tah, -pun), then prefix on remainder
  ├─ 5. POSSESSIVE (-ku, -mu, -nya), then prefix on remainder
  └─ 6. DERIVATIONAL SUFFIX (-kan, -an, -isme, -isasi, -isir)
         Guard: skipped if result < 3 chars, or -i suffix on short words
```

### Known limitations (by design)

| Limitation | Reason | Workaround |
|---|---|---|
| `menulis` → root `ulis`, not `tulis` | `men-` drops nasal; `t` restoration needs dict | Use `Stemmer` for final root |
| `mengirim` → root `irim`, not `kirim` | `meng-` drops `k` which needs dict to restore | Use `Stemmer` for final root |
| Ambiguous: `mengada` → `["ada", "ngada"]` | Both morphologically valid without dict | Check candidates against dict |
| `-is` / `-i` suffix not stripped aggressively | Avoids false positives on short words | Intentional guard |

### MorphAnalyzer API

```rust,ignore
let ma = MorphAnalyzer::new();    // or MorphAnalyzer::default()
let r: MorphAnalysis = ma.analyze(word);  // works on any &str
```

---

## 🏗 Architecture

```text
sastrawi-rs/
├── src/
│   ├── lib.rs                # Public API re-exports (both stemmers)
│   ├── stemmer.rs            # Indonesian: Nazief-Adriani pipeline + backtracking
│   ├── affixation.rs         # Indonesian: Prefix/suffix/confix orchestration
│   ├── affix_rules.rs        # Indonesian: Zero-regex morphological rules
│   ├── dictionary.rs         # Indonesian: FST-based dictionary (OnceLock)
│   ├── tokenizer.rs          # Shared zero-copy &str tokenizer
│   ├── stopword.rs           # Indonesian: FST stopword filter
│   └── javanese/
│       ├── mod.rs            # Javanese module re-exports
│       ├── stemmer.rs        # Javanese: Exhaustive suffix×prefix pipeline
│       ├── affixation.rs     # Javanese: Anuswara + Standard prefix orchestration
│       ├── affix_rules.rs    # Javanese: Meluluhkan/Menempel morphological rules
│       └── dictionary.rs     # Javanese: FST-based dictionary (OnceLock)
├── data/
│   ├── words.txt             # ~26k Indonesian root words (Kateglo, CC-BY-NC-SA 3.0)
│   ├── stopwords.txt         # Common Indonesian stopwords
│   └── javanese_words.txt    # ~1.3k Javanese root words (Riza et al. 2018, CC-BY 4.0)
├── build.rs                  # Compiles word lists → FST at build time
├── tests/test.rs             # Indonesian integration tests (6 suites, 200+ cases)
└── tests/javanese_test.rs    # Javanese integration tests (6 suites, 60+ cases)
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
# 12 test suites total (6 Indonesian + 6 Javanese), 260+ word cases
```

### Indonesian Suites

| Suite | Coverage |
|---|---|
| `test_stem_word` | 160+ Nazief-Adriani morphological cases |
| `test_stem_sentence` | Full sentence pipeline |
| `test_nge_informal_prefix` | `ngecat`, `ngegas`, `ngelepas` |
| `test_ecs_confixes` | `keamanan`, `pertanian`, `berhadapan` |
| `test_loanword_suffixes` | `-isasi`, `-isir`, `-isme`, `-is` |
| `test_stopword_filter` | `stem_sentence_filtered`, `is_stopword` |

### Javanese Suites

| Suite | Coverage |
|---|---|
| `test_javanese_anuswara_nasalization` | Meluluhkan & Menempel for `m-`, `n-`, `ng-`, `ny-` |
| `test_javanese_tripurusa_and_general_prefixes` | `di-`, `dipun-`, `dak-`, `ko-`, `ka-`, `kuma-`, etc. |
| `test_javanese_suffixes` | `-ake`, `-aken`, `-an`, `-ni`, `-ipun`, `-ne`, `-no`, etc. |
| `test_javanese_confixes_and_backtracking` | `dipunlampahaken`, `kepanasan`, `dituruake` |
| `test_javanese_complex_sandhi_and_circumfixes` | Vowel allomorphs (`nglarani`, `nggunakake`, `nglampahi`) |
| `test_javanese_new_affix_rules` | `pan-/pam-/pang-`, `nge-`, `-ane`, `kapi-`, `tar-/tok-`, `we-` |

---

## 🆕 Indonesian Extensions (2020–2026 Research)

Based on recent Indonesian NLP research (ECS [¹], IndoMorph [²], Aksara v1.5 [³]):

### A. `nge-` Informal Prefix
Colloquial prefix, mirror of `menge-`. Common in Jakarta informal speech and social media (e.g. MPStemmer [⁴]).

```text
ngecat    → cat
ngegas    → gas
ngelamar  → lamar
ngelepas  → lepas
```

### B. Confixes — ECS (Enhanced Confix Stripping)
Simultaneous prefix+suffix removal, proven to outperform plain Nazief-Adriani in accuracy [¹].

```text
keamanan    → aman    (ke-…-an)
pertanian   → tani    (per-…-an)
berhadapan  → hadap   (ber-…-an)
```

### C. Superlative `se-nya` Particle
```text
selengkapnya  → lengkap
seberhasilnya → hasil
```

### D. Loanword Suffixes
```text
idealisasi → ideal   (-isasi)
legalisir  → legal   (-isir)
idealisme  → ideal   (-isme)
idealis    → ideal   (-is)
```

---

## 📚 References & Credits

### Indonesian Stemmer
- [¹] **ECS**: Arifin, A., Mahendra, P., & Ciptaningtyas, H. T. (2009). _Enhanced Confix Stripping Stemmer and Ants Algorithm for Classifying News Document in Indonesian Language_.
- [²] **IndoMorph**: Kamajaya, I., & Moeljadi, D. (2025). _IndoMorph: a Morphology Engine for Indonesian_. [ACL Anthology](https://aclanthology.org/2025.sealp-1.7/).
- [³] **Aksara**: Universitas Indonesia (2023). _Aksara v1.5: Indonesian NLP tool conforming to UD v2 guidelines_. [GitHub](https://github.com/ir-nlp-csui/aksara).
- [⁴] **MPStemmer**: Prabono, A. G. (2020). _Mpstemmer: a multi-phase stemmer for standard and nonstandard Indonesian words_. [GitHub](https://github.com/ariaghora/mpstemmer).
- **Algorithm**: Nazief & Adriani (1996, 2007) — _"Confix Stripping: Approach to Stemming Algorithm for Bahasa Indonesia"_
- **PHP Sastrawi**: [Andy Librian](https://github.com/andylibrian) — original PHP implementation
- **rust-sastrawi**: [iDevoid](https://github.com/iDevoid/rust-sastrawi) — original Rust port (2019)

### Javanese Stemmer
- [⁵] **Javanese Nazief-Adriani**: _Stemming Javanese: Another Adaptation of the Nazief-Adriani affix rules_ (2020) — ISRITI. [Neliti](https://www.neliti.com/publications/330815/).
- [⁶] **Javanese ECS**: _Ngoko Javanese Stemmer using Enhanced Confix Stripping_ — ResearchGate. [ResearchGate](https://www.researchgate.net/publication/Stemming_Javanese).
- [⁷] **Complete Javanese Affix Taxonomy**: Semantic Scholar (2021–2023) — _A complete list of Javanese prefix/suffix rules including pan-/pam-/pang- nominal derivation, literary prefixes (kapi-, we-, a-), and dialectal forms (tar-, tok-)_. [Semantic Scholar](https://api.semanticscholar.org/graph/v1/paper/search?query=javanese+stemming+affix+rules).
- **JV Dictionary**: Riza, Hammam Riza et al. (2018) — _Indonesian Javanese Dictionary Starter Kit_ [Mendeley Data](https://data.mendeley.com/datasets/y3hstv4bfn) (CC-BY 4.0).

### General
- **sastrawi-rs**: [ibahasa Team](https://github.com/ibahasa/sastrawi-rs) — this modernized fork (2026)
- **ID Dictionary**: [Kateglo](http://kateglo.com/) by Ivan Lanin (CC-BY-NC-SA 3.0)

---

## 📄 License

MIT — see [LICENSE](LICENSE).
Dictionary data: Kateglo (CC-BY-NC-SA 3.0) — non-commercial use only for the bundled Indonesian word list.
Javanese dictionary: Riza et al. 2018 (CC-BY 4.0).
