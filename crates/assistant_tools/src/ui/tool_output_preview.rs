use gpui::{AnyView, prelude::*};
use ui::{Tooltip, prelude::*};

#[derive(IntoElement)]
pub struct ToolOutputPreview<F>
where
    F: Fn(bool, &mut Window, &mut App) + 'static,
{
    content: AnyView,
    full_height: bool,
    total_lines: usize,
    on_toggle: Option<F>,
}

pub const COLLAPSED_LINES: usize = 10;

impl<F> ToolOutputPreview<F>
where
    F: Fn(bool, &mut Window, &mut App) + 'static,
{
    pub fn new(content: AnyView) -> Self {
        Self {
            content,
            full_height: true,
            total_lines: 0,
            on_toggle: None,
        }
    }

    pub fn with_total_lines(mut self, total_lines: usize) -> Self {
        self.total_lines = total_lines;
        self
    }

    pub fn toggle_state(mut self, full_height: bool) -> Self {
        self.full_height = full_height;
        self
    }

    pub fn on_toggle(mut self, listener: F) -> Self {
        self.on_toggle = Some(listener);
        self
    }
}

impl<F> RenderOnce for ToolOutputPreview<F>
where
    F: Fn(bool, &mut Window, &mut App) + 'static,
{
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.total_lines <= COLLAPSED_LINES {
            return self.content.into_any_element();
        }
        let entity_id = self.content.entity_id();
        let border_color = cx.theme().colors().border.opacity(0.6);

        let (icon, tooltip_label) = if self.full_height {
            (IconName::ChevronUp, "Collapse")
        } else {
            (IconName::ChevronDown, "Expand")
        };

        v_flex()
            .child(self.content)
            .child(
                h_flex()
                    .id(("expand-button", entity_id))
                    .flex_none()
                    .cursor_pointer()
                    .h_5()
                    .justify_center()
                    .border_t_1()
                    .rounded_b_md()
                    .border_color(border_color)
                    .bg(cx.theme().colors().editor_background)
                    .hover(|style| style.bg(cx.theme().colors().element_hover.opacity(0.1)))
                    .child(Icon::new(icon).size(IconSize::Small).color(Color::Muted))
                    .tooltip(Tooltip::text(tooltip_label))
                    .when_some(self.on_toggle, |this, on_toggle| {
                        this.on_click({
                            move |_, window, cx| {
                                on_toggle(!self.full_height, window, cx);
                            }
                        })
                    }),
            )
            .into_any()
    }
}
