use gpui::{IntoElement, Window, prelude::*};

use crate::{ButtonLike, prelude::*};

/// A button that takes an underline to look like a regular web link.
/// It also contains an arrow icon to communicate the link takes you out of Zed.
///
/// # Usage Example
///
/// ```
/// use ui::ButtonLink;
///
/// let button_link = ButtonLink::new("Click me", "https://example.com");
/// ```
#[derive(IntoElement, RegisterComponent)]
pub struct ButtonLink {
    label: SharedString,
    label_size: LabelSize,
    label_color: Color,
    link: String,
    no_icon: bool,
}

impl ButtonLink {
    pub fn new(label: impl Into<SharedString>, link: impl Into<String>) -> Self {
        Self {
            link: link.into(),
            label: label.into(),
            label_size: LabelSize::Default,
            label_color: Color::Default,
            no_icon: false,
        }
    }

    pub fn no_icon(mut self, no_icon: bool) -> Self {
        self.no_icon = no_icon;
        self
    }

    pub fn label_size(mut self, label_size: LabelSize) -> Self {
        self.label_size = label_size;
        self
    }

    pub fn label_color(mut self, label_color: Color) -> Self {
        self.label_color = label_color;
        self
    }
}

impl RenderOnce for ButtonLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let id = format!("{}-{}", self.label, self.link);

        ButtonLike::new(id)
            .size(ButtonSize::None)
            .child(
                h_flex()
                    .gap_0p5()
                    .child(
                        Label::new(self.label)
                            .size(self.label_size)
                            .color(self.label_color)
                            .underline(),
                    )
                    .when(!self.no_icon, |this| {
                        this.child(
                            Icon::new(IconName::ArrowUpRight)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .on_click(move |_, _, cx| cx.open_url(&self.link))
            .into_any_element()
    }
}

impl Component for ButtonLink {
    fn scope() -> ComponentScope {
        ComponentScope::Navigation
    }

    fn description() -> Option<&'static str> {
        Some("A button that opens a URL.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .child(
                    example_group(vec![single_example(
                        "Simple",
                        ButtonLink::new("zed.dev", "https://zed.dev").into_any_element(),
                    )])
                    .vertical(),
                )
                .into_any_element(),
        )
    }
}
