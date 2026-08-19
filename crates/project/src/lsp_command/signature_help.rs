use std::{ops::Range, sync::Arc};

use gpui::{App, AppContext, Entity, FontWeight, HighlightStyle, SharedString};
use language::LanguageRegistry;
use lsp::LanguageServerId;
use markdown::Markdown;
use rpc::proto::{self, documentation};
use util::maybe;

#[derive(Debug)]
pub struct SignatureHelp {
    pub active_signature: usize,
    pub signatures: Vec<SignatureHelpData>,
    pub(super) original_data: lsp::SignatureHelp,
}

#[derive(Debug, Clone)]
pub struct SignatureHelpData {
    pub label: SharedString,
    pub documentation: Option<Entity<Markdown>>,
    pub highlights: Vec<(Range<usize>, HighlightStyle)>,
    pub active_parameter: Option<usize>,
    pub parameters: Vec<ParameterInfo>,
}

#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub label_range: Option<Range<usize>>,
    pub documentation: Option<Entity<Markdown>>,
}

impl SignatureHelp {
    pub fn new(
        help: lsp::SignatureHelp,
        language_registry: Option<Arc<LanguageRegistry>>,
        lang_server_id: Option<LanguageServerId>,
        cx: &mut App,
    ) -> Option<Self> {
        if help.signatures.is_empty() {
            return None;
        }
        let active_signature = help.active_signature.unwrap_or(0) as usize;
        let mut signatures = Vec::<SignatureHelpData>::with_capacity(help.signatures.capacity());
        for signature in &help.signatures {
            let label = SharedString::from(signature.label.clone());
            let active_parameter = signature
                .active_parameter
                .unwrap_or_else(|| help.active_parameter.unwrap_or(0))
                as usize;
            let mut highlights = Vec::new();
            let mut parameter_infos = Vec::new();

            if let Some(parameters) = &signature.parameters {
                for (index, parameter) in parameters.iter().enumerate() {
                    let label_range = match &parameter.label {
                        &lsp::ParameterLabel::LabelOffsets([offset1, offset2]) => {
                            maybe!({
                                let offset1 = offset1 as usize;
                                let offset2 = offset2 as usize;
                                if offset1 < offset2 {
                                    let mut indices = label.char_indices().scan(
                                        0,
                                        |utf16_offset_acc, (offset, c)| {
                                            let utf16_offset = *utf16_offset_acc;
                                            *utf16_offset_acc += c.len_utf16();
                                            Some((utf16_offset, offset))
                                        },
                                    );
                                    let (_, offset1) = indices
                                        .find(|(utf16_offset, _)| *utf16_offset == offset1)?;
                                    let (_, offset2) = indices
                                        .find(|(utf16_offset, _)| *utf16_offset == offset2)?;
                                    Some(offset1..offset2)
                                } else {
                                    log::warn!(
                                        "language server {lang_server_id:?} produced invalid parameter label range: {offset1:?}..{offset2:?}",
                                    );
                                    None
                                }
                            })
                        }
                        lsp::ParameterLabel::Simple(parameter_label) => {
                            if let Some(start) = signature.label.find(parameter_label) {
                                Some(start..start + parameter_label.len())
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(label_range) = &label_range
                        && index == active_parameter
                    {
                        highlights.push((
                            label_range.clone(),
                            HighlightStyle {
                                font_weight: Some(FontWeight::EXTRA_BOLD),
                                ..HighlightStyle::default()
                            },
                        ));
                    }

                    let documentation = parameter
                        .documentation
                        .as_ref()
                        .map(|doc| documentation_to_markdown(doc, language_registry.clone(), cx));

                    parameter_infos.push(ParameterInfo {
                        label_range,
                        documentation,
                    });
                }
            }

            let documentation = signature
                .documentation
                .as_ref()
                .map(|doc| documentation_to_markdown(doc, language_registry.clone(), cx));

            signatures.push(SignatureHelpData {
                label,
                documentation,
                highlights,
                active_parameter: Some(active_parameter),
                parameters: parameter_infos,
            });
        }
        Some(Self {
            signatures,
            active_signature,
            original_data: help,
        })
    }
}

fn documentation_to_markdown(
    documentation: &lsp::Documentation,
    language_registry: Option<Arc<LanguageRegistry>>,
    cx: &mut App,
) -> Entity<Markdown> {
    match documentation {
        lsp::Documentation::String(string) => {
            cx.new(|cx| Markdown::new_text(SharedString::from(string), cx))
        }
        lsp::Documentation::MarkupContent(markup) => match markup.kind {
            lsp::MarkupKind::PlainText => {
                cx.new(|cx| Markdown::new_text(SharedString::from(&markup.value), cx))
            }
            lsp::MarkupKind::Markdown => cx.new(|cx| {
                Markdown::new(
                    SharedString::from(&markup.value),
                    language_registry,
                    None,
                    cx,
                )
            }),
        },
    }
}

pub fn lsp_to_proto_signature(lsp_help: lsp::SignatureHelp) -> proto::SignatureHelp {
    proto::SignatureHelp {
        signatures: lsp_help
            .signatures
            .into_iter()
            .map(|signature| proto::SignatureInformation {
                label: signature.label,
                documentation: signature.documentation.map(lsp_to_proto_documentation),
                parameters: signature
                    .parameters
                    .unwrap_or_default()
                    .into_iter()
                    .map(|parameter_info| proto::ParameterInformation {
                        label: Some(match parameter_info.label {
                            lsp::ParameterLabel::Simple(label) => {
                                proto::parameter_information::Label::Simple(label)
                            }
                            lsp::ParameterLabel::LabelOffsets(offsets) => {
                                proto::parameter_information::Label::LabelOffsets(
                                    proto::LabelOffsets {
                                        start: offsets[0],
                                        end: offsets[1],
                                    },
                                )
                            }
                        }),
                        documentation: parameter_info.documentation.map(lsp_to_proto_documentation),
                    })
                    .collect(),
                active_parameter: signature.active_parameter,
            })
            .collect(),
        active_signature: lsp_help.active_signature,
        active_parameter: lsp_help.active_parameter,
    }
}

fn lsp_to_proto_documentation(documentation: lsp::Documentation) -> proto::Documentation {
    proto::Documentation {
        content: Some(match documentation {
            lsp::Documentation::String(string) => proto::documentation::Content::Value(string),
            lsp::Documentation::MarkupContent(content) => {
                proto::documentation::Content::MarkupContent(proto::MarkupContent {
                    is_markdown: matches!(content.kind, lsp::MarkupKind::Markdown),
                    value: content.value,
                })
            }
        }),
    }
}

pub fn proto_to_lsp_signature(proto_help: proto::SignatureHelp) -> lsp::SignatureHelp {
    lsp::SignatureHelp {
        signatures: proto_help
            .signatures
            .into_iter()
            .map(|signature| lsp::SignatureInformation {
                label: signature.label,
                documentation: signature.documentation.and_then(proto_to_lsp_documentation),
                parameters: Some(
                    signature
                        .parameters
                        .into_iter()
                        .filter_map(|parameter_info| {
                            Some(lsp::ParameterInformation {
                                label: match parameter_info.label? {
                                    proto::parameter_information::Label::Simple(string) => {
                                        lsp::ParameterLabel::Simple(string)
                                    }
                                    proto::parameter_information::Label::LabelOffsets(offsets) => {
                                        lsp::ParameterLabel::LabelOffsets([
                                            offsets.start,
                                            offsets.end,
                                        ])
                                    }
                                },
                                documentation: parameter_info
                                    .documentation
                                    .and_then(proto_to_lsp_documentation),
                            })
                        })
                        .collect(),
                ),
                active_parameter: signature.active_parameter,
            })
            .collect(),
        active_signature: proto_help.active_signature,
        active_parameter: proto_help.active_parameter,
    }
}

fn proto_to_lsp_documentation(documentation: proto::Documentation) -> Option<lsp::Documentation> {
    {
        Some(match documentation.content? {
            documentation::Content::Value(string) => lsp::Documentation::String(string),
            documentation::Content::MarkupContent(markup) => {
                lsp::Documentation::MarkupContent(if markup.is_markdown {
                    lsp::MarkupContent {
                        kind: lsp::MarkupKind::Markdown,
                        value: markup.value,
                    }
                } else {
                    lsp::MarkupContent {
                        kind: lsp::MarkupKind::PlainText,
                        value: markup.value,
                    }
                })
            }
        })
    }
}
