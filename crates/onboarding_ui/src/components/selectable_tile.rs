use component::{example_group_with_title, single_example};
use gpui::{ClickEvent, transparent_black};
use smallvec::SmallVec;
use ui::{Vector, VectorName, prelude::*, utils::CornerSolver};

#[derive(IntoElement, RegisterComponent)]
pub struct SelectableTile {
    id: ElementId,
    width: DefiniteLength,
    height: DefiniteLength,
    parent_focused: bool,
    selected: bool,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl SelectableTile {
    pub fn new(
        id: impl Into<ElementId>,
        width: impl Into<DefiniteLength>,
        height: impl Into<DefiniteLength>,
    ) -> Self {
        Self {
            id: id.into(),
            width: width.into(),
            height: height.into(),
            parent_focused: false,
            selected: false,
            children: SmallVec::new(),
            on_click: None,
        }
    }

    pub fn w(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.width = width.into();
        self
    }

    pub fn h(mut self, height: impl Into<DefiniteLength>) -> Self {
        self.height = height.into();
        self
    }

    pub fn parent_focused(mut self, focused: bool) -> Self {
        self.parent_focused = focused;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for SelectableTile {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let ring_corner_radius = px(8.);
        let ring_width = px(1.);
        let padding = px(2.);
        let content_border_width = px(0.);
        let content_border_radius = CornerSolver::child_radius(
            ring_corner_radius,
            ring_width,
            padding,
            content_border_width,
        );

        let mut element = h_flex()
            .id(self.id)
            .w(self.width)
            .h(self.height)
            .overflow_hidden()
            .rounded(ring_corner_radius)
            .border(ring_width)
            .border_color(if self.selected && self.parent_focused {
                cx.theme().status().info
            } else if self.selected {
                cx.theme().colors().border
            } else {
                transparent_black()
            })
            .p(padding)
            .child(
                h_flex()
                    .size_full()
                    .rounded(content_border_radius)
                    .items_center()
                    .justify_center()
                    .shadow_hairline()
                    .bg(cx.theme().colors().surface_background)
                    .children(self.children),
            );

        if let Some(on_click) = self.on_click {
            element = element.on_click(move |event, window, cx| {
                on_click(event, window, cx);
            });
        }

        element
    }
}

impl ParentElement for SelectableTile {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Component for SelectableTile {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let states = example_group(vec![
            single_example(
                "Default",
                SelectableTile::new("default", px(40.), px(40.))
                    .parent_focused(false)
                    .selected(false)
                    .child(div().p_4().child(Vector::new(
                        VectorName::ZedLogo,
                        rems(18. / 16.),
                        rems(18. / 16.),
                    )))
                    .into_any_element(),
            ),
            single_example(
                "Selected",
                SelectableTile::new("selected", px(40.), px(40.))
                    .parent_focused(false)
                    .selected(true)
                    .child(div().p_4().child(Vector::new(
                        VectorName::ZedLogo,
                        rems(18. / 16.),
                        rems(18. / 16.),
                    )))
                    .into_any_element(),
            ),
            single_example(
                "Selected & Parent Focused",
                SelectableTile::new("selected_focused", px(40.), px(40.))
                    .parent_focused(true)
                    .selected(true)
                    .child(div().p_4().child(Vector::new(
                        VectorName::ZedLogo,
                        rems(18. / 16.),
                        rems(18. / 16.),
                    )))
                    .into_any_element(),
            ),
        ]);

        Some(v_flex().p_4().gap_4().child(states).into_any_element())
    }
}
