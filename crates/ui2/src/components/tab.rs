use crate::prelude::*;
use crate::{Icon, IconColor, IconElement, Label, LabelColor};

#[derive(Component, Clone)]
pub struct Tab {
    id: ElementId,
    title: String,
    icon: Option<Icon>,
    current: bool,
    dirty: bool,
    fs_status: FileSystemStatus,
    git_status: GitStatus,
    diagnostic_status: DiagnosticStatus,
    close_side: IconSide,
}

#[derive(Clone, Debug)]
struct TabDragState {
    title: String,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: "untitled".to_string(),
            icon: None,
            current: false,
            dirty: false,
            fs_status: FileSystemStatus::None,
            git_status: GitStatus::None,
            diagnostic_status: DiagnosticStatus::None,
            close_side: IconSide::Right,
        }
    }

    pub fn current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn icon<I>(mut self, icon: I) -> Self
    where
        I: Into<Option<Icon>>,
    {
        self.icon = icon.into();
        self
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    pub fn fs_status(mut self, fs_status: FileSystemStatus) -> Self {
        self.fs_status = fs_status;
        self
    }

    pub fn git_status(mut self, git_status: GitStatus) -> Self {
        self.git_status = git_status;
        self
    }

    pub fn diagnostic_status(mut self, diagnostic_status: DiagnosticStatus) -> Self {
        self.diagnostic_status = diagnostic_status;
        self
    }

    pub fn close_side(mut self, close_side: IconSide) -> Self {
        self.close_side = close_side;
        self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let theme = theme(cx);
        let has_fs_conflict = self.fs_status == FileSystemStatus::Conflict;
        let is_deleted = self.fs_status == FileSystemStatus::Deleted;

        let label = match (self.git_status, is_deleted) {
            (_, true) | (GitStatus::Deleted, false) => Label::new(self.title.clone())
                .color(LabelColor::Hidden)
                .set_strikethrough(true),
            (GitStatus::None, false) => Label::new(self.title.clone()),
            (GitStatus::Created, false) => {
                Label::new(self.title.clone()).color(LabelColor::Created)
            }
            (GitStatus::Modified, false) => {
                Label::new(self.title.clone()).color(LabelColor::Modified)
            }
            (GitStatus::Renamed, false) => Label::new(self.title.clone()).color(LabelColor::Accent),
            (GitStatus::Conflict, false) => Label::new(self.title.clone()),
        };

        let close_icon = || IconElement::new(Icon::Close).color(IconColor::Muted);

        let (tab_bg, tab_hover_bg, tab_active_bg) = match self.current {
            true => (
                theme.ghost_element,
                theme.ghost_element_hover,
                theme.ghost_element_active,
            ),
            false => (
                theme.filled_element,
                theme.filled_element_hover,
                theme.filled_element_active,
            ),
        };

        let drag_state = TabDragState {
            title: self.title.clone(),
        };

        div()
            .id(self.id.clone())
            .on_drag(move |_view, _cx| {
                Drag::new(drag_state.clone(), |view, cx| div().w_8().h_4().bg(red()))
            })
            .drag_over::<TabDragState>(|d| d.bg(black()))
            .on_drop(|_view, state: TabDragState, cx| {
                dbg!(state);
            })
            .px_2()
            .py_0p5()
            .flex()
            .items_center()
            .justify_center()
            .bg(tab_bg)
            .hover(|h| h.bg(tab_hover_bg))
            .active(|a| a.bg(tab_active_bg))
            .child(
                div()
                    .px_1()
                    .flex()
                    .items_center()
                    .gap_1()
                    .children(has_fs_conflict.then(|| {
                        IconElement::new(Icon::ExclamationTriangle)
                            .size(crate::IconSize::Small)
                            .color(IconColor::Warning)
                    }))
                    .children(self.icon.map(IconElement::new))
                    .children(if self.close_side == IconSide::Left {
                        Some(close_icon())
                    } else {
                        None
                    })
                    .child(label)
                    .children(if self.close_side == IconSide::Right {
                        Some(close_icon())
                    } else {
                        None
                    }),
            )
    }
}

use gpui2::{black, red, Drag, ElementId};
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use strum::IntoEnumIterator;

    use crate::{h_stack, v_stack, Icon, Story};

    use super::*;

    #[derive(Component)]
    pub struct TabStory;

    impl TabStory {
        pub fn new() -> Self {
            Self
        }

        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            let git_statuses = GitStatus::iter();
            let fs_statuses = FileSystemStatus::iter();

            Story::container(cx)
                .child(Story::title_for::<_, Tab>(cx))
                .child(
                    h_stack().child(
                        v_stack()
                            .gap_2()
                            .child(Story::label(cx, "Default"))
                            .child(Tab::new("default")),
                    ),
                )
                .child(
                    h_stack().child(
                        v_stack().gap_2().child(Story::label(cx, "Current")).child(
                            h_stack()
                                .gap_4()
                                .child(
                                    Tab::new("current")
                                        .title("Current".to_string())
                                        .current(true),
                                )
                                .child(
                                    Tab::new("not_current")
                                        .title("Not Current".to_string())
                                        .current(false),
                                ),
                        ),
                    ),
                )
                .child(
                    h_stack().child(
                        v_stack()
                            .gap_2()
                            .child(Story::label(cx, "Titled"))
                            .child(Tab::new("titled").title("label".to_string())),
                    ),
                )
                .child(
                    h_stack().child(
                        v_stack()
                            .gap_2()
                            .child(Story::label(cx, "With Icon"))
                            .child(
                                Tab::new("with_icon")
                                    .title("label".to_string())
                                    .icon(Some(Icon::Envelope)),
                            ),
                    ),
                )
                .child(
                    h_stack().child(
                        v_stack()
                            .gap_2()
                            .child(Story::label(cx, "Close Side"))
                            .child(
                                h_stack()
                                    .gap_4()
                                    .child(
                                        Tab::new("left")
                                            .title("Left".to_string())
                                            .close_side(IconSide::Left),
                                    )
                                    .child(Tab::new("right").title("Right".to_string())),
                            ),
                    ),
                )
                .child(
                    v_stack()
                        .gap_2()
                        .child(Story::label(cx, "Git Status"))
                        .child(h_stack().gap_4().children(git_statuses.map(|git_status| {
                            Tab::new("git_status")
                                .title(git_status.to_string())
                                .git_status(git_status)
                        }))),
                )
                .child(
                    v_stack()
                        .gap_2()
                        .child(Story::label(cx, "File System Status"))
                        .child(h_stack().gap_4().children(fs_statuses.map(|fs_status| {
                            Tab::new("file_system_status")
                                .title(fs_status.to_string())
                                .fs_status(fs_status)
                        }))),
                )
        }
    }
}
