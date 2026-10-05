//! Detection of "surprising" Unicode characters in the domains and paths shown
//! in sandbox privilege-escalation prompts.
//!
//! Homoglyph/confusable attacks (a Cyrillic `а` standing in for a Latin `a`),
//! invisible characters (zero-width spaces), and bidirectional overrides can
//! make a requested domain or path look like something it is not, tricking the
//! user into granting access to the wrong target. Domains reach the prompt in
//! Punycode (`xn--…`) ASCII form, so a lookalike host is decoded back to
//! Unicode before scanning; paths are scanned as they are displayed.

use unicode_script::UnicodeScript as _;

/// Why a character in a domain or path is considered surprising.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuspiciousKind {
    /// A bidirectional control that can visually reorder surrounding text (for
    /// example U+202E RIGHT-TO-LEFT OVERRIDE) — the classic "Trojan Source"
    /// trick.
    BidiControl,
    /// A zero-width, invisible, or non-ASCII whitespace formatting character.
    Invisible,
    /// A visible non-ASCII character that can be confused with ASCII (a
    /// homoglyph) or that mixes an unexpected script into otherwise-ASCII text.
    Confusable,
}

/// A single surprising character discovered while scanning a domain or path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SuspiciousChar {
    pub character: char,
    pub kind: SuspiciousKind,
}

impl SuspiciousChar {
    /// A human-readable, one-line description for the approval banner, such as
    /// `‘а’ (U+0430 Cyrillic)` or `U+202E right-to-left override`.
    pub fn description(&self) -> String {
        let codepoint = format!("U+{:04X}", self.character as u32);
        match self.kind {
            SuspiciousKind::Confusable => {
                format!(
                    "‘{}’ ({codepoint} {})",
                    self.character,
                    self.character.script().full_name()
                )
            }
            // Bidi controls and invisible characters have no meaningful glyph to
            // show (and printing them could itself reorder the banner text), so
            // we render only the codepoint and a name.
            SuspiciousKind::BidiControl | SuspiciousKind::Invisible => {
                match well_known_name(self.character) {
                    Some(name) => format!("{codepoint} {name}"),
                    None => codepoint,
                }
            }
        }
    }
}

/// Scan a raw string for surprising Unicode characters, returning each distinct
/// offending character once, in order of first appearance.
pub fn scan(text: &str) -> Vec<SuspiciousChar> {
    let mut result: Vec<SuspiciousChar> = Vec::new();
    for character in text.chars() {
        if character.is_ascii() {
            continue;
        }
        if result.iter().any(|found| found.character == character) {
            continue;
        }
        result.push(SuspiciousChar {
            character,
            kind: classify(character),
        });
    }
    result
}

/// Scan a host for surprising characters, first decoding any IDN/Punycode
/// (`xn--…`) labels back to Unicode so a lookalike domain that reaches us as
/// ASCII is still caught. Returns the decoded (Unicode) host — which is what the
/// banner shows the user — alongside the findings. When nothing is surprising
/// the returned host equals the input.
pub fn scan_host(host: &str) -> (String, Vec<SuspiciousChar>) {
    // `domain_to_unicode` never fails destructively: on error it still returns a
    // best-effort decoding, which is exactly what we want to scan and show.
    let (decoded, _result) = idna::domain_to_unicode(host);
    let findings = scan(&decoded);
    (decoded, findings)
}

fn classify(character: char) -> SuspiciousKind {
    if is_bidi_control(character) {
        SuspiciousKind::BidiControl
    } else if is_invisible(character) {
        SuspiciousKind::Invisible
    } else {
        SuspiciousKind::Confusable
    }
}

fn is_bidi_control(character: char) -> bool {
    matches!(character,
        '\u{061C}' // ARABIC LETTER MARK
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK
        | '\u{202A}'..='\u{202E}' // LRE, RLE, PDF, LRO, RLO
        | '\u{2066}'..='\u{2069}' // LRI, RLI, FSI, PDI
    )
}

fn is_invisible(character: char) -> bool {
    matches!(character,
        '\u{00AD}' // SOFT HYPHEN
        | '\u{180E}' // MONGOLIAN VOWEL SEPARATOR
        | '\u{200B}' // ZERO WIDTH SPACE
        | '\u{200C}' // ZERO WIDTH NON-JOINER
        | '\u{200D}' // ZERO WIDTH JOINER
        | '\u{2060}' // WORD JOINER
        | '\u{2061}'..='\u{2064}' // invisible math operators
        | '\u{FEFF}' // ZERO WIDTH NO-BREAK SPACE (BOM)
    ) || is_non_ascii_space(character)
        // Any remaining control/format character (categories Cc/Cf) is
        // invisible for our purposes.
        || character.is_control()
}

fn is_non_ascii_space(character: char) -> bool {
    matches!(
        character,
        '\u{00A0}' // NO-BREAK SPACE
        | '\u{1680}' // OGHAM SPACE MARK
        | '\u{2000}'
            ..='\u{200A}' // EN QUAD … HAIR SPACE
        | '\u{202F}' // NARROW NO-BREAK SPACE
        | '\u{205F}' // MEDIUM MATHEMATICAL SPACE
        | '\u{3000}' // IDEOGRAPHIC SPACE
    )
}

/// Friendly names for the invisible/bidi characters most likely to show up in an
/// attack, so the banner reads better than a bare codepoint.
fn well_known_name(character: char) -> Option<&'static str> {
    Some(match character {
        '\u{00A0}' => "no-break space",
        '\u{00AD}' => "soft hyphen",
        '\u{061C}' => "arabic letter mark",
        '\u{180E}' => "mongolian vowel separator",
        '\u{200B}' => "zero-width space",
        '\u{200C}' => "zero-width non-joiner",
        '\u{200D}' => "zero-width joiner",
        '\u{200E}' => "left-to-right mark",
        '\u{200F}' => "right-to-left mark",
        '\u{202A}' => "left-to-right embedding",
        '\u{202B}' => "right-to-left embedding",
        '\u{202C}' => "pop directional formatting",
        '\u{202D}' => "left-to-right override",
        '\u{202E}' => "right-to-left override",
        '\u{2060}' => "word joiner",
        '\u{2066}' => "left-to-right isolate",
        '\u{2067}' => "right-to-left isolate",
        '\u{2068}' => "first strong isolate",
        '\u{2069}' => "pop directional isolate",
        '\u{3000}' => "ideographic space",
        '\u{FEFF}' => "zero-width no-break space",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_ascii_is_never_flagged() {
        assert!(scan("github.com").is_empty());
        assert!(scan("/home/user/project/src/main.rs").is_empty());
        assert!(scan("*.npmjs.org").is_empty());
    }

    #[test]
    fn detects_cyrillic_homoglyph() {
        // "gіthub.com" with a Cyrillic "і" (U+0456).
        let findings = scan("g\u{0456}thub.com");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].character, '\u{0456}');
        assert_eq!(findings[0].kind, SuspiciousKind::Confusable);
        assert!(findings[0].description().contains("U+0456"));
        assert!(findings[0].description().contains("Cyrillic"));
    }

    #[test]
    fn detects_bidi_override() {
        let findings = scan("safe\u{202E}txt.exe");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, SuspiciousKind::BidiControl);
        assert_eq!(findings[0].description(), "U+202E right-to-left override");
    }

    #[test]
    fn detects_zero_width_space() {
        let findings = scan("git\u{200B}hub.com");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, SuspiciousKind::Invisible);
        assert_eq!(findings[0].description(), "U+200B zero-width space");
    }

    #[test]
    fn deduplicates_repeated_characters() {
        // Two Cyrillic "а" (U+0430) should be reported once.
        let findings = scan("\u{0430}bc\u{0430}");
        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn scan_host_decodes_punycode_lookalike() {
        // "аpple.com" (leading Cyrillic а, U+0430) encodes to this Punycode.
        let (decoded, findings) = scan_host("xn--pple-43d.com");
        assert_eq!(decoded, "\u{0430}pple.com");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].character, '\u{0430}');
        assert_eq!(findings[0].kind, SuspiciousKind::Confusable);
    }

    #[test]
    fn scan_host_leaves_plain_domains_alone() {
        let (decoded, findings) = scan_host("github.com");
        assert_eq!(decoded, "github.com");
        assert!(findings.is_empty());
    }

    #[test]
    fn scan_host_handles_wildcard_subdomain_patterns() {
        // Host patterns can carry a leading `*.` wildcard; decoding must not
        // choke on it, and a lookalike label behind it is still caught.
        let (_decoded, findings) = scan_host("*.xn--pple-43d.com");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].character, '\u{0430}');

        let (decoded, findings) = scan_host("*.github.com");
        assert_eq!(decoded, "*.github.com");
        assert!(findings.is_empty());
    }
}
