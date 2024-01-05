use crate::{
    private::Sealed, AnyElement, AnyModel, AnyWeakModel, AppContext, AvailableSpace, BorrowWindow,
    Bounds, Element, ElementId, Entity, EntityId, Flatten, FocusHandle, FocusableView, IntoElement,
    LayoutId, Model, Pixels, Point, Render, Size, ViewContext, VisualContext, WeakModel,
    WindowContext,
};
use anyhow::{Context, Result};
use std::{
    any::TypeId,
    fmt,
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

    // pub fn render_with<E>(&self, component: E) -> RenderViewWith<E, V>
    // where
    //     E: 'static + Element,
    // {
    //     RenderViewWith {
    //         view: self.clone(),
    //         element: Some(component),
    //     }
    // }

    pub fn focus_handle(&self, cx: &AppContext) -> FocusHandle
    where
        V: FocusableView,
    {
        self.read(cx).focus_handle(cx)
    }
}

impl<V: Render> Element for View<V> {
    type State = Option<AnyElement>;

    fn request_layout(
        &mut self,
        _state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let mut element = self.update(cx, |view, cx| view.render(cx).into_any_element());
        let layout_id = element.request_layout(cx);
        (layout_id, Some(element))
    }

    fn paint(&mut self, _: Bounds<Pixels>, element: &mut Self::State, cx: &mut WindowContext) {
        element.take().unwrap().paint(cx);
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_dropped(&self) {
        self.model.assert_dropped()
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
    layout: fn(&AnyView, &mut WindowContext) -> (LayoutId, AnyElement),
    paint: fn(&AnyView, &mut AnyElement, &mut WindowContext),
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
            let (layout_id, mut rendered_element) = (self.layout)(self, cx);
            cx.compute_layout(layout_id, available_space);
            (self.paint)(self, &mut rendered_element, cx);
        })
    }
}

impl<V: Render> From<View<V>> for AnyView {
    fn from(value: View<V>) -> Self {
        AnyView {
            model: value.model.into_any(),
            layout: any_view::layout::<V>,
            paint: any_view::paint,
        }
    }
}

impl Element for AnyView {
    type State = Option<AnyElement>;

    fn request_layout(
        &mut self,
        _state: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        let (layout_id, state) = (self.layout)(self, cx);
        (layout_id, Some(state))
    }

    fn paint(&mut self, _: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        debug_assert!(
            state.is_some(),
            "state is None. Did you include an AnyView twice in the tree?"
        );
        (self.paint)(self, state.as_mut().unwrap(), cx)
    }
}

impl<V: 'static + Render> IntoElement for View<V> {
    type Element = View<V>;

    fn element_id(&self) -> Option<ElementId> {
        Some(ElementId::from_entity_id(self.model.entity_id))
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl IntoElement for AnyView {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        Some(ElementId::from_entity_id(self.model.entity_id))
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

pub struct AnyWeakView {
    model: AnyWeakModel,
    layout: fn(&AnyView, &mut WindowContext) -> (LayoutId, AnyElement),
    paint: fn(&AnyView, &mut AnyElement, &mut WindowContext),
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

impl<V: 'static + Render> From<WeakView<V>> for AnyWeakView {
    fn from(view: WeakView<V>) -> Self {
        Self {
            model: view.model.into(),
            layout: any_view::layout::<V>,
            paint: any_view::paint,
        }
    }
}

impl PartialEq for AnyWeakView {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model
    }
}

impl std::fmt::Debug for AnyWeakView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyWeakView")
            .field("entity_id", &self.model.entity_id)
            .finish_non_exhaustive()
    }
}

mod any_view {
    use crate::{AnyElement, AnyView, IntoElement, LayoutId, Render, WindowContext};

    pub(crate) fn layout<V: 'static + Render>(
        view: &AnyView,
        cx: &mut WindowContext,
    ) -> (LayoutId, AnyElement) {
        let view = view.clone().downcast::<V>().unwrap();
        let mut element = view.update(cx, |view, cx| view.render(cx).into_any_element());
        let layout_id = element.request_layout(cx);
        (layout_id, element)
    }

    pub(crate) fn paint(_view: &AnyView, element: &mut AnyElement, cx: &mut WindowContext) {
        element.paint(cx);
    }
}
