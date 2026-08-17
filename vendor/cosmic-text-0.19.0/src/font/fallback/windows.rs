// SPDX-License-Identifier: MIT OR Apache-2.0

use unicode_script::Script;

use super::Fallback;

/// A platform-specific font fallback list, for Windows.
#[derive(Debug)]
pub struct PlatformFallback;

impl Fallback for PlatformFallback {
    fn common_fallback(&self) -> &'static [&'static str] {
        common_fallback()
    }

    fn forbidden_fallback(&self) -> &'static [&'static str] {
        forbidden_fallback()
    }

    fn script_fallback(
        &self,
        script: unicode_script::Script,
        locale: &str,
    ) -> &'static [&'static str] {
        script_fallback(script, locale)
    }
}

// Fallbacks to use after any script specific fallbacks
fn common_fallback() -> &'static [&'static str] {
    //TODO: abstract style (sans/serif/monospaced)
    &[
        "Segoe UI",
        "Segoe UI Emoji",
        "Segoe UI Symbol",
        "Segoe UI Historic",
        //TODO: Add CJK script here for doublewides?
    ]
}

// Fallbacks to never use
fn forbidden_fallback() -> &'static [&'static str] {
    &[]
}

fn han_unification(locale: &str) -> &'static [&'static str] {
    //TODO!
    match locale {
        // Japan
        "ja" => &["Yu Gothic"],
        // Korea
        "ko" => &["Malgun Gothic"],
        // Hong Kong"
        "zh-HK" => &["MingLiU_HKSCS"],
        // Taiwan
        "zh-TW" => &["Microsoft JhengHei UI"],
        // Simplified Chinese is the default (also catches "zh-CN" for China)
        _ => &["Microsoft YaHei UI"],
    }
}

// Fallbacks to use per script
fn script_fallback(script: Script, locale: &str) -> &'static [&'static str] {
    //TODO: better match https://github.com/chromium/chromium/blob/master/third_party/blink/renderer/platform/fonts/win/font_fallback_win.cc#L99
    match script {
        Script::Adlam => &["Ebrima"],
        Script::Bengali => &["Nirmala UI"],
        Script::Canadian_Aboriginal => &["Gadugi"],
        Script::Chakma => &["Nirmala UI"],
        Script::Cherokee => &["Gadugi"],
        Script::Devanagari => &["Nirmala UI"],
        Script::Ethiopic => &["Ebrima"],
        Script::Gujarati => &["Nirmala UI"],
        Script::Gurmukhi => &["Nirmala UI"],
        Script::Han => han_unification(locale),
        Script::Hangul => han_unification("ko"),
        Script::Hiragana => han_unification("ja"),
        Script::Javanese => &["Javanese Text"],
        Script::Kannada => &["Nirmala UI"],
        Script::Katakana => han_unification("ja"),
        Script::Khmer => &["Leelawadee UI"],
        Script::Lao => &["Leelawadee UI"],
        Script::Malayalam => &["Nirmala UI"],
        Script::Mongolian => &["Mongolian Baiti"],
        Script::Myanmar => &["Myanmar Text"],
        Script::Oriya => &["Nirmala UI"],
        Script::Sinhala => &["Nirmala UI"],
        Script::Tamil => &["Nirmala UI"],
        Script::Telugu => &["Nirmala UI"],
        Script::Thaana => &["MV Boli"],
        Script::Thai => &["Leelawadee UI"],
        Script::Tibetan => &["Microsoft Himalaya"],
        Script::Tifinagh => &["Ebrima"],
        Script::Vai => &["Ebrima"],
        Script::Yi => &["Microsoft Yi Baiti"],
        _ => &[],
    }
}
