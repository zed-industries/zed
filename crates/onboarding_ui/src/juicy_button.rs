use gpui::{FontWeight, *};
use ui::prelude::*;

#[derive(IntoElement)]
pub struct JuicyButton {
    base: Div,
    label: SharedString,
    keybinding: Option<AnyElement>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl JuicyButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            label: label.into(),
            keybinding: None,
            on_click: None,
        }
    }

    pub fn keybinding(mut self, keybinding: Option<impl IntoElement>) -> Self {
        if let Some(kb) = keybinding {
            self.keybinding = Some(kb.into_any_element());
        }
        self
    }
}

impl Clickable for JuicyButton {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor(style);
        self
    }
}

impl RenderOnce for JuicyButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut children = vec![
            h_flex()
                .flex_1()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_size(px(14.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(cx.theme().colors().text)
                        .child(self.label),
                )
                .into_any_element(),
        ];

        if let Some(keybinding) = self.keybinding {
            children.push(
                div()
                    .flex_none()
                    .bg(gpui::white().opacity(0.2))
                    .rounded_md()
                    .px_1()
                    .child(keybinding)
                    .into_any_element(),
            );
        }

        self.base
            .id("juicy-button")
            .w_full()
            .h(rems(2.))
            .px(rems(1.5))
            .rounded(px(6.))
            .bg(cx.theme().colors().icon.opacity(0.12))
            .shadow_hairline()
            .hover(|style| {
                style.bg(cx.theme().colors().icon.opacity(0.12)) // Darker blue on hover
            })
            .active(|style| {
                style
                    .bg(rgb(0x1e40af)) // Even darker on active
                    .shadow_md()
            })
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_between()
            .when_some(self.on_click, |div, on_click| div.on_click(on_click))
            .children(children)
    }
}
