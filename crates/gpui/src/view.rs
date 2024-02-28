use crate::{
    seal::Sealed, AnyElement, AnyModel, AnyWeakModel, AppContext, Bounds, ContentMask, Element,
    ElementContext, ElementId, Entity, EntityId, Flatten, FocusHandle, FocusableView, IntoElement,
    LayoutId, Model, Pixels, Render, StackingOrder, Style, TextStyle, ViewContext, VisualContext,
    WeakModel,
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
#[derive(Default)]
pub struct AnyViewFrameState {
    element: Option<AnyElement>,
    root_style: Style,
}

struct AnyViewState {
    root_style: Style,
    cache_key: ViewCacheKey,
    next_stacking_order_id: u16,
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
    type BeforeLayout = AnyElement;
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        cx.with_view_id(self.entity_id(), |cx| {
            // TODO! Implement caching for these views as well?
            let mut element = self.update(cx, |view, cx| view.render(cx).into_any_element());
            let layout_id = element.before_layout(cx);
            (layout_id, element)
        })
    }

    fn after_layout(
        &mut self,
        _: Bounds<Pixels>,
        element: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) {
        cx.with_view_id(self.entity_id(), |cx| element.after_layout(cx));
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        element: &mut Self::BeforeLayout,
        _: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        cx.paint_view(self.entity_id(), |cx| element.paint(cx));
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
    before_layout: fn(&AnyView, &mut ElementContext) -> (LayoutId, AnyElement),
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
            layout: self.before_layout,
        }
    }

    /// Convert this to a [View] of a specific type.
    /// If this handle does not contain a view of the specified type, returns itself in an `Err` variant.
    pub fn downcast<T: 'static>(self) -> Result<View<T>, Self> {
        match self.model.downcast() {
            Ok(model) => Ok(View { model }),
            Err(model) => Err(Self {
                model,
                before_layout: self.before_layout,
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
}

impl<V: Render> From<View<V>> for AnyView {
    fn from(value: View<V>) -> Self {
        AnyView {
            model: value.model.into_any(),
            before_layout: any_view::before_layout::<V>,
            cache: false,
        }
    }
}

impl Element for AnyView {
    type BeforeLayout = AnyViewFrameState;
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        cx.with_view_id(self.entity_id(), |cx| {
            cx.with_element_state::<AnyViewState, _>(
                Some(ElementId::View(self.entity_id())),
                |element_state, cx| {
                    let mut element_state = element_state.unwrap();
                    let mut before_layout = AnyViewFrameState::default();

                    if self.cache
                        && !cx.window.dirty_views.contains(&self.entity_id())
                        && !cx.window.refreshing
                    {
                        if let Some(root_style) = element_state
                            .as_ref()
                            .map(|element_state| &element_state.root_style)
                        {
                            let layout_id = cx.before_layout(root_style, None);
                            return ((layout_id, before_layout), element_state);
                        }
                    }

                    let (layout_id, element) = (self.before_layout)(self, cx);
                    before_layout.element = Some(element);
                    before_layout.root_style = cx.layout_style(layout_id).unwrap().clone();

                    ((layout_id, before_layout), element_state)
                },
            )
        })
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) {
        cx.with_view_id(self.entity_id(), |cx| {
            cx.with_element_state::<AnyViewState, _>(
                Some(ElementId::View(self.entity_id())),
                |element_state, cx| {
                    let element_state = element_state.unwrap();

                    if !self.cache {
                        before_layout.element.as_mut().unwrap().after_layout(cx);
                        return ((), element_state);
                    }

                    if let Some(cache_key) = element_state
                        .as_ref()
                        .map(|element_state| &element_state.cache_key)
                    {
                        if cache_key.bounds == bounds
                            && cache_key.content_mask == cx.content_mask()
                            && cache_key.stacking_order == *cx.stacking_order()
                            && cache_key.text_style == cx.text_style()
                        {
                            todo!("reuse entries in the depth map");
                            return ((), element_state);
                        }
                    }

                    if let Some(element) = before_layout.element.as_mut() {
                        element.after_layout(cx);
                    } else {
                        let mut element = (self.before_layout)(self, cx).1;
                        element.after_layout(cx);
                        before_layout.element = Some(element);
                    }

                    (
                        (),
                        Some(AnyViewState {
                            root_style: before_layout.root_style.clone(),
                            cache_key: ViewCacheKey {
                                bounds,
                                stacking_order: cx.stacking_order().clone(),
                                content_mask: cx.content_mask(),
                                text_style: cx.text_style(),
                            },
                            next_stacking_order_id: cx
                                .window
                                .next_frame
                                .next_stacking_order_ids
                                .last()
                                .copied()
                                .unwrap(),
                        }),
                    )
                },
            )
        })
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        _: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        cx.paint_view(self.entity_id(), |cx| {
            cx.with_element_state::<AnyViewState, _>(
                Some(ElementId::View(self.entity_id())),
                |element_state, cx| {
                    let element_state = element_state.unwrap();
                    if let Some(element) = before_layout.element.as_mut() {
                        element.paint(cx);
                    } else {
                        let element_state = element_state.as_ref().unwrap();
                        cx.reuse_view(element_state.next_stacking_order_id);
                    }
                    ((), element_state)
                },
            )
        })
    }
}

impl<V: 'static + Render> IntoElement for View<V> {
    type Element = View<V>;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl IntoElement for AnyView {
    type Element = Self;

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
            before_layout: self.layout,
            cache: false,
        })
    }
}

impl<V: 'static + Render> From<WeakView<V>> for AnyWeakView {
    fn from(view: WeakView<V>) -> Self {
        Self {
            model: view.model.into(),
            layout: any_view::before_layout::<V>,
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

    pub(crate) fn before_layout<V: 'static + Render>(
        view: &AnyView,
        cx: &mut ElementContext,
    ) -> (LayoutId, AnyElement) {
        let view = view.clone().downcast::<V>().unwrap();
        let mut element = view.update(cx, |view, cx| view.render(cx).into_any_element());
        let layout_id = element.before_layout(cx);
        (layout_id, element)
    }
}
