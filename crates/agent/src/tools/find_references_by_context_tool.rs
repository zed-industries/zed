use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, Entity, SharedString, Task};
use language::Point;
use language_model::LanguageModelToolResultContent;
use project::{Project, WorktreeSettings};

use std::sync::Arc;
use text::OffsetRangeExt;
use text::ToPoint as _;

use super::goto_definition_by_context_tool::ContextPositionInput;

/// Tool: find_references_by_context
pub struct FindReferencesByContextTool {
    project: Entity<Project>,
}

// Config
const MAX_SCOPE_LINES: usize = 42;
const PAGINATE_LIMIT: usize = 24;

impl FindReferencesByContextTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for FindReferencesByContextTool {
    type Input = ContextPositionInput;
    type Output = LanguageModelToolResultContent;

    fn name() -> &'static str {
        "find_references_by_context"
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
            format!("Find references for `{}` in `{}`", input.token, input.path).into()
        } else {
            "Find references by context".into()
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
        let project_path = match project.read(cx).find_project_path(&input.path, cx) {
            Some(p) => p,
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

            if buffer.read_with(cx, |buffer, _| {
                buffer
                    .file()
                    .as_ref()
                    .is_none_or(|file| !file.disk_state().exists())
            })? {
                anyhow::bail!("{} not found", &input.path);
            }

            buffer
                .read_with(cx, |buffer, _| buffer.parsing_idle())?
                .await;

            // Find candidates by substring search (cheap) and then rely on tree-sitter for scope/validation.
            // Validate each token occurrence with Tree-sitter via snapshot.syntax_ancestor so that
            // substrings inside other identifiers (e.g. `is_path_excluded`) are not treated as the token.
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

                        // Validate candidate using tree-sitter via the buffer snapshot.
                        // Accept if:
                        // - there is no syntax node available (permissive fallback), OR
                        // - the ancestor node is named, not a comment/string, and its text equals the token.
                        let snapshot = buffer.snapshot();
                        let pt = snapshot.offset_to_point(tok_abs);
                        let token_point = Point::new(pt.row, pt.column);
                        let token_point_end = Point::new(
                            pt.row,
                            pt.column.saturating_add(input.token.len() as u32),
                        );
                        let token_point_range = token_point..token_point_end;

                        // debug logging removed


                        let mut accept = false;
                        if let Some(node) = snapshot.syntax_ancestor(token_point_range.clone()) {
                            if node.is_named() {
                                // Prefer a named descendant that exactly covers the token's byte range.
                                let tok_end = tok_abs + input.token.len();
                                if let Some(desc) = node.named_descendant_for_byte_range(tok_abs, tok_end) {
                                    let desc_range = desc.byte_range().to_point(&snapshot);
                                    let desc_text = snapshot
                                        .text_for_range(snapshot.anchor_before(desc_range.start)..snapshot.anchor_after(desc_range.end))
                                        .collect::<String>();
                                    let desc_kind = desc.kind();
                                    if desc_text.trim() == input.token && desc_kind != "comment" && desc_kind != "string" {
                                        accept = true;
                                    }
                                } else {
                                    // Fallback: compare the enclosing node's text (exact match).
                                    let node_range = node.byte_range().to_point(&snapshot);
                                    let node_text = snapshot
                                        .text_for_range(snapshot.anchor_before(node_range.start)..snapshot.anchor_after(node_range.end))
                                        .collect::<String>();
                                    let node_kind = node.kind();
                                    if node_text.trim() == input.token && node_kind != "comment" && node_kind != "string" {
                                        accept = true;
                                    }
                                }
                            }
                        } else {
                            // No parse tree available: permissive fallback — accept candidate.
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

            // Choose a candidate by index or produce an ambiguous listing with richer previews.
            let chosen_offset = if let Some(idx) = input.index {
                let idx_usize = idx as usize;
                if idx_usize >= candidates.len() {
                    anyhow::bail!("index out of range ({} candidates)", candidates.len());
                }
                candidates[idx_usize]
            } else if candidates.len() == 1 {
                candidates[0]
            } else {
                // Ambiguous: produce multi-line, syntax-aware previews for each match so the LLM can pick index.
                // Implementation split into two phases:
                // 1) compute preferred preview ranges (Point start/end) for each candidate and merge overlapping ranges
                // 2) extract text for each merged range once and then map candidates into those previews
                let out = buffer.read_with(cx, |buffer, _| {
                    let snapshot = buffer.snapshot();

                    // Collect candidate info: (candidate_index, token_row, preview_start_point, preview_end_point)
                    let mut candidate_infos: Vec<(usize, u32, Point, Point)> = Vec::new();
                    for (i, &off) in candidates.iter().enumerate() {
                        let pt = snapshot.offset_to_point(off);
                        let token_point = Point::new(pt.row, pt.column);
                        let token_point_end = Point::new(
                            pt.row,
                            pt.column.saturating_add(input.token.len() as u32),
                        );

                        // Default small-clamped preview around token row (-2..+2 lines)
                        let start_row = pt.row.saturating_sub(2);
                        let end_row = (pt.row + 2).min(snapshot.max_point().row);
                        let mut preview_start = Point::new(start_row, 0);
                        let mut preview_end = Point::new(end_row, snapshot.line_len(end_row));

                        // Prefer enclosing syntax node scope; climb to the largest ancestor within MAX_SCOPE_LINES
                        if let Some(node) = snapshot.syntax_ancestor(token_point..token_point_end) {
                            let mut candidate_node = node;
                            loop {
                                if let Some(parent) = candidate_node.parent() {
                                    let parent_range = parent.byte_range().to_point(&snapshot);
                                    let parent_span_lines =
                                        parent_range.end.row.saturating_sub(parent_range.start.row);
                                    if (parent_span_lines as usize) <= MAX_SCOPE_LINES {
                                        candidate_node = parent;
                                        continue;
                                    }
                                }
                                break;
                            }
                            let full_range = candidate_node.byte_range().to_point(&snapshot);
                            preview_start = Point::new(full_range.start.row, 0);
                            preview_end = Point::new(full_range.end.row, snapshot.line_len(full_range.end.row));
                        }

                        candidate_infos.push((i, pt.row, preview_start, preview_end));
                    }

                    // Sort candidate infos by preview start row to make merging easy
                    candidate_infos.sort_by_key(|(_, _tokrow, s, _e)| s.row);

                    // Merge overlapping/adjacent ranges (by row), collecting candidate indices per merged range
                    let mut merged: Vec<(u32, u32, Point, Point, Vec<usize>)> = Vec::new();
                    for (idx, _tokrow, s_pt, e_pt) in candidate_infos.iter() {
                        let srow = s_pt.row;
                        let erow = e_pt.row;
                        if let Some(last) = merged.last_mut() {
                            // if this start row intersects or touches previous range, merge it
                            if srow <= last.1 {
                                if erow > last.1 {
                                    last.1 = erow;
                                    last.3 = *e_pt;
                                }
                                last.4.push(*idx);
                                continue;
                            }
                        }
                        merged.push((srow, erow, *s_pt, *e_pt, vec![*idx]));
                    }

                    // Extract text previews for each merged range
                    let mut merged_previews: Vec<String> = Vec::new();
                    for (_srow, _erow, start_pt, end_pt, _cidxs) in merged.iter() {
                        let text = snapshot
                            .text_for_range(snapshot.anchor_before(start_pt)..snapshot.anchor_after(end_pt))
                            .collect::<String>();
                        merged_previews.push(text);
                    }

                    // Map candidate index -> merged preview index and compute display row
                    // We'll build a vec entries in candidate order for deterministic output
                    let mut candidate_to_preview: Vec<(usize, usize, u32)> = Vec::new(); // (candidate_idx, merged_idx, display_row)
                    for (merged_idx, (_srow, _erow, _s_pt, _e_pt, cidxs)) in merged.iter().enumerate() {
                        for &c in cidxs.iter() {
                            // find the token row for candidate c from candidate_infos
                            // candidate_infos was sorted; find the tuple where first element == c
                            let token_row = candidate_infos
                                .iter()
                                .find(|(ci, _, _, _)| *ci == c)
                                .map(|(_, tok_row, _, _)| *tok_row)
                                .unwrap_or(0);
                            candidate_to_preview.push((c, merged_idx, token_row + 1)); // display rows are 1-based
                        }
                    }

                    // Sort candidate_to_preview by candidate index so we output in candidate order
                    candidate_to_preview.sort_by_key(|(c, _, _)| *c);

                    // Build final ambiguous output string
                    let mut out = format!("Ambiguous token: found {} matches in {}:\n\n", candidates.len(), input.path);
                    for (_c, merged_idx, row) in candidate_to_preview.iter() {
                        let preview = &merged_previews[*merged_idx];
                        out.push_str(&format!("[{}] L{}:\n\n``` \n{}\n```\n\n", _c, row, preview.trim()));
                    }
                    out.push_str("\nProvide `index` (0-based) to disambiguate.");
                    out
                })?;
                return Ok(LanguageModelToolResultContent::Text(Arc::from(out)));
            };

            // Anchor for references
            let anchor = buffer.read_with(cx, |buffer, _| {
                let snapshot = buffer.snapshot();
                let point = snapshot.offset_to_point(chosen_offset);
                snapshot.anchor_before(point)
            })?;

            // Call project.references
            let refs_task =
                project.update(cx, |project, cx| project.references(&buffer, anchor, cx))?;
            let refs = refs_task.await?;

            let output = match refs {
                Some(locs) if !locs.is_empty() => {
                    let mut out = String::new();
                    let total = locs.len();
                    // Paginate: include up to PAGINATE_LIMIT results
                    for loc in locs.into_iter().take(PAGINATE_LIMIT) {
                        // render preview for each location using tree-sitter scope (or fallback)
                        let (start_line, end_line, preview, maybe_path) =
                            loc.buffer.read_with(cx, |buffer, cx| {
                                let snapshot = buffer.snapshot();
                                let start_pt = loc.range.start.to_point(&snapshot);
                                let end_pt = loc.range.end.to_point(&snapshot);

                                // Try syntax ancestor for the full logical scope, but climb to a larger parent
                                // when available so we return the most useful context that's still <= MAX_SCOPE_LINES.
                                if let Some(node) = snapshot.syntax_ancestor(start_pt..end_pt) {
                                    // climb to the largest ancestor within MAX_SCOPE_LINES
                                    let mut candidate = node;
                                    loop {
                                        if let Some(parent) = candidate.parent() {
                                            let parent_range = parent.byte_range().to_point(&snapshot);
                                            let parent_span_lines =
                                                parent_range.end.row.saturating_sub(parent_range.start.row);
                                            if (parent_span_lines as usize) <= MAX_SCOPE_LINES {
                                                candidate = parent;
                                                continue;
                                            }
                                        }
                                        break;
                                    }
                                    let node_range = candidate.byte_range().to_point(&snapshot);
                                    let span_lines =
                                        node_range.end.row.saturating_sub(node_range.start.row);
                                    if (span_lines as usize) <= MAX_SCOPE_LINES {
                                        let start_anchor = Point::new(node_range.start.row, 0);
                                        let end_row = node_range.end.row;
                                        let end_anchor = Point::new(end_row, snapshot.line_len(end_row));
                                        let preview = snapshot
                                            .text_for_range(
                                                snapshot.anchor_before(start_anchor)
                                                    ..snapshot.anchor_after(end_anchor),
                                            )
                                            .collect::<String>();
                                        let path = buffer.file().map(|f| f.full_path(cx));
                                        (node_range.start.row + 1, node_range.end.row + 1, preview, path)
                                    } else {
                                        // If the syntax node is too large, provide a clamped preview around node start
                                        let preview_start_row = node_range.start.row.saturating_sub(10);
                                        let preview_end_row = (node_range.start.row + 9).min(snapshot.max_point().row);
                                        let start_anchor = Point::new(preview_start_row, 0);
                                        let end_anchor = Point::new(preview_end_row, snapshot.line_len(preview_end_row));
                                        let preview = snapshot
                                            .text_for_range(
                                                snapshot.anchor_before(start_anchor)..snapshot.anchor_after(end_anchor),
                                            )
                                            .collect::<String>();
                                        let path = buffer.file().map(|f| f.full_path(cx));
                                        (node_range.start.row + 1, node_range.end.row + 1, preview, path)
                                    }
                                } else {
                                    // fallback to -10..+9 around start_pt
                                    let preview_start_row = start_pt.row.saturating_sub(10);
                                    let preview_end_row = (start_pt.row + 9).min(snapshot.max_point().row);
                                    let start_anchor = Point::new(preview_start_row, 0);
                                    let end_anchor = Point::new(preview_end_row, snapshot.line_len(preview_end_row));
                                    let preview = snapshot
                                        .text_for_range(
                                            snapshot.anchor_before(start_anchor)..snapshot.anchor_after(end_anchor),
                                        )
                                        .collect::<String>();
                                    let path = buffer.file().map(|f| f.full_path(cx));
                                    (start_pt.row + 1, end_pt.row + 1, preview, path)
                                }
                            })?;

                        let path_display = maybe_path
                            .as_ref()
                            .map(|p: &std::path::PathBuf| p.display().to_string())
                            .unwrap_or_else(|| "<buffer>".to_string());
                        out.push_str(&format!(
                            "{} [L{}-{}]\n\n",
                            path_display, start_line, end_line
                        ));
                        out.push_str("```\n");
                        out.push_str(&preview);
                        out.push_str("\n```\n\n");
                    }

                    if total > PAGINATE_LIMIT {
                        out.push_str(&format!(
                            "... ({} more results omitted) - request additional pages to see more.\n",
                            total - PAGINATE_LIMIT
                        ));
                    }

                    out
                }
                _ => "No references found (or language server not capable)".to_string(),
            };

            Ok(LanguageModelToolResultContent::Text(Arc::from(output)))
        })
    }
}
