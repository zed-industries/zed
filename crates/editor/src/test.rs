pub mod editor_lsp_test_context;
pub mod editor_test_context;

use std::sync::LazyLock;

use crate::{
    DisplayPoint, Editor, EditorMode, FoldPlaceholder, MultiBuffer,
    display_map::{DisplayMap, DisplaySnapshot, ToDisplayPoint},
};
use gpui::{
    AppContext as _, Context, Entity, Font, FontFeatures, FontStyle, FontWeight, Pixels, Window,
    font,
};
use project::Project;
use util::test::{marked_text_offsets, marked_text_ranges};

pub use crate::rust_analyzer_ext::expand_macro_recursively;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

pub fn test_font() -> Font {
    static TEST_FONT: LazyLock<Font> = LazyLock::new(|| {
        #[cfg(not(target_os = "windows"))]
        {
            font("Helvetica")
        }

        #[cfg(target_os = "windows")]
        {
            font("Courier New")
        }
    });

    TEST_FONT.clone()
}

// Returns a snapshot from text containing '|' character markers with the markers removed, and DisplayPoints for each one.
pub fn marked_display_snapshot(
    text: &str,
    cx: &mut gpui::App,
) -> (DisplaySnapshot, Vec<DisplayPoint>) {
    let (unmarked_text, markers) = marked_text_offsets(text);

    let font = Font {
        family: "Zed Plex Mono".into(),
        features: FontFeatures::default(),
        fallbacks: None,
        weight: FontWeight::default(),
        style: FontStyle::default(),
    };
    let font_size: Pixels = 14usize.into();

    let buffer = MultiBuffer::build_simple(&unmarked_text, cx);
    let display_map = cx.new(|cx| {
        DisplayMap::new(
            buffer,
            font,
            font_size,
            None,
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

pub fn select_ranges(
    editor: &mut Editor,
    marked_text: &str,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let (unmarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), unmarked_text);
    editor.change_selections(None, window, cx, |s| s.select_ranges(text_ranges));
}

#[track_caller]
pub fn assert_text_with_selections(
    editor: &mut Editor,
    marked_text: &str,
    cx: &mut Context<Editor>,
) {
    let (unmarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), unmarked_text);
    assert_eq!(editor.selections.ranges(cx), text_ranges);
}

// RA thinks this is dead code even though it is used in a whole lot of tests
#[allow(dead_code)]
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn build_editor(
    buffer: Entity<MultiBuffer>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Editor {
    Editor::new(EditorMode::full(), buffer, None, window, cx)
}

pub(crate) fn build_editor_with_project(
    project: Entity<Project>,
    buffer: Entity<MultiBuffer>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Editor {
    Editor::new(EditorMode::full(), buffer, Some(project), window, cx)
}
