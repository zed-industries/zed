use language::BufferSnapshot;
use std::{fmt::Write, ops::Range};

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: BufferSnapshot,
    range: Range<usize>,
    _project_name: Option<String>,
) -> anyhow::Result<String> {
    let mut prompt = String::new();

    let content_type = match language_name {
        None | Some("Markdown" | "Plain Text") => {
            writeln!(
                prompt,
                "Here's a file of text that I'm going to ask you to make an edit to."
            )?;
            "text"
        }
        Some(language_name) => {
            writeln!(
                prompt,
                "Here's a file of {language_name} that I'm going to ask you to make an edit to."
            )?;
            "code"
        }
    };

    const MAX_CTX: usize = 50000;
    let mut is_truncated = false;
    if range.is_empty() {
        prompt.push_str("The point you'll need to insert at is marked with <insert_here></insert_here>.\n\n<document>");
    } else {
        prompt.push_str("The section you'll need to rewrite is marked with <rewrite_this></rewrite_this> tags.\n\n<document>");
    }
    // Include file content.
    let before_range = 0..range.start;
    let truncated_before = if before_range.len() > MAX_CTX {
        is_truncated = true;
        range.start - MAX_CTX..range.start
    } else {
        before_range
    };
    let mut non_rewrite_len = truncated_before.len();
    for chunk in buffer.text_for_range(truncated_before) {
        prompt.push_str(chunk);
    }
    if !range.is_empty() {
        prompt.push_str("<rewrite_this>\n");
        for chunk in buffer.text_for_range(range.clone()) {
            prompt.push_str(chunk);
        }
        prompt.push_str("\n<rewrite_this>");
    } else {
        prompt.push_str("<insert_here></insert_here>");
    }
    let after_range = range.end..buffer.len();
    let truncated_after = if after_range.len() > MAX_CTX {
        is_truncated = true;
        range.end..range.end + MAX_CTX
    } else {
        after_range
    };
    non_rewrite_len += truncated_after.len();
    for chunk in buffer.text_for_range(truncated_after) {
        prompt.push_str(chunk);
    }

    write!(prompt, "</document>\n\n").unwrap();

    if is_truncated {
        writeln!(prompt, "The context around the relevant section has been truncated (possibly in the middle of a line) for brevity.\n")?;
    }

    if range.is_empty() {
        writeln!(
                prompt,
                "You can't replace {content_type}, your answer will be inserted in place of the `<insert_here></insert_here>` tags. Don't include the insert_here tags in your output.",
            )
            .unwrap();
        writeln!(
                prompt,
                "Generate {content_type} based on the following prompt:\n\n<prompt>\n{user_prompt}\n</prompt>",
            )
            .unwrap();
        writeln!(prompt, "Match the indentation in the original file in the inserted {content_type}, don't include any indentation on blank lines.\n").unwrap();
        prompt.push_str("Immediately start with the following format with no remarks:\n\n```\n{{INSERTED_CODE}}\n```");
    } else {
        writeln!(prompt, "Edit the section of {content_type} in <rewrite_this></rewrite_this> tags based on the following prompt:'").unwrap();
        writeln!(prompt, "\n<prompt>\n{user_prompt}\n</prompt>\n").unwrap();
        let rewrite_len = range.end - range.start;
        if rewrite_len < 20000 && rewrite_len * 2 < non_rewrite_len {
            writeln!(prompt, "And here's the section to rewrite based on that prompt again for reference:\n\n<rewrite_this>\n").unwrap();
            for chunk in buffer.text_for_range(range.clone()) {
                prompt.push_str(chunk);
            }
            writeln!(prompt, "\n</rewrite_this>\n").unwrap();
        }
        writeln!(prompt, "Only make changes that are necessary to fulfill the prompt, leave everything else as-is. All surrounding {content_type} will be preserved.\n").unwrap();
        write!(
            prompt,
            "Start at the indentation level in the original file in the rewritten {content_type}. "
        )
        .unwrap();
        prompt.push_str("Don't stop until you've rewritten the entire section, even if you have no more changes to make, always write out the whole section with no unnecessary elisions.");
        prompt.push_str("\n\nImmediately start with the following format with no remarks:\n\n```\n{{REWRITTEN_CODE}}\n```");
    }

    Ok(prompt)
}
