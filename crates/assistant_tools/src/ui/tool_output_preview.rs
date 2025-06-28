use gpui::{AnyElement, EntityId, prelude::*};
use ui::{Tooltip, prelude::*};

#[derive(IntoElement)]
pub struct ToolOutputPreview<F>
where
    F: Fn(bool, &mut Window, &mut App) + 'static,
{
    content: AnyElement,
    entity_id: EntityId,
    full_height: bool,
    total_lines: usize,
    collapsed_fade: bool,
    on_toggle: Option<F>,
}

pub const COLLAPSED_LINES: usize = 10;

impl<F> ToolOutputPreview<F>
where
    F: Fn(bool, &mut Window, &mut App) + 'static,
{
    pub fn new(content: AnyElement, entity_id: EntityId) -> Self {
        Self {
            content,
            entity_id,
            full_height: true,
            total_lines: 0,
            collapsed_fade: false,
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

    pub fn with_collapsed_fade(mut self) -> Self {
        self.collapsed_fade = true;
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
            return self.content;
        }
        let border_color = cx.theme().colors().border.opacity(0.6);

        let (icon, tooltip_label) = if self.full_height {
            (IconName::ChevronUp, "Collapse")
        } else {
            (IconName::ChevronDown, "Expand")
        };

        let gradient_overlay =
            if self.collapsed_fade && !self.full_height {
                Some(div().absolute().bottom_5().left_0().w_full().h_2_5().bg(
                    gpui::linear_gradient(
                        0.,
                        gpui::linear_color_stop(cx.theme().colors().editor_background, 0.),
                        gpui::linear_color_stop(
                            cx.theme().colors().editor_background.opacity(0.),
                            1.,
                        ),
                    ),
                ))
            } else {
                None
            };

        v_flex()
            .relative()
            .child(self.content)
            .children(gradient_overlay)
            .child(
                h_flex()
                    .id(("expand-button", self.entity_id))
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
