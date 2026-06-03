use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use editor::scroll::Autoscroll;
use editor::{
    Editor, EditorEvent, HighlightKey, MultiBuffer, RowHighlightOptions, SelectionEffects,
};
use gpui::{App, FocusHandle, Subscription, Task};
use gpui::{AppContext, Context, Entity, Window};
use language::Buffer;
use project::Project;
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
    content: Entity<EditorPreview>,
    pub layout: LayoutMode,
}

type Match = Box<dyn std::any::Any>;

impl Preview {
    pub fn new_editor(project: Entity<Project>, window: &mut Window, cx: &mut App) -> Self {
        Preview {
            content: cx.new(|cx| EditorPreview::new(project, window, cx)),
            layout: LayoutMode::default(),
        }
    }

    pub fn width(&self) -> ui::Pixels {
        match self.layout {
            LayoutMode::Stacked(_) => ui::Pixels::ZERO,
            LayoutMode::Telescope(layout) => layout.preview_width,
        }
    }

    pub fn height(&self) -> ui::Pixels {
        match self.layout {
            LayoutMode::Stacked(layout) => layout.preview_height,
            LayoutMode::Telescope(_) => ui::Pixels::ZERO,
        }
    }

    pub fn update(&mut self, update: Match, window: &mut Window, cx: &mut impl AppContext) {
        // self.content since this will become a match to support non editor previews
        self.content.update(cx, |content, cx| {
            content.update(update, window, cx);
        });
    }

    pub fn render(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // self.content since this will become a match to support non editor previews
        let layout = self.layout;
        self.content.update(cx, |content, cx| {
            content.render(layout, window, cx).into_any_element()
        })
    }
}

// pub struct Update {
//     buffer: Entity<Buffer>,
//     range: Range<usize>,
//     anchor_range: Range<language::Anchor>,
// }

pub struct Update {
    pub abs_path: PathBuf,
}

/// TODO! rename relative position
/// - wire up autosave for the editor

struct SearchMatchLineHighlight;

pub struct EditorPreview {
    project: Entity<Project>,
    current_path: Option<Arc<RelPath>>,
    // /// The buffer currently shown in the preview, if any.
    // buffer: Option<Entity<Buffer>>,
    preview_editor: Entity<Editor>,
    /// TODO! should probably be in Preview not here
    pub split_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    /// TODO! should probably be in Preview not here
    pub focus_handle: FocusHandle,
}

impl EditorPreview {
    fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let preview_editor = cx.new(|cx: &mut Context<Editor>| {
            let capability = language::Capability::ReadWrite; // Later narrowed per buffer
            let multi_buffer = cx.new(|_| MultiBuffer::without_headers(capability));
            Editor::for_multibuffer(multi_buffer, None, window, cx)
        });

        Self {
            project,
            preview_editor,
            current_path: None,
            // buffer: None,
            split_popover_menu_handle: PopoverMenuHandle::default(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn update(
        &mut self,
        update: Box<dyn std::any::Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Ok(update) = update.downcast::<Update>() else {
            return;
        };

        // TODO!(yara) debounce this/cache the last one for fast switching
        // between top two results.
        let open_task = self.project.update(cx, |project, cx| {
            match project.project_path_for_absolute_path(&update.abs_path, cx) {
                Some(project_path) => {
                    if let Some(buffer) = project.get_open_buffer(&project_path, cx) {
                        Task::ready(Ok(buffer))
                    } else {
                        project.open_buffer(project_path, cx)
                    }
                }
                None => project.open_local_buffer(&update.abs_path, cx),
            }
        });

        cx.spawn_in(window, async move |this, cx| {
            let buffer = open_task.await?;
            this.update(cx, |this, cx| {
                this.finish_update(buffer, cx);
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn finish_update(&mut self, buffer: Entity<Buffer>, cx: &mut App) {
        self.current_path = buffer.read(cx).file().map(|file| file.path().clone());

        let full_range = [rope::Point::zero()..buffer.read(cx).max_point()];
        self.preview_editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |multi_buffer, cx| {
                multi_buffer.clear(cx);
                multi_buffer.set_excerpts_for_buffer(buffer, full_range, 0, cx);
            });
        });
    }

    // fn update(&mut self, update: Match, window: &mut Window, cx: &mut impl AppContext) {
    //     let Ok(update) = update.downcast::<Update>() else {
    //         return;
    //     };
    //     self.current_path = cx.read_entity(&update.buffer, |buffer, _| {
    //         buffer.file().map(|file| file.path()).map(Arc::clone)
    //     });

    //     let buffer = update.buffer;
    //     let range = update.range;
    //     let anchor_range = update.anchor_range;

    //     self.preview_editor.update(cx, |editor, cx| {
    //         let multi_buffer = editor.buffer().clone();
    //         let max_point = buffer.read(cx).max_point();

    //         multi_buffer.update(cx, |multi_buffer, cx| {
    //             multi_buffer.clear(cx);
    //             multi_buffer.set_excerpts_for_buffer(
    //                 buffer.clone(),
    //                 [rope::Point::new(0, 0)..max_point],
    //                 0,
    //                 cx,
    //             );
    //         });

    //         let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
    //         if let (Some(start_anchor), Some(end_anchor)) = (
    //             multi_buffer_snapshot.anchor_in_excerpt(anchor_range.start),
    //             multi_buffer_snapshot.anchor_in_excerpt(anchor_range.end),
    //         ) {
    //             editor.highlight_rows::<SearchMatchLineHighlight>(
    //                 start_anchor..start_anchor,
    //                 cx.theme().colors().editor_active_line_background,
    //                 RowHighlightOptions::default(),
    //                 cx,
    //             );

    //             editor.highlight_background(
    //                 HighlightKey::QuickSearchView,
    //                 &[start_anchor..end_anchor],
    //                 |_, theme| theme.colors().search_match_background,
    //                 cx,
    //             );
    //         }

    //         let start = multi_buffer::MultiBufferOffset(range.start);
    //         let end = multi_buffer::MultiBufferOffset(range.end);
    //         editor.change_selections(
    //             SelectionEffects::scroll(Autoscroll::center()),
    //             window,
    //             cx,
    //             |s| {
    //                 s.select_ranges([start..end]);
    //             },
    //         );
    //     });
    // }
}
