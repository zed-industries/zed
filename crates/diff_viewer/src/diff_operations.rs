use buffer_diff::DiffHunkStatusKind;
use editor::{Editor, RowHighlightOptions};
use gpui::{Context, Entity, Hsla};
use language::Point;

use crate::connector_builder::{DiffBlock, build_connector_curves};
use crate::rendering::colors::get_diff_colors;
use crate::viewer::DiffViewer;

struct DiffAdditionHighlight;
struct DiffDeletionHighlight;
struct DiffModificationHighlight;

pub fn count_lines(content: &str) -> usize {
    if content.is_empty() {
        1
    } else {
        content.split('\n').count().max(1)
    }
}

impl DiffViewer {
    fn highlight_editor_range<T: 'static>(
        editor: &Entity<Editor>,
        row_range: std::ops::Range<usize>,
        color: Hsla,
        cx: &mut Context<Self>,
    ) {
        if row_range.is_empty() {
            return;
        }

        editor.update(cx, |editor, cx| {
            let start_row = row_range.start as u32;
            let end_row = row_range.end.saturating_sub(1).max(row_range.start) as u32;

            let buffer = editor.buffer().read(cx);
            let snapshot = buffer.snapshot(cx);

            let actual_end_row = end_row.min(snapshot.max_row().0);
            let start_anchor = snapshot.anchor_before(Point::new(start_row, 0));
            let end_anchor = snapshot.anchor_before(Point::new(actual_end_row + 1, 0));

            editor.highlight_rows::<T>(
                start_anchor..end_anchor,
                color,
                RowHighlightOptions {
                    autoscroll: false,
                    include_gutter: true,
                },
                cx,
            );
        });
    }

    pub fn extract_diff_blocks(&self, cx: &Context<Self>) -> Vec<DiffBlock> {
        use git2::{DiffOptions as GitOptions, Patch as GitPatch};

        let left_text = self
            .left_buffer
            .read(cx)
            .text_for_range(Point::new(0, 0)..Point::new(u32::MAX, 0))
            .collect::<String>();
        let right_text = self
            .right_buffer
            .read(cx)
            .text_for_range(Point::new(0, 0)..Point::new(u32::MAX, 0))
            .collect::<String>();

        let mut blocks = Vec::new();
        let mut options = GitOptions::new();

        options.context_lines(0);
        options.interhunk_lines(0);
        options.patience(true);
        options.minimal(true);
        options.indent_heuristic(true);

        if let Ok(patch) = GitPatch::from_buffers(
            left_text.as_bytes(),
            None,
            right_text.as_bytes(),
            None,
            Some(&mut options),
        ) {
            for hunk_idx in 0..patch.num_hunks() {
                if let Ok((hunk, _)) = patch.hunk(hunk_idx) {
                    let left_start = hunk.old_start() as usize;
                    let left_lines = hunk.old_lines() as usize;
                    let right_start = hunk.new_start() as usize;
                    let right_lines = hunk.new_lines() as usize;

                    let left_start = left_start.saturating_sub(1);
                    let right_start = right_start.saturating_sub(1);

                    let kind = if left_lines == 0 {
                        DiffHunkStatusKind::Added
                    } else if right_lines == 0 {
                        DiffHunkStatusKind::Deleted
                    } else {
                        DiffHunkStatusKind::Modified
                    };

                    blocks.push(DiffBlock {
                        left_range: left_start..(left_start + left_lines),
                        right_range: right_start..(right_start + right_lines),
                        kind,
                    });
                }
            }
        }

        blocks
    }

    pub fn apply_diff_highlights(&mut self, cx: &mut Context<Self>) {
        let (deleted_bg, created_bg, modified_bg) = get_diff_colors(cx);

        self.left_editor.update(cx, |editor, _cx| {
            editor.clear_row_highlights::<DiffDeletionHighlight>();
            editor.clear_row_highlights::<DiffModificationHighlight>();
        });

        self.right_editor.update(cx, |editor, _cx| {
            editor.clear_row_highlights::<DiffAdditionHighlight>();
            editor.clear_row_highlights::<DiffModificationHighlight>();
        });

        for block in &self.diff_blocks {
            match block.kind {
                DiffHunkStatusKind::Deleted => {
                    Self::highlight_editor_range::<DiffDeletionHighlight>(
                        &self.left_editor,
                        block.left_range.clone(),
                        deleted_bg,
                        cx,
                    );
                }
                DiffHunkStatusKind::Added => {
                    Self::highlight_editor_range::<DiffAdditionHighlight>(
                        &self.right_editor,
                        block.right_range.clone(),
                        created_bg,
                        cx,
                    );
                }
                DiffHunkStatusKind::Modified => {
                    // Both ranges need to be cloned since they're used independently
                    Self::highlight_editor_range::<DiffModificationHighlight>(
                        &self.left_editor,
                        block.left_range.clone(),
                        modified_bg,
                        cx,
                    );
                    Self::highlight_editor_range::<DiffModificationHighlight>(
                        &self.right_editor,
                        block.right_range.clone(),
                        modified_bg,
                        cx,
                    );
                }
            }
        }
    }

    pub fn update_crushed_blocks(&mut self, cx: &mut Context<Self>) {
        let (deleted_bg, created_bg, _modified_bg) = get_diff_colors(cx);

        if !self.left_crushed_blocks.is_empty() {
            self.left_editor.update(cx, |editor, cx| {
                editor.remove_blocks(
                    self.left_crushed_blocks.clone().into_iter().collect(),
                    None,
                    cx,
                );
            });
            self.left_crushed_blocks.clear();
        }

        if !self.right_crushed_blocks.is_empty() {
            self.right_editor.update(cx, |editor, cx| {
                editor.remove_blocks(
                    self.right_crushed_blocks.clone().into_iter().collect(),
                    None,
                    cx,
                );
            });
            self.right_crushed_blocks.clear();
        }

        let mut left_crushed_positions = Vec::new();
        let mut right_crushed_positions = Vec::new();

        for curve in &self.connector_curves {
            if curve.left_crushed {
                left_crushed_positions.push(curve.focus_line);
            }
            if curve.right_crushed {
                right_crushed_positions.push(curve.focus_line);
            }
        }

        for line in left_crushed_positions {
            let anchor = self.left_line_to_anchor(line as u32, cx);
            let block_props = self.create_crushed_block_properties(anchor, created_bg);
            let block_ids = self.left_editor.update(cx, |editor, cx| {
                editor.insert_blocks([block_props], None, cx)
            });
            self.left_crushed_blocks.extend(block_ids);
        }

        for line in right_crushed_positions {
            let anchor = self.right_line_to_anchor(line as u32, cx);
            let block_props = self.create_crushed_block_properties(anchor, deleted_bg);
            let block_ids = self.right_editor.update(cx, |editor, cx| {
                editor.insert_blocks([block_props], None, cx)
            });
            self.right_crushed_blocks.extend(block_ids);
        }
    }

    pub fn refresh_diff_on_content_change(&mut self, cx: &mut Context<Self>) {
        let left_content = self
            .left_buffer
            .read(cx)
            .text_for_range(Point::new(0, 0)..Point::new(u32::MAX, 0))
            .collect::<String>();
        let right_content = self
            .right_buffer
            .read(cx)
            .text_for_range(Point::new(0, 0)..Point::new(u32::MAX, 0))
            .collect::<String>();

        self.left_total_lines = count_lines(&left_content);
        self.right_total_lines = count_lines(&right_content);

        self.diff_blocks = self.extract_diff_blocks(cx);
        self.connector_curves = build_connector_curves(&self.diff_blocks);
        self.apply_diff_highlights(cx);

        self.pending_scroll = None;
        self.left_scroll_offset = 0.0;
        self.right_scroll_offset = 0.0;
        self.left_scroll_rows = 0.0;
        self.right_scroll_rows = 0.0;

        cx.notify();
    }

    pub fn handle_revert_block(&mut self, block_index: usize, cx: &mut Context<Self>) {
        if let Some(block) = self.diff_blocks.get(block_index) {
            match block.kind {
                DiffHunkStatusKind::Modified | DiffHunkStatusKind::Deleted
                    if !block.left_range.is_empty() =>
                {
                    let old_content = self
                        .left_buffer
                        .read(cx)
                        .text_for_range(
                            Point::new(block.left_range.start as u32, 0)
                                ..Point::new(block.left_range.end as u32, 0),
                        )
                        .collect::<String>();

                    self.right_buffer.update(cx, |buffer, cx| {
                        let (start, end) = if block.right_range.is_empty() {
                            let insert_point = buffer
                                .anchor_before(Point::new(block.right_range.start as u32 + 1, 0));
                            (insert_point, insert_point)
                        } else {
                            let start =
                                buffer.anchor_before(Point::new(block.right_range.start as u32, 0));
                            let end =
                                buffer.anchor_after(Point::new(block.right_range.end as u32, 0));
                            (start, end)
                        };
                        buffer.edit([(start..end, old_content)], None, cx);
                    });

                    cx.notify();
                }
                DiffHunkStatusKind::Added if !block.right_range.is_empty() => {
                    self.right_buffer.update(cx, |buffer, cx| {
                        let start =
                            buffer.anchor_before(Point::new(block.right_range.start as u32, 0));
                        let end = buffer.anchor_after(Point::new(block.right_range.end as u32, 0));
                        buffer.edit([(start..end, String::new())], None, cx);
                    });

                    cx.notify();
                }
                _ => {}
            }
        }
    }
}
