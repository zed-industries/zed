use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use editor::scroll::Autoscroll;
use editor::{
    Editor, EditorEvent, HighlightKey, MultiBuffer, RowHighlightOptions, SelectionEffects,
};
use gpui::{App, FocusHandle, Subscription};
use gpui::{AppContext, Context, Entity, Window};
use language::Buffer;
use ui::{ActiveTheme, ContextMenu, IntoElement, PopoverMenuHandle};
use util::rel_path::RelPath;

use crate::preview::state::{LayoutMode, SavedQuickSearchLayout, StackedLayout, TelescopeLayout};

pub mod render;
pub mod state;

/// The preview window of a [`Picker`](crate::Picker).
///
/// Why an enum? While most pickers will want to show just the buffer
/// there will be some: like bookmarks with description that want to display
/// other metadata too. A preview for breakpoints could be part editor part
/// showing any condition (if any) and how many times the breakpoint got hit.
pub struct Preview {
    content: EditorPreview,
    pub layout: LayoutMode,
}

type Match = Box<dyn std::any::Any>;

impl Preview {
    pub fn new_editor(window: &mut Window, cx: &mut App) -> Self {
        Preview {
            content: EditorPreview::new(window, cx),
            layout: LayoutMode::default(),
        }
    }

    pub fn update(&mut self, update: Match, window: &mut Window, cx: &mut impl AppContext) {
        self.content.update(update, window, cx)
    }

    pub fn render(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.content.render(self.layout, window, cx);
    }
}

pub(crate) struct Update {
    buffer: Entity<Buffer>,
    range: Range<usize>,
    anchor_range: Range<language::Anchor>,
}

/// TODO! rename relative position

struct SearchMatchLineHighlight;

/// TODO!
/// - wire up autosave for the editor
///

pub struct EditorPreview {
    current_path: Option<Arc<RelPath>>,
    preview_editor: Entity<Editor>,
    /// TODO! should probably be in Preview not here
    pub split_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    /// TODO! should probably be in Preview not here
    pub focus_handle: FocusHandle,
    /// TODO! rename to relative position
    layout_mode: LayoutMode,
    /// TODO! deprecate these they now live in LayoutMode
    stacked: StackedLayout,
    telescope: TelescopeLayout,
}

impl EditorPreview {
    fn new(window: &mut Window, cx: &mut App) -> Self {
        let preview_editor = cx.new(|cx: &mut Context<Editor>| {
            let capability = language::Capability::ReadWrite; // Later narrowed per buffer
            let multi_buffer = cx.new(|_| MultiBuffer::without_headers(capability));
            Editor::for_multibuffer(multi_buffer, None, window, cx)
        });

        let (layout_mode, stacked, telescope) =
            if let Some(saved) = cx.try_global::<SavedQuickSearchLayout>() {
                (
                    saved.layout_mode,
                    StackedLayout {
                        results_height: saved.stacked_results_height,
                        preview_height: saved.stacked_preview_height,
                    },
                    TelescopeLayout {
                        content_height: saved.telescope_content_height,
                        preview_width: saved.telescope_preview_width,
                    },
                )
            } else {
                (
                    LayoutMode::default(),
                    StackedLayout::new(),
                    TelescopeLayout::new(),
                )
            };

        Self {
            preview_editor,
            layout_mode,
            stacked,
            telescope,
        }
    }

    fn save_layout(&self, cx: &mut Context<Self>) {
        cx.set_global(SavedQuickSearchLayout {
            layout_mode: self.layout_mode,
            stacked_results_height: self.stacked.results_height,
            stacked_preview_height: self.stacked.preview_height,
            telescope_content_height: self.telescope.content_height,
            telescope_preview_width: self.telescope.preview_width,
        });
    }

    fn auto_save_when_edited(
        preview_editor: &Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) -> Subscription {
        cx.subscribe_in(
            preview_editor,
            window,
            |_, _, _: &EditorEvent, _, _| todo!(),
        )
    }

    fn update(&mut self, update: Match, window: &mut Window, cx: &mut impl AppContext) {
        let Ok(update) = update.downcast::<Update>() else {
            return;
        };
        self.current_path = cx.read_entity(&update.buffer, |buffer, _| {
            buffer.file().map(|file| file.path()).map(Arc::clone)
        });

        let buffer = update.buffer;
        let range = update.range;
        let anchor_range = update.anchor_range;

        self.preview_editor.update(cx, |editor, cx| {
            let multi_buffer = editor.buffer().clone();
            let max_point = buffer.read(cx).max_point();

            multi_buffer.update(cx, |multi_buffer, cx| {
                multi_buffer.clear(cx);
                multi_buffer.set_excerpts_for_buffer(
                    buffer.clone(),
                    [rope::Point::new(0, 0)..max_point],
                    0,
                    cx,
                );
            });

            let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
            if let (Some(start_anchor), Some(end_anchor)) = (
                multi_buffer_snapshot.anchor_in_excerpt(anchor_range.start),
                multi_buffer_snapshot.anchor_in_excerpt(anchor_range.end),
            ) {
                editor.highlight_rows::<SearchMatchLineHighlight>(
                    start_anchor..start_anchor,
                    cx.theme().colors().editor_active_line_background,
                    RowHighlightOptions::default(),
                    cx,
                );

                editor.highlight_background(
                    HighlightKey::QuickSearchView,
                    &[start_anchor..end_anchor],
                    |_, theme| theme.colors().search_match_background,
                    cx,
                );
            }

            let start = multi_buffer::MultiBufferOffset(range.start);
            let end = multi_buffer::MultiBufferOffset(range.end);
            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::center()),
                window,
                cx,
                |s| {
                    s.select_ranges([start..end]);
                },
            );
        });
    }
}
