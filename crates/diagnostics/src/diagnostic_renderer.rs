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
    ActiveTheme, AnyElement, App, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px, relative,
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
    pub(crate) initial_range: Range<Point>,
    pub(crate) severity: DiagnosticSeverity,
    pub(crate) group_id: usize,
    pub(crate) id: usize,
    pub(crate) buffer_id: BufferId,
    pub(crate) markdown: Entity<Markdown>,
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
