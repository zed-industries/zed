use crate::{LanguageId, LanguageMatcher, LanguageName, LoadedLanguage, ManifestName};
use anyhow::Result;
use collections::FxHashMap;
use globset::GlobSet;
use smallvec::SmallVec;
use std::{cell::LazyCell, path::Path, sync::Arc};
use sum_tree::Bias;
use text::{Point, Rope};
use unicase::UniCase;

#[derive(Clone)]
pub struct AvailableLanguage {
    pub(super) id: LanguageId,
    pub(super) name: LanguageName,
    pub(super) grammar: Option<Arc<str>>,
    pub(super) matcher: Arc<LanguageMatcher>,
    pub(super) hidden: bool,
    pub(super) load: Arc<dyn Fn() -> Result<LoadedLanguage> + 'static + Send + Sync>,
    pub(super) loaded: bool,
    pub(super) manifest_name: Option<ManifestName>,
}

impl AvailableLanguage {
    pub fn id(&self) -> LanguageId {
        self.id
    }

    pub fn name(&self) -> LanguageName {
        self.name.clone()
    }

    pub fn matcher(&self) -> &LanguageMatcher {
        &self.matcher
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }
}

#[derive(Default)]
pub(super) struct AvailableLanguages(Vec<AvailableLanguage>);

#[derive(Copy, Clone, Default)]
enum LanguageMatchPrecedence {
    #[default]
    Undetermined,
    PathOrContent(usize),
    UserConfigured(usize),
}

impl AvailableLanguages {
    pub(super) fn register(
        &mut self,
        name: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: Arc<LanguageMatcher>,
        hidden: bool,
        manifest_name: Option<ManifestName>,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + 'static + Send + Sync>,
    ) -> bool {
        if let Some(existing_language) = self
            .0
            .iter_mut()
            .find(|existing_language| existing_language.name == name)
        {
            existing_language.grammar = grammar;
            existing_language.matcher = matcher;
            existing_language.load = load;
            existing_language.manifest_name = manifest_name;
            false
        } else {
            self.add(AvailableLanguage {
                id: LanguageId::new(),
                name,
                grammar,
                matcher,
                hidden,
                load,
                loaded: false,
                manifest_name,
            });
            true
        }
    }

    pub(super) fn add(&mut self, language: AvailableLanguage) {
        self.0.push(language);
    }

    pub(super) fn unloaded_language_names(&self) -> Vec<LanguageName> {
        self.0
            .iter()
            .filter_map(|language| (!language.loaded).then_some(language.name.clone()))
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(super) fn name_for_id(&self, id: LanguageId) -> Option<LanguageName> {
        self.0
            .iter()
            .find(|language| language.id == id)
            .map(|language| language.name.clone())
    }

    pub(super) fn get_language(&self, id: LanguageId) -> Option<&AvailableLanguage> {
        self.0.iter().find(|language| language.id == id)
    }

    pub(super) fn find_name_by_extension(&self, extension: &str) -> Option<LanguageName> {
        self.0
            .iter()
            .find(|language| {
                language
                    .matcher
                    .path_suffixes
                    .iter()
                    .any(|suffix| suffix == extension)
            })
            .map(|language| language.name.clone())
    }

    pub(super) fn find_by_exact_name(&self, name: &str) -> Option<AvailableLanguage> {
        self.0
            .iter()
            .find(|language| language.name.0.as_ref() == name)
            .cloned()
    }

    pub(super) fn find_by_modeline_name(&self, modeline_name: &str) -> Option<AvailableLanguage> {
        let modeline_name = modeline_name.to_lowercase();
        self.0
            .iter()
            .find(|language| {
                language
                    .matcher
                    .modeline_aliases
                    .iter()
                    .any(|alias| alias.to_lowercase() == modeline_name)
            })
            .or_else(|| {
                self.0.iter().find(|language| {
                    language
                        .grammar
                        .as_ref()
                        .is_some_and(|grammar| grammar.to_lowercase() == modeline_name)
                })
            })
            .or_else(|| {
                self.0
                    .iter()
                    .find(|language| language.name.0.to_lowercase() == modeline_name)
            })
            .cloned()
    }

    pub(super) fn mark_all_unloaded(&mut self) {
        for language in &mut self.0 {
            language.loaded = false;
        }
    }

    pub(super) fn remove(&mut self, names: &[LanguageName]) {
        self.0.retain(|language| !names.contains(&language.name));
    }

    pub(super) fn mark_loaded(&mut self, id: LanguageId) {
        if let Some(language) = self.0.iter_mut().find(|language| language.id == id) {
            language.loaded = true;
        }
    }

    pub(super) fn find_by_name(&self, name: &str) -> Option<LanguageId> {
        let name = UniCase::new(name);
        self.find_best_match(
            |language_name, _, current_best_match| match current_best_match {
                LanguageMatchPrecedence::Undetermined if UniCase::new(&language_name.0) == name => {
                    Some(LanguageMatchPrecedence::PathOrContent(name.len()))
                }
                LanguageMatchPrecedence::Undetermined
                | LanguageMatchPrecedence::UserConfigured(_)
                | LanguageMatchPrecedence::PathOrContent(_) => None,
            },
        )
    }

    pub(super) fn find_by_name_or_extension(&self, string: &str) -> Option<LanguageId> {
        let string = UniCase::new(string);
        self.find_best_match(|name, matcher, current_best_match| {
            let name_matches = || {
                UniCase::new(&name.0) == string
                    || matcher
                        .path_suffixes
                        .iter()
                        .any(|suffix| UniCase::new(suffix) == string)
            };

            match current_best_match {
                LanguageMatchPrecedence::Undetermined => {
                    name_matches().then_some(LanguageMatchPrecedence::PathOrContent(string.len()))
                }
                LanguageMatchPrecedence::PathOrContent(len) => (string.len() > len
                    && name_matches())
                .then_some(LanguageMatchPrecedence::PathOrContent(string.len())),
                LanguageMatchPrecedence::UserConfigured(_) => None,
            }
        })
    }

    pub(super) fn find_for_file(
        &self,
        path: &Path,
        content: Option<&Rope>,
        user_file_types: Option<&FxHashMap<Arc<str>, (GlobSet, Vec<String>)>>,
    ) -> Option<LanguageId> {
        let filename = path.file_name().and_then(|filename| filename.to_str());
        // `Path.extension()` returns None for files with a leading '.'
        // and no other extension which is not the desired behavior here,
        // as we want `.zshrc` to result in extension being `Some("zshrc")`
        let extension = filename.and_then(|filename| filename.split('.').next_back());
        let path_suffixes = [extension, filename, path.to_str()]
            .iter()
            .filter_map(|suffix| suffix.map(|suffix| (suffix, globset::Candidate::new(suffix))))
            .collect::<SmallVec<[_; 3]>>();
        let content = LazyCell::new(|| {
            content.map(|content| {
                let end = content.clip_point(Point::new(0, 256), Bias::Left);
                let end = content.point_to_offset(end);
                content.chunks_in_range(0..end).collect::<String>()
            })
        });

        self.find_best_match(move |language_name, matcher, current_best_match| {
            let path_matches_default_suffix = || {
                let len =
                    matcher
                        .path_suffixes
                        .iter()
                        .fold(0, |acc: usize, path_suffix: &String| {
                            let ext = ".".to_string() + path_suffix;
                            let matched_suffix_len = path_suffixes
                                .iter()
                                .find(|(suffix, _)| suffix.ends_with(&ext) || suffix == path_suffix)
                                .map(|(suffix, _)| suffix.len());

                            matched_suffix_len.map_or(acc, |len| acc.max(len))
                        });
                (len > 0).then_some(len)
            };

            let path_matches_custom_suffix = || {
                user_file_types
                    .and_then(|types| types.get(language_name.as_ref()))
                    .and_then(|(custom_suffixes, _)| {
                        path_suffixes
                            .iter()
                            .find(|(_, candidate)| custom_suffixes.is_match_candidate(candidate))
                            .map(|(suffix, _)| suffix.len())
                    })
            };

            let content_matches = || {
                matcher.first_line_pattern.as_ref().is_some_and(|pattern| {
                    content
                        .as_ref()
                        .is_some_and(|content| pattern.is_match(content))
                })
            };

            // Only return a match for the given file if we have a better match than
            // the current one.
            match current_best_match {
                LanguageMatchPrecedence::PathOrContent(current_len) => {
                    if let Some(len) = path_matches_custom_suffix() {
                        // >= because user config should win tie with system ext len
                        (len >= current_len).then_some(LanguageMatchPrecedence::UserConfigured(len))
                    } else if let Some(len) = path_matches_default_suffix() {
                        // >= because user config should win tie with system ext len
                        (len >= current_len).then_some(LanguageMatchPrecedence::PathOrContent(len))
                    } else {
                        None
                    }
                }
                LanguageMatchPrecedence::Undetermined => {
                    if let Some(len) = path_matches_custom_suffix() {
                        Some(LanguageMatchPrecedence::UserConfigured(len))
                    } else if let Some(len) = path_matches_default_suffix() {
                        Some(LanguageMatchPrecedence::PathOrContent(len))
                    } else if content_matches() {
                        Some(LanguageMatchPrecedence::PathOrContent(1))
                    } else {
                        None
                    }
                }
                LanguageMatchPrecedence::UserConfigured(_) => None,
            }
        })
    }

    fn find_best_match(
        &self,
        callback: impl Fn(
            &LanguageName,
            &LanguageMatcher,
            LanguageMatchPrecedence,
        ) -> Option<LanguageMatchPrecedence>,
    ) -> Option<LanguageId> {
        self.0
            .iter()
            .rev()
            .fold(None, |best_language_match, language| {
                let current_match_type = best_language_match
                    .as_ref()
                    .map_or(LanguageMatchPrecedence::default(), |(_, score)| *score);
                let language_score =
                    callback(&language.name, &language.matcher, current_match_type);

                match (language_score, current_match_type) {
                    // no current best, so our candidate is better
                    (
                        Some(
                            LanguageMatchPrecedence::PathOrContent(_)
                            | LanguageMatchPrecedence::UserConfigured(_),
                        ),
                        LanguageMatchPrecedence::Undetermined,
                    ) => language_score.map(|new_score| (language, new_score)),

                    // our candidate is better only if the name is longer
                    (
                        Some(LanguageMatchPrecedence::PathOrContent(new_len)),
                        LanguageMatchPrecedence::PathOrContent(current_len),
                    )
                    | (
                        Some(LanguageMatchPrecedence::UserConfigured(new_len)),
                        LanguageMatchPrecedence::UserConfigured(current_len),
                    )
                    | (
                        Some(LanguageMatchPrecedence::PathOrContent(new_len)),
                        LanguageMatchPrecedence::UserConfigured(current_len),
                    ) => {
                        if new_len > current_len {
                            language_score.map(|new_score| (language, new_score))
                        } else {
                            best_language_match
                        }
                    }

                    // our candidate is better if the name is longer or equal to
                    (
                        Some(LanguageMatchPrecedence::UserConfigured(new_len)),
                        LanguageMatchPrecedence::PathOrContent(current_len),
                    ) => {
                        if new_len >= current_len {
                            language_score.map(|new_score| (language, new_score))
                        } else {
                            best_language_match
                        }
                    }
                    // no candidate, use current best
                    (None, _) | (Some(LanguageMatchPrecedence::Undetermined), _) => {
                        best_language_match
                    }
                }
            })
            .map(|(available_language, _)| available_language.id())
    }
}
