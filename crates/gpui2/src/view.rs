use crate::{
    private::Sealed, AnyBox, AnyElement, AnyModel, AnyWeakModel, AppContext, AvailableSpace,
    Bounds, Component, Element, ElementId, Entity, EntityId, Flatten, LayoutId, Model, Pixels,
    Size, ViewContext, VisualContext, WeakModel, WindowContext,
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
    initialize: fn(&AnyView, &mut WindowContext) -> AnyBox,
    layout: fn(&AnyView, &mut AnyBox, &mut WindowContext) -> LayoutId,
    paint: fn(&AnyView, &mut AnyBox, &mut WindowContext),
}

impl AnyView {
    pub fn downgrade(&self) -> AnyWeakView {
        AnyWeakView {
            model: self.model.downgrade(),
            initialize: self.initialize,
            layout: self.layout,
            paint: self.paint,
        }
    }

    pub fn downcast<T: 'static>(self) -> Result<View<T>, Self> {
        match self.model.downcast() {
            Ok(model) => Ok(View { model }),
            Err(model) => Err(Self {
                model,
                initialize: self.initialize,
                layout: self.layout,
                paint: self.paint,
            }),
        }
    }

    pub fn entity_type(&self) -> TypeId {
        self.model.entity_type
    }

    pub(crate) fn draw(&self, available_space: Size<AvailableSpace>, cx: &mut WindowContext) {
        let mut rendered_element = (self.initialize)(self, cx);
        let layout_id = (self.layout)(self, &mut rendered_element, cx);
        cx.window
            .layout_engine
            .compute_layout(layout_id, available_space);
        (self.paint)(self, &mut rendered_element, cx);
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
            initialize: any_view::initialize::<V>,
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

    fn initialize(
        &mut self,
        _view_state: &mut ParentViewState,
        _element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<ParentViewState>,
    ) -> Self::ElementState {
        (self.initialize)(self, cx)
    }

    fn layout(
        &mut self,
        _view_state: &mut ParentViewState,
        rendered_element: &mut Self::ElementState,
        cx: &mut ViewContext<ParentViewState>,
    ) -> LayoutId {
        (self.layout)(self, rendered_element, cx)
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
    initialize: fn(&AnyView, &mut WindowContext) -> AnyBox,
    layout: fn(&AnyView, &mut AnyBox, &mut WindowContext) -> LayoutId,
    paint: fn(&AnyView, &mut AnyBox, &mut WindowContext),
}

impl AnyWeakView {
    pub fn upgrade(&self) -> Option<AnyView> {
        let model = self.model.upgrade()?;
        Some(AnyView {
            model,
            initialize: self.initialize,
            layout: self.layout,
            paint: self.paint,
        })
    }
}

impl<V: Render> From<WeakView<V>> for AnyWeakView {
    fn from(view: WeakView<V>) -> Self {
        Self {
            model: view.model.into(),
            initialize: any_view::initialize::<V>,
            layout: any_view::layout::<V>,
            paint: any_view::paint::<V>,
        }
    }
}

impl<T, E> Render for T
where
    T: 'static + FnMut(&mut WindowContext) -> E,
    E: 'static + Send + Element<T>,
{
    type Element = E;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        (self)(cx)
    }
}

mod any_view {
    use crate::{AnyElement, AnyView, BorrowWindow, LayoutId, Render, WindowContext};
    use std::any::Any;

    pub(crate) fn initialize<V: Render>(view: &AnyView, cx: &mut WindowContext) -> Box<dyn Any> {
        cx.with_element_id(Some(view.model.entity_id), |cx| {
            let view = view.clone().downcast::<V>().unwrap();
            let element = view.update(cx, |view, cx| {
                let mut element = AnyElement::new(view.render(cx));
                element.initialize(view, cx);
                element
            });
            Box::new(element)
        })
    }

    pub(crate) fn layout<V: Render>(
        view: &AnyView,
        element: &mut Box<dyn Any>,
        cx: &mut WindowContext,
    ) -> LayoutId {
        cx.with_element_id(Some(view.model.entity_id), |cx| {
            let view = view.clone().downcast::<V>().unwrap();
            let element = element.downcast_mut::<AnyElement<V>>().unwrap();
            view.update(cx, |view, cx| element.layout(view, cx))
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
