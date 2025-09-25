//! Zeta2 prompt planning and generation code shared with cloud.

use anyhow::{Context as _, Result, anyhow};
use cloud_llm_client::predict_edits_v3::{self, Event, PromptFormat, ReferencedDeclaration};
use indoc::indoc;
use ordered_float::OrderedFloat;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Write;
use std::sync::Arc;
use std::{cmp::Reverse, collections::BinaryHeap, ops::Range, path::Path};
use strum::{EnumIter, IntoEnumIterator};

pub const DEFAULT_MAX_PROMPT_BYTES: usize = 10 * 1024;

pub const CURSOR_MARKER: &str = "<|cursor_position|>";
/// NOTE: Differs from zed version of constant - includes a newline
pub const EDITABLE_REGION_START_MARKER_WITH_NEWLINE: &str = "<|editable_region_start|>\n";
/// NOTE: Differs from zed version of constant - includes a newline
pub const EDITABLE_REGION_END_MARKER_WITH_NEWLINE: &str = "<|editable_region_end|>\n";

// TODO: use constants for markers?
const MARKED_EXCERPT_SYSTEM_PROMPT: &str = indoc! {"
    You are a code completion assistant and your task is to analyze user edits and then rewrite an excerpt that the user provides, suggesting the appropriate edits within the excerpt, taking into account the cursor location.

    The excerpt to edit will be wrapped in markers <|editable_region_start|> and <|editable_region_end|>. The cursor position is marked with <|cursor_position|>.  Please respond with edited code for that region.

    Other code is provided for context, and `…` indicates when code has been skipped.
"};

const LABELED_SECTIONS_SYSTEM_PROMPT: &str = indoc! {r#"
    You are a code completion assistant and your task is to analyze user edits, and suggest an edit to one of the provided sections of code.

    Sections of code are grouped by file and then labeled by `<|section_N|>` (e.g `<|section_8|>`).

    The cursor position is marked with `<|cursor_position|>` and it will appear within a special section labeled `<|current_section|>`. Prefer editing the current section until no more changes are needed within it.

    Respond ONLY with the name of the section to edit on a single line, followed by all of the code that should replace that section. For example:

    <|current_section|>
    for i in 0..16 {
        println!("{i}");
    }
"#};

pub struct PlannedPrompt<'a> {
    request: &'a predict_edits_v3::PredictEditsRequest,
    /// Snippets to include in the prompt. These may overlap - they are merged / deduplicated in
    /// `to_prompt_string`.
    snippets: Vec<PlannedSnippet<'a>>,
    budget_used: usize,
}

pub fn system_prompt(format: PromptFormat) -> &'static str {
    match format {
        PromptFormat::MarkedExcerpt => MARKED_EXCERPT_SYSTEM_PROMPT,
        PromptFormat::LabeledSections => LABELED_SECTIONS_SYSTEM_PROMPT,
    }
}

#[derive(Clone, Debug)]
pub struct PlannedSnippet<'a> {
    path: Arc<Path>,
    range: Range<usize>,
    text: &'a str,
    // TODO: Indicate this in the output
    #[allow(dead_code)]
    text_is_truncated: bool,
}

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum SnippetStyle {
    Signature,
    Declaration,
}

#[derive(Clone, Debug)]
pub struct SectionLabels {
    pub excerpt_index: usize,
    pub section_ranges: Vec<(Arc<Path>, Range<usize>)>,
}

impl<'a> PlannedPrompt<'a> {
    /// Greedy one-pass knapsack algorithm to populate the prompt plan. Does the following:
    ///
    /// Initializes a priority queue by populating it with each snippet, finding the SnippetStyle
    /// that minimizes `score_density = score / snippet.range(style).len()`. When a "signature"
    /// snippet is popped, insert an entry for the "declaration" variant that reflects the cost of
    /// upgrade.
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
        let mut this = PlannedPrompt {
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
            style: SnippetStyle,
        }

        // Initialize priority queue with the best score for each snippet.
        let mut queue: BinaryHeap<QueueEntry> = BinaryHeap::new();
        for (declaration_index, declaration) in request.referenced_declarations.iter().enumerate() {
            let (style, score_density) = SnippetStyle::iter()
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
                SnippetStyle::Signature => {
                    let Some(text) = declaration.text.get(declaration.signature_range.clone())
                    else {
                        return Err(anyhow!(
                            "Invalid declaration signature_range {:?} with text.len() = {}",
                            declaration.signature_range,
                            declaration.text.len()
                        ));
                    };
                    PlannedSnippet {
                        path: declaration.path.clone(),
                        range: (declaration.signature_range.start + declaration.range.start)
                            ..(declaration.signature_range.end + declaration.range.start),
                        text,
                        text_is_truncated: declaration.text_is_truncated,
                    }
                }
                SnippetStyle::Declaration => PlannedSnippet {
                    path: declaration.path.clone(),
                    range: declaration.range.clone(),
                    text: &declaration.text,
                    text_is_truncated: declaration.text_is_truncated,
                },
            };
            this.snippets.push(planned_snippet);

            // When a Signature is consumed, insert an entry for Definition style.
            if queue_entry.style == SnippetStyle::Signature {
                let signature_size = declaration_size(&declaration, SnippetStyle::Signature);
                let declaration_size = declaration_size(&declaration, SnippetStyle::Declaration);
                let signature_score = declaration_score(&declaration, SnippetStyle::Signature);
                let declaration_score = declaration_score(&declaration, SnippetStyle::Declaration);

                let score_diff = declaration_score - signature_score;
                let size_diff = declaration_size.saturating_sub(signature_size);
                if score_diff > 0.0001 && size_diff > 0 {
                    queue.push(QueueEntry {
                        declaration_index: queue_entry.declaration_index,
                        score_density: OrderedFloat(score_diff / (size_diff as f32)),
                        style: SnippetStyle::Declaration,
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
    pub fn to_prompt_string(&'a self) -> Result<(String, SectionLabels)> {
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
            range: self.request.excerpt_range.clone(),
            text: &self.request.excerpt,
            text_is_truncated: false,
        };
        excerpt_file_snippets.push(&excerpt_snippet);
        file_snippets.push((&self.request.excerpt_path, excerpt_file_snippets, true));

        let mut excerpt_file_insertions = match self.request.prompt_format {
            PromptFormat::MarkedExcerpt => vec![
                (
                    self.request.excerpt_range.start,
                    EDITABLE_REGION_START_MARKER_WITH_NEWLINE,
                ),
                (
                    self.request.excerpt_range.start + self.request.cursor_offset,
                    CURSOR_MARKER,
                ),
                (
                    self.request
                        .excerpt_range
                        .end
                        .saturating_sub(0)
                        .max(self.request.excerpt_range.start),
                    EDITABLE_REGION_END_MARKER_WITH_NEWLINE,
                ),
            ],
            PromptFormat::LabeledSections => vec![(
                self.request.excerpt_range.start + self.request.cursor_offset,
                CURSOR_MARKER,
            )],
        };

        let mut prompt = String::new();
        prompt.push_str("## User Edits\n\n");
        Self::push_events(&mut prompt, &self.request.events);

        prompt.push_str("\n## Code\n\n");
        let section_labels =
            self.push_file_snippets(&mut prompt, &mut excerpt_file_insertions, file_snippets)?;
        Ok((prompt, section_labels))
    }

    fn push_events(output: &mut String, events: &[predict_edits_v3::Event]) {
        for event in events {
            match event {
                Event::BufferChange {
                    path,
                    old_path,
                    diff,
                    predicted,
                } => {
                    if let Some(old_path) = &old_path
                        && let Some(new_path) = &path
                    {
                        if old_path != new_path {
                            writeln!(
                                output,
                                "User renamed {} to {}\n\n",
                                old_path.display(),
                                new_path.display()
                            )
                            .unwrap();
                        }
                    }

                    let path = path
                        .as_ref()
                        .map_or_else(|| "untitled".to_string(), |path| path.display().to_string());

                    if *predicted {
                        writeln!(
                            output,
                            "User accepted prediction {:?}:\n```diff\n{}\n```\n",
                            path, diff
                        )
                        .unwrap();
                    } else {
                        writeln!(output, "User edited {:?}:\n```diff\n{}\n```\n", path, diff)
                            .unwrap();
                    }
                }
            }
        }
    }

    fn push_file_snippets(
        &self,
        output: &mut String,
        excerpt_file_insertions: &mut Vec<(usize, &'static str)>,
        file_snippets: Vec<(&'a Path, Vec<&'a PlannedSnippet>, bool)>,
    ) -> Result<SectionLabels> {
        let mut section_ranges = Vec::new();
        let mut excerpt_index = None;

        for (file_path, mut snippets, is_excerpt_file) in file_snippets {
            snippets.sort_by_key(|s| (s.range.start, Reverse(s.range.end)));

            // TODO: What if the snippets get expanded too large to be editable?
            let mut current_snippet: Option<(&PlannedSnippet, Range<usize>)> = None;
            let mut disjoint_snippets: Vec<(&PlannedSnippet, Range<usize>)> = Vec::new();
            for snippet in snippets {
                if let Some((_, current_snippet_range)) = current_snippet.as_mut()
                    && snippet.range.start < current_snippet_range.end
                {
                    if snippet.range.end > current_snippet_range.end {
                        current_snippet_range.end = snippet.range.end;
                    }
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

            writeln!(output, "```{}", file_path.display()).ok();
            for (snippet, range) in disjoint_snippets {
                let section_index = section_ranges.len();

                match self.request.prompt_format {
                    PromptFormat::MarkedExcerpt => {
                        if range.start > 0 {
                            output.push_str("…\n");
                        }
                    }
                    PromptFormat::LabeledSections => {
                        if is_excerpt_file
                            && range.start <= self.request.excerpt_range.start
                            && range.end >= self.request.excerpt_range.end
                        {
                            writeln!(output, "<|current_section|>").ok();
                        } else {
                            writeln!(output, "<|section_{}|>", section_index).ok();
                        }
                    }
                }

                if is_excerpt_file {
                    excerpt_index = Some(section_index);
                    let mut last_offset = range.start;
                    let mut i = 0;
                    while i < excerpt_file_insertions.len() {
                        let (offset, insertion) = &excerpt_file_insertions[i];
                        let found = *offset >= range.start && *offset <= range.end;
                        if found {
                            output.push_str(
                                &snippet.text[last_offset - range.start..offset - range.start],
                            );
                            output.push_str(insertion);
                            last_offset = *offset;
                            excerpt_file_insertions.remove(i);
                            continue;
                        }
                        i += 1;
                    }
                    output.push_str(&snippet.text[last_offset - range.start..]);
                } else {
                    output.push_str(snippet.text);
                }

                section_ranges.push((snippet.path.clone(), range));
            }

            output.push_str("```\n\n");
        }

        Ok(SectionLabels {
            excerpt_index: excerpt_index.context("bug: no snippet found for excerpt")?,
            section_ranges,
        })
    }
}

fn declaration_score_density(declaration: &ReferencedDeclaration, style: SnippetStyle) -> f32 {
    declaration_score(declaration, style) / declaration_size(declaration, style) as f32
}

fn declaration_score(declaration: &ReferencedDeclaration, style: SnippetStyle) -> f32 {
    match style {
        SnippetStyle::Signature => declaration.signature_score,
        SnippetStyle::Declaration => declaration.declaration_score,
    }
}

fn declaration_size(declaration: &ReferencedDeclaration, style: SnippetStyle) -> usize {
    match style {
        SnippetStyle::Signature => declaration.signature_range.len(),
        SnippetStyle::Declaration => declaration.text.len(),
    }
}
