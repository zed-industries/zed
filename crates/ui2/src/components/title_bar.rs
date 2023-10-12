use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::prelude::*;
use crate::{
    theme, Avatar, Button, Icon, IconButton, IconColor, PlayerStack, PlayerWithCallStatus,
    ToolDivider, TrafficLights,
};

#[derive(Clone)]
pub struct Livestream {
    pub players: Vec<PlayerWithCallStatus>,
    pub channel: Option<String>, // projects
                                 // windows
}

#[derive(Element)]
pub struct TitleBar<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    /// If the window is active from the OS's perspective.
    is_active: Arc<AtomicBool>,
    livestream: Option<Livestream>,
}

impl<S: 'static + Send + Sync + Clone> TitleBar<S> {
    pub fn new(cx: &mut ViewContext<S>) -> Self {
        let is_active = Arc::new(AtomicBool::new(true));
        let active = is_active.clone();

        // cx.observe_window_activation(move |_, is_active, cx| {
        //     active.store(is_active, std::sync::atomic::Ordering::SeqCst);
        //     cx.notify();
        // })
        // .detach();

        Self {
            state_type: PhantomData,
            is_active,
            livestream: None,
        }
    }

    pub fn set_livestream(mut self, livestream: Option<Livestream>) -> Self {
        self.livestream = livestream;
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);
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
            .h_8()
            .fill(theme.lowest.base.default.background)
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
                            .child(Button::new("zed"))
                            .child(Button::new("nate/gpui2-ui-components")),
                    )
                    .children(player_list.map(|p| PlayerStack::new(p)))
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
                            .child(IconButton::new(Icon::Close)),
                    )
                    .child(ToolDivider::new())
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(IconButton::new(Icon::Mic))
                            .child(IconButton::new(Icon::AudioOn))
                            .child(IconButton::new(Icon::Screen).color(IconColor::Accent)),
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

    #[derive(Element)]
    pub struct TitleBarStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> TitleBarStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, TitleBar<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(TitleBar::new(cx))
        }
    }
}
