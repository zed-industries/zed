use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::ops::Range;
use std::path::PathBuf;

/// A warning about a blocking call detected in an async context.
#[derive(Debug, Clone)]
pub struct Warning {
    /// Path to the source file.
    pub path: PathBuf,
    /// Byte offset range of the call expression within the source file.
    pub span: Range<usize>,
    /// The resolved call path that matched the blocklist.
    pub call_path: String,
    /// Category from the blocklist entry (e.g., "filesystem").
    pub category: String,
    /// Help text from the blocklist entry.
    pub help: String,
    /// Description of the async context (e.g., "async fn `load_file`").
    pub context: String,
}

/// Render all warnings for a single file using codespan-reporting.
pub fn emit_file_warnings(warnings: &[Warning], source: &str, path: &str, format: &str) {
    if warnings.is_empty() {
        return;
    }
    match format {
        "json" => {
            for warning in warnings {
                emit_json(warning);
            }
        }
        _ => emit_human_batch(warnings, source, path),
    }
}

fn emit_human_batch(warnings: &[Warning], source: &str, path: &str) {
    let mut files = SimpleFiles::new();
    let file_id = files.add(path, source);

    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = term::Config::default();

    for warning in warnings {
        let diagnostic = Diagnostic::new(Severity::Warning)
            .with_code("blocking-in-async")
            .with_message("blocking call in async context")
            .with_labels(vec![Label::new(
                LabelStyle::Primary,
                file_id,
                warning.span.clone(),
            )
            .with_message(format!(
                "`{}` is a blocking {} operation",
                warning.call_path, warning.category
            ))])
            .with_notes(vec![
                format!("help: {}", warning.help),
                format!("context: {}", warning.context),
            ]);

        term::emit_to_write_style(&mut writer.lock(), &config, &files, &diagnostic)
            .expect("failed to write diagnostic");
    }
}

fn emit_json(warning: &Warning) {
    println!(
        r#"{{"level":"warning","code":"blocking-in-async","path":"{}","byte_start":{},"byte_end":{},"call":"{}","category":"{}","help":"{}","context":"{}"}}"#,
        warning.path.display(),
        warning.span.start,
        warning.span.end,
        warning.call_path,
        warning.category,
        warning.help,
        warning.context,
    );
}
