use crate::{
    private::Sealed, AnyBox, AnyElement, AnyModel, AnyWeakModel, AppContext, AvailableSpace,
    BorrowWindow, Bounds, Component, Element, ElementId, Entity, EntityId, Flatten, LayoutId,
    Model, Pixels, Size, ViewContext, VisualContext, WeakModel, WindowContext,
};
use anyhow::{Context, Result};
use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

pub trait Render: 'static + Sized {
    type Element: Element<Self> + 'static + Send;

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
        f: impl FnOnce(&mut V, &mut C::ViewContext<'_, V>) -> R,
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
        Entity::upgrade_from(self)
    }

    pub fn update<C, R>(
        &self,
        cx: &mut C,
        f: impl FnOnce(&mut V, &mut C::ViewContext<'_, V>) -> R,
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

    fn id(&self) -> Option<ElementId> {
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
    fn entity_type(&self) -> TypeId;
    fn entity_id(&self) -> EntityId;
    fn model(&self) -> AnyModel;
    fn initialize(&self, cx: &mut WindowContext) -> AnyBox;
    fn layout(&self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId;
    fn paint(&self, bounds: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext);
    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<V> ViewObject for View<V>
where
    V: Render,
{
    fn entity_type(&self) -> TypeId {
        TypeId::of::<V>()
    }

    fn entity_id(&self) -> EntityId {
        Entity::entity_id(self)
    }

    fn model(&self) -> AnyModel {
        self.model.clone().into_any()
    }

    fn initialize(&self, cx: &mut WindowContext) -> AnyBox {
        cx.with_element_id(ViewObject::entity_id(self), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let mut any_element = Box::new(AnyElement::new(state.render(cx)));
                any_element.initialize(state, cx);
                any_element
            })
        })
    }

    fn layout(&self, element: &mut AnyBox, cx: &mut WindowContext) -> LayoutId {
        cx.with_element_id(ViewObject::entity_id(self), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.layout(state, cx)
            })
        })
    }

    fn paint(&self, _: Bounds<Pixels>, element: &mut AnyBox, cx: &mut WindowContext) {
        cx.with_element_id(ViewObject::entity_id(self), |_global_id, cx| {
            self.update(cx, |state, cx| {
                let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                element.paint(state, cx);
            });
        });
    }

    fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("AnyView<{}>", std::any::type_name::<V>()))
            .field("entity_id", &ViewObject::entity_id(self).as_u64())
            .finish()
    }
}

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
            initialize: |view, cx| {
                cx.with_element_id(view.model.entity_id, |_, cx| {
                    let view = view.clone().downcast::<V>().unwrap();
                    Box::new(AnyElement::new(
                        view.update(cx, |view, cx| Render::render(view, cx)),
                    ))
                })
            },
            layout: |view, element, cx| {
                cx.with_element_id(view.model.entity_id, |_, cx| {
                    let view = view.clone().downcast::<V>().unwrap();
                    let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                    view.update(cx, |view, cx| element.layout(view, cx))
                })
            },
            paint: |view, element, cx| {
                cx.with_element_id(view.model.entity_id, |_, cx| {
                    let view = view.clone().downcast::<V>().unwrap();
                    let element = element.downcast_mut::<AnyElement<V>>().unwrap();
                    view.update(cx, |view, cx| element.paint(view, cx))
                })
            },
        }
    }
}

impl<ParentViewState: 'static> Element<ParentViewState> for AnyView {
    type ElementState = AnyBox;

    fn id(&self) -> Option<ElementId> {
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
