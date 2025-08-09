use gpui::{AnyElement, IntoElement, ParentElement, SharedString};
use ui::{ListItem, prelude::*};

/// A reusable list item component for adding LLM provider configuration instructions
pub struct InstructionListItem {
    label: SharedString,
    button_label: Option<SharedString>,
    button_link: Option<String>,
}

impl InstructionListItem {
    pub fn new(
        label: impl Into<SharedString>,
        button_label: Option<impl Into<SharedString>>,
        button_link: Option<impl Into<String>>,
    ) -> Self {
        Self {
            label: label.into(),
            button_label: button_label.map(|l| l.into()),
            button_link: button_link.map(|l| l.into()),
        }
    }

    pub fn text_only(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            button_label: None,
            button_link: None,
        }
    }
}

impl IntoElement for InstructionListItem {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let item_content = if let (Some(button_label), Some(button_link)) =
            (self.button_label, self.button_link)
        {
            let link = button_link.clone();
            let unique_id = SharedString::from(format!("{}-button", self.label));

            h_flex()
                .flex_wrap()
                .child(Label::new(self.label))
                .child(
                    Button::new(unique_id, button_label)
                        .style(ButtonStyle::Subtle)
                        .icon(IconName::ArrowUpRight)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .on_click(move |_, _window, cx| cx.open_url(&link)),
                )
                .into_any_element()
        } else {
            Label::new(self.label).into_any_element()
        };

        ListItem::new("list-item")
            .selectable(false)
            .start_slot(
                Icon::new(IconName::Dash)
                    .size(IconSize::XSmall)
                    .color(Color::Hidden),
            )
            .child(div().w_full().child(item_content))
            .into_any_element()
    }
}
