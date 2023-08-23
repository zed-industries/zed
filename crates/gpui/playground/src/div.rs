use crate::{
    element::{AnyElement, Element, Layout, ParentElement},
    interactive::{InteractionHandlers, Interactive},
    layout_context::LayoutContext,
    paint_context::PaintContext,
    style::{Style, StyleHelpers, StyleRefinement, Styleable},
};
use anyhow::Result;
use gpui::LayoutId;
use smallvec::SmallVec;

pub struct Div<V: 'static> {
    style: StyleRefinement,
    handlers: InteractionHandlers<V>,
    children: SmallVec<[AnyElement<V>; 2]>,
}

pub fn div<V>() -> Div<V> {
    Div {
        style: Default::default(),
        handlers: Default::default(),
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
        for child in &mut self.children {
            child.paint(view, cx);
        }
    }
}

impl<V> Styleable for Div<V> {
    type Style = Style;

    fn declared_style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<V> StyleHelpers for Div<V> {}

impl<V> Interactive<V> for Div<V> {
    fn interaction_handlers(&mut self) -> &mut InteractionHandlers<V> {
        &mut self.handlers
    }
}

impl<V: 'static> ParentElement<V> for Div<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
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
