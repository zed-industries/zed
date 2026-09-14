//! Eligibility only for independent, horizontal Canvas fallback of a complete
//! extended grapheme. Callers must still check whether bundled shaping supplies
//! the requested glyph and presentation; eligibility alone does not override it.
//!
//! This is intentionally not a Unicode sequence validator or an RGI emoji database.
//! Han variation selectors are checked structurally, not against the IVD registry.
//! Emoji selectors are checked against the Emoji property, not the registered
//! variation-sequence list. Eligibility does not guarantee browser coverage.
//! Flags accept two regional indicators, not only assigned country codes. ZWJ
//! support is limited to person/profession pairs and a small explicit allowlist;
//! tags are limited to the three subdivision flags. Other sequences stay on Cosmic.

use unicode_properties::{EmojiStatus, GeneralCategory, UnicodeEmoji, UnicodeGeneralCategory};
use unicode_script::{Script, UnicodeScript};
use unicode_segmentation::UnicodeSegmentation;

/// Controls browser-font fallback when loaded fonts lack a glyph or its emoji presentation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CanvasFontFallback {
    /// Use only fonts loaded into GPUI.
    Disabled,
    /// Use Canvas only for eligible graphemes requesting emoji presentation.
    #[default]
    Emoji,
    /// Also allow approximate independent rendering of eligible horizontal CJK text.
    EmojiAndCjk,
}

impl CanvasFontFallback {
    #[cfg(any(target_family = "wasm", test))]
    pub(crate) fn allows(self, emoji_presentation: bool) -> bool {
        match self {
            Self::Disabled => false,
            Self::Emoji => emoji_presentation,
            Self::EmojiAndCjk => true,
        }
    }
}

/// A supported grapheme assumed to be independently renderable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanvasFallback {
    /// Whether to request emoji presentation instead of ordinary text presentation.
    pub emoji_presentation: bool,
}

/// Returns a presentation only when `grapheme` is exactly one supported extended
/// grapheme cluster. No normalization, splitting, or font-coverage decision occurs.
pub fn classify_canvas_fallback(grapheme: &str) -> Option<CanvasFallback> {
    if grapheme.is_ascii() {
        return None;
    }
    let mut graphemes = grapheme.graphemes(true);
    if graphemes.next()? != grapheme || graphemes.next().is_some() {
        return None;
    }

    if is_cjk(grapheme) {
        Some(CanvasFallback {
            emoji_presentation: false,
        })
    } else {
        classify_emoji(grapheme).map(|emoji_presentation| CanvasFallback { emoji_presentation })
    }
}

fn is_cjk(grapheme: &str) -> bool {
    let mut characters = grapheme.chars();
    let Some(base) = characters.next() else {
        return false;
    };
    let suffix = characters.as_str();

    // The script/category checks exclude unassigned holes; the ranges exclude
    // Han radicals, iteration marks, and other non-ideographic Han characters.
    if base.script() == Script::Han
        && base.general_category() == GeneralCategory::OtherLetter
        && matches!(base, '\u{3400}'..='\u{9fff}' | '\u{f900}'..='\u{faff}'
            | '\u{20000}'..='\u{323af}')
    {
        return suffix.is_empty()
            || matches!(
                (characters.next(), characters.next()),
                (Some('\u{fe00}'..='\u{fe02}' | '\u{e0100}'..='\u{e01ef}'), None)
            );
    }

    if matches!(base, '\u{3041}'..='\u{3096}' | '\u{30a1}'..='\u{30fa}') {
        return suffix.is_empty()
            || match suffix {
                "\u{3099}" => "うかきくけこさしすせそたちつてとはひふへほウカキクケコサシスセソタチツテトハヒフヘホワヰヱヲ".contains(base),
                "\u{309a}" => "はひふへほハヒフヘホ".contains(base),
                _ => false,
            };
    }

    if matches!(base, '\u{ac00}'..='\u{d7a3}') {
        return suffix.is_empty()
            || ((base as u32 - 0xac00).is_multiple_of(28)
                && is_single_modern_trailing_jamo(suffix));
    }
    if matches!(base, '\u{1100}'..='\u{1112}') {
        return matches!(characters.next(), Some('\u{1161}'..='\u{1175}'))
            && (characters.as_str().is_empty()
                || is_single_modern_trailing_jamo(characters.as_str()));
    }

    suffix.is_empty()
        && (matches!(base, '\u{3131}'..='\u{3163}')
            || "、。〈〉《》「」『』【】〔〕（）［］｛｝，．！？：；・ー々〆".contains(base))
}

fn is_single_modern_trailing_jamo(text: &str) -> bool {
    let mut characters = text.chars();
    matches!(characters.next(), Some('\u{11a8}'..='\u{11c2}')) && characters.next().is_none()
}

fn classify_emoji(grapheme: &str) -> Option<bool> {
    let mut characters = grapheme.chars();
    let base = characters.next()?;
    let suffix = characters.as_str();

    if matches!(base, '0'..='9' | '#' | '*') {
        return match suffix {
            "\u{20e3}" | "\u{fe0f}\u{20e3}" => Some(true),
            _ => None,
        };
    }
    if unicode_properties::emoji::is_regional_indicator(base) {
        return (characters
            .next()
            .is_some_and(unicode_properties::emoji::is_regional_indicator)
            && characters.next().is_none())
        .then_some(true);
    }
    if matches!(
        grapheme,
        "🏴\u{e0067}\u{e0062}\u{e0065}\u{e006e}\u{e0067}\u{e007f}"
            | "🏴\u{e0067}\u{e0062}\u{e0073}\u{e0063}\u{e0074}\u{e007f}"
            | "🏴\u{e0067}\u{e0062}\u{e0077}\u{e006c}\u{e0073}\u{e007f}"
    ) {
        return Some(true);
    }
    if grapheme.contains('\u{200d}') {
        return is_supported_zwj_sequence(grapheme).then_some(true);
    }
    classify_emoji_unit(grapheme)
}

fn classify_emoji_unit(text: &str) -> Option<bool> {
    let mut characters = text.chars();
    let base = characters.next()?;
    if !base.is_emoji_char() || base.is_emoji_component() {
        return None;
    }
    let status = base.emoji_status();
    let mut emoji_presentation = matches!(
        status,
        EmojiStatus::EmojiPresentation | EmojiStatus::EmojiPresentationAndModifierBase
    );
    let mut next = characters.next();
    let explicit_text = next == Some('\u{fe0e}');
    if matches!(next, Some('\u{fe0e}' | '\u{fe0f}')) {
        emoji_presentation = !explicit_text;
        next = characters.next();
    }
    if let Some(modifier) = next {
        if explicit_text
            || !matches!(
                status,
                EmojiStatus::EmojiModifierBase | EmojiStatus::EmojiPresentationAndModifierBase
            )
            || modifier.emoji_status() != EmojiStatus::EmojiPresentationAndModifierAndEmojiComponent
        {
            return None;
        }
        emoji_presentation = true;
    }
    characters.next().is_none().then_some(emoji_presentation)
}

fn is_supported_zwj_sequence(grapheme: &str) -> bool {
    if matches!(
        grapheme,
        "👨‍👩‍👧" | "👨‍👩‍👧‍👦"
            | "👩‍👩‍👧‍👦"
            | "👨‍👨‍👧‍👦"
            | "🏳️‍🌈"
            | "🏳️‍⚧️"
            | "🏴‍☠️"
            | "❤️‍🔥"
            | "❤️‍🩹"
            | "👁️‍🗨️"
            | "🐻‍❄️"
    ) {
        return true;
    }
    let Some((person, profession)) = grapheme.split_once('\u{200d}') else {
        return false;
    };
    matches!(person.chars().next(), Some('👨' | '👩' | '🧑'))
        && classify_emoji_unit(person) == Some(true)
        && matches!(
            profession,
            "⚕️" | "⚖️"
                | "✈️"
                | "🌾"
                | "🍳"
                | "🎓"
                | "🎤"
                | "🎨"
                | "🏫"
                | "🏭"
                | "💻"
                | "💼"
                | "🔧"
                | "🔬"
                | "🚀"
                | "🚒"
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_font_fallback_policy() {
        assert_eq!(CanvasFontFallback::default(), CanvasFontFallback::Emoji);
        for grapheme in ["😀", "❤️", "1️⃣", "👨‍👩‍👧‍👦", "中", "か\u{3099}", "각", "©", "❤︎"]
        {
            let fallback = classify_canvas_fallback(grapheme).expect("eligible grapheme");
            assert!(!CanvasFontFallback::Disabled.allows(fallback.emoji_presentation));
            assert_eq!(
                CanvasFontFallback::Emoji.allows(fallback.emoji_presentation),
                ["😀", "❤️", "1️⃣", "👨‍👩‍👧‍👦"].contains(&grapheme),
            );
            assert!(CanvasFontFallback::EmojiAndCjk.allows(fallback.emoji_presentation));
        }
    }

    #[test]
    fn ascii_is_ineligible_but_keycaps_are_preserved() {
        for byte in 0..=0x7f_u8 {
            assert_eq!(classify_canvas_fallback(&(byte as char).to_string()), None);
        }
        for text in ["", "Hello", "0123456789#*", "\r\n"] {
            assert_eq!(classify_canvas_fallback(text), None);
        }
        for base in "0123456789#*".chars() {
            for text in [format!("{base}\u{20e3}"), format!("{base}\u{fe0f}\u{20e3}")] {
                assert_eq!(
                    classify_canvas_fallback(&text),
                    Some(CanvasFallback {
                        emoji_presentation: true,
                    }),
                    "{text:?}"
                );
            }
        }
    }

    #[test]
    fn ordinary_cjk_clusters() {
        for text in [
            "漢",
            "𠀀",
            "﨑",
            "漢\u{fe00}",
            "葛\u{e0100}",
            "葛\u{e01ef}",
            "あ",
            "ガ",
            "か\u{3099}",
            "ハ\u{309a}",
            "가",
            "각",
            "가",
            "각",
            "각",
            "ㄱ",
            "、",
            "。",
            "「",
            "」",
            "（",
            "！",
            "ー",
            "々",
        ] {
            assert_eq!(
                classify_canvas_fallback(text),
                Some(CanvasFallback {
                    emoji_presentation: false,
                }),
                "{text:?}"
            );
        }
    }

    #[test]
    fn emoji_presentation_is_preserved() {
        for (text, emoji_presentation) in [
            ("😀", true),
            ("©", false),
            ("©\u{fe0e}", false),
            ("©\u{fe0f}", true),
            ("❤", false),
            ("❤\u{fe0e}", false),
            ("❤\u{fe0f}", true),
            ("😀\u{fe0e}", false),
            ("👍🏽", true),
            ("☝🏽", true),
            ("👍\u{fe0f}🏽", true),
            ("🇯🇵", true),
            ("1\u{20e3}", true),
            ("#\u{fe0f}\u{20e3}", true),
            ("*\u{fe0f}\u{20e3}", true),
            ("👩🏽‍💻", true),
            ("🧑‍⚕️", true),
            ("👨‍👩‍👧‍👦", true),
            ("🏳️‍🌈", true),
            (
                "🏴\u{e0067}\u{e0062}\u{e0065}\u{e006e}\u{e0067}\u{e007f}",
                true,
            ),
            (
                "🏴\u{e0067}\u{e0062}\u{e0073}\u{e0063}\u{e0074}\u{e007f}",
                true,
            ),
            (
                "🏴\u{e0067}\u{e0062}\u{e0077}\u{e006c}\u{e0073}\u{e007f}",
                true,
            ),
        ] {
            assert_eq!(
                classify_canvas_fallback(text),
                Some(CanvasFallback { emoji_presentation }),
                "{text:?}"
            );
        }
    }

    #[test]
    fn unsupported_scripts_and_cjk_forms_stay_on_cosmic() {
        for text in [
            "a",
            "é",
            "α",
            "Ж",
            "ش",
            "ש",
            "क",
            "क्ष",
            "ก",
            "ក",
            "ᠠ",
            "ཀ",
            "\0",
            "\n",
            "\r\n",
            "\u{202e}",
            "\u{200d}",
            "\u{fdd0}",
            "\u{10ffff}",
            "\u{2a6e0}",
            "\u{e000}",
            "\u{3099}",
            "\u{fe0f}",
            "\u{e0100}",
            "漢\u{301}",
            "漢\u{fe03}",
            "漢\u{fe0f}",
            "漢\u{e0100}\u{e0101}",
            "あ\u{3099}",
            "か\u{309a}",
            "か\u{3099}\u{3099}",
            "ᄀ",
            "ᅡ",
            "ᆨ",
            "ᄓᅡ",
            "ᄀᅶ",
            "가ᇃ",
            "각ᆨ",
            "ᄀ가",
            "\u{3164}",
            "\u{3165}",
            "⺀",
            "⼀",
            "㇀",
            "㆐",
            "ㄅ",
            "㐀\u{200d}",
            "\u{1b000}",
            "ｶ",
            "！\u{301}",
        ] {
            assert_eq!(classify_canvas_fallback(text), None, "{text:?}");
        }
    }

    #[test]
    fn malformed_or_out_of_policy_emoji_stay_on_cosmic() {
        for text in [
            "0",
            "#",
            "*",
            "1\u{fe0f}",
            "#\u{fe0e}",
            "1\u{fe0e}\u{20e3}",
            "a\u{20e3}",
            "🏽",
            "🇯",
            "😀🏽",
            "👍🏽🏽",
            "👍\u{fe0e}🏽",
            "👍🏽\u{fe0f}",
            "❤\u{fe0f}\u{fe0f}",
            "😀\u{301}",
            "🦰",
            "😀‍😀",
            "👩‍",
            "👩‍💻‍🚀",
            "👩\u{fe0e}‍💻",
            "🏴\u{e0067}",
            "🏴\u{e0061}\u{e0062}\u{e007f}",
            "😀\u{e0067}\u{e007f}",
        ] {
            assert_eq!(classify_canvas_fallback(text), None, "{text:?}");
        }
    }

    #[test]
    fn accepts_only_one_whole_extended_grapheme() {
        for text in [
            "",
            "漢字",
            "あい",
            "가나",
            "😀😀",
            "🇯🇵🇺",
            "🇯🇵🇺🇸",
            " 漢",
            "漢\n",
        ] {
            assert_eq!(classify_canvas_fallback(text), None, "{text:?}");
        }
        let text = "aか\u{3099}👩🏽‍💻漢\u{e0100}";
        let eligible: Vec<_> = text
            .grapheme_indices(true)
            .filter_map(|(start, grapheme)| {
                classify_canvas_fallback(grapheme).map(|_| start..start + grapheme.len())
            })
            .collect();
        assert_eq!(
            eligible
                .iter()
                .map(|range| &text[range.clone()])
                .collect::<Vec<_>>(),
            ["か\u{3099}", "👩🏽‍💻", "漢\u{e0100}"]
        );
    }
}
