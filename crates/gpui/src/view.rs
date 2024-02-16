use crate::{
    seal::Sealed, AnyElement, AnyModel, AnyWeakModel, AppContext, AvailableSpace, Bounds,
    ContentMask, Element, ElementContext, ElementId, Entity, EntityId, Flatten, FocusHandle,
    FocusableView, IntoElement, LayoutId, Model, Pixels, Point, Render, Size, StackingOrder, Style,
    TextStyle, ViewContext, VisualContext, WeakModel,
};
use anyhow::{Context, Result};
use std::{
    any::{type_name, TypeId},
    fmt,
    hash::{Hash, Hasher},
};

/// A view is a piece of state that can be presented on screen by implementing the [Render] trait.
/// Views implement [Element] and can composed with other views, and every window is created with a root view.
pub struct View<V> {
    /// A view is just a [Model] whose type implements `Render`, and the model is accessible via this field.
    pub model: Model<V>,
}

impl<V> Sealed for View<V> {}

#[doc(hidden)]
pub struct AnyViewState {
    root_style: Style,
    next_stacking_order_id: u16,
    cache_key: Option<ViewCacheKey>,
    element: Option<AnyElement>,
}

struct ViewCacheKey {
    bounds: Bounds<Pixels>,
    stacking_order: StackingOrder,
    content_mask: ContentMask<Pixels>,
    text_style: TextStyle,
}

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

    /// Updates the view's state with the given function, which is passed a mutable reference and a context.
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

    /// Obtain a read-only reference to this view's state.
    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a V {
        self.model.read(cx)
    }

    /// Gets a [FocusHandle] for this view when its state implements [FocusableView].
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
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        cx.with_view_id(self.entity_id(), |cx| {
            let mut element = self.update(cx, |view, cx| view.render(cx).into_any_element());
            let layout_id = element.request_layout(cx);
            (layout_id, Some(element))
        })
    }

    fn paint(&mut self, _: Bounds<Pixels>, element: &mut Self::State, cx: &mut ElementContext) {
        cx.paint_view(self.entity_id(), |cx| element.take().unwrap().paint(cx));
    }
}

impl<V> Clone for View<V> {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}

impl<T> std::fmt::Debug for View<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&format!("View<{}>", type_name::<T>()))
            .field("entity_id", &self.model.entity_id)
            .finish_non_exhaustive()
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

/// A weak variant of [View] which does not prevent the view from being released.
pub struct WeakView<V> {
    pub(crate) model: WeakModel<V>,
}

impl<V: 'static> WeakView<V> {
    /// Gets the entity id associated with this handle.
    pub fn entity_id(&self) -> EntityId {
        self.model.entity_id
    }

    /// Obtain a strong handle for the view if it hasn't been released.
    pub fn upgrade(&self) -> Option<View<V>> {
        Entity::upgrade_from(self)
    }

    /// Updates this view's state if it hasn't been released.
    /// Returns an error if this view has been released.
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

    /// Assert that the view referenced by this handle has been released.
    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_released(&self) {
        self.model.assert_released()
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

/// A dynamically-typed handle to a view, which can be downcast to a [View] for a specific type.
#[derive(Clone, Debug)]
pub struct AnyView {
    model: AnyModel,
    request_layout: fn(&AnyView, &mut ElementContext) -> (LayoutId, AnyElement),
    cache: bool,
}

impl AnyView {
    /// Indicate that this view should be cached when using it as an element.
    /// When using this method, the view's previous layout and paint will be recycled from the previous frame if [ViewContext::notify] has not been called since it was rendered.
    /// The one exception is when [WindowContext::refresh] is called, in which case caching is ignored.
    pub fn cached(mut self) -> Self {
        self.cache = true;
        self
    }

    /// Convert this to a weak handle.
    pub fn downgrade(&self) -> AnyWeakView {
        AnyWeakView {
            model: self.model.downgrade(),
            layout: self.request_layout,
        }
    }

    /// Convert this to a [View] of a specific type.
    /// If this handle does not contain a view of the specified type, returns itself in an `Err` variant.
    pub fn downcast<T: 'static>(self) -> Result<View<T>, Self> {
        match self.model.downcast() {
            Ok(model) => Ok(View { model }),
            Err(model) => Err(Self {
                model,
                request_layout: self.request_layout,
                cache: self.cache,
            }),
        }
    }

    /// Gets the [TypeId] of the underlying view.
    pub fn entity_type(&self) -> TypeId {
        self.model.entity_type
    }

    /// Gets the entity id of this handle.
    pub fn entity_id(&self) -> EntityId {
        self.model.entity_id()
    }

    pub(crate) fn draw(
        &self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) {
        cx.paint_view(self.entity_id(), |cx| {
            cx.with_absolute_element_offset(origin, |cx| {
                let (layout_id, mut rendered_element) = (self.request_layout)(self, cx);
                cx.compute_layout(layout_id, available_space);
                rendered_element.paint(cx)
            });
        })
    }
}

impl<V: Render> From<View<V>> for AnyView {
    fn from(value: View<V>) -> Self {
        AnyView {
            model: value.model.into_any(),
            request_layout: any_view::request_layout::<V>,
            cache: false,
        }
    }
}

impl Element for AnyView {
    type State = AnyViewState;

    fn request_layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        cx.with_view_id(self.entity_id(), |cx| {
            if self.cache
                && !cx.window.dirty_views.contains(&self.entity_id())
                && !cx.window.refreshing
            {
                if let Some(state) = state {
                    let layout_id = cx.request_layout(&state.root_style, None);
                    return (layout_id, state);
                }
            }

            let (layout_id, element) = (self.request_layout)(self, cx);
            let root_style = cx.layout_style(layout_id).unwrap().clone();
            let state = AnyViewState {
                root_style,
                next_stacking_order_id: 0,
                cache_key: None,
                element: Some(element),
            };
            (layout_id, state)
        })
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut ElementContext) {
        cx.paint_view(self.entity_id(), |cx| {
            if !self.cache {
                state.element.take().unwrap().paint(cx);
                return;
            }

            if let Some(cache_key) = state.cache_key.as_mut() {
                if cache_key.bounds == bounds
                    && cache_key.content_mask == cx.content_mask()
                    && cache_key.stacking_order == *cx.stacking_order()
                    && cache_key.text_style == cx.text_style()
                {
                    cx.reuse_view(state.next_stacking_order_id);
                    return;
                }
            }

            if let Some(mut element) = state.element.take() {
                element.paint(cx);
            } else {
                let mut element = (self.request_layout)(self, cx).1;
                element.draw(bounds.origin, bounds.size.into(), cx);
            }

            state.next_stacking_order_id = cx
                .window
                .next_frame
                .next_stacking_order_ids
                .last()
                .copied()
                .unwrap();
            state.cache_key = Some(ViewCacheKey {
                bounds,
                stacking_order: cx.stacking_order().clone(),
                content_mask: cx.content_mask(),
                text_style: cx.text_style(),
            });
        })
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

/// A weak, dynamically-typed view handle that does not prevent the view from being released.
pub struct AnyWeakView {
    model: AnyWeakModel,
    layout: fn(&AnyView, &mut ElementContext) -> (LayoutId, AnyElement),
}

impl AnyWeakView {
    /// Convert to a strongly-typed handle if the referenced view has not yet been released.
    pub fn upgrade(&self) -> Option<AnyView> {
        let model = self.model.upgrade()?;
        Some(AnyView {
            model,
            request_layout: self.layout,
            cache: false,
        })
    }
}

impl<V: 'static + Render> From<WeakView<V>> for AnyWeakView {
    fn from(view: WeakView<V>) -> Self {
        Self {
            model: view.model.into(),
            layout: any_view::request_layout::<V>,
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
    use crate::{AnyElement, AnyView, ElementContext, IntoElement, LayoutId, Render};

    pub(crate) fn request_layout<V: 'static + Render>(
        view: &AnyView,
        cx: &mut ElementContext,
    ) -> (LayoutId, AnyElement) {
        let view = view.clone().downcast::<V>().unwrap();
        let mut element = view.update(cx, |view, cx| view.render(cx).into_any_element());
        let layout_id = element.request_layout(cx);
        (layout_id, element)
    }
}
