use anyhow::Result;
use gpui::{AppContext, Context, Entity, Task};
use language::{Buffer, BufferSnapshot, Language, Location, Point, PointUtf16};
use regex::Regex;
use std::sync::LazyLock;
use util::paths::{PathMatcher, PathStyle};

use crate::{LocationLink, Project, SearchQuery, SearchResult, SearchResults};

static CLASS_ATTRIBUTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"class\s*=\s*["']([^"']*)["']"#).expect("valid class attribute regex")
});

/// Declarative rule for a definition fallback.
///
/// Add a new `const` for each symbol type/language pair, then call
/// `Project::run_definition_fallback` with it from `Project::definitions`.
struct DefinitionFallback {
    languages: &'static [&'static str],
    target_globs: &'static [&'static str],
    exclude_globs: &'static [&'static str],
    extract_token: fn(&BufferSnapshot, PointUtf16) -> Option<String>,
    selector_pattern: fn(&str) -> String,
}

const CSS_CLASS_FALLBACK: DefinitionFallback = DefinitionFallback {
    languages: &["HTML", "Angular", "Astro", "Vue", "Svelte"],
    target_globs: &[
        "**/*.css",
        "**/*.scss",
        "**/*.less",
        "**/*.astro",
        "**/*.html",
    ],
    exclude_globs: &["**/node_modules/**", "**/dist/**", "**/.git/**"],
    extract_token: class_name_at_position,
    selector_pattern: css_selector_pattern,
};

impl Project {
    /// Fallback go-to-definition for `class` attribute values in HTML-like files.
    pub fn css_class_definition(
        &mut self,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<LocationLink>>>> {
        self.run_definition_fallback(&CSS_CLASS_FALLBACK, buffer, position, cx)
    }

    /// Run a declarative fallback rule when the language server returns no definitions.
    fn run_definition_fallback(
        &mut self,
        fallback: &DefinitionFallback,
        buffer: &Entity<Buffer>,
        position: PointUtf16,
        cx: &mut Context<Self>,
    ) -> Task<Result<Option<Vec<LocationLink>>>> {
        let Some(language) = buffer.read(cx).language().cloned() else {
            return Task::ready(Ok(None));
        };

        if !fallback.applies_to(&language) {
            return Task::ready(Ok(None));
        }

        let snapshot = buffer.read(cx).snapshot();
        let Some(token) = (fallback.extract_token)(&snapshot, position) else {
            return Task::ready(Ok(None));
        };

        let pattern = (fallback.selector_pattern)(&token);
        let path_style = PathStyle::local();

        let files_to_include = PathMatcher::new(fallback.target_globs.iter().copied(), path_style)
            .expect("valid globs");
        let files_to_exclude = PathMatcher::new(fallback.exclude_globs.iter().copied(), path_style)
            .expect("valid globs");

        let query = match SearchQuery::regex(
            pattern,
            false,
            true,
            false,
            true,
            files_to_include,
            files_to_exclude,
            false,
            None,
        ) {
            Ok(query) => query,
            Err(err) => return Task::ready(Err(err)),
        };

        let search = self.search(query, cx);
        let source_buffer = buffer.clone();

        cx.background_spawn(async move {
            let mut links = Vec::new();
            let SearchResults { task_handle, rx } = search;
            let _task_handle = task_handle;

            while let Ok(result) = rx.recv().await {
                if let SearchResult::Buffer { buffer, ranges } = result {
                    for range in ranges {
                        links.push(LocationLink {
                            origin: Some(Location {
                                buffer: source_buffer.clone(),
                                range: range.clone(),
                            }),
                            target: Location {
                                buffer: buffer.clone(),
                                range,
                            },
                        });
                    }
                }
            }

            Ok(Some(links).filter(|links| !links.is_empty()))
        })
    }
}

impl DefinitionFallback {
    fn applies_to(&self, language: &Language) -> bool {
        self.languages.contains(&language.name().as_ref())
    }
}

fn css_selector_pattern(token: &str) -> String {
    format!(r"\.{token}\b", token = regex::escape(token))
}

fn class_name_at_position(snapshot: &BufferSnapshot, position: PointUtf16) -> Option<String> {
    let row = position.row;
    let line_start = Point::new(row, 0);
    let line_end = Point::new(row + 1, 0);
    let line = snapshot
        .text_for_range(line_start..line_end)
        .collect::<String>();
    let col = position.column as usize;

    for capture in CLASS_ATTRIBUTE.captures_iter(&line) {
        let value = capture.get(1)?;
        let value_range = value.range();

        if !value_range.contains(&col) {
            continue;
        }

        let value = value.as_str();
        let rel = col - value_range.start;
        let start = value[..rel]
            .rfind(char::is_whitespace)
            .map_or(0, |idx| idx + 1);
        let end = value[rel..]
            .find(char::is_whitespace)
            .map_or(value.len(), |idx| rel + idx);
        let token = value[start..end].to_string();

        return (!token.is_empty()).then_some(token);
    }

    None
}
