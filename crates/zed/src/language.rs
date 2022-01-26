pub use language::*;
use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::{str, sync::Arc};

#[derive(RustEmbed)]
#[folder = "languages"]
struct LanguageDir;

struct RustDiagnosticProcessor;

impl DiagnosticProcessor for RustDiagnosticProcessor {
    fn process_diagnostics(&self, params: &mut lsp::PublishDiagnosticsParams) {
        lazy_static! {
            static ref REGEX: Regex = Regex::new("(?m)`([^`]+)\n`$").unwrap();
        }

        for diagnostic in &mut params.diagnostics {
            for message in diagnostic
                .related_information
                .iter_mut()
                .flatten()
                .map(|info| &mut info.message)
                .chain([&mut diagnostic.message])
            {
                if let Cow::Owned(sanitized) = REGEX.replace_all(message, "`$1`") {
                    *message = sanitized;
                }
            }
        }
    }
}

pub fn build_language_registry() -> LanguageRegistry {
    let mut languages = LanguageRegistry::default();
    languages.add(Arc::new(rust()));
    languages.add(Arc::new(markdown()));
    languages
}

fn rust() -> Language {
    let grammar = tree_sitter_rust::language();
    let config = toml::from_slice(&LanguageDir::get("rust/config.toml").unwrap().data).unwrap();
    Language::new(config, Some(grammar))
        .with_highlights_query(load_query("rust/highlights.scm").as_ref())
        .unwrap()
        .with_brackets_query(load_query("rust/brackets.scm").as_ref())
        .unwrap()
        .with_indents_query(load_query("rust/indents.scm").as_ref())
        .unwrap()
        .with_outline_query(load_query("rust/outline.scm").as_ref())
        .unwrap()
        .with_diagnostics_processor(RustDiagnosticProcessor)
}

fn markdown() -> Language {
    let grammar = tree_sitter_markdown::language();
    let config = toml::from_slice(&LanguageDir::get("markdown/config.toml").unwrap().data).unwrap();
    Language::new(config, Some(grammar))
        .with_highlights_query(load_query("markdown/highlights.scm").as_ref())
        .unwrap()
}

fn load_query(path: &str) -> Cow<'static, str> {
    match LanguageDir::get(path).unwrap().data {
        Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
        Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use language::DiagnosticProcessor;

    use super::RustDiagnosticProcessor;

    #[test]
    fn test_process_rust_diagnostics() {
        let mut params = lsp::PublishDiagnosticsParams {
            uri: lsp::Url::from_file_path("/a").unwrap(),
            version: None,
            diagnostics: vec![
                // no newlines
                lsp::Diagnostic {
                    message: "use of moved value `a`".to_string(),
                    ..Default::default()
                },
                // newline at the end of a code span
                lsp::Diagnostic {
                    message: "consider importing this struct: `use b::c;\n`".to_string(),
                    ..Default::default()
                },
                // code span starting right after a newline
                lsp::Diagnostic {
                    message: "cannot borrow `self.d` as mutable\n`self` is a `&` reference"
                        .to_string(),
                    ..Default::default()
                },
            ],
        };
        RustDiagnosticProcessor.process_diagnostics(&mut params);

        assert_eq!(params.diagnostics[0].message, "use of moved value `a`");

        // remove trailing newline from code span
        assert_eq!(
            params.diagnostics[1].message,
            "consider importing this struct: `use b::c;`"
        );

        // do not remove newline before the start of code span
        assert_eq!(
            params.diagnostics[2].message,
            "cannot borrow `self.d` as mutable\n`self` is a `&` reference"
        );
    }
}
