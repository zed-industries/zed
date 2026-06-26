use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use serde_yaml::Value;
use std::{ops::Range, path::Path};

pub struct MissingPermissionsError;

/// Ensures every workflow declares a top-level `permissions:` key.
///
/// Individual jobs are allowed to narrow their permissions further, but a
/// top-level default must always be present so that workflows default to the
/// least privilege they need rather than inheriting the repository default.
pub fn validate_permissions(workflow: &Value) -> Result<(), MissingPermissionsError> {
    if workflow.get("permissions").is_some() {
        Ok(())
    } else {
        Err(MissingPermissionsError)
    }
}

impl MissingPermissionsError {
    pub fn annotation_group<'a>(&self, file_path: &Path, raw_content: &'a str) -> Group<'a> {
        Level::ERROR
            .primary_title("Workflow is missing a top-level `permissions:` key")
            .element(
                Snippet::source(raw_content)
                    .path(file_path.display().to_string())
                    .annotations(std::iter::once(
                        AnnotationKind::Primary
                            .span(top_level_span(raw_content))
                            .label(
                                "Add a top-level `permissions:` key so this workflow \
                                defaults to the least privilege it needs",
                            ),
                    )),
            )
    }
}

/// Picks a span to anchor the missing-permissions diagnostic on. Prefers the
/// `name:` line of the workflow, falling back to the start of the file.
fn top_level_span(raw_content: &str) -> Range<usize> {
    let mut offset = 0;
    for line in raw_content.lines() {
        let start = offset;
        offset += line.len() + 1;

        let trimmed = line.trim_start();
        if trimmed.starts_with("name:") {
            return start..start + line.len();
        }
    }

    Default::default()
}
