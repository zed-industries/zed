use crate::{
    seal::Sealed, AnyElement, AnyModel, AnyWeakModel, AppContext, AvailableSpace, BorrowWindow,
    Bounds, ContentMask, Element, ElementId, Entity, EntityId, Flatten, FocusHandle, FocusableView,
    IntoElement, LayoutId, Model, Pixels, Point, Render, Size, StackingOrder, Style, TextStyle,
    ViewContext, VisualContext, WeakModel, WindowContext,
};
use anyhow::{Context, Result};
use std::{
    any::{type_name, TypeId},
    fmt,
    hash::{Hash, Hasher},
};

pub struct View<V> {
    pub model: Model<V>,
}

impl<V> Sealed for View<V> {}

pub struct AnyViewState {
    root_style: Style,
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
        cx.with_view_id(self.entity_id(), |cx| {
            let mut element = self.update(cx, |view, cx| view.render(cx).into_any_element());
            let layout_id = element.request_layout(cx);
            (layout_id, Some(element))
        })
    }

    fn paint(&mut self, _: Bounds<Pixels>, element: &mut Self::State, cx: &mut WindowContext) {
        cx.with_view_id(self.entity_id(), |cx| element.take().unwrap().paint(cx));
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
    request_layout: fn(&AnyView, &mut WindowContext) -> (LayoutId, AnyElement),
    cache: bool,
}

impl AnyView {
    pub fn cached(mut self) -> Self {
        self.cache = true;
        self
    }

    pub fn downgrade(&self) -> AnyWeakView {
        AnyWeakView {
            model: self.model.downgrade(),
            layout: self.request_layout,
        }
    }

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
        cx.with_view_id(self.entity_id(), |cx| {
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
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::State) {
        cx.with_view_id(self.entity_id(), |cx| {
            if self.cache {
                if let Some(state) = state {
                    let layout_id = cx.request_layout(&state.root_style, None);
                    return (layout_id, state);
                }
            }

            let (layout_id, element) = (self.request_layout)(self, cx);
            let root_style = cx.layout_style(layout_id).unwrap().clone();
            let state = AnyViewState {
                root_style,
                cache_key: None,
                element: Some(element),
            };
            (layout_id, state)
        })
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut WindowContext) {
        cx.with_view_id(self.entity_id(), |cx| {
            if !self.cache {
                state.element.take().unwrap().paint(cx);
                return;
            }

            if let Some(cache_key) = state.cache_key.as_mut() {
                if cache_key.bounds == bounds
                    && cache_key.content_mask == cx.content_mask()
                    && cache_key.stacking_order == *cx.stacking_order()
                    && cache_key.text_style == cx.text_style()
                    && !cx.window.dirty_views.contains(&self.entity_id())
                {
                    cx.reuse_geometry();
                    return;
                }
            }

            let mut element = state
                .element
                .take()
                .unwrap_or_else(|| (self.request_layout)(self, cx).1);
            element.draw(bounds.origin, bounds.size.into(), cx);

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

pub struct AnyWeakView {
    model: AnyWeakModel,
    layout: fn(&AnyView, &mut WindowContext) -> (LayoutId, AnyElement),
}

impl AnyWeakView {
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
    use crate::{AnyElement, AnyView, IntoElement, LayoutId, Render, WindowContext};

    pub(crate) fn request_layout<V: 'static + Render>(
        view: &AnyView,
        cx: &mut WindowContext,
    ) -> (LayoutId, AnyElement) {
        let view = view.clone().downcast::<V>().unwrap();
        let mut element = view.update(cx, |view, cx| view.render(cx).into_any_element());
        let layout_id = element.request_layout(cx);
        (layout_id, element)
    }
}
