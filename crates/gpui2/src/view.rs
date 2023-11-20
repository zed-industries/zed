use crate::{
    private::Sealed, AnyElement, AnyModel, AnyWeakModel, AppContext, AvailableSpace, BorrowWindow,
    Bounds, Element, ElementId, Entity, EntityId, Flatten, FocusHandle, FocusableView, LayoutId,
    Model, Pixels, Point, Render, RenderOnce, Size, ViewContext, VisualContext, WeakModel,
    WindowContext,
};
use anyhow::{Context, Result};
use std::{
    any::{Any, TypeId},
    hash::{Hash, Hasher},
};

pub struct View<V> {
    pub model: Model<V>,
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

    pub fn render_with<E>(&self, component: E) -> RenderViewWith<E, V>
    where
        E: 'static + Element<V>,
    {
        RenderViewWith {
            view: self.clone(),
            element: Some(component),
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
    paint: fn(&AnyView, Box<dyn Any>, &mut WindowContext),
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

    pub fn entity_id(&self) -> EntityId {
        self.model.entity_id()
    }

    pub(crate) fn draw(
        &self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) {
        cx.with_absolute_element_offset(origin, |cx| {
            let (layout_id, rendered_element) = (self.layout)(self, cx);
            cx.window
                .layout_engine
                .compute_layout(layout_id, available_space);
            (self.paint)(self, rendered_element, cx);
        })
    }
}

impl<V: 'static + Render<V>> From<View<V>> for AnyView {
    fn from(value: View<V>) -> Self {
        AnyView {
            model: value.model.into_any(),
            layout: any_view::layout::<V>,
            paint: any_view::paint::<V>,
        }
    }
}

impl<V: 'static + Render<V>, ParentV: 'static> Element<ParentV> for View<V> {
    type State = Option<AnyElement<V>>;

    fn layout(
        &mut self,
        _parent_view: &mut ParentV,
        _state: Option<Self::State>,
        cx: &mut ViewContext<ParentV>,
    ) -> (LayoutId, Self::State) {
        self.update(cx, |view, cx| {
            let mut element = view.render(cx).into_any();
            let layout_id = element.layout(view, cx);
            (layout_id, Some(element))
        })
    }

    fn paint(
        self,
        _: Bounds<Pixels>,
        _parent: &mut ParentV,
        element: &mut Self::State,
        cx: &mut ViewContext<ParentV>,
    ) {
        self.update(cx, |view, cx| {
            element.take().unwrap().paint(view, cx);
        });
    }
}

impl<V: 'static + Render<V>, ParentV: 'static> RenderOnce<ParentV> for View<V> {
    type Element = View<V>;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.model.entity_id.into())
    }

    fn render_once(self) -> Self::Element {
        self
    }
}

impl<V: 'static> Element<V> for AnyView {
    type State = Option<Box<dyn Any>>;

    fn layout(
        &mut self,
        _view_state: &mut V,
        _element_state: Option<Self::State>,
        cx: &mut ViewContext<V>,
    ) -> (LayoutId, Self::State) {
        let (layout_id, rendered_element) = (self.layout)(self, cx);
        (layout_id, Some(rendered_element))
    }

    fn paint(
        mut self,
        _bounds: Bounds<Pixels>,
        _view_state: &mut V,
        rendered_element: &mut Self::State,
        cx: &mut ViewContext<V>,
    ) {
        (self.paint)(&mut self, rendered_element.take().unwrap(), cx)
    }
}

impl<ParentV: 'static> RenderOnce<ParentV> for AnyView {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        Some(self.model.entity_id.into())
    }

    fn render_once(self) -> Self::Element {
        self
    }
}

pub struct AnyWeakView {
    model: AnyWeakModel,
    layout: fn(&AnyView, &mut WindowContext) -> (LayoutId, Box<dyn Any>),
    paint: fn(&AnyView, Box<dyn Any>, &mut WindowContext),
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

impl<V: 'static + Render<V>> From<WeakView<V>> for AnyWeakView {
    fn from(view: WeakView<V>) -> Self {
        Self {
            model: view.model.into(),
            layout: any_view::layout::<V>,
            paint: any_view::paint::<V>,
        }
    }
}

pub struct RenderViewWith<E, V> {
    view: View<V>,
    element: Option<E>,
}

impl<E, ParentV, V> Element<ParentV> for RenderViewWith<E, V>
where
    E: 'static + Element<V>,
    ParentV: 'static,
    V: 'static,
{
    type State = Option<AnyElement<V>>;

    fn layout(
        &mut self,
        _: &mut ParentV,
        _: Option<Self::State>,
        cx: &mut ViewContext<ParentV>,
    ) -> (LayoutId, Self::State) {
        self.view.update(cx, |view, cx| {
            let mut element = self.element.take().unwrap().into_any();
            let layout_id = element.layout(view, cx);
            (layout_id, Some(element))
        })
    }

    fn paint(
        self,
        _: Bounds<Pixels>,
        _: &mut ParentV,
        element: &mut Self::State,
        cx: &mut ViewContext<ParentV>,
    ) {
        self.view
            .update(cx, |view, cx| element.take().unwrap().paint(view, cx))
    }
}

impl<E, V, ParentV> RenderOnce<ParentV> for RenderViewWith<E, V>
where
    E: 'static + Element<V>,
    V: 'static,
    ParentV: 'static,
{
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        self.element.as_ref().unwrap().element_id()
    }

    fn render_once(self) -> Self::Element {
        self
    }
}

mod any_view {
    use crate::{AnyElement, AnyView, BorrowWindow, LayoutId, Render, WindowContext};
    use std::any::Any;

    pub(crate) fn layout<V: 'static + Render<V>>(
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

    pub(crate) fn paint<V: 'static + Render<V>>(
        view: &AnyView,
        element: Box<dyn Any>,
        cx: &mut WindowContext,
    ) {
        cx.with_element_id(Some(view.model.entity_id), |cx| {
            let view = view.clone().downcast::<V>().unwrap();
            let element = element.downcast::<AnyElement<V>>().unwrap();
            view.update(cx, |view, cx| element.paint(view, cx))
        })
    }
}
