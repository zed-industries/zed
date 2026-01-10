use gpui::{
    AnyElement, App, ClipboardItem, IntoElement, ParentElement, RenderOnce, Styled, Window,
};

use crate::{Tooltip, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct CopyButton {
    message: SharedString,
    icon_size: IconSize,
    disabled: bool,
    tooltip_label: SharedString,
    visible_on_hover: Option<SharedString>,
    custom_on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl CopyButton {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            icon_size: IconSize::Small,
            disabled: false,
            tooltip_label: "Copy".into(),
            visible_on_hover: None,
            custom_on_click: None,
        }
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn tooltip_label(mut self, tooltip_label: impl Into<SharedString>) -> Self {
        self.tooltip_label = tooltip_label.into();
        self
    }

    pub fn visible_on_hover(mut self, visible_on_hover: impl Into<SharedString>) -> Self {
        self.visible_on_hover = Some(visible_on_hover.into());
        self
    }

    pub fn custom_on_click(
        mut self,
        custom_on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.custom_on_click = Some(Box::new(custom_on_click));
        self
    }
}

impl RenderOnce for CopyButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let message = self.message;
        let message_clone = message.clone();

        let id = format!("copy-button-{}", message_clone);

        let copied = cx
            .read_from_clipboard()
            .map(|item| item.text().as_ref() == Some(&message_clone.into()))
            .unwrap_or(false);

        let (icon, color, tooltip) = if copied {
            (IconName::Check, Color::Success, "Copied!".into())
        } else {
            (IconName::Copy, Color::Muted, self.tooltip_label)
        };

        let custom_on_click = self.custom_on_click;
        let visible_on_hover = self.visible_on_hover;

        let button = IconButton::new(id, icon)
            .icon_color(color)
            .icon_size(self.icon_size)
            .disabled(self.disabled)
            .tooltip(Tooltip::text(tooltip))
            .on_click(move |_, window, cx| {
                if let Some(custom_on_click) = custom_on_click.as_ref() {
                    (custom_on_click)(window, cx);
                } else {
                    cx.stop_propagation();
                    cx.write_to_clipboard(ClipboardItem::new_string(message.clone().into()));
                }
            });

        if let Some(visible_on_hover) = visible_on_hover {
            button.visible_on_hover(visible_on_hover)
        } else {
            button
        }
    }
}

impl Component for CopyButton {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> Option<&'static str> {
        Some("An icon button that encapsulates the logic to copy a string into the clipboard.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let label_text = "Here's an example label";
        let mut counter: usize = 0;

        let mut copy_b = || {
            counter += 1;
            CopyButton::new(format!(
                "Here's an example label (id for uniqueness: {} — ignore this)",
                counter
            ))
        };

        let example = vec![
            single_example(
                "Default",
                h_flex()
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(copy_b())
                    .into_any_element(),
            ),
            single_example(
                "Multiple Icon Sizes",
                h_flex()
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(copy_b().icon_size(IconSize::XSmall))
                    .child(copy_b().icon_size(IconSize::Medium))
                    .child(copy_b().icon_size(IconSize::XLarge))
                    .into_any_element(),
            ),
            single_example(
                "Custom Tooltip Label",
                h_flex()
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(copy_b().tooltip_label("Custom tooltip label"))
                    .into_any_element(),
            ),
            single_example(
                "Visible On Hover",
                h_flex()
                    .group("container")
                    .gap_1()
                    .child(Label::new(label_text).size(LabelSize::Small))
                    .child(copy_b().visible_on_hover("container"))
                    .into_any_element(),
            ),
        ];

        Some(example_group(example).vertical().into_any_element())
    }
}
