//! Character Tables
use std::cmp::Ordering;
use std::str::Chars;
use unicode_bidi::{bidi_class, BidiClass};
use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};

use super::rfc3454;

/// A.1 Unassigned code points in Unicode 3.2
pub fn unassigned_code_point(c: char) -> bool {
    rfc3454::A_1
        .binary_search_by(|&(start, end)| {
            if start > c {
                Ordering::Greater
            } else if end < c {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .is_ok()
}

/// B.1 Commonly mapped to nothing
pub fn commonly_mapped_to_nothing(c: char) -> bool {
    matches!(
        c,
        '\u{00AD}'
            | '\u{034F}'
            | '\u{1806}'
            | '\u{180B}'
            | '\u{180C}'
            | '\u{180D}'
            | '\u{200B}'
            | '\u{200C}'
            | '\u{200D}'
            | '\u{2060}'
            | '\u{FE00}'
            | '\u{FE01}'
            | '\u{FE02}'
            | '\u{FE03}'
            | '\u{FE04}'
            | '\u{FE05}'
            | '\u{FE06}'
            | '\u{FE07}'
            | '\u{FE08}'
            | '\u{FE09}'
            | '\u{FE0A}'
            | '\u{FE0B}'
            | '\u{FE0C}'
            | '\u{FE0D}'
            | '\u{FE0E}'
            | '\u{FE0F}'
            | '\u{FEFF}'
    )
}

/// B.2 Mapping for case-folding used with NFKC.
pub fn case_fold_for_nfkc(c: char) -> CaseFoldForNfkc {
    let inner = match rfc3454::B_2.binary_search_by_key(&c, |e| e.0) {
        Ok(idx) => FoldInner::Chars(rfc3454::B_2[idx].1.chars()),
        Err(_) => FoldInner::Char(Some(c)),
    };
    CaseFoldForNfkc(inner)
}

enum FoldInner {
    Chars(Chars<'static>),
    Char(Option<char>),
}

/// The iterator returned by `case_fold_for_nfkc`.
pub struct CaseFoldForNfkc(FoldInner);

impl Iterator for CaseFoldForNfkc {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.0 {
            FoldInner::Chars(ref mut it) => it.next(),
            FoldInner::Char(ref mut ch) => ch.take(),
        }
    }
}

/// C.1.1 ASCII space characters
pub fn ascii_space_character(c: char) -> bool {
    c == ' '
}

/// C.1.2 Non-ASCII space characters
pub fn non_ascii_space_character(c: char) -> bool {
    matches!(
        c,
        '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'
            | '\u{2001}'
            | '\u{2002}'
            | '\u{2003}'
            | '\u{2004}'
            | '\u{2005}'
            | '\u{2006}'
            | '\u{2007}'
            | '\u{2008}'
            | '\u{2009}'
            | '\u{200A}'
            | '\u{200B}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
    )
}

/// C.2.1 ASCII control characters
pub fn ascii_control_character(c: char) -> bool {
    matches!(c, '\u{0000}'..='\u{001F}' | '\u{007F}')
}

/// C.2.2 Non-ASCII control characters
pub fn non_ascii_control_character(c: char) -> bool {
    matches!(c, '\u{0080}'..='\u{009F}'
        | '\u{06DD}'
        | '\u{070F}'
        | '\u{180E}'
        | '\u{200C}'
        | '\u{200D}'
        | '\u{2028}'
        | '\u{2029}'
        | '\u{2060}'
        | '\u{2061}'
        | '\u{2062}'
        | '\u{2063}'
        | '\u{206A}'..='\u{206F}'
        | '\u{FEFF}'
        | '\u{FFF9}'..='\u{FFFC}'
        | '\u{1D173}'..='\u{1D17A}')
}

/// C.3 Private use
pub fn private_use(c: char) -> bool {
    matches!(c, '\u{E000}'..='\u{F8FF}' | '\u{F0000}'..='\u{FFFFD}' | '\u{100000}'..='\u{10FFFD}')
}

/// C.4 Non-character code points
pub fn non_character_code_point(c: char) -> bool {
    matches!(c, '\u{FDD0}'..='\u{FDEF}'
        | '\u{FFFE}'..='\u{FFFF}'
        | '\u{1FFFE}'..='\u{1FFFF}'
        | '\u{2FFFE}'..='\u{2FFFF}'
        | '\u{3FFFE}'..='\u{3FFFF}'
        | '\u{4FFFE}'..='\u{4FFFF}'
        | '\u{5FFFE}'..='\u{5FFFF}'
        | '\u{6FFFE}'..='\u{6FFFF}'
        | '\u{7FFFE}'..='\u{7FFFF}'
        | '\u{8FFFE}'..='\u{8FFFF}'
        | '\u{9FFFE}'..='\u{9FFFF}'
        | '\u{AFFFE}'..='\u{AFFFF}'
        | '\u{BFFFE}'..='\u{BFFFF}'
        | '\u{CFFFE}'..='\u{CFFFF}'
        | '\u{DFFFE}'..='\u{DFFFF}'
        | '\u{EFFFE}'..='\u{EFFFF}'
        | '\u{FFFFE}'..='\u{FFFFF}'
        | '\u{10FFFE}'..='\u{10FFFF}')
}

/// C.5 Surrogate codes
#[allow(clippy::match_single_binding)]
pub fn surrogate_code(c: char) -> bool {
    match c {
        // forbidden by rust
        /*'\u{D800}'..='\u{DFFF}' => true,*/
        _ => false,
    }
}

/// C.6 Inappropriate for plain text
pub fn inappropriate_for_plain_text(c: char) -> bool {
    matches!(
        c,
        '\u{FFF9}' | '\u{FFFA}' | '\u{FFFB}' | '\u{FFFC}' | '\u{FFFD}'
    )
}

/// C.7 Inappropriate for canonical representation
pub fn inappropriate_for_canonical_representation(c: char) -> bool {
    matches!(c, '\u{2FF0}'..='\u{2FFB}')
}

/// C.8 Change display properties or are deprecated
pub fn change_display_properties_or_deprecated(c: char) -> bool {
    matches!(
        c,
        '\u{0340}'
            | '\u{0341}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{202A}'
            | '\u{202B}'
            | '\u{202C}'
            | '\u{202D}'
            | '\u{202E}'
            | '\u{206A}'
            | '\u{206B}'
            | '\u{206C}'
            | '\u{206D}'
            | '\u{206E}'
            | '\u{206F}'
    )
}

/// C.9 Tagging characters
pub fn tagging_character(c: char) -> bool {
    matches!(c, '\u{E0001}' | '\u{E0020}'..='\u{E007F}')
}

/// D.1 Characters with bidirectional property "R" or "AL"
pub fn bidi_r_or_al(c: char) -> bool {
    matches!(bidi_class(c), BidiClass::R | BidiClass::AL)
}

/// D.2 Characters with bidirectional property "L"
pub fn bidi_l(c: char) -> bool {
    matches!(bidi_class(c), BidiClass::L)
}

/// Determines if `c` is to be removed according to section 7.2 of
/// [ITU-T Recommendation X.520 (2019)](https://www.itu.int/rec/T-REC-X.520-201910-I/en).
pub fn x520_mapped_to_nothing(c: char) -> bool {
    match c {
        '\u{00AD}'
        | '\u{1806}'
        | '\u{034F}'
        | '\u{180B}'..='\u{180D}'
        | '\u{FE00}'..='\u{FE0F}'
        | '\u{FFFC}'
        | '\u{200B}' => true,
        // Technically control characters, but mapped to whitespace in X.520.
        '\u{09}' | '\u{0A}'..='\u{0D}' | '\u{85}' => false,
        _ => c.is_control(),
    }
}

/// Determines if `c` is to be replaced by SPACE (0x20) according to section 7.2 of
/// [ITU-T Recommendation X.520 (2019)](https://www.itu.int/rec/T-REC-X.520-201910-I/en).
pub fn x520_mapped_to_space(c: char) -> bool {
    match c {
        '\u{09}' | '\u{0A}'..='\u{0D}' | '\u{85}' => true,
        _ => c.general_category_group() == GeneralCategoryGroup::Separator,
    }
}
