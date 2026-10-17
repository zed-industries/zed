pub mod editor_lsp_test_context;
pub mod editor_test_context;

use std::{rc::Rc, sync::LazyLock};

pub use crate::emmet_ext::wrap_with_abbreviation;
pub use crate::rust_analyzer_ext::expand_macro_recursively;
use crate::{
    DisplayPoint, Editor, EditorMode, FoldPlaceholder, MultiBuffer, SelectionEffects, Size,
    display_map::{Block, CustomBlockId, DisplayMap, DisplayRow, DisplaySnapshot, ToDisplayPoint},
};
use collections::HashMap;
use gpui::{
    AppContext as _, Context, Entity, EntityId, Font, FontFeatures, FontStyle, FontWeight, Pixels,
    VisualTestContext, Window, font, size,
};
use multi_buffer::MultiBufferOffset;
use pretty_assertions::assert_eq;
use project::{Project, project_settings::DiagnosticSeverity};
use ui::{App, BorrowAppContext, IntoElement, px};
use util::test::{generate_marked_text, marked_text_offsets, marked_text_ranges};

#[cfg(test)]
#[ctor::ctor(unsafe)]
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
        .map(|offset| MultiBufferOffset(offset).to_display_point(&snapshot))
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
        s.select_ranges(
            text_ranges
                .into_iter()
                .map(|range| MultiBufferOffset(range.start)..MultiBufferOffset(range.end)),
        )
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
        &editor
            .selections
            .ranges::<MultiBufferOffset>(&editor.display_snapshot(cx))
            .into_iter()
            .map(|range| range.start.0..range.end.0)
            .collect::<Vec<_>>(),
        marked_text.contains("«"),
    );
    assert_eq!(actual, marked_text, "Selections don't match");
}

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
    editor_content_with_blocks_and_width(editor, px(3000.), cx)
}

pub fn editor_content_with_blocks_and_width(
    editor: &Entity<Editor>,
    width: Pixels,
    cx: &mut VisualTestContext,
) -> String {
    editor_content_with_blocks_and_size(editor, size(width, px(3000.0)), cx)
}

pub fn editor_content_with_blocks_and_size(
    editor: &Entity<Editor>,
    draw_size: Size<Pixels>,
    cx: &mut VisualTestContext,
) -> String {
    cx.simulate_resize(draw_size);
    cx.draw(gpui::Point::default(), draw_size, |_, _| {
        editor.clone().into_any_element()
    });
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
                while lines.len() <= row.0 as usize {
                    lines.push(String::new());
                }
                lines[row.0 as usize].push_str(&cx.update(|_, cx| {
                    format!(
                        "§ {}",
                        first_excerpt
                            .buffer(snapshot.buffer_snapshot())
                            .file()
                            .map(|file| file.file_name(cx))
                            .unwrap_or("<no file>")
                    )
                }));
                for row in row.0 + 1..row.0 + height {
                    while lines.len() <= row as usize {
                        lines.push(String::new());
                    }
                    lines[row as usize].push_str("§ -----");
                }
            }
            Block::ExcerptBoundary { height, .. } => {
                for row in row.0..row.0 + height {
                    while lines.len() <= row as usize {
                        lines.push(String::new());
                    }
                    lines[row as usize].push_str("§ -----");
                }
            }
            Block::BufferHeader { excerpt, height } => {
                while lines.len() <= row.0 as usize {
                    lines.push(String::new());
                }
                lines[row.0 as usize].push_str(&cx.update(|_, cx| {
                    format!(
                        "§ {}",
                        excerpt
                            .buffer(snapshot.buffer_snapshot())
                            .file()
                            .map(|file| file.file_name(cx))
                            .unwrap_or("<no file>")
                    )
                }));
                for row in row.0 + 1..row.0 + height {
                    while lines.len() <= row as usize {
                        lines.push(String::new());
                    }
                    lines[row as usize].push_str("§ -----");
                }
            }
            Block::Spacer { height, .. } => {
                for row in row.0..row.0 + height {
                    while lines.len() <= row as usize {
                        lines.push(String::new());
                    }
                    lines[row as usize].push_str("§ spacer");
                }
            }
        }
    }
    lines.join("\n")
}

#[cfg(test)]
pub(crate) use shaping::ShapingTextSystem;

#[cfg(test)]
mod shaping {
    use gpui::{
        Bounds, DevicePixels, Font, FontId, FontMetrics, FontRun, FontStyle, FontWeight, GlyphId,
        LineLayout, NoopTextSystem, Pixels, PlatformTextSystem, RenderGlyphParams, ShapedGlyph,
        ShapedRun, Size, TextRenderingMode, point, px,
    };
    use std::borrow::Cow;

    pub(crate) struct ShapingTextSystem(pub(crate) NoopTextSystem);

    impl ShapingTextSystem {
        pub(crate) const ADVANCE: f32 = 0.5;
        pub(crate) const PRE_BASE_VOWEL: char = '\u{93f}';
        pub(crate) const KERNING: f32 = 2.;
        pub(crate) const BOLD_EXTRA: f32 = 1.;
        pub(crate) const REGULAR: FontId = FontId(1);
        pub(crate) const BOLD: FontId = FontId(2);
        pub(crate) const ITALIC: FontId = FontId(3);

        fn layout_line_inner(
            &self,
            text: &str,
            font_size: Pixels,
            runs: &[FontRun],
            mut glyph_x_positions: Option<&mut Vec<f64>>,
        ) -> (LineLayout, f64) {
            let advance = font_size * Self::ADVANCE;
            let mut x = px(0.);
            let mut precise_x = 0.0;
            let mut shaped_runs = Vec::new();
            let mut run_start = 0;
            for run in runs {
                let run_text = &text[run_start..run_start + run.len];
                let extra = if run.font_id == Self::BOLD {
                    px(Self::BOLD_EXTRA)
                } else {
                    px(0.)
                };
                let mut glyphs = Vec::new();
                let mut previous = None;
                let mut characters = run_text.char_indices().peekable();
                while let Some((index, character)) = characters.next() {
                    if previous == Some('A') && character == 'V' {
                        x -= px(Self::KERNING);
                        precise_x -= f64::from(Self::KERNING);
                    }
                    if let Some((vowel_index, _)) =
                        characters.next_if(|(_, next)| *next == Self::PRE_BASE_VOWEL)
                    {
                        for glyph_index in [vowel_index, index] {
                            let position = if let Some(positions) = glyph_x_positions.as_mut() {
                                positions.push(precise_x);
                                Pixels::from(precise_x)
                            } else {
                                x
                            };
                            glyphs.push(ShapedGlyph {
                                id: GlyphId(3),
                                position: point(position, px(0.)),
                                index: run_start + glyph_index,
                                is_emoji: false,
                            });
                            x += advance + extra;
                            precise_x += f64::from(advance + extra);
                        }
                        previous = Some(Self::PRE_BASE_VOWEL);
                        continue;
                    }
                    let ligated = match character {
                        'f' => characters.next_if(|(_, next)| *next == 'i'),
                        'w' => characters.next_if(|(_, next)| *next == 'w'),
                        _ => None,
                    };
                    let position = if let Some(positions) = glyph_x_positions.as_mut() {
                        positions.push(precise_x);
                        Pixels::from(precise_x)
                    } else {
                        x
                    };
                    glyphs.push(ShapedGlyph {
                        id: GlyphId(if ligated.is_some() { 2 } else { 1 }),
                        position: point(position, px(0.)),
                        index: run_start + index,
                        is_emoji: false,
                    });
                    match ligated {
                        Some((_, ligated)) => {
                            previous = Some(ligated);
                            x += advance * 1.5 + extra;
                            precise_x += f64::from(advance * 1.5 + extra);
                        }
                        None => {
                            previous = Some(character);
                            x += advance + extra;
                            precise_x += f64::from(advance + extra);
                        }
                    }
                }
                if !glyphs.is_empty() {
                    shaped_runs.push(ShapedRun {
                        font_id: run.font_id,
                        glyphs,
                    });
                }
                run_start += run.len;
            }
            let bold_ascent = if runs.iter().any(|run| run.font_id == Self::BOLD) {
                px(Self::BOLD_EXTRA)
            } else {
                px(0.)
            };
            (
                LineLayout {
                    font_size,
                    width: if glyph_x_positions.is_some() {
                        Pixels::from(precise_x)
                    } else {
                        x
                    },
                    ascent: font_size * 0.8 + bold_ascent,
                    descent: font_size * -0.2,
                    runs: shaped_runs,
                    len: text.len(),
                },
                precise_x,
            )
        }
    }

    impl PlatformTextSystem for ShapingTextSystem {
        fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> anyhow::Result<()> {
            self.0.add_fonts(fonts)
        }

        fn all_font_names(&self) -> Vec<String> {
            self.0.all_font_names()
        }

        fn font_id(&self, descriptor: &Font) -> anyhow::Result<FontId> {
            Ok(if descriptor.weight >= FontWeight::BOLD {
                Self::BOLD
            } else if descriptor.style == FontStyle::Italic {
                Self::ITALIC
            } else {
                Self::REGULAR
            })
        }

        fn font_metrics(&self, font_id: FontId) -> FontMetrics {
            self.0.font_metrics(font_id)
        }

        fn typographic_bounds(
            &self,
            font_id: FontId,
            glyph_id: GlyphId,
        ) -> anyhow::Result<Bounds<f32>> {
            self.0.typographic_bounds(font_id, glyph_id)
        }

        fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Size<f32>> {
            self.0.advance(font_id, glyph_id)
        }

        fn glyph_for_char(&self, font_id: FontId, character: char) -> Option<GlyphId> {
            self.0.glyph_for_char(font_id, character)
        }

        fn glyph_raster_bounds(
            &self,
            params: &RenderGlyphParams,
        ) -> anyhow::Result<Bounds<DevicePixels>> {
            self.0.glyph_raster_bounds(params)
        }

        fn rasterize_glyph(
            &self,
            params: &RenderGlyphParams,
            raster_bounds: Bounds<DevicePixels>,
        ) -> anyhow::Result<(Size<DevicePixels>, Vec<u8>)> {
            self.0.rasterize_glyph(params, raster_bounds)
        }

        fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
            self.layout_line_inner(text, font_size, runs, None).0
        }

        fn layout_line_with_precise_positions(
            &self,
            text: &str,
            font_size: Pixels,
            runs: &[FontRun],
        ) -> (LineLayout, Vec<f64>, f64) {
            let mut glyph_x_positions = Vec::new();
            let (layout, width) =
                self.layout_line_inner(text, font_size, runs, Some(&mut glyph_x_positions));
            (layout, glyph_x_positions, width)
        }

        fn recommended_rendering_mode(
            &self,
            font_id: FontId,
            font_size: Pixels,
        ) -> TextRenderingMode {
            self.0.recommended_rendering_mode(font_id, font_size)
        }
    }
}
