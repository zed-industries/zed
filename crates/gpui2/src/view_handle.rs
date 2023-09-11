use crate::{style::Style, Element, IntoElement, ViewContext};
use gpui::{
    geometry::{Point, Size},
    taffy::style::Overflow,
    AnyElement, View, ViewHandle,
};

impl<ParentView: 'static, ChildView: View> Element<ParentView> for ViewHandle<ChildView> {
    type PaintState = AnyElement<ChildView>;

    fn layout(
        &mut self,
        _: &mut ParentView,
        cx: &mut ViewContext<ParentView>,
    ) -> anyhow::Result<(gpui::LayoutId, Self::PaintState)>
    where
        Self: Sized,
    {
        let layout_id = cx.add_layout_node(
            Style {
                overflow: Point {
                    x: Overflow::Hidden,
                    y: Overflow::Hidden,
                },
                size: Size::full(),
                ..Default::default()
            },
            None,
        )?;
        let element = self.update(cx, |view, cx| view.render(cx));
        Ok((layout_id, element))
    }

    fn paint(
        &mut self,
        _: &mut ParentView,
        parent_origin: gpui::geometry::vector::Vector2F,
        layout: &gpui::Layout,
        element: &mut AnyElement<ChildView>,
        cx: &mut ViewContext<ParentView>,
    ) where
        Self: Sized,
    {
        self.update(cx, |view, cx| {
            let bounds = layout.bounds + parent_origin;
            element.layout(gpui::SizeConstraint::strict(bounds.size()), view, cx);
            cx.paint_layer(Some(layout.bounds), |cx| {
                element.paint(bounds.origin(), bounds, view, cx);
            });
        })
    }
}

impl<ParentView: 'static, ChildView: View> IntoElement<ParentView> for ViewHandle<ChildView> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
