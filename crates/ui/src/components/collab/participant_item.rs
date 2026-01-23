use gpui::{AnyElement, ClickEvent, Hsla, ImageSource, IntoElement, SharedString};

use crate::{Avatar, prelude::*};

/// Represents a shared project from a participant.
#[derive(Debug, Clone)]
pub struct ParticipantProject {
    /// The display name of the project (worktree root names joined).
    pub name: SharedString,
    /// Whether this is the last item in the list (for rendering the tree branch).
    pub is_last: bool,
}

/// Represents a shared screen from a participant.
#[derive(Debug, Clone)]
pub struct ParticipantScreen {
    /// Whether this is the last item in the list (for rendering the tree branch).
    pub is_last: bool,
}

#[derive(RegisterComponent, IntoElement)]
pub struct ParticipantItem {
    avatar_src: Option<ImageSource>,
    display_name: SharedString,
    is_current_user: bool,
    is_muted: bool,
    is_deafened: bool,
    is_speaking: bool,
    is_guest: bool,
    is_following: bool,
    /// The player color for this participant (used for following indicator).
    player_color: Option<Hsla>,
    /// Projects shared by this participant.
    projects: Vec<ParticipantProject>,
    /// Screen share state for this participant.
    screen: Option<ParticipantScreen>,
    /// Called when the participant row is clicked (for follow/unfollow).
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    /// Called when a project is clicked.
    on_project_click: Option<Box<dyn Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static>>,
    /// Called when the screen share is clicked.
    on_screen_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ParticipantItem {
    pub fn new(display_name: impl Into<SharedString>) -> Self {
        Self {
            avatar_src: None,
            display_name: display_name.into(),
            is_current_user: false,
            is_muted: false,
            is_deafened: false,
            is_speaking: false,
            is_guest: false,
            is_following: false,
            player_color: None,
            projects: Vec::new(),
            screen: None,
            on_click: None,
            on_project_click: None,
            on_screen_click: None,
        }
    }

    pub fn avatar(mut self, src: impl Into<ImageSource>) -> Self {
        self.avatar_src = Some(src.into());
        self
    }

    pub fn current_user(mut self, is_current_user: bool) -> Self {
        self.is_current_user = is_current_user;
        self
    }

    pub fn muted(mut self, is_muted: bool) -> Self {
        self.is_muted = is_muted;
        self
    }

    pub fn guest(mut self, is_guest: bool) -> Self {
        self.is_guest = is_guest;
        self
    }

    pub fn deafened(mut self, is_deafened: bool) -> Self {
        self.is_deafened = is_deafened;
        self
    }

    pub fn speaking(mut self, is_speaking: bool) -> Self {
        self.is_speaking = is_speaking;
        self
    }

    pub fn following(mut self, is_following: bool) -> Self {
        self.is_following = is_following;
        self
    }

    /// Set the player color for this participant (used for following indicator).
    pub fn player_color(mut self, color: Hsla) -> Self {
        self.player_color = Some(color);
        self
    }

    /// Add projects shared by this participant.
    pub fn projects(mut self, projects: Vec<ParticipantProject>) -> Self {
        self.projects = projects;
        self
    }

    /// Set the screen share state for this participant.
    pub fn screen(mut self, screen: ParticipantScreen) -> Self {
        self.screen = Some(screen);
        self
    }

    /// Set the click handler for the participant row (follow/unfollow).
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Set the click handler for project items.
    /// The handler receives the project index.
    pub fn on_project_click(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_project_click = Some(Box::new(handler));
        self
    }

    /// Set the click handler for the screen share item.
    pub fn on_screen_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_screen_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ParticipantItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let display_name = self.display_name.clone();

        let following_bg = if self.is_following {
            self.player_color
                .map(|c| c.opacity(0.15))
                .unwrap_or_else(|| cx.theme().colors().element_selected)
        } else {
            cx.theme().colors().elevated_surface_background
        };

        let participant_row = h_flex()
            .id(SharedString::from(format!("player-{}", display_name)))
            .border_b_1()
            .py_1()
            .px_2()
            .w_full()
            .justify_between()
            .bg(following_bg)
            .hover(|s| s.bg(cx.theme().colors().element_hover))
            .when(!self.is_current_user, |this| this.cursor_pointer())
            .when_some(self.on_click, |this, handler| {
                this.on_click(move |event, window, cx| {
                    handler(event, window, cx);
                })
            })
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Avatar::new(self.avatar_src.clone().unwrap_or_else(|| {
                            "https://avatars.githubusercontent.com/u/1?v=4".into()
                        }))
                        .when(self.is_following, |this| {
                            if let Some(color) = self.player_color {
                                this.border_color(color)
                            } else {
                                this
                            }
                        }),
                    )
                    .child(Label::new(self.display_name.clone()).size(LabelSize::Small))
                    .when(self.is_guest, |this| {
                        this.child(
                            Label::new("Guest")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        )
                    }),
            )
            .child(
                h_flex()
                    .gap_1()
                    .when(self.is_speaking, |this| {
                        this.child(
                            Icon::new(IconName::AudioOn)
                                .size(IconSize::Small)
                                .color(Color::Success),
                        )
                    })
                    .when(self.is_muted && !self.is_speaking, |this| {
                        this.child(Icon::new(IconName::MicMute).size(IconSize::Small).color(
                            if self.is_current_user {
                                Color::Error
                            } else {
                                Color::Muted
                            },
                        ))
                    })
                    .when(self.is_deafened, |this| {
                        this.child(Icon::new(IconName::AudioOff).size(IconSize::Small).color(
                            if self.is_current_user {
                                Color::Error
                            } else {
                                Color::Muted
                            },
                        ))
                    }),
            );

        let mut result = v_flex().w_full().child(participant_row);

        for (index, project) in self.projects.iter().enumerate() {
            let project_name = project.name.clone();
            let is_last = project.is_last && self.screen.is_none();

            let project_row = h_flex()
                .id(SharedString::from(format!(
                    "project-{}-{}",
                    self.display_name, index
                )))
                .cursor_pointer()
                .py_0p5()
                .px_2()
                .w_full()
                .gap_2()
                .border_b_1()
                .hover(|s| s.bg(cx.theme().colors().element_hover))
                .child(
                    h_flex().h_5().w_4().flex_shrink_0().justify_center().child(
                        div()
                            .w_px()
                            .when(is_last, |this| this.h_2p5())
                            .when(!is_last, |this| this.h_full())
                            .bg(cx.theme().colors().border),
                    ),
                )
                .child(
                    Icon::new(IconName::Folder)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Label::new(project_name)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                );

            if let Some(ref handler) = self.on_project_click {
                let handler_ptr = handler as *const _;
                result = result.child(project_row.on_click(move |event, window, cx| {
                    let handler = unsafe {
                        &*(handler_ptr
                            as *const Box<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>)
                    };
                    handler(index, event, window, cx);
                }));
            } else {
                result = result.child(project_row);
            }
        }

        if let Some(ref screen) = self.screen {
            let screen_row = h_flex()
                .id(SharedString::from(format!("screen-{}", self.display_name)))
                .cursor_pointer()
                .py_0p5()
                .px_2()
                .w_full()
                .gap_2()
                .border_b_1()
                .hover(|s| s.bg(cx.theme().colors().element_hover))
                .child(
                    h_flex().h_5().w_4().flex_shrink_0().justify_center().child(
                        div()
                            .w_px()
                            .when(screen.is_last, |this| this.h_2p5())
                            .when(!screen.is_last, |this| this.h_full())
                            .bg(cx.theme().colors().border),
                    ),
                )
                .child(
                    Icon::new(IconName::Screen)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    Label::new("Screen")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                );

            if let Some(ref handler) = self.on_screen_click {
                let handler_ptr = handler as *const _;
                result = result.child(screen_row.on_click(move |event, window, cx| {
                    let handler = unsafe {
                        &*(handler_ptr as *const Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>)
                    };
                    handler(event, window, cx);
                }));
            } else {
                result = result.child(screen_row);
            }
        }

        result
    }
}

impl Component for ParticipantItem {
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let ex_container = v_flex()
            .w_80()
            .gap_2()
            .border_1()
            .border_color(cx.theme().colors().border)
            .p_2();

        let player_color = cx.theme().players().color_for_participant(1).cursor;

        let examples = vec![
            single_example(
                "Basic participant",
                h_flex()
                    .w_80()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(ParticipantItem::new("Alice"))
                    .into_any_element(),
            ),
            single_example(
                "Current user (muted)",
                h_flex()
                    .w_80()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(ParticipantItem::new("You").current_user(true).muted(true))
                    .into_any_element(),
            ),
            single_example(
                "Following participant",
                h_flex()
                    .w_80()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        ParticipantItem::new("Bob")
                            .following(true)
                            .player_color(player_color),
                    )
                    .into_any_element(),
            ),
            single_example(
                "With projects and screen",
                ex_container
                    .child(
                        ParticipantItem::new("Charlie")
                            .speaking(true)
                            .projects(vec![
                                ParticipantProject {
                                    name: "zed".into(),
                                    is_last: false,
                                },
                                ParticipantProject {
                                    name: "gpui".into(),
                                    is_last: true,
                                },
                            ])
                            .screen(ParticipantScreen { is_last: true }),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Guest participant",
                h_flex()
                    .w_80()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .child(ParticipantItem::new("Guest User").guest(true))
                    .into_any_element(),
            ),
        ];

        Some(example_group(examples).vertical().into_any_element())
    }
}
