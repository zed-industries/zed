use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The single-select field key used when presenting a fixed list of options.
const CHOICE_FIELD: &str = "choice";
/// The free-text field key used when the user may type their own answer.
const OTHER_FIELD: &str = "other";

/// Ask the user a question, presenting selectable options and/or a free-text
/// field.
///
/// Use this when you need a decision or a piece of information from the user
/// mid-task — for example choosing between implementation approaches, confirming
/// an assumption, or supplying a value you can't infer. The question is rendered
/// as a small form in the conversation, and the user's answer is returned to you
/// verbatim.
///
/// You control the shape of the answer:
/// - Provide `options` (two or more) to present clickable choices.
/// - Set `allow_free_text` to `true` to let the user type their own answer. Do
///   this when the listed options might not be exhaustive, or when you want a
///   free-form reply with no preset options at all.
///
/// You must provide either at least two `options`, or set `allow_free_text` to
/// `true` (or both). If both are supplied, the user may pick an option or type
/// their own answer, and a typed answer takes precedence.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AskUserToolInput {
    /// The question to ask the user. Keep it short and specific.
    pub question: String,
    /// The answer choices to present as selectable options. Each entry is the
    /// exact label shown, and is returned verbatim when chosen. Provide at
    /// least two options, or leave empty and set `allow_free_text` to `true`.
    #[serde(default)]
    pub options: Vec<String>,
    /// Whether the user may type their own free-form answer instead of (or in
    /// addition to) picking an option. Set to `true` when the options may not
    /// be exhaustive, or when you want a free-form reply.
    #[serde(default)]
    pub allow_free_text: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AskUserToolOutput {
    Answered { selected: String },
    Error { error: String },
}

impl From<AskUserToolOutput> for LanguageModelToolResultContent {
    fn from(value: AskUserToolOutput) -> Self {
        match value {
            AskUserToolOutput::Answered { selected } => {
                format!("The user selected: {selected}").into()
            }
            AskUserToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct AskUserTool;

impl AgentTool for AskUserTool {
    type Input = AskUserToolInput;
    type Output = AskUserToolOutput;

    const NAME: &'static str = "ask_user";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        match input {
            Ok(input) if !input.question.is_empty() => SharedString::from(input.question),
            _ => "Asking a question".into(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|error| AskUserToolOutput::Error {
                error: error.to_string(),
            })?;

            if !input.allow_free_text && input.options.len() < 2 {
                return Err(AskUserToolOutput::Error {
                    error: "The `ask_user` tool needs at least two `options`, or \
                            `allow_free_text` set to true. Ask an open-ended question \
                            in prose instead."
                        .to_string(),
                });
            }

            let schema = build_schema(&input.options, input.allow_free_text);

            let prompt = cx.update(|cx| {
                event_stream.request_elicitation(input.question.clone(), schema, cx)
            });
            let response = prompt.await.map_err(|error| AskUserToolOutput::Error {
                error: error.to_string(),
            })?;

            let selected = match response.action {
                acp::ElicitationAction::Accept(accept) => {
                    let content = accept.content.unwrap_or_default();
                    // A typed answer takes precedence over a picked option.
                    string_field(&content, OTHER_FIELD)
                        .or_else(|| string_field(&content, CHOICE_FIELD))
                        .ok_or_else(|| AskUserToolOutput::Error {
                            error: "The user submitted the form without providing an answer."
                                .to_string(),
                        })?
                }
                acp::ElicitationAction::Decline => {
                    return Err(AskUserToolOutput::Error {
                        error: "The user declined to answer the question.".to_string(),
                    });
                }
                acp::ElicitationAction::Cancel => {
                    return Err(AskUserToolOutput::Error {
                        error: "The user cancelled the question without answering.".to_string(),
                    });
                }
                _ => {
                    return Err(AskUserToolOutput::Error {
                        error: "The question was dismissed without an answer.".to_string(),
                    });
                }
            };

            event_stream.update_fields(
                acp::ToolCallUpdateFields::new().title(format!("Answered: {selected}")),
            );

            Ok(AskUserToolOutput::Answered { selected })
        })
    }
}

/// Builds the elicitation form schema for a question.
///
/// - Presenting a fixed list of options adds a required single-select field,
///   unless free text is also allowed (in which case picking is optional).
/// - Allowing free text adds a text field, required only when there are no
///   options to choose from.
fn build_schema(options: &[String], allow_free_text: bool) -> acp::ElicitationSchema {
    let mut schema = acp::ElicitationSchema::new();

    if !options.is_empty() {
        let enum_options = options
            .iter()
            .map(|label| acp::EnumOption::new(label.clone(), label.clone()))
            .collect::<Vec<_>>();
        schema = schema.property(
            CHOICE_FIELD,
            acp::StringPropertySchema::new()
                .title("Choose an option")
                .one_of(enum_options),
            !allow_free_text,
        );
    }

    if allow_free_text {
        let title = if options.is_empty() {
            "Your answer"
        } else {
            "Or type your own answer"
        };
        schema = schema.property(
            OTHER_FIELD,
            acp::StringPropertySchema::new().title(title),
            options.is_empty(),
        );
    }

    schema
}

/// Extracts a non-empty string value for `key` from an elicitation response.
fn string_field(
    content: &BTreeMap<String, acp::ElicitationContentValue>,
    key: &str,
) -> Option<String> {
    match content.get(key) {
        Some(acp::ElicitationContentValue::String(value)) if !value.is_empty() => {
            Some(value.clone())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    fn accept_with(
        entries: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> acp::CreateElicitationResponse {
        let content: BTreeMap<String, acp::ElicitationContentValue> = entries
            .into_iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    acp::ElicitationContentValue::String(value.to_string()),
                )
            })
            .collect();
        acp::CreateElicitationResponse::new(acp::ElicitationAction::Accept(
            acp::ElicitationAcceptAction::new().content(content),
        ))
    }

    #[gpui::test]
    async fn test_ask_user_returns_selected_option(cx: &mut TestAppContext) {
        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let tool = Arc::new(AskUserTool);
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(AskUserToolInput {
                    question: "Which approach?".into(),
                    options: vec!["Approach A".into(), "Approach B".into()],
                    allow_free_text: false,
                }),
                event_stream,
                cx,
            )
        });

        let request = event_rx.expect_elicitation().await;
        assert_eq!(request.message, "Which approach?");
        assert!(request.schema.properties.contains_key(CHOICE_FIELD));
        assert!(!request.schema.properties.contains_key(OTHER_FIELD));

        request.response.send(accept_with([(CHOICE_FIELD, "Approach B")])).unwrap();

        match task.await {
            Ok(AskUserToolOutput::Answered { selected }) => assert_eq!(selected, "Approach B"),
            other => panic!("expected an answer, got {other:?}"),
        }
    }

    #[gpui::test]
    async fn test_ask_user_free_text_answer(cx: &mut TestAppContext) {
        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let tool = Arc::new(AskUserTool);
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(AskUserToolInput {
                    question: "What should I name it?".into(),
                    options: vec![],
                    allow_free_text: true,
                }),
                event_stream,
                cx,
            )
        });

        let request = event_rx.expect_elicitation().await;
        assert!(request.schema.properties.contains_key(OTHER_FIELD));
        assert!(!request.schema.properties.contains_key(CHOICE_FIELD));

        request.response.send(accept_with([(OTHER_FIELD, "widget")])).unwrap();

        match task.await {
            Ok(AskUserToolOutput::Answered { selected }) => assert_eq!(selected, "widget"),
            other => panic!("expected an answer, got {other:?}"),
        }
    }

    #[gpui::test]
    async fn test_ask_user_free_text_takes_precedence(cx: &mut TestAppContext) {
        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let tool = Arc::new(AskUserTool);
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(AskUserToolInput {
                    question: "Pick or type".into(),
                    options: vec!["A".into(), "B".into()],
                    allow_free_text: true,
                }),
                event_stream,
                cx,
            )
        });

        let request = event_rx.expect_elicitation().await;
        assert!(request.schema.properties.contains_key(CHOICE_FIELD));
        assert!(request.schema.properties.contains_key(OTHER_FIELD));

        request
            .response
            .send(accept_with([(CHOICE_FIELD, "A"), (OTHER_FIELD, "custom")]))
            .unwrap();

        match task.await {
            Ok(AskUserToolOutput::Answered { selected }) => assert_eq!(selected, "custom"),
            other => panic!("expected the typed answer, got {other:?}"),
        }
    }

    #[gpui::test]
    async fn test_ask_user_decline_is_an_error(cx: &mut TestAppContext) {
        let (event_stream, mut event_rx) = ToolCallEventStream::test();
        let tool = Arc::new(AskUserTool);
        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(AskUserToolInput {
                    question: "Proceed?".into(),
                    options: vec!["Yes".into(), "No".into()],
                    allow_free_text: false,
                }),
                event_stream,
                cx,
            )
        });

        let request = event_rx.expect_elicitation().await;
        request
            .response
            .send(acp::CreateElicitationResponse::new(
                acp::ElicitationAction::Decline,
            ))
            .unwrap();

        match task.await {
            Err(AskUserToolOutput::Error { error }) => {
                assert!(error.contains("declined"), "got: {error}");
            }
            other => panic!("expected an error, got {other:?}"),
        }
    }

    #[gpui::test]
    async fn test_ask_user_requires_options_or_free_text(cx: &mut TestAppContext) {
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let tool = Arc::new(AskUserTool);
        let result = cx
            .update(|cx| {
                tool.run(
                    ToolInput::resolved(AskUserToolInput {
                        question: "Proceed?".into(),
                        options: vec!["Only one".into()],
                        allow_free_text: false,
                    }),
                    event_stream,
                    cx,
                )
            })
            .await;

        match result {
            Err(AskUserToolOutput::Error { error }) => {
                assert!(error.contains("at least two"), "got: {error}");
            }
            other => panic!("expected an error, got {other:?}"),
        }
    }
}
