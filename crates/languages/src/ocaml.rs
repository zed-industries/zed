use std::{any::Any, ops::Range, path::PathBuf, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use language::{CodeLabel, LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::{CompletionItemKind, LanguageServerBinary, SymbolKind};
use rope::Rope;

const OPERATOR_CHAR: [char; 17] = [
    '~', '!', '?', '%', '<', ':', '.', '$', '&', '*', '+', '-', '/', '=', '>', '@', '^',
];

pub struct OCamlLspAdapter;

#[async_trait]
impl LspAdapter for OCamlLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("ocamllsp".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()))
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        Err(anyhow!(
            "ocamllsp (ocaml-language-server) must be installed manually."
        ))
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LanguageServerBinary {
            path: "ocamllsp".into(),
            env: None,
            arguments: vec![],
        })
    }

    fn can_be_reinstalled(&self) -> bool {
        false
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<CodeLabel> {
        let name = &completion.label;
        let detail = completion.detail.as_ref().map(|s| s.replace('\n', " "));

        match completion.kind.zip(detail) {
            // Error of 'b : ('a, 'b) result
            // Stack_overflow : exn
            Some((CompletionItemKind::CONSTRUCTOR | CompletionItemKind::ENUM_MEMBER, detail)) => {
                let (argument, return_t) = detail
                    .split_once("->")
                    .map_or((None, detail.as_str()), |(arg, typ)| {
                        (Some(arg.trim()), typ.trim())
                    });

                let constr_decl = argument.map_or(name.to_string(), |argument| {
                    format!("{} of {}", name, argument)
                });

                let constr_host = if return_t.ends_with("exn") {
                    "exception "
                } else {
                    "type t = "
                };

                let source_host = Rope::from([constr_host, &constr_decl].join(" "));
                let mut source_highlight = {
                    let constr_host_len = constr_host.len() + 1;

                    language.highlight_text(
                        &source_host,
                        Range {
                            start: constr_host_len,
                            end: constr_host_len + constr_decl.len(),
                        },
                    )
                };

                let signature_host: Rope = Rope::from(format!("let _ : {} = ()", return_t));

                // We include the ': ' in the range as we use it later
                let mut signature_highlight =
                    language.highlight_text(&signature_host, 6..8 + return_t.len());

                if let Some(last) = source_highlight.last() {
                    let offset = last.0.end + 1;

                    signature_highlight.iter_mut().for_each(|(r, _)| {
                        r.start += offset;
                        r.end += offset;
                    });
                };

                Some(CodeLabel {
                    text: format!("{} : {}", constr_decl, return_t),
                    runs: {
                        source_highlight.append(&mut signature_highlight);
                        source_highlight
                    },
                    filter_range: 0..name.len(),
                })
            }
            // version : string
            // NOTE: (~|?) are omitted as we don't use them in the fuzzy filtering
            Some((CompletionItemKind::FIELD, detail))
                if name.starts_with('~') || name.starts_with('?') =>
            {
                let label = name.trim_start_matches(&['~', '?']);
                let text = format!("{} : {}", label, detail);

                let signature_host = Rope::from(format!("let _ : {} = ()", detail));
                let signature_highlight =
                    &mut language.highlight_text(&signature_host, 6..8 + detail.len());

                let offset = label.len() + 1;
                for (r, _) in signature_highlight.iter_mut() {
                    r.start += offset;
                    r.end += offset;
                }

                let mut label_highlight = vec![(
                    0..label.len(),
                    language.grammar()?.highlight_id_for_name("property")?,
                )];

                Some(CodeLabel {
                    text,
                    runs: {
                        label_highlight.append(signature_highlight);
                        label_highlight
                    },
                    filter_range: 0..label.len(),
                })
            }
            // version: string;
            Some((CompletionItemKind::FIELD, detail)) => {
                let (_record_t, field_t) = detail.split_once("->")?;

                let text = format!("{}: {};", name, field_t);
                let source_host: Rope = Rope::from(format!("type t = {{ {} }}", text));

                let runs: Vec<(Range<usize>, language::HighlightId)> =
                    language.highlight_text(&source_host, 11..11 + text.len());

                Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                })
            }
            // let* : 'a t -> ('a -> 'b t) -> 'b t
            Some((CompletionItemKind::VALUE, detail))
                if name.contains(OPERATOR_CHAR)
                    || (name.starts_with("let") && name.contains(OPERATOR_CHAR)) =>
            {
                let text = format!("{} : {}", name, detail);

                let source_host = Rope::from(format!("let ({}) : {} = ()", name, detail));
                let mut runs = language.highlight_text(&source_host, 5..6 + text.len());

                if runs.len() > 1 {
                    // ')'
                    runs.remove(1);

                    for run in &mut runs[1..] {
                        run.0.start -= 1;
                        run.0.end -= 1;
                    }
                }

                Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                })
            }
            // version : Version.t list -> Version.t option Lwt.t
            Some((CompletionItemKind::VALUE, detail)) => {
                let text = format!("{} : {}", name, detail);

                let source_host = Rope::from(format!("let {} = ()", text));
                let runs = language.highlight_text(&source_host, 4..4 + text.len());

                Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                })
            }
            // status : string
            Some((CompletionItemKind::METHOD, detail)) => {
                let text = format!("{} : {}", name, detail);

                let method_host = Rope::from(format!("class c : object method {} end", text));
                let runs = language.highlight_text(&method_host, 24..24 + text.len());

                Some(CodeLabel {
                    text,
                    runs,
                    filter_range: 0..name.len(),
                })
            }
            Some((kind, _)) => {
                let highlight_name = match kind {
                    CompletionItemKind::MODULE | CompletionItemKind::INTERFACE => "title",
                    CompletionItemKind::KEYWORD => "keyword",
                    CompletionItemKind::TYPE_PARAMETER => "type",
                    _ => return None,
                };

                Some(CodeLabel {
                    text: name.clone(),
                    runs: vec![(
                        0..name.len(),
                        language.grammar()?.highlight_id_for_name(highlight_name)?,
                    )],
                    filter_range: 0..name.len(),
                })
            }
            _ => None,
        }
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: SymbolKind,
        language: &Arc<language::Language>,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            SymbolKind::PROPERTY => {
                let text = format!("type t = {{ {}: (); }}", name);
                let filter_range: Range<usize> = 0..name.len();
                let display_range = 11..11 + name.len();
                (text, filter_range, display_range)
            }
            SymbolKind::FUNCTION
                if name.contains(OPERATOR_CHAR)
                    || (name.starts_with("let") && name.contains(OPERATOR_CHAR)) =>
            {
                let text = format!("let ({}) () = ()", name);

                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end + 1;
                (text, filter_range, display_range)
            }
            SymbolKind::FUNCTION => {
                let text = format!("let {} () = ()", name);

                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            SymbolKind::CONSTRUCTOR => {
                let text = format!("type t = {}", name);
                let filter_range = 0..name.len();
                let display_range = 9..9 + name.len();
                (text, filter_range, display_range)
            }
            SymbolKind::MODULE => {
                let text = format!("module {} = struct end", name);
                let filter_range = 7..7 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            SymbolKind::CLASS => {
                let text = format!("class {} = object end", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            SymbolKind::METHOD => {
                let text = format!("class c = object method {} = () end", name);
                let filter_range = 0..name.len();
                let display_range = 17..24 + name.len();
                (text, filter_range, display_range)
            }
            SymbolKind::STRING => {
                let text = format!("type {} = T", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }
}
