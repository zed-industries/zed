use gpui::{ClickEvent, IntoElement, ParentElement, SharedString};
use ui::{Tooltip, prelude::*};

#[derive(IntoElement)]
pub struct ConfiguredApiCard {
    label: SharedString,
    button_label: Option<SharedString>,
    tooltip_label: Option<SharedString>,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ConfiguredApiCard {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            button_label: None,
            tooltip_label: None,
            disabled: false,
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn button_label(mut self, button_label: impl Into<SharedString>) -> Self {
        self.button_label = Some(button_label.into());
        self
    }

    pub fn tooltip_label(mut self, tooltip_label: impl Into<SharedString>) -> Self {
        self.tooltip_label = Some(tooltip_label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for ConfiguredApiCard {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let button_label = self.button_label.unwrap_or("Reset Key".into());
        let button_id = SharedString::new(format!("id-{}", button_label));

        h_flex()
            .mt_0p5()
            .p_1()
            .justify_between()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().background)
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(Icon::new(IconName::Check).color(Color::Success))
                    .child(Label::new(self.label).truncate()),
            )
            .child(
                Button::new(button_id, button_label)
                    .label_size(LabelSize::Small)
                    .icon(IconName::Undo)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .icon_position(IconPosition::Start)
                    .disabled(self.disabled)
                    .when_some(self.tooltip_label, |this, label| {
                        this.tooltip(Tooltip::text(label))
                    })
                    .when_some(
                        self.on_click.filter(|_| !self.disabled),
                        |this, on_click| this.on_click(on_click),
                    ),
            )
    }
}
