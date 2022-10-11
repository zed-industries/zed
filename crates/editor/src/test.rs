pub mod editor_lsp_test_context;
pub mod editor_test_context;

use crate::{
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
    DisplayPoint, Editor, EditorMode, MultiBuffer,
};

use gpui::{ModelHandle, ViewContext};

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
    cx: &mut gpui::MutableAppContext,
) -> (DisplaySnapshot, Vec<DisplayPoint>) {
    let (unmarked_text, markers) = marked_text_offsets(text);

    let family_id = cx.font_cache().load_family(&["Helvetica"]).unwrap();
    let font_id = cx
        .font_cache()
        .select_font(family_id, &Default::default())
        .unwrap();
    let font_size = 14.0;

    let buffer = MultiBuffer::build_simple(&unmarked_text, cx);
    let display_map =
        cx.add_model(|cx| DisplayMap::new(buffer, font_id, font_size, None, 1, 1, cx));
    let snapshot = display_map.update(cx, |map, cx| map.snapshot(cx));
    let markers = markers
        .into_iter()
        .map(|offset| offset.to_display_point(&snapshot))
        .collect();

    (snapshot, markers)
}

pub fn select_ranges(editor: &mut Editor, marked_text: &str, cx: &mut ViewContext<Editor>) {
    let (umarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), umarked_text);
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

pub(crate) fn build_editor(
    buffer: ModelHandle<MultiBuffer>,
    cx: &mut ViewContext<Editor>,
) -> Editor {
    Editor::new(EditorMode::Full, buffer, None, None, cx)
}
