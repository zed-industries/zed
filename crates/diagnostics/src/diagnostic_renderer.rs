use std::{ops::Range, sync::Arc};

use editor::{
    Anchor, Editor, EditorSnapshot, ToOffset,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle},
    hover_markdown_style,
    scroll::Autoscroll,
};
use gpui::{AppContext, Entity, Focusable, WeakEntity};
use language::{BufferId, Diagnostic, DiagnosticEntry};
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
        let group_id = primary.diagnostic.group_id;
        let mut results = vec![];
        for entry in diagnostic_group.iter() {
            if entry.diagnostic.is_primary {
                let mut markdown = Self::markdown(&entry.diagnostic);
                let diagnostic = &primary.diagnostic;
                if diagnostic.source.is_some() || diagnostic.code.is_some() {
                    markdown.push_str(" (");
                }
                if let Some(source) = diagnostic.source.as_ref() {
                    markdown.push_str(&Markdown::escape(&source));
                }
                if diagnostic.source.is_some() && diagnostic.code.is_some() {
                    markdown.push(' ');
                }
                if let Some(code) = diagnostic.code.as_ref() {
                    if let Some(description) = diagnostic.code_description.as_ref() {
                        markdown.push('[');
                        markdown.push_str(&Markdown::escape(&code.to_string()));
                        markdown.push_str("](");
                        markdown.push_str(&Markdown::escape(description.as_ref()));
                        markdown.push(')');
                    } else {
                        markdown.push_str(&Markdown::escape(&code.to_string()));
                    }
                }
                if diagnostic.source.is_some() || diagnostic.code.is_some() {
                    markdown.push(')');
                }

                for (ix, entry) in diagnostic_group.iter().enumerate() {
                    if entry.range.start.row.abs_diff(primary.range.start.row) >= 5 {
                        markdown.push_str("\n- hint: [");
                        markdown.push_str(&Markdown::escape(&entry.diagnostic.message));
                        markdown.push_str(&format!(
                            "](file://#diagnostic-{buffer_id}-{group_id}-{ix})\n",
                        ))
                    }
                }
                results.push(DiagnosticBlock {
                    initial_range: primary.range.clone(),
                    severity: primary.diagnostic.severity,
                    diagnostics_editor: diagnostics_editor.clone(),
                    markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
                });
            } else if entry.range.start.row.abs_diff(primary.range.start.row) < 5 {
                let markdown = Self::markdown(&entry.diagnostic);

                results.push(DiagnosticBlock {
                    initial_range: entry.range.clone(),
                    severity: entry.diagnostic.severity,
                    diagnostics_editor: diagnostics_editor.clone(),
                    markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
                });
            } else {
                let mut markdown = Self::markdown(&entry.diagnostic);
                markdown.push_str(&format!(
                    " ([back](file://#diagnostic-{buffer_id}-{group_id}-{primary_ix}))"
                ));

                results.push(DiagnosticBlock {
                    initial_range: entry.range.clone(),
                    severity: entry.diagnostic.severity,
                    diagnostics_editor: diagnostics_editor.clone(),
                    markdown: cx.new(|cx| Markdown::new(markdown.into(), None, None, cx)),
                });
            }
        }

        results
    }

    fn markdown(diagnostic: &Diagnostic) -> String {
        let mut markdown = String::new();

        if let Some(md) = &diagnostic.markdown {
            markdown.push_str(md);
        } else {
            markdown.push_str(&Markdown::escape(&diagnostic.message));
        };
        markdown
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

    fn render_hover(
        &self,
        diagnostic_group: Vec<DiagnosticEntry<Point>>,
        range: Range<Point>,
        buffer_id: BufferId,
        cx: &mut App,
    ) -> Option<Entity<Markdown>> {
        let blocks = Self::diagnostic_blocks_for_group(diagnostic_group, buffer_id, None, cx);
        blocks.into_iter().find_map(|block| {
            if block.initial_range == range {
                Some(block.markdown)
            } else {
                None
            }
        })
    }

    fn open_link(
        &self,
        editor: &mut Editor,
        link: SharedString,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        DiagnosticBlock::open_link(editor, &None, link, window, cx);
    }
}

#[derive(Clone)]
pub(crate) struct DiagnosticBlock {
    pub(crate) initial_range: Range<Point>,
    pub(crate) severity: DiagnosticSeverity,
    pub(crate) markdown: Entity<Markdown>,
    pub(crate) diagnostics_editor: Option<WeakEntity<ProjectDiagnosticsEditor>>,
}

impl DiagnosticBlock {
    pub fn render_block(&self, editor: WeakEntity<Editor>, bcx: &BlockContext) -> AnyElement {
        let cx = &bcx.app;
        let status_colors = bcx.app.theme().status();

        let max_width = bcx.em_width * 120.;

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
                            editor
                                .update(cx, |editor, cx| {
                                    Self::open_link(editor, &diagnostics_editor, link, window, cx)
                                })
                                .ok();
                        }
                    }),
            )
            .into_any_element()
    }

    pub fn open_link(
        editor: &mut Editor,
        diagnostics_editor: &Option<WeakEntity<ProjectDiagnosticsEditor>>,
        link: SharedString,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let Some(diagnostic_link) = link.strip_prefix("file://#diagnostic-") else {
            editor::hover_popover::open_markdown_url(link, window, cx);
            return;
        };
        let Some((buffer_id, group_id, ix)) = maybe!({
            let mut parts = diagnostic_link.split('-');
            let buffer_id: u64 = parts.next()?.parse().ok()?;
            let group_id: usize = parts.next()?.parse().ok()?;
            let ix: usize = parts.next()?.parse().ok()?;
            Some((BufferId::new(buffer_id).ok()?, group_id, ix))
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
                            Anchor::range_in_buffer(excerpt_id, buffer_id, diagnostic.range),
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
