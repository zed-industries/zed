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
pub struct Layout<V, E: Element<V>> {
    id: LayoutId,
    engine_layout: Option<EngineLayout>,
    #[deref]
    #[deref_mut]
    element_data: E::Layout,
}

impl<V: 'static, E: Element<V>> Layout<V, E> {
    pub fn new(id: LayoutId, engine_layout: Option<EngineLayout>, element_data: E::Layout) -> Self {
        Self {
            id,
            engine_layout,
            element_data,
        }
    }

    pub fn bounds(&mut self, cx: &mut PaintContext<V>) -> RectF {
        self.engine_layout(cx).bounds
    }

    fn engine_layout(&mut self, cx: &mut PaintContext<'_, '_, '_, '_, V>) -> &mut EngineLayout {
        self.engine_layout
            .get_or_insert_with(|| cx.computed_layout(self.id).log_err().unwrap_or_default())
    }
}

pub trait Element<V> {
    type Layout;

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<Layout<V, Self>>
    where
        Self: Sized;

    fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>)
    where
        Self: Sized;

    /// ## Helpers

    fn hoverable(self) -> Hoverable<V, Self>
    where
        Self: Styleable + Sized,
    {
        hoverable(self)
    }
}

use crate as playground; // Macro invocation below references this crate as playground.
pub trait Styleable {
    type Style: refineable::Refineable;

    fn declared_style(&mut self) -> &mut playground::style::StyleRefinement;

    fn style(&mut self) -> playground::style::Style {
        let mut style = playground::style::Style::default();
        style.refine(self.declared_style());
        style
    }

    // Tailwind-style helpers methods that take and return mut self
    //
    // Example:
    // // Sets the padding to 0.5rem, just like class="p-2" in Tailwind.
    // fn p_2(mut self) -> Self where Self: Sized;
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

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<Layout<V, Self>>
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

    fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>)
    where
        Self: Sized,
    {
        let style = self.style();
    }
}

pub struct Hoverable<V, E: Element<V> + Styleable> {
    default_style: Style,
    hovered_style: StyleRefinement,
    child: E,
    view_type: PhantomData<V>,
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

pub fn hoverable<V, E: Element<V> + Styleable>(mut child: E) -> Hoverable<V, E> {
    Hoverable {
        default_style: child.style(),
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

impl<V, E: Element<V> + Styleable> Element<V> for Hoverable<V, E> {
    type Layout = E::Layout;

    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<Layout<V, Self>>
    where
        Self: Sized,
    {
        todo!()
    }

    fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>)
    where
        Self: Sized,
    {
        todo!()
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
