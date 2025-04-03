use std::{
    borrow::Cow,
    ops::Range,
    sync::{Arc, OnceLock},
};

use anyhow::{Result, anyhow};
use collections::HashMap;
use editor::{
    Anchor, Bias, Direction, DisplayPoint, Editor, EditorElement, EditorSnapshot, EditorStyle,
    MultiBuffer, MultiBufferSnapshot,
    actions::{Cancel, GoToDiagnostic},
    diagnostic_style,
    display_map::{
        BlockContext, BlockPlacement, BlockProperties, BlockStyle, DisplayRow, RenderBlock,
    },
};
use gpui::{
    AppContext, AsyncWindowContext, AvailableSpace, ClipboardItem, Entity, FontWeight,
    HighlightStyle, Size, StyledText, Task, TextStyle, TextStyleRefinement, size,
};
use gpui::{Hsla, Refineable};
use indoc;
use language::{Buffer, Diagnostic, DiagnosticEntry, DiagnosticSet, Point, PointUtf16};
use lsp::DiagnosticSeverity;
use markdown::{Markdown, MarkdownStyle};
use settings::Settings;
use theme::{StatusColors, ThemeSettings};
use ui::{
    ActiveTheme, AnyElement, App, ButtonCommon, ButtonSize, ButtonStyle, Clickable, Color,
    ComponentPreview, DefiniteLength, Element, FluentBuilder, IconButton, IconName,
    InteractiveElement, IntoComponent, IntoElement, ParentElement, Pixels, SharedString,
    StatefulInteractiveElement, Styled, Tooltip, VisibleOnHover, VisualContext, Window, div,
    h_flex, px, relative, v_flex,
};
use util::ResultExt;

pub fn diagnostic_block_renderer(
    diagnostic: Diagnostic,
    max_message_rows: Option<u8>,
    allow_closing: bool,
) -> RenderBlock {
    let (text_without_backticks, code_ranges) =
        highlight_diagnostic_message(&diagnostic, max_message_rows);

    Arc::new(move |cx: &mut BlockContext| {
        let group_id: SharedString = cx.block_id.to_string().into();

        let mut text_style = cx.window.text_style().clone();
        text_style.color = diagnostic_style(diagnostic.severity, cx.theme().status());
        let theme_settings = ThemeSettings::get_global(cx);
        text_style.font_family = theme_settings.buffer_font.family.clone();
        text_style.font_style = theme_settings.buffer_font.style;
        text_style.font_features = theme_settings.buffer_font.features.clone();
        text_style.font_weight = theme_settings.buffer_font.weight;

        let multi_line_diagnostic = diagnostic.message.contains('\n');

        let buttons = |diagnostic: &Diagnostic| {
            if multi_line_diagnostic {
                v_flex()
            } else {
                h_flex()
            }
            .when(allow_closing, |div| {
                div.children(diagnostic.is_primary.then(|| {
                    IconButton::new("close-block", IconName::XCircle)
                        .icon_color(Color::Muted)
                        .size(ButtonSize::Compact)
                        .style(ButtonStyle::Transparent)
                        .visible_on_hover(group_id.clone())
                        .on_click(move |_click, window, cx| {
                            window.dispatch_action(Box::new(Cancel), cx)
                        })
                        .tooltip(|window, cx| {
                            Tooltip::for_action("Close Diagnostics", &Cancel, window, cx)
                        })
                }))
            })
            .child(
                IconButton::new("copy-block", IconName::Copy)
                    .icon_color(Color::Muted)
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Transparent)
                    .visible_on_hover(group_id.clone())
                    .on_click({
                        let message = diagnostic.message.clone();
                        move |_click, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(message.clone()))
                        }
                    })
                    .tooltip(Tooltip::text("Copy diagnostic message")),
            )
        };

        let icon_size = buttons(&diagnostic).into_any_element().layout_as_root(
            AvailableSpace::min_size(),
            cx.window,
            cx.app,
        );

        h_flex()
            .id(cx.block_id)
            .group(group_id.clone())
            .relative()
            .size_full()
            .block_mouse_down()
            .pl(cx.gutter_dimensions.width)
            .w(cx.max_width - cx.gutter_dimensions.full_width())
            .child(
                div()
                    .flex()
                    .w(cx.anchor_x - cx.gutter_dimensions.width - icon_size.width)
                    .flex_shrink(),
            )
            .child(buttons(&diagnostic))
            .child(div().flex().flex_shrink_0().child(
                StyledText::new(text_without_backticks.clone()).with_default_highlights(
                    &text_style,
                    code_ranges.iter().map(|range| {
                        (
                            range.clone(),
                            HighlightStyle {
                                font_weight: Some(FontWeight::BOLD),
                                ..Default::default()
                            },
                        )
                    }),
                ),
            ))
            .into_any_element()
    })
}

pub fn highlight_diagnostic_message(
    diagnostic: &Diagnostic,
    mut max_message_rows: Option<u8>,
) -> (SharedString, Vec<Range<usize>>) {
    let mut text_without_backticks = String::new();
    let mut code_ranges = Vec::new();

    if let Some(source) = &diagnostic.source {
        text_without_backticks.push_str(source);
        code_ranges.push(0..source.len());
        text_without_backticks.push_str(": ");
    }

    let mut prev_offset = 0;
    let mut in_code_block = false;
    let has_row_limit = max_message_rows.is_some();
    let mut newline_indices = diagnostic
        .message
        .match_indices('\n')
        .filter(|_| has_row_limit)
        .map(|(ix, _)| ix)
        .fuse()
        .peekable();

    for (quote_ix, _) in diagnostic
        .message
        .match_indices('`')
        .chain([(diagnostic.message.len(), "")])
    {
        let mut first_newline_ix = None;
        let mut last_newline_ix = None;
        while let Some(newline_ix) = newline_indices.peek() {
            if *newline_ix < quote_ix {
                if first_newline_ix.is_none() {
                    first_newline_ix = Some(*newline_ix);
                }
                last_newline_ix = Some(*newline_ix);

                if let Some(rows_left) = &mut max_message_rows {
                    if *rows_left == 0 {
                        break;
                    } else {
                        *rows_left -= 1;
                    }
                }
                let _ = newline_indices.next();
            } else {
                break;
            }
        }
        let prev_len = text_without_backticks.len();
        let new_text = &diagnostic.message[prev_offset..first_newline_ix.unwrap_or(quote_ix)];
        text_without_backticks.push_str(new_text);
        if in_code_block {
            code_ranges.push(prev_len..text_without_backticks.len());
        }
        prev_offset = last_newline_ix.unwrap_or(quote_ix) + 1;
        in_code_block = !in_code_block;
        if first_newline_ix.map_or(false, |newline_ix| newline_ix < quote_ix) {
            text_without_backticks.push_str("...");
            break;
        }
    }

    (text_without_backticks.into(), code_ranges)
}

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
        snapshot: EditorSnapshot,
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
        for entry in diagnostic_group {
            if entry.diagnostic.is_primary {
                continue;
            }
            if false && entry.range.start.row() == primary.range.start.row() {
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
                distant.push(entry)
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

        for entry in &distant {
            markdown.push_str("\n- hint: [");
            markdown.push_str(&escape_markdown(&entry.diagnostic.message));
            markdown.push_str("](https://google.com)\n")
        }

        let block = Self::create_block(
            markdown,
            primary.diagnostic.severity,
            snapshot.display_point_to_anchor(primary.range.start, Bias::Right),
            0,
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
                cx,
            )
            .await?;
            results.push(block)
        }

        for entry in distant {
            let mut markdown = if let Some(source) = entry.diagnostic.source.as_ref() {
                format!("{}: {}", source, entry.diagnostic.message)
            } else {
                entry.diagnostic.message
            };
            markdown.push_str(" ([back](https://google.com))");

            let block = Self::create_block(
                markdown,
                entry.diagnostic.severity,
                snapshot.display_point_to_anchor(entry.range.start, Bias::Right),
                results.len(),
                cx,
            )
            .await?;
            results.push(block)
        }

        Ok(results)
    }

    async fn create_block(
        markdown: String,
        severity: DiagnosticSeverity,
        position: Anchor,
        id: usize,
        cx: &mut AsyncWindowContext,
    ) -> Result<BlockProperties<Anchor>> {
        let mut editor_line_height = px(0.);
        let mut text_style = None;
        let markdown = cx.new_window_entity(|window, cx| {
            let settings = ThemeSettings::get_global(cx);
            dbg!(settings.line_height());
            let settings = ThemeSettings::get_global(cx);
            editor_line_height = (settings.line_height() * settings.buffer_font_size(cx)).round();
            dbg!(editor_line_height);
            text_style.replace(TextStyleRefinement {
                font_family: Some(settings.ui_font.family.clone()),
                font_fallbacks: settings.ui_font.fallbacks.clone(),
                font_size: Some(settings.buffer_font_size(cx).into()),
                line_height: Some((editor_line_height - px(2.)).into()),
                color: Some(cx.theme().colors().editor_foreground),
                ..Default::default()
            });
            let markdown_style = MarkdownStyle {
                base_text_style: text_style.as_ref().unwrap().clone().into(),
                selection_background_color: { cx.theme().players().local().selection },
                link: TextStyleRefinement {
                    underline: Some(gpui::UnderlineStyle {
                        thickness: px(1.),
                        color: Some(cx.theme().colors().editor_foreground),
                        wavy: false,
                    }),
                    ..Default::default()
                },
                compact: true,
                ..Default::default()
            };
            Markdown::new(SharedString::new(markdown), markdown_style, None, None, cx)
                .open_url(editor::hover_popover::open_markdown_url)
        })?;

        markdown
            .update(cx, |parsed, cx| parsed.when_parsing_complete())?
            .await
            .ok();
        let measured = cx.update(|window, cx| {
            dbg!("MEASURING...");
            let mut d = div()
                .max_w(px(600.))
                .border_l_2()
                .px_2()
                .child(markdown.clone().into_any_element());
            *d.text_style() = Some(text_style.clone().unwrap());
            window.measure(
                d.into_any_element(),
                size(AvailableSpace::MinContent, AvailableSpace::MinContent),
                cx,
            )
        })?;
        dbg!("MEASURED...", &measured);
        let block = DiagnosticBlock {
            severity,
            id,
            markdown,
        };

        dbg!(measured.height, editor_line_height);
        let lines = ((measured.height - px(1.)) / editor_line_height).ceil();
        let lines = lines.min(4.);

        let true_size = size(measured.width + px(4.), editor_line_height * lines);

        Ok(BlockProperties {
            style: BlockStyle::Fixed,
            placement: BlockPlacement::Below(position),
            // todo!()
            height: lines as u32,
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
        snapshot: EditorSnapshot,
        window: &Window,
        cx: &App,
    ) -> Task<Vec<BlockProperties<Anchor>>> {
        let mut window = window.to_async(cx);
        cx.spawn(async move |_| {
            Self::render(diagnostic_group, snapshot, &mut window)
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
