use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use editor::display_map::ToDisplayPoint;
use editor::{Editor, HighlightKey, MultiBuffer, RowHighlightOptions};
use gpui::{App, Task};
use gpui::{AppContext, Context, Entity, Window};
use language::Buffer;
use project::Project;
use ui::{ActiveTheme, IntoElement};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Layout {
    Hidden,
    Below,
    Right,
}

impl Default for Layout {
    fn default() -> Self {
        Self::Hidden
    }
}

impl Preview {
    pub fn new_editor(project: Entity<Project>, window: &mut Window, cx: &mut App) -> Self {
        Preview {
            content: cx.new(|cx| EditorPreview::new(project, window, cx)),
            layout: Layout::default(),
        }
    }

    pub fn update(&mut self, update: Update, window: &mut Window, cx: &mut impl AppContext) {
        // self.content since this will become a match to support non editor previews
        self.content.update(cx, |content, cx| {
            content.update(update, window, cx);
        });
    }

    pub fn render(&self, cx: &mut App) -> impl IntoElement {
        // self.content since this will become a match to support non editor previews
        let layout = self.layout;
        self.content.update(cx, |content, cx| {
            content.render(layout, cx).into_any_element()
        })
    }
}

/// Identifies the buffer a preview should show.
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
}

/// Describes a location within the previewed buffer that should be highlighted
/// and scrolled into view.
pub struct PreviewHighlight {
    /// The location of the match, used to highlight the match and place the
    /// cursor.
    pub anchor_range: Range<language::Anchor>,
    /// The location of the match as an offset, used to select the matched text
    /// so the editor scrolls to it.
    pub range: Range<usize>,
}

/// An update for the [`Preview`] window of a [`Picker`](crate::Picker).
pub struct Update {
    /// Where to source the buffer to preview.
    pub source: PreviewSource,
    /// The location to highlight and scroll to, if any.
    pub highlight: Option<PreviewHighlight>,
}

impl Update {
    /// Preview the buffer at `abs_path` without highlighting anything.
    pub fn from_path(abs_path: PathBuf) -> Self {
        Self {
            source: PreviewSource::Path(abs_path),
            highlight: None,
        }
    }

    /// Preview `buffer`, highlighting and scrolling to `highlight`.
    pub fn from_buffer(buffer: Entity<Buffer>, highlight: PreviewHighlight) -> Self {
        Self {
            source: PreviewSource::Buffer(buffer),
            highlight: Some(highlight),
        }
    }
}

/// TODO! rename relative position
/// - wire up autosave for the editor

struct SearchMatchLineHighlight;

pub struct EditorPreview {
    project: Entity<Project>,
    current_path: Option<Arc<RelPath>>,
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
            editor.set_show_gutter(false, cx);
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

        Self {
            project,
            preview_editor,
            current_path: None,
        }
    }

    pub(crate) fn has_content(&self, cx: &App) -> bool {
        !self.preview_editor.read(cx).is_empty(cx)
    }

    fn update(&mut self, update: Update, window: &mut Window, cx: &mut Context<Self>) {
        let Update { source, highlight } = update;

        match source {
            PreviewSource::Path(abs_path) => {
                self.update_from_path(abs_path, highlight, window, cx);
            }
            PreviewSource::Buffer(buffer) => {
                self.finish_update(buffer, highlight, window, cx);
                cx.notify();
            }
        }
    }

    fn update_from_path(
        &mut self,
        abs_path: PathBuf,
        highlight: Option<PreviewHighlight>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // TODO!(yara) debounce this/cache the last one for fast switching
        // between top two results.
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
                this.finish_update(buffer, highlight, window, cx);
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    /// TODO!(yara) we are not focussing the selection correctly.
    fn finish_update(
        &mut self,
        buffer: Entity<Buffer>,
        highlight: Option<PreviewHighlight>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current_path = buffer.read(cx).file().map(|file| file.path().clone());

        // TODO!(yara) do not set full range. We are not allowing scrolling anyway
        let full_range = [rope::Point::zero()..buffer.read(cx).max_point()];
        self.preview_editor.update(cx, |editor, cx| {
            let multi_buffer = editor.buffer().clone();
            multi_buffer.update(cx, |multi_buffer, cx| {
                multi_buffer.clear(cx);
                multi_buffer.set_excerpts_for_buffer(buffer, full_range, 0, cx);
            });

            editor.clear_row_highlights::<SearchMatchLineHighlight>();
            editor.clear_background_highlights(HighlightKey::PickerPreview, cx);

            let Some(highlight) = highlight else {
                return;
            };

            let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
            let (Some(start_anchor), Some(end_anchor)) = (
                multi_buffer_snapshot.anchor_in_excerpt(highlight.anchor_range.start),
                multi_buffer_snapshot.anchor_in_excerpt(highlight.anchor_range.end),
            ) else {
                return;
            };

            editor.highlight_rows::<SearchMatchLineHighlight>(
                start_anchor..start_anchor,
                cx.theme().colors().editor_active_line_background,
                RowHighlightOptions::default(),
                cx,
            );

            editor.highlight_background(
                HighlightKey::PickerPreview,
                &[start_anchor..end_anchor],
                |_, theme| theme.colors().search_match_background,
                cx,
            );

            // The editor forbids vertical scrolling so the user can't scroll the
            // preview themselves. That also blocks programmatic autoscroll, so we
            // compute the scroll position ourselves and apply it while temporarily
            // lifting the restriction.
            let display_snapshot = editor.display_snapshot(cx);
            let start_point = start_anchor.to_display_point(&display_snapshot);
            let end_point = end_anchor.to_display_point(&display_snapshot);

            // Vertically center the match.
            let target_row = start_point.row().0 as f64;
            let centered_y = editor
                .visible_line_count()
                .map_or(target_row, |visible_lines| {
                    (target_row - (visible_lines - 1.) / 2.).max(0.)
                });

            // Scroll horizontally as far left as possible while keeping the match
            // visible, so the editor doesn't drift right over time.
            let start_column = start_point.column() as f64;
            let end_column = end_point.column() as f64;
            let centered_x = editor.visible_column_count().map_or(0., |visible_columns| {
                let min_x_for_end = (end_column - visible_columns + 1.).max(0.);
                min_x_for_end.min(start_column)
            });

            editor.scroll_manager.set_forbid_vertical_scroll(false);
            editor.set_scroll_position(gpui::Point::new(centered_x, centered_y), window, cx);
            editor.scroll_manager.set_forbid_vertical_scroll(true);
        });
    }
}
