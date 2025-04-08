use std::{
    borrow::Cow,
    ops::Range,
    sync::{Arc, OnceLock},
};

use anyhow::{Result, anyhow};
use editor::{
    Anchor, Bias, Direction, DisplayPoint, Editor, EditorElement, EditorSnapshot, EditorStyle,
    MultiBuffer,
    actions::Cancel,
    diagnostic_style,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, RenderBlock},
    scroll::Autoscroll,
};
use gpui::{
    AppContext, AsyncWindowContext, AvailableSpace, ClipboardItem, Entity, FontWeight,
    HighlightStyle, StyledText, Task, TextStyle, TextStyleRefinement, WeakEntity, size,
};
use indoc;
use language::{Buffer, BufferId, Diagnostic, DiagnosticEntry, DiagnosticSet, PointUtf16};
use lsp::DiagnosticSeverity;
use markdown::{Markdown, MarkdownStyle};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, App, ButtonCommon, ButtonSize, ButtonStyle, Clickable, Color,
    ComponentPreview, Context, FluentBuilder, IconButton, IconName, InteractiveElement,
    IntoComponent, IntoElement, ParentElement, Pixels, SharedString, StatefulInteractiveElement,
    Styled, Tooltip, VisibleOnHover, VisualContext, Window, div, h_flex, px, relative, v_flex,
};
use util::{ResultExt, maybe};

fn escape_markdown<'a>(s: &'a str) -> Cow<'a, str> {
    if s.chars().any(|c| c.is_ascii_punctuation()) {
        let mut output = String::new();
        for c in s.chars() {
            if c.is_ascii_punctuation() {
                output.push('\\')
            }
            output.push(c)
        }
        output.into()
    } else {
        s.into()
    }
}

impl DiagnosticRenderer {
    async fn render(
        diagnostic_group: Vec<DiagnosticEntry<DisplayPoint>>,
        buffer_id: BufferId,
        snapshot: EditorSnapshot,
        editor: WeakEntity<Editor>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Vec<BlockProperties<Anchor>>> {
        let primary_ix = diagnostic_group
            .iter()
            .position(|d| d.diagnostic.is_primary)
            .ok_or_else(|| anyhow!("no primary diagnostic"))?;
        let primary = diagnostic_group[primary_ix].clone();
        let mut same_row = Vec::new();
        let mut close = Vec::new();
        let mut distant = Vec::new();
        let group_id = primary.diagnostic.group_id;
        for (ix, entry) in diagnostic_group.into_iter().enumerate() {
            if entry.diagnostic.is_primary {
                continue;
            }
            if entry.range.start.row() == primary.range.start.row() {
                same_row.push(entry)
            } else if entry
                .range
                .start
                .row()
                .0
                .abs_diff(primary.range.start.row().0)
                < 2
            // todo!(5)
            {
                close.push(entry)
            } else {
                distant.push((ix, entry))
            }
        }

        let mut markdown =
            escape_markdown(&if let Some(source) = primary.diagnostic.source.as_ref() {
                format!("{}: {}", source, primary.diagnostic.message)
            } else {
                primary.diagnostic.message
            })
            .to_string();
        for entry in same_row {
            markdown.push_str("\n- hint: ");
            markdown.push_str(&escape_markdown(&entry.diagnostic.message))
        }

        for (ix, entry) in &distant {
            markdown.push_str("\n- hint: [");
            markdown.push_str(&escape_markdown(&entry.diagnostic.message));
            markdown.push_str(&format!("](file://#diagnostic-{group_id}-{ix})\n",))
        }

        let block = Self::create_block(
            markdown,
            primary.diagnostic.severity,
            snapshot.display_point_to_anchor(primary.range.start, Bias::Right),
            0,
            buffer_id,
            editor.clone(),
            cx,
        )
        .await?;
        let mut results = vec![block];

        for entry in close {
            let markdown = if let Some(source) = entry.diagnostic.source.as_ref() {
                format!("{}: {}", source, entry.diagnostic.message)
            } else {
                entry.diagnostic.message
            };

            let block = Self::create_block(
                markdown,
                entry.diagnostic.severity,
                snapshot.display_point_to_anchor(entry.range.start, Bias::Right),
                results.len(),
                buffer_id,
                editor.clone(),
                cx,
            )
            .await?;
            results.push(block)
        }

        for (_, entry) in distant {
            let mut markdown = if let Some(source) = entry.diagnostic.source.as_ref() {
                format!("{}: {}", source, entry.diagnostic.message)
            } else {
                entry.diagnostic.message
            };
            markdown.push_str(&format!(
                " ([back](file://#diagnostic-{group_id}-{primary_ix}))"
            ));

            let block = Self::create_block(
                markdown,
                entry.diagnostic.severity,
                snapshot.display_point_to_anchor(entry.range.start, Bias::Right),
                results.len(),
                buffer_id,
                editor.clone(),
                cx,
            )
            .await?;
            results.push(block)
        }

        Ok(results)
    }

    pub fn open_link(
        editor: &mut Editor,
        link: SharedString,
        window: &mut Window,
        buffer_id: BufferId,
        cx: &mut Context<Editor>,
    ) {
        let diagnostic = maybe!({
            let diagnostic = link.strip_prefix("file://#diagnostic-")?;
            let (group_id, ix) = diagnostic.split_once('-')?;

            editor
                .snapshot(window, cx)
                .buffer_snapshot
                .diagnostic_group(buffer_id, group_id.parse().ok()?)
                .nth(ix.parse().ok()?)
        });
        let Some(diagnostic) = diagnostic else {
            editor::hover_popover::open_markdown_url(link, window, cx);
            return;
        };
        editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
            s.select_ranges([diagnostic.range.start..diagnostic.range.start]);
        })
    }

    async fn create_block(
        markdown: String,
        severity: DiagnosticSeverity,
        position: Anchor,
        id: usize,
        buffer_id: BufferId,
        editor: WeakEntity<Editor>,
        cx: &mut AsyncWindowContext,
    ) -> BlockProperties<Anchor> {
        let mut editor_line_height = px(0.);
        let mut text_style = None;
        let markdown = cx.new(|cx| {
            let settings = ThemeSettings::get_global(cx);
            let settings = ThemeSettings::get_global(cx);
            editor_line_height = (settings.line_height() * settings.buffer_font_size(cx)).round();
            text_style.replace(TextStyleRefinement {
                font_family: Some(settings.ui_font.family.clone()),
                font_fallbacks: settings.ui_font.fallbacks.clone(),
                font_size: Some(settings.buffer_font_size(cx).into()),
                line_height: Some((editor_line_height - px(2.)).into()),
                color: Some(cx.theme().colors().editor_foreground),
                ..Default::default()
            });
            Markdown::new(SharedString::new(markdown), None, None, cx).open_url(
                move |link, window, cx| {
                    editor
                        .update(cx, |editor, cx| {
                            Self::open_link(editor, link, window, buffer_id, cx)
                        })
                        .ok();
                },
            )
        })?;
        let block = DiagnosticBlock {
            severity,
            id,
            markdown,
        };

        Ok(BlockProperties {
            style: BlockStyle::Fixed,
            placement: BlockPlacement::Below(position),
            height: Some(1),
            render: Arc::new(move |bcx| block.render_block(measured.width + px(4.), lines, bcx)),
            priority: 0,
        })
    }
}

struct DiagnosticBlock {
    severity: DiagnosticSeverity,
    id: usize,
    markdown: Entity<Markdown>,
}

impl DiagnosticBlock {
    fn render_block(&self, width: Pixels, height_in_lines: f32, bcx: &BlockContext) -> AnyElement {
        let cx = &bcx.app;
        let status_colors = bcx.app.theme().status();

        let (background_color, border_color) = match self.severity {
            DiagnosticSeverity::ERROR => (status_colors.error_background, status_colors.error),
            DiagnosticSeverity::WARNING => {
                (status_colors.warning_background, status_colors.warning)
            }
            DiagnosticSeverity::INFORMATION => (status_colors.info_background, status_colors.info),
            DiagnosticSeverity::HINT => (status_colors.hint_background, status_colors.info),
            _ => (status_colors.ignored_background, status_colors.ignored),
        };
        let min_left = bcx.gutter_dimensions.full_width();
        let max_left = min_left + bcx.max_width - width;
        let left = bcx.anchor_x.min(max_left).max(min_left);
        let settings = ThemeSettings::get_global(cx);
        let editor_line_height = (settings.line_height() * settings.buffer_font_size(cx)).round();
        let line_height = editor_line_height - px(2.);

        div()
            .border_l_2()
            .px_2()
            .my(px(1.)) // NOTE: we can only borrow space from the line-height...
            .h(height_in_lines * editor_line_height - px(2.))
            .line_height(line_height)
            .bg(background_color)
            .border_color(border_color)
            .id(self.id)
            .ml(left)
            .max_w(width)
            .overflow_y_hidden()
            .overflow_scroll()
            .child(self.markdown.clone())
            .into_any_element()
    }
}

impl editor::DiagnosticRenderer for DiagnosticRenderer {
    fn render_group(
        &self,
        diagnostic_group: Vec<DiagnosticEntry<DisplayPoint>>,
        buffer_id: BufferId,
        snapshot: EditorSnapshot,
        editor: WeakEntity<Editor>,
        window: &Window,
        cx: &App,
    ) -> Task<Vec<BlockProperties<Anchor>>> {
        let mut window = window.to_async(cx);
        cx.spawn(async move |_| {
            Self::render(diagnostic_group, buffer_id, snapshot, editor, &mut window)
                .await
                .log_err()
                .unwrap_or_default()
        })
    }
}
#[derive(IntoComponent)]
#[component(scope = "Diagnostics")]
pub struct DiagnosticRenderer;

fn new_entry(
    range: Range<PointUtf16>,
    severity: lsp::DiagnosticSeverity,
    message: &str,
    group_id: usize,
    is_primary: bool,
) -> DiagnosticEntry<PointUtf16> {
    DiagnosticEntry {
        range,
        diagnostic: Diagnostic {
            source: Some("rustc".to_string()),
            code: None,
            severity,
            message: message.to_owned(),
            group_id,
            is_primary,
            is_disk_based: false,
            is_unnecessary: false,
            data: None,
        },
    }
}

impl ComponentPreview for DiagnosticRenderer {
    fn preview(window: &mut Window, cx: &mut App) -> AnyElement {
        static EDITOR: OnceLock<Entity<Editor>> = OnceLock::new();
        let editor = EDITOR.get_or_init(|| {
            let buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(
                    indoc::indoc! {
                        r#" //
                fn main() {
                    println!("{}", foo(1, "Hello, world!"));
                }

                fn foo(_: i32) -> bool {
                    true
                }
            "#
                    },
                    cx,
                );
                let entries = [
                    new_entry(
                        PointUtf16::new(2, 26)..PointUtf16::new(2, 42),
                        DiagnosticSeverity::ERROR,
                        "expected 1 argument, found 2",
                        1,
                        true,
                    ),
                    new_entry(
                        PointUtf16::new(2, 19)..PointUtf16::new(2, 22),
                        DiagnosticSeverity::ERROR,
                        "this function takes 1 argument but 2 arguments were supplied",
                        2,
                        true,
                    ),
                    new_entry(
                        PointUtf16::new(2, 26)..PointUtf16::new(2, 41),
                        DiagnosticSeverity::HINT,
                        "unexpected argument #2 of type `&'static str`",
                        2,
                        false,
                    ),
                    new_entry(
                        PointUtf16::new(5, 3)..PointUtf16::new(5, 6),
                        DiagnosticSeverity::HINT,
                        "function defined here",
                        2,
                        false,
                    ),
                    new_entry(
                        PointUtf16::new(2, 24)..PointUtf16::new(2, 41),
                        DiagnosticSeverity::HINT,
                        "remove the extra argument",
                        2,
                        false,
                    ),
                ];

                let diagnostics = DiagnosticSet::new(entries, &buffer.snapshot());
                buffer.update_diagnostics(lsp::LanguageServerId(1), diagnostics, cx);
                buffer
            });

            cx.new(|cx| {
                let mut editor = Editor::new(
                    editor::EditorMode::Full,
                    cx.new(|cx| MultiBuffer::singleton(buffer, cx)),
                    None,
                    window,
                    cx,
                );
                editor.go_to_diagnostic_impl(Direction::Next, window, cx);

                editor
            })
        });
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.buffer_font.family.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(settings.buffer_line_height.value()),
            ..TextStyle::default()
        };
        div()
            .h(px(500.))
            .w_full()
            .child(EditorElement::new(
                &editor,
                EditorStyle {
                    background: cx.theme().colors().editor_background,
                    local_player: cx.theme().players().local(),
                    text: text_style,
                    ..EditorStyle::default()
                },
            ))
            .into_any_element()
    }
}
