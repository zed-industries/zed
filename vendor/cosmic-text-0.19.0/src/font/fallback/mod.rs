// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::{mem, ops::Range};
use fontdb::Family;
use unicode_script::Script;

use crate::{BuildHasher, Font, FontMatchKey, FontSystem, HashMap, ShapeBuffer};

#[cfg(not(any(all(unix, not(target_os = "android")), target_os = "windows")))]
#[path = "other.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod platform;

#[cfg(all(unix, not(any(target_os = "android", target_os = "macos"))))]
#[path = "unix.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform;

/// The `Fallback` trait allows for configurable font fallback lists to be set during construction of the [`FontSystem`].
///
/// A custom fallback list can be added via the [`FontSystem::new_with_locale_and_db_and_fallback`] constructor.
///
/// A default implementation is provided by the [`PlatformFallback`] struct, which encapsulates the target platform's pre-configured fallback lists.
///
/// ```rust
/// # use unicode_script::Script;
/// # use cosmic_text::{Fallback, FontSystem};
/// struct MyFallback;
/// impl Fallback for MyFallback {
///     fn common_fallback(&self) -> &[&'static str] {
///         &[
///             "Segoe UI",
///             "Segoe UI Emoji",
///             "Segoe UI Symbol",
///             "Segoe UI Historic",
///         ]
///     }
///
///     fn forbidden_fallback(&self) -> &[&'static str] {
///         &[]
///     }
///
///     fn script_fallback(&self, script: Script, locale: &str) -> &[&'static str] {
///         match script {
///             Script::Adlam => &["Ebrima"],
///             Script::Bengali => &["Nirmala UI"],
///             Script::Canadian_Aboriginal => &["Gadugi"],
///             // ...
///             _ => &[],
///        }
///     }
/// }
///
/// let locale = "en-US".to_string();
/// let db = fontdb::Database::new();
/// let font_system = FontSystem::new_with_locale_and_db_and_fallback(locale, db, MyFallback);
/// ```
pub trait Fallback: Send + Sync {
    /// Fallbacks to use after any script specific fallbacks
    fn common_fallback(&self) -> &[&'static str];

    /// Fallbacks to never use
    fn forbidden_fallback(&self) -> &[&'static str];

    /// Fallbacks to use per script
    fn script_fallback(&self, script: Script, locale: &str) -> &[&'static str];
}

#[derive(Debug, Default)]
pub struct Fallbacks {
    lists: Vec<&'static str>,
    common_fallback_range: Range<usize>,
    forbidden_fallback_range: Range<usize>,
    // PERF: Consider using NoHashHasher since Script is just an integer
    script_fallback_ranges: HashMap<Script, Range<usize>>,
    locale: String,
}

impl Fallbacks {
    pub(crate) fn new(fallbacks: &dyn Fallback, scripts: &[Script], locale: &str) -> Self {
        let common_fallback = fallbacks.common_fallback();

        let forbidden_fallback = fallbacks.forbidden_fallback();

        let mut lists =
            Vec::with_capacity(common_fallback.len() + forbidden_fallback.len() + scripts.len());

        let mut index = lists.len();
        let mut new_range = |lists: &Vec<&str>| {
            let old_index = index;
            index = lists.len();
            old_index..index
        };

        lists.extend_from_slice(common_fallback);
        let common_fallback_range = new_range(&lists);

        lists.extend_from_slice(forbidden_fallback);
        let forbidden_fallback_range = new_range(&lists);

        let mut script_fallback_ranges =
            HashMap::with_capacity_and_hasher(scripts.len(), BuildHasher::default());
        for &script in scripts {
            let script_fallback = fallbacks.script_fallback(script, locale);
            lists.extend_from_slice(script_fallback);
            let script_fallback_range = new_range(&lists);
            script_fallback_ranges.insert(script, script_fallback_range);
        }

        let locale = locale.to_owned();
        Self {
            lists,
            common_fallback_range,
            forbidden_fallback_range,
            script_fallback_ranges,
            locale,
        }
    }

    pub(crate) fn extend(&mut self, fallbacks: &dyn Fallback, scripts: &[Script]) {
        self.lists.reserve(scripts.len());

        let mut index = self.lists.len();
        let mut new_range = |lists: &Vec<&str>| {
            let old_index = index;
            index = lists.len();
            old_index..index
        };

        for &script in scripts {
            self.script_fallback_ranges
                .entry(script)
                .or_insert_with_key(|&script| {
                    let script_fallback = fallbacks.script_fallback(script, &self.locale);
                    self.lists.extend_from_slice(script_fallback);
                    new_range(&self.lists)
                });
        }
    }

    pub(crate) fn common_fallback(&self) -> &[&'static str] {
        &self.lists[self.common_fallback_range.clone()]
    }

    pub(crate) fn forbidden_fallback(&self) -> &[&'static str] {
        &self.lists[self.forbidden_fallback_range.clone()]
    }

    pub(crate) fn script_fallback(&self, script: Script) -> &[&'static str] {
        self.script_fallback_ranges
            .get(&script)
            .map_or(&[], |range| &self.lists[range.clone()])
    }
}

pub use platform::PlatformFallback;

#[cfg(not(feature = "warn_on_missing_glyphs"))]
use log::debug as missing_warn;
#[cfg(feature = "warn_on_missing_glyphs")]
use log::warn as missing_warn;

// Match on lowest font_weight_diff, then script_non_matches, then font_weight
// Default font gets None for both `weight_offset` and `script_non_matches`, and thus, it is
// always the first to be popped from the set.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonospaceFallbackInfo {
    font_weight_diff: Option<u16>,
    codepoint_non_matches: Option<usize>,
    font_weight: u16,
    id: fontdb::ID,
}

#[derive(Debug)]
pub struct FontFallbackIter<'a> {
    font_system: &'a mut FontSystem,
    font_match_keys: &'a [FontMatchKey],
    default_families: &'a [&'a Family<'a>],
    default_i: usize,
    scripts: &'a [Script],
    word: &'a str,
    script_i: (usize, usize),
    common_i: usize,
    other_i: usize,
    end: bool,
    ideal_weight: fontdb::Weight,
}

impl<'a> FontFallbackIter<'a> {
    pub fn new(
        font_system: &'a mut FontSystem,
        font_match_keys: &'a [FontMatchKey],
        default_families: &'a [&'a Family<'a>],
        scripts: &'a [Script],
        word: &'a str,
        ideal_weight: fontdb::Weight,
    ) -> Self {
        font_system
            .fallbacks
            .extend(font_system.dyn_fallback.as_ref(), scripts);
        font_system.monospace_fallbacks_buffer.clear();
        Self {
            font_system,
            font_match_keys,
            default_families,
            default_i: 0,
            scripts,
            word,
            script_i: (0, 0),
            common_i: 0,
            other_i: 0,
            end: false,
            ideal_weight,
        }
    }

    pub fn check_missing(&self, word: &str) {
        if self.end {
            missing_warn!(
                "Failed to find any fallback for {:?} locale '{}': '{}'",
                self.scripts,
                self.font_system.locale(),
                word
            );
        } else if self.other_i > 0 {
            missing_warn!(
                "Failed to find preset fallback for {:?} locale '{}', used '{}': '{}'",
                self.scripts,
                self.font_system.locale(),
                self.face_name(self.font_match_keys[self.other_i - 1].id),
                word
            );
        } else if !self.scripts.is_empty() && self.common_i > 0 {
            let family = self.font_system.fallbacks.common_fallback()[self.common_i - 1];
            missing_warn!(
                "Failed to find script fallback for {:?} locale '{}', used '{}': '{}'",
                self.scripts,
                self.font_system.locale(),
                family,
                word
            );
        }
    }

    pub fn face_name(&self, id: fontdb::ID) -> &str {
        self.font_system
            .db()
            .face(id)
            .map_or("invalid font id", |face| {
                if let Some((name, _)) = face.families.first() {
                    name
                } else {
                    &face.post_script_name
                }
            })
    }

    pub fn shape_caches(&mut self) -> &mut ShapeBuffer {
        &mut self.font_system.shape_buffer
    }

    fn face_contains_family(&self, id: fontdb::ID, family_name: &str) -> bool {
        self.font_system
            .db()
            .face(id)
            .is_some_and(|face| face.families.iter().any(|(name, _)| name == family_name))
    }

    fn default_font_match_key(&self) -> Option<&FontMatchKey> {
        let default_family = self.default_families[self.default_i - 1];
        let default_family_name = self.font_system.db().family_name(default_family);

        self.font_match_keys
            .iter()
            .filter(|m_key| m_key.font_weight_diff == 0 || m_key.variable_weight_match)
            .find(|m_key| self.face_contains_family(m_key.id, default_family_name))
    }

    fn next_item(&mut self, fallbacks: &Fallbacks) -> Option<<Self as Iterator>::Item> {
        if let Some(fallback_info) = self.font_system.monospace_fallbacks_buffer.pop_first() {
            if let Some(font) = self
                .font_system
                .get_font(fallback_info.id, self.ideal_weight)
            {
                return Some(font);
            }
        }

        let font_match_keys_iter = |is_mono| {
            self.font_match_keys.iter().filter(move |m_key| {
                m_key.font_weight_diff == 0 || m_key.variable_weight_match || is_mono
            })
        };

        'DEF_FAM: while self.default_i < self.default_families.len() {
            self.default_i += 1;
            let is_mono = self.default_families[self.default_i - 1] == &Family::Monospace;
            let default_font_match_key = self.default_font_match_key().copied();
            let word_chars_count = self.word.chars().count();

            macro_rules! mk_mono_fallback_info {
                ($m_key:expr) => {{
                    let supported_cp_count_opt =
                        self.font_system.get_font_supported_codepoints_in_word(
                            $m_key.id,
                            self.ideal_weight,
                            self.word,
                        );

                    supported_cp_count_opt.map(|supported_cp_count| {
                        let codepoint_non_matches = word_chars_count - supported_cp_count;

                        MonospaceFallbackInfo {
                            font_weight_diff: Some($m_key.font_weight_diff),
                            codepoint_non_matches: Some(codepoint_non_matches),
                            font_weight: $m_key.font_weight,
                            id: $m_key.id,
                        }
                    })
                }};
            }

            match (is_mono, default_font_match_key.as_ref()) {
                (false, None) => {
                    missing_warn!(
                        "No default font match for {:?} at weight {}",
                        self.default_families[self.default_i - 1],
                        self.ideal_weight.0,
                    );
                    break 'DEF_FAM;
                }
                (false, Some(m_key)) => {
                    if let Some(font) = self.font_system.get_font(m_key.id, self.ideal_weight) {
                        return Some(font);
                    }
                    break 'DEF_FAM;
                }
                (true, None) => (),
                (true, Some(m_key)) => {
                    // Default Monospace font
                    if let Some(mut fallback_info) = mk_mono_fallback_info!(m_key) {
                        fallback_info.font_weight_diff = None;

                        // Return early if default Monospace font supports all word codepoints.
                        // Otherewise, add to fallbacks set
                        if fallback_info.codepoint_non_matches == Some(0) {
                            if let Some(font) =
                                self.font_system.get_font(m_key.id, self.ideal_weight)
                            {
                                return Some(font);
                            }
                        } else {
                            assert!(self
                                .font_system
                                .monospace_fallbacks_buffer
                                .insert(fallback_info));
                        }
                    }
                }
            }

            let mono_ids_for_scripts = if is_mono && !self.scripts.is_empty() {
                let scripts = self.scripts.iter().filter_map(|script| {
                    let script_as_lower = script.short_name().to_lowercase();
                    <[u8; 4]>::try_from(script_as_lower.as_bytes()).ok()
                });
                self.font_system.get_monospace_ids_for_scripts(scripts)
            } else {
                Vec::new()
            };

            for m_key in font_match_keys_iter(is_mono) {
                if Some(m_key.id) != default_font_match_key.as_ref().map(|m_key| m_key.id) {
                    let is_mono_id = if mono_ids_for_scripts.is_empty() {
                        self.font_system.is_monospace(m_key.id)
                    } else {
                        mono_ids_for_scripts.binary_search(&m_key.id).is_ok()
                    };

                    if is_mono_id {
                        let supported_cp_count_opt =
                            self.font_system.get_font_supported_codepoints_in_word(
                                m_key.id,
                                self.ideal_weight,
                                self.word,
                            );
                        if let Some(supported_cp_count) = supported_cp_count_opt {
                            let codepoint_non_matches =
                                self.word.chars().count() - supported_cp_count;

                            let fallback_info = MonospaceFallbackInfo {
                                font_weight_diff: Some(m_key.font_weight_diff),
                                codepoint_non_matches: Some(codepoint_non_matches),
                                font_weight: m_key.font_weight,
                                id: m_key.id,
                            };
                            assert!(self
                                .font_system
                                .monospace_fallbacks_buffer
                                .insert(fallback_info));
                        }
                    }
                }
            }
            // If default family is Monospace fallback to first monospaced font
            if let Some(fallback_info) = self.font_system.monospace_fallbacks_buffer.pop_first() {
                if let Some(font) = self
                    .font_system
                    .get_font(fallback_info.id, self.ideal_weight)
                {
                    return Some(font);
                }
            }
        }

        while self.script_i.0 < self.scripts.len() {
            let script = self.scripts[self.script_i.0];

            let script_families = fallbacks.script_fallback(script);

            while self.script_i.1 < script_families.len() {
                let script_family = script_families[self.script_i.1];
                self.script_i.1 += 1;
                for m_key in font_match_keys_iter(false) {
                    if self.face_contains_family(m_key.id, script_family) {
                        if let Some(font) = self.font_system.get_font(m_key.id, self.ideal_weight) {
                            return Some(font);
                        }
                    }
                }
                log::debug!(
                    "failed to find family '{}' for script {:?} and locale '{}'",
                    script_family,
                    script,
                    self.font_system.locale(),
                );
            }

            self.script_i.0 += 1;
            self.script_i.1 = 0;
        }

        let common_families = fallbacks.common_fallback();
        while self.common_i < common_families.len() {
            let common_family = common_families[self.common_i];
            self.common_i += 1;
            for m_key in font_match_keys_iter(false) {
                if self.face_contains_family(m_key.id, common_family) {
                    if let Some(font) = self.font_system.get_font(m_key.id, self.ideal_weight) {
                        return Some(font);
                    }
                }
            }
            log::debug!("failed to find family '{common_family}'");
        }

        //TODO: do we need to do this?
        //TODO: do not evaluate fonts more than once!
        let forbidden_families = fallbacks.forbidden_fallback();
        while self.other_i < self.font_match_keys.len() {
            let id = self.font_match_keys[self.other_i].id;
            self.other_i += 1;
            if forbidden_families
                .iter()
                .all(|family_name| !self.face_contains_family(id, family_name))
            {
                if let Some(font) = self.font_system.get_font(id, self.ideal_weight) {
                    return Some(font);
                }
            }
        }

        self.end = true;
        None
    }
}

impl Iterator for FontFallbackIter<'_> {
    type Item = Arc<Font>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut fallbacks = mem::take(&mut self.font_system.fallbacks);
        let item = self.next_item(&fallbacks);
        mem::swap(&mut fallbacks, &mut self.font_system.fallbacks);
        item
    }
}
