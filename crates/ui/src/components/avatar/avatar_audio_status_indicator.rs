use gpui::AnyView;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum AudioStatus {
    Muted,
    Deafened,
}

#[derive(IntoElement)]
pub struct AvatarAudioStatusIndicator {
    audio_status: AudioStatus,
    tooltip: Option<Box<dyn Fn(&mut WindowContext) -> AnyView>>,
}

impl AvatarAudioStatusIndicator {
    pub fn new(audio_status: AudioStatus) -> Self {
        Self {
            audio_status,
            tooltip: None,
        }
    }

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
            .bottom(rems(-3. / 16.))
            .right(rems(-6. / 16.))
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
