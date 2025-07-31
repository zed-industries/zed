use gpui::{ClickEvent, ElementId, IntoElement, ParentElement, Styled};
use ui::prelude::*;

#[derive(IntoElement)]
pub struct NewThreadButton {
    id: ElementId,
    label: SharedString,
    icon: IconName,
    keybinding: Option<ui::KeyBinding>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl NewThreadButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon,
            keybinding: None,
            on_click: None,
        }
    }

    pub fn keybinding(mut self, keybinding: Option<ui::KeyBinding>) -> Self {
        self.keybinding = keybinding;
        self
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Box::new(
            move |_: &ClickEvent, window: &mut Window, cx: &mut App| handler(window, cx),
        ));
        self
    }
}

impl RenderOnce for NewThreadButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .w_full()
            .py_1p5()
            .px_2()
            .gap_1()
            .justify_between()
            .rounded_md()
            .border_1()
            .border_color(cx.theme().colors().border.opacity(0.4))
            .bg(cx.theme().colors().element_active.opacity(0.2))
            .hover(|style| {
                style
                    .bg(cx.theme().colors().element_hover)
                    .border_color(cx.theme().colors().border)
            })
            .child(
                h_flex()
                    .gap_1p5()
                    .child(
                        Icon::new(self.icon)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(Label::new(self.label).size(LabelSize::Small)),
            )
            .when_some(self.keybinding, |this, keybinding| {
                this.child(keybinding.size(rems_from_px(10.)))
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |event, window, cx| on_click(event, window, cx))
            })
    }
}
