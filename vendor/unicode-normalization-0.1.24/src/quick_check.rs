use crate::lookups::canonical_combining_class;
use crate::stream_safe;
use crate::tables;
use crate::UnicodeNormalization;

/// QuickCheck quickly determines if a string is normalized, it can return
/// `Maybe`
///
/// The QuickCheck algorithm can quickly determine if a text is or isn't
/// normalized without any allocations in many cases, but it has to be able to
/// return `Maybe` when a full decomposition and recomposition is necessary.
#[derive(Debug, Eq, PartialEq)]
pub enum IsNormalized {
    /// The text is definitely normalized.
    Yes,
    /// The text is definitely not normalized.
    No,
    /// The text may be normalized.
    Maybe,
}

// https://unicode.org/reports/tr15/#Detecting_Normalization_Forms
#[inline]
fn quick_check<F, I>(s: I, is_allowed: F, stream_safe: bool) -> IsNormalized
where
    I: Iterator<Item = char>,
    F: Fn(char) -> IsNormalized,
{
    let mut last_cc = 0u8;
    let mut nonstarter_count = 0;
    let mut result = IsNormalized::Yes;
    for ch in s {
        // For ASCII we know it's always allowed and a starter
        if ch <= '\x7f' {
            last_cc = 0;
            nonstarter_count = 0;
            continue;
        }

        // Otherwise, lookup the combining class and QC property
        let cc = canonical_combining_class(ch);
        if last_cc > cc && cc != 0 {
            return IsNormalized::No;
        }
        match is_allowed(ch) {
            IsNormalized::Yes => (),
            IsNormalized::No => return IsNormalized::No,
            IsNormalized::Maybe => {
                result = IsNormalized::Maybe;
            }
        }
        if stream_safe {
            let decomp = stream_safe::classify_nonstarters(ch);

            // If we're above `MAX_NONSTARTERS`, we're definitely *not*
            // stream-safe normalized.
            if nonstarter_count + decomp.leading_nonstarters > stream_safe::MAX_NONSTARTERS {
                return IsNormalized::No;
            }
            if decomp.leading_nonstarters == decomp.decomposition_len {
                nonstarter_count += decomp.decomposition_len;
            } else {
                nonstarter_count = decomp.trailing_nonstarters;
            }
        }
        last_cc = cc;
    }
    result
}

/// Quickly check if a string is in NFC, potentially returning
/// `IsNormalized::Maybe` if further checks are necessary.  In this case a check
/// like `s.chars().nfc().eq(s.chars())` should suffice.
#[inline]
pub fn is_nfc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfc, false)
}

/// Quickly check if a string is in NFKC.
#[inline]
pub fn is_nfkc_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfkc, false)
}

/// Quickly check if a string is in NFD.
#[inline]
pub fn is_nfd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfd, false)
}

/// Quickly check if a string is in NFKD.
#[inline]
pub fn is_nfkd_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfkd, false)
}

/// Quickly check if a string is Stream-Safe NFC.
#[inline]
pub fn is_nfc_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfc, true)
}

/// Quickly check if a string is Stream-Safe NFD.
#[inline]
pub fn is_nfd_stream_safe_quick<I: Iterator<Item = char>>(s: I) -> IsNormalized {
    quick_check(s, tables::qc_nfd, true)
}

/// Authoritatively check if a string is in NFC.
#[inline]
pub fn is_nfc(s: &str) -> bool {
    match is_nfc_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().nfc()),
    }
}

/// Authoritatively check if a string is in NFKC.
#[inline]
pub fn is_nfkc(s: &str) -> bool {
    match is_nfkc_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().nfkc()),
    }
}

/// Authoritatively check if a string is in NFD.
#[inline]
pub fn is_nfd(s: &str) -> bool {
    match is_nfd_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().nfd()),
    }
}

/// Authoritatively check if a string is in NFKD.
#[inline]
pub fn is_nfkd(s: &str) -> bool {
    match is_nfkd_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().nfkd()),
    }
}

/// Authoritatively check if a string is Stream-Safe NFC.
#[inline]
pub fn is_nfc_stream_safe(s: &str) -> bool {
    match is_nfc_stream_safe_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().stream_safe().nfc()),
    }
}

/// Authoritatively check if a string is Stream-Safe NFD.
#[inline]
pub fn is_nfd_stream_safe(s: &str) -> bool {
    match is_nfd_stream_safe_quick(s.chars()) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => s.chars().eq(s.chars().stream_safe().nfd()),
    }
}

#[cfg(test)]
mod tests {
    use super::{is_nfc_stream_safe_quick, is_nfd_stream_safe_quick, IsNormalized};

    #[test]
    fn test_stream_safe_nfd() {
        let okay = "Da\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}ngerzone";
        assert_eq!(is_nfd_stream_safe_quick(okay.chars()), IsNormalized::Yes);

        let too_much = "Da\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{031e}\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}ngerzone";
        assert_eq!(is_nfd_stream_safe_quick(too_much.chars()), IsNormalized::No);
    }

    #[test]
    fn test_stream_safe_nfc() {
        let okay = "ok\u{e0}\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}y";
        assert_eq!(is_nfc_stream_safe_quick(okay.chars()), IsNormalized::Maybe);

        let too_much = "not ok\u{e0}\u{031b}\u{0316}\u{0317}\u{0318}\u{0319}\u{031c}\u{031d}\u{031e}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{031a}y";
        assert_eq!(is_nfc_stream_safe_quick(too_much.chars()), IsNormalized::No);
    }
}
