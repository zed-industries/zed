//! The editor-backed implementation of [`picker::PreviewBackend`].

use std::sync::Arc;

use gpui::{
    AnyElement, App, AppContext as _, Context, Entity, IntoElement, Pixels, StyledText, Task,
    Window, px,
};
use language::{Bias, Buffer, HighlightedText, HighlightedTextBuilder, ToPoint};
use picker::{MatchLocation, PreviewBackend, PreviewLayout, PreviewSource, PreviewUpdate};
use project::{Project, Symbol};
use rope::Point;
use settings::Settings;
use ui::{ActiveTheme, Color, div, prelude::*, v_flex};
use util::ResultExt as _;
use util::rel_path::RelPath;

use editor::{Editor, EditorSettings, RowHighlightOptions, display_map::HighlightKey};
use multi_buffer::{MultiBuffer, MultiBufferOffset};

pub fn editor_preview(
    project: Entity<Project>,
    window: &mut Window,
    cx: &mut App,
) -> Arc<dyn PreviewBackend> {
    Arc::new(EditorPreviewHandle(
        cx.new(|cx| EditorPreview::new(project, window, cx)),
    ))
}

struct EditorPreviewHandle(Entity<EditorPreview>);

impl PreviewBackend for EditorPreviewHandle {
    fn update(&self, update: PreviewUpdate, window: &mut Window, cx: &mut App) {
        self.0
            .update(cx, |content, cx| content.update(update, window, cx));
    }

    fn render(&self, layout: PreviewLayout, cx: &mut App) -> AnyElement {
        self.0.update(cx, |content, cx| {
            content.render(layout, cx).into_any_element()
        })
    }

    fn adjust_to_new_size(&self, window: &mut Window, cx: &mut App) {
        self.0
            .update(cx, |content, cx| content.scroll_to_focus_match(window, cx));
    }

    fn clear(&self, cx: &mut App) {
        self.0.update(cx, |content, _| content.clear());
    }
}

struct SearchMatchLineHighlight;

struct EditorPreview {
    project: Entity<Project>,
    current_path: Option<Arc<RelPath>>,
    /// When set show a text message instead of a preview
    message: Option<HighlightedText>,
    preview_editor: Entity<Editor>,
    /// Store the load preview task so we have only one at the time
    pending_update: Task<()>,
}

impl EditorPreview {
    fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let preview_editor = cx.new(|cx: &mut Context<Editor>| {
            let capability = language::Capability::ReadWrite; // Later narrowed per buffer
            let multi_buffer = cx.new(|_| MultiBuffer::without_headers(capability));
            let mut editor = Editor::for_multibuffer(multi_buffer, None, window, cx);
            let editor_settings = EditorSettings::get_global(cx).clone();

            // We want editing to happen in the multibuffer not in the modal. The editor acts
            // as one big <send to multibuffer> button.
            editor.set_read_only(true);
            editor.set_input_enabled(false);
            editor.scroll_manager.set_forbid_vertical_scroll(true);
            editor.disable_scrollbars_and_minimap(window, cx);
            editor.disable_inline_diagnostics();
            editor.disable_diagnostics(cx);
            editor.disable_expand_excerpt_buttons(cx);
            editor.disable_mouse_wheel_zoom();
            editor.set_show_gutter(editor_settings.gutter.line_numbers, cx); // needed for line numbers
            editor.set_show_line_numbers(editor_settings.gutter.line_numbers, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_bookmarks(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_cursor_when_unfocused(true, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
            editor
        });

        let mut this = Self {
            project,
            preview_editor,
            current_path: None,
            message: None,
            pending_update: Task::ready(()),
        };
        this.clear(); // picker starts with no results.
        this
    }

    fn clear(&mut self) {
        let mut message = HighlightedTextBuilder::default();
        message.push_plain("No results to preview");
        self.message = Some(message.build());
    }

    fn update(&mut self, update: PreviewUpdate, window: &mut Window, cx: &mut Context<Self>) {
        let PreviewUpdate {
            source,
            match_location: highlight,
        } = update;

        match source {
            PreviewSource::Path(abs_path) => {
                self.update_from_path(abs_path, highlight, window, cx);
            }
            PreviewSource::Buffer(buffer) => {
                self.update_from_buffer(buffer, highlight, window, cx);
                cx.notify();
            }
            PreviewSource::Symbol(symbol) => {
                self.update_from_symbol(symbol, window, cx);
            }
            PreviewSource::Message(message) => {
                self.message = Some(message);
                cx.notify();
            }
        }
    }

    fn update_from_path(
        &mut self,
        abs_path: std::path::PathBuf,
        highlight: Option<MatchLocation>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let open_task = self.project.update(cx, |project, cx| {
            match project.project_path_for_absolute_path(&abs_path, cx) {
                Some(project_path) => {
                    if let Some(buffer) = project.get_open_buffer(&project_path, cx) {
                        Task::ready(Ok(buffer))
                    } else {
                        project.open_buffer(project_path, cx)
                    }
                }
                None => project.open_local_buffer(&abs_path, cx),
            }
        });

        self.pending_update = cx.spawn_in(window, async move |this, cx| {
            let Some(buffer) = open_task.await.log_err() else {
                return;
            };
            this.update_in(cx, |this, window, cx| {
                this.update_from_buffer(buffer, highlight, window, cx);
                cx.notify();
            })
            .ok();
        });
    }

    fn update_from_symbol(&mut self, symbol: Symbol, window: &mut Window, cx: &mut Context<Self>) {
        let open_task = self.project.update(cx, |project, cx| {
            project.open_buffer_for_symbol(&symbol, cx)
        });

        self.pending_update = cx.spawn_in(window, async move |this, cx| {
            let Some(buffer) = open_task.await.log_err() else {
                return;
            };
            this.update_in(cx, |this, window, cx| {
                let snapshot = buffer.read(cx).text_snapshot();
                let start = snapshot.clip_point_utf16(symbol.range.start, Bias::Left);
                let end = snapshot.clip_point_utf16(symbol.range.end, Bias::Left);
                let highlight = MatchLocation {
                    anchor_range: snapshot.anchor_before(start)..snapshot.anchor_after(end),
                    range: snapshot.point_utf16_to_offset(start)
                        ..snapshot.point_utf16_to_offset(end),
                };
                this.update_from_buffer(buffer, Some(highlight), window, cx);
                cx.notify();
            })
            .ok();
        });
    }

    fn update_from_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        highlight: Option<MatchLocation>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.message = None; // show the preview editor
        self.current_path = buffer.read(cx).file().map(|file| file.path().clone());

        const MIN_LINE_HEIGHT_PX: Pixels = px(6.0);
        const MARGIN: u32 = 2; // scrolling can offset things;
        let max_visible_rows =
            (window.viewport_size().height / MIN_LINE_HEIGHT_PX).ceil() as u32 + MARGIN;

        self.preview_editor.update(cx, |editor, cx| {
            let focus_row = highlight
                .as_ref()
                .map(|hl| {
                    hl.anchor_range
                        .start
                        .to_point(&buffer.read(cx).text_snapshot())
                        .row
                })
                .unwrap_or_default();

            let multi_buffer = editor.buffer().clone();
            multi_buffer.update(cx, |multi_buffer, cx| {
                multi_buffer.clear(cx);
                multi_buffer.set_excerpts_for_buffer(
                    buffer,
                    [Point::new(focus_row, 0)..Point::new(focus_row, 0)],
                    max_visible_rows,
                    cx,
                );
            });

            editor.clear_row_highlights::<SearchMatchLineHighlight>();
            editor.clear_background_highlights(HighlightKey::PickerPreview, cx);

            let Some(highlight) = highlight else {
                return;
            };

            let mb = multi_buffer.read(cx).snapshot(cx);
            let Some(range) = mb
                .anchor_in_excerpt(highlight.anchor_range.start)
                .zip(mb.anchor_in_excerpt(highlight.anchor_range.end))
                .map(|(start, end)| start..end)
            else {
                return;
            };

            editor.highlight_rows::<SearchMatchLineHighlight>(
                range.clone(),
                |cx| cx.theme().colors().editor_active_line_background,
                RowHighlightOptions::default(),
                cx,
            );

            editor.highlight_background(
                HighlightKey::PickerPreview,
                &[range],
                |_, theme| theme.colors().search_match_background,
                cx,
            );
        });
        self.scroll_to_focus_match(window, cx);
    }

    /// Keep the scroll as far left as possible while showing the match.
    /// Vertically center the match as much as possible
    fn scroll_to_focus_match(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.preview_editor.update(cx, |editor, cx| {
            let display_snapshot = editor.display_snapshot(cx);
            let buffer_snapshot = display_snapshot.buffer_snapshot();
            let search_range = buffer_snapshot.anchor_before(MultiBufferOffset(0))
                ..buffer_snapshot.anchor_after(buffer_snapshot.len());

            // There is at most one highlighted match in the preview, so take the
            // first background highlight range as the match to focus.
            let Some((range, _)) = editor
                .background_highlights_in_range(search_range, &display_snapshot, cx.theme())
                .into_iter()
                .next()
            else {
                return;
            };

            let target_row = range.start.row().0 as f64;
            let centered_y = editor
                .visible_line_count()
                .map_or(target_row, |visible_lines| {
                    (target_row - (visible_lines - 1.) / 2.).max(0.)
                });

            let start_column = range.start.column() as f64;
            let end_column = range.end.column() as f64;
            let centered_x = editor.visible_column_count().map_or(0., |visible_columns| {
                let min_x_for_end = (end_column - visible_columns + 1.).max(0.);
                min_x_for_end.min(start_column)
            });

            editor.scroll_manager.set_forbid_vertical_scroll(false);
            editor.set_scroll_position(gpui::Point::new(centered_x, centered_y), window, cx);
            editor.scroll_manager.set_forbid_vertical_scroll(true);
        })
    }

    fn render(&self, layout: PreviewLayout, cx: &App) -> impl IntoElement {
        match layout {
            PreviewLayout::Below => self.render_preview_below(cx).into_any_element(),
            PreviewLayout::Right => self.render_preview_right(cx).into_any_element(),
            PreviewLayout::Hidden => gpui::Empty.into_any_element(),
        }
    }

    fn render_preview_right(&self, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_t_md()
            .rounded_b_md()
            .child(self.render_message_or_editor(cx))
    }

    fn render_preview_below(&self, cx: &App) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_b_md()
            .child(self.render_message_or_editor(cx))
    }

    fn render_message_or_editor(&self, cx: &App) -> impl IntoElement {
        if let Some(message) = &self.message {
            self.render_message(message, cx).into_any_element()
        } else {
            div()
                .flex_1()
                .overflow_hidden()
                .child(self.occluded_editor())
                .into_any_element()
        }
    }

    fn render_message(&self, message: &HighlightedText, cx: &App) -> impl IntoElement {
        // `with_highlights` inherits the container's text style (set below),
        // while keeping the message's own highlights (e.g. the file path in
        // the file finder's "Create new file" entry).
        let content = StyledText::new(message.text.clone())
            .with_highlights(message.highlights.iter().cloned())
            .into_any_element();
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .font_ui(cx)
            .text_ui(cx)
            .text_color(Color::Muted.color(cx))
            .child(content)
    }

    fn occluded_editor(&self) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .child(self.preview_editor.clone())
            .child(
                div()
                    .id("picker-preview-editor")
                    .absolute()
                    .inset_0()
                    .occlude(),
            )
    }
}
