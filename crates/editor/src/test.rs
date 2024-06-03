pub mod editor_lsp_test_context;
pub mod editor_test_context;

use crate::{
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
    DisplayPoint, Editor, EditorMode, FoldPlaceholder, MultiBuffer,
};
use gpui::{Context, Font, FontFeatures, FontStyle, FontWeight, Model, Pixels, ViewContext};
use project::Project;
use util::test::{marked_text_offsets, marked_text_ranges};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

// Returns a snapshot from text containing '|' character markers with the markers removed, and DisplayPoints for each one.
pub fn marked_display_snapshot(
    text: &str,
    cx: &mut gpui::AppContext,
) -> (DisplaySnapshot, Vec<DisplayPoint>) {
    let (unmarked_text, markers) = marked_text_offsets(text);

    let font = Font {
        family: "Courier".into(),
        features: FontFeatures::default(),
        weight: FontWeight::default(),
        style: FontStyle::default(),
    };
    let font_size: Pixels = 14usize.into();

    let buffer = MultiBuffer::build_simple(&unmarked_text, cx);
    let display_map = cx.new_model(|cx| {
        DisplayMap::new(
            buffer,
            font,
            font_size,
            None,
            true,
            1,
            1,
            1,
            FoldPlaceholder::test(),
            cx,
        )
    });
    let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
    let markers = markers
        .into_iter()
        .map(|offset| offset.to_display_point(&snapshot))
        .collect();

    (snapshot, markers)
}

pub fn select_ranges(editor: &mut Editor, marked_text: &str, cx: &mut ViewContext<Editor>) {
    let (unmarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), unmarked_text);
    editor.change_selections(None, cx, |s| s.select_ranges(text_ranges));
}

pub fn assert_text_with_selections(
    editor: &mut Editor,
    marked_text: &str,
    cx: &mut ViewContext<Editor>,
) {
    let (unmarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), unmarked_text);
    assert_eq!(editor.selections.ranges(cx), text_ranges);
}

// RA thinks this is dead code even though it is used in a whole lot of tests
#[allow(dead_code)]
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn build_editor(buffer: Model<MultiBuffer>, cx: &mut ViewContext<Editor>) -> Editor {
    Editor::new(EditorMode::Full, buffer, None, true, cx)
}

pub(crate) fn build_editor_with_project(
    project: Model<Project>,
    buffer: Model<MultiBuffer>,
    cx: &mut ViewContext<Editor>,
) -> Editor {
    Editor::new(EditorMode::Full, buffer, Some(project), true, cx)
}

#[cfg(any(test, feature = "test-support"))]
pub fn editor_hunks(
    editor: &Editor,
    snapshot: &DisplaySnapshot,
    cx: &mut ViewContext<'_, Editor>,
) -> Vec<(
    String,
    git::diff::DiffHunkStatus,
    std::ops::Range<crate::DisplayRow>,
)> {
    use multi_buffer::MultiBufferRow;
    use text::Point;

    use crate::hunk_status;

    snapshot
        .buffer_snapshot
        .git_diff_hunks_in_range(MultiBufferRow::MIN..MultiBufferRow::MAX)
        .map(|hunk| {
            let display_range = Point::new(hunk.associated_range.start.0, 0)
                .to_display_point(snapshot)
                .row()
                ..Point::new(hunk.associated_range.end.0, 0)
                    .to_display_point(snapshot)
                    .row();
            let (_, buffer, _) = editor
                .buffer()
                .read(cx)
                .excerpt_containing(Point::new(hunk.associated_range.start.0, 0), cx)
                .expect("no excerpt for expanded buffer's hunk start");
            let diff_base = buffer
                .read(cx)
                .diff_base()
                .expect("should have a diff base for expanded hunk")
                .slice(hunk.diff_base_byte_range.clone())
                .to_string();
            (diff_base, hunk_status(&hunk), display_range)
        })
        .collect()
}

#[cfg(any(test, feature = "test-support"))]
pub fn expanded_hunks(
    editor: &Editor,
    snapshot: &DisplaySnapshot,
    cx: &mut ViewContext<'_, Editor>,
) -> Vec<(
    String,
    git::diff::DiffHunkStatus,
    std::ops::Range<crate::DisplayRow>,
)> {
    editor
        .expanded_hunks
        .hunks(false)
        .map(|expanded_hunk| {
            let hunk_display_range = expanded_hunk
                .hunk_range
                .start
                .to_display_point(snapshot)
                .row()
                ..expanded_hunk
                    .hunk_range
                    .end
                    .to_display_point(snapshot)
                    .row();
            let (_, buffer, _) = editor
                .buffer()
                .read(cx)
                .excerpt_containing(expanded_hunk.hunk_range.start, cx)
                .expect("no excerpt for expanded buffer's hunk start");
            let diff_base = buffer
                .read(cx)
                .diff_base()
                .expect("should have a diff base for expanded hunk")
                .slice(expanded_hunk.diff_base_byte_range.clone())
                .to_string();
            (diff_base, expanded_hunk.status, hunk_display_range)
        })
        .collect()
}

#[cfg(any(test, feature = "test-support"))]
pub fn expanded_hunks_background_highlights(
    editor: &mut Editor,
    cx: &mut gpui::WindowContext,
) -> Vec<std::ops::RangeInclusive<crate::DisplayRow>> {
    use crate::DisplayRow;

    let mut highlights = Vec::new();

    let mut range_start = 0;
    let mut previous_highlighted_row = None;
    for (highlighted_row, _) in editor.highlighted_display_rows(cx) {
        match previous_highlighted_row {
            Some(previous_row) => {
                if previous_row + 1 != highlighted_row.0 {
                    highlights.push(DisplayRow(range_start)..=DisplayRow(previous_row));
                    range_start = highlighted_row.0;
                }
            }
            None => {
                range_start = highlighted_row.0;
            }
        }
        previous_highlighted_row = Some(highlighted_row.0);
    }
    if let Some(previous_row) = previous_highlighted_row {
        highlights.push(DisplayRow(range_start)..=DisplayRow(previous_row));
    }

    highlights
}
