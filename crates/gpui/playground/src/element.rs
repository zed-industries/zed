use std::{any::Any, rc::Rc};

use crate::{
    adapter::Adapter,
    style::{Display, ElementStyle, Fill, Overflow, Position},
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::{
    geometry::{DefinedLength, Length},
    scene::MouseClick,
    EngineLayout, LayoutContext as LegacyLayoutContext, PaintContext as LegacyPaintContext,
};
use playground_macros::tailwind_lengths;
pub use taffy::tree::NodeId;

#[derive(Deref, DerefMut)]
pub struct LayoutContext<'a, 'b, 'c, 'd, V> {
    pub(crate) legacy_cx: &'d mut LegacyLayoutContext<'a, 'b, 'c, V>,
}

#[derive(Deref, DerefMut)]
pub struct PaintContext<'a, 'b, 'c, 'd, V> {
    #[deref]
    #[deref_mut]
    pub(crate) legacy_cx: &'d mut LegacyPaintContext<'a, 'b, 'c, V>,
    pub(crate) scene: &'d mut gpui::SceneBuilder,
}

pub struct Layout<'a, E: ?Sized> {
    pub from_engine: EngineLayout,
    pub from_element: &'a mut E,
}

pub struct ElementHandlers<V> {
    click: Option<Rc<dyn Fn(&mut V, MouseClick)>>,
}

pub struct ElementMetadata<V> {
    pub style: ElementStyle,
    pub handlers: ElementHandlers<V>,
}

impl<V> Default for ElementMetadata<V> {
    fn default() -> Self {
        Self {
            style: ElementStyle::default(),
            handlers: ElementHandlers::default(),
        }
    }
}

impl<V> Default for ElementHandlers<V> {
    fn default() -> Self {
        ElementHandlers { click: None }
    }
}

pub trait Element<V: 'static>: 'static {
    type Layout: 'static;

    fn style_mut(&mut self) -> &mut ElementStyle;
    fn handlers_mut(&mut self) -> &mut ElementHandlers<V>;
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>)
        -> Result<(NodeId, Self::Layout)>;
    fn paint<'a>(
        &mut self,
        layout: Layout<Self::Layout>,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Result<()>;

    /// Convert to a dynamically-typed element suitable for layout and paint.
    fn into_any(self) -> AnyElement<V>
    where
        Self: 'static + Sized,
    {
        AnyElement {
            element: Box::new(self) as Box<dyn ElementObject<V>>,
            layout: None,
        }
    }

    fn adapt(self) -> Adapter<V>
    where
        Self: Sized,
        Self: Element<V>,
    {
        Adapter(self.into_any())
    }

    fn click(mut self, handler: impl Fn(&mut V, MouseClick) + 'static) -> Self
    where
        Self: Sized,
    {
        self.handlers_mut().click = Some(Rc::new(move |view, event| {
            handler(view, event);
        }));
        self
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
    fn inset_(mut self, length: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().inset.top = length;
        self.style_mut().inset.right = length;
        self.style_mut().inset.bottom = length;
        self.style_mut().inset.left = length;
        self
    }

    fn w(mut self, width: impl Into<Length>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.width = width.into();
        self
    }

    fn w_auto(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.width = Length::Auto;
        self
    }

    #[tailwind_lengths]
    fn w_(mut self, length: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.width = length;
        self
    }

    #[tailwind_lengths]
    fn min_w_(mut self, length: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().min_size.width = length;
        self
    }

    fn h(mut self, height: impl Into<Length>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.height = height.into();
        self
    }

    fn h_auto(mut self) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.height = Length::Auto;
        self
    }

    #[tailwind_lengths]
    fn h_(mut self, height: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().size.height = height;
        self
    }

    #[tailwind_lengths]
    fn min_h_(mut self, length: DefinedLength) -> Self
    where
        Self: Sized,
    {
        self.style_mut().min_size.height = length;
        self
    }

    fn fill(mut self, fill: impl Into<Fill>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().fill = fill.into();
        self
    }
}

pub trait ElementObject<V> {
    fn style_mut(&mut self) -> &mut ElementStyle;
    fn handlers_mut(&mut self) -> &mut ElementHandlers<V>;
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>)
        -> Result<(NodeId, Box<dyn Any>)>;
    fn paint(
        &mut self,
        layout: Layout<dyn Any>,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Result<()>;
}

impl<V: 'static, E: Element<V>> ElementObject<V> for E {
    fn style_mut(&mut self) -> &mut ElementStyle {
        Element::style_mut(self)
    }

    fn handlers_mut(&mut self) -> &mut ElementHandlers<V> {
        Element::handlers_mut(self)
    }

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Result<(NodeId, Box<dyn Any>)> {
        let (node_id, layout) = self.layout(view, cx)?;
        let layout = Box::new(layout) as Box<dyn Any>;
        Ok((node_id, layout))
    }

    fn paint(
        &mut self,
        layout: Layout<dyn Any>,
        view: &mut V,
        cx: &mut PaintContext<V>,
    ) -> Result<()> {
        let layout = Layout {
            from_engine: layout.from_engine,
            from_element: layout.from_element.downcast_mut::<E::Layout>().unwrap(),
        };

        self.paint(layout, view, cx)
    }

    // fn clone_object(&self) -> Box<dyn ElementObject<V>> {
    //     Box::new(Clone::clone(self))
    // }
}

pub struct AnyElement<V> {
    element: Box<dyn ElementObject<V>>,
    layout: Option<(NodeId, Box<dyn Any>)>,
}

impl<V> AnyElement<V> {
    pub fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<NodeId> {
        let (node_id, layout) = self.element.layout(view, cx)?;
        self.layout = Some((node_id, layout));
        Ok(node_id)
    }

    pub fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
        let (layout_node_id, element_layout) =
            self.layout.as_mut().expect("paint called before layout");

        let layout = Layout {
            from_engine: cx
                .layout_engine()
                .unwrap()
                .computed_layout(*layout_node_id)
                .expect("you can currently only use playground elements within an adapter"),
            from_element: element_layout.as_mut(),
        };

        self.element.paint(layout, view, cx)
    }
}

impl<V: 'static> Element<V> for AnyElement<V> {
    type Layout = ();

    fn style_mut(&mut self) -> &mut ElementStyle {
        self.element.style_mut()
    }

    fn handlers_mut(&mut self) -> &mut ElementHandlers<V> {
        self.element.handlers_mut()
    }

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Result<(NodeId, Self::Layout)> {
        Ok((self.layout(view, cx)?, ()))
    }

    fn paint(&mut self, layout: Layout<()>, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
        self.paint(view, cx)
    }
}
