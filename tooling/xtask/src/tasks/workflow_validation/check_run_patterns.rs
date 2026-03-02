use std::{ops::Range, path::Path, sync::LazyLock};

use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use regex::Regex;

static GITHUB_INPUT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\$\{\{[[:blank:]]*([[:alnum:]]|[[:punct:]])+?[[:blank:]]*\}\}"#)
        .expect("Should compile")
});

pub struct InvalidPatternsErrror {
    pub patterns: Vec<(String, Range<usize>)>,
}

pub fn annotations_for_indices<'a>(
    patterns: impl IntoIterator<Item = Range<usize>>,
    source: &'a str,
    file: &Path,
) -> Group<'a> {
    Level::ERROR
        .primary_title("Found GitHub input injection in run command")
        .element(
            Snippet::source(source)
                .path(file.display().to_string())
                .annotations(patterns.into_iter().map(|range| {
                    AnnotationKind::Primary
                        .span(range)
                        .label("This should be passed via an environment variable")
                })),
        )
}

pub fn validate_run_command(command: &str) -> Result<(), InvalidPatternsErrror> {
    let patterns: Vec<_> = command
        .lines()
        .flat_map(move |line| {
            GITHUB_INPUT_PATTERN
                .find_iter(line)
                .map(|m| (line.to_owned(), m.range()))
        })
        .collect();

    if patterns.is_empty() {
        Ok(())
    } else {
        Err(InvalidPatternsErrror { patterns })
    }
}
