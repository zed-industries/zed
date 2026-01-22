use gpui::{ClickEvent, ImageSource, IntoElement, SharedString};

use crate::{Avatar, prelude::*};

#[derive(Debug, Clone)]
pub struct ScreenShareState {
    /// The name of the application or window being shared.
    pub window_name: Option<SharedString>,
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
    screen_share: Option<ScreenShareState>,
    is_following: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
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
            screen_share: None,
            is_following: false,
            on_click: None,
        }
    }

    pub fn avatar(mut self, src: impl Into<ImageSource>) -> Self {
        self.avatar_src = Some(src.into());
        self
    }

    pub fn screen_share(mut self, state: ScreenShareState) -> Self {
        self.screen_share = Some(state);
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

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ParticipantItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (mic_icon, mic_icon_color) = if self.is_muted {
            (IconName::MicMute, Color::Muted)
        } else {
            (IconName::Mic, Color::Default)
        };

        v_flex()
            .bg(cx.theme().colors().surface_background)
            .child(
                h_flex()
                    .id(SharedString::from(format!("player-{}", self.display_name)))
                    .cursor_pointer()
                    .py_1()
                    .px_2()
                    .w_full()
                    .justify_between()
                    .hover(|s| s.bg(cx.theme().colors().element_hover))
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Avatar::new(
                                    self.avatar_src
                                        .clone()
                                        .unwrap_or_else(|| "https://avatars.githubusercontent.com/u/1?v=4".into()),
                                )
                                .when(self.is_following, |this| {
                                    this.border_color(cx.theme().players().agent().cursor)
                                }),
                            )
                            .child(Label::new(self.display_name).size(LabelSize::Small)),
                    )
                    .child(
                        Icon::new(mic_icon)
                            .size(IconSize::Small)
                            .color(mic_icon_color),
                    ),
            )
            .child(
                h_flex()
                    // .id(SharedString::from(format!("player-{}", self.display_name)))
                    .cursor_pointer()
                    .px_2()
                    .w_full()
                    .gap_2()
                    .hover(|s| s.bg(cx.theme().colors().element_hover))
                    .child(
                        h_flex()
                            .h_5()
                            .w_4()
                            .flex_shrink_0()
                            .justify_center()
                            .child(div().w_px().h_full().bg(cx.theme().colors().border)),
                    )
                    .child(
                        Icon::new(IconName::Folder)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new("zed").color(Color::Muted)),
            )
    }
}

impl Component for ParticipantItem {
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let ex_container = h_flex()
            .w_80()
            .border_1()
            .border_color(cx.theme().colors().border);

        let examples = vec![single_example(
            "Default",
            ex_container
                .child(ParticipantItem::new("Matt"))
                .into_any_element(),
        )];

        Some(example_group(examples).vertical().into_any_element())
    }
}
