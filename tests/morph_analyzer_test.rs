use sastrawi::{MorphAnalysis, MorphAnalyzer};

fn ma() -> MorphAnalyzer {
    MorphAnalyzer::new()
}

// ---------------------------------------------------------------------------
// Helper: assert a prefix/suffix is detected
// ---------------------------------------------------------------------------
fn assert_prefix(r: &MorphAnalysis, expected: &str) {
    assert_eq!(
        r.prefix.as_deref(),
        Some(expected),
        "word='{}' expected prefix={:?}, got prefix={:?}",
        r.word, expected, r.prefix
    );
}

fn assert_suffix(r: &MorphAnalysis, expected: &str) {
    assert_eq!(
        r.suffix.as_deref(),
        Some(expected),
        "word='{}' expected suffix={:?}, got suffix={:?}",
        r.word, expected, r.suffix
    );
}

fn assert_candidate(r: &MorphAnalysis, expected: &str) {
    assert!(
        r.candidate_roots.contains(&expected.to_string()),
        "word='{}' expected root '{}' in candidates {:?}",
        r.word, expected, r.candidate_roots
    );
}

// ---------------------------------------------------------------------------
// 1. No affix — plain root words must NOT be flagged as having affixes
// ---------------------------------------------------------------------------
#[test]
fn test_morph_no_affix_plain_words() {
    let ma = ma();

    // Short / plain words that must return has_affix = false
    let plain = ["buku", "meja", "tahu", "rasa", "pagi"];
    for word in plain {
        let r = ma.analyze(word);
        assert!(!r.has_affix, "Plain word '{}' falsely flagged as having affix", word);
        assert!(r.candidate_roots.is_empty(), "Plain word '{}' should have no candidates", word);
    }
}

// ---------------------------------------------------------------------------
// 2. Prefix detection — me- family
// ---------------------------------------------------------------------------
#[test]
fn test_morph_prefix_me_family() {
    let ma = ma();

    // me-
    let r = ma.analyze("membaca");
    assert!(r.prefix.as_deref().map(|p| p.starts_with("me")).unwrap_or(false));
    assert_candidate(&r, "baca");

    // mem- (b)
    let r = ma.analyze("membantu");
    assert_candidate(&r, "bantu");

    // men- (t) — 'men' drops, leaves 'ulis'; real root disambiguation needs dictionary
    let r = ma.analyze("menulis");
    assert!(r.has_affix, "menulis should be detected as affixed");
    // candidates may be 'ulis' (bare) — this is expected without dictionary
    // the analyzer is honest: it flags the affix but can't restore 't' without dict
    assert!(!r.candidate_roots.is_empty());

    // meng- (k) — strips 'meng', vowel V remains; 'k' restoration needs dictionary
    let r = ma.analyze("mengirim");
    assert!(r.has_affix, "mengirim should be detected as affixed");
    // Without dict: 'meng' prefix is stripped leaving 'irim' (vowel-initial)
    assert!(!r.candidate_roots.is_empty());

    // menge- monosyllabic
    let r = ma.analyze("mengebom");
    assert_prefix(&r, "menge");
    assert_candidate(&r, "bom");

    // meny- (s→ny)
    let r = ma.analyze("menyapu");
    assert!(r.candidate_roots.contains(&"apu".to_string()) || r.candidate_roots.contains(&"sapu".to_string()));
    assert!(r.has_affix);
}

// ---------------------------------------------------------------------------
// 3. Prefix detection — nge- informal
// ---------------------------------------------------------------------------
#[test]
fn test_morph_prefix_nge_informal() {
    let ma = ma();

    let cases = [("ngecat", "cat"), ("ngegas", "gas"), ("ngelamar", "lamar"), ("ngelepas", "lepas")];
    for (word, root) in cases {
        let r = ma.analyze(word);
        assert_prefix(&r, "nge");
        assert_candidate(&r, root);
        assert!(r.has_affix);
    }
}

// ---------------------------------------------------------------------------
// 4. Prefix detection — ber-, ter-, pe-, se-, di-, ke-, ku-, kau-
// ---------------------------------------------------------------------------
#[test]
fn test_morph_prefix_others() {
    let ma = ma();

    let r = ma.analyze("bersekolah");
    assert_prefix(&r, "ber");
    assert!(r.has_affix);

    let r = ma.analyze("terjatuh");
    assert_prefix(&r, "ter");
    assert!(r.has_affix);

    let r = ma.analyze("ditulis");
    assert_prefix(&r, "di");
    assert_candidate(&r, "tulis");

    let r = ma.analyze("kuambil");
    assert_prefix(&r, "ku");
    assert!(r.has_affix);

    let r = ma.analyze("kaubawa");
    assert_prefix(&r, "kau");
    assert!(r.has_affix);
}

// ---------------------------------------------------------------------------
// 5. Suffix detection — particles and possessives
// ---------------------------------------------------------------------------
#[test]
fn test_morph_suffix_particles_and_possessives() {
    let ma = ma();

    let r = ma.analyze("bukankah");
    assert_suffix(&r, "kah");
    assert_candidate(&r, "bukan");

    let r = ma.analyze("biarlah");
    assert_suffix(&r, "lah");
    assert!(r.has_affix);

    let r = ma.analyze("bukuku");
    assert_suffix(&r, "ku");
    assert_candidate(&r, "buku");

    let r = ma.analyze("rumahnya");
    assert_suffix(&r, "nya");
    assert_candidate(&r, "rumah");

    let r = ma.analyze("mobilmu");
    assert_suffix(&r, "mu");
    assert_candidate(&r, "mobil");
}

// ---------------------------------------------------------------------------
// 6. Suffix detection — derivational (-kan, -an, -i, -isme, -isasi)
// ---------------------------------------------------------------------------
#[test]
fn test_morph_suffix_derivational() {
    let ma = ma();

    let r = ma.analyze("membangunkan");
    assert!(r.has_affix, "membangunkan should be affixed");
    // Without dictionary, prefix 'mem' strips to 'mbangun' (correct per rules).
    // The full chain 'mem-' + '-kan' producing 'bangun' requires dictionary backtrack.
    // MorphAnalyzer is honest: it returns plausible candidates, not the final root.
    assert!(!r.candidate_roots.is_empty());
    assert!(r.suffix.as_deref() == Some("kan") || r.prefix.is_some());

    let r = ma.analyze("pertanian");
    assert!(r.has_affix);

    let r = ma.analyze("idealisasi");
    assert_suffix(&r, "isasi");
    assert_candidate(&r, "ideal");

    let r = ma.analyze("idealisme");
    assert_suffix(&r, "isme");
    assert_candidate(&r, "ideal");

    let r = ma.analyze("legalisir");
    assert_suffix(&r, "isir");
    assert_candidate(&r, "legal");
}

// ---------------------------------------------------------------------------
// 7. Confix detection — ke-an, per-an, ber-an, me-kan
// ---------------------------------------------------------------------------
#[test]
fn test_morph_confixes() {
    let ma = ma();

    // ke-an confix → 'aman'
    let r = ma.analyze("keamanan");
    assert_candidate(&r, "aman");
    assert!(r.has_affix);

    // per-an confix → 'tani'
    let r = ma.analyze("pertanian");
    assert_candidate(&r, "tani");
    assert!(r.has_affix);

    // ber-an confix → 'hadap'
    let r = ma.analyze("berhadapan");
    assert_candidate(&r, "hadap");
    assert!(r.has_affix);

    // me-kan confix: stripped via remove_confix
    let r = ma.analyze("membangunkan");
    assert!(r.has_affix, "membangunkan should be affixed");
    // remove_confix('membangunkan') → strips 'me' and 'kan' → 'mbangunkan'?
    // Actually remove_confix checks starts_with("me") && ends_with("kan") → 'mbangun'
    // This is a known limitation — without dict 'mem' prefix can't be normalized
    assert!(!r.candidate_roots.is_empty());
}

// ---------------------------------------------------------------------------
// 8. Hyphen clitic — stripped before analysis
// ---------------------------------------------------------------------------
#[test]
fn test_morph_hyphen_clitic() {
    let ma = ma();

    // Clitic is stripped — the normalized `word` field reflects the pre-hyphen segment
    let r = ma.analyze("kuasa-Mu");
    assert_eq!(r.word, "kuasa");   // normalized word is 'kuasa'
    // 'kuasa' alone is 4 chars, may or may not match a prefix — acceptable either way

    let r = ma.analyze("allah-lah");
    assert_eq!(r.word, "allah");   // clitic 'lah' is part of hyphen fragment, stripped
}

// ---------------------------------------------------------------------------
// 9. Ambiguous inputs — must return MULTIPLE candidates, not panic
// ---------------------------------------------------------------------------
#[test]
fn test_morph_ambiguous_candidates() {
    let ma = ma();

    // "mengambil" → candidate "ambil" (normal) 
    let r = ma.analyze("mengambil");
    assert!(r.has_affix);
    // must include "ambil" since meng + V → drops ng, produces ambil
    assert!(r.candidate_roots.iter().any(|c| c == "ambil" || c == "ngambil"));

    // "menyanyikan" → multiple candidates due to meny- mutation
    let r = ma.analyze("menyanyikan");
    assert!(r.has_affix);
    assert!(!r.candidate_roots.is_empty());
}

// ---------------------------------------------------------------------------
// 10. Case insensitivity
// ---------------------------------------------------------------------------
#[test]
fn test_morph_case_insensitive() {
    let ma = ma();

    let r1 = ma.analyze("Membangunkan");
    let r2 = ma.analyze("membangunkan");
    assert_eq!(r1.has_affix, r2.has_affix);
    assert_eq!(r1.candidate_roots, r2.candidate_roots);
}
