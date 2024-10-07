use gpui::AnyView;

use crate::prelude::*;

/// The audio status of an player, for use in representing
/// their status visually on their avatar.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum AudioStatus {
    /// The player's microphone is muted.
    Muted,
    /// The player's microphone is muted, and collaboration audio is disabled.
    Deafened,
}

/// An indicator that shows the audio status of a player.
#[derive(IntoElement)]
pub struct AvatarAudioStatusIndicator {
    audio_status: AudioStatus,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
}

impl AvatarAudioStatusIndicator {
    /// Creates a new `AvatarAudioStatusIndicator`
    pub fn new(audio_status: AudioStatus) -> Self {
        Self {
            audio_status,
            tooltip: None,
        }
    }

    /// Sets the tooltip for the indicator.
    pub fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl RenderOnce for AvatarAudioStatusIndicator {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let icon_size = IconSize::Indicator;

        let width_in_px = icon_size.rems() * cx.rem_size();
        let padding_x = px(4.);

        div()
            .absolute()
            .bottom(rems_from_px(-3.))
            .right(rems_from_px(-6.))
            .w(width_in_px + padding_x)
            .h(icon_size.rems())
            .child(
                h_flex()
                    .id("muted-indicator")
                    .justify_center()
                    .px(padding_x)
                    .py(px(2.))
                    .bg(cx.theme().status().error_background)
                    .rounded_md()
                    .child(
                        Icon::new(match self.audio_status {
                            AudioStatus::Muted => IconName::MicMute,
                            AudioStatus::Deafened => IconName::AudioOff,
                        })
                        .size(icon_size)
                        .color(Color::Error),
                    )
                    .when_some(self.tooltip, |this, tooltip| {
                        this.tooltip(move |cx| tooltip(cx))
                    }),
            )
    }
}
