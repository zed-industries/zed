use crate::{
    private::Sealed, AnyBox, AnyElement, AnyModel, AnyWeakModel, AppContext, AvailableSpace,
    BorrowWindow, Bounds, Component, Element, ElementId, Entity, EntityId, Flatten, FocusHandle,
    FocusableView, LayoutId, Model, Pixels, Point, Size, ViewContext, VisualContext, WeakModel,
    WindowContext,
};
use anyhow::{Context, Result};
use std::{
    any::{Any, TypeId},
    hash::{Hash, Hasher},
};

pub trait Render: 'static + Sized {
    type Element: Element<Self> + 'static;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element;
}

pub struct View<V> {
    pub(crate) model: Model<V>,
}

impl<V> Sealed for View<V> {}

impl<V: 'static> Entity<V> for View<V> {
    type Weak = WeakView<V>;

    fn entity_id(&self) -> EntityId {
        self.model.entity_id
    }

    fn downgrade(&self) -> Self::Weak {
        WeakView {
            model: self.model.downgrade(),
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        let model = weak.model.upgrade()?;
        Some(View { model })
    }
}

impl<V: 'static> View<V> {
    /// Convert this strong view reference into a weak view reference.
    pub fn downgrade(&self) -> WeakView<V> {
        Entity::downgrade(self)
    }

    pub fn update<C, R>(
        &self,
        cx: &mut C,
        f: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> C::Result<R>
    where
        C: VisualContext,
    {
        cx.update_view(self, f)
    }

    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a V {
        self.model.read(cx)
    }

    pub fn render_with<C>(&self, component: C) -> RenderViewWith<C, V>
    where
        C: 'static + Component<V>,
    {
        RenderViewWith {
            view: self.clone(),
            component: Some(component),
        }
    }

    pub fn focus_handle(&self, cx: &AppContext) -> FocusHandle
    where
        V: FocusableView,
    {
        self.read(cx).focus_handle(cx)
    }
}

impl<V> Clone for View<V> {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}

impl<V> Hash for View<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.model.hash(state);
    }
}

impl<V> PartialEq for View<V> {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model
    }
}

impl<V> Eq for View<V> {}

impl<V: Render, ParentViewState: 'static> Component<ParentViewState> for View<V> {
    fn render(self) -> AnyElement<ParentViewState> {
        AnyElement::new(AnyView::from(self))
    }
}

pub struct WeakView<V> {
    pub(crate) model: WeakModel<V>,
}

impl<V: 'static> WeakView<V> {
    pub fn entity_id(&self) -> EntityId {
        self.model.entity_id
    }

    pub fn upgrade(&self) -> Option<View<V>> {
        Entity::upgrade_from(self)
    }

    pub fn update<C, R>(
        &self,
        cx: &mut C,
        f: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Result<R>
    where
        C: VisualContext,
        Result<C::Result<R>>: Flatten<R>,
    {
        let view = self.upgrade().context("error upgrading view")?;
        Ok(view.update(cx, f)).flatten()
    }
}

impl<V> Clone for WeakView<V> {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}

impl<V> Hash for WeakView<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.model.hash(state);
    }
}

impl<V> PartialEq for WeakView<V> {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model
    }
}

impl<V> Eq for WeakView<V> {}

#[derive(Clone, Debug)]
pub struct AnyView {
    model: AnyModel,
    layout: fn(&AnyView, &mut WindowContext) -> (LayoutId, Box<dyn Any>),
    paint: fn(&AnyView, &mut AnyBox, &mut WindowContext),
}

impl AnyView {
    pub fn downgrade(&self) -> AnyWeakView {
        AnyWeakView {
            model: self.model.downgrade(),
            layout: self.layout,
            paint: self.paint,
        }
    }

    pub fn downcast<T: 'static>(self) -> Result<View<T>, Self> {
        match self.model.downcast() {
            Ok(model) => Ok(View { model }),
            Err(model) => Err(Self {
                model,
                layout: self.layout,
                paint: self.paint,
            }),
        }
    }

    pub fn entity_type(&self) -> TypeId {
        self.model.entity_type
    }

    pub(crate) fn draw(
        &self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) {
        cx.with_absolute_element_offset(origin, |cx| {
            let (layout_id, mut rendered_element) = (self.layout)(self, cx);
            cx.window
                .layout_engine
                .compute_layout(layout_id, available_space);
            (self.paint)(self, &mut rendered_element, cx);
        })
    }
}

impl<V: 'static> Component<V> for AnyView {
    fn render(self) -> AnyElement<V> {
        AnyElement::new(self)
    }
}

impl<V: Render> From<View<V>> for AnyView {
    fn from(value: View<V>) -> Self {
        AnyView {
            model: value.model.into_any(),
            layout: any_view::layout::<V>,
            paint: any_view::paint::<V>,
        }
    }
}

impl<ParentViewState: 'static> Element<ParentViewState> for AnyView {
    type ElementState = Box<dyn Any>;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.model.entity_id.into())
    }

    fn layout(
        &mut self,
        _view_state: &mut ParentViewState,
        _element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<ParentViewState>,
    ) -> (LayoutId, Self::ElementState) {
        (self.layout)(self, cx)
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _view_state: &mut ParentViewState,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentViewState>,
    ) {
        (self.paint)(self, rendered_element, cx)
    }
}

pub struct AnyWeakView {
    model: AnyWeakModel,
    layout: fn(&AnyView, &mut WindowContext) -> (LayoutId, Box<dyn Any>),
    paint: fn(&AnyView, &mut AnyBox, &mut WindowContext),
}

impl AnyWeakView {
    pub fn upgrade(&self) -> Option<AnyView> {
        let model = self.model.upgrade()?;
        Some(AnyView {
            model,
            layout: self.layout,
            paint: self.paint,
        })
    }
}

impl<V: Render> From<WeakView<V>> for AnyWeakView {
    fn from(view: WeakView<V>) -> Self {
        Self {
            model: view.model.into(),
            layout: any_view::layout::<V>,
            paint: any_view::paint::<V>,
        }
    }
}

// impl<T, E> Render for T
// where
//     T: 'static + FnMut(&mut WindowContext) -> E,
//     E: 'static + Send + Element<T>,
// {
//     type Element = E;

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
//         (self)(cx)
//     }
// }

pub struct RenderViewWith<C, V> {
    view: View<V>,
    component: Option<C>,
}

impl<C, ParentViewState, ViewState> Component<ParentViewState> for RenderViewWith<C, ViewState>
where
    C: 'static + Component<ViewState>,
    ParentViewState: 'static,
    ViewState: 'static,
{
    fn render(self) -> AnyElement<ParentViewState> {
        AnyElement::new(self)
    }
}

impl<C, ParentViewState, ViewState> Element<ParentViewState> for RenderViewWith<C, ViewState>
where
    C: 'static + Component<ViewState>,
    ParentViewState: 'static,
    ViewState: 'static,
{
    type ElementState = AnyElement<ViewState>;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.view.entity_id().into())
    }

    fn layout(
        &mut self,
        _: &mut ParentViewState,
        _: Option<Self::ElementState>,
        cx: &mut ViewContext<ParentViewState>,
    ) -> (LayoutId, Self::ElementState) {
        self.view.update(cx, |view, cx| {
            let mut element = self.component.take().unwrap().render();
            let layout_id = element.layout(view, cx);
            (layout_id, element)
        })
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut ParentViewState,
        element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentViewState>,
    ) {
        self.view.update(cx, |view, cx| element.paint(view, cx))
    }
}

mod any_view {
    use crate::{AnyElement, AnyView, BorrowWindow, LayoutId, Render, WindowContext};
    use std::any::Any;

    pub(crate) fn layout<V: Render>(
        view: &AnyView,
        cx: &mut WindowContext,
    ) -> (LayoutId, Box<dyn Any>) {
        cx.with_element_id(Some(view.model.entity_id), |cx| {
            let view = view.clone().downcast::<V>().unwrap();
            view.update(cx, |view, cx| {
                let mut element = AnyElement::new(view.render(cx));
                let layout_id = element.layout(view, cx);
                (layout_id, Box::new(element) as Box<dyn Any>)
            })
        })
    }

    pub(crate) fn paint<V: Render>(
        view: &AnyView,
        element: &mut Box<dyn Any>,
        cx: &mut WindowContext,
    ) {
        cx.with_element_id(Some(view.model.entity_id), |cx| {
            let view = view.clone().downcast::<V>().unwrap();
            let element = element.downcast_mut::<AnyElement<V>>().unwrap();
            view.update(cx, |view, cx| element.paint(view, cx))
        })
    }
}
