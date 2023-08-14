use crate::{
    adapter::Adapter,
    style::{DefinedLength, Display, Overflow, Position, Style},
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::{Layout, LayoutContext as LegacyLayoutContext, PaintContext as LegacyPaintContext};
use playground_macros::tailwind_lengths;
pub use taffy::tree::NodeId;

#[derive(Deref, DerefMut)]
pub struct LayoutContext<'a, 'b, 'c, 'd, V> {
    pub(crate) legacy_cx: &'d mut LegacyLayoutContext<'a, 'b, 'c, Adapter<V>>,
}

#[derive(Deref, DerefMut)]
pub struct PaintContext<'a, 'b, 'c, 'd, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, Adapter<V>>,
    pub(crate) scene: &'d mut gpui::SceneBuilder,
}

pub trait Element<V: 'static>: 'static + Clone {
    fn style_mut(&mut self) -> &mut Style;
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<NodeId>;
    fn paint(&mut self, layout: Layout, view: &mut V, cx: &mut PaintContext<V>) -> Result<()>;

    /// Convert to a dynamically-typed element suitable for layout and paint.
    fn into_any(self) -> AnyElement<V>
    where
        Self: 'static + Sized,
    {
        AnyElement {
            element: Box::new(self) as Box<dyn ElementObject<V>>,
            layout_node_id: None,
        }
    }

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
    fn inset(mut self, length: DefinedLength) -> Self
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
    fn w(mut self, length: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.width = length;
        self
    }

    #[tailwind_lengths]
    fn min_w(mut self, length: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.width = length;
        self
    }

    #[tailwind_lengths]
    fn h(mut self, length: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.height = length;
        self
    }
}

pub trait ElementObject<V> {
    fn clone_object(&self) -> Box<dyn ElementObject<V>>;
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<NodeId>;
    fn paint(&mut self, layout: Layout, view: &mut V, cx: &mut PaintContext<V>) -> Result<()>;
}

impl<V: 'static, E: Element<V>> ElementObject<V> for E {
    fn clone_object(&self) -> Box<dyn ElementObject<V>> {
        Box::new(Clone::clone(self))
    }

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<NodeId> {
        self.layout(view, cx)
    }

    fn paint(&mut self, layout: Layout, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
        self.paint(layout, view, cx)
    }
}

pub struct AnyElement<V> {
    element: Box<dyn ElementObject<V>>,
    layout_node_id: Option<NodeId>,
}

impl<V> AnyElement<V> {
    pub fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<NodeId> {
        let layout_node_id = self.element.layout(view, cx)?;
        self.layout_node_id = Some(layout_node_id);
        Ok(layout_node_id)
    }

    pub fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
        let layout_node_id = self.layout_node_id.expect("paint called before layout");
        let layout = cx
            .layout_engine()
            .unwrap()
            .computed_layout(layout_node_id)
            .expect("you can currently only use playground elements within an adapter");
        self.element.paint(layout, view, cx)
    }
}

impl<V> Clone for AnyElement<V> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone_object(),
            layout_node_id: self.layout_node_id,
        }
    }
}
