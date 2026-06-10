use crate::{
    FormatPromptArgs, PredictionProvider,
    example::{ActualCursor, Example, ExamplePrompt},
    headless::EpAppState,
    progress::{ExampleProgress, Step},
    retrieve_context::{ContextRetrievalType, run_context_retrieval},
};
use anyhow::{Context as _, Result, anyhow};
use gpui::AsyncApp;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;
use zeta_prompt::{
    ZetaFormat, ZetaPromptInput, format_edit_history_within_budget, format_expected_output,
    format_zeta_prompt,
    hashed_regions::{self, SnippetMarkers, SnippetSource},
    max_edit_event_count_for_format, resolve_cursor_region,
};

fn resolved_excerpt_ranges_for_format(
    input: &zeta_prompt::ZetaPromptInput,
    format: ZetaFormat,
) -> (Range<usize>, Range<usize>) {
    let (_, editable_range_in_context, context_range, _) = resolve_cursor_region(input, format);
    let editable_range = (context_range.start + editable_range_in_context.start)
        ..(context_range.start + editable_range_in_context.end);
    (editable_range, context_range)
}

pub async fn run_format_prompt(
    example: &mut Example,
    args: &FormatPromptArgs,
    app_state: Arc<EpAppState>,
    example_progress: &ExampleProgress,
    cx: AsyncApp,
) -> Result<()> {
    run_context_retrieval(
        example,
        app_state.clone(),
        example_progress,
        vec![ContextRetrievalType::Lsp],
        false,
        cx.clone(),
    )
    .await?;

    let step_progress = example_progress.start(Step::FormatPrompt);

    let prompt_inputs = example
        .prompt_inputs
        .as_ref()
        .context("prompt_inputs must be set after context retrieval")?;

    match args.provider {
        PredictionProvider::Teacher(_, zeta_format)
        | PredictionProvider::TeacherNonBatching(_, zeta_format) => {
            step_progress.set_substatus("formatting teacher prompt");

            let (editable_range, context_range) =
                resolved_excerpt_ranges_for_format(prompt_inputs, zeta_format);

            let include_diagnostics = matches!(zeta_format, ZetaFormat::V0420Diagnostics);

            let prompt = TeacherPrompt::format_prompt(
                example,
                editable_range,
                context_range,
                include_diagnostics,
            );
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: None,
                rejected_output: None,
                prefill: None,
                provider: args.provider,
            });
        }
        PredictionProvider::TeacherJumps(_) | PredictionProvider::TeacherJumpsNonBatching(_) => {
            step_progress.set_substatus("formatting teacher jumps prompt");

            let prompt = TeacherJumpsPrompt::format_prompt(example, args.related_files_budget)?;
            example.prompt = Some(ExamplePrompt {
                input: prompt,
                expected_output: None,
                rejected_output: None,
                prefill: None,
                provider: args.provider,
            });
        }
        PredictionProvider::Zeta2(zeta_format) => {
            step_progress.set_substatus("formatting zeta2 prompt");

            let prompt = format_zeta_prompt(prompt_inputs, zeta_format);
            let prefill = zeta_prompt::get_prefill(prompt_inputs, zeta_format);
            let expected_output = example
                .spec
                .expected_patches_with_cursor_positions()
                .into_iter()
                .next()
                .and_then(|(expected_patch, expected_cursor_offset)| {
                    format_expected_output(
                        prompt_inputs,
                        zeta_format,
                        &expected_patch,
                        expected_cursor_offset,
                    )
                    .ok()
                });

            let rejected_output = example.spec.rejected_patch.as_ref().and_then(|patch| {
                format_expected_output(prompt_inputs, zeta_format, patch, None).ok()
            });

            example.prompt = prompt.map(|prompt| ExamplePrompt {
                input: prompt,
                expected_output,
                rejected_output,
                provider: args.provider,
                prefill: Some(prefill),
            });
        }
        _ => {
            panic!("Cannot format prompt for {:?}", args.provider);
        }
    };
    Ok(())
}

pub struct TeacherPrompt;

impl TeacherPrompt {
    pub(crate) const EDITABLE_REGION_START: &str = "<|editable_region_start|>\n";
    pub(crate) const EDITABLE_REGION_END: &str = "\n<|editable_region_end|>";
    pub(crate) const USER_CURSOR_MARKER: &str = "<|user_cursor|>";
    pub(crate) const NO_EDITS: &str = "NO_EDITS";

    /// Truncate edit history to this number of last lines
    const MAX_HISTORY_LINES: usize = 128;

    pub fn format_prompt(
        example: &Example,
        editable_range: Range<usize>,
        context_range: Range<usize>,
        include_diagnostics: bool,
    ) -> String {
        let edit_history = Self::format_edit_history(&example.spec.edit_history);
        let context = Self::format_context(example);
        let cursor_excerpt = Self::format_cursor_excerpt(example, editable_range, context_range);
        let diagnostics = include_diagnostics
            .then(|| Self::format_diagnostics(example))
            .map(|diagnostics| format!("# 4. Diagnostics\n\n{diagnostics}"));

        let prompt_template = crate::prompt_assets::get_prompt("teacher.md");
        let prompt = prompt_template
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history)
            .replace("{{diagnostics}}", diagnostics.as_deref().unwrap_or(""))
            .replace("{{cursor_excerpt}}", &cursor_excerpt);

        prompt
    }

    pub fn parse(example: &Example, response: &str) -> Result<(String, Option<ActualCursor>)> {
        // Check if the model indicated no edits are needed
        let no_edits = (String::new(), None);
        if let Some(last_codeblock) = extract_last_codeblock(&response) {
            if last_codeblock.trim() == Self::NO_EDITS {
                return Ok(no_edits);
            }
        }

        if response
            .trim_end_matches(&[' ', '\n', '`'])
            .ends_with(Self::NO_EDITS)
        {
            return Ok(no_edits);
        }

        // Extract updated (new) editable region from the model response.
        let new_editable_region = Self::extract_editable_region(&response)?;
        let cursor_offset = new_editable_region.find(Self::USER_CURSOR_MARKER);
        let mut new_editable_region = new_editable_region.replace(Self::USER_CURSOR_MARKER, "");
        let old_editable_region = Self::extract_editable_region(
            &example
                .prompt
                .as_ref()
                .context("example prompt missing")?
                .input,
        )?
        .replace(Self::USER_CURSOR_MARKER, "");

        let prompt_inputs = example
            .prompt_inputs
            .as_ref()
            .context("example is missing prompt inputs")?;

        // Normalize leading newlines: if old starts with newline but new doesn't,
        // prepend newline to new to preserve whitespace structure.
        // This handles the case where the model drops the leading blank line.
        if old_editable_region.starts_with('\n') && !new_editable_region.starts_with('\n') {
            new_editable_region.insert(0, '\n');
        }

        let excerpt = prompt_inputs.cursor_excerpt.as_ref();
        let (editable_region_offset, _) = excerpt
            .match_indices(&old_editable_region)
            .min_by_key(|(index, _)| index.abs_diff(prompt_inputs.cursor_offset_in_excerpt))
            .context("editable region not found in prompt content")?;
        let editable_region_start_line = excerpt[..editable_region_offset].matches('\n').count();

        let editable_region_lines = old_editable_region.lines().count() as u32;
        let diff = language::unified_diff_with_context(
            &old_editable_region,
            &new_editable_region,
            editable_region_start_line as u32,
            editable_region_start_line as u32,
            editable_region_lines,
        );

        let diff = indoc::formatdoc! {"
            --- a/{path}
            +++ b/{path}
            {diff}",
            path = example.spec.cursor_path.to_string_lossy(),
            diff = diff,
        };

        let actual_cursor = cursor_offset.map(|editable_region_cursor_offset| {
            ActualCursor::from_editable_region(
                &example.spec.cursor_path,
                editable_region_cursor_offset,
                &new_editable_region,
                excerpt,
                editable_region_offset,
                editable_region_start_line,
            )
        });

        Ok((diff, actual_cursor))
    }

    fn format_edit_history(edit_history: &str) -> String {
        let lines: Vec<&str> = edit_history.lines().collect();

        if lines.is_empty() {
            return "(No edit history)".to_string();
        }

        if lines.len() > Self::MAX_HISTORY_LINES {
            let truncated = lines[lines.len() - Self::MAX_HISTORY_LINES..].join("\n");
            format!("{truncated}\n[...truncated...]")
        } else {
            lines.join("\n")
        }
    }

    pub fn format_context(example: &Example) -> String {
        let related_files = example
            .prompt_inputs
            .as_ref()
            .and_then(|pi| pi.related_files.as_deref());

        let Some(related_files) = related_files else {
            return "(No context)".to_string();
        };

        if related_files.is_empty() {
            return "(No context)".to_string();
        }

        let prefix = "`````";
        let suffix = "`````\n\n";
        let max_tokens = 1024;
        zeta_prompt::format_related_files_within_budget(related_files, &prefix, &suffix, max_tokens)
    }

    fn format_cursor_excerpt(
        example: &Example,
        editable_range: Range<usize>,
        context_range: Range<usize>,
    ) -> String {
        let mut result = String::new();

        let prompt_inputs = example.prompt_inputs.as_ref().unwrap();
        let excerpt = prompt_inputs.cursor_excerpt.as_ref();
        let cursor_offset = prompt_inputs.cursor_offset_in_excerpt;

        let path_str = example.spec.cursor_path.to_string_lossy();
        result.push_str(&format!("`````{path_str}\n"));
        result.push_str(&excerpt[context_range.start..editable_range.start]);
        result.push_str(Self::EDITABLE_REGION_START);
        result.push_str(&excerpt[editable_range.start..cursor_offset]);
        result.push_str(Self::USER_CURSOR_MARKER);
        result.push_str(&excerpt[cursor_offset..editable_range.end]);
        result.push_str(Self::EDITABLE_REGION_END);
        result.push_str(&excerpt[editable_range.end..context_range.end]);
        result.push_str("\n`````");

        result
    }

    pub fn extract_editable_region(text: &str) -> Result<String> {
        let start = text
            .rfind(Self::EDITABLE_REGION_START)
            .map_or(0, |pos| pos + Self::EDITABLE_REGION_START.len());
        let end = text.rfind(Self::EDITABLE_REGION_END).unwrap_or(text.len());

        if start >= end {
            return Err(anyhow!("Invalid editable region markers"));
        }

        let region = &text[start..end];
        Ok(region.strip_suffix('\n').unwrap_or(region).to_string())
    }

    fn format_diagnostics(example: &Example) -> String {
        let Some(prompt_inputs) = example.prompt_inputs.as_ref() else {
            return "No Diagnostics".to_string();
        };

        let cursor_buffer_row = prompt_inputs.excerpt_start_row.map(|excerpt_start_row| {
            excerpt_start_row
                + prompt_inputs.cursor_excerpt[..prompt_inputs.cursor_offset_in_excerpt]
                    .bytes()
                    .filter(|byte| *byte == b'\n')
                    .count() as u32
        });
        let diagnostics = zeta_prompt::format_active_buffer_diagnostics_with_budget(
            &prompt_inputs.active_buffer_diagnostics,
            cursor_buffer_row,
            2_000,
        );

        let diagnostics = diagnostics
            .strip_prefix("<filename>diagnostics\n")
            .unwrap_or(&diagnostics);

        if diagnostics.is_empty() {
            "No Diagnostics".to_string()
        } else {
            diagnostics.to_string()
        }
    }
}

/// Teacher prompt for long-range edit prediction ("jumps"). All prompt
/// context — the cursor file and every related-file excerpt — is annotated
/// with hashed region markers (V0609HashedRegions), and the teacher may
/// output a sequence of marker-bounded edits targeting any of it.
pub struct TeacherJumpsPrompt;

struct ParsedSpanEdit {
    snippet_ix: usize,
    range: Range<usize>,
    new_text: String,
    cursor_offset_in_new_text: Option<usize>,
}

impl TeacherJumpsPrompt {
    pub(crate) const USER_CURSOR_MARKER: &str = "<|user_cursor|>";
    pub(crate) const NO_EDITS: &str = "NO_EDITS";

    const MAX_HISTORY_TOKENS: usize = 4000;

    pub const DEFAULT_RELATED_FILES_BUDGET: usize = 8192;

    pub fn format_prompt(example: &Example, related_files_budget: usize) -> Result<String> {
        let prompt_inputs = example
            .prompt_inputs
            .as_ref()
            .context("example is missing prompt inputs")?;
        let marker_table = hashed_regions::build_marker_table(prompt_inputs);

        let edit_history = Self::format_edit_history(&prompt_inputs);
        let context = Self::format_context(prompt_inputs, &marker_table, related_files_budget);
        let cursor_excerpt = Self::format_cursor_excerpt(example, prompt_inputs, &marker_table)?;

        let prompt_template = crate::prompt_assets::get_prompt("teacher_jumps.md");
        let prompt = prompt_template
            .replace("{{context}}", &context)
            .replace("{{edit_history}}", &edit_history)
            .replace("{{cursor_excerpt}}", &cursor_excerpt);

        Ok(prompt)
    }

    pub fn parse(example: &Example, response: &str) -> Result<(String, Option<ActualCursor>)> {
        let no_edits = (String::new(), None);
        if let Some(last_codeblock) = extract_last_codeblock(&response) {
            if last_codeblock.trim() == Self::NO_EDITS {
                return Ok(no_edits);
            }
        }

        if response.trim().ends_with(Self::NO_EDITS) {
            return Ok(no_edits);
        }

        let prompt_inputs = example
            .prompt_inputs
            .as_ref()
            .context("example is missing prompt inputs")?;

        let marker_table = hashed_regions::build_marker_table(prompt_inputs);
        let mut marker_index: HashMap<&str, (usize, usize)> = HashMap::new();
        for (snippet_ix, snippet) in marker_table.iter().enumerate() {
            for (id, offset) in &snippet.markers {
                marker_index.insert(id.as_str(), (snippet_ix, *offset));
            }
        }

        let codeblocks: Vec<String> = extract_all_codeblocks(response)
            .into_iter()
            .filter(|block| block.contains(hashed_regions::MARKER_TAG_PREFIX))
            .collect();
        if codeblocks.is_empty() {
            return Err(anyhow!(
                "no marker-bounded edit codeblocks found in model response"
            ));
        }

        let mut edits = Vec::new();
        for codeblock in &codeblocks {
            let (start_id, end_id, raw_new_span) = hashed_regions::extract_marker_span(codeblock)?;
            let &(start_snippet, start_byte) = marker_index
                .get(start_id.as_str())
                .with_context(|| format!("unknown start marker `{start_id}`"))?;
            let &(end_snippet, end_byte) = marker_index
                .get(end_id.as_str())
                .with_context(|| format!("unknown end marker `{end_id}`"))?;

            if start_snippet != end_snippet {
                return Err(anyhow!(
                    "markers `{start_id}` and `{end_id}` belong to different context snippets"
                ));
            }
            if start_byte >= end_byte {
                return Err(anyhow!(
                    "start marker `{start_id}` must come before end marker `{end_id}`"
                ));
            }

            let snippet_text = Self::snippet_text(prompt_inputs, &marker_table[start_snippet])?;
            let old_span = &snippet_text[start_byte..end_byte];

            let cursor_in_span = raw_new_span.find(Self::USER_CURSOR_MARKER);
            let mut new_span = raw_new_span.replace(Self::USER_CURSOR_MARKER, "");
            if old_span.ends_with('\n') && !new_span.ends_with('\n') && !new_span.is_empty() {
                new_span.push('\n');
            }
            if !old_span.ends_with('\n') && new_span.ends_with('\n') {
                new_span.pop();
            }

            // A replacement that reproduces the start of the span verbatim and
            // then stops almost always means the model quit writing before
            // reaching the end marker, not that it intends to delete the
            // omitted tail. Accepting it would silently drop code, so reject
            // the whole response. Genuine tail deletions can still be
            // expressed with an end marker placed beyond the deleted code.
            if !new_span.is_empty()
                && old_span.len() > new_span.len()
                && old_span.starts_with(&new_span)
                && !old_span[new_span.len()..].trim().is_empty()
            {
                let dropped = old_span[new_span.len()..].trim_end();
                return Err(anyhow!(
                    "edit span `{start_id}`..`{end_id}` looks truncated: the replacement \
                     matches the start of the original span and then stops before the end \
                     marker, which would silently delete:\n{dropped}"
                ));
            }

            edits.push(ParsedSpanEdit {
                snippet_ix: start_snippet,
                range: start_byte..end_byte,
                new_text: new_span,
                cursor_offset_in_new_text: cursor_in_span,
            });
        }

        // Emit one diff section per edited snippet, in the order snippets
        // first appear in the model's edit sequence.
        let mut snippet_order: Vec<usize> = Vec::new();
        for edit in &edits {
            if !snippet_order.contains(&edit.snippet_ix) {
                snippet_order.push(edit.snippet_ix);
            }
        }

        let mut diff_output = String::new();
        let mut actual_cursor = None;

        for &snippet_ix in &snippet_order {
            let snippet = &marker_table[snippet_ix];
            let mut snippet_edits: Vec<&ParsedSpanEdit> = edits
                .iter()
                .filter(|edit| edit.snippet_ix == snippet_ix)
                .collect();
            snippet_edits.sort_by_key(|edit| edit.range.start);
            for window in snippet_edits.windows(2) {
                if window[1].range.start < window[0].range.end {
                    return Err(anyhow!("edits overlap within the same context snippet"));
                }
            }

            let old_text = Self::snippet_text(prompt_inputs, snippet)?;
            let (path, start_row) =
                Self::snippet_path_and_start_row(example, prompt_inputs, snippet)?;

            let mut new_text = String::new();
            let mut position = 0;
            let mut cursor_in_new_text = None;
            for edit in &snippet_edits {
                new_text.push_str(&old_text[position..edit.range.start]);
                if let Some(cursor_offset) = edit.cursor_offset_in_new_text {
                    cursor_in_new_text = Some(new_text.len() + cursor_offset);
                }
                new_text.push_str(&edit.new_text);
                position = edit.range.end;
            }
            new_text.push_str(&old_text[position..]);

            let diff =
                language::unified_diff_with_context(old_text, &new_text, start_row, start_row, 3);
            if !diff.is_empty() {
                let path_str = path.to_string_lossy();
                writeln!(diff_output, "--- a/{path_str}")?;
                writeln!(diff_output, "+++ b/{path_str}")?;
                diff_output.push_str(&diff);
                if !diff_output.ends_with('\n') {
                    diff_output.push('\n');
                }
            }

            if actual_cursor.is_none() {
                if let Some(cursor_offset) = cursor_in_new_text {
                    actual_cursor = Some(ActualCursor::from_editable_region(
                        &path,
                        cursor_offset,
                        &new_text,
                        old_text,
                        0,
                        start_row as usize,
                    ));
                }
            }
        }

        Ok((diff_output, actual_cursor))
    }

    fn snippet_text<'a>(
        prompt_inputs: &'a ZetaPromptInput,
        snippet: &SnippetMarkers,
    ) -> Result<&'a str> {
        match snippet.source {
            SnippetSource::CursorFile => Ok(prompt_inputs.cursor_excerpt.as_ref()),
            SnippetSource::RelatedFile {
                file_ix,
                excerpt_ix,
            } => {
                let related_files = prompt_inputs
                    .related_files
                    .as_deref()
                    .context("prompt inputs are missing related files")?;
                let file = related_files
                    .get(file_ix)
                    .context("related file index out of range")?;
                let excerpt = file
                    .excerpts
                    .get(excerpt_ix)
                    .context("related excerpt index out of range")?;
                Ok(excerpt.text.as_ref())
            }
        }
    }

    fn snippet_path_and_start_row(
        example: &Example,
        prompt_inputs: &ZetaPromptInput,
        snippet: &SnippetMarkers,
    ) -> Result<(std::path::PathBuf, u32)> {
        match snippet.source {
            // Scoring applies the cursor-file patch to `cursor_excerpt`, so
            // hunk rows stay excerpt-relative (row 0 = excerpt start).
            SnippetSource::CursorFile => Ok((example.spec.cursor_path.as_ref().to_path_buf(), 0)),
            SnippetSource::RelatedFile {
                file_ix,
                excerpt_ix,
            } => {
                let related_files = prompt_inputs
                    .related_files
                    .as_deref()
                    .context("prompt inputs are missing related files")?;
                let file = related_files
                    .get(file_ix)
                    .context("related file index out of range")?;
                let excerpt = file
                    .excerpts
                    .get(excerpt_ix)
                    .context("related excerpt index out of range")?;
                Ok((
                    Self::related_file_patch_path(
                        example.spec.cursor_path.as_ref(),
                        file.path.as_ref(),
                    ),
                    excerpt.row_range.start,
                ))
            }
        }
    }

    fn related_file_patch_path(cursor_path: &Path, related_path: &Path) -> std::path::PathBuf {
        let cursor_first_component = cursor_path.components().next();
        let related_first_component = related_path.components().next();
        if related_first_component.is_some()
            && cursor_first_component != related_first_component
            && related_path.components().count() > 1
        {
            related_path.iter().skip(1).collect()
        } else {
            related_path.to_path_buf()
        }
    }

    fn format_edit_history(prompt_inputs: &ZetaPromptInput) -> String {
        format_edit_history_within_budget(
            &prompt_inputs.events,
            "",
            "",
            Self::MAX_HISTORY_TOKENS,
            max_edit_event_count_for_format(&ZetaFormat::V0327SingleFile),
        )
    }

    /// Render related files with hashed region markers, within a token
    /// budget. Mirrors `zeta_prompt::format_related_files_within_budget`,
    /// but inserts marker tags into every included excerpt.
    fn format_context(
        prompt_inputs: &ZetaPromptInput,
        marker_table: &[SnippetMarkers],
        max_tokens: usize,
    ) -> String {
        let Some(related_files) = prompt_inputs.related_files.as_deref() else {
            return "(No context)".to_string();
        };
        if related_files.is_empty() {
            return "(No context)".to_string();
        }

        let estimate_tokens = |bytes: usize| bytes / 3;

        struct RenderedExcerpt {
            file_ix: usize,
            excerpt_ix: usize,
            order: usize,
            rendered: String,
        }

        let mut candidates = Vec::new();
        for (file_ix, file) in related_files.iter().enumerate() {
            for (excerpt_ix, excerpt) in file.excerpts.iter().enumerate() {
                let markers = marker_table.iter().find_map(|snippet| {
                    (snippet.source
                        == SnippetSource::RelatedFile {
                            file_ix,
                            excerpt_ix,
                        })
                    .then_some(&snippet.markers)
                });
                let mut rendered = String::new();
                match markers {
                    Some(markers) => hashed_regions::write_snippet_with_markers(
                        &mut rendered,
                        &excerpt.text,
                        markers,
                        None,
                    ),
                    None => rendered.push_str(&excerpt.text),
                }
                if !rendered.ends_with('\n') {
                    rendered.push('\n');
                }
                if excerpt.row_range.end < file.max_row {
                    rendered.push_str("...\n");
                }
                candidates.push(RenderedExcerpt {
                    file_ix,
                    excerpt_ix,
                    order: excerpt.order,
                    rendered,
                });
            }
        }

        let file_headers: Vec<String> = related_files
            .iter()
            .map(|file| format!("`````{}\n", file.path.to_string_lossy()))
            .collect();
        let file_suffix = "`````\n\n";

        let mut selection_order: Vec<usize> = (0..candidates.len()).collect();
        selection_order.sort_by_key(|&candidate_ix| {
            let candidate = &candidates[candidate_ix];
            (candidate.order, candidate.file_ix, candidate.excerpt_ix)
        });

        let mut total_tokens = 0;
        let mut included = vec![false; candidates.len()];
        let mut file_included = vec![false; related_files.len()];
        for &candidate_ix in &selection_order {
            let candidate = &candidates[candidate_ix];
            let header_cost = if file_included[candidate.file_ix] {
                0
            } else {
                estimate_tokens(file_headers[candidate.file_ix].len() + file_suffix.len())
            };
            let excerpt_cost = estimate_tokens(candidate.rendered.len());
            if total_tokens + header_cost + excerpt_cost > max_tokens {
                break;
            }
            total_tokens += header_cost + excerpt_cost;
            file_included[candidate.file_ix] = true;
            included[candidate_ix] = true;
        }

        let mut result = String::new();
        let mut last_file_ix = None;
        for (candidate_ix, candidate) in candidates.iter().enumerate() {
            if !included[candidate_ix] {
                continue;
            }
            if last_file_ix != Some(candidate.file_ix) {
                if last_file_ix.is_some() {
                    result.push_str(file_suffix);
                }
                result.push_str(&file_headers[candidate.file_ix]);
                last_file_ix = Some(candidate.file_ix);
            }
            result.push_str(&candidate.rendered);
        }
        if last_file_ix.is_some() {
            result.push_str(file_suffix);
        }

        if result.is_empty() {
            "(No context)".to_string()
        } else {
            result
        }
    }

    fn format_cursor_excerpt(
        example: &Example,
        prompt_inputs: &ZetaPromptInput,
        marker_table: &[SnippetMarkers],
    ) -> Result<String> {
        let cursor_markers = marker_table
            .iter()
            .find_map(|snippet| {
                (snippet.source == SnippetSource::CursorFile).then_some(&snippet.markers)
            })
            .context("marker table is missing the cursor file snippet")?;

        let excerpt = prompt_inputs.cursor_excerpt.as_ref();
        let cursor_offset = prompt_inputs.cursor_offset_in_excerpt;

        let path_str = example.spec.cursor_path.to_string_lossy();
        let mut result = format!("`````{path_str}\n");
        hashed_regions::write_snippet_with_markers(
            &mut result,
            excerpt,
            cursor_markers,
            Some((cursor_offset, Self::USER_CURSOR_MARKER)),
        );
        result.push_str("\n`````");

        Ok(result)
    }
}

/// Extract the cursor excerpt from an example.
/// First tries to extract from an existing prompt, then falls back to constructing from prompt_inputs.
pub fn extract_cursor_excerpt_from_example(example: &Example) -> Option<String> {
    // If we have the original prompt, extract the cursor excerpt from it
    if let Some(prompt) = &example.prompt {
        // Find "# 3. Current File" section and extract the content
        if let Some(start) = prompt.input.find("# 3. Current File") {
            let content_start = prompt.input[start..].find('`').map(|i| start + i)?;
            let backtick_count = prompt.input[content_start..]
                .chars()
                .take_while(|&c| c == '`')
                .count();
            let content_start = content_start + backtick_count;

            // Find the path line and skip it
            let newline_pos = prompt.input[content_start..].find('\n')?;
            let text_start = content_start + newline_pos + 1;

            // Find the closing backticks
            let closing_pattern = "`".repeat(backtick_count);
            let text_end = prompt.input[text_start..].find(&closing_pattern)?;
            let cursor_excerpt = &prompt.input[text_start..text_start + text_end];

            let path_str = example.spec.cursor_path.to_string_lossy();
            return Some(format!("`````{path_str}\n{cursor_excerpt}`````"));
        }
    }

    // Fallback: construct from prompt_inputs if available
    let prompt_inputs = example.prompt_inputs.as_ref()?;
    let excerpt = prompt_inputs.cursor_excerpt.as_ref();
    let cursor_offset = prompt_inputs.cursor_offset_in_excerpt;

    // Simple fallback: just show content around cursor with markers
    let path_str = example.spec.cursor_path.to_string_lossy();
    let mut result = format!("`````{path_str}\n");
    result.push_str(TeacherPrompt::EDITABLE_REGION_START);
    result.push_str(&excerpt[..cursor_offset]);
    result.push_str(TeacherPrompt::USER_CURSOR_MARKER);
    result.push_str(&excerpt[cursor_offset..]);
    result.push_str(TeacherPrompt::EDITABLE_REGION_END);
    result.push_str("\n`````");

    Some(result)
}

/// Extract all top-level fenced codeblocks from `text`, in order.
///
/// A fence opens with 3+ backticks (optionally followed by an info string)
/// and closes with a line of at least as many backticks, so codeblocks that
/// themselves contain shorter fences are handled.
pub(crate) fn extract_all_codeblocks(text: &str) -> Vec<String> {
    let mut codeblocks = Vec::new();
    let mut current_block: Option<(usize, Vec<&str>)> = None;

    for line in text.lines() {
        match &mut current_block {
            None => {
                let backtick_count = line.chars().take_while(|&c| c == '`').count();
                if backtick_count >= 3 {
                    current_block = Some((backtick_count, Vec::new()));
                }
            }
            Some((opening_count, lines)) => {
                let trimmed = line.trim();
                if trimmed.len() >= *opening_count && trimmed.chars().all(|c| c == '`') {
                    let mut content = lines.join("\n");
                    if !content.is_empty() {
                        content.push('\n');
                    }
                    codeblocks.push(content);
                    current_block = None;
                } else {
                    lines.push(line);
                }
            }
        }
    }

    codeblocks
}

pub(crate) fn extract_last_codeblock(text: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();

    // Search from the end for a closing fence (line containing only backticks, 3+)
    let mut closing_line_idx = None;
    let mut backtick_count = 0;

    for i in (0..lines.len()).rev() {
        let line = lines[i].trim();
        if line.len() >= 3 && line.chars().all(|c| c == '`') {
            closing_line_idx = Some(i);
            backtick_count = line.len();
            break;
        }
    }

    let closing_idx = closing_line_idx?;

    // Search backwards for matching opening fence
    // Opening fence starts with same backtick count, possibly followed by language/metadata
    let opening_pattern = "`".repeat(backtick_count);

    for i in (0..closing_idx).rev() {
        let line = lines[i];
        if line.starts_with(&opening_pattern) {
            // Ensure it's exactly the right number of backticks (not more)
            let rest = &line[backtick_count..];
            if rest.is_empty() || !rest.starts_with('`') {
                // Found matching opening fence
                // Extract content between opening and closing (exclusive)
                if closing_idx > i + 1 {
                    let content = lines[i + 1..closing_idx].join("\n");
                    // Preserve trailing newline to match previous behavior
                    return Some(format!("{}\n", content));
                } else {
                    // Empty block
                    return Some(String::new());
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use zeta_prompt::multi_region;

    fn make_example(
        cursor_excerpt: &str,
        cursor_offset: usize,
        related: &[(&str, &[(&str, u32)])],
    ) -> Example {
        let related_files = related
            .iter()
            .map(|(path, excerpts)| zeta_prompt::RelatedFile {
                path: std::sync::Arc::from(std::path::Path::new(path)),
                max_row: 1000,
                excerpts: excerpts
                    .iter()
                    .map(|(text, start_row)| zeta_prompt::RelatedExcerpt {
                        row_range: *start_row..*start_row + text.matches('\n').count() as u32,
                        text: std::sync::Arc::from(*text),
                        order: 0,
                        context_source: zeta_prompt::ContextSource::CurrentFile,
                    })
                    .collect(),
                in_open_source_repo: false,
            })
            .collect();

        Example {
            spec: edit_prediction::example_spec::ExampleSpec {
                name: "test".to_string(),
                repository_url: "https://github.com/zed-industries/zed.git".to_string(),
                revision: "HEAD".to_string(),
                tags: Vec::new(),
                reasoning: None,
                uncommitted_diff: String::new(),
                recently_opened_files: Vec::new(),
                recently_viewed_files: Vec::new(),
                uncommitted_diff_contains_edit_history: false,
                cursor_path: std::sync::Arc::from(std::path::Path::new("src/main.rs")),
                cursor_position: "0:0".to_string(),
                edit_history: String::new(),
                expected_patches: Vec::new(),
                rejected_patch: None,
                telemetry: None,
                human_feedback: Vec::new(),
                rating: None,
            },
            prompt_inputs: Some(zeta_prompt::ZetaPromptInput {
                cursor_path: std::path::Path::new("src/main.rs").into(),
                cursor_excerpt: cursor_excerpt.into(),
                cursor_offset_in_excerpt: cursor_offset,
                excerpt_start_row: Some(0),
                events: Vec::new(),
                related_files: Some(related_files),
                active_buffer_diagnostics: Vec::new(),
                excerpt_ranges: zeta_prompt::ExcerptRanges::default(),
                syntax_ranges: None,
                in_open_source_repo: false,
                can_collect_data: false,
                repo_url: None,
            }),
            prompt: None,
            predictions: Vec::new(),
            score: Vec::new(),
            qa: Vec::new(),
            zed_version: None,
            state: None,
        }
    }

    #[test]
    fn test_teacher_jumps_format_prompt_markers_everywhere() {
        let example = make_example(
            "fn main() {\n    let x = 1;\n}\n",
            16,
            &[("src/lib.rs", &[("pub fn helper() {}\n", 5)])],
        );
        let prompt = TeacherJumpsPrompt::format_prompt(&example, 8192).unwrap();

        assert!(prompt.contains(TeacherJumpsPrompt::USER_CURSOR_MARKER));
        assert!(prompt.contains("`````src/main.rs\n"));
        assert!(prompt.contains("`````src/lib.rs\n"));
        // Markers in both the current file and the related excerpt.
        let marker_table =
            hashed_regions::build_marker_table(example.prompt_inputs.as_ref().unwrap());
        for snippet in &marker_table {
            for (id, _) in &snippet.markers {
                assert!(
                    prompt.contains(&hashed_regions::marker_tag(id)),
                    "prompt is missing marker {id}"
                );
            }
        }
    }

    #[test]
    fn test_teacher_jumps_parse_single_edit_in_cursor_file() {
        let example = make_example("fn main() {\n    let x = 1;\n}\n", 16, &[]);
        let marker_table =
            hashed_regions::build_marker_table(example.prompt_inputs.as_ref().unwrap());
        let cursor_markers = &marker_table[0].markers;
        let start_tag = hashed_regions::marker_tag(&cursor_markers[0].0);
        let end_tag = hashed_regions::marker_tag(&cursor_markers[cursor_markers.len() - 1].0);

        let response = format!(
            "The user is changing x.\n\n`````\n{start_tag}\nfn main() {{\n    let x = 2;<|user_cursor|>\n}}\n{end_tag}\n`````\n"
        );
        let (patch, cursor) = TeacherJumpsPrompt::parse(&example, &response).unwrap();

        assert!(patch.contains("--- a/src/main.rs"), "patch: {patch}");
        assert!(patch.contains("-    let x = 1;"), "patch: {patch}");
        assert!(patch.contains("+    let x = 2;"), "patch: {patch}");
        let cursor = cursor.unwrap();
        assert_eq!(cursor.path, "src/main.rs");
        assert_eq!(cursor.row, 1);
    }

    #[test]
    fn test_teacher_jumps_parse_sequence_across_files() {
        let example = make_example(
            "fn fetch_user_cached() {}\n",
            0,
            &[(
                "src/server.rs",
                &[("fn handle() {\n    fetch_user();\n}\n", 10)],
            )],
        );
        let marker_table =
            hashed_regions::build_marker_table(example.prompt_inputs.as_ref().unwrap());
        assert_eq!(marker_table.len(), 2);
        let related_markers = &marker_table[1].markers;
        let start_tag = hashed_regions::marker_tag(&related_markers[0].0);
        let end_tag = hashed_regions::marker_tag(&related_markers[related_markers.len() - 1].0);

        let response = format!(
            "Updating the call site to use the new name.\n\n\
             `````\n{start_tag}\nfn handle() {{\n    fetch_user_cached();\n}}\n{end_tag}\n`````\n"
        );
        let (patch, cursor) = TeacherJumpsPrompt::parse(&example, &response).unwrap();

        assert!(patch.contains("--- a/src/server.rs"), "patch: {patch}");
        assert!(patch.contains("-    fetch_user();"), "patch: {patch}");
        assert!(
            patch.contains("+    fetch_user_cached();"),
            "patch: {patch}"
        );
        // Hunk rows are file-absolute for related files (1-based in the
        // hunk header, excerpt starts at 0-based row 10).
        assert!(patch.contains("@@ -11,"), "patch: {patch}");
        assert!(cursor.is_none());
    }

    #[test]
    fn test_teacher_jumps_parse_multiple_edits_same_file() {
        let cursor_excerpt = "\
            fn alpha() {\n    one();\n}\n\nfn beta() {\n    two();\n}\n\n\
            fn gamma() {\n    three();\n}\n\nfn delta() {\n    four();\n}\n";
        let example = make_example(cursor_excerpt, 0, &[]);
        let marker_table =
            hashed_regions::build_marker_table(example.prompt_inputs.as_ref().unwrap());
        let markers = &marker_table[0].markers;
        assert!(
            markers.len() >= 3,
            "expected internal markers, got {markers:?}"
        );

        // First edit: between the first two markers; second edit: between the
        // second and last markers.
        let tag = |ix: usize| hashed_regions::marker_tag(&markers[ix].0);
        let old_first_span = &cursor_excerpt[markers[0].1..markers[1].1];
        let old_second_span = &cursor_excerpt[markers[1].1..markers[markers.len() - 1].1];
        let new_first_span = old_first_span.replace("one()", "uno()");
        let new_second_span = old_second_span.replace("four()", "cuatro()");

        let response = format!(
            "Renaming calls.\n\n`````\n{}\n{}{}\n`````\n\n`````\n{}\n{}{}\n`````\n",
            tag(0),
            new_first_span,
            tag(1),
            tag(1),
            new_second_span,
            tag(markers.len() - 1),
        );
        let (patch, _) = TeacherJumpsPrompt::parse(&example, &response).unwrap();

        assert!(patch.contains("+    uno();"), "patch: {patch}");
        assert!(patch.contains("+    cuatro();"), "patch: {patch}");
        assert_eq!(patch.matches("--- a/src/main.rs").count(), 1);
    }

    #[test]
    fn test_teacher_jumps_parse_no_edits() {
        let example = make_example("fn main() {}\n", 0, &[]);
        let (patch, cursor) =
            TeacherJumpsPrompt::parse(&example, "All good.\n\n`````\nNO_EDITS\n`````\n").unwrap();
        assert!(patch.is_empty());
        assert!(cursor.is_none());
    }

    #[test]
    fn test_teacher_jumps_parse_rejects_truncated_span() {
        let cursor_excerpt = "\
            fn alpha() {\n    one();\n}\n\nfn beta() {\n    two();\n}\n\n\
            fn gamma() {\n    three();\n}\n\nfn delta() {\n    four();\n}\n";
        let example = make_example(cursor_excerpt, 0, &[]);
        let marker_table =
            hashed_regions::build_marker_table(example.prompt_inputs.as_ref().unwrap());
        let markers = &marker_table[0].markers;
        assert!(markers.len() >= 3);
        let start_tag = hashed_regions::marker_tag(&markers[0].0);
        let end_tag = hashed_regions::marker_tag(&markers[markers.len() - 1].0);

        // The model reproduces only the head of the span and stops before the
        // end marker; accepting this would silently delete the rest.
        let head = &cursor_excerpt[markers[0].1..markers[1].1];
        let response = format!("Minor cleanup.\n\n`````\n{start_tag}\n{head}{end_tag}\n`````\n");
        let error = TeacherJumpsPrompt::parse(&example, &response).unwrap_err();
        assert!(
            error.to_string().contains("looks truncated"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn test_teacher_jumps_parse_allows_empty_span_deletion() {
        let cursor_excerpt = "\
            fn alpha() {\n    one();\n}\n\nfn beta() {\n    two();\n}\n\n\
            fn gamma() {\n    three();\n}\n\nfn delta() {\n    four();\n}\n";
        let example = make_example(cursor_excerpt, 0, &[]);
        let marker_table =
            hashed_regions::build_marker_table(example.prompt_inputs.as_ref().unwrap());
        let markers = &marker_table[0].markers;
        assert!(markers.len() >= 3);
        let start_tag = hashed_regions::marker_tag(&markers[0].0);
        let end_tag = hashed_regions::marker_tag(&markers[1].0);

        // Deleting an entire span by replacing it with nothing is fine.
        let response = format!("Removing alpha.\n\n`````\n{start_tag}\n{end_tag}\n`````\n");
        let (patch, _) = TeacherJumpsPrompt::parse(&example, &response).unwrap();
        assert!(patch.contains("-fn alpha() {"), "patch: {patch}");
    }

    #[test]
    fn test_teacher_jumps_parse_rejects_unknown_marker() {
        let example = make_example("fn main() {}\n", 0, &[]);
        let response = "`````\n<|marker_zzzz|>\nnew\n<|marker_yyyy|>\n`````\n";
        assert!(TeacherJumpsPrompt::parse(&example, response).is_err());
    }

    #[test]
    fn test_extract_all_codeblocks_multiple() {
        let text = indoc::indoc! {"
            First edit:

            `````
            block one
            `````

            Second edit:

            `````
            block two
            with ``` nested
            `````
            "};
        let blocks = extract_all_codeblocks(text);
        assert_eq!(
            blocks,
            vec![
                "block one\n".to_string(),
                "block two\nwith ``` nested\n".to_string()
            ]
        );
    }

    #[test]
    fn test_extract_last_code_block() {
        let text = indoc::indoc! {"
            Some thinking

            ```
            first block
            ```

            `````path='something' lines=1:2
            last block
            `````
            "};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(last_block, "last block\n");
    }

    #[test]
    fn test_extract_codeblock_with_nested_fences() {
        let text = indoc::indoc! {"
            `````
            content with ``` inline
            and ```python nested
            more content
            `````
            "};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(
            last_block,
            "content with ``` inline\nand ```python nested\nmore content\n"
        );
    }

    #[test]
    fn test_extract_codeblock_ignores_inline_backticks() {
        let text = indoc::indoc! {"
            `````
            here is some `code` with inline backticks
            and here```more```stuff
            `````
            "};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(
            last_block,
            "here is some `code` with inline backticks\nand here```more```stuff\n"
        );
    }

    #[test]
    fn test_extract_editable_region_old_format() {
        let text = indoc::indoc! {"
            some lines
            are
            here
            <|editable_region_start|>
            one
            two three

            <|editable_region_end|>
            more
            lines here
            "};
        let parsed = TeacherPrompt::extract_editable_region(text).unwrap();
        assert_eq!(
            parsed,
            indoc::indoc! {"
            one
            two three"}
        );
    }

    #[test]
    fn test_extract_editable_region_marker_format() {
        let text = indoc::indoc! {"
            some context
            <|marker_1|>
            one
            two three
            <|marker_2|>
            more context
            "};
        let parsed = multi_region::extract_editable_region_from_markers(text).unwrap();
        assert_eq!(parsed, "one\ntwo three");
    }

    #[test]
    fn test_extract_editable_region_multi_markers() {
        let text = indoc::indoc! {"
            prefix
            <|marker_1|>
            aaa
            bbb
            <|marker_2|>
            ccc
            ddd
            <|marker_3|>
            suffix
            "};
        let parsed = multi_region::extract_editable_region_from_markers(text).unwrap();
        // Intermediate marker and its trailing \n are stripped
        assert_eq!(parsed, "aaa\nbbb\nccc\nddd");
    }

    #[test]
    fn test_extract_last_codeblock_nested_bibtex() {
        let text = indoc::indoc! {r#"
            Looking at the edit history, I can see that a Citation section was just added.

            `````
            ## Collaborations
            Our mission is to create a 4D generative model.

            ## Citation

            If you found Unique3D helpful, please cite our report:
            ```bibtex
            @misc{wu2024unique3d,
                  title={Unique3D},
            }
            ```
            `````
            "#};
        let last_block = extract_last_codeblock(text).unwrap();
        assert_eq!(
            last_block,
            indoc::indoc! {r#"
            ## Collaborations
            Our mission is to create a 4D generative model.

            ## Citation

            If you found Unique3D helpful, please cite our report:
            ```bibtex
            @misc{wu2024unique3d,
                  title={Unique3D},
            }
            ```
            "#}
        );
    }

    #[test]
    fn test_extract_editable_region_no_markers() {
        let text = indoc::indoc! {"
            one
            two three"};
        let parsed = TeacherPrompt::extract_editable_region(text).unwrap();
        assert_eq!(
            parsed,
            indoc::indoc! {"
            one
            two three"}
        );
    }

    #[test]
    fn test_parse_no_edits_response() {
        let response = indoc::indoc! {"
            The code is already complete. There is no clear next edit to make.

            `````
            NO_EDITS
            `````
        "};
        let codeblock = extract_last_codeblock(response).unwrap();
        assert_eq!(codeblock.trim(), TeacherPrompt::NO_EDITS);
    }

    #[test]
    fn test_extract_codeblock_no_valid_block() {
        // Text with no code blocks should return None
        let text = "Just some plain text without any code blocks";
        assert!(extract_last_codeblock(text).is_none());

        // Unclosed code block should return None
        let text = indoc::indoc! {"
            ```
            unclosed block
        "};
        assert!(extract_last_codeblock(text).is_none());

        // Analysis text with nested markdown but no proper outer block
        let text = indoc::indoc! {"
            # Analysis
            Looking at this:
            ```
            some code
            ```
            But then more analysis without wrapping block
        "};
        // This should find the inner block
        let result = extract_last_codeblock(text).unwrap();
        assert_eq!(result, "some code\n");
    }

    #[test]
    fn test_extract_codeblock_no_trailing_newline() {
        // Text ending without trailing newline after closing fence
        let text = "`````\ncontent here\n`````";
        let result = extract_last_codeblock(text).unwrap();
        assert_eq!(result, "content here\n");
    }

    #[test]
    fn test_parse_no_edits_response_with_trailing_backticks() {
        let response = "NO_EDITS```";

        let parsed = TeacherPrompt::parse(
            &Example {
                spec: edit_prediction::example_spec::ExampleSpec {
                    name: "test".to_string(),
                    repository_url: "https://github.com/zed-industries/zed.git".to_string(),
                    revision: "HEAD".to_string(),
                    tags: Vec::new(),
                    reasoning: None,
                    uncommitted_diff: String::new(),
                    recently_opened_files: Vec::new(),
                    recently_viewed_files: Vec::new(),
                    uncommitted_diff_contains_edit_history: false,
                    cursor_path: std::sync::Arc::from(std::path::Path::new("src/main.rs")),
                    cursor_position: "0:0".to_string(),
                    edit_history: String::new(),
                    expected_patches: Vec::new(),
                    rejected_patch: None,
                    telemetry: None,
                    human_feedback: Vec::new(),
                    rating: None,
                },
                prompt_inputs: None,
                prompt: None,
                predictions: Vec::new(),
                score: Vec::new(),
                qa: Vec::new(),
                zed_version: None,
                state: None,
            },
            response,
        )
        .unwrap();

        assert!(parsed.0.is_empty());
        assert!(parsed.1.is_none());
    }

    #[test]
    fn test_v0327_teacher_prompt_uses_resolved_ranges() {
        let excerpt = (0..80)
            .map(|index| format!("line{index:02}\n"))
            .collect::<String>();
        let cursor_offset = excerpt.find("line40").expect("cursor line exists");
        let prompt_inputs = zeta_prompt::ZetaPromptInput {
            cursor_path: std::path::Path::new("src/main.rs").into(),
            cursor_excerpt: excerpt.clone().into(),
            cursor_offset_in_excerpt: cursor_offset,
            excerpt_start_row: None,
            events: Vec::new(),
            related_files: Some(Vec::new()),
            active_buffer_diagnostics: Vec::new(),
            excerpt_ranges: zeta_prompt::ExcerptRanges {
                editable_150: 0..32,
                editable_180: 0..32,
                editable_350: 0..32,
                editable_512: None,
                editable_150_context_350: 0..48,
                editable_180_context_350: 0..48,
                editable_350_context_150: 20..50,
                editable_350_context_512: None,
                editable_350_context_1024: None,
                context_4096: None,
                context_8192: Some(30..excerpt.len()),
            },
            syntax_ranges: None,
            in_open_source_repo: false,
            can_collect_data: false,
            repo_url: None,
        };

        let (stored_editable_range, stored_context_range) = zeta_prompt::excerpt_range_for_format(
            ZetaFormat::V0327SingleFile,
            &prompt_inputs.excerpt_ranges,
        );
        assert!(stored_context_range.start > stored_editable_range.start);

        let (editable_range, context_range) =
            resolved_excerpt_ranges_for_format(&prompt_inputs, ZetaFormat::V0327SingleFile);
        assert_eq!(context_range, 0..excerpt.len());
        assert!(editable_range.start < cursor_offset);
        assert!(editable_range.end > cursor_offset);

        let prompt = TeacherPrompt::format_prompt(
            &Example {
                spec: edit_prediction::example_spec::ExampleSpec {
                    name: "test".to_string(),
                    repository_url: "https://github.com/zed-industries/zed.git".to_string(),
                    revision: "HEAD".to_string(),
                    tags: Vec::new(),
                    reasoning: None,
                    uncommitted_diff: String::new(),
                    recently_opened_files: Vec::new(),
                    recently_viewed_files: Vec::new(),
                    uncommitted_diff_contains_edit_history: false,
                    cursor_path: std::sync::Arc::from(std::path::Path::new("src/main.rs")),
                    cursor_position: "0:0".to_string(),
                    edit_history: String::new(),
                    expected_patches: Vec::new(),
                    rejected_patch: None,
                    telemetry: None,
                    human_feedback: Vec::new(),
                    rating: None,
                },
                prompt_inputs: Some(prompt_inputs),
                prompt: None,
                predictions: Vec::new(),
                score: Vec::new(),
                qa: Vec::new(),
                zed_version: None,
                state: None,
            },
            editable_range,
            context_range,
            false,
        );

        assert!(prompt.contains(TeacherPrompt::EDITABLE_REGION_START));
        assert!(prompt.contains(TeacherPrompt::USER_CURSOR_MARKER));
        assert!(prompt.contains("line40"));
    }
}
