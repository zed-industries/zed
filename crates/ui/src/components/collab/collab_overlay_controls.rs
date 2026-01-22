use crate::{Avatar, Tooltip, prelude::*};
use gpui::{AnyElement, ClickEvent, ImageSource, IntoElement};

#[derive(IntoElement, RegisterComponent)]
pub struct CollabOverlayControls {
    avatar: ImageSource,
    is_open: bool,
    is_muted: bool,
    is_deafened: bool,
    is_screen_sharing: bool,
    on_toggle_mute: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_toggle_deafen: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    screen_share_menu: Option<AnyElement>,
    on_leave: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl CollabOverlayControls {
    pub fn new(avatar: impl Into<ImageSource>) -> Self {
        Self {
            avatar: avatar.into(),
            is_open: false,
            is_muted: false,
            is_deafened: false,
            is_screen_sharing: false,
            on_toggle_mute: None,
            on_toggle_deafen: None,
            screen_share_menu: None,
            on_leave: None,
        }
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn is_muted(mut self, is_muted: bool) -> Self {
        self.is_muted = is_muted;
        self
    }

    pub fn is_deafened(mut self, is_deafened: bool) -> Self {
        self.is_deafened = is_deafened;
        self
    }

    pub fn is_screen_sharing(mut self, is_screen_sharing: bool) -> Self {
        self.is_screen_sharing = is_screen_sharing;
        self
    }

    pub fn on_toggle_mute(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle_mute = Some(Box::new(handler));
        self
    }

    pub fn on_toggle_deafen(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle_deafen = Some(Box::new(handler));
        self
    }

    pub fn screen_share_menu(mut self, menu: impl IntoElement) -> Self {
        self.screen_share_menu = Some(menu.into_any_element());
        self
    }

    pub fn on_leave(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_leave = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for CollabOverlayControls {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mic_icon = if self.is_muted {
            IconName::MicMute
        } else {
            IconName::Mic
        };

        let audio_icon = if self.is_deafened {
            IconName::AudioOff
        } else {
            IconName::AudioOn
        };

        h_flex()
            .py_1()
            .px_2()
            .w_full()
            .gap_1()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().surface_background)
            .child(
                h_flex().gap_1().child(Avatar::new(self.avatar)).child(
                    h_flex()
                        .child(
                            IconButton::new("mic", mic_icon)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text(if self.is_muted {
                                    "Unmute"
                                } else {
                                    "Mute"
                                }))
                                .when(self.is_muted, |this| this.icon_color(Color::Error))
                                .when_some(self.on_toggle_mute, |this, handler| {
                                    this.on_click(handler)
                                }),
                        )
                        .child(
                            IconButton::new("audio", audio_icon)
                                .icon_size(IconSize::Small)
                                .tooltip(Tooltip::text(if self.is_deafened {
                                    "Undeafen"
                                } else {
                                    "Deafen"
                                }))
                                .when(self.is_deafened, |this| this.icon_color(Color::Error))
                                .when_some(self.on_toggle_deafen, |this, handler| {
                                    this.on_click(handler)
                                }),
                        )
                        .when_some(self.screen_share_menu, |this, menu| this.child(menu)),
                ),
            )
            .child(
                Button::new("leave", "Leave")
                    .label_size(LabelSize::Small)
                    .icon(IconName::Exit)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Error)
                    .tooltip(Tooltip::text("Leave Call"))
                    .when_some(self.on_leave, |this, handler| this.on_click(handler)),
            )
    }
}

impl Component for CollabOverlayControls {
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
                .child(
                    CollabOverlayControls::new(
                        "https://avatars.githubusercontent.com/u/67129314?v=4",
                    )
                    .is_open(true),
                )
                .into_any_element(),
        )];

        Some(example_group(examples).vertical().into_any_element())
    }
}
