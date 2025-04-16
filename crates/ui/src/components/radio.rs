use std::sync::Arc;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct RadioWithLabel {
    id: ElementId,
    label: Label,
    selected: bool,
    on_click: Arc<dyn Fn(&bool, &mut Window, &mut App) + 'static>,
}

impl RadioWithLabel {
    pub fn new(
        id: impl Into<ElementId>,
        label: Label,
        selected: bool,
        on_click: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label,
            selected,
            on_click: Arc::new(on_click),
        }
    }
}

impl RenderOnce for RadioWithLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let inner_diameter = rems_from_px(6.);
        let outer_diameter = rems_from_px(16.);
        let border_width = rems_from_px(1.);
        h_flex()
            .id(self.id)
            .gap(DynamicSpacing::Base08.rems(cx))
            .group("")
            .child(
                div()
                    .size(outer_diameter)
                    .rounded(outer_diameter / 2.)
                    .border_color(cx.theme().colors().border)
                    .border(border_width)
                    .group_hover("", |el| el.bg(cx.theme().colors().element_hover))
                    .when(self.selected, |el| {
                        el.child(
                            div()
                                .m((outer_diameter - inner_diameter) / 2. - border_width)
                                .size(inner_diameter)
                                .rounded(inner_diameter / 2.)
                                .bg(cx.theme().colors().icon_accent),
                        )
                    }),
            )
            .child(self.label)
            .on_click(move |_event, window, cx| {
                (self.on_click)(&true, window, cx);
            })
    }
}
