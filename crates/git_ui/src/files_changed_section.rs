use crate::git_status_icon;
use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, MouseButton, Pixels, Point,
    UniformListScrollHandle, Window, px, uniform_list,
};
use ui::{
    ButtonSize, Color, IconButton, IconName, IntoElement, Label, LabelSize, LineHeightStyle,
    ParentElement, Render, Tooltip, WithScrollbar, div, h_flex, prelude::*, v_flex,
};

/// Delegate trait for providing files changed data and handling events
pub trait FilesChangedDelegate: 'static {
    /// Get the files to display
    fn files(&self) -> &[CommitFileInfo];

    /// Get the currently selected file index
    fn selected_index(&self) -> Option<usize>;

    /// Set the selected file index
    fn set_selected_index(&mut self, index: Option<usize>);

    /// Handle file click (left click)
    fn on_file_click(&mut self, file_index: usize, window: &mut Window, cx: &mut App);

    /// Handle file context menu (right click)
    fn on_file_context_menu(
        &mut self,
        file_index: usize,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
}

#[derive(Clone, Debug)]
pub struct CommitFileInfo {
    pub path: String,
    pub status: FileStatus,
}

#[derive(Clone, Debug)]
pub struct FileClickedEvent;

#[derive(Clone, Debug)]
pub struct FileContextMenuRequestEvent {
    pub file_index: usize,
    pub position: Point<Pixels>,
}

#[derive(Clone, Debug)]
pub struct OpenAllFilesDiffEvent;

const FILE_ROW_HEIGHT: f32 = 24.0;

pub struct FilesChangedSection<D: FilesChangedDelegate> {
    focus_handle: FocusHandle,
    delegate: D,
    scroll_handle: UniformListScrollHandle,
}

impl<D: FilesChangedDelegate> FilesChangedSection<D> {
    pub fn new(delegate: D, focus_handle: FocusHandle) -> Self {
        Self {
            focus_handle,
            delegate,
            scroll_handle: UniformListScrollHandle::new(),
        }
    }

    pub fn render_file_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let files = self.delegate.files().to_vec();
        let file_count = files.len();

        uniform_list(
            "files-changed-list",
            file_count,
            cx.processor(
                move |this: &mut Self, range: std::ops::Range<usize>, _window, cx| {
                    let colors = cx.theme().colors();
                    let selected_idx = this.delegate.selected_index();

                    files
                        .iter()
                        .enumerate()
                        .skip(range.start)
                        .take(range.end - range.start)
                        .map(|(idx, file)| {
                            let is_selected = selected_idx == Some(idx);

                            // Parse the file path to separate directory from filename
                            let path = std::path::Path::new(&file.path);
                            let parent_dir = path.parent().and_then(|p| {
                                if p.as_os_str().is_empty() {
                                    None
                                } else {
                                    Some(p.to_string_lossy().to_string())
                                }
                            });
                            let filename = path
                                .file_name()
                                .map(|f| f.to_string_lossy().to_string())
                                .unwrap_or_else(|| file.path.clone());

                            let git_status = match file.status {
                                FileStatus::Added => {
                                    git::status::FileStatus::Tracked(git::status::TrackedStatus {
                                        index_status: git::status::StatusCode::Added,
                                        worktree_status: git::status::StatusCode::Unmodified,
                                    })
                                }
                                FileStatus::Modified => {
                                    git::status::FileStatus::Tracked(git::status::TrackedStatus {
                                        index_status: git::status::StatusCode::Modified,
                                        worktree_status: git::status::StatusCode::Unmodified,
                                    })
                                }
                                FileStatus::Deleted => {
                                    git::status::FileStatus::Tracked(git::status::TrackedStatus {
                                        index_status: git::status::StatusCode::Deleted,
                                        worktree_status: git::status::StatusCode::Unmodified,
                                    })
                                }
                            };

                            let path_color = Color::Muted;
                            let label_color = Color::Default;

                            div()
                                .id(("file-item", idx))
                                .group("files-changed")
                                .h(px(FILE_ROW_HEIGHT))
                                .px_2()
                                .w_full()
                                .flex()
                                .items_center()
                                .gap_2()
                                .when(is_selected, |div| div.bg(colors.element_selected))
                                .hover(|style| style.bg(colors.element_hover))
                                .on_click(cx.listener(move |this, _event, window, cx| {
                                    cx.emit(FileClickedEvent);
                                    this.delegate.on_file_click(idx, window, cx);
                                    cx.notify();
                                }))
                                .on_mouse_down(
                                    MouseButton::Right,
                                    cx.listener(
                                        move |this, event: &gpui::MouseDownEvent, window, cx| {
                                            cx.emit(FileContextMenuRequestEvent {
                                                file_index: idx,
                                                position: event.position,
                                            });
                                            this.delegate.on_file_context_menu(
                                                idx,
                                                event.position,
                                                window,
                                                cx,
                                            );
                                            cx.notify();
                                        },
                                    ),
                                )
                                .child(git_status_icon(git_status))
                                .child(
                                    h_flex()
                                        .items_center()
                                        .flex_1()
                                        .when_some(parent_dir, |this, parent| {
                                            this.child(
                                                Label::new(format!("{}/", parent))
                                                    .color(path_color)
                                                    .single_line()
                                                    .when(
                                                        file.status == FileStatus::Deleted,
                                                        |this| this.strikethrough(),
                                                    ),
                                            )
                                        })
                                        .child(
                                            Label::new(filename)
                                                .color(label_color)
                                                .single_line()
                                                .when(file.status == FileStatus::Deleted, |this| {
                                                    this.strikethrough()
                                                }),
                                        ),
                                )
                        })
                        .collect()
                },
            ),
        )
        .size_full()
        .track_scroll(self.scroll_handle.clone())
    }

    pub fn delegate(&self) -> &D {
        &self.delegate
    }

    pub fn delegate_mut(&mut self) -> &mut D {
        &mut self.delegate
    }

    pub fn get_selected_file(&self) -> Option<CommitFileInfo> {
        self.delegate
            .selected_index()
            .and_then(|index| self.delegate.files().get(index).cloned())
    }
}

impl<D: FilesChangedDelegate> Focusable for FilesChangedSection<D> {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl<D: FilesChangedDelegate> EventEmitter<DismissEvent> for FilesChangedSection<D> {}

impl<D: FilesChangedDelegate> EventEmitter<FileClickedEvent> for FilesChangedSection<D> {}

impl<D: FilesChangedDelegate> EventEmitter<FileContextMenuRequestEvent> for FilesChangedSection<D> {}

impl<D: FilesChangedDelegate> EventEmitter<OpenAllFilesDiffEvent> for FilesChangedSection<D> {}

impl<D: FilesChangedDelegate> Render for FilesChangedSection<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let files = self.delegate.files();
        let file_count = files.len();

        v_flex()
            .size_full()
            .child(
                h_flex()
                    .px_2()
                    .py_2()
                    .pb(rems(0.3125))
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new(format!("Files Changed ({})", file_count))
                            .color(Color::Muted)
                            .size(LabelSize::Small)
                            .line_height_style(LineHeightStyle::UiLabel)
                            .single_line(),
                    )
                    .when(file_count > 0, |this| {
                        this.child(
                            IconButton::new("open-all-files-diff", IconName::Diff)
                                .size(ButtonSize::Compact)
                                .tooltip(Tooltip::text("Open diff for all files"))
                                .on_click(cx.listener(|_this, _event, _window, cx| {
                                    cx.emit(OpenAllFilesDiffEvent);
                                })),
                        )
                    }),
            )
            .child(
                div()
                    .flex_1()
                    .size_full()
                    .child(self.render_file_list(cx))
                    .custom_scrollbars(
                        ui::Scrollbars::new(ui::ScrollAxes::Vertical)
                            .tracked_scroll_handle(self.scroll_handle.clone())
                            .id("files-changed-scrollbar"),
                        window,
                        cx,
                    ),
            )
    }
}
