use anyhow::Result;
use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequest, Role};
use editor::Editor;
use gpui::{actions, AppContext, Task, ViewContext};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

actions!(ai, [Assist]);

pub fn init(cx: &mut AppContext) {
    cx.add_async_action(assist)
}

fn assist(
    editor: &mut Editor,
    _: &Assist,
    cx: &mut ViewContext<Editor>,
) -> Option<Task<Result<()>>> {
    let markdown = editor.text(cx);
    parse_dialog(&markdown);
    None
}

fn parse_dialog(markdown: &str) -> CreateChatCompletionRequest {
    let parser = Parser::new(markdown);
    let mut messages = Vec::new();

    let mut current_role: Option<(Role, Option<String>)> = None;
    let mut buffer = String::new();
    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H2, _, _)) => {
                if let Some((role, name)) = current_role.take() {
                    if !buffer.is_empty() {
                        messages.push(ChatCompletionRequestMessage {
                            role,
                            content: buffer.trim().to_string(),
                            name,
                        });
                        buffer.clear();
                    }
                }
            }
            Event::Text(text) => {
                if current_role.is_some() {
                    buffer.push_str(&text);
                } else {
                    // Determine the current role based on the H2 header text
                    let mut chars = text.chars();
                    let first_char = chars.by_ref().skip_while(|c| c.is_whitespace()).next();
                    let name = chars.take_while(|c| *c != '\n').collect::<String>();
                    let name = if name.is_empty() { None } else { Some(name) };

                    let role = match first_char {
                        Some('@') => Some(Role::User),
                        Some('/') => Some(Role::Assistant),
                        Some('#') => Some(Role::System),
                        _ => None,
                    };

                    current_role = role.map(|role| (role, name));
                }
            }
            _ => (),
        }
    }
    if let Some((role, name)) = current_role {
        messages.push(ChatCompletionRequestMessage {
            role,
            content: buffer,
            name,
        });
    }

    CreateChatCompletionRequest {
        model: "gpt-4".into(),
        messages,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dialog() {
        use unindent::Unindent;

        let test_input = r#"
            ## @nathan
            Hey there, welcome to Zed!

            ## /sky
            Thanks! I'm excited to be here. I have much to learn, but also much to teach, and I'm growing fast.
        "#.unindent();

        let expected_output = vec![
            ChatCompletionRequestMessage {
                role: Role::User,
                content: "Hey there, welcome to Zed!".to_string(),
                name: Some("nathan".to_string()),
            },
            ChatCompletionRequestMessage {
                role: Role::Assistant,
                content: "Thanks! I'm excited to be here. I have much to learn, but also much to teach, and I'm growing fast.".to_string(),
                name: Some("sky".to_string()),
            },
        ];

        assert_eq!(parse_dialog(&test_input).messages, expected_output);
    }
}
