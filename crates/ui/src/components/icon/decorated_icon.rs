use gpui::{IntoElement, Point};

use crate::{
    prelude::*, traits::component_preview::ComponentPreview, IconDecoration, IconDecorationKind,
};

#[derive(IntoElement)]
pub struct DecoratedIcon {
    icon: Icon,
    decoration: Option<IconDecoration>,
}

impl DecoratedIcon {
    pub fn new(icon: Icon, decoration: Option<IconDecoration>) -> Self {
        Self { icon, decoration }
    }
}

impl RenderOnce for DecoratedIcon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .relative()
            .size(self.icon.size)
            .child(self.icon)
            .children(self.decoration)
    }
}

impl ComponentPreview for DecoratedIcon {
    fn examples(cx: &mut WindowContext) -> Vec<ComponentExampleGroup<Self>> {
        let icon_1 = Icon::new(IconName::FileDoc);
        let icon_2 = Icon::new(IconName::FileDoc);
        let icon_3 = Icon::new(IconName::FileDoc);
        let icon_4 = Icon::new(IconName::FileDoc);

        let decoration_x = IconDecoration::new(
            IconDecorationKind::X,
            cx.theme().colors().surface_background,
            cx,
        )
        .color(cx.theme().status().error)
        .position(Point {
            x: px(-2.),
            y: px(-2.),
        });

        let decoration_triangle = IconDecoration::new(
            IconDecorationKind::Triangle,
            cx.theme().colors().surface_background,
            cx,
        )
        .color(cx.theme().status().error)
        .position(Point {
            x: px(-2.),
            y: px(-2.),
        });

        let decoration_dot = IconDecoration::new(
            IconDecorationKind::Dot,
            cx.theme().colors().surface_background,
            cx,
        )
        .color(cx.theme().status().error)
        .position(Point {
            x: px(-2.),
            y: px(-2.),
        });

        let examples = vec![
            single_example("no_decoration", DecoratedIcon::new(icon_1, None)),
            single_example(
                "with_decoration",
                DecoratedIcon::new(icon_2, Some(decoration_x)),
            ),
            single_example(
                "with_decoration",
                DecoratedIcon::new(icon_3, Some(decoration_triangle)),
            ),
            single_example(
                "with_decoration",
                DecoratedIcon::new(icon_4, Some(decoration_dot)),
            ),
        ];

        vec![example_group(examples)]
    }
}
