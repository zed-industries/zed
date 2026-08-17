//! # i18n
//!
//! Internationalization and Localization infrastructure for Zed.
//! (Section 6.2 & Phase 3.1 of Space-Grade Audit)
//!
//! Supports runtime locale switching, fallback resolution chains, parameter
//! interpolation, and embedded dictionaries for core locales:
//! - en (English, default)
//! - zh_CN (Simplified Chinese)
//! - de (German)
//! - es (Spanish)
//! - ja (Japanese)

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// Available supported language locales
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Locale {
    En,
    ZhCn,
    De,
    Es,
    Ja,
}

impl Locale {
    pub fn code(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::ZhCn => "zh-CN",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Ja => "ja",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().replace('_', "-").as_str() {
            "en" | "en-us" | "en-gb" => Some(Locale::En),
            "zh" | "zh-cn" | "zh-hans" => Some(Locale::ZhCn),
            "de" | "de-de" | "de-at" => Some(Locale::De),
            "es" | "es-es" | "es-mx" => Some(Locale::Es),
            "ja" | "ja-jp" => Some(Locale::Ja),
            _ => None,
        }
    }
}

impl Default for Locale {
    fn default() -> Self {
        Locale::En
    }
}

/// Global Localization Registry
pub struct I18nRegistry {
    current_locale: Locale,
    translations: HashMap<Locale, HashMap<String, String>>,
}

impl I18nRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            current_locale: Locale::En,
            translations: HashMap::new(),
        };
        registry.load_embedded_dictionaries();
        registry
    }

    pub fn set_locale(&mut self, locale: Locale) {
        self.current_locale = locale;
    }

    pub fn current_locale(&self) -> Locale {
        self.current_locale
    }

    /// Load default strings for built-in locales from embedded JSON files
    fn load_embedded_dictionaries(&mut self) {
        const EN_JSON: &str = include_str!("../locales/en.json");
        const ZH_JSON: &str = include_str!("../locales/zh-CN.json");
        const DE_JSON: &str = include_str!("../locales/de.json");
        const ES_JSON: &str = include_str!("../locales/es.json");
        const JA_JSON: &str = include_str!("../locales/ja.json");

        if let Ok(dict) = serde_json::from_str::<HashMap<String, String>>(EN_JSON) {
            self.translations.insert(Locale::En, dict);
        }
        if let Ok(dict) = serde_json::from_str::<HashMap<String, String>>(ZH_JSON) {
            self.translations.insert(Locale::ZhCn, dict);
        }
        if let Ok(dict) = serde_json::from_str::<HashMap<String, String>>(DE_JSON) {
            self.translations.insert(Locale::De, dict);
        }
        if let Ok(dict) = serde_json::from_str::<HashMap<String, String>>(ES_JSON) {
            self.translations.insert(Locale::Es, dict);
        }
        if let Ok(dict) = serde_json::from_str::<HashMap<String, String>>(JA_JSON) {
            self.translations.insert(Locale::Ja, dict);
        }
    }

    /// Translate a key with parameter replacement and fallback to English
    pub fn translate(&self, key: &str, params: &[(&str, &str)]) -> String {
        let text = self.translations
            .get(&self.current_locale)
            .and_then(|dict| dict.get(key))
            .or_else(|| {
                // Fallback to English
                self.translations.get(&Locale::En).and_then(|dict| dict.get(key))
            })
            .cloned()
            .unwrap_or_else(|| key.to_string());

        let mut result = text;
        for (param_key, param_val) in params {
            let placeholder = format!("{{{}}}", param_key);
            result = result.replace(&placeholder, param_val);
        }
        result
    }
}

impl Default for I18nRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global singleton access
static GLOBAL_I18N: once_cell::sync::Lazy<Arc<RwLock<I18nRegistry>>> =
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(I18nRegistry::new())));

/// Translate key with the active global locale
pub fn t(key: &str) -> String {
    GLOBAL_I18N.read().translate(key, &[])
}

/// Translate key with dynamic parameter interpolation
pub fn t_args(key: &str, params: &[(&str, &str)]) -> String {
    GLOBAL_I18N.read().translate(key, params)
}

/// Set active global locale
pub fn set_locale(locale: Locale) {
    GLOBAL_I18N.write().set_locale(locale);
}

/// Get current global locale
pub fn current_locale() -> Locale {
    GLOBAL_I18N.read().current_locale()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_fallback_and_switch() {
        let mut registry = I18nRegistry::new();
        assert_eq!(registry.translate("file.save", &[]), "Save File");

        registry.set_locale(Locale::ZhCn);
        assert_eq!(registry.translate("file.save", &[]), "保存文件");

        registry.set_locale(Locale::De);
        assert_eq!(registry.translate("file.save", &[]), "Datei speichern");

        registry.set_locale(Locale::Ja);
        assert_eq!(registry.translate("file.save", &[]), "ファイルを保存");

        // Parameter interpolation
        assert_eq!(
            registry.translate("daemon.running", &[("addr", "127.0.0.1:9257")]),
            "デーモンが 127.0.0.1:9257 で稼働中です"
        );
    }
}
