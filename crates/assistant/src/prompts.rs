use language::BufferSnapshot;
use std::{fmt::Write, ops::Range};

pub fn generate_content_prompt(
    user_prompt: String,
    language_name: Option<&str>,
    buffer: BufferSnapshot,
    range: Range<usize>,
    project_name: Option<String>,
) -> anyhow::Result<String> {
    let mut prompt = String::new();

    let content_type = match language_name {
        None | Some("Markdown" | "Plain Text") => {
            writeln!(prompt, "You are an expert engineer.")?;
            "Text"
        }
        Some(language_name) => {
            writeln!(prompt, "You are an expert {language_name} engineer.")?;
            writeln!(
                prompt,
                "Your answer MUST always and only be valid {}.",
                language_name
            )?;
            "Code"
        }
    };

    if let Some(project_name) = project_name {
        writeln!(
            prompt,
            "You are currently working inside the '{project_name}' project in code editor Zed."
        )?;
    }

    writeln!(
        prompt,
        "The user has the following file open in the editor:"
    )?;
    if range.is_empty() {
        write!(prompt, "```")?;
        if let Some(language_name) = language_name {
            write!(prompt, "{language_name}")?;
        }

        for chunk in buffer.as_rope().chunks_in_range(0..range.start) {
            prompt.push_str(chunk);
        }
        prompt.push_str("<|CURSOR|>");
        for chunk in buffer.as_rope().chunks_in_range(range.start..buffer.len()) {
            prompt.push_str(chunk);
        }
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        writeln!(prompt, "```")?;
        prompt.push('\n');

        writeln!(
            prompt,
            "Assume the cursor is located where the `<|CURSOR|>` span is."
        )
        .unwrap();
        writeln!(
            prompt,
            "{content_type} can't be replaced, so assume your answer will be inserted at the cursor.",
        )
        .unwrap();
        writeln!(
            prompt,
            "Generate {content_type} based on the users prompt: {user_prompt}",
        )
        .unwrap();
    } else {
        write!(prompt, "```")?;
        for chunk in buffer.as_rope().chunks() {
            prompt.push_str(chunk);
        }
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        writeln!(prompt, "```")?;
        prompt.push('\n');

        writeln!(
            prompt,
            "In particular, the following piece of text is selected:"
        )?;
        write!(prompt, "```")?;
        if let Some(language_name) = language_name {
            write!(prompt, "{language_name}")?;
        }
        prompt.push('\n');
        for chunk in buffer.text_for_range(range.clone()) {
            prompt.push_str(chunk);
        }
        if !prompt.ends_with('\n') {
            prompt.push('\n');
        }
        writeln!(prompt, "```")?;
        prompt.push('\n');

        writeln!(
            prompt,
            "Modify the user's selected {content_type} based upon the users prompt: {user_prompt}"
        )
        .unwrap();
        writeln!(
            prompt,
            "You must reply with only the adjusted {content_type}, not the entire file."
        )
        .unwrap();
    }

    writeln!(prompt, "Never make remarks about the output.").unwrap();
    writeln!(
        prompt,
        "Do not return anything else, except the generated {content_type}."
    )
    .unwrap();

    Ok(prompt)
}
