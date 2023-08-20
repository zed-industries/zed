use crate::{
    element::{AnyElement, Element, Layout},
    layout_context::LayoutContext,
    paint_context::PaintContext,
    style::{Style, StyleRefinement, Styleable},
};
use anyhow::Result;
use gpui::{platform::MouseMovedEvent, EventContext, LayoutId};
use smallvec::SmallVec;
use std::rc::Rc;

pub struct Div<V> {
    style: StyleRefinement,
    children: SmallVec<[AnyElement<V>; 2]>,
}

impl<V> Styleable for Div<V> {
    type Style = Style;

    fn declared_style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

pub fn div<V>() -> Div<V> {
    Div {
        style: Default::default(),
        children: Default::default(),
    }
}

impl<V: 'static> Element<V> for Div<V> {
    type Layout = ();

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<Layout<V, ()>>
    where
        Self: Sized,
    {
        let children = self
            .children
            .iter_mut()
            .map(|child| child.layout(view, cx))
            .collect::<Result<Vec<LayoutId>>>()?;

        cx.add_layout_node(self.style(), (), children)
    }

    fn paint(&mut self, view: &mut V, layout: &mut Layout<V, ()>, cx: &mut PaintContext<V>)
    where
        Self: Sized,
    {
        let style = self.style();

        style.paint_background::<V, Self>(layout, cx);
    }
}

pub trait Interactive<V> {
    fn declared_interactions(&mut self) -> &mut Interactions<V>;

    fn on_mouse_move<H>(mut self, handler: H) -> Self
    where
        H: 'static + Fn(&mut V, &MouseMovedEvent, &mut EventContext<V>),
        Self: Sized,
    {
        self.declared_interactions().mouse_moved = Some(Rc::new(move |view, event, cx| {
            handler(view, event, cx);
            cx.bubble
        }));
        self
    }
}

pub struct Interactions<V> {
    mouse_moved: Option<Rc<dyn Fn(&mut V, &MouseMovedEvent, &mut EventContext<V>) -> bool>>,
}

#[test]
fn test() {
    // let elt = div().w_auto();
}

// trait Element<V: 'static> {
//     type Style;

//     fn layout()
// }

// trait Stylable<V: 'static>: Element<V> {
//     type Style;

//     fn with_style(self, style: Self::Style) -> Self;
// }

// pub struct HoverStyle<S> {
//     default: S,
//     hovered: S,
// }

// struct Hover<V: 'static, C: Stylable<V>> {
//     child: C,
//     style: HoverStyle<C::Style>,
// }

// impl<V: 'static, C: Stylable<V>> Hover<V, C> {
//     fn new(child: C, style: HoverStyle<C::Style>) -> Self {
//         Self { child, style }
//     }
// }
