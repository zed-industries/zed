use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui::geometry::rect::RectF;
use gpui::EngineLayout;
use std::marker::PhantomData;
use util::ResultExt;

use crate::layout_context::LayoutContext;
use crate::paint_context::PaintContext;

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

    fn into_any(mut self) -> AnyElement<V>
    where
        Self: Sized,
    {
        AnyElement(Box::new(ElementWithLayout {
            element: self,
            layout: None,
        }))
    }
}

trait ElementTraitObject<V> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId>;
    fn paint(&mut self, view: &mut V, layout_id: LayoutId, cx: &mut PaintContext<V>);
}

struct ElementWithLayout<V, E: Element<V>> {
    element: E,
    layout: Option<Layout<V, E::Layout>>,
}

impl<V, E: Element<V>> ElementTraitObject<V> for ElementWithLayout<V, E> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId> {
        let layout = Element::layout(self, view, cx)?;
        let layout_id = layout.id;
        self.layout = Some(layout);
        Ok(layout_id)
    }

    fn paint(&mut self, view: &mut V, layout_id: LayoutId, cx: &mut PaintContext<V>) {
        let layout = self.layout.as_mut().expect("paint called before layout");
        Element::paint(self, view, layout, cx);
    }
}

pub struct AnyElement<V>(Box<dyn ElementTraitObject<V>>);

impl<V> AnyElement<V> {
    fn layout(&mut self, view: &mut V, cx: &mut LayoutContext<V>) -> Result<LayoutId> {
        self.0.layout(view, cx)
    }

    fn paint(&mut self, view: &mut V, layout_id: LayoutId, cx: &mut PaintContext<V>) {
        self.0.paint(view, layout_id, cx)
    }
}

pub trait ParentElement<V> {
    fn children_mut(&mut self) -> &mut Vec<AnyElement<V>>;

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

pub trait IntoElement<V> {
    type Element: Element<V>;

    fn into_element(self) -> Self::Element;
}
