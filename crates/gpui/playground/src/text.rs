use std::borrow::Cow;

use crate::element::Element;

impl<V, S> Element<V> for S
where
    V: 'static,
    S: 'static + Into<Cow<'static, str>>,
{
    type Layout = Cow<'static, str>;

    fn style_mut(&mut self) -> &mut crate::style::ElementStyle {
        todo!()
    }

    fn handlers_mut(&mut self) -> &mut crate::element::ElementHandlers<V> {
        todo!()
    }

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut crate::element::LayoutContext<V>,
    ) -> anyhow::Result<(taffy::tree::NodeId, Self::Layout)> {
        todo!()
    }

    fn paint<'a>(
        &mut self,
        layout: crate::element::Layout<Self::Layout>,
        view: &mut V,
        cx: &mut crate::element::PaintContext<V>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
