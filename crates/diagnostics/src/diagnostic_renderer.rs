use std::{ops::Range, sync::Arc};

use editor::{
    Anchor, Editor, EditorSnapshot, ToOffset,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle},
    hover_markdown_style,
    scroll::Autoscroll,
};
use gpui::{AppContext, Entity, Focusable, WeakEntity};
use language::{BufferId, DiagnosticEntry};
use lsp::DiagnosticSeverity;
use markdown::{Markdown, MarkdownElement};
use settings::Settings;
use text::{AnchorRangeExt, Point};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, App, Context, IntoElement, ParentElement, SharedString, Styled,
    Window, div,
};
use util::maybe;

use crate::ProjectDiagnosticsEditor;

pub struct DiagnosticRenderer;

impl DiagnosticRenderer {
    fn format_diagnostic_message(diagnostic: &language::Diagnostic) -> String {
        if let Some(code) = &diagnostic.code {
            format!("{} ({})", diagnostic.message, code)
        } else {
            diagnostic.message.clone()
        }
    }

    fn format_diagnostic(diagnostic: &language::Diagnostic) -> String {
        if let Some(source) = diagnostic.source.as_ref() {
            format!(
                "{}: {}",
                source,
                Self::format_diagnostic_message(diagnostic)
            )
        } else {
            Self::format_diagnostic_message(diagnostic)
        }
    }

    pub fn diagnostic_blocks_for_group(
        diagnostic_group: Vec<DiagnosticEntry<Point>>,
        buffer_id: BufferId,
        diagnostics_editor: Option<WeakEntity<ProjectDiagnosticsEditor>>,
        cx: &mut App,
    ) -> Vec<DiagnosticBlock> {
        let Some(primary_ix) = diagnostic_group
            .iter()
            .position(|d| d.diagnostic.is_primary)
        else {
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
            Markdown::escape(&Self::format_diagnostic(&primary.diagnostic)).to_string();
        for entry in same_row {
            markdown.push_str("\n- hint: ");
            markdown.push_str(&Markdown::escape(&Self::format_diagnostic_message(
                &entry.diagnostic,
            )))
        }

        for (ix, entry) in &distant {
            markdown.push_str("\n- hint: [");
            markdown.push_str(&Markdown::escape(&Self::format_diagnostic_message(
                &entry.diagnostic,
            )));
            markdown.push_str(&format!("](file://#diagnostic-{group_id}-{ix})\n",))
        }

        let mut results = vec![DiagnosticBlock {
            initial_range: primary.range,
            severity: primary.diagnostic.severity,
            buffer_id,
            diagnostics_editor: diagnostics_editor.clone(),
            markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
        }];

        for entry in close {
            let markdown =
                Markdown::escape(&Self::format_diagnostic(&entry.diagnostic)).to_string();

            results.push(DiagnosticBlock {
                initial_range: entry.range,
                severity: entry.diagnostic.severity,
                buffer_id,
                diagnostics_editor: diagnostics_editor.clone(),
                markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
            });
        }

        for (_, entry) in distant {
            let mut markdown =
                Markdown::escape(&Self::format_diagnostic(&entry.diagnostic)).to_string();
            markdown.push_str(&format!(
                " ([back](file://#diagnostic-{group_id}-{primary_ix}))"
            ));
            // problem: group-id changes...
            //  - only an issue in diagnostics because caching

            results.push(DiagnosticBlock {
                initial_range: entry.range,
                severity: entry.diagnostic.severity,
                buffer_id,
                diagnostics_editor: diagnostics_editor.clone(),
                markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
            });
        }

        results
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
        let blocks = Self::diagnostic_blocks_for_group(diagnostic_group, buffer_id, None, cx);
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

#[derive(Clone)]
pub(crate) struct DiagnosticBlock {
    pub(crate) initial_range: Range<Point>,
    pub(crate) severity: DiagnosticSeverity,
    pub(crate) buffer_id: BufferId,
    pub(crate) markdown: Entity<Markdown>,
    pub(crate) diagnostics_editor: Option<WeakEntity<ProjectDiagnosticsEditor>>,
}

impl DiagnosticBlock {
    pub fn render_block(&self, editor: WeakEntity<Editor>, bcx: &BlockContext) -> AnyElement {
        let cx = &bcx.app;
        let status_colors = bcx.app.theme().status();

        let max_width = bcx.em_width * 100.;

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
        let line_height = editor_line_height;
        let buffer_id = self.buffer_id;
        let diagnostics_editor = self.diagnostics_editor.clone();

        div()
            .border_l_2()
            .px_2()
            .line_height(line_height)
            .bg(background_color)
            .border_color(border_color)
            .max_w(max_width)
            .child(
                MarkdownElement::new(self.markdown.clone(), hover_markdown_style(bcx.window, cx))
                    .on_url_click({
                        move |link, window, cx| {
                            Self::open_link(
                                editor.clone(),
                                &diagnostics_editor,
                                link,
                                window,
                                buffer_id,
                                cx,
                            )
                        }
                    }),
            )
            .into_any_element()
    }

    pub fn open_link(
        editor: WeakEntity<Editor>,
        diagnostics_editor: &Option<WeakEntity<ProjectDiagnosticsEditor>>,
        link: SharedString,
        window: &mut Window,
        buffer_id: BufferId,
        cx: &mut App,
    ) {
        editor
            .update(cx, |editor, cx| {
                let Some(diagnostic_link) = link.strip_prefix("file://#diagnostic-") else {
                    editor::hover_popover::open_markdown_url(link, window, cx);
                    return;
                };
                let Some((group_id, ix)) = maybe!({
                    let (group_id, ix) = diagnostic_link.split_once('-')?;
                    let group_id: usize = group_id.parse().ok()?;
                    let ix: usize = ix.parse().ok()?;
                    Some((group_id, ix))
                }) else {
                    return;
                };

                if let Some(diagnostics_editor) = diagnostics_editor {
                    if let Some(diagnostic) = diagnostics_editor
                        .update(cx, |diagnostics, _| {
                            diagnostics
                                .diagnostics
                                .get(&buffer_id)
                                .cloned()
                                .unwrap_or_default()
                                .into_iter()
                                .filter(|d| d.diagnostic.group_id == group_id)
                                .nth(ix)
                        })
                        .ok()
                        .flatten()
                    {
                        let multibuffer = editor.buffer().read(cx);
                        let Some(snapshot) = multibuffer
                            .buffer(buffer_id)
                            .map(|entity| entity.read(cx).snapshot())
                        else {
                            return;
                        };

                        for (excerpt_id, range) in multibuffer.excerpts_for_buffer(buffer_id, cx) {
                            if range.context.overlaps(&diagnostic.range, &snapshot) {
                                Self::jump_to(
                                    editor,
                                    Anchor::range_in_buffer(
                                        excerpt_id,
                                        buffer_id,
                                        diagnostic.range,
                                    ),
                                    window,
                                    cx,
                                );
                                return;
                            }
                        }
                    }
                } else {
                    if let Some(diagnostic) = editor
                        .snapshot(window, cx)
                        .buffer_snapshot
                        .diagnostic_group(buffer_id, group_id)
                        .nth(ix)
                    {
                        Self::jump_to(editor, diagnostic.range, window, cx)
                    }
                };
            })
            .ok();
    }

    fn jump_to<T: ToOffset>(
        editor: &mut Editor,
        range: Range<T>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let snapshot = &editor.buffer().read(cx).snapshot(cx);
        let range = range.start.to_offset(&snapshot)..range.end.to_offset(&snapshot);

        editor.unfold_ranges(&[range.start..range.end], true, false, cx);
        editor.change_selections(Some(Autoscroll::fit()), window, cx, |s| {
            s.select_ranges([range.start..range.start]);
        });
        window.focus(&editor.focus_handle(cx));
    }
}
