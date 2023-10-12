use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use chrono::DateTime;
use gpui3::{px, relative, rems, view, Context, Size, View};

use crate::prelude::*;
use crate::{
    hello_world_rust_editor_with_status_example, random_players_with_call_status, theme, v_stack,
    AssistantPanel, ChatMessage, ChatPanel, CollabPanel, EditorPane, Label, LanguageSelector,
    Livestream, Pane, PaneGroup, Panel, PanelAllowedSides, PanelSide, ProjectPanel, SplitDirection,
    StatusBar, Terminal, TitleBar, Toast, ToastOrigin,
};

pub struct WorkspaceState {
    pub show_project_panel: Arc<AtomicBool>,
    pub show_collab_panel: Arc<AtomicBool>,
    pub show_chat_panel: Arc<AtomicBool>,
    pub show_assistant_panel: Arc<AtomicBool>,
    pub show_terminal: Arc<AtomicBool>,
    pub show_language_selector: Arc<AtomicBool>,
}

impl WorkspaceState {
    fn toggle_value(current_value: &AtomicBool) {
        let value = current_value.load(Ordering::SeqCst);

        current_value
            .compare_exchange(value, !value, Ordering::SeqCst, Ordering::SeqCst)
            .unwrap();
    }

    pub fn is_project_panel_open(&self) -> bool {
        self.show_project_panel.load(Ordering::SeqCst)
    }

    pub fn toggle_project_panel(&self) {
        Self::toggle_value(&self.show_project_panel);

        self.show_collab_panel.store(false, Ordering::SeqCst);
    }

    pub fn is_collab_panel_open(&self) -> bool {
        self.show_collab_panel.load(Ordering::SeqCst)
    }

    pub fn toggle_collab_panel(&self) {
        Self::toggle_value(&self.show_collab_panel);

        self.show_project_panel.store(false, Ordering::SeqCst);
    }

    pub fn is_terminal_open(&self) -> bool {
        self.show_terminal.load(Ordering::SeqCst)
    }

    pub fn toggle_terminal(&self) {
        Self::toggle_value(&self.show_terminal);
    }

    pub fn is_chat_panel_open(&self) -> bool {
        self.show_chat_panel.load(Ordering::SeqCst)
    }

    pub fn toggle_chat_panel(&self) {
        Self::toggle_value(&self.show_chat_panel);

        self.show_assistant_panel.store(false, Ordering::SeqCst);
    }

    pub fn is_assistant_panel_open(&self) -> bool {
        self.show_assistant_panel.load(Ordering::SeqCst)
    }

    pub fn toggle_assistant_panel(&self) {
        Self::toggle_value(&self.show_assistant_panel);

        self.show_chat_panel.store(false, Ordering::SeqCst);
    }

    pub fn is_language_selector_open(&self) -> bool {
        self.show_language_selector.load(Ordering::SeqCst)
    }

    pub fn toggle_language_selector(&self) {
        Self::toggle_value(&self.show_language_selector);
    }
}

/// HACK: This is just a temporary way to start hooking up interactivity until
/// I can get an explainer on how we should actually be managing state.
static WORKSPACE_STATE: OnceLock<WorkspaceState> = OnceLock::new();

pub fn get_workspace_state() -> &'static WorkspaceState {
    let state = WORKSPACE_STATE.get_or_init(|| WorkspaceState {
        show_project_panel: Arc::new(AtomicBool::new(true)),
        show_collab_panel: Arc::new(AtomicBool::new(false)),
        show_chat_panel: Arc::new(AtomicBool::new(true)),
        show_assistant_panel: Arc::new(AtomicBool::new(false)),
        show_terminal: Arc::new(AtomicBool::new(true)),
        show_language_selector: Arc::new(AtomicBool::new(false)),
    });

    state
}

// #[derive(Element)]
#[derive(Clone)]
pub struct Workspace {
    show_project_panel: bool,
    show_collab_panel: bool,
    left_panel_scroll_state: ScrollState,
    right_panel_scroll_state: ScrollState,
    tab_bar_scroll_state: ScrollState,
    bottom_panel_scroll_state: ScrollState,
}

fn workspace(cx: &mut WindowContext) -> View<Workspace> {
    view(cx.entity(|cx| Workspace::new()), Workspace::render)
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            show_project_panel: true,
            show_collab_panel: false,
            left_panel_scroll_state: ScrollState::default(),
            right_panel_scroll_state: ScrollState::default(),
            tab_bar_scroll_state: ScrollState::default(),
            bottom_panel_scroll_state: ScrollState::default(),
        }
    }

    pub fn is_project_panel_open(&self) -> bool {
        dbg!(self.show_project_panel)
    }

    pub fn toggle_project_panel(&mut self, cx: &mut ViewContext<Self>) {
        self.show_project_panel = !self.show_project_panel;

        self.show_collab_panel = false;

        dbg!(self.show_project_panel);

        cx.notify();
    }

    pub fn is_collab_panel_open(&self) -> bool {
        self.show_collab_panel
    }

    pub fn toggle_collab_panel(&mut self) {
        self.show_collab_panel = !self.show_collab_panel;

        self.show_project_panel = false;
    }

    pub fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
        let theme = theme(cx).clone();

        let workspace_state = get_workspace_state();

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
                            |_, payload| {
                                let theme = payload.downcast_ref::<Arc<Theme>>().unwrap();

                                vec![EditorPane::new(hello_world_rust_editor_with_status_example(
                                    &theme,
                                ))
                                .into_any()]
                            },
                            Box::new(theme.clone()),
                        ),
                        Pane::new(
                            ScrollState::default(),
                            Size {
                                width: relative(1.).into(),
                                height: temp_size,
                            },
                            |_, _| vec![Terminal::new().into_any()],
                            Box::new(()),
                        ),
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
                        |_, payload| {
                            let theme = payload.downcast_ref::<Arc<Theme>>().unwrap();

                            vec![EditorPane::new(hello_world_rust_editor_with_status_example(
                                &theme,
                            ))
                            .into_any()]
                        },
                        Box::new(theme.clone()),
                    )],
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
            .child(TitleBar::new(cx).set_livestream(Some(Livestream {
                players: random_players_with_call_status(7),
                channel: Some("gpui2-ui".to_string()),
            })))
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
                            Panel::new(
                                self.left_panel_scroll_state.clone(),
                                |_, payload| {
                                    vec![ProjectPanel::new(ScrollState::default()).into_any()]
                                },
                                Box::new(()),
                            )
                            .side(PanelSide::Left),
                        )
                        .filter(|_| workspace_state.is_project_panel_open()),
                    )
                    .children(
                        Some(
                            Panel::new(
                                self.left_panel_scroll_state.clone(),
                                |_, payload| {
                                    vec![CollabPanel::new(ScrollState::default()).into_any()]
                                },
                                Box::new(()),
                            )
                            .side(PanelSide::Left),
                        )
                        .filter(|_| workspace_state.is_collab_panel_open()),
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
                                    Panel::new(
                                        self.bottom_panel_scroll_state.clone(),
                                        |_, _| vec![Terminal::new().into_any()],
                                        Box::new(()),
                                    )
                                    .allowed_sides(PanelAllowedSides::BottomOnly)
                                    .side(PanelSide::Bottom),
                                )
                                .filter(|_| workspace_state.show_terminal.load(Ordering::SeqCst)),
                            ),
                    )
                    .children(
                        Some(
                            Panel::new(
                                self.right_panel_scroll_state.clone(),
                                |_, payload| {
                                    vec![ChatPanel::new(ScrollState::default())
                                        .with_messages(vec![
                                            ChatMessage::new(
                                                "osiewicz".to_string(),
                                                "is this thing on?".to_string(),
                                                DateTime::parse_from_rfc3339(
                                                    "2023-09-27T15:40:52.707Z",
                                                )
                                                .unwrap()
                                                .naive_local(),
                                            ),
                                            ChatMessage::new(
                                                "maxdeviant".to_string(),
                                                "Reading you loud and clear!".to_string(),
                                                DateTime::parse_from_rfc3339(
                                                    "2023-09-28T15:40:52.707Z",
                                                )
                                                .unwrap()
                                                .naive_local(),
                                            ),
                                        ])
                                        .into_any()]
                                },
                                Box::new(()),
                            )
                            .side(PanelSide::Right),
                        )
                        .filter(|_| workspace_state.is_chat_panel_open()),
                    )
                    .children(
                        Some(Panel::new(
                            self.right_panel_scroll_state.clone(),
                            |_, _| vec![AssistantPanel::new().into_any()],
                            Box::new(()),
                        ))
                        .filter(|_| workspace_state.is_assistant_panel_open()),
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
                .filter(|_| workspace_state.is_language_selector_open()),
            )
            .child(Toast::new(
                ToastOrigin::Bottom,
                |_, _| vec![Label::new("A toast").into_any()],
                Box::new(()),
            ))
            .child(Toast::new(
                ToastOrigin::BottomRight,
                |_, _| vec![Label::new("Another toast").into_any()],
                Box::new(()),
            ))
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

    pub fn workspace_story(cx: &mut WindowContext) -> View<WorkspaceStory> {
        view(
            cx.entity(|cx| WorkspaceStory {
                workspace: workspace(cx),
            }),
            |view, cx| view.workspace.clone(),
        )
    }
}
