use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, Entity, SharedString, Task};
use language::Point;
use language_model::LanguageModelToolResultContent;
use project::{Project, WorktreeSettings};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use text::OffsetRangeExt;
use text::ToPoint as _;

const MAX_SCOPE_LINES: usize = 42;
const PAGINATE_LIMIT: usize = 24;

/// Input used by both goto-definition and find-references tools that locate a token
/// by searching for a multi-word `context` that must contain the `token`.
/// The optional `index` disambiguates multiple occurrences.
///
/// The `context` MUST contain `token`. `index` is 0-based and selects which occurrence
/// to use when multiple matches are found.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ContextPositionInput {
    /// Project-relative path to the file containing the symbol (e.g. "src/main.rs").
    pub path: String,

    /// A multi-word snippet from the file which explicitly contains `token`.
    /// The tool will search the file for this exact snippet to locate the token's position.
    pub context: String,

    /// The exact token inside `context` to locate.
    pub token: String,

    /// Optional 0-based index to disambiguate multiple matches.
    #[serde(default)]
    pub index: Option<u32>,
}

/// Tool: goto_definition_by_context
pub struct GotoDefinitionByContextTool {
    project: Entity<Project>,
}

impl GotoDefinitionByContextTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for GotoDefinitionByContextTool {
    type Input = ContextPositionInput;
    type Output = LanguageModelToolResultContent;

    fn name() -> &'static str {
        "goto_definition_by_context"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Goto definition for `{}` in `{}`", input.token, input.path).into()
        } else {
            "Goto definition by context".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        // Validate early
        if !input.context.contains(&input.token) {
            return Task::ready(Err(anyhow!(
                "The provided `context` must contain the `token`."
            )));
        }

        let project = self.project.clone();

        // Resolve project path and perform WorktreeSettings checks on the foreground thread (cx: &mut App).
        // This avoids calling `WorktreeSettings::get_global` and similar from within the async closure.
        let project_path = match project.read(cx).find_project_path(&input.path, cx) {
            Some(p) => p.clone(),
            None => return Task::ready(Err(anyhow!("Path {} not found in project", &input.path))),
        };

        // Security checks (mirror read_file behavior)
        let global_settings = <WorktreeSettings as settings::Settings>::get_global(cx);
        if global_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches global file_scan_exclusions: {}",
                &input.path
            )));
        }
        if global_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches global private_files: {}",
                &input.path
            )));
        }
        let worktree_settings =
            <WorktreeSettings as settings::Settings>::get(Some((&project_path).into()), cx);
        if worktree_settings.is_path_excluded(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches worktree file_scan_exclusions: {}",
                &input.path
            )));
        }
        if worktree_settings.is_path_private(&project_path.path) {
            return Task::ready(Err(anyhow!(
                "Cannot read file because its path matches worktree private_files: {}",
                &input.path
            )));
        }

        // Clone project_path for the async closure and proceed on the async thread.
        let project_path_clone = project_path.clone();
        cx.spawn(async move |cx| {
            let project_path = project_path_clone;
            // Open buffer
            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })?
                .await?;

            // Ensure file exists
            if buffer.read_with(cx, |buffer, _| {
                buffer
                    .file()
                    .as_ref()
                    .is_none_or(|file| !file.disk_state().exists())
            })? {
                anyhow::bail!("{} not found", &input.path);
            }

            // Wait for parsing to be idle to get stable snapshots
            buffer
                .read_with(cx, |buffer, _| buffer.parsing_idle())?
                .await;

            // Gather candidates: find each occurrence of context, then token within it.
            // Validate each token occurrence with Tree-sitter (via snapshot.syntax_ancestor)
            // to ensure the substring corresponds to an identifier/token in the parse tree.
            let candidates: Vec<usize> = buffer.read_with(cx, |buffer, _| {
                let text = buffer.text();
                let mut found = Vec::new();
                let mut start = 0usize;
                while let Some(pos) = text[start..].find(&input.context) {
                    let ctx_start = start + pos;
                    let ctx_end = ctx_start + input.context.len();
                    let ctx_slice = &text[ctx_start..ctx_end];
                    let mut inner = 0usize;
                    while let Some(tokpos) = ctx_slice[inner..].find(&input.token) {
                        let tok_abs = ctx_start + inner + tokpos;

                        // Validate token occurrence against syntax tree.
                        let snapshot = buffer.snapshot();
                        // Convert byte offset to point
                        let pt = snapshot.offset_to_point(tok_abs);
                        let token_point = Point::new(pt.row, pt.column);
                        let token_point_end =
                            Point::new(pt.row, pt.column.saturating_add(input.token.len() as u32));
                        let token_point_range = token_point..token_point_end;

                        let mut accept = false;
                        if let Some(node) = snapshot.syntax_ancestor(token_point_range.clone()) {
                            if node.is_named() {
                                // Read node text using snapshot anchors for accurate comparison
                                let node_range = node.byte_range().to_point(&snapshot);
                                let start_anchor = Point::new(node_range.start.row, 0);
                                let end_row = node_range.end.row;
                                let end_anchor = Point::new(end_row, snapshot.line_len(end_row));
                                let node_text = snapshot
                                    .text_for_range(
                                        snapshot.anchor_before(start_anchor)
                                            ..snapshot.anchor_after(end_anchor),
                                    )
                                    .collect::<String>();
                                let node_kind = node.kind();
                                // Accept only if the node text exactly equals the token (excluding comments/strings)
                                if node_text.trim() == input.token
                                    && node_kind != "comment"
                                    && node_kind != "string"
                                {
                                    accept = true;
                                }
                            }
                        } else {
                            // No parse tree available for this region: permissive fallback — accept candidate.
                            accept = true;
                        }

                        if accept {
                            found.push(tok_abs);
                        }

                        inner += tokpos + input.token.len();
                        if inner >= ctx_slice.len() {
                            break;
                        }
                    }
                    start = ctx_start + 1;
                    if start >= text.len() {
                        break;
                    }
                }
                found
            })?;

            if candidates.is_empty() {
                anyhow::bail!("context/token not found in file");
            }

            // Choose candidate by index or error on ambiguity
            let chosen_offset = if let Some(idx) = input.index {
                let idx_usize = idx as usize;
                if idx_usize >= candidates.len() {
                    anyhow::bail!("index out of range ({} candidates)", candidates.len());
                }
                candidates[idx_usize]
            } else if candidates.len() == 1 {
                candidates[0]
            } else {
                // Ambiguous: return a helpful list so LLM can choose an index.
                let mut out = format!(
                    "Ambiguous token: found {} matches in {}:\n\n",
                    candidates.len(),
                    input.path
                );
                for (i, &off) in candidates.iter().enumerate() {
                    // For each candidate, extract a scope preview using tree-sitter if available,
                    // otherwise fallback to -10..+9 lines around the match.
                    let (row, preview) = buffer.read_with(cx, |buffer, _| {
                        let snapshot = buffer.snapshot();
                        let pt = snapshot.offset_to_point(off);
                        // Try to get enclosing syntax node for a small token range
                        let token_point = Point::new(pt.row, pt.column);
                        let token_point_end =
                            Point::new(pt.row, pt.column + input.token.len() as u32);
                        let token_range = token_point..token_point_end;
                        let preview = if let Some(node) =
                            snapshot.syntax_ancestor(token_range.clone())
                        {
                            let full_range = node.byte_range().to_point(&snapshot);
                            let span_lines =
                                full_range.end.row.saturating_sub(full_range.start.row);
                            if (span_lines as usize) <= MAX_SCOPE_LINES {
                                let start_anchor = Point::new(full_range.start.row, 0);
                                let end_row = full_range.end.row;
                                let end_anchor = Point::new(end_row, snapshot.line_len(end_row));
                                snapshot
                                    .text_for_range(
                                        snapshot.anchor_before(start_anchor)
                                            ..snapshot.anchor_after(end_anchor),
                                    )
                                    .collect::<String>()
                            } else {
                                // fallback to -10..+9 around match row
                                let start_row = full_range.start.row.saturating_sub(10);
                                let end_row =
                                    (full_range.start.row + 9).min(snapshot.max_point().row);
                                let start_anchor = Point::new(start_row, 0);
                                let end_anchor = Point::new(end_row, snapshot.line_len(end_row));
                                snapshot
                                    .text_for_range(
                                        snapshot.anchor_before(start_anchor)
                                            ..snapshot.anchor_after(end_anchor),
                                    )
                                    .collect::<String>()
                            }
                        } else {
                            // No syntax node: fallback to -10..+9 around pt
                            let start_row = pt.row.saturating_sub(10);
                            let end_row = (pt.row + 9).min(snapshot.max_point().row);
                            let start_anchor = Point::new(start_row, 0);
                            let end_anchor = Point::new(end_row, snapshot.line_len(end_row));
                            snapshot
                                .text_for_range(
                                    snapshot.anchor_before(start_anchor)
                                        ..snapshot.anchor_after(end_anchor),
                                )
                                .collect::<String>()
                        };
                        (pt.row + 1, preview)
                    })?;
                    out.push_str(&format!(
                        "[{}] L{}:\n\n```\n{}\n```\n\n",
                        i,
                        row,
                        preview.trim()
                    ));
                }
                out.push_str("\nProvide `index` (0-based) to disambiguate.");
                return Ok(LanguageModelToolResultContent::Text(Arc::from(out)));
            };

            // Convert chosen_offset to an anchor suitable for project methods
            let anchor = buffer.read_with(cx, |buffer, _| {
                let snapshot = buffer.snapshot();
                let point = snapshot.offset_to_point(chosen_offset);
                snapshot.anchor_before(point)
            })?;

            // Ask project for definitions at this anchor
            let defs_task =
                project.update(cx, |project, cx| project.definitions(&buffer, anchor, cx))?;
            let defs = defs_task.await?;

            // Format results
            let output = match defs {
                Some(loc_links) if !loc_links.is_empty() => {
                    let mut out = String::new();
                    let total = loc_links.len();
                    let page_limit = PAGINATE_LIMIT;
                    for link in loc_links.into_iter().take(page_limit) {
                        // For each LocationLink, produce a preview preferring Tree-sitter scope
                        let (start_line, end_line, preview, maybe_path) =
                            link.target.buffer.read_with(cx, |buffer, cx| {
                                let snapshot = buffer.snapshot();
                                let start_pt = link.target.range.start.to_point(&snapshot);
                                let end_pt = link.target.range.end.to_point(&snapshot);
                                // Try syntax ancestor for the target range
                                let point_start = Point::new(start_pt.row, start_pt.column);
                                let point_end = Point::new(end_pt.row, end_pt.column);
                                let preview = if let Some(node) =
                                    snapshot.syntax_ancestor(point_start..point_end)
                                {
                                    let full_range = node.byte_range().to_point(&snapshot);
                                    let span_lines =
                                        full_range.end.row.saturating_sub(full_range.start.row);
                                    if (span_lines as usize) <= MAX_SCOPE_LINES {
                                        let start_anchor = Point::new(full_range.start.row, 0);
                                        let end_row = full_range.end.row;
                                        let end_anchor =
                                            Point::new(end_row, snapshot.line_len(end_row));
                                        snapshot
                                            .text_for_range(
                                                snapshot.anchor_before(start_anchor)
                                                    ..snapshot.anchor_after(end_anchor),
                                            )
                                            .collect::<String>()
                                    } else {
                                        // fallback to -10..+9 around start_pt
                                        let start_row = start_pt.row.saturating_sub(10);
                                        let end_row =
                                            (start_pt.row + 9).min(snapshot.max_point().row);
                                        let start_anchor = Point::new(start_row, 0);
                                        let end_anchor =
                                            Point::new(end_row, snapshot.line_len(end_row));
                                        snapshot
                                            .text_for_range(
                                                snapshot.anchor_before(start_anchor)
                                                    ..snapshot.anchor_after(end_anchor),
                                            )
                                            .collect::<String>()
                                    }
                                } else {
                                    // fallback to clamped lines around start_pt
                                    let start_row = start_pt.row.saturating_sub(10);
                                    let end_row = (start_pt.row + 9).min(snapshot.max_point().row);
                                    let start_anchor = Point::new(start_row, 0);
                                    let end_anchor =
                                        Point::new(end_row, snapshot.line_len(end_row));
                                    snapshot
                                        .text_for_range(
                                            snapshot.anchor_before(start_anchor)
                                                ..snapshot.anchor_after(end_anchor),
                                        )
                                        .collect::<String>()
                                };
                                let path = buffer.file().map(|f| f.full_path(cx));
                                (start_pt.row + 1, end_pt.row + 1, preview, path)
                            })?;
                        let path_display = maybe_path
                            .as_ref()
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|| "<buffer>".to_string());
                        out.push_str(&format!(
                            "{} [L{}-{}]\n\n",
                            path_display, start_line, end_line
                        ));
                        out.push_str("```\n");
                        out.push_str(&preview);
                        out.push_str("\n```\n\n");
                    }
                    if total > page_limit {
                        out.push_str(&format!(
                            "Showing {} of {} definitions. Request additional pages to see more.\n",
                            page_limit, total
                        ));
                    }
                    out
                }
                _ => "No definitions found (or language server not capable)".to_string(),
            };

            Ok(LanguageModelToolResultContent::Text(Arc::from(output)))
        })
    }
}
