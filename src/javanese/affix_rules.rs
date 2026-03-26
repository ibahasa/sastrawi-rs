
/// Removes Javanese Possessives/Pronouns suffixes (-ku, -mu, -ne, -ipun)
pub fn remove_possessive(word: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let suffixes = ["ipun", "ne", "ku", "mu"];
    for s in suffixes {
        if word.ends_with(s) && word.len() > s.len() {
            let root = &word[..word.len() - s.len()];
            results.push((s.to_string(), root.to_string()));
        }
    }
    results
}

/// Removes Javanese Particles and Allomorphs
/// (-a, -i, -e, -en, -an, -ake, -na, -no, -ana, -aken, -ni, -nan, -kake, -kaken, -ane)
pub fn remove_particle(word: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let suffixes = [
        "kaken", "kake", "aken", "ake", "ana", "ane", "nan", "an", "en", "na", "no", "ni", "a",
        "i", "e",
    ];
    for s in suffixes {
        if word.ends_with(s) && word.len() > s.len() {
            let root = &word[..word.len() - s.len()];
            results.push((s.to_string(), root.to_string()));
        }
    }
    results
}

/// Removes Ater-ater Tripurusa and general standard prefixes.
/// Covers Ngoko, Krama, literary (kapi-, we-, a-), dialectal (tar-, tok-, ben-).
pub fn remove_standard_prefixes(word: &str) -> Vec<String> {
    let mut results = Vec::new();
    let prefixes = [
        // Krama/Formal
        "dipun", "kuma", "kapi",
        // Nominal derivation (pan- allomorphs: pam- before labial, pang- before velar)
        "pang", "pam", "pan",
        // General
        "pra", "dak", "tak", "kok", "ko", "di",
        "pa", "pi", "ka", "sa", "ma", "ke",
        // Literary / archaic
        "we", "ben",
        // Dialectal (Jawa Timur)
        "tar", "tok",
        // Archaic vowel-initial prefix
        "a",
    ];

    for p in prefixes {
        if word.starts_with(p) && word.len() > p.len() + 1 {
            let root = &word[p.len()..];
            results.push(root.to_string());
        }
    }
    results
}

/// Removes Ater-ater Anuswara (nasalization).
/// Accurately handles Javanese 'Meluluhkan' (melting/replacing) and 'Menempel' (attaching).
pub fn remove_anuswara_prefixes(word: &str) -> Vec<String> {
    let mut results = Vec::new();

    if word.starts_with("nge") && word.len() > 3 {
        // Special 'nge-' prefix for monosyllabic roots: ngecet -> cet, ngecat -> cat
        let rest = &word[3..];
        results.push(rest.to_string());
    }

    if word.starts_with("ng") && word.len() > 2 {
        let rest = &word[2..];
        results.push(rest.to_string()); // ngombe -> ombe (vowel)
        results.push(format!("k{}", rest)); // ngirim -> kirim (meluluhkan k)
        // Menempel: nggawa -> ng + gawa, the rest is "gawa", so rest.to_string() correctly covers 'gawa'.
    }

    if word.starts_with("ny") && word.len() > 2 {
        let rest = &word[2..];
        results.push(format!("s{}", rest)); // nyapu -> sapu (meluluhkan s)
        results.push(format!("c{}", rest)); // nyekel -> cekel (meluluhkan c)
    }

    // Menempel 'm-' pada 'b' (mbalang -> balang)
    if word.starts_with("mb") && word.len() > 2 {
        let rest = &word[1..];
        results.push(rest.to_string());
    } else if word.starts_with('m') && word.len() > 1 {
        let rest = &word[1..];
        results.push(format!("p{}", rest)); // mangan -> pangan (meluluhkan p)
        results.push(format!("w{}", rest)); // maca -> waca (meluluhkan w)
        results.push(rest.to_string()); // munggah -> unggah (vowel)
    }

    // Menempel 'n-' pada 'd', 'dh', 'j', 'th' (ndawuh -> dawuh, njupuk -> jupuk)
    if (word.starts_with("nd") || word.starts_with("nj") || word.starts_with("nth"))
        && word.len() > 2
    {
        let rest = &word[1..];
        results.push(rest.to_string());
    } else if word.starts_with('n')
        && word.len() > 1
        && !word.starts_with("ny")
        && !word.starts_with("ng")
    {
        let rest = &word[1..];
        results.push(format!("t{}", rest)); // nulis -> tulis (meluluhkan t)
        results.push(format!("th{}", rest)); // nuthuk -> thuthuk (meluluhkan th)
        results.push(rest.to_string()); // vowel
    }

    results
}
