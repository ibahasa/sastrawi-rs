extern crate sastrawi;

use sastrawi::*;

#[test]
fn test_stem_word() {
    let _test_items = &[
        ("mei", "mei"),
        ("bui", "bui"),
        ("nilai", "nilai"),
        ("hancurlah", "hancur"),
        ("benarkah", "benar"),
        ("apatah", "apa"),
        ("siapapun", "siapa"),
        ("jubahku", "jubah"),
        ("bajumu", "baju"),
        ("celananya", "celana"),
        ("hantui", "hantu"),
        ("belikan", "beli"),
        ("jualan", "jual"),
        ("bukumukah", "buku"),
        ("miliknyalah", "milik"),
        ("kulitkupun", "kulit"),
        ("berikanku", "beri"),
        ("sakitimu", "sakit"),
        ("beriannya", "beri"),
        ("kasihilah", "kasih"),
        ("dibuang", "buang"),
        ("kesakitan", "sakit"),
        ("sesuap", "suap"),
        ("beradu", "adu"),
        ("berambut", "rambut"),
        ("bersuara", "suara"),
        ("berdaerah", "daerah"),
        ("belajar", "ajar"),
        ("bekerja", "kerja"),
        ("beternak", "ternak"),
        ("terasing", "asing"),
        ("teraup", "raup"),
        ("tergerak", "gerak"),
        ("terpuruk", "puruk"),
        ("teterbang", "terbang"),
        ("melipat", "lipat"),
        ("meringkas", "ringkas"),
        ("mewarnai", "warna"),
        ("meyakinkan", "yakin"),
        ("membangun", "bangun"),
        ("memfitnah", "fitnah"),
        ("memvonis", "vonis"),
        ("memperbaru", "baru"),
        ("mempelajar", "ajar"),
        ("meminum", "minum"),
        ("memukul", "pukul"),
        ("mencinta", "cinta"),
        ("mendua", "dua"),
        ("menjauh", "jauh"),
        ("menziarah", "ziarah"),
        ("menuklir", "nuklir"),
        ("menangkap", "tangkap"),
        ("menggila", "gila"),
        ("menghajar", "hajar"),
        ("mengqasar", "qasar"),
        ("mengudara", "udara"),
        ("mengupas", "kupas"),
        ("menyuarakan", "suara"),
        ("mempopulerkan", "populer"),
        ("pewarna", "warna"),
        ("peyoga", "yoga"),
        ("peradilan", "adil"),
        ("perumahan", "rumah"),
        ("permuka", "muka"),
        ("perdaerah", "daerah"),
        ("pembangun", "bangun"),
        ("pemfitnah", "fitnah"),
        ("pemvonis", "vonis"),
        ("peminum", "minum"),
        ("pemukul", "pukul"),
        ("pencinta", "cinta"),
        ("pendua", "dua"),
        ("penjauh", "jauh"),
        ("penziarah", "ziarah"),
        ("penuklir", "nuklir"),
        ("penangkap", "tangkap"),
        ("penggila", "gila"),
        ("penghajar", "hajar"),
        ("pengqasar", "qasar"),
        ("pengudara", "udara"),
        ("pengupas", "kupas"),
        ("penyuara", "suara"),
        ("pelajar", "ajar"),
        ("pelabuh", "labuh"),
        ("petarung", "tarung"),
        ("terpercaya", "percaya"),
        ("pekerja", "kerja"),
        ("peserta", "serta"),
        ("mempengaruhi", "pengaruh"),
        ("mengkritik", "kritik"),
        ("bersekolah", "sekolah"),
        ("bertahan", "tahan"),
        ("mencapai", "capai"),
        ("dimulai", "mulai"),
        ("petani", "tani"),
        ("terabai", "abai"),
        ("mensyaratkan", "syarat"),
        ("mensyukuri", "syukur"),
        ("mengebom", "bom"),
        ("mempromosikan", "promosi"),
        ("memproteksi", "proteksi"),
        ("memprediksi", "prediksi"),
        ("pengkajian", "kaji"),
        ("pengebom", "bom"),
        ("bersembunyi", "sembunyi"),
        ("bersembunyilah", "sembunyi"),
        ("pelanggan", "langgan"),
        ("pelaku", "laku"),
        ("pelangganmukah", "langgan"),
        ("pelakunyalah", "laku"),
        ("perbaikan", "baik"),
        ("kebaikannya", "baik"),
        ("bisikan", "bisik"),
        ("menerangi", "terang"),
        ("berimanlah", "iman"),
        ("memuaskan", "puas"),
        ("berpelanggan", "langgan"),
        ("bermakanan", "makan"),
        ("menyala", "nyala"),
        ("menyanyikan", "nyanyi"),
        ("menyatakannya", "nyata"),
        ("penyanyi", "nyanyi"),
        ("penyawaan", "nyawa"),
        ("rerata", "rata"),
        ("lelembut", "lembut"),
        ("lemigas", "ligas"),
        ("kinerja", "kerja"),
        ("bertebaran", "tebar"),
        ("terasingkan", "asing"),
        ("membangunkan", "bangun"),
        ("mencintai", "cinta"),
        ("menduakan", "dua"),
        ("menjauhi", "jauh"),
        ("menggilai", "gila"),
        ("pembangunan", "bangun"),
        ("marwan", "marwan"),
        ("subarkah", "subarkah"),
        ("memberdayakan", "daya"),
        ("persemakmuran", "makmur"),
        ("keberuntunganmu", "untung"),
        ("kesepersepuluhnya", "sepuluh"),
        ("Perekonomian", "ekonomi"),
        ("menahan", "tahan"),
        ("peranan", "peran"),
        ("memberikan", "beri"),
        ("medannya", "medan"),
        ("idealis", "ideal"),
        ("idealisme", "ideal"),
        ("finalisasi", "final"),
        ("mentaati", "taat"),
        ("melewati", "lewat"),
        ("menganga", "nganga"),
        ("kupukul", "pukul"),
        ("kauhajar", "hajar"),
        ("kuasa-Mu", "kuasa"),
        ("nikmat-Ku", "nikmat"),
        ("allah-lah", "allah"),
    ];

    let words = &[
        "hancur", "benar", "apa", "siapa", "jubah", "baju", "beli", "celana", "hantu", "jual",
        "buku", "milik", "kulit", "sakit", "kasih", "buang", "suap", "nilai", "beri", "rambut",
        "adu", "suara", "daerah", "ajar", "kerja", "ternak", "asing", "raup", "gerak", "puruk",
        "terbang", "lipat", "ringkas", "warna", "yakin", "bangun", "fitnah", "vonis", "baru",
        "ajar", "tangkap", "kupas", "minum", "pukul", "cinta", "dua", "jauh", "ziarah", "nuklir",
        "gila", "hajar", "qasar", "udara", "populer", "warna", "yoga", "adil", "rumah", "muka",
        "labuh", "tarung", "tebar", "indah", "daya", "untung", "sepuluh", "ekonomi", "makmur",
        "telah", "serta", "percaya", "pengaruh", "kritik", "seko", "sekolah", "tahan", "capa",
        "capai", "mula", "mulai", "petan", "tani", "aba", "abai", "balas", "balik", "peran",
        "medan", "syukur", "syarat", "bom", "promosi", "proteksi", "prediksi", "kaji", "sembunyi",
        "langgan", "laku", "baik", "terang", "iman", "bisik", "taat", "puas", "makan", "nyala",
        "nyanyi", "nyata", "nyawa", "rata", "lembut", "ligas", "budaya", "karya", "ideal", "final",
        "taat", "tiru", "sepak", "kuasa", "malaikat", "nikmat", "lewat", "nganga", "allah",
    ];

    let dict = Dictionary::custom(words);
    let stemmer = Stemmer::new(&dict);
    for (value, expected) in _test_items.iter() {
        let actual = stemmer.stem_word(value);
        assert_eq!(
            actual.as_ref(),
            *expected,
            "Stemming failed for word: {}",
            value
        );
    }
}

#[test]
fn test_stem_sentence() {
    let expected = [
        "ekonomi",
        "indonesia",
        "sedang",
        "dalam",
        "tumbuh",
        "yang",
        "bangga",
    ];
    let dict = Dictionary::new();
    let stemmer = Stemmer::new(&dict);
    let sentence = "Perekonomian Indonesia sedang dalam pertumbuhan yang membanggakan";
    let stemmed_iterator = stemmer.stem_sentence(sentence);
    for (index, actual) in stemmed_iterator.enumerate() {
        assert_eq!(actual.as_ref(), expected[index]);
    }
}

// ---------------------------------------------------------------------------
// Test: nge- Informal Prefix (2020-2026 research — MPStemmer, Aksara v1.2+)
// Colloquial/lisan form common in social media, Jakarta speech, ibahasa slang corpus
// ---------------------------------------------------------------------------
#[test]
fn test_nge_informal_prefix() {
    let words = &[
        "cat", "gas", "bom", "lap", "pel", "lamar", "rasa", "lepas", "charge",
    ];
    let dict = Dictionary::custom(words);
    let stemmer = Stemmer::new(&dict);

    let cases = &[
        ("ngecat", "cat"),
        ("ngegas", "gas"),
        ("ngebom", "bom"),
        ("ngelap", "lap"),
        ("ngepel", "pel"),
        ("ngelamar", "lamar"),
        ("ngelepas", "lepas"),
    ];

    for (value, expected) in cases.iter() {
        let actual = stemmer.stem_word(value);
        assert_eq!(actual.as_ref(), *expected, "nge- failed for: {}", value);
    }
}

// ---------------------------------------------------------------------------
// Test: ECS Confixes — ke-an, per-an, ber-an simultaneous strip
// Based on Enhanced Confix Stripping (outperforms plain Nazief-Adriani per 2025 research)
// ---------------------------------------------------------------------------
#[test]
fn test_ecs_confixes() {
    let words = &[
        "aman",
        "tani",
        "hadap",
        "sakit",
        "tumbuh",
        "indah",
        "cantik",
        "sejahtera",
        "maju",
        "daerah",
    ];
    let dict = Dictionary::custom(words);
    let stemmer = Stemmer::new(&dict);

    let cases = &[
        // ke-an confix
        ("keamanan", "aman"),
        ("kesakitan", "sakit"),
        ("keindahan", "indah"),
        ("kecantikan", "cantik"),
        // per-an confix
        ("pertanian", "tani"),
        ("kemajuan", "maju"),
        ("kesejahteraan", "sejahtera"),
        // ber-an confix
        ("berhadapan", "hadap"),
    ];

    for (value, expected) in cases.iter() {
        let actual = stemmer.stem_word(value);
        assert_eq!(
            actual.as_ref(),
            *expected,
            "ECS confix failed for: {}",
            value
        );
    }
}

// ---------------------------------------------------------------------------
// Test: Loanword Suffixes (-isasi, -isir, -isme, -is)
// Common in modern formal Indonesian from 2000-onwards, especially technical/academic text
// ---------------------------------------------------------------------------
#[test]
fn test_loanword_suffixes() {
    let words = &[
        "ideal", "final", "legal", "normal", "formal", "digital", "kapital", "modern", "liberal",
        "sosial",
    ];
    let dict = Dictionary::custom(words);
    let stemmer = Stemmer::new(&dict);

    let cases = &[
        // -isasi
        ("idealisasi", "ideal"),
        ("finalisasi", "final"),
        ("digitalisasi", "digital"),
        ("normalisasi", "normal"),
        ("modernisasi", "modern"),
        // -isir
        ("legalisir", "legal"),
        ("formalisir", "formal"),
        // -isme
        ("idealisme", "ideal"),
        ("liberalisme", "liberal"),
        ("kapitalisme", "kapital"),
        // -is
        ("idealis", "ideal"),
        ("sosialis", "sosial"),
        ("formalis", "formal"),
    ];

    for (value, expected) in cases.iter() {
        let actual = stemmer.stem_word(value);
        assert_eq!(
            actual.as_ref(),
            *expected,
            "loanword suffix failed for: {}",
            value
        );
    }
}

// ---------------------------------------------------------------------------
// Test: Stopword Filter — stem_sentence_filtered & is_stopword
// Common function words (yang, di, dari, pada, dalam, …) should be skipped
// when building a search index or running NLP analysis.
// ---------------------------------------------------------------------------
#[test]
fn test_stopword_filter() {
    let dict = Dictionary::new();
    let stemmer = Stemmer::new(&dict);

    // is_stopword: spot-check common Indonesian stopwords
    assert!(
        stemmer.is_stopword("yang"),
        "expected 'yang' to be a stopword"
    );
    assert!(stemmer.is_stopword("di"), "expected 'di' to be a stopword");
    assert!(
        stemmer.is_stopword("dari"),
        "expected 'dari' to be a stopword"
    );
    assert!(
        stemmer.is_stopword("dalam"),
        "expected 'dalam' to be a stopword"
    );
    assert!(
        stemmer.is_stopword("dengan"),
        "expected 'dengan' to be a stopword"
    );
    assert!(
        !stemmer.is_stopword("ekonomi"),
        "'ekonomi' should NOT be a stopword"
    );
    assert!(
        !stemmer.is_stopword("tumbuh"),
        "'tumbuh' should NOT be a stopword"
    );

    // stem_sentence_filtered: stopwords excluded, content words stemmed
    let sentence = "Perekonomian Indonesia sedang dalam pertumbuhan yang membanggakan";
    let filtered: Vec<String> = stemmer
        .stem_sentence_filtered(sentence)
        .map(|w| w.into_owned())
        .collect();

    // "sedang", "dalam", "yang" must not appear
    assert!(
        !filtered.contains(&"sedang".to_string()),
        "stopword 'sedang' leaked through"
    );
    assert!(
        !filtered.contains(&"dalam".to_string()),
        "stopword 'dalam' leaked through"
    );
    assert!(
        !filtered.contains(&"yang".to_string()),
        "stopword 'yang' leaked through"
    );

    // content words must be stemmed and present
    assert!(
        filtered.contains(&"ekonomi".to_string()),
        "expected 'ekonomi' in output"
    );
    assert!(
        filtered.contains(&"tumbuh".to_string()),
        "expected 'tumbuh' in output"
    );
    assert!(
        filtered.contains(&"bangga".to_string()),
        "expected 'bangga' in output"
    );
}
