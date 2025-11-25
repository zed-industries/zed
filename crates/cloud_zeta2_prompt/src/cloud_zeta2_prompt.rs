//! Zeta2 prompt planning and generation code shared with cloud.
pub mod retrieval_prompt;

use anyhow::{Context as _, Result, anyhow};
use cloud_llm_client::predict_edits_v3::{
    self, DiffPathFmt, Event, Excerpt, IncludedFile, Line, Point, PromptFormat,
    ReferencedDeclaration,
};
use indoc::indoc;
use ordered_float::OrderedFloat;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Serialize;
use std::cmp;
use std::fmt::Write;
use std::sync::Arc;
use std::{cmp::Reverse, collections::BinaryHeap, ops::Range, path::Path};
use strum::{EnumIter, IntoEnumIterator};

pub const DEFAULT_MAX_PROMPT_BYTES: usize = 10 * 1024;

pub const CURSOR_MARKER: &str = "<|user_cursor|>";
/// NOTE: Differs from zed version of constant - includes a newline
pub const EDITABLE_REGION_START_MARKER_WITH_NEWLINE: &str = "<|editable_region_start|>\n";
/// NOTE: Differs from zed version of constant - includes a newline
pub const EDITABLE_REGION_END_MARKER_WITH_NEWLINE: &str = "<|editable_region_end|>\n";

// TODO: use constants for markers?
const MARKED_EXCERPT_INSTRUCTIONS: &str = indoc! {"
    You are a code completion assistant and your task is to analyze user edits and then rewrite an excerpt that the user provides, suggesting the appropriate edits within the excerpt, taking into account the cursor location.

    The excerpt to edit will be wrapped in markers <|editable_region_start|> and <|editable_region_end|>. The cursor position is marked with <|user_cursor|>.  Please respond with edited code for that region.

    Other code is provided for context, and `…` indicates when code has been skipped.

    ## Edit History

"};

const LABELED_SECTIONS_INSTRUCTIONS: &str = indoc! {r#"
    You are a code completion assistant and your task is to analyze user edits, and suggest an edit to one of the provided sections of code.

    Sections of code are grouped by file and then labeled by `<|section_N|>` (e.g `<|section_8|>`).

    The cursor position is marked with `<|user_cursor|>` and it will appear within a special section labeled `<|current_section|>`. Prefer editing the current section until no more changes are needed within it.

    Respond ONLY with the name of the section to edit on a single line, followed by all of the code that should replace that section. For example:

    <|current_section|>
    for i in 0..16 {
        println!("{i}");
    }

    ## Edit History

"#};

const NUMBERED_LINES_INSTRUCTIONS: &str = indoc! {r#"
    # Instructions

    You are an edit prediction agent in a code editor.
    Your job is to predict the next edit that the user will make,
    based on their last few edits and their current cursor location.

    ## Output Format

    You must briefly explain your understanding of the user's goal, in one
    or two sentences, and then specify their next edit in the form of a
    unified diff, like this:

    ```
    --- a/src/myapp/cli.py
    +++ b/src/myapp/cli.py
    @@ ... @@
     import os
     import time
     import sys
    +from constants import LOG_LEVEL_WARNING
    @@ ... @@
     config.headless()
     config.set_interactive(false)
    -config.set_log_level(LOG_L)
    +config.set_log_level(LOG_LEVEL_WARNING)
     config.set_use_color(True)
    ```

    ## Edit History

"#};

const STUDENT_MODEL_INSTRUCTIONS: &str = indoc! {r#"
    You are a code completion assistant that analyzes edit history to identify and systematically complete incomplete refactorings or patterns across the entire codebase.

    ## Edit History

    "#};

const UNIFIED_DIFF_REMINDER: &str = indoc! {"
    ---

    Analyze the edit history and the files, then provide the unified diff for your predicted edits.
    Do not include the cursor marker in your output.
    Your diff should include edited file paths in its file headers (lines beginning with `---` and `+++`).
    Do not include line numbers in the hunk headers, use `@@ ... @@`.
    Removed lines begin with `-`.
    Added lines begin with `+`.
    Context lines begin with an extra space.
    Context and removed lines are used to match the target edit location, so make sure to include enough of them
    to uniquely identify it amongst all excerpts of code provided.
"};

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

pub fn build_prompt(
    request: &predict_edits_v3::PredictEditsRequest,
) -> Result<(String, SectionLabels)> {
    let mut section_labels = Default::default();

    let prompt_data = PromptData {
        events: request.events.clone(),
        cursor_point: request.cursor_point,
        cursor_path: request.excerpt_path.clone(),
        included_files: request.included_files.clone(),
    };
    match request.prompt_format {
        PromptFormat::MinimalQwen => {
            return Ok((MinimalQwenPrompt.render(&prompt_data), section_labels));
        }
        PromptFormat::SeedCoder1120 => {
            return Ok((SeedCoder1120Prompt.render(&prompt_data), section_labels));
        }
        _ => (),
    };

    let mut insertions = match request.prompt_format {
        PromptFormat::MarkedExcerpt => vec![
            (
                Point {
                    line: request.excerpt_line_range.start,
                    column: 0,
                },
                EDITABLE_REGION_START_MARKER_WITH_NEWLINE,
            ),
            (request.cursor_point, CURSOR_MARKER),
            (
                Point {
                    line: request.excerpt_line_range.end,
                    column: 0,
                },
                EDITABLE_REGION_END_MARKER_WITH_NEWLINE,
            ),
        ],
        PromptFormat::LabeledSections
        | PromptFormat::NumLinesUniDiff
        | PromptFormat::Minimal
        | PromptFormat::OldTextNewText => {
            vec![(request.cursor_point, CURSOR_MARKER)]
        }
        PromptFormat::OnlySnippets => vec![],
        PromptFormat::MinimalQwen => unreachable!(),
        PromptFormat::SeedCoder1120 => unreachable!(),
    };

    let mut prompt = match request.prompt_format {
        PromptFormat::MarkedExcerpt => MARKED_EXCERPT_INSTRUCTIONS.to_string(),
        PromptFormat::LabeledSections => LABELED_SECTIONS_INSTRUCTIONS.to_string(),
        PromptFormat::NumLinesUniDiff => NUMBERED_LINES_INSTRUCTIONS.to_string(),
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
        PromptFormat::NumLinesUniDiff | PromptFormat::OldTextNewText => indoc! {"
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
        _ => indoc! {"
            ## Code Excerpts

            The cursor marker <|user_cursor|> indicates the current user cursor position.
            The file is in current state, edits from edit history have been applied.
        "},
    };

    prompt.push_str(excerpts_preamble);
    prompt.push('\n');

    if !request.referenced_declarations.is_empty() || !request.signatures.is_empty() {
        let syntax_based_prompt = SyntaxBasedPrompt::populate(request)?;
        section_labels = syntax_based_prompt.write(&mut insertions, &mut prompt)?;
    } else {
        if request.prompt_format == PromptFormat::LabeledSections {
            anyhow::bail!("PromptFormat::LabeledSections cannot be used with ContextMode::Llm");
        }

        let include_line_numbers = matches!(
            request.prompt_format,
            PromptFormat::NumLinesUniDiff | PromptFormat::Minimal
        );
        for related_file in &request.included_files {
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
    }

    match request.prompt_format {
        PromptFormat::NumLinesUniDiff => {
            prompt.push_str(UNIFIED_DIFF_REMINDER);
        }
        PromptFormat::OldTextNewText => {
            prompt.push_str(OLD_TEXT_NEW_TEXT_REMINDER);
        }
        PromptFormat::Minimal => {
            prompt.push_str(MINIMAL_PROMPT_REMINDER);
        }
        _ => {}
    }

    Ok((prompt, section_labels))
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

pub struct SyntaxBasedPrompt<'a> {
    request: &'a predict_edits_v3::PredictEditsRequest,
    /// Snippets to include in the prompt. These may overlap - they are merged / deduplicated in
    /// `to_prompt_string`.
    snippets: Vec<PlannedSnippet<'a>>,
    budget_used: usize,
}

#[derive(Clone, Debug)]
pub struct PlannedSnippet<'a> {
    path: Arc<Path>,
    range: Range<Line>,
    text: &'a str,
    // TODO: Indicate this in the output
    #[allow(dead_code)]
    text_is_truncated: bool,
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum DeclarationStyle {
    Signature,
    Declaration,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct SectionLabels {
    pub excerpt_index: usize,
    pub section_ranges: Vec<(Arc<Path>, Range<Line>)>,
}

impl<'a> SyntaxBasedPrompt<'a> {
    /// Greedy one-pass knapsack algorithm to populate the prompt plan. Does the following:
    ///
    /// Initializes a priority queue by populating it with each snippet, finding the
    /// DeclarationStyle that minimizes `score_density = score / snippet.range(style).len()`. When a
    /// "signature" snippet is popped, insert an entry for the "declaration" variant that reflects
    /// the cost of upgrade.
    ///
    /// TODO: Implement an early halting condition. One option might be to have another priority
    /// queue where the score is the size, and update it accordingly. Another option might be to
    /// have some simpler heuristic like bailing after N failed insertions, or based on how much
    /// budget is left.
    ///
    /// TODO: Has the current known sources of imprecision:
    ///
    /// * Does not consider snippet overlap when ranking. For example, it might add a field to the
    /// plan even though the containing struct is already included.
    ///
    /// * Does not consider cost of signatures when ranking snippets - this is tricky since
    /// signatures may be shared by multiple snippets.
    ///
    /// * Does not include file paths / other text when considering max_bytes.
    pub fn populate(request: &'a predict_edits_v3::PredictEditsRequest) -> Result<Self> {
        let mut this = Self {
            request,
            snippets: Vec::new(),
            budget_used: request.excerpt.len(),
        };
        let mut included_parents = FxHashSet::default();
        let additional_parents = this.additional_parent_signatures(
            &request.excerpt_path,
            request.excerpt_parent,
            &included_parents,
        )?;
        this.add_parents(&mut included_parents, additional_parents);

        let max_bytes = request.prompt_max_bytes.unwrap_or(DEFAULT_MAX_PROMPT_BYTES);

        if this.budget_used > max_bytes {
            return Err(anyhow!(
                "Excerpt + signatures size of {} already exceeds budget of {}",
                this.budget_used,
                max_bytes
            ));
        }

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        struct QueueEntry {
            score_density: OrderedFloat<f32>,
            declaration_index: usize,
            style: DeclarationStyle,
        }

        // Initialize priority queue with the best score for each snippet.
        let mut queue: BinaryHeap<QueueEntry> = BinaryHeap::new();
        for (declaration_index, declaration) in request.referenced_declarations.iter().enumerate() {
            let (style, score_density) = DeclarationStyle::iter()
                .map(|style| {
                    (
                        style,
                        OrderedFloat(declaration_score_density(&declaration, style)),
                    )
                })
                .max_by_key(|(_, score_density)| *score_density)
                .unwrap();
            queue.push(QueueEntry {
                score_density,
                declaration_index,
                style,
            });
        }

        // Knapsack selection loop
        while let Some(queue_entry) = queue.pop() {
            let Some(declaration) = request
                .referenced_declarations
                .get(queue_entry.declaration_index)
            else {
                return Err(anyhow!(
                    "Invalid declaration index {}",
                    queue_entry.declaration_index
                ));
            };

            let mut additional_bytes = declaration_size(declaration, queue_entry.style);
            if this.budget_used + additional_bytes > max_bytes {
                continue;
            }

            let additional_parents = this.additional_parent_signatures(
                &declaration.path,
                declaration.parent_index,
                &mut included_parents,
            )?;
            additional_bytes += additional_parents
                .iter()
                .map(|(_, snippet)| snippet.text.len())
                .sum::<usize>();
            if this.budget_used + additional_bytes > max_bytes {
                continue;
            }

            this.budget_used += additional_bytes;
            this.add_parents(&mut included_parents, additional_parents);
            let planned_snippet = match queue_entry.style {
                DeclarationStyle::Signature => {
                    let Some(text) = declaration.text.get(declaration.signature_range.clone())
                    else {
                        return Err(anyhow!(
                            "Invalid declaration signature_range {:?} with text.len() = {}",
                            declaration.signature_range,
                            declaration.text.len()
                        ));
                    };
                    let signature_start_line = declaration.range.start
                        + Line(
                            declaration.text[..declaration.signature_range.start]
                                .lines()
                                .count() as u32,
                        );
                    let signature_end_line = signature_start_line
                        + Line(
                            declaration.text
                                [declaration.signature_range.start..declaration.signature_range.end]
                                .lines()
                                .count() as u32,
                        );
                    let range = signature_start_line..signature_end_line;

                    PlannedSnippet {
                        path: declaration.path.clone(),
                        range,
                        text,
                        text_is_truncated: declaration.text_is_truncated,
                    }
                }
                DeclarationStyle::Declaration => PlannedSnippet {
                    path: declaration.path.clone(),
                    range: declaration.range.clone(),
                    text: &declaration.text,
                    text_is_truncated: declaration.text_is_truncated,
                },
            };
            this.snippets.push(planned_snippet);

            // When a Signature is consumed, insert an entry for Definition style.
            if queue_entry.style == DeclarationStyle::Signature {
                let signature_size = declaration_size(&declaration, DeclarationStyle::Signature);
                let declaration_size =
                    declaration_size(&declaration, DeclarationStyle::Declaration);
                let signature_score = declaration_score(&declaration, DeclarationStyle::Signature);
                let declaration_score =
                    declaration_score(&declaration, DeclarationStyle::Declaration);

                let score_diff = declaration_score - signature_score;
                let size_diff = declaration_size.saturating_sub(signature_size);
                if score_diff > 0.0001 && size_diff > 0 {
                    queue.push(QueueEntry {
                        declaration_index: queue_entry.declaration_index,
                        score_density: OrderedFloat(score_diff / (size_diff as f32)),
                        style: DeclarationStyle::Declaration,
                    });
                }
            }
        }

        anyhow::Ok(this)
    }

    fn add_parents(
        &mut self,
        included_parents: &mut FxHashSet<usize>,
        snippets: Vec<(usize, PlannedSnippet<'a>)>,
    ) {
        for (parent_index, snippet) in snippets {
            included_parents.insert(parent_index);
            self.budget_used += snippet.text.len();
            self.snippets.push(snippet);
        }
    }

    fn additional_parent_signatures(
        &self,
        path: &Arc<Path>,
        parent_index: Option<usize>,
        included_parents: &FxHashSet<usize>,
    ) -> Result<Vec<(usize, PlannedSnippet<'a>)>> {
        let mut results = Vec::new();
        self.additional_parent_signatures_impl(path, parent_index, included_parents, &mut results)?;
        Ok(results)
    }

    fn additional_parent_signatures_impl(
        &self,
        path: &Arc<Path>,
        parent_index: Option<usize>,
        included_parents: &FxHashSet<usize>,
        results: &mut Vec<(usize, PlannedSnippet<'a>)>,
    ) -> Result<()> {
        let Some(parent_index) = parent_index else {
            return Ok(());
        };
        if included_parents.contains(&parent_index) {
            return Ok(());
        }
        let Some(parent_signature) = self.request.signatures.get(parent_index) else {
            return Err(anyhow!("Invalid parent index {}", parent_index));
        };
        results.push((
            parent_index,
            PlannedSnippet {
                path: path.clone(),
                range: parent_signature.range.clone(),
                text: &parent_signature.text,
                text_is_truncated: parent_signature.text_is_truncated,
            },
        ));
        self.additional_parent_signatures_impl(
            path,
            parent_signature.parent_index,
            included_parents,
            results,
        )
    }

    /// Renders the planned context. Each file starts with "```FILE_PATH\n` and ends with triple
    /// backticks, with a newline after each file. Outputs a line with "..." between nonconsecutive
    /// chunks.
    pub fn write(
        &'a self,
        excerpt_file_insertions: &mut Vec<(Point, &'static str)>,
        prompt: &mut String,
    ) -> Result<SectionLabels> {
        let mut file_to_snippets: FxHashMap<&'a std::path::Path, Vec<&PlannedSnippet<'a>>> =
            FxHashMap::default();
        for snippet in &self.snippets {
            file_to_snippets
                .entry(&snippet.path)
                .or_default()
                .push(snippet);
        }

        // Reorder so that file with cursor comes last
        let mut file_snippets = Vec::new();
        let mut excerpt_file_snippets = Vec::new();
        for (file_path, snippets) in file_to_snippets {
            if file_path == self.request.excerpt_path.as_ref() {
                excerpt_file_snippets = snippets;
            } else {
                file_snippets.push((file_path, snippets, false));
            }
        }
        let excerpt_snippet = PlannedSnippet {
            path: self.request.excerpt_path.clone(),
            range: self.request.excerpt_line_range.clone(),
            text: &self.request.excerpt,
            text_is_truncated: false,
        };
        excerpt_file_snippets.push(&excerpt_snippet);
        file_snippets.push((&self.request.excerpt_path, excerpt_file_snippets, true));

        let section_labels =
            self.push_file_snippets(prompt, excerpt_file_insertions, file_snippets)?;

        Ok(section_labels)
    }

    fn push_file_snippets(
        &self,
        output: &mut String,
        excerpt_file_insertions: &mut Vec<(Point, &'static str)>,
        file_snippets: Vec<(&'a Path, Vec<&'a PlannedSnippet>, bool)>,
    ) -> Result<SectionLabels> {
        let mut section_ranges = Vec::new();
        let mut excerpt_index = None;

        for (file_path, mut snippets, is_excerpt_file) in file_snippets {
            snippets.sort_by_key(|s| (s.range.start, Reverse(s.range.end)));

            // TODO: What if the snippets get expanded too large to be editable?
            let mut current_snippet: Option<(&PlannedSnippet, Range<Line>)> = None;
            let mut disjoint_snippets: Vec<(&PlannedSnippet, Range<Line>)> = Vec::new();
            for snippet in snippets {
                if let Some((_, current_snippet_range)) = current_snippet.as_mut()
                    && snippet.range.start <= current_snippet_range.end
                {
                    current_snippet_range.end = current_snippet_range.end.max(snippet.range.end);
                    continue;
                }
                if let Some(current_snippet) = current_snippet.take() {
                    disjoint_snippets.push(current_snippet);
                }
                current_snippet = Some((snippet, snippet.range.clone()));
            }
            if let Some(current_snippet) = current_snippet.take() {
                disjoint_snippets.push(current_snippet);
            }

            writeln!(output, "`````path={}", file_path.display()).ok();
            let mut skipped_last_snippet = false;
            for (snippet, range) in disjoint_snippets {
                let section_index = section_ranges.len();

                match self.request.prompt_format {
                    PromptFormat::MarkedExcerpt
                    | PromptFormat::OnlySnippets
                    | PromptFormat::OldTextNewText
                    | PromptFormat::Minimal
                    | PromptFormat::NumLinesUniDiff => {
                        if range.start.0 > 0 && !skipped_last_snippet {
                            output.push_str("…\n");
                        }
                    }
                    PromptFormat::LabeledSections => {
                        if is_excerpt_file
                            && range.start <= self.request.excerpt_line_range.start
                            && range.end >= self.request.excerpt_line_range.end
                        {
                            writeln!(output, "<|current_section|>").ok();
                        } else {
                            writeln!(output, "<|section_{}|>", section_index).ok();
                        }
                    }
                    PromptFormat::MinimalQwen => unreachable!(),
                    PromptFormat::SeedCoder1120 => unreachable!(),
                }

                let push_full_snippet = |output: &mut String| {
                    if self.request.prompt_format == PromptFormat::NumLinesUniDiff {
                        for (i, line) in snippet.text.lines().enumerate() {
                            writeln!(output, "{}|{}", i as u32 + range.start.0 + 1, line)?;
                        }
                    } else {
                        output.push_str(&snippet.text);
                    }
                    anyhow::Ok(())
                };

                if is_excerpt_file {
                    if self.request.prompt_format == PromptFormat::OnlySnippets {
                        if range.start >= self.request.excerpt_line_range.start
                            && range.end <= self.request.excerpt_line_range.end
                        {
                            skipped_last_snippet = true;
                        } else {
                            skipped_last_snippet = false;
                            output.push_str(snippet.text);
                        }
                    } else if !excerpt_file_insertions.is_empty() {
                        let lines = snippet.text.lines().collect::<Vec<_>>();
                        let push_line = |output: &mut String, line_ix: usize| {
                            if self.request.prompt_format == PromptFormat::NumLinesUniDiff {
                                write!(output, "{}|", line_ix as u32 + range.start.0 + 1)?;
                            }
                            anyhow::Ok(writeln!(output, "{}", lines[line_ix])?)
                        };
                        let mut last_line_ix = 0;
                        let mut insertion_ix = 0;
                        while insertion_ix < excerpt_file_insertions.len() {
                            let (point, insertion) = &excerpt_file_insertions[insertion_ix];
                            let found = point.line >= range.start && point.line <= range.end;
                            if found {
                                excerpt_index = Some(section_index);
                                let insertion_line_ix = (point.line.0 - range.start.0) as usize;
                                for line_ix in last_line_ix..insertion_line_ix {
                                    push_line(output, line_ix)?;
                                }
                                if let Some(next_line) = lines.get(insertion_line_ix) {
                                    if self.request.prompt_format == PromptFormat::NumLinesUniDiff {
                                        write!(
                                            output,
                                            "{}|",
                                            insertion_line_ix as u32 + range.start.0 + 1
                                        )?
                                    }
                                    output.push_str(&next_line[..point.column as usize]);
                                    output.push_str(insertion);
                                    writeln!(output, "{}", &next_line[point.column as usize..])?;
                                } else {
                                    writeln!(output, "{}", insertion)?;
                                }
                                last_line_ix = insertion_line_ix + 1;
                                excerpt_file_insertions.remove(insertion_ix);
                                continue;
                            }
                            insertion_ix += 1;
                        }
                        skipped_last_snippet = false;
                        for line_ix in last_line_ix..lines.len() {
                            push_line(output, line_ix)?;
                        }
                    } else {
                        skipped_last_snippet = false;
                        push_full_snippet(output)?;
                    }
                } else {
                    skipped_last_snippet = false;
                    push_full_snippet(output)?;
                }

                section_ranges.push((snippet.path.clone(), range));
            }

            output.push_str("`````\n\n");
        }

        Ok(SectionLabels {
            // TODO: Clean this up
            excerpt_index: match self.request.prompt_format {
                PromptFormat::OnlySnippets => 0,
                _ => excerpt_index.context("bug: no snippet found for excerpt")?,
            },
            section_ranges,
        })
    }
}

fn declaration_score_density(declaration: &ReferencedDeclaration, style: DeclarationStyle) -> f32 {
    declaration_score(declaration, style) / declaration_size(declaration, style) as f32
}

fn declaration_score(declaration: &ReferencedDeclaration, style: DeclarationStyle) -> f32 {
    match style {
        DeclarationStyle::Signature => declaration.signature_score,
        DeclarationStyle::Declaration => declaration.declaration_score,
    }
}

fn declaration_size(declaration: &ReferencedDeclaration, style: DeclarationStyle) -> usize {
    match style {
        DeclarationStyle::Signature => declaration.signature_range.len(),
        DeclarationStyle::Declaration => declaration.text.len(),
    }
}

struct PromptData {
    events: Vec<Arc<Event>>,
    cursor_point: Point,
    cursor_path: Arc<Path>, // TODO: make a common struct with cursor_point
    included_files: Vec<IncludedFile>,
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

    fn fmt_fim(&self, file: &IncludedFile, cursor_point: Point) -> String {
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
