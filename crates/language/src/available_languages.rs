use crate::{
    LanguageId, LanguageLoader, LanguageMatcher, LanguageName, LanguageRegistrationStatus,
};
use collections::FxHashMap;
use globset::{GlobSet, GlobSetBuilder};
use gpui::SharedString;
use regex::RegexSet;
use smallvec::SmallVec;
use std::{path::Path, sync::Arc};
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
    pub(super) load: LanguageLoader,
    pub(super) loaded: bool,
    pub(super) origin: LanguageOrigin,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LanguageOrigin {
    Native,
    Extension,
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
pub(super) struct AvailableLanguages {
    languages: Vec<AvailableLanguage>,
    lookup: LanguageLookup,
}

/// A position in `AvailableLanguages::languages`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LanguageIndex(usize);

/// A position in the `LanguageMatcher::path_suffixes` of a language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PathSuffixIndex(usize);

/// Lookup structures over `AvailableLanguages::languages`, rebuilt whenever the
/// set of languages changes, so that detecting a language doesn't need to visit
/// every registered language.
#[derive(Default)]
struct LanguageLookup {
    suffix_globs: Option<GlobSet>,
    /// The suffix that each glob in `suffix_globs` was built from, indexed by glob.
    suffixes: Vec<(LanguageIndex, PathSuffixIndex)>,
    first_line_patterns: Option<RegexSet>,
    /// The language of each pattern in `first_line_patterns`, indexed by pattern.
    first_line_pattern_languages: Vec<LanguageIndex>,
    languages_by_name: FxHashMap<LanguageName, SmallVec<[LanguageIndex; 1]>>,
    /// Modeline aliases, grammar names and language names, each mapping to the
    /// first language that declares them.
    modeline_aliases: FxHashMap<UniCase<SharedString>, LanguageIndex>,
    grammar_names: FxHashMap<UniCase<SharedString>, LanguageIndex>,
    language_names: FxHashMap<UniCase<SharedString>, LanguageIndex>,
}

impl LanguageLookup {
    fn new(languages: &[AvailableLanguage]) -> Self {
        let mut suffix_globs = GlobSetBuilder::new();
        let mut suffixes = Vec::new();
        let mut first_line_patterns = Vec::new();
        let mut first_line_pattern_languages = Vec::new();
        let mut languages_by_name = FxHashMap::<_, SmallVec<_>>::default();
        let mut modeline_aliases = FxHashMap::default();
        let mut grammar_names = FxHashMap::default();
        let mut language_names = FxHashMap::default();

        for (language_index, language) in languages.iter().enumerate() {
            let language_index = LanguageIndex(language_index);
            for (suffix_index, suffix) in language.matcher.path_suffixes.iter().enumerate() {
                let suffix_index = PathSuffixIndex(suffix_index);
                match suffix.glob() {
                    Ok(glob) => {
                        suffix_globs.add(glob);
                        suffixes.push((language_index, suffix_index));
                    }
                    Err(error) => {
                        log::error!(
                            "invalid path suffix {suffix:?} for language {}: {error}",
                            language.name
                        );
                    }
                }
            }
            if let Some(pattern) = &language.matcher.first_line_pattern {
                first_line_patterns.push(pattern.as_str());
                first_line_pattern_languages.push(language_index);
            }
            languages_by_name
                .entry(language.name.clone())
                .or_default()
                .push(language_index);
            for alias in &language.matcher.modeline_aliases {
                modeline_aliases
                    .entry(UniCase::new(alias.clone()))
                    .or_insert(language_index);
            }
            if let Some(grammar) = &language.grammar {
                grammar_names
                    .entry(UniCase::new(SharedString::from(grammar)))
                    .or_insert(language_index);
            }
            language_names
                .entry(UniCase::new(language.name.0.clone()))
                .or_insert(language_index);
        }

        let suffix_globs = suffix_globs
            .build()
            .inspect_err(|error| log::error!("failed to compile language suffix index: {error}"))
            .ok();
        let first_line_patterns = if first_line_patterns.is_empty() {
            None
        } else {
            RegexSet::new(first_line_patterns)
                .inspect_err(|error| {
                    log::error!("failed to compile language first line patterns: {error}")
                })
                .ok()
        };

        Self {
            suffix_globs,
            suffixes,
            first_line_patterns,
            first_line_pattern_languages,
            languages_by_name,
            modeline_aliases,
            grammar_names,
            language_names,
        }
    }

    /// Returns `(language, score)` pairs, sorted by language, for every language
    /// with a path suffix matching one of the `candidates`, which are built from
    /// `filename` and `paths`.
    fn suffix_scores(
        &self,
        languages: &[AvailableLanguage],
        filename: Option<&str>,
        paths: &[&Path],
        candidates: &[(&str, globset::Candidate<'_>)],
    ) -> SmallVec<[(LanguageIndex, usize); 4]> {
        let score = |&(language_index, suffix_index): &(LanguageIndex, PathSuffixIndex)| {
            let suffix = languages
                .get(language_index.0)?
                .matcher
                .path_suffixes
                .get(suffix_index.0)?;
            // The glob is only a suffix filter. Verify the dot boundary and recover
            // the original score, including literal Windows separators.
            let score = suffix_score(suffix, candidates);
            (score > 0).then_some((language_index, score))
        };

        let mut scores: SmallVec<[(LanguageIndex, usize); 4]> =
            match (&self.suffix_globs, Self::glob_targets(filename, paths)) {
                (Some(globs), Some(targets)) => {
                    let mut glob_indices = Vec::new();
                    let mut target_matches = Vec::new();
                    for target in &targets {
                        globs.matches_candidate_into(
                            &globset::Candidate::new(target),
                            &mut target_matches,
                        );
                        glob_indices.append(&mut target_matches);
                    }
                    glob_indices.sort_unstable();
                    glob_indices.dedup();
                    glob_indices
                        .into_iter()
                        .filter_map(|glob_index| self.suffixes.get(glob_index))
                        .filter_map(score)
                        .collect()
                }
                _ => self.suffixes.iter().filter_map(score).collect(),
            };
        scores.sort_unstable_by(|(language_a, score_a), (language_b, score_b)| {
            language_a.cmp(language_b).then(score_b.cmp(score_a))
        });
        scores.dedup_by_key(|(language_index, _)| *language_index);
        scores
    }

    /// Returns the texts to match the suffix globs against, such that every
    /// candidate built from `filename` and `paths` is a suffix of one of them.
    ///
    /// Returns `None` if a path needs an exact scan instead: for non-UTF8 parents
    /// and trailing separators or `/.`, `Path::file_name` and globset do not agree
    /// about the path's basename.
    fn glob_targets<'a>(
        filename: Option<&'a str>,
        paths: &[&'a Path],
    ) -> Option<SmallVec<[&'a str; 4]>> {
        let mut targets = SmallVec::<[&str; 4]>::new();
        for path in paths {
            let Some(path_text) = path.to_str() else {
                continue;
            };
            let path_filename = path.file_name().and_then(|name| name.to_str())?;
            if !path_text.ends_with(path_filename) {
                return None;
            }
            targets.push(path_text);
        }
        // The extension is always a suffix of the filename, so covering the filename
        // covers both of them.
        if let Some(filename) = filename
            && !targets.iter().any(|target| target.ends_with(filename))
        {
            if filename.is_empty() || filename.contains(['/', '\\']) {
                return None;
            }
            targets.push(filename);
        }
        Some(targets)
    }

    fn first_line_matches(&self, content: &str) -> SmallVec<[LanguageIndex; 2]> {
        let Some(patterns) = &self.first_line_patterns else {
            return SmallVec::new();
        };
        patterns
            .matches(content)
            .into_iter()
            .filter_map(|pattern_index| self.first_line_pattern_languages.get(pattern_index))
            .copied()
            .collect()
    }
}

fn suffix_score(suffix: &str, candidates: &[(&str, globset::Candidate<'_>)]) -> usize {
    candidates
        .iter()
        .find(|(text, _)| {
            *text == suffix
                || text
                    .strip_suffix(suffix)
                    .is_some_and(|prefix| prefix.ends_with('.'))
        })
        .map_or(0, |(text, _)| text.len())
}

#[derive(Copy, Clone, Default)]
enum LanguageMatchPrecedence {
    #[default]
    Undetermined,
    PathOrContent(usize),
    UserConfigured(usize),
}

/// The result of [`AvailableLanguages::update`].
pub(super) struct AvailableLanguagesUpdate {
    /// The extension languages that were removed.
    pub(super) removed: Vec<LanguageName>,
    /// The loaded languages that were replaced by a new registration.
    pub(super) reloaded: SmallVec<[LanguageName; 4]>,
    /// The status of each registration, in the order they were passed.
    pub(super) registrations: SmallVec<[LanguageRegistrationStatus; 8]>,
}

impl AvailableLanguages {
    /// Applies the removals before the registrations, rebuilding the lookup at most once.
    ///
    /// Only extension languages are removed, and an extension registration is
    /// rejected if a language with the same name is registered natively.
    pub(super) fn update(
        &mut self,
        names_to_remove: &[LanguageName],
        registrations: Vec<AvailableLanguage>,
    ) -> AvailableLanguagesUpdate {
        let mut update = AvailableLanguagesUpdate {
            removed: Vec::new(),
            reloaded: SmallVec::new(),
            registrations: SmallVec::with_capacity(registrations.len()),
        };
        if names_to_remove.is_empty() && registrations.is_empty() {
            return update;
        }

        let mut languages = self.languages.clone();
        languages.retain(|language| {
            let remove = language.origin == LanguageOrigin::Extension
                && names_to_remove.contains(&language.name);
            if remove {
                update.removed.push(language.name.clone());
            }
            !remove
        });
        for registration in registrations {
            if let Some(existing) = languages
                .iter_mut()
                .find(|language| language.name == registration.name)
            {
                if registration.origin == LanguageOrigin::Extension
                    && existing.origin == LanguageOrigin::Native
                {
                    log::warn!(
                        "not registering extension language {}: \
                        a language with this name is already registered outside of extensions",
                        registration.name
                    );
                    update
                        .registrations
                        .push(LanguageRegistrationStatus::Rejected);
                    continue;
                }
                if existing.loaded {
                    update.reloaded.push(existing.name.clone());
                }
                *existing = registration;
            } else {
                languages.push(registration);
            }
            update
                .registrations
                .push(LanguageRegistrationStatus::Registered);
        }
        if !update.removed.is_empty()
            || update
                .registrations
                .iter()
                .any(|status| status.is_registered())
        {
            self.replace_languages(languages);
        }
        update
    }

    pub(super) fn add(&mut self, language: AvailableLanguage) {
        let mut languages = self.languages.clone();
        languages.push(language);
        self.replace_languages(languages);
    }

    fn replace_languages(&mut self, languages: Vec<AvailableLanguage>) {
        // Build before publishing so lookups always see registrations and their matching index.
        let lookup = LanguageLookup::new(&languages);
        *self = Self { languages, lookup };
    }

    fn language_at(&self, index: LanguageIndex) -> Option<&AvailableLanguage> {
        self.languages.get(index.0)
    }

    pub(super) fn unloaded_language_names(&self) -> Vec<LanguageName> {
        self.languages
            .iter()
            .filter_map(|language| (!language.loaded).then_some(language.name.clone()))
            .collect()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(super) fn name_for_id(&self, id: LanguageId) -> Option<LanguageName> {
        self.languages
            .iter()
            .find(|language| language.id == id)
            .map(|language| language.name.clone())
    }

    pub(super) fn get_language(&self, id: LanguageId) -> Option<&AvailableLanguage> {
        self.languages.iter().find(|language| language.id == id)
    }

    pub(super) fn find_name_by_extension(&self, extension: &str) -> Option<LanguageName> {
        self.languages
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
        self.languages
            .iter()
            .find(|language| language.name.0.as_ref() == name)
            .cloned()
    }

    pub(super) fn find_by_modeline_name(&self, modeline_name: &str) -> Option<AvailableLanguage> {
        let modeline_name = UniCase::new(SharedString::from(modeline_name));
        [
            &self.lookup.modeline_aliases,
            &self.lookup.grammar_names,
            &self.lookup.language_names,
        ]
        .into_iter()
        .find_map(|names| names.get(&modeline_name))
        .and_then(|language_index| self.language_at(*language_index))
        .cloned()
    }

    pub(super) fn mark_all_unloaded(&mut self) {
        for language in &mut self.languages {
            language.loaded = false;
        }
    }

    pub(super) fn mark_loaded(&mut self, id: LanguageId) {
        if let Some(language) = self.languages.iter_mut().find(|language| language.id == id) {
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
        filename: Option<&str>,
        paths: &[&Path],
        content: Option<&Rope>,
        user_file_types: Option<&FxHashMap<Arc<str>, (GlobSet, Vec<String>)>>,
    ) -> Option<LanguageId> {
        // `Path.extension()` returns None for files with a leading '.'
        // and no other extension which is not the desired behavior here,
        // as we want `.zshrc` to result in extension being `Some("zshrc")`
        let extension = filename.and_then(|filename| filename.split('.').next_back());
        let path_suffixes = [extension, filename]
            .into_iter()
            .chain(paths.iter().map(|path| path.to_str()))
            .filter_map(|suffix| suffix.map(|suffix| (suffix, globset::Candidate::new(suffix))))
            .collect::<SmallVec<[_; 6]>>();
        let default_scores =
            self.lookup
                .suffix_scores(&self.languages, filename, paths, &path_suffixes);
        let custom_scores = user_file_types
            .into_iter()
            .flatten()
            .filter_map(|(language_name, (custom_suffixes, _))| {
                let language_indices = self.lookup.languages_by_name.get(language_name.as_ref())?;
                let len = path_suffixes
                    .iter()
                    .find(|(_, candidate)| custom_suffixes.is_match_candidate(candidate))
                    .map(|(suffix, _)| suffix.len())?;
                Some(
                    language_indices
                        .iter()
                        .map(move |language_index| (*language_index, len)),
                )
            })
            .flatten()
            .collect::<SmallVec<[_; 2]>>();

        let mut candidates = default_scores
            .iter()
            .chain(&custom_scores)
            .map(|(language_index, _)| *language_index)
            .collect::<SmallVec<[_; 8]>>();
        // Content is only consulted while no match has been found yet, so it can only
        // affect the result for languages registered after every path match.
        let highest_path_candidate = candidates.iter().max().copied();
        let content_matches = if let Some(content) = content
            && self
                .lookup
                .first_line_pattern_languages
                .last()
                .is_some_and(|language_index| {
                    highest_path_candidate.is_none_or(|highest| *language_index > highest)
                }) {
            let end = content.clip_point(Point::new(0, 256), Bias::Left);
            let end = content.point_to_offset(end);
            let first_line = content.chunks_in_range(0..end).collect::<String>();
            self.lookup.first_line_matches(&first_line)
        } else {
            SmallVec::new()
        };
        candidates.extend(content_matches.iter().copied());
        candidates.sort_unstable_by(|a, b| b.cmp(a));
        candidates.dedup();

        self.find_best_match_among(
            candidates,
            |language_index, _language, current_best_match| {
                let path_matches_default_suffix = || {
                    default_scores
                        .binary_search_by_key(&language_index, |(language_index, _)| {
                            *language_index
                        })
                        .ok()
                        .and_then(|position| default_scores.get(position))
                        .map(|(_, len)| *len)
                };

                let path_matches_custom_suffix = || {
                    custom_scores
                        .iter()
                        .find(|(custom_language_index, _)| *custom_language_index == language_index)
                        .map(|(_, len)| *len)
                };

                let content_matches = || content_matches.contains(&language_index);

                // Only return a match for the given file if we have a better match than
                // the current one.
                match current_best_match {
                    LanguageMatchPrecedence::PathOrContent(current_len) => {
                        if let Some(len) = path_matches_custom_suffix() {
                            // >= because user config should win tie with system ext len
                            (len >= current_len)
                                .then_some(LanguageMatchPrecedence::UserConfigured(len))
                        } else if let Some(len) = path_matches_default_suffix() {
                            // >= because user config should win tie with system ext len
                            (len >= current_len)
                                .then_some(LanguageMatchPrecedence::PathOrContent(len))
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
            },
        )
    }

    fn find_best_match(
        &self,
        callback: impl Fn(
            &LanguageName,
            &LanguageMatcher,
            LanguageMatchPrecedence,
        ) -> Option<LanguageMatchPrecedence>,
    ) -> Option<LanguageId> {
        self.find_best_match_among(
            (0..self.languages.len()).rev().map(LanguageIndex),
            |_, language, precedence| callback(&language.name, &language.matcher, precedence),
        )
    }

    /// Folds over the languages at `candidates`, which must be in descending order.
    /// Languages that are not candidates must be ones for which `callback` would return `None`.
    fn find_best_match_among(
        &self,
        candidates: impl IntoIterator<Item = LanguageIndex>,
        callback: impl Fn(
            LanguageIndex,
            &AvailableLanguage,
            LanguageMatchPrecedence,
        ) -> Option<LanguageMatchPrecedence>,
    ) -> Option<LanguageId> {
        candidates
            .into_iter()
            .filter_map(|language_index| Some((language_index, self.language_at(language_index)?)))
            .fold(None, |best_language_match, (language_index, language)| {
                let current_match_type = best_language_match
                    .as_ref()
                    .map_or(LanguageMatchPrecedence::default(), |(_, score)| *score);
                let language_score = callback(language_index, language, current_match_type);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn language(name: &str, suffixes: &[&str]) -> AvailableLanguage {
        AvailableLanguage {
            id: LanguageId::new(),
            name: LanguageName::new(name),
            grammar: None,
            matcher: Arc::new(LanguageMatcher {
                path_suffixes: suffixes.iter().map(|suffix| (*suffix).into()).collect(),
                ..Default::default()
            }),
            hidden: false,
            load: Arc::new(|| Box::pin(async { anyhow::bail!("detection fixture has no loader") })),
            loaded: false,
            origin: LanguageOrigin::Extension,
        }
    }

    /// Mirrors how `LanguageRegistry::language_for_file_path` queries a single path.
    fn find_for_path(
        languages: &AvailableLanguages,
        path: &Path,
        content: Option<&Rope>,
    ) -> Option<LanguageId> {
        languages.find_for_file(
            path.file_name().and_then(|filename| filename.to_str()),
            &[path],
            content,
            None,
        )
    }

    fn detected(languages: &AvailableLanguages, path: &Path) -> Option<LanguageName> {
        find_for_path(languages, path, None)
            .and_then(|id| languages.get_language(id))
            .map(AvailableLanguage::name)
    }

    #[test]
    fn literal_suffixes_do_not_become_globs() {
        for suffix in ["a*b", "a?b", "[ab]", "{a,b}", r"a\b", "é", "", "a\nb"] {
            let mut languages = AvailableLanguages::default();
            languages.add(language("Literal", &[suffix]));
            for path in [suffix.to_owned(), format!("prefix.{suffix}")] {
                if !path.is_empty() {
                    assert_eq!(
                        detected(&languages, Path::new(&path)),
                        (!suffix.is_empty()).then(|| "Literal".into()),
                        "{path:?}"
                    );
                }
            }
            assert_eq!(
                detected(&languages, Path::new("prefix.axb")),
                None,
                "{suffix:?}"
            );
        }
    }

    #[test]
    fn path_filter_preserves_literal_matching_and_noncanonical_paths() {
        for (suffix, path, matches) in [
            ("gz", "archive.gz/", true),
            ("gz", "archive.gz/.", true),
            ("gz/", "archive.gz/", true),
            ("", "foo/..", true),
            ("", "..", true),
            (".", "foo/..", true),
            (".", "..", true),
            (".", "foo..", true),
            ("", "file.", false),
            ("", "", false),
            ("", "/", false),
            ("/", "/", true),
            ("c", "x.rc", false),
            ("js", "x.cjs", false),
        ] {
            let mut languages = AvailableLanguages::default();
            languages.add(language("Literal", &[suffix]));
            assert_eq!(
                detected(&languages, Path::new(path)),
                matches.then(|| "Literal".into()),
                "suffix {suffix:?}, path {path:?}",
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn path_filter_verifies_literal_windows_separators() {
        let mut languages = AvailableLanguages::default();
        languages.add(language("Backslash", &[r"directory\file.rs"]));
        for (path, matches) in [
            (r"C:\long.directory\file.rs", true),
            (r"C:\long.directory/file.rs", false),
            (r"C:/long.directory\file.rs", true),
        ] {
            assert_eq!(
                detected(&languages, Path::new(path)),
                matches.then(|| "Backslash".into()),
                "{path:?}",
            );
        }
    }

    #[test]
    fn suffix_scores_use_first_candidate_then_maximum_per_language() {
        let mut languages = AvailableLanguages::default();
        languages.update(
            &[],
            vec![
                language("Compound", &["gz", "tar.gz"]),
                language("Simple", &["gz"]),
                language("Compound Tie", &["tar.gz"]),
            ],
        );
        assert_eq!(
            detected(&languages, Path::new("/long/directory/archive.tar.gz")),
            Some("Compound Tie".into())
        );
        // A longer filename candidate beats the extension, but equal scores keep reverse order.
        assert_eq!(
            detected(&languages, Path::new("/long/directory/archive.gz")),
            Some("Simple".into())
        );
        languages.add(language("Full Path", &["directory/archive.tar.gz"]));
        assert_eq!(
            detected(&languages, Path::new("/long.directory/archive.tar.gz")),
            Some("Full Path".into())
        );
        languages.lookup.suffix_globs = None;
        assert_eq!(
            detected(&languages, Path::new("/long.directory/archive.tar.gz")),
            Some("Full Path".into())
        );
    }

    #[test]
    fn populated_index_tracks_replacement_and_removal() {
        let mut languages = AvailableLanguages::default();
        let old = language("Replace", &["old"]);
        let old_id = old.id;
        languages.add(old);
        assert_eq!(
            find_for_path(&languages, Path::new("file.old"), None),
            Some(old_id)
        );
        let replacement = language("Replace", &["new"]);
        let new_id = replacement.id;
        languages.mark_loaded(old_id);
        let update = languages.update(&[], vec![replacement]);
        assert!(update.removed.is_empty());
        assert_eq!(update.reloaded.as_slice(), [LanguageName::new("Replace")]);
        assert_eq!(
            update.registrations.as_slice(),
            [LanguageRegistrationStatus::Registered]
        );
        assert_ne!(new_id, old_id);
        assert_eq!(
            find_for_path(&languages, Path::new("file.new"), None),
            Some(new_id),
        );
        assert_eq!(detected(&languages, Path::new("file.old")), None);
        assert_eq!(
            detected(&languages, Path::new("file.new")),
            Some("Replace".into())
        );
        languages.update(&["Replace".into()], vec![language("Other", &["other"])]);
        assert_eq!(detected(&languages, Path::new("file.new")), None);
        assert_eq!(
            detected(&languages, Path::new("file.other")),
            Some("Other".into())
        );
    }

    #[test]
    fn first_line_patterns_only_apply_without_a_better_path_match() {
        let with_first_line = |name: &str, suffixes: &[&str], pattern: &str| AvailableLanguage {
            matcher: Arc::new(LanguageMatcher {
                path_suffixes: suffixes.iter().map(|suffix| (*suffix).into()).collect(),
                first_line_pattern: Some(crate::FirstLinePattern::new(pattern).unwrap()),
                ..Default::default()
            }),
            ..language(name, &[])
        };
        let mut languages = AvailableLanguages::default();
        languages.update(
            &[],
            vec![
                with_first_line("Shell", &["sh"], r"^#!.*\bbash\b"),
                with_first_line("Python", &[], r"^#!.*\bpython"),
                language("Text", &["txt"]),
            ],
        );
        let detected_with_content = |path: &str, content: &str| {
            find_for_path(&languages, Path::new(path), Some(&Rope::from(content)))
                .and_then(|id| languages.get_language(id))
                .map(AvailableLanguage::name)
        };

        let python = "#!/usr/bin/env python\nprint()\n";
        assert_eq!(
            detected_with_content("script", python),
            Some("Python".into())
        );
        assert_eq!(
            detected_with_content("script", "#!/bin/bash\n"),
            Some("Shell".into())
        );
        assert_eq!(detected_with_content("script", "plain\n"), None);
        assert_eq!(
            detected_with_content("script.sh", python),
            Some("Shell".into())
        );
        assert_eq!(
            detected_with_content("script.txt", python),
            Some("Text".into())
        );
    }

    #[test]
    fn modeline_names_prefer_aliases_then_grammars_then_names() {
        let mut alpha = language("Alpha", &[]);
        alpha.grammar = Some("alpha_grammar".into());
        alpha.matcher = Arc::new(LanguageMatcher {
            modeline_aliases: vec!["Shared".into()],
            ..Default::default()
        });
        let mut beta = language("Beta", &[]);
        beta.matcher = Arc::new(LanguageMatcher {
            modeline_aliases: vec!["shared".into()],
            ..Default::default()
        });
        let mut languages = AvailableLanguages::default();
        languages.update(&[], vec![language("Shared", &[]), alpha, beta]);

        let modeline_language = |name: &str| {
            languages
                .find_by_modeline_name(name)
                .map(|language| language.name())
        };
        assert_eq!(modeline_language("SHARED"), Some("Alpha".into()));
        assert_eq!(modeline_language("Alpha_Grammar"), Some("Alpha".into()));
        assert_eq!(modeline_language("beta"), Some("Beta".into()));
        assert_eq!(modeline_language("missing"), None);
    }

    #[test]
    fn suffixes_match_against_every_path() {
        let mut languages = AvailableLanguages::default();
        languages.update(
            &[],
            vec![
                language("Text", &["txt"]),
                language("Ssh Config", &["ssh/config"]),
            ],
        );
        let detected_for = |filename: &str, paths: &[&str]| {
            let paths = paths.iter().map(Path::new).collect::<Vec<_>>();
            languages
                .find_for_file(Some(filename), &paths, None, None)
                .and_then(|id| languages.get_language(id))
                .map(AvailableLanguage::name)
        };

        // A single-file worktree's full path is only the filename, while the absolute
        // path carries the directories a suffix may need.
        assert_eq!(
            detected_for("config", &["config", "/home/user/.ssh/config"]),
            Some("Ssh Config".into())
        );
        assert_eq!(detected_for("config", &["config"]), None);
        // A filename that differs from every path's file name is matched, too.
        assert_eq!(
            detected_for("notes.txt", &["worktree"]),
            Some("Text".into())
        );
        assert_eq!(detected_for("notes.txt", &[]), Some("Text".into()));
    }

    #[cfg(unix)]
    #[test]
    fn non_utf8_parent_does_not_hide_utf8_filename() {
        use std::os::unix::ffi::OsStringExt;
        let mut languages = AvailableLanguages::default();
        languages.add(language("Text", &["txt"]));
        let path =
            std::path::PathBuf::from(std::ffi::OsString::from_vec(b"bad\xff/file.txt".to_vec()));
        assert_eq!(detected(&languages, &path), Some("Text".into()));
    }
}
