use chrono::DateTime;
use gpui3::{px, relative, rems, view, Context, Size, View};

use crate::{
    hello_world_rust_editor_with_status_example, theme, v_stack, AssistantPanel, Button,
    ChatMessage, ChatPanel, CollabPanel, EditorPane, Label, LanguageSelector, Pane, PaneGroup,
    Panel, PanelAllowedSides, PanelSide, ProjectPanel, SplitDirection, StatusBar, Terminal,
    TitleBar, Toast, ToastOrigin,
};
use crate::{prelude::*, NotificationToast};

#[derive(Clone)]
pub struct Workspace {
    title_bar: View<TitleBar>,
    show_project_panel: bool,
    show_collab_panel: bool,
    show_chat_panel: bool,
    show_assistant_panel: bool,
    show_terminal: bool,
    show_language_selector: bool,
    left_panel_scroll_state: ScrollState,
    right_panel_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
    bottom_panel_scroll_state: ScrollState,
}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            title_bar: TitleBar::view(cx),
            show_project_panel: true,
            show_collab_panel: false,
            show_chat_panel: true,
            show_assistant_panel: false,
            show_terminal: true,
            show_language_selector: false,
            left_panel_scroll_state: ScrollState::default(),
            right_panel_scroll_state: ScrollState::default(),
            tab_bar_scroll_state: ScrollState::default(),
            bottom_panel_scroll_state: ScrollState::default(),
        }
    }

    pub fn is_project_panel_open(&self) -> bool {
        self.show_project_panel
    }

    pub fn toggle_project_panel(&mut self, cx: &mut ViewContext<Self>) {
        self.show_project_panel = !self.show_project_panel;

        self.show_collab_panel = false;

        cx.notify();
    }

    pub fn is_collab_panel_open(&self) -> bool {
        self.show_collab_panel
    }

    pub fn toggle_collab_panel(&mut self) {
        self.show_collab_panel = !self.show_collab_panel;

        self.show_project_panel = false;
    }

    pub fn is_terminal_open(&self) -> bool {
        self.show_terminal
    }

    pub fn toggle_terminal(&mut self, cx: &mut ViewContext<Self>) {
        self.show_terminal = !self.show_terminal;

        cx.notify();
    }

    pub fn is_chat_panel_open(&self) -> bool {
        self.show_chat_panel
    }

    pub fn toggle_chat_panel(&mut self, cx: &mut ViewContext<Self>) {
        self.show_chat_panel = !self.show_chat_panel;

        self.show_assistant_panel = false;

        cx.notify();
    }

    pub fn is_assistant_panel_open(&self) -> bool {
        self.show_assistant_panel
    }

    pub fn toggle_assistant_panel(&mut self, cx: &mut ViewContext<Self>) {
        self.show_assistant_panel = !self.show_assistant_panel;

        self.show_chat_panel = false;

        cx.notify();
    }

    pub fn is_language_selector_open(&self) -> bool {
        self.show_language_selector
    }

    pub fn toggle_language_selector(&mut self, cx: &mut ViewContext<Self>) {
        self.show_language_selector = !self.show_language_selector;

        cx.notify();
    }

    pub fn view(cx: &mut WindowContext) -> View<Self> {
        view(cx.entity(|cx| Self::new(cx)), Self::render)
    }

    pub fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
        let theme = theme(cx).clone();

        let temp_size = rems(36.).into();

        let root_group = PaneGroup::new_groups(
            vec![
                PaneGroup::new_panes(
                    vec![
                        Pane::new(
                            ScrollState::default(),
                            Size {
                                width: relative(1.).into(),
                                height: temp_size,
                            },
                        )
                        .child(EditorPane::new(
                            hello_world_rust_editor_with_status_example(&theme),
                        )),
                        Pane::new(
                            ScrollState::default(),
                            Size {
                                width: relative(1.).into(),
                                height: temp_size,
                            },
                        )
                        .child(Terminal::new()),
                    ],
                    SplitDirection::Vertical,
                ),
                PaneGroup::new_panes(
                    vec![Pane::new(
                        ScrollState::default(),
                        Size {
                            width: relative(1.).into(),
                            height: relative(1.).into(),
                        },
                    )
                    .child(EditorPane::new(
                        hello_world_rust_editor_with_status_example(&theme),
                    ))],
                    SplitDirection::Vertical,
                ),
            ],
            SplitDirection::Horizontal,
        );

        div()
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans Extended")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.lowest.base.default.foreground)
            .fill(theme.lowest.base.default.background)
            .child(self.title_bar.clone())
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .border_t()
                    .border_b()
                    .border_color(theme.lowest.base.default.border)
                    .children(
                        Some(
                            Panel::new(self.left_panel_scroll_state.clone())
                                .side(PanelSide::Left)
                                .child(ProjectPanel::new(ScrollState::default())),
                        )
                        .filter(|_| self.is_project_panel_open()),
                    )
                    .children(
                        Some(
                            Panel::new(self.left_panel_scroll_state.clone())
                                .child(CollabPanel::new(ScrollState::default()))
                                .side(PanelSide::Left),
                        )
                        .filter(|_| self.is_collab_panel_open()),
                    )
                    .child(
                        v_stack()
                            .flex_1()
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    // CSS Hack: Flex 1 has to have a set height to properly fill the space
                                    // Or it will give you a height of 0
                                    // Marshall: We may not need this anymore with `gpui3`. It seems to render
                                    //           fine without it.
                                    .h_px()
                                    .child(root_group),
                            )
                            .children(
                                Some(
                                    Panel::new(self.bottom_panel_scroll_state.clone())
                                        .child(Terminal::new())
                                        .allowed_sides(PanelAllowedSides::BottomOnly)
                                        .side(PanelSide::Bottom),
                                )
                                .filter(|_| self.is_terminal_open()),
                            ),
                    )
                    .children(
                        Some(
                            Panel::new(self.right_panel_scroll_state.clone())
                                .side(PanelSide::Right)
                                .child(ChatPanel::new(ScrollState::default()).messages(vec![
                                    ChatMessage::new(
                                        "osiewicz".to_string(),
                                        "is this thing on?".to_string(),
                                        DateTime::parse_from_rfc3339("2023-09-27T15:40:52.707Z")
                                            .unwrap()
                                            .naive_local(),
                                    ),
                                    ChatMessage::new(
                                        "maxdeviant".to_string(),
                                        "Reading you loud and clear!".to_string(),
                                        DateTime::parse_from_rfc3339("2023-09-28T15:40:52.707Z")
                                            .unwrap()
                                            .naive_local(),
                                    ),
                                ])),
                        )
                        .filter(|_| self.is_chat_panel_open()),
                    )
                    .children(
                        Some(
                            Panel::new(self.right_panel_scroll_state.clone())
                                .child(AssistantPanel::new()),
                        )
                        .filter(|_| self.is_assistant_panel_open()),
                    ),
            )
            .child(StatusBar::new())
            .children(
                Some(
                    div()
                        .absolute()
                        .top(px(50.))
                        .left(px(640.))
                        .z_index(999)
                        .child(LanguageSelector::new()),
                )
                .filter(|_| self.is_language_selector_open()),
            )
            .child(Toast::new(ToastOrigin::Bottom).child(Label::new("A toast")))
            // .child(Toast::new(ToastOrigin::BottomRight).child(Label::new("Another toast")))
            .child(NotificationToast::new(
                "Can't pull changes from origin",
                "Your local branch is behind the remote branch. Please pull the latest changes before pushing.",
                Button::new("Stash & Switch").variant(ButtonVariant::Filled),
            ).secondary_action(Button::new("Cancel")))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;

    pub struct WorkspaceStory {
        workspace: View<Workspace>,
    }

    impl WorkspaceStory {
        pub fn view(cx: &mut WindowContext) -> View<Self> {
            view(
                cx.entity(|cx| Self {
                    workspace: Workspace::view(cx),
                }),
                |view, cx| view.workspace.clone(),
            )
        }
    }
}
