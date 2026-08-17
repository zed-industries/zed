// SPDX-License-Identifier: MIT OR Apache-2.0

use unicode_script::Script;

use super::Fallback;

/// A platform-specific font fallback list, for `MacOS`.
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
    &[
        ".SF NS",
        "Menlo",
        "Apple Color Emoji",
        "Geneva",
        "Arial Unicode MS",
    ]
}

// Fallbacks to never use
fn forbidden_fallback() -> &'static [&'static str] {
    &[".LastResort"]
}

fn han_unification(locale: &str) -> &'static [&'static str] {
    match locale {
        // Japan
        "ja" => &["Hiragino Sans"],
        // Korea
        "ko" => &["Apple SD Gothic Neo"],
        // Hong Kong
        "zh-HK" => &["PingFang HK"],
        // Taiwan
        "zh-TW" => &["PingFang TC"],
        // Simplified Chinese is the default (also catches "zh-CN" for China)
        _ => &["PingFang SC"],
    }
}

// Fallbacks to use per script
fn script_fallback(script: Script, locale: &str) -> &'static [&'static str] {
    //TODO: abstract style (sans/serif/monospaced)
    //TODO: pull more data from about:config font.name-list.sans-serif in Firefox
    match script {
        Script::Adlam => &["Noto Sans Adlam"],
        Script::Arabic => &["Geeza Pro"],
        Script::Armenian => &["Noto Sans Armenian"],
        Script::Bengali => &["Bangla Sangam MN"],
        Script::Buhid => &["Noto Sans Buhid"],
        Script::Canadian_Aboriginal => &["Euphemia UCAS"],
        Script::Chakma => &["Noto Sans Chakma"],
        Script::Devanagari => &["Devanagari Sangam MN"],
        Script::Ethiopic => &["Kefa"],
        Script::Gothic => &["Noto Sans Gothic"],
        Script::Grantha => &["Grantha Sangam MN"],
        Script::Gujarati => &["Gujarati Sangam MN"],
        Script::Gurmukhi => &["Gurmukhi Sangam MN"],
        Script::Han => han_unification(locale),
        Script::Hangul => han_unification("ko"),
        Script::Hanunoo => &["Noto Sans Hanunoo"],
        Script::Hebrew => &["Arial"],
        Script::Hiragana => han_unification("ja"),
        Script::Javanese => &["Noto Sans Javanese"],
        Script::Kannada => &["Noto Sans Kannada"],
        Script::Katakana => han_unification("ja"),
        Script::Khmer => &["Khmer Sangam MN"],
        Script::Lao => &["Lao Sangam MN"],
        Script::Malayalam => &["Malayalam Sangam MN"],
        Script::Mongolian => &["Noto Sans Mongolian"],
        Script::Myanmar => &["Noto Sans Myanmar"],
        Script::Oriya => &["Noto Sans Oriya"],
        Script::Sinhala => &["Sinhala Sangam MN"],
        Script::Syriac => &["Noto Sans Syriac"],
        Script::Tagalog => &["Noto Sans Tagalog"],
        Script::Tagbanwa => &["Noto Sans Tagbanwa"],
        Script::Tai_Le => &["Noto Sans Tai Le"],
        Script::Tai_Tham => &["Noto Sans Tai Tham"],
        Script::Tai_Viet => &["Noto Sans Tai Viet"],
        Script::Tamil => &["InaiMathi"],
        Script::Telugu => &["Telugu Sangam MN"],
        Script::Thaana => &["Noto Sans Thaana"],
        Script::Thai => &["Ayuthaya"],
        Script::Tibetan => &["Kailasa"],
        Script::Tifinagh => &["Noto Sans Tifinagh"],
        Script::Vai => &["Noto Sans Vai"],
        //TODO: Use han_unification?
        Script::Yi => &["Noto Sans Yi", "PingFang SC"],
        _ => &[],
    }
}
