use std::{
    ops::Range,
    sync::{Arc, OnceLock},
};

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
    AppContext, AvailableSpace, ClipboardItem, Entity, FontWeight, HighlightStyle, StyledText,
    Task, TextStyle,
};
use indoc;
use language::{Buffer, Diagnostic, DiagnosticEntry, DiagnosticSet, Point, PointUtf16};
use lsp::DiagnosticSeverity;
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, App, ButtonCommon, ButtonSize, ButtonStyle, Clickable, Color,
    ComponentPreview, Element, FluentBuilder, IconButton, IconName, InteractiveElement,
    IntoComponent, IntoElement, ParentElement, SharedString, StatefulInteractiveElement, Styled,
    Tooltip, VisibleOnHover, Window, div, h_flex, px, relative, v_flex,
};

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

impl editor::DiagnosticRenderer for DiagnosticRenderer {
    fn render_group(
        &self,
        diagnostic_group: Vec<DiagnosticEntry<DisplayPoint>>,
        snapshot: EditorSnapshot,
        cx: &App,
    ) -> Task<Vec<BlockProperties<Anchor>>> {
        cx.spawn(async move |_| {
            let diagnostics_by_rows =
                diagnostic_group
                    .into_iter()
                    .fold(HashMap::default(), |mut acc, diagnostic| {
                        acc.entry(diagnostic.range.start.row())
                            .or_insert_with(Vec::new)
                            .push(diagnostic);
                        acc
                    });

            for (row, mut diagnostics) in diagnostics_by_rows {
                diagnostics.sort_by_key(|diagnostic| {
                    (
                        !diagnostic.diagnostic.is_primary,
                        diagnostic.diagnostic.severity,
                    )
                });
                let primary = diagnostics.remove(0);

                let mut markdown = if let Some(source) = primary.diagnostic.source {
                    format!("{}: {}", source, primary.diagnostic.message)
                } else {
                    primary.diagnostic.message.clone()
                };
                for diagnostic in diagnostics {
                    markdown.push_str("\n- hint: ");
                    if let Some(source) = diagnostic.diagnostic.source {
                        markdown.push(format!("{source}: "));
                    }
                    markdown.push(secondary.diagnostic.message)
                }
                let parsed_content = cx
                    .new_window_entity(|window, cx| {
                        let status_colors = cx.theme().status();

                        match local_diagnostic.diagnostic.severity {
                            DiagnosticSeverity::ERROR => {
                                background_color = Some(status_colors.error_background);
                                border_color = Some(status_colors.error_border);
                            }
                            DiagnosticSeverity::WARNING => {
                                background_color = Some(status_colors.warning_background);
                                border_color = Some(status_colors.warning_border);
                            }
                            DiagnosticSeverity::INFORMATION => {
                                background_color = Some(status_colors.info_background);
                                border_color = Some(status_colors.info_border);
                            }
                            DiagnosticSeverity::HINT => {
                                background_color = Some(status_colors.hint_background);
                                border_color = Some(status_colors.hint_border);
                            }
                            _ => {
                                background_color = Some(status_colors.ignored_background);
                                border_color = Some(status_colors.ignored_border);
                            }
                        };
                        let settings = ThemeSettings::get_global(cx);
                        let mut base_text_style = window.text_style();
                        base_text_style.refine(&TextStyleRefinement {
                            font_family: Some(settings.ui_font.family.clone()),
                            font_fallbacks: settings.ui_font.fallbacks.clone(),
                            font_size: Some(settings.ui_font_size(cx).into()),
                            color: Some(cx.theme().colors().editor_foreground),
                            background_color: Some(gpui::transparent_black()),

                            ..Default::default()
                        });
                        let markdown_style = MarkdownStyle {
                            base_text_style,
                            selection_background_color: { cx.theme().players().local().selection },
                            link: TextStyleRefinement {
                                underline: Some(gpui::UnderlineStyle {
                                    thickness: px(1.),
                                    color: Some(cx.theme().colors().editor_foreground),
                                    wavy: false,
                                }),
                                ..Default::default()
                            },
                            ..Default::default()
                        };
                        Markdown::new_text(SharedString::new(text), markdown_style.clone(), cx)
                            .open_url(open_markdown_url)
                    })
                    .await;
            }

            // diagnostic_group
            //     .into_iter()
            //     .map(|entry| {
            //         let diagnostic = entry.diagnostic.clone();
            //         let message_height = diagnostic.message.matches('\n').count() as u32 + 1;
            //         BlockProperties {
            //             style: BlockStyle::Fixed,
            //             placement: BlockPlacement::Below(
            //                 snapshot
            //                     .buffer_snapshot
            //                     .anchor_after(entry.range.start.to_point(&snapshot)),
            //             ),
            //             height: message_height,
            //             render: diagnostic_block_renderer(diagnostic, None, true),
            //             priority: 0,
            //         }
            //     })
            //     .collect()
            //
            Vec::new()
        })
    }
}
#[derive(IntoComponent)]
#[component(scope = "Version Control")]
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
