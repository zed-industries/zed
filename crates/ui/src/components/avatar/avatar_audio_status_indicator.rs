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
        div()
            .absolute()
            .bottom(px(-1.))
            .right(px(-4.))
            .w(rems(12. / 16.))
            .h(rems(10. / 16.))
            .child(
                h_flex()
                    .id("muted-indicator")
                    .justify_center()
                    .px(px(1.))
                    .py(px(2.))
                    .bg(cx.theme().status().error_background)
                    .rounded_md()
                    .child(
                        Icon::new(match self.audio_status {
                            AudioStatus::Muted => IconName::MicMute,
                            AudioStatus::Deafened => IconName::AudioOff,
                        })
                        .size(IconSize::Indicator)
                        .color(Color::Error),
                    )
                    .when_some(self.tooltip, |this, tooltip| {
                        this.tooltip(move |cx| tooltip(cx))
                    }),
            )
    }
}
