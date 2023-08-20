use std::cell::Cell;
use std::{marker::PhantomData, rc::Rc};

use crate::element::{AnyElement, PaintContext};
use crate::layout_context::LayoutContext;
use crate::style::{Style, StyleRefinement};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::EngineLayout;
use gpui::{geometry::rect::RectF, platform::MouseMovedEvent, EventContext};
use playground_macros::styleable_helpers;
use refineable::Refineable;
use smallvec::SmallVec;
use util::ResultExt;

type LayoutId = gpui::LayoutId;

#[derive(Deref, DerefMut)]
pub struct Layout<V, D> {
    id: LayoutId,
    engine_layout: Option<EngineLayout>,
    #[deref]
    #[deref_mut]
    element_data: D,
    view_type: PhantomData<V>,
}

impl<V: 'static, D> Layout<V, D> {
    pub fn new(id: LayoutId, engine_layout: Option<EngineLayout>, element_data: D) -> Self {
        Self {
            id,
            engine_layout,
            element_data,
            view_type: PhantomData,
        }
    }

    pub fn bounds(&mut self, cx: &mut PaintContext<V>) -> RectF {
        self.engine_layout(cx).bounds
    }

    pub fn order(&mut self, cx: &mut PaintContext<V>) -> u32 {
        self.engine_layout(cx).order
    }

    fn engine_layout(&mut self, cx: &mut PaintContext<'_, '_, '_, '_, V>) -> &mut EngineLayout {
        self.engine_layout
            .get_or_insert_with(|| cx.computed_layout(self.id).log_err().unwrap_or_default())
    }
}

pub trait Element<V> {
    type Layout;

    fn layout(
        &mut self,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Result<Layout<V, Self::Layout>>
    where
        Self: Sized;

    fn paint(
        &mut self,
        view: &mut V,
        layout: &mut Layout<V, Self::Layout>,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized;

    /// ## Helpers

    fn hoverable(self) -> Hoverable<V, Self>
    where
        Self: Styleable + Sized,
    {
        hoverable(self)
    }
}

pub trait Styleable {
    type Style: refineable::Refineable;

    fn declared_style(&mut self) -> &mut playground::style::StyleRefinement;

    fn style(&mut self) -> playground::style::Style {
        let mut style = playground::style::Style::default();
        style.refine(self.declared_style());
        style
    }
}

// Tailwind-style helpers methods that take and return mut self
//
// Example:
// // Sets the padding to 0.5rem, just like class="p-2" in Tailwind.
// fn p_2(mut self) -> Self where Self: Sized;
use crate as playground; // Macro invocation references this crate as playground.
pub trait StyleHelpers: Styleable<Style = Style> {
    styleable_helpers!();
}

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

pub struct Hoverable<V, E: Element<V> + Styleable> {
    hovered: Cell<bool>,
    child_style: StyleRefinement,
    hovered_style: StyleRefinement,
    child: E,
    view_type: PhantomData<V>,
}

pub fn hoverable<V, E: Element<V> + Styleable>(mut child: E) -> Hoverable<V, E> {
    Hoverable {
        hovered: Cell::new(false),
        child_style: child.declared_style().clone(),
        hovered_style: Default::default(),
        child,
        view_type: PhantomData,
    }
}

impl<V, E: Element<V> + Styleable> Styleable for Hoverable<V, E> {
    type Style = E::Style;

    fn declared_style(&mut self) -> &mut playground::style::StyleRefinement {
        self.child.declared_style()
    }
}

impl<V: 'static, E: Element<V> + Styleable> Element<V> for Hoverable<V, E> {
    type Layout = E::Layout;

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<Layout<V, Self::Layout>>
    where
        Self: Sized,
    {
        if self.hovered.get() {
            // If hovered, refine the child's style with this element's style.
            self.child.declared_style().refine(&self.hovered_style);
        } else {
            // Otherwise, set the child's style back to its original style.
            *self.child.declared_style() = self.child_style.clone();
        }

        self.child.layout(view, cx)
    }

    fn paint(
        &mut self,
        view: &mut V,
        layout: &mut Layout<V, Self::Layout>,
        cx: &mut PaintContext<V>,
    ) where
        Self: Sized,
    {
        let bounds = layout.bounds(cx);
        let order = layout.order(cx);
        self.hovered.set(bounds.contains_point(cx.mouse_position()));
        let hovered = self.hovered.clone();
        cx.on_event(order, move |view, event: &MouseMovedEvent, cx| {});
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
