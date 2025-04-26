use gpui::{AnyElement, IntoElement, Point};

use crate::{IconDecoration, IconDecorationKind, prelude::*};

#[derive(IntoElement, RegisterComponent)]
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

impl Component for DecoratedIcon {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Images
    }

    fn description() -> Option<&'static str> {
        Some(
            "An icon with an optional decoration overlay (like an X, triangle, or dot) that can be positioned relative to the icon",
        )
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, cx: &mut App) -> Option<AnyElement> {
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

        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group_with_title(
                    "Decorations",
                    vec![
                        single_example(
                            "No Decoration",
                            DecoratedIcon::new(Icon::new(IconName::FileDoc), None)
                                .into_any_element(),
                        ),
                        single_example(
                            "X Decoration",
                            DecoratedIcon::new(Icon::new(IconName::FileDoc), Some(decoration_x))
                                .into_any_element(),
                        ),
                        single_example(
                            "Triangle Decoration",
                            DecoratedIcon::new(
                                Icon::new(IconName::FileDoc),
                                Some(decoration_triangle),
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Dot Decoration",
                            DecoratedIcon::new(Icon::new(IconName::FileDoc), Some(decoration_dot))
                                .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
