use crate::style::{Display, Length, Overflow, Position, Style};
use gpui::{LayoutContext, PaintContext};
use playground_macros::tailwind_lengths;
pub use taffy::tree::{Layout, NodeId};

pub trait Element<V> {
    fn style_mut(&mut self) -> &mut Style;
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> NodeId;
    fn paint(&mut self, layout: &Layout, view: &mut V, cx: &mut gpui::PaintContext<V>);

    // Display ////////////////////

    fn block(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().display = Display::Block;
        self
    }

    fn flex(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().display = Display::Flex;
        self
    }

    fn grid(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().display = Display::Grid;
        self
    }

    // style::Overflow ///////////////////

    fn overflow_visible(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.x = Overflow::Visible;
        self.style_mut().overflow.y = Overflow::Visible;
        self
    }

    fn overflow_hidden(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.x = Overflow::Hidden;
        self.style_mut().overflow.y = Overflow::Hidden;
        self
    }

    fn overflow_scroll(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.x = Overflow::Scroll;
        self.style_mut().overflow.y = Overflow::Scroll;
        self
    }

    fn overflow_x_visible(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.x = Overflow::Visible;
        self
    }

    fn overflow_x_hidden(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.x = Overflow::Hidden;
        self
    }

    fn overflow_x_scroll(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.x = Overflow::Scroll;
        self
    }

    fn overflow_y_visible(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.y = Overflow::Visible;
        self
    }

    fn overflow_y_hidden(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.y = Overflow::Hidden;
        self
    }

    fn overflow_y_scroll(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().overflow.y = Overflow::Scroll;
        self
    }

    // Position ///////////////////

    fn relative(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().position = Position::Relative;
        self
    }

    fn absolute(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().position = Position::Absolute;

        self
    }

    #[tailwind_lengths]
    fn inset(mut self, length: Length) -> Self
    where
        Self: Sized,
    {
        self.style_mut().inset.top = length;
        self.style_mut().inset.right = length;
        self.style_mut().inset.bottom = length;
        self.style_mut().inset.left = length;
        self
    }

    #[tailwind_lengths]
    fn w(mut self, length: Length) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.width = length;
        self
    }

    #[tailwind_lengths]
    fn min_w(mut self, length: Length) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.width = length;
        self
    }

    #[tailwind_lengths]
    fn h(mut self, length: Length) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.height = length;
        self
    }
}

pub struct AnyElement<V> {
    element: Box<dyn Element<V>>,
    layout_node_id: Option<NodeId>,
}

impl<V> AnyElement<V> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> NodeId {
        let layout_node_id = self.element.layout(view, cx);
        self.layout_node_id = Some(layout_node_id);
        layout_node_id
    }

    fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) {
        let layout_node_id = self.layout_node_id.expect("paint called before layout");
        let layout = cx.layout_engine().layout(layout_node_id).unwrap().clone();
        self.element.paint(&layout, view, cx);
    }
}
