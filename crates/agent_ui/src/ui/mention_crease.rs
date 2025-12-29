use std::time::Duration;

use gpui::{Animation, AnimationExt, AnyView, IntoElement, Window, pulsating_between};
use settings::Settings;
use theme::ThemeSettings;
use ui::{ButtonLike, TintColor, prelude::*};

#[derive(IntoElement)]
pub struct MentionCrease {
    id: ElementId,
    icon: SharedString,
    label: SharedString,
    is_toggled: bool,
    is_loading: bool,
    image_preview: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
}

impl MentionCrease {
    pub fn new(
        id: impl Into<ElementId>,
        icon: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            label: label.into(),
            is_toggled: false,
            is_loading: false,
            image_preview: None,
        }
    }

    pub fn is_toggled(mut self, is_toggled: bool) -> Self {
        self.is_toggled = is_toggled;
        self
    }

    pub fn is_loading(mut self, is_loading: bool) -> Self {
        self.is_loading = is_loading;
        self
    }

    pub fn image_preview(
        mut self,
        builder: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.image_preview = Some(Box::new(builder));
        self
    }
}

impl RenderOnce for MentionCrease {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let font_size = settings.agent_buffer_font_size(cx);
        let buffer_font = settings.buffer_font.clone();

        let button_height = DefiniteLength::Absolute(AbsoluteLength::Pixels(
            px(window.line_height().into()) - px(1.),
        ));

        ButtonLike::new(self.id)
            .style(ButtonStyle::Outlined)
            .size(ButtonSize::Compact)
            .height(button_height)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            .toggle_state(self.is_toggled)
            .when_some(self.image_preview, |this, image_preview| {
                this.hoverable_tooltip(image_preview)
            })
            .child(
                h_flex()
                    .pb_px()
                    .gap_1()
                    .font(buffer_font)
                    .text_size(font_size)
                    .child(
                        Icon::from_path(self.icon.clone())
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(self.label.clone())
                    .map(|this| {
                        if self.is_loading {
                            this.with_animation(
                                "loading-context-crease",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 0.8)),
                                |label, delta| label.opacity(delta),
                            )
                            .into_any()
                        } else {
                            this.into_any()
                        }
                    }),
            )
    }
}
