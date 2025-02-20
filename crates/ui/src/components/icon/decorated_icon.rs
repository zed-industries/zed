use gpui::{AnyElement, IntoElement, Point};

use crate::{prelude::*, IconDecoration, IconDecorationKind};

#[derive(IntoElement, IntoComponent)]
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .size(self.icon.size)
            .child(self.icon)
            .children(self.decoration)
    }
}

// View this component preview using `workspace: open component-preview`
impl ComponentPreview for DecoratedIcon {
    fn preview(_window: &mut Window, cx: &App) -> AnyElement {
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

        v_flex()
            .gap_6()
            .children(vec![example_group_with_title(
                "Decorations",
                vec![
                    single_example(
                        "No Decoration",
                        DecoratedIcon::new(Icon::new(IconName::FileDoc), None).into_any_element(),
                    ),
                    single_example(
                        "X Decoration",
                        DecoratedIcon::new(Icon::new(IconName::FileDoc), Some(decoration_x))
                            .into_any_element(),
                    ),
                    single_example(
                        "Triangle Decoration",
                        DecoratedIcon::new(Icon::new(IconName::FileDoc), Some(decoration_triangle))
                            .into_any_element(),
                    ),
                    single_example(
                        "Dot Decoration",
                        DecoratedIcon::new(Icon::new(IconName::FileDoc), Some(decoration_dot))
                            .into_any_element(),
                    ),
                ],
            )])
            .into_any_element()
    }
}
