pub mod editor_lsp_test_context;
pub mod editor_test_context;

use crate::{
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
    DisplayPoint, Editor, EditorMode, MultiBuffer,
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
    let display_map = cx.new_model(|cx| DisplayMap::new(buffer, font, font_size, None, 1, 1, cx));
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
    Editor::new(EditorMode::Full, buffer, None, cx)
}

pub(crate) fn build_editor_with_project(
    project: Model<Project>,
    buffer: Model<MultiBuffer>,
    cx: &mut ViewContext<Editor>,
) -> Editor {
    Editor::new(EditorMode::Full, buffer, Some(project), cx)
}

#[cfg(any(test, feature = "test-support"))]
pub fn editor_hunks(
    editor: &Editor,
    snapshot: &DisplaySnapshot,
    cx: &mut ViewContext<'_, Editor>,
) -> Vec<(String, git::diff::DiffHunkStatus, core::ops::Range<u32>)> {
    use text::Point;

    snapshot
        .buffer_snapshot
        .git_diff_hunks_in_range(0..u32::MAX)
        .map(|hunk| {
            let display_range = Point::new(hunk.associated_range.start, 0)
                .to_display_point(snapshot)
                .row()
                ..Point::new(hunk.associated_range.end, 0)
                    .to_display_point(snapshot)
                    .row();
            let (_, buffer, _) = editor
                .buffer()
                .read(cx)
                .excerpt_containing(Point::new(hunk.associated_range.start, 0), cx)
                .expect("no excerpt for expanded buffer's hunk start");
            let diff_base = &buffer
                .read(cx)
                .diff_base()
                .expect("should have a diff base for expanded hunk")
                [hunk.diff_base_byte_range.clone()];
            (diff_base.to_owned(), hunk.status(), display_range)
        })
        .collect()
}

#[cfg(any(test, feature = "test-support"))]
pub fn expanded_hunks(
    editor: &Editor,
    snapshot: &DisplaySnapshot,
    cx: &mut ViewContext<'_, Editor>,
) -> Vec<(String, git::diff::DiffHunkStatus, core::ops::Range<u32>)> {
    editor
        .expanded_hunks
        .iter()
        .filter(|hunk| !hunk.folded)
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
            let diff_base = &buffer
                .read(cx)
                .diff_base()
                .expect("should have a diff base for expanded hunk")
                [expanded_hunk.diff_base_byte_range.clone()];
            (
                diff_base.to_owned(),
                expanded_hunk.status,
                hunk_display_range,
            )
        })
        .collect()
}

#[cfg(any(test, feature = "test-support"))]
pub fn expanded_hunks_background_highlights(
    editor: &Editor,
    snapshot: &DisplaySnapshot,
) -> Vec<core::ops::Range<u32>> {
    use itertools::Itertools;

    editor
        .highlighted_rows::<crate::GitRowHighlight>()
        .into_iter()
        .flatten()
        .map(|(range, _)| {
            range.start.to_display_point(snapshot).row()..range.end.to_display_point(snapshot).row()
        })
        .unique()
        .collect()
}
