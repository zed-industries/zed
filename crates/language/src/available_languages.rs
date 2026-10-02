use crate::{LanguageId, LanguageLoader, LanguageMatcher, LanguageName};
use collections::FxHashMap;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
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
    suffix_index: SuffixIndex,
}

#[derive(Default)]
struct SuffixIndex {
    globs: Option<GlobSet>,
    suffixes: Vec<(String, usize)>,
}

impl SuffixIndex {
    fn new(languages: &[AvailableLanguage]) -> Self {
        let mut builder = GlobSetBuilder::new();
        let mut suffixes = Vec::new();
        for (language_index, language) in languages.iter().enumerate() {
            for suffix in &language.matcher.path_suffixes {
                let mut pattern = String::from("*");
                for character in suffix.chars() {
                    let character = if cfg!(windows) && character == '\\' {
                        '/'
                    } else {
                        character
                    };
                    if matches!(character, '\\' | '*' | '?' | '[' | ']' | '{' | '}') {
                        pattern.push('\\');
                    }
                    pattern.push(character);
                }
                builder.add(
                    GlobBuilder::new(&pattern)
                        .backslash_escape(true)
                        .literal_separator(false)
                        .build()
                        .expect("escaped literal suffix must be a valid glob"),
                );
                suffixes.push((suffix.clone(), language_index));
            }
        }
        let globs = match builder.build() {
            Ok(globs) => Some(globs),
            Err(error) => {
                log::error!("failed to compile language suffix index: {error}");
                None
            }
        };
        Self { globs, suffixes }
    }

    fn literal_scores(
        &self,
        candidates: &[(&str, globset::Candidate<'_>)],
        language_count: usize,
    ) -> Vec<usize> {
        let mut scores = vec![0; language_count];
        for (suffix, language_index) in &self.suffixes {
            scores[*language_index] = scores[*language_index].max(suffix_score(suffix, candidates));
        }
        scores
    }

    fn scores_for_path(
        &self,
        path: &Path,
        candidates: &[(&str, globset::Candidate<'_>)],
        language_count: usize,
    ) -> Vec<usize> {
        // On ordinary paths all three legacy candidates end at the end of the full
        // path. Non-UTF8 parents and trailing separators or `/.` need an exact scan:
        // Path::file_name and globset do not agree about these paths' basenames.
        let full_path_candidate = path
            .to_str()
            .zip(path.file_name().and_then(|name| name.to_str()))
            .filter(|(path, filename)| path.ends_with(filename))
            .and_then(|_| candidates.last());
        let (Some(globs), Some((_, candidate))) = (&self.globs, full_path_candidate) else {
            return self.literal_scores(candidates, language_count);
        };

        let mut scores = vec![0; language_count];
        let mut matches = Vec::new();
        globs.matches_candidate_into(candidate, &mut matches);
        for pattern_index in matches {
            let (suffix, language_index) = &self.suffixes[pattern_index];
            // The glob is only a suffix filter. Verify the dot boundary and recover
            // the original score, including literal Windows separators.
            scores[*language_index] = scores[*language_index].max(suffix_score(suffix, candidates));
        }
        scores
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

impl AvailableLanguages {
    pub(super) fn update(
        &mut self,
        names_to_remove: &[LanguageName],
        registrations: Vec<AvailableLanguage>,
    ) -> (Vec<LanguageName>, Vec<bool>) {
        if names_to_remove.is_empty() && registrations.is_empty() {
            return (Vec::new(), Vec::new());
        }

        let mut languages = self.languages.clone();
        let mut invalidated = Vec::new();
        languages.retain(|language| {
            let remove = language.origin == LanguageOrigin::Extension
                && names_to_remove.contains(&language.name);
            if remove {
                invalidated.push(language.name.clone());
            }
            !remove
        });
        let mut registered = Vec::with_capacity(registrations.len());
        for registration in registrations {
            if let Some(existing) = languages
                .iter_mut()
                .find(|language| language.name == registration.name)
            {
                if registration.origin == LanguageOrigin::Extension
                    && existing.origin == LanguageOrigin::Native
                {
                    log::warn!(
                        "not registering extension language {}: a language with this name is already registered outside of extensions",
                        registration.name
                    );
                    registered.push(false);
                    continue;
                }
                if existing.loaded {
                    invalidated.push(existing.name.clone());
                }
                *existing = registration;
            } else {
                languages.push(registration);
            }
            registered.push(true);
        }
        if !invalidated.is_empty() || registered.iter().any(|registered| *registered) {
            self.replace_languages(languages);
        }
        (invalidated, registered)
    }

    pub(super) fn add(&mut self, language: AvailableLanguage) {
        let mut languages = self.languages.clone();
        languages.push(language);
        self.replace_languages(languages);
    }

    fn replace_languages(&mut self, languages: Vec<AvailableLanguage>) {
        // Build before publishing so lookups always see registrations and their matching index.
        let suffix_index = SuffixIndex::new(&languages);
        *self = Self {
            languages,
            suffix_index,
        };
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
        let modeline_name = modeline_name.to_lowercase();
        self.languages
            .iter()
            .find(|language| {
                language
                    .matcher
                    .modeline_aliases
                    .iter()
                    .any(|alias| alias.to_lowercase() == modeline_name)
            })
            .or_else(|| {
                self.languages.iter().find(|language| {
                    language
                        .grammar
                        .as_ref()
                        .is_some_and(|grammar| grammar.to_lowercase() == modeline_name)
                })
            })
            .or_else(|| {
                self.languages
                    .iter()
                    .find(|language| language.name.0.to_lowercase() == modeline_name)
            })
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
        let scores = LazyCell::new(|| {
            self.suffix_index
                .scores_for_path(path, &path_suffixes, self.languages.len())
        });
        let content = LazyCell::new(|| {
            content.map(|content| {
                let end = content.clip_point(Point::new(0, 256), Bias::Left);
                let end = content.point_to_offset(end);
                content.chunks_in_range(0..end).collect::<String>()
            })
        });

        self.find_best_match_indexed(
            |language_index, language_name, matcher, current_best_match| {
                let path_matches_default_suffix = || {
                    let len = scores[language_index];
                    (len > 0).then_some(len)
                };

                let path_matches_custom_suffix = || {
                    user_file_types
                        .and_then(|types| types.get(language_name.as_ref()))
                        .and_then(|(custom_suffixes, _)| {
                            path_suffixes
                                .iter()
                                .find(|(_, candidate)| {
                                    custom_suffixes.is_match_candidate(candidate)
                                })
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
        self.find_best_match_indexed(|_, name, matcher, precedence| {
            callback(name, matcher, precedence)
        })
    }

    fn find_best_match_indexed(
        &self,
        callback: impl Fn(
            usize,
            &LanguageName,
            &LanguageMatcher,
            LanguageMatchPrecedence,
        ) -> Option<LanguageMatchPrecedence>,
    ) -> Option<LanguageId> {
        self.languages
            .iter()
            .enumerate()
            .rev()
            .fold(None, |best_language_match, (language_index, language)| {
                let current_match_type = best_language_match
                    .as_ref()
                    .map_or(LanguageMatchPrecedence::default(), |(_, score)| *score);
                let language_score = callback(
                    language_index,
                    &language.name,
                    &language.matcher,
                    current_match_type,
                );

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
                path_suffixes: suffixes.iter().map(|suffix| suffix.to_string()).collect(),
                ..Default::default()
            }),
            hidden: false,
            load: Arc::new(|| Box::pin(async { anyhow::bail!("detection fixture has no loader") })),
            loaded: false,
            origin: LanguageOrigin::Extension,
        }
    }

    fn detected(languages: &AvailableLanguages, path: &Path) -> Option<LanguageName> {
        languages
            .find_for_file(path, None, None)
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
        languages.suffix_index.globs = None;
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
            languages.find_for_file(Path::new("file.old"), None, None),
            Some(old_id)
        );
        let replacement = language("Replace", &["new"]);
        let new_id = replacement.id;
        languages.mark_loaded(old_id);
        let (invalidated, registered) = languages.update(&[], vec![replacement]);
        assert_eq!(invalidated, vec![LanguageName::new("Replace")]);
        assert_eq!(registered, vec![true]);
        assert_ne!(new_id, old_id);
        assert_eq!(
            languages.find_for_file(Path::new("file.new"), None, None),
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
