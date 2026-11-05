use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use regex::Regex;
use std::{collections::HashMap, ops::Range, path::Path, sync::LazyLock};

static GITHUB_INPUT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\$\{\{[[:blank:]]*([[:alnum:]]|[[:punct:]])+?[[:blank:]]*\}\}"#)
        .expect("Should compile")
});

pub struct RunValidationError {
    found_injection_patterns: Vec<(String, Range<usize>)>,
}

impl RunValidationError {
    /// Renders the GitHub input injection patterns found in a single `run:`
    /// command as one diagnostic group.
    pub fn annotation_group<'a>(&self, file_path: &Path, raw_content: &'a str) -> Group<'a> {
        let mut identical_lines = HashMap::new();

        let ranges = self
            .found_injection_patterns
            .iter()
            .map(|(line, pattern_range)| {
                let initial_offset = identical_lines
                    .get(&(line.as_str(), pattern_range.start))
                    .copied()
                    .unwrap_or_default();

                let line_start = raw_content[initial_offset..]
                    .find(line.as_str())
                    .map(|offset| offset + initial_offset)
                    .unwrap_or_default();

                let pattern_start = line_start + pattern_range.start;
                let pattern_end = pattern_start + pattern_range.len();

                identical_lines.insert((line.as_str(), pattern_range.start), pattern_end);

                pattern_start..pattern_end
            });

        Level::ERROR
            .primary_title("Found GitHub input injection in run command")
            .element(
                Snippet::source(raw_content)
                    .path(file_path.display().to_string())
                    .annotations(ranges.map(|range| {
                        AnnotationKind::Primary
                            .span(range)
                            .label("This should be passed via an environment variable")
                    })),
            )
    }
}

pub fn validate_run_command(command: &str) -> Result<(), RunValidationError> {
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
        Err(RunValidationError {
            found_injection_patterns: patterns,
        })
    }
}
