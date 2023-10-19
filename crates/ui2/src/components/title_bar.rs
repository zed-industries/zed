use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use gpui3::{view, Context, View};

use crate::prelude::*;
use crate::settings::user_settings;
use crate::{
    random_players_with_call_status, theme, Avatar, Button, Icon, IconButton, IconColor, MicStatus,
    PlayerWithCallStatus, ScreenShareStatus, ToolDivider, TrafficLights,
};

#[derive(Clone)]
pub struct Livestream {
    pub players: Vec<PlayerWithCallStatus>,
    pub channel: Option<String>, // projects
                                 // windows
}

#[derive(Clone)]
pub struct TitleBar {
    /// If the window is active from the OS's perspective.
    is_active: Arc<AtomicBool>,
    livestream: Option<Livestream>,
    mic_status: MicStatus,
    is_deafened: bool,
    screen_share_status: ScreenShareStatus,
}

impl TitleBar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let is_active = Arc::new(AtomicBool::new(true));
        let active = is_active.clone();

        // cx.observe_window_activation(move |_, is_active, cx| {
        //     active.store(is_active, std::sync::atomic::Ordering::SeqCst);
        //     cx.notify();
        // })
        // .detach();

        Self {
            is_active,
            livestream: None,
            mic_status: MicStatus::Unmuted,
            is_deafened: false,
            screen_share_status: ScreenShareStatus::NotShared,
        }
    }

    pub fn set_livestream(mut self, livestream: Option<Livestream>) -> Self {
        self.livestream = livestream;
        self
    }

    pub fn is_mic_muted(&self) -> bool {
        self.mic_status == MicStatus::Muted
    }

    pub fn toggle_mic_status(&mut self, cx: &mut ViewContext<Self>) {
        self.mic_status = self.mic_status.inverse();

        // Undeafen yourself when unmuting the mic while deafened.
        if self.is_deafened && self.mic_status == MicStatus::Unmuted {
            self.is_deafened = false;
        }

        cx.notify();
    }

    pub fn toggle_deafened(&mut self, cx: &mut ViewContext<Self>) {
        self.is_deafened = !self.is_deafened;
        self.mic_status = MicStatus::Muted;

        cx.notify()
    }

    pub fn toggle_screen_share_status(&mut self, cx: &mut ViewContext<Self>) {
        self.screen_share_status = self.screen_share_status.inverse();

        cx.notify();
    }

    pub fn view(cx: &mut WindowContext) -> View<Self> {
        view(
            cx.entity(|cx| {
                Self::new(cx).set_livestream(Some(Livestream {
                    players: random_players_with_call_status(7),
                    channel: Some("gpui2-ui".to_string()),
                }))
            }),
            Self::render,
        )
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
        let color = ThemeColor::new(cx);
        let color = ThemeColor::new(cx);
        let setting = user_settings();

        // let has_focus = cx.window_is_active();
        let has_focus = true;

        let player_list = if let Some(livestream) = &self.livestream {
            livestream.players.clone().into_iter()
        } else {
            vec![].into_iter()
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .bg(color.background)
            .py_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_full()
                    .gap_4()
                    .px_2()
                    .child(TrafficLights::new().window_has_focus(has_focus))
                    // === Project Info === //
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .when(*setting.titlebar.show_project_owner, |this| {
                                this.child(Button::new("iamnbutler"))
                            })
                            .child(Button::new("zed"))
                            .child(Button::new("nate/gpui2-ui-components")),
                    )
                    // .children(player_list.map(|p| PlayerStack::new(p)))
                    .child(IconButton::new(Icon::Plus)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(IconButton::new(Icon::FolderX))
                            .child(IconButton::new(Icon::Exit)),
                    )
                    .child(ToolDivider::new())
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                IconButton::<TitleBar>::new(Icon::Mic)
                                    .when(self.is_mic_muted(), |this| this.color(IconColor::Error))
                                    .on_click(|title_bar, cx| title_bar.toggle_mic_status(cx)),
                            )
                            .child(
                                IconButton::<TitleBar>::new(Icon::AudioOn)
                                    .when(self.is_deafened, |this| this.color(IconColor::Error))
                                    .on_click(|title_bar, cx| title_bar.toggle_deafened(cx)),
                            )
                            .child(
                                IconButton::<TitleBar>::new(Icon::Screen)
                                    .when(
                                        self.screen_share_status == ScreenShareStatus::Shared,
                                        |this| this.color(IconColor::Accent),
                                    )
                                    .on_click(|title_bar, cx| {
                                        title_bar.toggle_screen_share_status(cx)
                                    }),
                            ),
                    )
                    .child(
                        div().px_2().flex().items_center().child(
                            Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .shape(Shape::RoundedRectangle),
                        ),
                    ),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    pub struct TitleBarStory {
        title_bar: View<TitleBar>,
    }

    impl TitleBarStory {
        pub fn view(cx: &mut WindowContext) -> View<Self> {
            view(
                cx.entity(|cx| Self {
                    title_bar: TitleBar::view(cx),
                }),
                Self::render,
            )
        }

        fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
            Story::container(cx)
                .child(Story::title_for::<_, TitleBar>(cx))
                .child(Story::label(cx, "Default"))
                .child(self.title_bar.clone())
        }
    }
}
