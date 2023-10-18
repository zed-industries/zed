use language::ToOffset;

use crate::templates::base::PromptArguments;
use crate::templates::base::PromptTemplate;
use std::fmt::Write;

pub struct FileContext {}

impl PromptTemplate for FileContext {
    fn generate(
        &self,
        args: &PromptArguments,
        max_token_length: Option<usize>,
    ) -> anyhow::Result<(String, usize)> {
        let mut prompt = String::new();

        // Add Initial Preamble
        // TODO: Do we want to add the path in here?
        writeln!(
            prompt,
            "The file you are currently working on has the following content:"
        )
        .unwrap();

        let language_name = args
            .language_name
            .clone()
            .unwrap_or("".to_string())
            .to_lowercase();
        writeln!(prompt, "```{language_name}").unwrap();

        if let Some(buffer) = &args.buffer {
            let mut content = String::new();

            if let Some(selected_range) = &args.selected_range {
                let start = selected_range.start.to_offset(buffer);
                let end = selected_range.end.to_offset(buffer);

                writeln!(
                    prompt,
                    "{}",
                    buffer.text_for_range(0..start).collect::<String>()
                )
                .unwrap();

                if start == end {
                    write!(prompt, "<|START|>").unwrap();
                } else {
                    write!(prompt, "<|START|").unwrap();
                }

                write!(
                    prompt,
                    "{}",
                    buffer.text_for_range(start..end).collect::<String>()
                )
                .unwrap();
                if start != end {
                    write!(prompt, "|END|>").unwrap();
                }

                write!(
                    prompt,
                    "{}",
                    buffer.text_for_range(end..buffer.len()).collect::<String>()
                )
                .unwrap();

                writeln!(prompt, "```").unwrap();

                if start == end {
                    writeln!(prompt, "In particular, the user's cursor is currently on the '<|START|>' span in the above content, with no text selected.").unwrap();
                } else {
                    writeln!(prompt, "In particular, the user has selected a section of the text between the '<|START|' and '|END|>' spans.").unwrap();
                }
            } else {
                // If we dont have a selected range, include entire file.
                writeln!(prompt, "{}", &buffer.text()).unwrap();
            }
        }

        let token_count = args.model.count_tokens(&prompt)?;
        anyhow::Ok((prompt, token_count))
    }
}
