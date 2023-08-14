use crate::{element::Element, style::Style};

pub struct Frame {
    style: Style,
    children: Vec<Frame>,
}

impl<V: 'static> Element<V> for Frame {
    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn layout(&mut self, view: &mut V, cx: &mut gpui::LayoutContext<V>) -> taffy::tree::NodeId {
        let child_layout_node_ids = self
            .children
            .iter_mut()
            .map(|child| child.layout(view, cx))
            .collect::<Vec<_>>();

        let rem_size = cx.rem_pixels();
        cx.layout_engine()
            .new_with_children(self.style.to_taffy(rem_size), &child_layout_node_ids)
            .unwrap()
    }

    fn paint(
        &mut self,
        layout: &taffy::tree::Layout,
        view: &mut V,
        cx: &mut gpui::PaintContext<V>,
    ) {
        todo!()
    }
}
