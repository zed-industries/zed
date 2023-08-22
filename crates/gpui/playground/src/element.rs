use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::{geometry::rect::RectF, EngineLayout};
use smallvec::SmallVec;
use std::marker::PhantomData;
use util::ResultExt;

pub use crate::layout_context::LayoutContext;
pub use crate::paint_context::PaintContext;

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
    pub fn new(id: LayoutId, element_data: D) -> Self {
        Self {
            id,
            engine_layout: None,
            element_data: element_data,
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

impl<V: 'static> Layout<V, Option<AnyElement<V>>> {
    pub fn paint(&mut self, view: &mut V, cx: &mut PaintContext<V>) {
        let mut element = self.element_data.take().unwrap();
        element.paint(view, self.id, cx);
        self.element_data = Some(element);
    }
}

pub trait Element<V: 'static>: 'static {
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

    fn into_any(self) -> AnyElement<V>
    where
        Self: 'static + Sized,
    {
        AnyElement(Box::new(ElementState {
            element: self,
            layout: None,
        }))
    }
}

/// Used to make ElementState<V, E> into a trait object, so we can wrap it in AnyElement<V>.
trait ElementStateObject<V> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId>;
    fn paint(&mut self, view: &mut V, layout_id: LayoutId, cx: &mut PaintContext<V>);
}

/// A wrapper around an element that stores its layout state.
struct ElementState<V: 'static, E: Element<V>> {
    element: E,
    layout: Option<Layout<V, E::Layout>>,
}

/// We blanket-implement the object-safe ElementStateObject interface to make ElementStates into trait objects
impl<V, E: Element<V>> ElementStateObject<V> for ElementState<V, E> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId> {
        let layout = self.element.layout(view, cx)?;
        let layout_id = layout.id;
        self.layout = Some(layout);
        Ok(layout_id)
    }

    fn paint(&mut self, view: &mut V, layout_id: LayoutId, cx: &mut PaintContext<V>) {
        let layout = self.layout.as_mut().expect("paint called before layout");
        if layout.engine_layout.is_none() {
            layout.engine_layout = cx.computed_layout(layout_id).log_err()
        }
        self.element.paint(view, layout, cx)
    }
}

/// A dynamic element.
pub struct AnyElement<V>(Box<dyn ElementStateObject<V>>);

impl<V> AnyElement<V> {
    pub fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId> {
        self.0.layout(view, cx)
    }

    pub fn paint(&mut self, view: &mut V, layout_id: LayoutId, cx: &mut PaintContext<V>) {
        self.0.paint(view, layout_id, cx)
    }
}

pub trait ParentElement<V: 'static> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]>;

    fn child(mut self, child: impl IntoElement<V>) -> Self
    where
        Self: Sized,
    {
        self.children_mut().push(child.into_element().into_any());
        self
    }

    fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: IntoElement<V>,
        Self: Sized,
    {
        self.children_mut().extend(
            children
                .into_iter()
                .map(|child| child.into_element().into_any()),
        );
        self
    }
}

pub trait IntoElement<V: 'static> {
    type Element: Element<V>;

    fn into_element(self) -> Self::Element;
}
