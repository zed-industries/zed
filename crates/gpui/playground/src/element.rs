use crate::{
    adapter::Adapter,
    color::Hsla,
    style::{Display, ElementStyle, Fill, Overflow, Position},
};
use anyhow::Result;
pub use gpui::LayoutContext;
use gpui::{
    geometry::{DefinedLength, Length},
    platform::{MouseButton, MouseButtonEvent},
    EngineLayout, EventContext, RenderContext, ViewContext,
};
use playground_macros::tailwind_lengths;
use std::{
    any::{Any, TypeId},
    cell::Cell,
    rc::Rc,
};

pub use crate::paint_context::PaintContext;
pub use taffy::tree::NodeId;

pub struct Layout<'a, E: ?Sized> {
    pub from_engine: EngineLayout,
    pub from_element: &'a mut E,
}

pub struct ElementMetadata<V> {
    pub style: ElementStyle,
    pub handlers: Vec<EventHandler<V>>,
}

pub struct EventHandler<V> {
    handler: Rc<dyn Fn(&mut V, &dyn Any, &mut EventContext<V>)>,
    event_type: TypeId,
    outside_bounds: bool,
}

impl<V> Clone for EventHandler<V> {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
            event_type: self.event_type,
            outside_bounds: self.outside_bounds,
        }
    }
}

impl<V> Default for ElementMetadata<V> {
    fn default() -> Self {
        Self {
            style: ElementStyle::default(),
            handlers: Vec::new(),
        }
    }
}

pub trait Element<V: 'static>: 'static {
    type Layout: 'static;

    fn style_mut(&mut self) -> &mut ElementStyle;
    fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>>;

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

    fn click(
        self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut ViewContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        let pressed: Rc<Cell<bool>> = Default::default();
        self.mouse_down(button, {
            let pressed = pressed.clone();
            move |_, _, _| {
                pressed.set(true);
            }
        })
        .mouse_up_outside(button, {
            let pressed = pressed.clone();
            move |_, _, _| {
                pressed.set(false);
            }
        })
        .mouse_up(button, move |view, event, event_cx| {
            if pressed.get() {
                pressed.set(false);
                handler(view, event, event_cx);
            }
        })
    }

    fn mouse_down(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.handlers_mut().push(EventHandler {
            handler: Rc::new(move |view, event, event_cx| {
                let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                if event.button == button && event.is_down {
                    handler(view, event, event_cx);
                }
            }),
            event_type: TypeId::of::<MouseButtonEvent>(),
            outside_bounds: false,
        });
        self
    }

    fn mouse_down_outside(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.handlers_mut().push(EventHandler {
            handler: Rc::new(move |view, event, event_cx| {
                let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                if event.button == button && event.is_down {
                    handler(view, event, event_cx);
                }
            }),
            event_type: TypeId::of::<MouseButtonEvent>(),
            outside_bounds: true,
        });
        self
    }

    fn mouse_up(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.handlers_mut().push(EventHandler {
            handler: Rc::new(move |view, event, event_cx| {
                let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                if event.button == button && !event.is_down {
                    handler(view, event, event_cx);
                }
            }),
            event_type: TypeId::of::<MouseButtonEvent>(),
            outside_bounds: false,
        });
        self
    }

    fn mouse_up_outside(
        mut self,
        button: MouseButton,
        handler: impl Fn(&mut V, &MouseButtonEvent, &mut EventContext<V>) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.handlers_mut().push(EventHandler {
            handler: Rc::new(move |view, event, event_cx| {
                let event = event.downcast_ref::<MouseButtonEvent>().unwrap();
                if event.button == button && !event.is_down {
                    handler(view, event, event_cx);
                }
            }),
            event_type: TypeId::of::<MouseButtonEvent>(),
            outside_bounds: true,
        });
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
        self.style_mut().fill = Some(fill.into());
        self
    }

    fn hover_fill(mut self, fill: impl Into<Fill>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().hover_fill = Some(fill.into());
        self
    }

    fn text_color(mut self, color: impl Into<Hsla>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().text_color = Some(color.into());
        self
    }
}

// Object-safe counterpart of Element used by AnyElement to store elements as trait objects.
trait ElementObject<V> {
    fn style_mut(&mut self) -> &mut ElementStyle;
    fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>>;
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

    fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
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
}

/// A dynamically typed element.
pub struct AnyElement<V> {
    element: Box<dyn ElementObject<V>>,
    layout: Option<(NodeId, Box<dyn Any>)>,
}

impl<V: 'static> AnyElement<V> {
    pub fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<NodeId> {
        let pushed_text_style = self.push_text_style(cx);

        let (node_id, layout) = self.element.layout(view, cx)?;
        self.layout = Some((node_id, layout));

        if pushed_text_style {
            cx.pop_text_style();
        }

        Ok(node_id)
    }

    pub fn push_text_style(&mut self, cx: &mut impl RenderContext) -> bool {
        let text_style = self.element.style_mut().text_style();
        if let Some(text_style) = text_style {
            let mut current_text_style = cx.text_style();
            text_style.apply(&mut current_text_style);
            cx.push_text_style(current_text_style);
            true
        } else {
            false
        }
    }

    pub fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) -> Result<()> {
        let pushed_text_style = self.push_text_style(cx);

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

        for event_handler in self.element.handlers_mut().iter().cloned() {
            let EngineLayout { order, bounds } = layout.from_engine;

            let view_id = cx.view_id();
            let view_event_handler = event_handler.handler.clone();

            // TODO: Tuck this into a method on PaintContext.
            cx.scene
                .interactive_regions
                .push(gpui::scene::InteractiveRegion {
                    order,
                    bounds,
                    outside_bounds: event_handler.outside_bounds,
                    event_handler: Rc::new(move |view, event, window_cx, view_id| {
                        let mut view_context = ViewContext::mutable(window_cx, view_id);
                        let mut event_context = EventContext::new(&mut view_context);
                        view_event_handler(view.downcast_mut().unwrap(), event, &mut event_context);
                    }),
                    event_type: event_handler.event_type,
                    view_id,
                });
        }

        self.element.paint(layout, view, cx)?;
        if pushed_text_style {
            cx.pop_text_style();
        }

        Ok(())
    }
}

impl<V: 'static> Element<V> for AnyElement<V> {
    type Layout = ();

    fn style_mut(&mut self) -> &mut ElementStyle {
        self.element.style_mut()
    }

    fn handlers_mut(&mut self) -> &mut Vec<EventHandler<V>> {
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

pub trait IntoElement<V: 'static> {
    type Element: Element<V>;

    fn into_element(self) -> Self::Element;

    fn into_any_element(self) -> AnyElement<V>
    where
        Self: Sized,
    {
        self.into_element().into_any()
    }
}
