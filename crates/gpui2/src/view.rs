use crate::{
    AnyBox, AnyElement, AvailableSpace, BorrowWindow, Bounds, Component, Element, ElementId,
    EntityId, LayoutId, Model, Pixels, Size, ViewContext, VisualContext, WeakModel, WindowContext,
};
use anyhow::{Context, Result};
use std::{marker::PhantomData, sync::Arc};

pub trait Render: 'static + Sized {
    type Element: Element<Self> + 'static + Send;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element;
}

pub struct View<V> {
    pub(crate) model: Model<V>,
}

impl<V: Render> View<V> {
    pub fn into_any(self) -> AnyView {
        AnyView(Arc::new(self))
    }
}

impl<V: 'static> View<V> {
    pub fn downgrade(&self) -> WeakView<V> {
        WeakView {
            model: self.model.downgrade(),
        }
    }

    pub fn update<C, R>(
        &self,
        cx: &mut C,
        f: impl FnOnce(&mut V, &mut C::ViewContext<'_, '_, V>) -> R,
    ) -> C::Result<R>
    where
        C: VisualContext,
    {
        cx.update_view(self, f)
    }
}

impl<V> Clone for View<V> {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}

impl<V: Render, ParentViewState: 'static> Component<ParentViewState> for View<V> {
    fn render(self) -> AnyElement<ParentViewState> {
        AnyElement::new(EraseViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl<V> Element<()> for View<V>
where
    V: Render,
{
    type ElementState = AnyElement<V>;

    fn id(&self) -> Option<crate::ElementId> {
        Some(ElementId::View(self.model.entity_id))
    }

    fn initialize(
        &mut self,
        _: &mut (),
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<()>,
    ) -> Self::ElementState {
        self.update(cx, |state, cx| {
            let mut any_element = AnyElement::new(state.render(cx));
            any_element.initialize(state, cx);
            any_element
        })
    }

    fn layout(
        &mut self,
        _: &mut (),
        element: &mut Self::ElementState,
        cx: &mut ViewContext<()>,
    ) -> LayoutId {
        self.update(cx, |state, cx| element.layout(state, cx))
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut (),
        element: &mut Self::ElementState,
        cx: &mut ViewContext<()>,
    ) {
        self.update(cx, |state, cx| element.paint(state, cx))
    }
}

pub struct WeakView<V> {
    pub(crate) model: WeakModel<V>,
}

impl<V: 'static> WeakView<V> {
    pub fn upgrade(&self) -> Option<View<V>> {
        let model = self.model.upgrade()?;
        Some(View { model })
    }

    pub fn update<R>(
        &self,
        cx: &mut WindowContext,
        f: impl FnOnce(&mut V, &mut ViewContext<V>) -> R,
    ) -> Result<R> {
        let view = self.upgrade().context("error upgrading view")?;
        Ok(view.update(cx, f))
    }
}

impl<V> Clone for WeakView<V> {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}

struct EraseViewState<V, ParentV> {
    view: View<V>,
    parent_view_state_type: PhantomData<ParentV>,
}

unsafe impl<V, ParentV> Send for EraseViewState<V, ParentV> {}

impl<V: Render, ParentV: 'static> Component<ParentV> for EraseViewState<V, ParentV> {
    fn render(self) -> AnyElement<ParentV> {
        AnyElement::new(self)
    }
}

impl<V: Render, ParentV: 'static> Element<ParentV> for EraseViewState<V, ParentV> {
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Element::id(&self.view)
    }

    fn initialize(
        &mut self,
        _: &mut ParentV,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<ParentV>,
    ) -> Self::ElementState {
        ViewObject::initialize(&mut self.view, cx)
    }

    fn layout(
        &mut self,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) -> LayoutId {
        ViewObject::layout(&mut self.view, element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) {
        ViewObject::paint(&mut self.view, bounds, element, cx)
    }
}

trait ViewObject: Send + Sync {
    fn entity_id(&self) -> EntityId;
    fn initialize(&self, cx: &mut WindowContext) -> AnyBox;
    fn layout(&self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId;
    fn paint(&self, bounds: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext);
}

impl<V> ViewObject for View<V>
where
    V: Render,
{
    fn entity_id(&self) -> EntityId {
        self.model.entity_id
    }

    fn initialize(&self, cx: &mut WindowContext) -> AnyBox {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let mut any_element = Box::new(AnyElement::new(state.render(cx)));
                any_element.initialize(state, cx);
                any_element
            })
        })
    }

    fn layout(&self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.layout(state, cx)
            })
        })
    }

    fn paint(&self, _: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext) {
        cx.with_element_id(self.entity_id(), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.paint(state, cx);
            });
        });
    }
}

#[derive(Clone)]
pub struct AnyView(Arc<dyn ViewObject>);

impl AnyView {
    pub(crate) fn draw(&self, available_space: Size<AvailableSpace>, cx: &mut WindowContext) {
        let mut rendered_element = self.0.initialize(cx);
        let layout_id = self.0.layout(&mut rendered_element, cx);
        cx.window
            .layout_engine
            .compute_layout(layout_id, available_space);
        let bounds = cx.window.layout_engine.layout_bounds(layout_id);
        self.0.paint(bounds, &mut rendered_element, cx);
    }
}

impl<ParentV: 'static> Component<ParentV> for AnyView {
    fn render(self) -> AnyElement<ParentV> {
        AnyElement::new(EraseAnyViewState {
            view: self,
            parent_view_state_type: PhantomData,
        })
    }
}

impl Element<()> for AnyView {
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Some(ElementId::View(self.0.entity_id()))
    }

    fn initialize(
        &mut self,
        _: &mut (),
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<()>,
    ) -> Self::ElementState {
        self.0.initialize(cx)
    }

    fn layout(
        &mut self,
        _: &mut (),
        element: &mut Self::ElementState,
        cx: &mut ViewContext<()>,
    ) -> LayoutId {
        self.0.layout(element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut (),
        element: &mut AnyBox,
        cx: &mut ViewContext<()>,
    ) {
        self.0.paint(bounds, element, cx)
    }
}

struct EraseAnyViewState<ParentViewState> {
    view: AnyView,
    parent_view_state_type: PhantomData<ParentViewState>,
}

unsafe impl<ParentV> Send for EraseAnyViewState<ParentV> {}

impl<ParentV: 'static> Component<ParentV> for EraseAnyViewState<ParentV> {
    fn render(self) -> AnyElement<ParentV> {
        AnyElement::new(self)
    }
}

impl<ParentV: 'static> Element<ParentV> for EraseAnyViewState<ParentV> {
    type ElementState = AnyBox;

    fn id(&self) -> Option<crate::ElementId> {
        Element::id(&self.view)
    }

    fn initialize(
        &mut self,
        _: &mut ParentV,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<ParentV>,
    ) -> Self::ElementState {
        self.view.0.initialize(cx)
    }

    fn layout(
        &mut self,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) -> LayoutId {
        self.view.0.layout(element, cx)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _: &mut ParentV,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentV>,
    ) {
        self.view.0.paint(bounds, element, cx)
    }
}
