use crate::Empty;
use crate::{
    seal::Sealed, AnyElement, AnyModel, AnyWeakModel, AppContext, Bounds, ContentMask, Element,
    ElementId, Entity, EntityId, Flatten, FocusHandle, FocusableView, GlobalElementId, IntoElement,
    LayoutId, Model, PaintIndex, Pixels, PrepaintStateIndex, Render, Style, StyleRefinement,
    TextStyle, ViewContext, VisualContext, WeakModel, WindowContext,
};
use anyhow::{Context, Result};
use refineable::Refineable;
use std::{
    any::{type_name, TypeId},
    fmt,
    hash::{Hash, Hasher},
    ops::Range,
};

/// A view is a piece of state that can be presented on screen by implementing the [Render] trait.
/// Views implement [Element] and can composed with other views, and every window is created with a root view.
pub struct View<V> {
    /// A view is just a [Model] whose type implements `Render`, and the model is accessible via this field.
    pub model: Model<V>,
}

impl<V> Sealed for View<V> {}

struct AnyViewState {
    prepaint_range: Range<PrepaintStateIndex>,
    paint_range: Range<PaintIndex>,
    cache_key: ViewCacheKey,
}

#[derive(Default)]
struct ViewCacheKey {
    bounds: Bounds<Pixels>,
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
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::View(self.entity_id()))
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut element = self.update(cx, |view, cx| view.render(cx).into_any_element());
        let layout_id = element.request_layout(cx);
        (layout_id, element)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) {
        cx.set_view_id(self.entity_id());
        element.prepaint(cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        element.paint(cx);
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
    render: fn(&AnyView, &mut WindowContext) -> AnyElement,
    cached_style: Option<StyleRefinement>,
}

impl AnyView {
    /// Indicate that this view should be cached when using it as an element.
    /// When using this method, the view's previous layout and paint will be recycled from the previous frame if [ViewContext::notify] has not been called since it was rendered.
    /// The one exception is when [WindowContext::refresh] is called, in which case caching is ignored.
    pub fn cached(mut self, style: StyleRefinement) -> Self {
        self.cached_style = Some(style);
        self
    }

    /// Convert this to a weak handle.
    pub fn downgrade(&self) -> AnyWeakView {
        AnyWeakView {
            model: self.model.downgrade(),
            render: self.render,
        }
    }

    /// Convert this to a [View] of a specific type.
    /// If this handle does not contain a view of the specified type, returns itself in an `Err` variant.
    pub fn downcast<T: 'static>(self) -> Result<View<T>, Self> {
        match self.model.downcast() {
            Ok(model) => Ok(View { model }),
            Err(model) => Err(Self {
                model,
                render: self.render,
                cached_style: self.cached_style,
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
            render: any_view::render::<V>,
            cached_style: None,
        }
    }
}

impl PartialEq for AnyView {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model
    }
}

impl Eq for AnyView {}

impl Element for AnyView {
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = Option<AnyElement>;

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::View(self.entity_id()))
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        if let Some(style) = self.cached_style.as_ref() {
            let mut root_style = Style::default();
            root_style.refine(style);
            let layout_id = cx.request_layout(root_style, None);
            (layout_id, None)
        } else {
            let mut element = (self.render)(self, cx);
            let layout_id = element.request_layout(cx);
            (layout_id, Some(element))
        }
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Option<AnyElement> {
        cx.set_view_id(self.entity_id());
        if self.cached_style.is_some() {
            cx.with_element_state::<AnyViewState, _>(global_id.unwrap(), |element_state, cx| {
                let content_mask = cx.content_mask();
                let text_style = cx.text_style();

                if let Some(mut element_state) = element_state {
                    if element_state.cache_key.bounds == bounds
                        && element_state.cache_key.content_mask == content_mask
                        && element_state.cache_key.text_style == text_style
                        && !cx.window.dirty_views.contains(&self.entity_id())
                        && !cx.window.refreshing
                    {
                        let prepaint_start = cx.prepaint_index();
                        cx.reuse_prepaint(element_state.prepaint_range.clone());
                        let prepaint_end = cx.prepaint_index();
                        element_state.prepaint_range = prepaint_start..prepaint_end;
                        return (None, element_state);
                    }
                }

                let prepaint_start = cx.prepaint_index();
                let mut element = (self.render)(self, cx);
                element.layout_as_root(bounds.size.into(), cx);
                element.prepaint_at(bounds.origin, cx);
                let prepaint_end = cx.prepaint_index();

                (
                    Some(element),
                    AnyViewState {
                        prepaint_range: prepaint_start..prepaint_end,
                        paint_range: PaintIndex::default()..PaintIndex::default(),
                        cache_key: ViewCacheKey {
                            bounds,
                            content_mask,
                            text_style,
                        },
                    },
                )
            })
        } else {
            let mut element = element.take().unwrap();
            element.prepaint(cx);
            Some(element)
        }
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        element: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        if self.cached_style.is_some() {
            cx.with_element_state::<AnyViewState, _>(global_id.unwrap(), |element_state, cx| {
                let mut element_state = element_state.unwrap();

                let paint_start = cx.paint_index();

                if let Some(element) = element {
                    element.paint(cx);
                } else {
                    cx.reuse_paint(element_state.paint_range.clone());
                }

                let paint_end = cx.paint_index();
                element_state.paint_range = paint_start..paint_end;

                ((), element_state)
            })
        } else {
            element.as_mut().unwrap().paint(cx);
        }
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
    render: fn(&AnyView, &mut WindowContext) -> AnyElement,
}

impl AnyWeakView {
    /// Convert to a strongly-typed handle if the referenced view has not yet been released.
    pub fn upgrade(&self) -> Option<AnyView> {
        let model = self.model.upgrade()?;
        Some(AnyView {
            model,
            render: self.render,
            cached_style: None,
        })
    }
}

impl<V: 'static + Render> From<WeakView<V>> for AnyWeakView {
    fn from(view: WeakView<V>) -> Self {
        Self {
            model: view.model.into(),
            render: any_view::render::<V>,
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
    use crate::{AnyElement, AnyView, IntoElement, Render, WindowContext};

    pub(crate) fn render<V: 'static + Render>(
        view: &AnyView,
        cx: &mut WindowContext,
    ) -> AnyElement {
        let view = view.clone().downcast::<V>().unwrap();
        view.update(cx, |view, cx| view.render(cx).into_any_element())
    }
}

/// A view that renders nothing
pub struct EmptyView;

impl Render for EmptyView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Empty
    }
}
