// Invisibility in a Unicode context is not well defined, so we have to guess.
//
// We highlight all ASCII control codes, and unicode whitespace because they are likely
// confused with an ASCII space in a programming context (U+0020).
//
// We also highlight the handful of blank non-space characters:
//   U+2800 BRAILLE PATTERN BLANK - Category: So
//   U+115F HANGUL CHOSEONG FILLER - Category: Lo
//   U+1160 HANGUL CHOSEONG FILLER - Category: Lo
//   U+3164 HANGUL FILLER - Category: Lo
//   U+FFA0 HALFWIDTH HANGUL FILLER - Category: Lo
//   U+FFFC OBJECT REPLACEMENT CHARACTER - Category: So
//
// For the rest of Unicode, invisibility happens for two reasons:
// * A Format character (like a byte order mark or right-to-left override)
// * An invisible Nonspacing Mark character (like U+034F, or variation selectors)
//
// We don't consider unassigned codepoints invisible as the font renderer already shows
// a replacement character in that case (and there are a *lot* of them)
//
// Control characters are mostly fine to highlight; except:
// * U+E0020..=U+E007F are used in emoji flags. We don't highlight them right now, but we could if we tightened our heuristics.
// * U+200D is used to join characters. We highlight this but don't replace it. As our font system ignores mid-glyph highlights this mostly works to highlight unexpected uses.
//
// Nonspacing marks are handled like U+200D. This means that mid-glyph we ignore them, but
// probably causes issues with end-of-glyph usage.
//
// ref: https://invisible-characters.com
// ref: https://www.compart.com/en/unicode/category/Cf
// ref: https://gist.github.com/ConradIrwin/f759e1fc29267143c4c7895aa495dca5?h=1
// ref: https://unicode.org/Public/emoji/13.0/emoji-test.txt
// https://github.com/bits/UTF-8-Unicode-Test-Documents/blob/master/UTF-8_sequence_separated/utf8_sequence_0-0x10ffff_assigned_including-unprintable-asis.txt
pub fn is_invisible(c: char) -> bool {
    if c <= '\u{1f}' {
        c != '\t' && c != '\n' && c != '\r'
    } else if c >= '\u{7f}' {
        c <= '\u{9f}'
            || (c.is_whitespace() && c != IDEOGRAPHIC_SPACE)
            || contains(c, FORMAT)
            || contains(c, OTHER)
    } else {
        false
    }
}
// ASCII control characters have fancy unicode glyphs, everything else
// is replaced by a space - unless it is used in combining characters in
// which case we need to leave it in the string.
pub fn replacement(c: char) -> Option<&'static str> {
    if c <= '\x1f' {
        Some(C0_SYMBOLS[c as usize])
    } else if c == '\x7f' {
        Some(DEL)
    } else if contains(c, PRESERVE) {
        None
    } else {
        Some("\u{2007}") // fixed width space
    }
}
// IDEOGRAPHIC SPACE is common alongside Chinese and other wide character sets.
// We don't highlight this for now (as it already shows up wide in the editor),
// but could if we tracked state in the classifier.
const IDEOGRAPHIC_SPACE: char = '\u{3000}';

const C0_SYMBOLS: &[&str] = &[
    "␀", "␁", "␂", "␃", "␄", "␅", "␆", "␇", "␈", "␉", "␊", "␋", "␌", "␍", "␎", "␏", "␐", "␑", "␒",
    "␓", "␔", "␕", "␖", "␗", "␘", "␙", "␚", "␛", "␜", "␝", "␞", "␟",
];
const DEL: &str = "␡";

// generated using ucd-generate: ucd-generate general-category --include Format --chars ucd-16.0.0
pub const FORMAT: &[(char, char)] = &[
    ('\u{ad}', '\u{ad}'),
    ('\u{600}', '\u{605}'),
    ('\u{61c}', '\u{61c}'),
    ('\u{6dd}', '\u{6dd}'),
    ('\u{70f}', '\u{70f}'),
    ('\u{890}', '\u{891}'),
    ('\u{8e2}', '\u{8e2}'),
    ('\u{180e}', '\u{180e}'),
    ('\u{200b}', '\u{200f}'),
    ('\u{202a}', '\u{202e}'),
    ('\u{2060}', '\u{2064}'),
    ('\u{2066}', '\u{206f}'),
    ('\u{feff}', '\u{feff}'),
    ('\u{fff9}', '\u{fffb}'),
    ('\u{110bd}', '\u{110bd}'),
    ('\u{110cd}', '\u{110cd}'),
    ('\u{13430}', '\u{1343f}'),
    ('\u{1bca0}', '\u{1bca3}'),
    ('\u{1d173}', '\u{1d17a}'),
    ('\u{e0001}', '\u{e0001}'),
    ('\u{e0020}', '\u{e007f}'),
];

// hand-made base on https://invisible-characters.com (Excluding Cf)
pub const OTHER: &[(char, char)] = &[
    ('\u{034f}', '\u{034f}'),
    ('\u{115F}', '\u{1160}'),
    ('\u{17b4}', '\u{17b5}'),
    ('\u{180b}', '\u{180d}'),
    ('\u{2800}', '\u{2800}'),
    ('\u{3164}', '\u{3164}'),
    ('\u{fe00}', '\u{fe0d}'),
    ('\u{ffa0}', '\u{ffa0}'),
    ('\u{fffc}', '\u{fffc}'),
    ('\u{e0100}', '\u{e01ef}'),
];

// a subset of FORMAT/OTHER that may appear within glyphs
const PRESERVE: &[(char, char)] = &[
    ('\u{034f}', '\u{034f}'),
    ('\u{200d}', '\u{200d}'),
    ('\u{17b4}', '\u{17b5}'),
    ('\u{180b}', '\u{180d}'),
    ('\u{e0061}', '\u{e007a}'),
    ('\u{e007f}', '\u{e007f}'),
];

fn contains(c: char, list: &[(char, char)]) -> bool {
    for (start, end) in list {
        if c < *start {
            return false;
        }
        if c <= *end {
            return true;
        }
    }
    false
}
