use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use serde_yaml::Value;
use std::{ops::Range, path::Path};

pub enum PermissionsError {
    /// No top-level `permissions:` key.
    Missing,
    /// Top-level default grants more than `contents: read`.
    ExcessiveDefault,
}

/// Validates a workflow's top-level (default) `permissions:`.
///
/// Every workflow must declare one, and it may grant at most `contents: read`;
/// jobs that need more must request it at the job level.
pub fn validate_permissions(workflow: &Value) -> Result<(), PermissionsError> {
    let Some(permissions) = workflow.get("permissions") else {
        return Err(PermissionsError::Missing);
    };

    let is_minimal = match permissions {
        Value::Mapping(mapping) => mapping.iter().all(|(scope, level)| {
            let level = level.as_str();
            level == Some("none") || (scope.as_str() == Some("contents") && level == Some("read"))
        }),
        // String forms such as `read-all`/`write-all` always exceed the allowance.
        _ => false,
    };

    if is_minimal {
        Ok(())
    } else {
        Err(PermissionsError::ExcessiveDefault)
    }
}

impl PermissionsError {
    pub fn annotation_group<'a>(&self, file_path: &Path, raw_content: &'a str) -> Group<'a> {
        let (title, span, label) = match self {
            PermissionsError::Missing => (
                "Workflow is missing a top-level `permissions:` key",
                first_line_span(raw_content, "name:"),
                "Add a top-level `permissions:` key so this workflow defaults to the least privilege it needs",
            ),
            PermissionsError::ExcessiveDefault => (
                "Top-level workflow permissions must grant at most `contents: read`",
                first_line_span(raw_content, "permissions:"),
                "Lower the default to `contents: read` (or `{}`) and move elevated permissions to the jobs that need them",
            ),
        };

        Level::ERROR.primary_title(title).element(
            Snippet::source(raw_content)
                .path(file_path.display().to_string())
                .annotations(std::iter::once(
                    AnnotationKind::Primary.span(span).label(label),
                )),
        )
    }
}

/// Span of the first column-0 line starting with `prefix`, else empty.
fn first_line_span(raw_content: &str, prefix: &str) -> Range<usize> {
    let mut offset = 0;
    for line in raw_content.lines() {
        let start = offset;
        offset += line.len() + 1;
        if line.starts_with(prefix) {
            return start..start + line.len();
        }
    }

    Default::default()
}
