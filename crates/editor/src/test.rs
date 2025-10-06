pub mod editor_lsp_test_context;
pub mod editor_test_context;

use std::{rc::Rc, sync::LazyLock};

pub use crate::rust_analyzer_ext::expand_macro_recursively;
use crate::{
    DisplayPoint, Editor, EditorMode, FoldPlaceholder, MultiBuffer, SelectionEffects,
    display_map::{
        Block, BlockPlacement, CustomBlockId, DisplayMap, DisplayRow, DisplaySnapshot,
        ToDisplayPoint,
    },
};
use collections::HashMap;
use gpui::{
    AppContext as _, Context, Entity, EntityId, Font, FontFeatures, FontStyle, FontWeight, Pixels,
    VisualTestContext, Window, font, size,
};
use multi_buffer::ToPoint;
use pretty_assertions::assert_eq;
use project::{Project, project_settings::DiagnosticSeverity};
use ui::{App, BorrowAppContext, px};
use util::test::{generate_marked_text, marked_text_offsets, marked_text_ranges};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
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
#[track_caller]
pub fn marked_display_snapshot(
    text: &str,
    cx: &mut gpui::App,
) -> (DisplaySnapshot, Vec<DisplayPoint>) {
    let (unmarked_text, markers) = marked_text_offsets(text);

    let font = Font {
        family: ".ZedMono".into(),
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
            DiagnosticSeverity::Warning,
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

#[track_caller]
pub fn select_ranges(
    editor: &mut Editor,
    marked_text: &str,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let (unmarked_text, text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), unmarked_text);
    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
        s.select_ranges(text_ranges)
    });
}

#[track_caller]
pub fn assert_text_with_selections(
    editor: &mut Editor,
    marked_text: &str,
    cx: &mut Context<Editor>,
) {
    let (unmarked_text, _text_ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), unmarked_text, "text doesn't match");
    let actual = generate_marked_text(
        &editor.text(cx),
        &editor.selections.ranges(cx),
        marked_text.contains("«"),
    );
    assert_eq!(actual, marked_text, "Selections don't match");
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

#[derive(Default)]
struct TestBlockContent(
    HashMap<(EntityId, CustomBlockId), Rc<dyn Fn(&mut VisualTestContext) -> String>>,
);

impl gpui::Global for TestBlockContent {}

pub fn set_block_content_for_tests(
    editor: &Entity<Editor>,
    id: CustomBlockId,
    cx: &mut App,
    f: impl Fn(&mut VisualTestContext) -> String + 'static,
) {
    cx.update_default_global::<TestBlockContent, _>(|bc, _| {
        bc.0.insert((editor.entity_id(), id), Rc::new(f))
    });
}

pub fn block_content_for_tests(
    editor: &Entity<Editor>,
    id: CustomBlockId,
    cx: &mut VisualTestContext,
) -> Option<String> {
    let f = cx.update(|_, cx| {
        cx.default_global::<TestBlockContent>()
            .0
            .get(&(editor.entity_id(), id))
            .cloned()
    })?;
    Some(f(cx))
}

pub fn editor_content_with_blocks(editor: &Entity<Editor>, cx: &mut VisualTestContext) -> String {
    cx.draw(
        gpui::Point::default(),
        size(px(3000.0), px(3000.0)),
        |_, _| editor.clone(),
    );
    let (snapshot, mut lines, blocks) = editor.update_in(cx, |editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let text = editor.display_text(cx);
        let lines = text.lines().map(|s| s.to_string()).collect::<Vec<String>>();
        let blocks = snapshot
            .blocks_in_range(DisplayRow(0)..snapshot.max_point().row())
            .map(|(row, block)| (row, block.clone()))
            .collect::<Vec<_>>();
        (snapshot, lines, blocks)
    });
    for (row, block) in blocks {
        match block {
            Block::Custom(custom_block) => {
                if let BlockPlacement::Near(x) = &custom_block.placement
                    && snapshot.intersects_fold(x.to_point(&snapshot.buffer_snapshot()))
                {
                    continue;
                };
                let content = block_content_for_tests(editor, custom_block.id, cx)
                    .expect("block content not found");
                // 2: "related info 1 for diagnostic 0"
                if let Some(height) = custom_block.height {
                    if height == 0 {
                        lines[row.0 as usize - 1].push_str(" § ");
                        lines[row.0 as usize - 1].push_str(&content);
                    } else {
                        let block_lines = content.lines().collect::<Vec<_>>();
                        assert_eq!(block_lines.len(), height as usize);
                        lines[row.0 as usize].push_str("§ ");
                        lines[row.0 as usize].push_str(block_lines[0].trim_end());
                        for i in 1..height as usize {
                            if row.0 as usize + i >= lines.len() {
                                lines.push("".to_string());
                            };
                            lines[row.0 as usize + i].push_str("§ ");
                            lines[row.0 as usize + i].push_str(block_lines[i].trim_end());
                        }
                    }
                }
            }
            Block::FoldedBuffer {
                first_excerpt,
                height,
            } => {
                lines[row.0 as usize].push_str(&cx.update(|_, cx| {
                    format!("§ {}", first_excerpt.buffer.file().unwrap().file_name(cx))
                }));
                for row in row.0 + 1..row.0 + height {
                    lines[row as usize].push_str("§ -----");
                }
            }
            Block::ExcerptBoundary { height, .. } => {
                for row in row.0..row.0 + height {
                    lines[row as usize].push_str("§ -----");
                }
            }
            Block::BufferHeader { excerpt, height } => {
                lines[row.0 as usize].push_str(
                    &cx.update(|_, cx| {
                        format!("§ {}", excerpt.buffer.file().unwrap().file_name(cx))
                    }),
                );
                for row in row.0 + 1..row.0 + height {
                    lines[row as usize].push_str("§ -----");
                }
            }
        }
    }
    lines.join("\n")
}
