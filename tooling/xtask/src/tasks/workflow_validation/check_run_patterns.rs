use std::{ops::Range, path::Path, sync::LazyLock};

use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use regex::Regex;

static GITHUB_INPUT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\$\{\{[[:blank:]]*([[:alnum:]]|[[:punct:]])+?[[:blank:]]*\}\}"#)
        .expect("Should compile")
});

pub struct InvalidPatternsErrror {
    pub command: String,
    pub patterns: Vec<Range<usize>>,
}

pub fn annotations_for_indices<'a>(
    patterns: impl IntoIterator<Item = Range<usize>>,
    source: &'a str,
    file: &Path,
) -> Group<'a> {
    Level::ERROR
        .primary_title("Found GitHub input injection in rum command")
        .element(
            Snippet::source(source)
                .path(file.display().to_string())
                .annotations(patterns.into_iter().map(|range| {
                    AnnotationKind::Primary
                        .span(range)
                        .label("This should be passed via environment variables")
                })),
        )
}

pub fn validate_run_command(command: &str) -> Result<(), InvalidPatternsErrror> {
    let patterns: Vec<_> = GITHUB_INPUT_PATTERN
        .find_iter(command)
        .map(|m| m.range())
        .collect();

    if patterns.is_empty() {
        Ok(())
    } else {
        Err(InvalidPatternsErrror {
            command: command.to_owned(),
            patterns,
        })
    }
}
