use crate::affix_rules;

/// Detected morphological components of a word.
///
/// Returned by [`MorphAnalyzer::analyze`]. All fields are optional — a word may
/// have no affix at all, or only a prefix, or only a suffix.
///
/// # Limitations
/// Because `MorphAnalyzer` works **without a dictionary**, it cannot validate
/// whether the candidate root is a real word. Ambiguous inputs (e.g. "mengada")
/// may produce multiple plausible candidates. Use the dictionary-backed [`Stemmer`]
/// when you need a single, validated root.
///
/// [`Stemmer`]: crate::Stemmer
#[derive(Debug, Clone, PartialEq)]
pub struct MorphAnalysis {
    /// The original lowercased input word.
    pub word: String,

    /// Detected prefix (e.g. `"me"`, `"ber"`, `"nge"`, `"ke"`).
    /// `None` if no prefix pattern matched.
    pub prefix: Option<String>,

    /// Detected suffix or particle (e.g. `"kan"`, `"an"`, `"lah"`, `"ku"`).
    /// `None` if no suffix pattern matched.
    pub suffix: Option<String>,

    /// All candidate roots produced by stripping detected affix(es).
    /// May contain more than one entry due to ambiguity (e.g. Anuswara mutations).
    /// Empty only when the input is too short or no pattern matched at all.
    pub candidate_roots: Vec<String>,

    /// `true` if at least one prefix **or** suffix was detected.
    pub has_affix: bool,
}

/// Dictionary-free morphological analyzer for Bahasa Indonesia.
///
/// Unlike [`Stemmer`], `MorphAnalyzer` requires no dictionary and performs no
/// root validation. It simply detects affix patterns and returns all plausible
/// candidate roots. Ideal for:
///
/// - **Game word validation** — check whether a submitted word *looks*
///   morphologically valid (has a real prefix/suffix pattern) before hitting
///   the dictionary.
/// - **Feature extraction** — feed `prefix`/`suffix` signals into an ML pipeline.
/// - **Candidate generation** — expand a word into possible roots for
///   autocomplete or admin suggestion UIs.
/// - **Spell-check heuristics** — a word with a valid prefix pattern but no
///   dictionary root is a likely misspelling.
///
/// # Honest Caveats
///
/// - **Ambiguity is expected.** "mengada" → candidates `["ada", "ngada"]`.
///   The analyzer returns all of them; you decide which applies.
/// - **No validation.** Candidates are morphologically plausible, not
///   guaranteed to be real roots. Always verify against a dictionary
///   before showing to users.
///
/// # Example
///
/// ```rust,ignore
/// use sastrawi::MorphAnalyzer;
///
/// let ma = MorphAnalyzer::new();
///
/// let r = ma.analyze("membangunkan");
/// assert_eq!(r.prefix, Some("me".into()));
/// assert_eq!(r.suffix, Some("kan".into()));
/// assert!(r.candidate_roots.contains(&"bangun".to_string()));
/// assert!(r.has_affix);
///
/// let r2 = ma.analyze("buku");
/// assert_eq!(r2.has_affix, false);  // no affix detected
/// ```
///
/// [`Stemmer`]: crate::Stemmer
pub struct MorphAnalyzer;

impl MorphAnalyzer {
    /// Creates a new `MorphAnalyzer`. Zero allocation — no dictionary loaded.
    pub fn new() -> Self {
        MorphAnalyzer
    }

    /// Analyzes a single word and returns its detected morphological components.
    ///
    /// The input is lowercased and hyphen-clitic trimmed before analysis.
    pub fn analyze(&self, word: &str) -> MorphAnalysis {
        // 0. Normalize: lowercase + strip hyphen-clitics (kuasa-Mu → kuasa)
        let normalized = {
            let lower = word.to_lowercase();
            if let Some(idx) = lower.find('-') {
                lower[..idx].to_string()
            } else {
                lower
            }
        };

        // Short words — too risky to analyze
        if normalized.len() < 4 {
            return MorphAnalysis {
                word: normalized,
                prefix: None,
                suffix: None,
                candidate_roots: vec![],
                has_affix: false,
            };
        }

        let mut prefix: Option<String> = None;
        let mut suffix: Option<String> = None;
        let mut candidates: Vec<String> = Vec::new();

        // Priority 1: CONFIX (ke-an, per-an, ber-an, me-kan, pe-an, ter-kan)
        // These lock in both prefix and suffix simultaneously — highest precision.
        if let Some(root) = affix_rules::remove_confix(&normalized) {
            let (p, s) = infer_confix_label(&normalized);
            if !p.is_empty() {
                prefix = Some(p.to_string());
                suffix = Some(s.to_string());
                push_unique(&mut candidates, root);
            }
        }

        // Priority 2: PREFIX-only on full word (me-, ber-, ter-, pe-, nge-, di-, etc.)
        // Only if confix didn't already claim the prefix.
        if prefix.is_none() {
            self.detect_prefix(&normalized, &mut prefix, &mut candidates);
        }

        // Priority 3: Particle (-lah, -kah, -tah, -pun)
        // Only if no suffix claimed yet.
        if suffix.is_none() {
            let (p, after) = affix_rules::remove_particle(&normalized);
            if !p.is_empty() {
                suffix = Some(p.to_string());
                let r = after.to_string();
                push_unique(&mut candidates, r.clone());
                // Also try prefix on the particle-stripped remainder
                if prefix.is_none() {
                    self.detect_prefix(&r, &mut prefix, &mut candidates);
                }
            }
        }

        // Priority 4: Possessive (-ku, -mu, -nya)
        if suffix.is_none() {
            let (p, after) = affix_rules::remove_possessive(&normalized);
            if !p.is_empty() {
                suffix = Some(p.to_string());
                let r = after.to_string();
                push_unique(&mut candidates, r.clone());
                if prefix.is_none() {
                    self.detect_prefix(&r, &mut prefix, &mut candidates);
                }
            }
        }

        // Priority 5: Derivational suffix (-kan, -an, -isme, -isasi, -isir, -is)
        // Guard: only strip if:
        //   (a) no suffix was found yet
        //   (b) the remaining root is >= 3 chars
        //   (c) the suffix is NOT just -i alone (too aggressive on plain words like 'pagi')
        if suffix.is_none() {
            let (s, after) = affix_rules::remove_suffix(&normalized);
            let safe = !s.is_empty()
                && after.len() >= 3
                && s != "is"                         // -is is too short, handled by compound only
                && !(s == "i" && normalized.len() < 6); // -i only on longer words
            if safe {
                suffix = Some(s.to_string());
                let r = after.to_string();
                push_unique(&mut candidates, r.clone());
                if prefix.is_none() {
                    self.detect_prefix(&r, &mut prefix, &mut candidates);
                }
            }
        }

        // Deduplicate and filter trivial
        candidates.dedup();
        candidates.retain(|r| r != &normalized && r.len() >= 2);

        let has_affix = prefix.is_some() || suffix.is_some();

        MorphAnalysis {
            word: normalized,
            prefix,
            suffix,
            candidate_roots: candidates,
            has_affix,
        }
    }

    fn detect_prefix(
        &self,
        word: &str,
        detected_prefix: &mut Option<String>,
        candidates: &mut Vec<String>,
    ) {
        // me- family
        let (after_me, _) = affix_rules::remove_prefix_me(word);
        if after_me.as_ref() != word {
            *detected_prefix = Some(infer_me_prefix(word).to_string());
            push_unique(candidates, after_me.to_string());
            return;
        }
        // nge- informal
        let (after_nge, _) = affix_rules::remove_prefix_nge(word);
        if after_nge.as_ref() != word {
            *detected_prefix = Some("nge".to_string());
            push_unique(candidates, after_nge.to_string());
            return;
        }
        // pe- family
        let (after_pe, _) = affix_rules::remove_prefix_pe(word);
        if after_pe.as_ref() != word {
            *detected_prefix = Some(infer_pe_prefix(word).to_string());
            push_unique(candidates, after_pe.to_string());
            return;
        }
        // ber-
        let (after_be, _) = affix_rules::remove_prefix_be(word);
        if after_be.as_ref() != word {
            *detected_prefix = Some("ber".to_string());
            push_unique(candidates, after_be.to_string());
            return;
        }
        // ter-
        let (after_te, _) = affix_rules::remove_prefix_te(word);
        if after_te.as_ref() != word {
            *detected_prefix = Some("ter".to_string());
            push_unique(candidates, after_te.to_string());
            return;
        }
        // se-, di-, ke-, ku-, kau-
        for (pfx, label) in [("se", "se"), ("di", "di"), ("ke", "ke"), ("ku", "ku"), ("kau", "kau")] {
            if word.starts_with(pfx) && word.len() > pfx.len() + 1 {
                *detected_prefix = Some(label.to_string());
                push_unique(candidates, word[pfx.len()..].to_string());
                return;
            }
        }
    }
}

impl Default for MorphAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn push_unique(v: &mut Vec<String>, s: String) {
    if !v.contains(&s) {
        v.push(s);
    }
}

fn infer_me_prefix(word: &str) -> &'static str {
    if word.starts_with("menge") { "menge" }
    else if word.starts_with("meny") { "meny" }
    else if word.starts_with("meng") { "meng" }
    else if word.starts_with("men") { "men" }
    else if word.starts_with("mem") { "mem" }
    else { "me" }
}

fn infer_pe_prefix(word: &str) -> &'static str {
    if word.starts_with("penge") { "penge" }
    else if word.starts_with("peny") { "peny" }
    else if word.starts_with("peng") { "peng" }
    else if word.starts_with("pen") { "pen" }
    else if word.starts_with("pem") { "pem" }
    else if word.starts_with("per") { "per" }
    else { "pe" }
}

fn infer_confix_label(word: &str) -> (&'static str, &'static str) {
    if word.starts_with("ke") && word.ends_with("an") { return ("ke", "an"); }
    if word.starts_with("per") && word.ends_with("an") { return ("per", "an"); }
    if word.starts_with("ber") && word.ends_with("an") { return ("ber", "an"); }
    if word.starts_with("me") && word.ends_with("kan") { return ("me", "kan"); }
    if word.starts_with("pe") && word.ends_with("an") { return ("pe", "an"); }
    if word.starts_with("ter") && word.ends_with("kan") { return ("ter", "kan"); }
    ("", "")
}
