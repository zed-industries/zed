use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use editor::{Editor, HighlightKey, MultiBuffer, RowHighlightOptions};
use gpui::{App, Task};
use gpui::{AppContext, Context, Entity, TaskExt, Window};
use language::{Buffer, HighlightedText, HighlightedTextBuilder, ToPoint};
use project::Project;
use rope::Point;
use ui::{ActiveTheme, IntoElement, Pixels, px};
use util::rel_path::RelPath;

pub mod render;

/// The preview window of a [`Picker`](crate::Picker).
///
/// Why an enum? While most pickers will want to show just the buffer
/// there will be some: like bookmarks with description that want to display
/// other metadata too. A preview for breakpoints could be part editor part
/// showing any condition (if any) and how many times the breakpoint got hit.
pub struct Preview {
    content: Entity<EditorPreview>,
    pub(crate) layout: Layout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Layout {
    #[default]
    Hidden,
    Below,
    Right,
}

impl Preview {
    pub fn new_editor(project: Entity<Project>, window: &mut Window, cx: &mut App) -> Self {
        Preview {
            content: cx.new(|cx| EditorPreview::new(project, window, cx)),
            layout: Layout::default(),
        }
    }

    pub fn update(&mut self, update: Update, window: &mut Window, cx: &mut impl AppContext) {
        // self.content since this will become a match to support non editor or composite previews
        self.content.update(cx, |content, cx| {
            content.update(update, window, cx);
        });
    }

    pub fn render(&self, cx: &mut App) -> impl IntoElement {
        let layout = self.layout;
        self.content.update(cx, |content, cx| {
            content.render(layout, cx).into_any_element()
        })
    }

    /// Called after a resize to let the preview do resizing logic
    /// like scrolling
    pub fn adjust_to_new_size(&self, window: &mut Window, cx: &mut App) {
        self.content.update(cx, |content, cx| {
            content.scroll_to_focus_match(window, cx);
        })
    }

    /// Empty the preview and show a placeholder message
    pub(crate) fn clear(&self, cx: &mut App) {
        self.content.update(cx, |content, _| content.clear())
    }
}

/// Identifies what a preview should show.
pub enum PreviewSource {
    /// The buffer is identified by its absolute path; the preview opens it.
    ///
    /// Used by pickers (like the file finder) that only know the path of the
    /// match.
    Path(PathBuf),
    /// The buffer is provided directly.
    ///
    /// Used by pickers (like the text picker) that already hold the matched
    /// buffer.
    Buffer(Entity<Buffer>),
    /// No buffer to show; display this message centered in the preview instead.
    ///
    /// Used by pickers that have a selection without a previewable buffer (like
    /// the file finder's "create new file" entry). Built as a [`HighlightedText`]
    /// so callers can emphasize parts of the message (e.g. a file path).
    Message(HighlightedText),
}

pub struct MatchLocation {
    /// The location of the match (for highlighting)
    pub anchor_range: Range<language::Anchor>,
    /// The location of the match as an offset (for scrolling)
    pub range: Range<usize>,
}

/// An update for the [`Preview`] window of a [`Picker`](crate::Picker).
pub struct Update {
    /// Where to source the buffer to preview.
    pub source: PreviewSource,
    /// The location to highlight and scroll to, if any.
    pub match_location: Option<MatchLocation>,
}

impl Update {
    /// Preview the buffer at `abs_path` without highlighting anything.
    pub fn from_path(abs_path: PathBuf) -> Self {
        Self {
            source: PreviewSource::Path(abs_path),
            match_location: None,
        }
    }

    /// Show `message` centered in the preview instead of a buffer.
    ///
    /// The message is a [`HighlightedText`], so parts of it (e.g. a file path)
    /// can be emphasized.
    pub fn message(message: HighlightedText) -> Self {
        Self {
            source: PreviewSource::Message(message),
            match_location: None,
        }
    }

    /// Preview `buffer`, highlighting and scrolling to `highlight`.
    pub fn from_buffer(buffer: Entity<Buffer>, highlight: MatchLocation) -> Self {
        Self {
            source: PreviewSource::Buffer(buffer),
            match_location: Some(highlight),
        }
    }
}

struct SearchMatchLineHighlight;

pub struct EditorPreview {
    project: Entity<Project>,
    current_path: Option<Arc<RelPath>>,
    /// When set show a text message instead of a preview
    message: Option<HighlightedText>,
    preview_editor: Entity<Editor>,
}

impl EditorPreview {
    fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let preview_editor = cx.new(|cx: &mut Context<Editor>| {
            let capability = language::Capability::ReadWrite; // Later narrowed per buffer
            let multi_buffer = cx.new(|_| MultiBuffer::without_headers(capability));
            let mut editor = Editor::for_multibuffer(multi_buffer, None, window, cx);

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
            editor.set_show_gutter(true, cx); // needed for line numbers
            editor.set_show_line_numbers(true, cx);
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
        };
        this.clear(); // picker starts with no results.
        this
    }

    fn clear(&mut self) {
        let mut message = HighlightedTextBuilder::default();
        message.push_plain("No results to preview");
        self.message = Some(message.build());
    }

    fn update(&mut self, update: Update, window: &mut Window, cx: &mut Context<Self>) {
        let Update {
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
            PreviewSource::Message(message) => {
                self.message = Some(message);
                cx.notify();
            }
        }
    }

    fn update_from_path(
        &mut self,
        abs_path: PathBuf,
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

        cx.spawn_in(window, async move |this, cx| {
            let buffer = open_task.await?;
            this.update_in(cx, |this, window, cx| {
                this.update_from_buffer(buffer, highlight, window, cx);
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
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
            let search_range = buffer_snapshot.anchor_before(multi_buffer::MultiBufferOffset(0))
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
}
