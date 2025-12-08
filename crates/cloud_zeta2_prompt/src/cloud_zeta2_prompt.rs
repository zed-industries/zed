use anyhow::Result;
use cloud_llm_client::predict_edits_v3::{
    self, DiffPathFmt, Event, Excerpt, Line, Point, PromptFormat, RelatedFile,
};
use indoc::indoc;
use std::cmp;
use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;

pub const DEFAULT_MAX_PROMPT_BYTES: usize = 10 * 1024;

pub const CURSOR_MARKER: &str = "<|user_cursor|>";
/// NOTE: Differs from zed version of constant - includes a newline
pub const EDITABLE_REGION_START_MARKER_WITH_NEWLINE: &str = "<|editable_region_start|>\n";
/// NOTE: Differs from zed version of constant - includes a newline
pub const EDITABLE_REGION_END_MARKER_WITH_NEWLINE: &str = "<|editable_region_end|>\n";

const STUDENT_MODEL_INSTRUCTIONS: &str = indoc! {r#"
    You are a code completion assistant that analyzes edit history to identify and systematically complete incomplete refactorings or patterns across the entire codebase.

    ## Edit History

    "#};

const MINIMAL_PROMPT_REMINDER: &str = indoc! {"
    ---

    Please analyze the edit history and the files, then provide the unified diff for your predicted edits.
    Do not include the cursor marker in your output.
    If you're editing multiple files, be sure to reflect filename in the hunk's header.
    "};

const XML_TAGS_INSTRUCTIONS: &str = indoc! {r#"
    # Instructions

    You are an edit prediction agent in a code editor.

    Analyze the history of edits made by the user in order to infer what they are currently trying to accomplish.
    Then complete the remainder of the current change if it is incomplete, or predict the next edit the user intends to make.
    Always continue along the user's current trajectory, rather than changing course.

    ## Output Format

    You should briefly explain your understanding of the user's overall goal in one sentence, then explain what the next change
    along the users current trajectory will be in another, and finally specify the next edit using the following XML-like format:

    <edits path="my-project/src/myapp/cli.py">
    <old_text>
    OLD TEXT 1 HERE
    </old_text>
    <new_text>
    NEW TEXT 1 HERE
    </new_text>

    <old_text>
    OLD TEXT 1 HERE
    </old_text>
    <new_text>
    NEW TEXT 1 HERE
    </new_text>
    </edits>

    - Specify the file to edit using the `path` attribute.
    - Use `<old_text>` and `<new_text>` tags to replace content
    - `<old_text>` must exactly match existing file content, including indentation
    - `<old_text>` cannot be empty
    - Do not escape quotes, newlines, or other characters within tags
    - Always close all tags properly
    - Don't include the <|user_cursor|> marker in your output.

    ## Edit History

"#};

const OLD_TEXT_NEW_TEXT_REMINDER: &str = indoc! {r#"
    ---

    Remember that the edits in the edit history have already been applied.
"#};

pub fn build_prompt(request: &predict_edits_v3::PredictEditsRequest) -> Result<String> {
    let prompt_data = PromptData {
        events: request.events.clone(),
        cursor_point: request.cursor_point,
        cursor_path: request.excerpt_path.clone(),
        included_files: request.related_files.clone(),
    };
    match request.prompt_format {
        PromptFormat::MinimalQwen => {
            return Ok(MinimalQwenPrompt.render(&prompt_data));
        }
        PromptFormat::SeedCoder1120 => {
            return Ok(SeedCoder1120Prompt.render(&prompt_data));
        }
        _ => (),
    };

    let insertions = match request.prompt_format {
        PromptFormat::Minimal | PromptFormat::OldTextNewText => {
            vec![(request.cursor_point, CURSOR_MARKER)]
        }
        PromptFormat::OnlySnippets => vec![],
        PromptFormat::MinimalQwen => unreachable!(),
        PromptFormat::SeedCoder1120 => unreachable!(),
    };

    let mut prompt = match request.prompt_format {
        PromptFormat::OldTextNewText => XML_TAGS_INSTRUCTIONS.to_string(),
        PromptFormat::OnlySnippets => String::new(),
        PromptFormat::Minimal => STUDENT_MODEL_INSTRUCTIONS.to_string(),
        PromptFormat::MinimalQwen => unreachable!(),
        PromptFormat::SeedCoder1120 => unreachable!(),
    };

    if request.events.is_empty() {
        prompt.push_str("(No edit history)\n\n");
    } else {
        let edit_preamble = if request.prompt_format == PromptFormat::Minimal {
            "The following are the latest edits made by the user, from earlier to later.\n\n"
        } else {
            "Here are the latest edits made by the user, from earlier to later.\n\n"
        };
        prompt.push_str(edit_preamble);
        push_events(&mut prompt, &request.events);
    }

    let excerpts_preamble = match request.prompt_format {
        PromptFormat::Minimal => indoc! {"
             ## Part of the file under the cursor

             (The cursor marker <|user_cursor|> indicates the current user cursor position.
             The file is in current state, edits from edit history has been applied.
             We only show part of the file around the cursor.
             You can only edit exactly this part of the file.
             We prepend line numbers (e.g., `123|<actual line>`); they are not part of the file.)
             "},
        PromptFormat::OldTextNewText => indoc! {"
            ## Code Excerpts

            Here is some excerpts of code that you should take into account to predict the next edit.

            The cursor position is marked by `<|user_cursor|>` as it stands after the last edit in the history.

            In addition other excerpts are included to better understand what the edit will be, including the declaration
            or references of symbols around the cursor, or other similar code snippets that may need to be updated
            following patterns that appear in the edit history.

            Consider each of them carefully in relation to the edit history, and that the user may not have navigated
            to the next place they want to edit yet.

            Lines starting with `…` indicate omitted line ranges. These may appear inside multi-line code constructs.
        "},
        PromptFormat::OnlySnippets | PromptFormat::MinimalQwen | PromptFormat::SeedCoder1120 => {
            indoc! {"
            ## Code Excerpts

            The cursor marker <|user_cursor|> indicates the current user cursor position.
            The file is in current state, edits from edit history have been applied.
        "}
        }
    };

    prompt.push_str(excerpts_preamble);
    prompt.push('\n');

    let include_line_numbers = matches!(request.prompt_format, PromptFormat::Minimal);
    for related_file in &request.related_files {
        if request.prompt_format == PromptFormat::Minimal {
            write_codeblock_with_filename(
                &related_file.path,
                &related_file.excerpts,
                if related_file.path == request.excerpt_path {
                    &insertions
                } else {
                    &[]
                },
                related_file.max_row,
                include_line_numbers,
                &mut prompt,
            );
        } else {
            write_codeblock(
                &related_file.path,
                &related_file.excerpts,
                if related_file.path == request.excerpt_path {
                    &insertions
                } else {
                    &[]
                },
                related_file.max_row,
                include_line_numbers,
                &mut prompt,
            );
        }
    }

    match request.prompt_format {
        PromptFormat::OldTextNewText => {
            prompt.push_str(OLD_TEXT_NEW_TEXT_REMINDER);
        }
        PromptFormat::Minimal => {
            prompt.push_str(MINIMAL_PROMPT_REMINDER);
        }
        _ => {}
    }

    Ok(prompt)
}

pub fn generation_params(prompt_format: PromptFormat) -> GenerationParams {
    match prompt_format {
        PromptFormat::SeedCoder1120 => SeedCoder1120Prompt::generation_params(),
        _ => GenerationParams::default(),
    }
}

pub fn write_codeblock<'a>(
    path: &Path,
    excerpts: impl IntoIterator<Item = &'a Excerpt>,
    sorted_insertions: &[(Point, &str)],
    file_line_count: Line,
    include_line_numbers: bool,
    output: &'a mut String,
) {
    writeln!(output, "`````{}", DiffPathFmt(path)).unwrap();

    write_excerpts(
        excerpts,
        sorted_insertions,
        file_line_count,
        include_line_numbers,
        output,
    );
    write!(output, "`````\n\n").unwrap();
}

fn write_codeblock_with_filename<'a>(
    path: &Path,
    excerpts: impl IntoIterator<Item = &'a Excerpt>,
    sorted_insertions: &[(Point, &str)],
    file_line_count: Line,
    include_line_numbers: bool,
    output: &'a mut String,
) {
    writeln!(output, "`````filename={}", DiffPathFmt(path)).unwrap();

    write_excerpts(
        excerpts,
        sorted_insertions,
        file_line_count,
        include_line_numbers,
        output,
    );
    write!(output, "`````\n\n").unwrap();
}

pub fn write_excerpts<'a>(
    excerpts: impl IntoIterator<Item = &'a Excerpt>,
    sorted_insertions: &[(Point, &str)],
    file_line_count: Line,
    include_line_numbers: bool,
    output: &mut String,
) {
    let mut current_row = Line(0);
    let mut sorted_insertions = sorted_insertions.iter().peekable();

    for excerpt in excerpts {
        if excerpt.start_line > current_row {
            writeln!(output, "…").unwrap();
        }
        if excerpt.text.is_empty() {
            return;
        }

        current_row = excerpt.start_line;

        for mut line in excerpt.text.lines() {
            if include_line_numbers {
                write!(output, "{}|", current_row.0 + 1).unwrap();
            }

            while let Some((insertion_location, insertion_marker)) = sorted_insertions.peek() {
                match current_row.cmp(&insertion_location.line) {
                    cmp::Ordering::Equal => {
                        let (prefix, suffix) = line.split_at(insertion_location.column as usize);
                        output.push_str(prefix);
                        output.push_str(insertion_marker);
                        line = suffix;
                        sorted_insertions.next();
                    }
                    cmp::Ordering::Less => break,
                    cmp::Ordering::Greater => {
                        sorted_insertions.next();
                        break;
                    }
                }
            }
            output.push_str(line);
            output.push('\n');
            current_row.0 += 1;
        }
    }

    if current_row < file_line_count {
        writeln!(output, "…").unwrap();
    }
}

pub fn push_events(output: &mut String, events: &[Arc<predict_edits_v3::Event>]) {
    if events.is_empty() {
        return;
    };

    writeln!(output, "`````diff").unwrap();
    for event in events {
        writeln!(output, "{}", event).unwrap();
    }
    writeln!(output, "`````\n").unwrap();
}

struct PromptData {
    events: Vec<Arc<Event>>,
    cursor_point: Point,
    cursor_path: Arc<Path>, // TODO: make a common struct with cursor_point
    included_files: Vec<RelatedFile>,
}

#[derive(Default)]
pub struct GenerationParams {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub stop: Option<Vec<String>>,
}

trait PromptFormatter {
    fn render(&self, data: &PromptData) -> String;

    fn generation_params() -> GenerationParams {
        return GenerationParams::default();
    }
}

struct MinimalQwenPrompt;

impl PromptFormatter for MinimalQwenPrompt {
    fn render(&self, data: &PromptData) -> String {
        let edit_history = self.fmt_edit_history(data);
        let context = self.fmt_context(data);

        format!(
            "{instructions}\n\n{edit_history}\n\n{context}",
            instructions = MinimalQwenPrompt::INSTRUCTIONS,
            edit_history = edit_history,
            context = context
        )
    }
}

impl MinimalQwenPrompt {
    const INSTRUCTIONS: &str = "You are a code completion assistant that analyzes edit history to identify and systematically complete incomplete refactorings or patterns across the entire codebase.\n";

    fn fmt_edit_history(&self, data: &PromptData) -> String {
        if data.events.is_empty() {
            "(No edit history)\n\n".to_string()
        } else {
            let mut events_str = String::new();
            push_events(&mut events_str, &data.events);
            format!(
                "The following are the latest edits made by the user, from earlier to later.\n\n{}",
                events_str
            )
        }
    }

    fn fmt_context(&self, data: &PromptData) -> String {
        let mut context = String::new();
        let include_line_numbers = true;

        for related_file in &data.included_files {
            writeln!(context, "<|file_sep|>{}", DiffPathFmt(&related_file.path)).unwrap();

            if related_file.path == data.cursor_path {
                write!(context, "<|fim_prefix|>").unwrap();
                write_excerpts(
                    &related_file.excerpts,
                    &[(data.cursor_point, "<|fim_suffix|>")],
                    related_file.max_row,
                    include_line_numbers,
                    &mut context,
                );
                writeln!(context, "<|fim_middle|>").unwrap();
            } else {
                write_excerpts(
                    &related_file.excerpts,
                    &[],
                    related_file.max_row,
                    include_line_numbers,
                    &mut context,
                );
            }
        }
        context
    }
}

struct SeedCoder1120Prompt;

impl PromptFormatter for SeedCoder1120Prompt {
    fn render(&self, data: &PromptData) -> String {
        let edit_history = self.fmt_edit_history(data);
        let context = self.fmt_context(data);

        format!(
            "# Edit History:\n{edit_history}\n\n{context}",
            edit_history = edit_history,
            context = context
        )
    }

    fn generation_params() -> GenerationParams {
        GenerationParams {
            temperature: Some(0.2),
            top_p: Some(0.9),
            stop: Some(vec!["<[end_of_sentence]>".into()]),
        }
    }
}

impl SeedCoder1120Prompt {
    fn fmt_edit_history(&self, data: &PromptData) -> String {
        if data.events.is_empty() {
            "(No edit history)\n\n".to_string()
        } else {
            let mut events_str = String::new();
            push_events(&mut events_str, &data.events);
            events_str
        }
    }

    fn fmt_context(&self, data: &PromptData) -> String {
        let mut context = String::new();
        let include_line_numbers = true;

        for related_file in &data.included_files {
            writeln!(context, "# Path: {}\n", DiffPathFmt(&related_file.path)).unwrap();

            if related_file.path == data.cursor_path {
                let fim_prompt = self.fmt_fim(&related_file, data.cursor_point);
                context.push_str(&fim_prompt);
            } else {
                write_excerpts(
                    &related_file.excerpts,
                    &[],
                    related_file.max_row,
                    include_line_numbers,
                    &mut context,
                );
            }
        }
        context
    }

    fn fmt_fim(&self, file: &RelatedFile, cursor_point: Point) -> String {
        let mut buf = String::new();
        const FIM_SUFFIX: &str = "<[fim-suffix]>";
        const FIM_PREFIX: &str = "<[fim-prefix]>";
        const FIM_MIDDLE: &str = "<[fim-middle]>";
        write!(buf, "{}", FIM_PREFIX).unwrap();
        write_excerpts(
            &file.excerpts,
            &[(cursor_point, FIM_SUFFIX)],
            file.max_row,
            true,
            &mut buf,
        );

        // Swap prefix and suffix parts
        let index = buf.find(FIM_SUFFIX).unwrap();
        let prefix = &buf[..index];
        let suffix = &buf[index..];

        format!("{}{}{}", suffix, prefix, FIM_MIDDLE)
    }
}
