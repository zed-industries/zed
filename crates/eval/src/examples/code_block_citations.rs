use agent_settings::AgentProfileId;
use anyhow::Result;
use async_trait::async_trait;
use markdown::PathWithRange;

use crate::example::{Example, ExampleContext, ExampleMetadata, JudgeAssertion, LanguageServer};

pub struct CodeBlockCitations;

const FENCE: &str = "```";

#[async_trait(?Send)]
impl Example for CodeBlockCitations {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "code_block_citations".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "f69aeb6311dde3c0b8979c293d019d66498d54f2".to_string(),
            language_server: Some(LanguageServer {
                file_extension: "rs".to_string(),
                allow_preexisting_diagnostics: false,
            }),
            max_assertions: None,
            profile_id: AgentProfileId::default(),
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        const FILENAME: &str = "assistant_tool.rs";
        cx.push_user_message(format!(
            r#"
            Show me the method bodies of all the methods of the `Tool` trait in {FILENAME}.

            Please show each method in a separate code snippet.
            "#
        ));

        // Verify that the messages all have the correct formatting.
        let texts: Vec<String> = cx.run_to_end().await?.texts().collect();
        let closing_fence = format!("\n{FENCE}");

        for text in texts.iter() {
            let mut text = text.as_str();

            while let Some(index) = text.find(FENCE) {
                // Advance text past the opening backticks.
                text = &text[index + FENCE.len()..];

                // Find the closing backticks.
                let content_len = text.find(&closing_fence);

                // Verify the citation format - e.g. ```path/to/foo.txt#L123-456
                if let Some(citation_len) = text.find('\n') {
                    let citation = &text[..citation_len];

                    if let Ok(()) =
                        cx.assert(citation.contains("/"), format!("Slash in {citation:?}",))
                    {
                        let path_range = PathWithRange::new(citation);
                        let path = cx
                            .agent_thread()
                            .update(cx, |thread, cx| {
                                thread
                                    .project()
                                    .read(cx)
                                    .find_project_path(path_range.path, cx)
                            })
                            .ok()
                            .flatten();

                        if let Ok(path) = cx.assert_some(path, format!("Valid path: {citation:?}"))
                        {
                            let buffer_text = {
                                let buffer = match cx.agent_thread().update(cx, |thread, cx| {
                                    thread
                                        .project()
                                        .update(cx, |project, cx| project.open_buffer(path, cx))
                                }) {
                                    Ok(buffer_task) => buffer_task.await.ok(),
                                    Err(err) => {
                                        cx.assert(
                                            false,
                                            format!("Expected Ok(buffer), not {err:?}"),
                                        )
                                        .ok();
                                        break;
                                    }
                                };

                                let Ok(buffer_text) = cx.assert_some(
                                    buffer.and_then(|buffer| {
                                        buffer.read_with(cx, |buffer, _| buffer.text()).ok()
                                    }),
                                    "Reading buffer text succeeded",
                                ) else {
                                    continue;
                                };
                                buffer_text
                            };

                            if let Some(content_len) = content_len {
                                // + 1 because there's a newline character after the citation.
                                let start_index = citation.len() + 1;
                                let end_index = content_len.saturating_sub(start_index);

                                if cx
                                    .assert(
                                        start_index <= end_index,
                                        "Code block had a valid citation",
                                    )
                                    .is_ok()
                                {
                                    let content = &text[start_index..end_index];

                                    // deindent (trim the start of each line) because sometimes the model
                                    // chooses to deindent its code snippets for the sake of readability,
                                    // which in markdown is not only reasonable but usually desirable.
                                    cx.assert(
                                        deindent(&buffer_text)
                                            .trim()
                                            .contains(deindent(&content).trim()),
                                        "Code block content was found in file",
                                    )
                                    .ok();

                                    if let Some(range) = path_range.range {
                                        let start_line_index = range.start.line.saturating_sub(1);
                                        let line_count =
                                            range.end.line.saturating_sub(start_line_index);
                                        let mut snippet = buffer_text
                                            .lines()
                                            .skip(start_line_index as usize)
                                            .take(line_count as usize)
                                            .collect::<Vec<&str>>()
                                            .join("\n");

                                        if let Some(start_col) = range.start.col {
                                            snippet = snippet[start_col as usize..].to_string();
                                        }

                                        if let Some(end_col) = range.end.col {
                                            let last_line = snippet.lines().last().unwrap();
                                            snippet = snippet[..snippet.len() - last_line.len()
                                                + end_col as usize]
                                                .to_string();
                                        }

                                        // deindent (trim the start of each line) because sometimes the model
                                        // chooses to deindent its code snippets for the sake of readability,
                                        // which in markdown is not only reasonable but usually desirable.
                                        cx.assert_eq(
                                            deindent(snippet.as_str()).trim(),
                                            deindent(content).trim(),
                                            format!(
                                                "Code block was at {:?}-{:?}",
                                                range.start, range.end
                                            ),
                                        )
                                        .ok();
                                    }
                                }
                            }
                        }
                    }
                } else {
                    cx.assert(
                        false,
                        format!("Opening {FENCE} did not have a newline anywhere after it."),
                    )
                    .ok();
                }

                if let Some(content_len) = content_len {
                    // Advance past the closing backticks
                    text = &text[content_len + FENCE.len()..];
                } else {
                    // There were no closing backticks associated with these opening backticks.
                    cx.assert(
                        false,
                        "Code block opening had matching closing backticks.".to_string(),
                    )
                    .ok();

                    // There are no more code blocks to parse, so we're done.
                    break;
                }
            }
        }

        Ok(())
    }

    fn thread_assertions(&self) -> Vec<JudgeAssertion> {
        vec![
            JudgeAssertion {
                id: "trait method bodies are shown".to_string(),
                description:
                    "All method bodies of the Tool trait are shown."
                        .to_string(),
            },
            JudgeAssertion {
                id: "code blocks used".to_string(),
                description:
                   "All code snippets are rendered inside markdown code blocks (as opposed to any other formatting besides code blocks)."
                        .to_string(),
            },
            JudgeAssertion {
              id: "code blocks use backticks".to_string(),
              description:
                  format!("All markdown code blocks use backtick fences ({FENCE}) rather than indentation.")
            }
        ]
    }
}

fn deindent(as_str: impl AsRef<str>) -> String {
    as_str
        .as_ref()
        .lines()
        .map(|line| line.trim_start())
        .collect::<Vec<&str>>()
        .join("\n")
}
