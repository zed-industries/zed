use collections::HashMap;
use gpui::{App, AsyncApp, Entity, EntityId};
use language::{Buffer, BufferSnapshot, Point, ToPoint as _};
use project::Project;
use std::{ops::Range, path::Path, sync::Arc};
use text::Anchor;
use zeta_prompt::{ContextSource, RelatedExcerpt, RelatedFile};

/// This module contains collectors for editable context:
/// excerpts in files that are likely to be edited.

const CURSOR_CONTEXT_LINE_COUNT: u32 = 20;
const EDIT_HISTORY_CONTEXT_LINE_COUNT: u32 = 20;

type RangesByBuffer = HashMap<EntityId, (Entity<Buffer>, Vec<EditableContextRange>)>;

#[derive(Clone)]
pub struct EditHistoryContextEntry {
    pub buffer: Entity<Buffer>,
    pub edited_range: Range<Anchor>,
}

struct EditableContextRange {
    range: Range<Anchor>,
    order: usize,
    context_source: ContextSource,
}

struct ResolvedEditableContextRange {
    range: Range<Point>,
    order: usize,
    context_sources: Vec<ContextSource>,
}

pub async fn collect_editable_context(
    project: Entity<Project>,
    active_buffer: Entity<Buffer>,
    cursor_position: Anchor,
    edit_history: Vec<EditHistoryContextEntry>,
    cx: &mut AsyncApp,
) -> anyhow::Result<Vec<RelatedFile>> {
    let mut ranges_by_buffer = RangesByBuffer::default();

    collect_current_cursor_context(&mut ranges_by_buffer, active_buffer, cursor_position, cx);
    collect_edit_history_context(&mut ranges_by_buffer, edit_history, cx);

    Ok(cx.update(|cx| {
        let project = project.read(cx);
        let mut related_files = ranges_by_buffer
            .into_values()
            .filter_map(|(buffer, ranges)| related_file_for_ranges(&project, &buffer, ranges, cx))
            .collect::<Vec<_>>();
        related_files.sort_by_key(|file| {
            file.excerpts
                .iter()
                .map(|excerpt| excerpt.order)
                .min()
                .unwrap_or(usize::MAX)
        });
        related_files
    }))
}

fn collect_current_cursor_context(
    ranges_by_buffer: &mut RangesByBuffer,
    active_buffer: Entity<Buffer>,
    cursor_position: Anchor,
    cx: &mut AsyncApp,
) {
    let cursor_range = active_buffer.read_with(cx, |buffer, _cx| {
        let snapshot = buffer.snapshot();
        expanded_anchor_range(
            &snapshot,
            cursor_position..cursor_position,
            CURSOR_CONTEXT_LINE_COUNT,
        )
    });

    push_context_range(
        ranges_by_buffer,
        active_buffer,
        cursor_range,
        0,
        ContextSource::CurrentFile,
    );
}

fn collect_edit_history_context(
    ranges_by_buffer: &mut RangesByBuffer,
    edit_history: Vec<EditHistoryContextEntry>,
    cx: &mut AsyncApp,
) {
    for (index, entry) in edit_history.into_iter().enumerate() {
        let edit_history_range = entry.buffer.read_with(cx, |buffer, _cx| {
            expanded_anchor_range(
                &buffer.snapshot(),
                entry.edited_range,
                EDIT_HISTORY_CONTEXT_LINE_COUNT,
            )
        });

        push_context_range(
            ranges_by_buffer,
            entry.buffer,
            edit_history_range,
            index + 1,
            ContextSource::EditHistory,
        );
    }
}

fn expanded_anchor_range(
    snapshot: &BufferSnapshot,
    range: Range<Anchor>,
    context_line_count: u32,
) -> Range<Anchor> {
    let start = range.start.to_point(snapshot);
    let end = range.end.to_point(snapshot);
    let start_row = start.row.saturating_sub(context_line_count);
    let end_row = end
        .row
        .saturating_add(context_line_count)
        .min(snapshot.max_point().row);
    let start = snapshot.anchor_before(Point::new(start_row, 0));
    let end = snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
    start..end
}

fn push_context_range(
    ranges_by_buffer: &mut RangesByBuffer,
    buffer: Entity<Buffer>,
    range: Range<Anchor>,
    order: usize,
    context_source: ContextSource,
) {
    ranges_by_buffer
        .entry(buffer.entity_id())
        .or_insert_with(|| (buffer.clone(), Vec::new()))
        .1
        .push(EditableContextRange {
            range,
            order,
            context_source,
        });
}

fn related_file_for_ranges(
    project: &Project,
    buffer: &Entity<Buffer>,
    ranges: Vec<EditableContextRange>,
    cx: &App,
) -> Option<RelatedFile> {
    let buffer = buffer.read(cx);
    let snapshot = buffer.snapshot();
    let file = snapshot.file()?;
    let worktree = project.worktree_for_id(file.worktree_id(cx), cx)?;
    let path: Arc<Path> = Path::new(&format!(
        "{}/{}",
        worktree.read(cx).root_name().as_unix_str(),
        file.path().as_unix_str()
    ))
    .into();

    let mut ranges = resolved_context_ranges(ranges, &snapshot);
    merge_overlapping_ranges(&mut ranges);

    let mut excerpts = ranges
        .into_iter()
        .map(|range| RelatedExcerpt {
            row_range: range.range.start.row..range.range.end.row,
            text: snapshot
                .text_for_range(range.range)
                .collect::<String>()
                .into(),
            order: range.order,
            context_sources: range.context_sources,
        })
        .collect::<Vec<_>>();
    excerpts.sort_by_key(|excerpt| excerpt.order);

    Some(RelatedFile {
        path,
        max_row: snapshot.max_point().row,
        excerpts,
        in_open_source_repo: false,
    })
}

fn resolved_context_ranges(
    ranges: Vec<EditableContextRange>,
    snapshot: &BufferSnapshot,
) -> Vec<ResolvedEditableContextRange> {
    ranges
        .into_iter()
        .filter_map(|range| {
            let start = range.range.start.to_point(snapshot);
            let end = range.range.end.to_point(snapshot);
            if start >= end {
                return None;
            }

            Some(ResolvedEditableContextRange {
                range: start..end,
                order: range.order,
                context_sources: vec![range.context_source],
            })
        })
        .collect()
}

fn merge_overlapping_ranges(ranges: &mut Vec<ResolvedEditableContextRange>) {
    ranges.sort_by_key(|range| (range.range.start, range.range.end));
    let mut merged: Vec<ResolvedEditableContextRange> = Vec::new();

    for mut range in ranges.drain(..) {
        if let Some(last_range) = merged.last_mut()
            && range.range.start <= last_range.range.end
        {
            last_range.range.end = last_range.range.end.max(range.range.end);
            last_range.order = last_range.order.min(range.order);
            for context_source in range.context_sources.drain(..) {
                push_context_source(&mut last_range.context_sources, context_source);
            }
            continue;
        }

        range
            .context_sources
            .sort_by_key(|context_source| context_source_order(*context_source));
        merged.push(range);
    }

    *ranges = merged;
}

fn push_context_source(context_sources: &mut Vec<ContextSource>, context_source: ContextSource) {
    if !context_sources.contains(&context_source) {
        context_sources.push(context_source);
        context_sources.sort_by_key(|context_source| context_source_order(*context_source));
    }
}

fn context_source_order(context_source: ContextSource) -> usize {
    match context_source {
        ContextSource::Lsp => 0,
        ContextSource::CurrentFile => 1,
        ContextSource::EditHistory => 2,
    }
}
