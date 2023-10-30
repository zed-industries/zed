use std::sync::Arc;

use chrono::DateTime;
use gpui2::{px, relative, rems, AppContext, Context, Size, View};

use crate::{prelude::*, NotificationsPanel};
use crate::{
    static_livestream, user_settings_mut, v_stack, AssistantPanel, Button, ChatMessage, ChatPanel,
    CollabPanel, EditorPane, FakeSettings, Label, LanguageSelector, Pane, PaneGroup, Panel,
    PanelAllowedSides, PanelSide, ProjectPanel, SettingValue, SplitDirection, StatusBar, Terminal,
    TitleBar, Toast, ToastOrigin,
};

#[derive(Clone)]
pub struct Gpui2UiDebug {
    pub in_livestream: bool,
    pub enable_user_settings: bool,
    pub show_toast: bool,
}

impl Default for Gpui2UiDebug {
    fn default() -> Self {
        Self {
            in_livestream: false,
            enable_user_settings: false,
            show_toast: false,
        }
    }
}

#[derive(Clone)]
pub struct Workspace {
    title_bar: View<TitleBar>,
    editor_1: View<EditorPane>,
    show_project_panel: bool,
    show_collab_panel: bool,
    show_chat_panel: bool,
    show_assistant_panel: bool,
    show_notifications_panel: bool,
    show_terminal: bool,
    show_debug: bool,
    show_language_selector: bool,
    debug: Gpui2UiDebug,
}

impl Workspace {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            title_bar: TitleBar::view(cx, None),
            editor_1: EditorPane::view(cx),
            show_project_panel: true,
            show_collab_panel: false,
            show_chat_panel: false,
            show_assistant_panel: false,
            show_terminal: true,
            show_language_selector: false,
            show_debug: false,
            show_notifications_panel: true,
            debug: Gpui2UiDebug::default(),
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
        self.show_notifications_panel = false;

        cx.notify();
    }

    pub fn is_notifications_panel_open(&self) -> bool {
        self.show_notifications_panel
    }

    pub fn toggle_notifications_panel(&mut self, cx: &mut ViewContext<Self>) {
        self.show_notifications_panel = !self.show_notifications_panel;

        self.show_chat_panel = false;
        self.show_assistant_panel = false;

        cx.notify();
    }

    pub fn is_assistant_panel_open(&self) -> bool {
        self.show_assistant_panel
    }

    pub fn toggle_assistant_panel(&mut self, cx: &mut ViewContext<Self>) {
        self.show_assistant_panel = !self.show_assistant_panel;

        self.show_chat_panel = false;
        self.show_notifications_panel = false;

        cx.notify();
    }

    pub fn is_language_selector_open(&self) -> bool {
        self.show_language_selector
    }

    pub fn toggle_language_selector(&mut self, cx: &mut ViewContext<Self>) {
        self.show_language_selector = !self.show_language_selector;

        cx.notify();
    }

    pub fn toggle_debug(&mut self, cx: &mut ViewContext<Self>) {
        self.show_debug = !self.show_debug;

        cx.notify();
    }

    pub fn debug_toggle_user_settings(&mut self, cx: &mut ViewContext<Self>) {
        self.debug.enable_user_settings = !self.debug.enable_user_settings;

        cx.notify();
    }

    pub fn debug_toggle_livestream(&mut self, cx: &mut ViewContext<Self>) {
        self.debug.in_livestream = !self.debug.in_livestream;

        self.title_bar = TitleBar::view(
            cx,
            Some(static_livestream()).filter(|_| self.debug.in_livestream),
        );

        cx.notify();
    }

    pub fn debug_toggle_toast(&mut self, cx: &mut ViewContext<Self>) {
        self.debug.show_toast = !self.debug.show_toast;

        cx.notify();
    }

    pub fn view(cx: &mut AppContext) -> View<Self> {
        {
            let state = cx.build_model(|cx| Self::new(cx));
            let render = Self::render;
            View::for_handle(state, render)
        }
    }

    pub fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Component<Self> {
        let theme = theme(cx);

        // HACK: This should happen inside of `debug_toggle_user_settings`, but
        // we don't have `cx.global::<FakeSettings>()` in event handlers at the moment.
        // Need to talk with Nathan/Antonio about this.
        {
            let settings = user_settings_mut(cx);

            if self.debug.enable_user_settings {
                settings.list_indent_depth = SettingValue::UserDefined(rems(0.5).into());
                settings.ui_scale = SettingValue::UserDefined(1.25);
            } else {
                *settings = FakeSettings::default();
            }
        }

        let root_group = PaneGroup::new_panes(
            vec![Pane::new(
                "pane-0",
                Size {
                    width: relative(1.).into(),
                    height: relative(1.).into(),
                },
            )
            .child(self.editor_1.clone())],
            SplitDirection::Horizontal,
        );

        div()
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .font("Zed Sans")
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(theme.text)
            .bg(theme.background)
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
                    .border_color(theme.border)
                    .children(
                        Some(
                            Panel::new("project-panel-outer", cx)
                                .side(PanelSide::Left)
                                .child(ProjectPanel::new("project-panel-inner")),
                        )
                        .filter(|_| self.is_project_panel_open()),
                    )
                    .children(
                        Some(
                            Panel::new("collab-panel-outer", cx)
                                .child(CollabPanel::new("collab-panel-inner"))
                                .side(PanelSide::Left),
                        )
                        .filter(|_| self.is_collab_panel_open()),
                    )
                    // .child(NotificationToast::new(
                    //     "maxbrunsfeld has requested to add you as a contact.".into(),
                    // ))
                    .child(
                        v_stack()
                            .flex_1()
                            .h_full()
                            .child(div().flex().flex_1().child(root_group))
                            .children(
                                Some(
                                    Panel::new("terminal-panel", cx)
                                        .child(Terminal::new())
                                        .allowed_sides(PanelAllowedSides::BottomOnly)
                                        .side(PanelSide::Bottom),
                                )
                                .filter(|_| self.is_terminal_open()),
                            ),
                    )
                    .children(
                        Some(
                            Panel::new("chat-panel-outer", cx)
                                .side(PanelSide::Right)
                                .child(ChatPanel::new("chat-panel-inner").messages(vec![
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
                            Panel::new("notifications-panel-outer", cx)
                                .side(PanelSide::Right)
                                .child(NotificationsPanel::new("notifications-panel-inner")),
                        )
                        .filter(|_| self.is_notifications_panel_open()),
                    )
                    .children(
                        Some(
                            Panel::new("assistant-panel-outer", cx)
                                .child(AssistantPanel::new("assistant-panel-inner")),
                        )
                        .filter(|_| self.is_assistant_panel_open()),
                    ),
            )
            .child(StatusBar::new())
            .when(self.debug.show_toast, |this| {
                this.child(Toast::new(ToastOrigin::Bottom).child(Label::new("A toast")))
            })
            .children(
                Some(
                    div()
                        .absolute()
                        .top(px(50.))
                        .left(px(640.))
                        .z_index(8)
                        .child(LanguageSelector::new("language-selector")),
                )
                .filter(|_| self.is_language_selector_open()),
            )
            .z_index(8)
            // Debug
            .child(
                v_stack()
                    .z_index(9)
                    .absolute()
                    .bottom_10()
                    .left_1_4()
                    .w_40()
                    .gap_2()
                    .when(self.show_debug, |this| {
                        this.child(Button::<Workspace>::new("Toggle User Settings").on_click(
                            Arc::new(|workspace, cx| workspace.debug_toggle_user_settings(cx)),
                        ))
                        .child(
                            Button::<Workspace>::new("Toggle Toasts").on_click(Arc::new(
                                |workspace, cx| workspace.debug_toggle_toast(cx),
                            )),
                        )
                        .child(
                            Button::<Workspace>::new("Toggle Livestream").on_click(Arc::new(
                                |workspace, cx| workspace.debug_toggle_livestream(cx),
                            )),
                        )
                    })
                    .child(
                        Button::<Workspace>::new("Toggle Debug")
                            .on_click(Arc::new(|workspace, cx| workspace.toggle_debug(cx))),
                    ),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui2::VisualContext;

    use super::*;

    pub struct WorkspaceStory {
        workspace: View<Workspace>,
    }

    impl WorkspaceStory {
        pub fn view(cx: &mut WindowContext) -> View<Self> {
            cx.build_view(
                |cx| Self {
                    workspace: Workspace::view(cx),
                },
                |view, cx| view.workspace.clone(),
            )
        }
    }
}
