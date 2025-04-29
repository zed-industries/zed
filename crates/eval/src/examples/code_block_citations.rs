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
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        let todo = (); // TODO change max_assertions from being on ExampleMetadata to being something we can set on cx.

        const FILENAME: &str = "assistant_tool.rs";
        cx.push_user_message(format!(
            r#"
            Show me the method bodies of all the methods of the `Tool` trait in {FILENAME}.
            "#
        ));

        // Verify that the messages all have the correct formatting.
        for mut text in cx.run_to_end().await?.texts() {
            while let Some(index) = text.find(FENCE) {
                // Advance text past the opening backticks.
                text = &text[index + FENCE.len()..];

                let content_len = text.find(FENCE);

                // Verify the citation format - e.g. ```path/to/foo.txt#L123-456
                if let Some(citation_len) = text.find('\n') {
                    let citation = &text[..citation_len];

                    if let Ok(()) = cx.assert(
                        citation.contains("/"),
                        format!("{citation:?} contains a slash.",),
                    ) {
                        let path_range = PathWithRange::new(citation);
                        let path_exists = {
                            let todo = (); // TODO look this up in the project.
                            true
                        };

                        if let Ok(()) =
                            cx.assert(path_exists, format!("{citation:?} has valid path"))
                        {
                            if let Some(content_len) = text.find(FENCE) {
                                let content = &text[..content_len];
                                let todo = (); // TODO verify that the file contents actually contain this content.

                                // Advance past the closing backticks.
                                text = &text[content_len..];
                            } else {
                                cx.assert(false, format!("Code block has closing {FENCE}"))
                                    .ok();
                            }

                            let valid_line_range = {
                                let todo = (); // TODO look this up in the project.
                                true
                            };

                            if let Ok(()) = cx.assert(
                                valid_line_range,
                                format!("{citation:?} has valid line range in file."),
                            ) {
                                let diff = {
                                    let todo = (); // TODO look this up in the project. Note that this requires looking up the closing ``` and getting what's in between there.
                                    ""
                                };
                                cx.assert(
                                    diff.is_empty(),
                                    format!("{citation:?} snippet matches line range contents."),
                                )
                                .ok();
                            } else {
                                let todo = (); // TODO there was no valid line range (or no line range at all?) but we can still check if the file contained the snippet.
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
