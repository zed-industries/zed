use gpui::SharedString;
use lsp::{DiagnosticSeverity, MarkupContent, MarkupKind, NumberOrString};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::ops::Range;

/// A diagnostic's display text together with the information needed to render it and
/// round-trip server-provided markup.
///
/// The text is shared so large Markdown messages do not need separate owned copies for
/// display and LSP requests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticMessage {
    text: SharedString,
    rendering: DiagnosticMessageRendering,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DiagnosticMessageRendering {
    Plain,
    AdapterMarkdown(SharedString),
    LspMarkup {
        kind: MarkupKind,
        /// Kept only when trimming changes the display text.
        untrimmed: Option<SharedString>,
    },
}

impl DiagnosticMessage {
    pub fn plain(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            rendering: DiagnosticMessageRendering::Plain,
        }
    }

    pub fn plain_with_adapter_markdown(
        text: impl Into<SharedString>,
        markdown: Option<SharedString>,
    ) -> Self {
        let text = text.into();
        let rendering = markdown.map_or(DiagnosticMessageRendering::Plain, |markdown| {
            DiagnosticMessageRendering::AdapterMarkdown(if markdown == text {
                text.clone()
            } else {
                markdown
            })
        });
        Self { text, rendering }
    }

    pub fn from_lsp_markup(markup: &MarkupContent) -> Self {
        let trimmed = markup.value.trim();
        let (text, untrimmed) = if trimmed == markup.value {
            (SharedString::from(markup.value.as_str()), None)
        } else {
            (
                SharedString::from(trimmed),
                Some(SharedString::from(markup.value.as_str())),
            )
        };
        Self {
            text,
            rendering: DiagnosticMessageRendering::LspMarkup {
                kind: markup.kind.clone(),
                untrimmed,
            },
        }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn as_shared_string(&self) -> &SharedString {
        &self.text
    }

    pub fn markdown(&self) -> Option<&str> {
        match &self.rendering {
            DiagnosticMessageRendering::Plain => None,
            DiagnosticMessageRendering::AdapterMarkdown(markdown) => Some(markdown),
            DiagnosticMessageRendering::LspMarkup {
                kind: MarkupKind::Markdown,
                untrimmed,
            } => Some(untrimmed.as_ref().unwrap_or(&self.text)),
            DiagnosticMessageRendering::LspMarkup {
                kind: MarkupKind::PlainText,
                ..
            } => None,
        }
    }

    pub fn lsp_markup(&self) -> Option<(&MarkupKind, &str)> {
        match &self.rendering {
            DiagnosticMessageRendering::LspMarkup { kind, untrimmed } => {
                Some((kind, untrimmed.as_ref().unwrap_or(&self.text)))
            }
            DiagnosticMessageRendering::Plain | DiagnosticMessageRendering::AdapterMarkdown(_) => {
                None
            }
        }
    }

    pub fn has_lsp_markup(&self) -> bool {
        self.lsp_markup().is_some()
    }

    pub fn rendered_eq(&self, other: &Self) -> bool {
        self.text == other.text && self.markdown() == other.markdown()
    }

    pub fn to_lsp_message(&self) -> lsp::DiagnosticMessage {
        if let Some((kind, value)) = self.lsp_markup() {
            lsp::DiagnosticMessage::MarkupContent(MarkupContent {
                kind: kind.clone(),
                value: value.to_string(),
            })
        } else {
            lsp::DiagnosticMessage::String(self.text.to_string())
        }
    }
}

impl Default for DiagnosticMessage {
    fn default() -> Self {
        Self::plain(SharedString::default())
    }
}

impl AsRef<str> for DiagnosticMessage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DiagnosticMessage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<String> for DiagnosticMessage {
    fn from(text: String) -> Self {
        Self::plain(SharedString::from(text))
    }
}

impl From<&str> for DiagnosticMessage {
    fn from(text: &str) -> Self {
        Self::plain(SharedString::from(text))
    }
}

impl From<SharedString> for DiagnosticMessage {
    fn from(text: SharedString) -> Self {
        Self::plain(text)
    }
}

#[derive(Serialize)]
struct SerializedDiagnosticMessage<'a> {
    message: &'a str,
    markdown: Option<&'a str>,
    lsp_markup: Option<SerializedMarkupContent<'a>>,
}

#[derive(Serialize)]
struct SerializedMarkupContent<'a> {
    kind: &'a MarkupKind,
    value: &'a str,
}

impl Serialize for DiagnosticMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let lsp_markup = match &self.rendering {
            DiagnosticMessageRendering::LspMarkup { kind, untrimmed } => {
                Some(SerializedMarkupContent {
                    kind,
                    value: untrimmed.as_ref().unwrap_or(&self.text),
                })
            }
            DiagnosticMessageRendering::Plain | DiagnosticMessageRendering::AdapterMarkdown(_) => {
                None
            }
        };
        SerializedDiagnosticMessage {
            message: self.as_str(),
            markdown: self.markdown(),
            lsp_markup,
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize)]
struct DeserializedDiagnosticMessage {
    message: SharedString,
    #[serde(default)]
    markdown: Option<SharedString>,
    #[serde(default)]
    lsp_markup: Option<DeserializedMarkupContent>,
}

#[derive(Deserialize)]
struct DeserializedMarkupContent {
    kind: MarkupKind,
    value: SharedString,
}

impl<'de> Deserialize<'de> for DiagnosticMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized = DeserializedDiagnosticMessage::deserialize(deserializer)?;
        if let Some(markup) = serialized.lsp_markup {
            let untrimmed = (markup.value != serialized.message).then_some(markup.value);
            Ok(Self {
                text: serialized.message,
                rendering: DiagnosticMessageRendering::LspMarkup {
                    kind: markup.kind,
                    untrimmed,
                },
            })
        } else {
            Ok(Self::plain_with_adapter_markdown(
                serialized.message,
                serialized.markdown,
            ))
        }
    }
}

/// A diagnostic associated with a certain range of a buffer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The name of the service that produced this diagnostic.
    pub source: Option<String>,
    /// The ID provided by the dynamic registration that produced this diagnostic.
    pub registration_id: Option<SharedString>,
    /// A machine-readable code that identifies this diagnostic.
    pub code: Option<NumberOrString>,
    pub code_description: Option<lsp::Uri>,
    /// Whether this diagnostic is a hint, warning, or error.
    pub severity: DiagnosticSeverity,
    /// The human-readable message associated with this diagnostic.
    #[serde(flatten)]
    pub message: DiagnosticMessage,
    /// An id that identifies the group to which this diagnostic belongs.
    ///
    /// When a language server produces a diagnostic with
    /// one or more associated diagnostics, those diagnostics are all
    /// assigned a single group ID.
    pub group_id: usize,
    /// Whether this diagnostic is the primary diagnostic for its group.
    ///
    /// In a given group, the primary diagnostic is the top-level diagnostic
    /// returned by the language server. The non-primary diagnostics are the
    /// associated diagnostics.
    pub is_primary: bool,
    /// Whether this diagnostic is considered to originate from an analysis of
    /// files on disk, as opposed to any unsaved buffer contents. This is a
    /// property of a given diagnostic source, and is configured for a given
    /// language server via the `LspAdapter::disk_based_diagnostic_sources` method
    /// for the language server.
    pub is_disk_based: bool,
    /// Whether this diagnostic marks unnecessary code.
    pub is_unnecessary: bool,
    /// Quick separation of diagnostics groups based by their source.
    pub source_kind: DiagnosticSourceKind,
    /// Data from language server that produced this diagnostic. Passed back to the LS when we request code actions for this diagnostic.
    pub data: Option<Value>,
    /// Whether to underline the corresponding text range in the editor.
    pub underline: bool,
}

/// A location and message the language server attached to a diagnostic.
///
/// Kept as the server sent it, since flattening it into non-primary entries is lossy,
/// and passed back to the LS when we request code actions for the diagnostic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RelatedInformation<T> {
    /// The location the diagnostic points at.
    pub location: RelatedLocation<T>,
    /// The message as the language server sent it.
    pub message: String,
}

/// Where a [`RelatedInformation`] points.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum RelatedLocation<T> {
    /// A range of the buffer the diagnostic belongs to, in the same coordinates as the
    /// diagnostic's own range, so that it follows edits the same way.
    InBuffer(Range<T>),
    /// A location in another file, as the language server published it. There is
    /// nothing in this buffer to anchor it to.
    InAnotherFile(lsp::Location),
}

impl<T: Clone> RelatedInformation<T> {
    /// Converts the coordinates of this related information to a different type.
    pub(crate) fn map_location<O>(
        &self,
        map: impl FnOnce(&Range<T>) -> Range<O>,
    ) -> RelatedInformation<O> {
        RelatedInformation {
            location: match &self.location {
                RelatedLocation::InBuffer(range) => RelatedLocation::InBuffer(map(range)),
                RelatedLocation::InAnotherFile(location) => {
                    RelatedLocation::InAnotherFile(location.clone())
                }
            },
            message: self.message.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSourceKind {
    Pulled,
    Pushed,
    Other,
}

impl Default for Diagnostic {
    fn default() -> Self {
        Self {
            source: Default::default(),
            source_kind: DiagnosticSourceKind::Other,
            code: None,
            code_description: None,
            severity: DiagnosticSeverity::ERROR,
            message: Default::default(),
            group_id: 0,
            is_primary: false,
            is_disk_based: false,
            is_unnecessary: false,
            underline: true,
            data: None,
            registration_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_markdown_round_trips_as_plain_text() {
        let message =
            DiagnosticMessage::plain_with_adapter_markdown("message", Some("message".into()));

        assert_eq!(message.markdown(), Some("message"));
        assert!(!message.has_lsp_markup());
        assert_eq!(
            message.to_lsp_message(),
            lsp::DiagnosticMessage::String("message".to_string())
        );
    }

    #[test]
    fn test_diagnostic_message_reuses_server_markup_text() {
        let message = DiagnosticMessage::from_lsp_markup(&MarkupContent {
            kind: MarkupKind::Markdown,
            value: "**message**".to_string(),
        });

        assert!(matches!(
            &message.rendering,
            DiagnosticMessageRendering::LspMarkup {
                untrimmed: None,
                ..
            }
        ));
        assert_eq!(message.markdown(), Some("**message**"));
    }

    #[test]
    fn test_diagnostic_message_preserves_lsp_markup() {
        let markdown = MarkupContent {
            kind: MarkupKind::Markdown,
            value: "\n**message**\n".to_string(),
        };
        let message = DiagnosticMessage::from_lsp_markup(&markdown);
        assert_eq!(message.as_str(), "**message**");
        assert_eq!(message.markdown(), Some("\n**message**\n"));
        assert!(message.has_lsp_markup());
        assert_eq!(
            message.to_lsp_message(),
            lsp::DiagnosticMessage::MarkupContent(markdown)
        );

        let plain_text = MarkupContent {
            kind: MarkupKind::PlainText,
            value: "  plain text  ".to_string(),
        };
        let message = DiagnosticMessage::from_lsp_markup(&plain_text);
        assert_eq!(message.as_str(), "plain text");
        assert_eq!(message.markdown(), None);
        assert_eq!(
            message.to_lsp_message(),
            lsp::DiagnosticMessage::MarkupContent(plain_text)
        );
    }

    #[test]
    fn test_diagnostic_message_serialization_remains_flat() {
        let message = DiagnosticMessage::from_lsp_markup(&MarkupContent {
            kind: MarkupKind::Markdown,
            value: "\n**message**\n".to_string(),
        });
        let diagnostic = Diagnostic {
            message,
            ..Diagnostic::default()
        };

        let serialized = serde_json::to_value(&diagnostic).expect("serialize diagnostic");
        assert_eq!(serialized["message"], "**message**");
        assert_eq!(serialized["markdown"], "\n**message**\n");
        assert_eq!(serialized["lsp_markup"]["kind"], "markdown");
        assert_eq!(serialized["lsp_markup"]["value"], "\n**message**\n");

        let deserialized: Diagnostic =
            serde_json::from_value(serialized).expect("deserialize diagnostic");
        assert_eq!(deserialized, diagnostic);
    }
}
