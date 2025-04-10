use std::{
    borrow::Cow,
    ops::Range,
    sync::{Arc, OnceLock},
};

use editor::{
    Anchor, Bias, Direction, DisplayPoint, Editor, EditorElement, EditorSnapshot, EditorStyle,
    MultiBuffer,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle},
    hover_markdown_style,
    scroll::Autoscroll,
};
use gpui::{AppContext, Entity, Task, TextStyle, WeakEntity};
use indoc;
use language::{Buffer, BufferId, Diagnostic, DiagnosticEntry, DiagnosticSet, PointUtf16};
use lsp::DiagnosticSeverity;
use markdown::{Markdown, MarkdownElement};
use settings::Settings;
use text::Point;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, App, ComponentPreview, InteractiveElement, IntoComponent, IntoElement,
    ParentElement, SharedString, StatefulInteractiveElement, Styled, Window, div, px, relative,
};
use util::maybe;

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
    pub fn diagnostic_blocks_for_group(
        diagnostic_group: Vec<DiagnosticEntry<Point>>,
        buffer_id: BufferId,
        cx: &mut App,
    ) -> Vec<DiagnosticBlock> {
        let Some(primary_ix) = diagnostic_group
            .iter()
            .position(|d| d.diagnostic.is_primary)
        else {
            dbg!("ignoring", diagnostic_group);
            return Vec::new();
        };
        let primary = diagnostic_group[primary_ix].clone();
        let mut same_row = Vec::new();
        let mut close = Vec::new();
        let mut distant = Vec::new();
        let group_id = primary.diagnostic.group_id;
        for (ix, entry) in diagnostic_group.into_iter().enumerate() {
            if entry.diagnostic.is_primary {
                continue;
            }
            if entry.range.start.row == primary.range.start.row {
                same_row.push(entry)
            } else if entry.range.start.row.abs_diff(primary.range.start.row) < 5 {
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

        let mut results = vec![DiagnosticBlock {
            initial_range: primary.range,
            severity: primary.diagnostic.severity,
            group_id,
            id: 0,
            buffer_id,
            markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
        }];

        for entry in close {
            let markdown = if let Some(source) = entry.diagnostic.source.as_ref() {
                format!("{}: {}", source, entry.diagnostic.message)
            } else {
                entry.diagnostic.message
            };
            let markdown = escape_markdown(&markdown).to_string();

            results.push(DiagnosticBlock {
                initial_range: entry.range,
                severity: entry.diagnostic.severity,
                group_id,
                id: results.len(),
                buffer_id,
                markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
            });
        }

        for (_, entry) in distant {
            let markdown = if let Some(source) = entry.diagnostic.source.as_ref() {
                format!("{}: {}", source, entry.diagnostic.message)
            } else {
                entry.diagnostic.message
            };
            let mut markdown = escape_markdown(&markdown).to_string();
            markdown.push_str(&format!(
                " ([back](file://#diagnostic-{group_id}-{primary_ix}))"
            ));

            results.push(DiagnosticBlock {
                initial_range: entry.range,
                severity: entry.diagnostic.severity,
                group_id,
                id: results.len(),
                buffer_id,
                markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
            });
        }

        results
    }
}

pub(crate) struct DiagnosticBlock {
    initial_range: Range<Point>,
    severity: DiagnosticSeverity,
    group_id: usize,
    id: usize,
    buffer_id: BufferId,
    markdown: Entity<Markdown>,
}

impl DiagnosticBlock {
    pub fn render_block(&self, editor: WeakEntity<Editor>, bcx: &BlockContext) -> AnyElement {
        let cx = &bcx.app;
        let status_colors = bcx.app.theme().status();
        let max_width = px(600.);

        let (background_color, border_color) = match self.severity {
            DiagnosticSeverity::ERROR => (status_colors.error_background, status_colors.error),
            DiagnosticSeverity::WARNING => {
                (status_colors.warning_background, status_colors.warning)
            }
            DiagnosticSeverity::INFORMATION => (status_colors.info_background, status_colors.info),
            DiagnosticSeverity::HINT => (status_colors.hint_background, status_colors.info),
            _ => (status_colors.ignored_background, status_colors.ignored),
        };
        let settings = ThemeSettings::get_global(cx);
        let editor_line_height = (settings.line_height() * settings.buffer_font_size(cx)).round();
        let line_height = editor_line_height; // - px(2.);
        let buffer_id = self.buffer_id;

        div()
            .border_l_2()
            .px_2()
            .line_height(line_height)
            .bg(background_color)
            .border_color(border_color)
            .id(self.id)
            .max_w(max_width)
            .child(
                MarkdownElement::new(self.markdown.clone(), hover_markdown_style(bcx.window, cx))
                    .on_url_click({
                        move |link, window, cx| {
                            Self::open_link(editor.clone(), link, window, buffer_id, cx)
                        }
                    }),
            )
            .into_any_element()
    }

    pub fn open_link(
        editor: WeakEntity<Editor>,
        link: SharedString,
        window: &mut Window,
        buffer_id: BufferId,
        cx: &mut App,
    ) {
        editor
            .update(cx, |editor, cx| {
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
            })
            .ok();
    }
}

impl editor::DiagnosticRenderer for DiagnosticRenderer {
    fn render_group(
        &self,
        diagnostic_group: Vec<DiagnosticEntry<Point>>,
        buffer_id: BufferId,
        snapshot: EditorSnapshot,
        editor: WeakEntity<Editor>,
        cx: &mut App,
    ) -> Vec<BlockProperties<Anchor>> {
        let blocks = Self::diagnostic_blocks_for_group(diagnostic_group, buffer_id, cx);
        blocks
            .into_iter()
            .map(|block| {
                let editor = editor.clone();
                BlockProperties {
                    placement: BlockPlacement::Near(
                        snapshot
                            .buffer_snapshot
                            .anchor_after(block.initial_range.start),
                    ),
                    height: Some(1),
                    style: BlockStyle::Flex,
                    render: Arc::new(move |bcx| block.render_block(editor.clone(), bcx)),
                    priority: 1,
                }
            })
            .collect()
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
