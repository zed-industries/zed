use std::{
    path::{Component, Path, Prefix},
    process::Stdio,
};

use cargo_metadata::{
    Message,
    diagnostic::{Applicability, Diagnostic as CargoDiagnostic, DiagnosticLevel, DiagnosticSpan},
};
use collections::HashMap;
use gpui::{AppContext, Entity, Task};
use itertools::Itertools as _;
use project::{Worktree, project_settings::ProjectSettings};
use serde::Deserialize as _;
use settings::Settings;
use smol::{
    channel::Receiver,
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};
use ui::App;
use util::ResultExt;

use crate::ProjectDiagnosticsEditor;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum CargoMessage {
    Cargo(Message),
    Rustc(CargoDiagnostic),
}

/// Appends formatted string to a `String`.
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        {
            use ::std::fmt::Write as _;
            // We can't do ::std::fmt::Write::write_fmt($buf, format_args!($lit $($arg)*))
            // unfortunately, as that loses out on autoref behavior.
            _ = $buf.write_fmt(format_args!($lit $($arg)*))
        }
    };
}

pub fn cargo_diagnostics_sources(
    editor: &ProjectDiagnosticsEditor,
    cx: &App,
) -> Vec<Entity<Worktree>> {
    let fetch_cargo_diagnostics = ProjectSettings::get_global(cx)
        .diagnostics
        .fetch_cargo_diagnostics();
    if !fetch_cargo_diagnostics {
        return Vec::new();
    }
    editor
        .project
        .read(cx)
        .worktrees(cx)
        .filter(|worktree| worktree.read(cx).entry_for_path("Cargo.toml").is_some())
        .collect()
}

pub fn fetch_worktree_diagnostics(
    worktree_root: &Path,
    cx: &App,
) -> Option<(Task<()>, Receiver<CargoDiagnostic>)> {
    let diagnostics_settings = ProjectSettings::get_global(cx)
        .diagnostics
        .cargo
        .as_ref()
        .filter(|settings| settings.fetch_cargo_diagnostics)?;
    let command_string = diagnostics_settings
        .diagnostics_fetch_command
        .iter()
        .join(" ");
    let mut command_parts = diagnostics_settings.diagnostics_fetch_command.iter();
    let mut command = Command::new(command_parts.next()?)
        .args(command_parts)
        .envs(diagnostics_settings.env.clone())
        .current_dir(worktree_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .log_err()?;

    let stdout = command.stdout.take()?;
    let mut reader = BufReader::new(stdout);
    let (tx, rx) = smol::channel::unbounded();
    let error_threshold = 10;

    let cargo_diagnostics_fetch_task = cx.background_spawn(async move {
        let mut errors = 0;
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    return;
                },
                Ok(_) => {
                    errors = 0;
                    let mut deserializer = serde_json::Deserializer::from_str(&line);
                    deserializer.disable_recursion_limit();
                    match CargoMessage::deserialize(&mut deserializer).map(|cargo_message| {
                        match cargo_message {
                            CargoMessage::Cargo(message) => match message {
                                Message::CompilerMessage(msg) => {
                                    Some(msg.message)
                                }
                                _ => None,
                            },
                            CargoMessage::Rustc(message) => Some(message),
                        }
                    }) {
                        Ok(Some(message)) => {
                            if tx.send(message).await.is_err() {
                                return;
                            }
                        }
                        Ok(None) => {}
                        Err(_) => log::debug!("Failed to parse cargo diagnostics from line '{line}'"),
                    };
                },
                Err(e) => {
                    log::error!("Failed to read line from {command_string} command output when fetching cargo diagnostics: {e}");
                    errors += 1;
                    if errors >= error_threshold {
                        log::error!("Failed {error_threshold} times, aborting the diagnostics fetch");
                        return;
                    }
                },
            }
        }
    });

    Some((cargo_diagnostics_fetch_task, rx))
}

/// Converts a Rust root diagnostic to LSP form
///
/// This flattens the Rust diagnostic by:
///
/// 1. Creating a LSP diagnostic with the root message and primary span.
/// 2. Adding any labelled secondary spans to `relatedInformation`
/// 3. Categorising child diagnostics as either `SuggestedFix`es,
///    `relatedInformation` or additional message lines.
///
/// If the diagnostic has no primary span this will return `None`
///
/// Taken from https://github.com/rust-lang/rust-analyzer/blob/fe7b4f2ad96f7c13cc571f45edc2c578b35dddb4/crates/rust-analyzer/src/diagnostics/to_proto.rs#L275-L285
pub(crate) fn map_rust_diagnostic_to_lsp(
    worktree_root: &Path,
    cargo_diagnostic: &CargoDiagnostic,
) -> Vec<(lsp::Url, lsp::Diagnostic)> {
    let primary_spans: Vec<&DiagnosticSpan> = cargo_diagnostic
        .spans
        .iter()
        .filter(|s| s.is_primary)
        .collect();
    if primary_spans.is_empty() {
        return Vec::new();
    }

    let severity = diagnostic_severity(cargo_diagnostic.level);

    let mut source = String::from("rustc");
    let mut code = cargo_diagnostic.code.as_ref().map(|c| c.code.clone());

    if let Some(code_val) = &code {
        // See if this is an RFC #2103 scoped lint (e.g. from Clippy)
        let scoped_code: Vec<&str> = code_val.split("::").collect();
        if scoped_code.len() == 2 {
            source = String::from(scoped_code[0]);
            code = Some(String::from(scoped_code[1]));
        }
    }

    let mut needs_primary_span_label = true;
    let mut subdiagnostics = Vec::new();
    let mut tags = Vec::new();

    for secondary_span in cargo_diagnostic.spans.iter().filter(|s| !s.is_primary) {
        if let Some(label) = secondary_span.label.clone() {
            subdiagnostics.push(lsp::DiagnosticRelatedInformation {
                location: location(worktree_root, secondary_span),
                message: label,
            });
        }
    }

    let mut message = cargo_diagnostic.message.clone();
    for child in &cargo_diagnostic.children {
        let child = map_rust_child_diagnostic(worktree_root, child);
        match child {
            MappedRustChildDiagnostic::SubDiagnostic(sub) => {
                subdiagnostics.push(sub);
            }
            MappedRustChildDiagnostic::MessageLine(message_line) => {
                format_to!(message, "\n{message_line}");

                // These secondary messages usually duplicate the content of the
                // primary span label.
                needs_primary_span_label = false;
            }
        }
    }

    if let Some(code) = &cargo_diagnostic.code {
        let code = code.code.as_str();
        if matches!(
            code,
            "dead_code"
                | "unknown_lints"
                | "unreachable_code"
                | "unused_attributes"
                | "unused_imports"
                | "unused_macros"
                | "unused_variables"
        ) {
            tags.push(lsp::DiagnosticTag::UNNECESSARY);
        }

        if matches!(code, "deprecated") {
            tags.push(lsp::DiagnosticTag::DEPRECATED);
        }
    }

    let code_description = match source.as_str() {
        "rustc" => rustc_code_description(code.as_deref()),
        "clippy" => clippy_code_description(code.as_deref()),
        _ => None,
    };

    primary_spans
        .iter()
        .flat_map(|primary_span| {
            let primary_location = primary_location(worktree_root, primary_span);
            let message = {
                let mut message = message.clone();
                if needs_primary_span_label {
                    if let Some(primary_span_label) = &primary_span.label {
                        format_to!(message, "\n{primary_span_label}");
                    }
                }
                message
            };
            // Each primary diagnostic span may result in multiple LSP diagnostics.
            let mut diagnostics = Vec::new();

            let mut related_info_macro_calls = vec![];

            // If error occurs from macro expansion, add related info pointing to
            // where the error originated
            // Also, we would generate an additional diagnostic, so that exact place of macro
            // will be highlighted in the error origin place.
            let span_stack = std::iter::successors(Some(*primary_span), |span| {
                Some(&span.expansion.as_ref()?.span)
            });
            for (i, span) in span_stack.enumerate() {
                if is_dummy_macro_file(&span.file_name) {
                    continue;
                }

                // First span is the original diagnostic, others are macro call locations that
                // generated that code.
                let is_in_macro_call = i != 0;

                let secondary_location = location(worktree_root, span);
                if secondary_location == primary_location {
                    continue;
                }
                related_info_macro_calls.push(lsp::DiagnosticRelatedInformation {
                    location: secondary_location.clone(),
                    message: if is_in_macro_call {
                        "Error originated from macro call here".to_owned()
                    } else {
                        "Actual error occurred here".to_owned()
                    },
                });
                // For the additional in-macro diagnostic we add the inverse message pointing to the error location in code.
                let information_for_additional_diagnostic =
                    vec![lsp::DiagnosticRelatedInformation {
                        location: primary_location.clone(),
                        message: "Exact error occurred here".to_owned(),
                    }];

                let diagnostic = lsp::Diagnostic {
                    range: secondary_location.range,
                    // downgrade to hint if we're pointing at the macro
                    severity: Some(lsp::DiagnosticSeverity::HINT),
                    code: code.clone().map(lsp::NumberOrString::String),
                    code_description: code_description.clone(),
                    source: Some(source.clone()),
                    message: message.clone(),
                    related_information: Some(information_for_additional_diagnostic),
                    tags: if tags.is_empty() {
                        None
                    } else {
                        Some(tags.clone())
                    },
                    data: Some(serde_json::json!({ "rendered": cargo_diagnostic.rendered })),
                };
                diagnostics.push((secondary_location.uri, diagnostic));
            }

            // Emit the primary diagnostic.
            diagnostics.push((
                primary_location.uri.clone(),
                lsp::Diagnostic {
                    range: primary_location.range,
                    severity,
                    code: code.clone().map(lsp::NumberOrString::String),
                    code_description: code_description.clone(),
                    source: Some(source.clone()),
                    message,
                    related_information: {
                        let info = related_info_macro_calls
                            .iter()
                            .cloned()
                            .chain(subdiagnostics.iter().cloned())
                            .collect::<Vec<_>>();
                        if info.is_empty() { None } else { Some(info) }
                    },
                    tags: if tags.is_empty() {
                        None
                    } else {
                        Some(tags.clone())
                    },
                    data: Some(serde_json::json!({ "rendered": cargo_diagnostic.rendered })),
                },
            ));

            // Emit hint-level diagnostics for all `related_information` entries such as "help"s.
            // This is useful because they will show up in the user's editor, unlike
            // `related_information`, which just produces hard-to-read links, at least in VS Code.
            let back_ref = lsp::DiagnosticRelatedInformation {
                location: primary_location,
                message: "original diagnostic".to_owned(),
            };
            for sub in &subdiagnostics {
                diagnostics.push((
                    sub.location.uri.clone(),
                    lsp::Diagnostic {
                        range: sub.location.range,
                        severity: Some(lsp::DiagnosticSeverity::HINT),
                        code: code.clone().map(lsp::NumberOrString::String),
                        code_description: code_description.clone(),
                        source: Some(source.clone()),
                        message: sub.message.clone(),
                        related_information: Some(vec![back_ref.clone()]),
                        tags: None, // don't apply modifiers again
                        data: None,
                    },
                ));
            }

            diagnostics
        })
        .collect()
}

fn rustc_code_description(code: Option<&str>) -> Option<lsp::CodeDescription> {
    code.filter(|code| {
        let mut chars = code.chars();
        chars.next() == Some('E')
            && chars.by_ref().take(4).all(|c| c.is_ascii_digit())
            && chars.next().is_none()
    })
    .and_then(|code| {
        lsp::Url::parse(&format!(
            "https://doc.rust-lang.org/error-index.html#{code}"
        ))
        .ok()
        .map(|href| lsp::CodeDescription { href })
    })
}

fn clippy_code_description(code: Option<&str>) -> Option<lsp::CodeDescription> {
    code.and_then(|code| {
        lsp::Url::parse(&format!(
            "https://rust-lang.github.io/rust-clippy/master/index.html#{code}"
        ))
        .ok()
        .map(|href| lsp::CodeDescription { href })
    })
}

/// Determines the LSP severity from a diagnostic
fn diagnostic_severity(level: DiagnosticLevel) -> Option<lsp::DiagnosticSeverity> {
    let res = match level {
        DiagnosticLevel::Ice => lsp::DiagnosticSeverity::ERROR,
        DiagnosticLevel::Error => lsp::DiagnosticSeverity::ERROR,
        DiagnosticLevel::Warning => lsp::DiagnosticSeverity::WARNING,
        DiagnosticLevel::Note => lsp::DiagnosticSeverity::INFORMATION,
        DiagnosticLevel::Help => lsp::DiagnosticSeverity::HINT,
        _ => return None,
    };
    Some(res)
}

enum MappedRustChildDiagnostic {
    SubDiagnostic(lsp::DiagnosticRelatedInformation),
    MessageLine(String),
}

fn map_rust_child_diagnostic(
    worktree_root: &Path,
    cargo_diagnostic: &CargoDiagnostic,
) -> MappedRustChildDiagnostic {
    let spans: Vec<&DiagnosticSpan> = cargo_diagnostic
        .spans
        .iter()
        .filter(|s| s.is_primary)
        .collect();
    if spans.is_empty() {
        // `rustc` uses these spanless children as a way to print multi-line
        // messages
        return MappedRustChildDiagnostic::MessageLine(cargo_diagnostic.message.clone());
    }

    let mut edit_map: HashMap<lsp::Url, Vec<lsp::TextEdit>> = HashMap::default();
    let mut suggested_replacements = Vec::new();
    for &span in &spans {
        if let Some(suggested_replacement) = &span.suggested_replacement {
            if !suggested_replacement.is_empty() {
                suggested_replacements.push(suggested_replacement);
            }
            let location = location(worktree_root, span);
            let edit = lsp::TextEdit::new(location.range, suggested_replacement.clone());

            // Only actually emit a quickfix if the suggestion is "valid enough".
            // We accept both "MaybeIncorrect" and "MachineApplicable". "MaybeIncorrect" means that
            // the suggestion is *complete* (contains no placeholders where code needs to be
            // inserted), but might not be what the user wants, or might need minor adjustments.
            if matches!(
                span.suggestion_applicability,
                None | Some(Applicability::MaybeIncorrect | Applicability::MachineApplicable)
            ) {
                edit_map.entry(location.uri).or_default().push(edit);
            }
        }
    }

    // rustc renders suggestion diagnostics by appending the suggested replacement, so do the same
    // here, otherwise the diagnostic text is missing useful information.
    let mut message = cargo_diagnostic.message.clone();
    if !suggested_replacements.is_empty() {
        message.push_str(": ");
        let suggestions = suggested_replacements
            .iter()
            .map(|suggestion| format!("`{suggestion}`"))
            .join(", ");
        message.push_str(&suggestions);
    }

    MappedRustChildDiagnostic::SubDiagnostic(lsp::DiagnosticRelatedInformation {
        location: location(worktree_root, spans[0]),
        message,
    })
}

/// Converts a Rust span to a LSP location
fn location(worktree_root: &Path, span: &DiagnosticSpan) -> lsp::Location {
    let file_name = worktree_root.join(&span.file_name);
    let uri = url_from_abs_path(&file_name);

    let range = {
        lsp::Range::new(
            position(span, span.line_start, span.column_start.saturating_sub(1)),
            position(span, span.line_end, span.column_end.saturating_sub(1)),
        )
    };
    lsp::Location::new(uri, range)
}

/// Returns a `Url` object from a given path, will lowercase drive letters if present.
/// This will only happen when processing windows paths.
///
/// When processing non-windows path, this is essentially the same as `Url::from_file_path`.
pub(crate) fn url_from_abs_path(path: &Path) -> lsp::Url {
    let url = lsp::Url::from_file_path(path).unwrap();
    match path.components().next() {
        Some(Component::Prefix(prefix))
            if matches!(prefix.kind(), Prefix::Disk(_) | Prefix::VerbatimDisk(_)) =>
        {
            // Need to lowercase driver letter
        }
        _ => return url,
    }

    let driver_letter_range = {
        let (scheme, drive_letter, _rest) = match url.as_str().splitn(3, ':').collect_tuple() {
            Some(it) => it,
            None => return url,
        };
        let start = scheme.len() + ':'.len_utf8();
        start..(start + drive_letter.len())
    };

    // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
    // machinery *also* canonicalizes the drive letter. So, just massage the
    // string in place.
    let mut url: String = url.into();
    url[driver_letter_range].make_ascii_lowercase();
    lsp::Url::parse(&url).unwrap()
}

fn position(
    span: &DiagnosticSpan,
    line_number: usize,
    column_offset_utf32: usize,
) -> lsp::Position {
    let line_index = line_number - span.line_start;

    let column_offset_encoded = match span.text.get(line_index) {
        // Fast path.
        Some(line) if line.text.is_ascii() => column_offset_utf32,
        Some(line) => {
            let line_prefix_len = line
                .text
                .char_indices()
                .take(column_offset_utf32)
                .last()
                .map(|(pos, c)| pos + c.len_utf8())
                .unwrap_or(0);
            let line_prefix = &line.text[..line_prefix_len];
            line_prefix.len()
        }
        None => column_offset_utf32,
    };

    lsp::Position {
        line: (line_number as u32).saturating_sub(1),
        character: column_offset_encoded as u32,
    }
}

/// Checks whether a file name is from macro invocation and does not refer to an actual file.
fn is_dummy_macro_file(file_name: &str) -> bool {
    // FIXME: current rustc does not seem to emit `<macro file>` files anymore?
    file_name.starts_with('<') && file_name.ends_with('>')
}

/// Extracts a suitable "primary" location from a rustc diagnostic.
///
/// This takes locations pointing into the standard library, or generally outside the current
/// workspace into account and tries to avoid those, in case macros are involved.
fn primary_location(worktree_root: &Path, span: &DiagnosticSpan) -> lsp::Location {
    let span_stack = std::iter::successors(Some(span), |span| Some(&span.expansion.as_ref()?.span));
    for span in span_stack.clone() {
        let abs_path = worktree_root.join(&span.file_name);
        if !is_dummy_macro_file(&span.file_name) && abs_path.starts_with(worktree_root) {
            return location(worktree_root, span);
        }
    }

    // Fall back to the outermost macro invocation if no suitable span comes up.
    let last_span = span_stack.last().unwrap();
    location(worktree_root, last_span)
}
