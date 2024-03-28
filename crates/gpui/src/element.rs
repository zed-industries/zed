//! Elements are the workhorses of GPUI. They are responsible for laying out and painting all of
//! the contents of a window. Elements form a tree and are laid out according to the web layout
//! standards as implemented by [taffy](https://github.com/DioxusLabs/taffy). Most of the time,
//! you won't need to interact with this module or these APIs directly. Elements provide their
//! own APIs and GPUI, or other element implementation, uses the APIs in this module to convert
//! that element tree into the pixels you see on the screen.
//!
//! # Element Basics
//!
//! Elements are constructed by calling [`Render::render()`] on the root view of the window, which
//! which recursively constructs the element tree from the current state of the application,.
//! These elements are then laid out by Taffy, and painted to the screen according to their own
//! implementation of [`Element::paint()`]. Before the start of the next frame, the entire element
//! tree and any callbacks they have registered with GPUI are dropped and the process repeats.
//!
//! But some state is too simple and voluminous to store in every view that needs it, e.g.
//! whether a hover has been started or not. For this, GPUI provides the [`Element::State`], associated type.
//!
//! # Implementing your own elements
//!
//! Elements are intended to be the low level, imperative API to GPUI. They are responsible for upholding,
//! or breaking, GPUI's features as they deem necessary. As an example, most GPUI elements are expected
//! to stay in the bounds that their parent element gives them. But with [`WindowContext::break_content_mask`],
//! you can ignore this restriction and paint anywhere inside of the window's bounds. This is useful for overlays
//! and popups and anything else that shows up 'on top' of other elements.
//! With great power, comes great responsibility.
//!
//! However, most of the time, you won't need to implement your own elements. GPUI provides a number of
//! elements that should cover most common use cases out of the box and it's recommended that you use those
//! to construct `components`, using the [`RenderOnce`] trait and the `#[derive(IntoElement)]` macro. Only implement
//! elements when you need to take manual control of the layout and painting process, such as when using
//! your own custom layout algorithm or rendering a code editor.

use crate::{
    util::FluentBuilder, ArenaBox, AvailableSpace, Bounds, DispatchNodeId, ElementContext,
    ElementId, LayoutId, Pixels, Point, Size, ViewContext, WindowContext, ELEMENT_ARENA,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{any::Any, fmt::Debug, mem, ops::DerefMut};

/// Implemented by types that participate in laying out and painting the contents of a window.
/// Elements form a tree and are laid out according to web-based layout rules, as implemented by Taffy.
/// You can create custom elements by implementing this trait, see the module-level documentation
/// for more details.
pub trait Element: 'static + IntoElement {
    /// The type of state returned from [`Element::before_layout`]. A mutable reference to this state is subsequently
    /// provided to [`Element::after_layout`] and [`Element::paint`].
    type BeforeLayout: 'static;

    /// The type of state returned from [`Element::after_layout`]. A mutable reference to this state is subsequently
    /// provided to [`Element::paint`].
    type AfterLayout: 'static;

    /// Before an element can be painted, we need to know where it's going to be and how big it is.
    /// Use this method to request a layout from Taffy and initialize the element's state.
    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout);

    /// After laying out an element, we need to commit its bounds to the current frame for hitbox
    /// purposes. The state argument is the same state that was returned from [`Element::before_layout()`].
    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Self::AfterLayout;

    /// Once layout has been completed, this method will be called to paint the element to the screen.
    /// The state argument is the same state that was returned from [`Element::before_layout()`].
    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        before_layout: &mut Self::BeforeLayout,
        after_layout: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    );

    /// Convert this element into a dynamically-typed [`AnyElement`].
    fn into_any(self) -> AnyElement {
        AnyElement::new(self)
    }
}

/// Implemented by any type that can be converted into an element.
pub trait IntoElement: Sized {
    /// The specific type of element into which the implementing type is converted.
    /// Useful for converting other types into elements automatically, like Strings
    type Element: Element;

    /// Convert self into a type that implements [`Element`].
    fn into_element(self) -> Self::Element;

    /// Convert self into a dynamically-typed [`AnyElement`].
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl<T: IntoElement> FluentBuilder for T {}

/// An object that can be drawn to the screen. This is the trait that distinguishes `Views` from
/// models. Views are drawn to the screen and care about the current window's state, models are not and do not.
pub trait Render: 'static + Sized {
    /// Render this view into an element tree.
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement;
}

impl Render for Empty {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Empty
    }
}

/// You can derive [`IntoElement`] on any type that implements this trait.
/// It is used to construct reusable `components` out of plain data. Think of
/// components as a recipe for a certain pattern of elements. RenderOnce allows
/// you to invoke this pattern, without breaking the fluent builder pattern of
/// the element APIs.
pub trait RenderOnce: 'static {
    /// Render this component into an element tree. Note that this method
    /// takes ownership of self, as compared to [`Render::render()`] method
    /// which takes a mutable reference.
    fn render(self, cx: &mut WindowContext) -> impl IntoElement;
}

/// This is a helper trait to provide a uniform interface for constructing elements that
/// can accept any number of any kind of child elements
pub trait ParentElement {
    /// Extend this element's children with the given child elements.
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>);

    /// Add a single child element to this element.
    fn child(mut self, child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.extend(std::iter::once(child.into_element().into_any()));
        self
    }

    /// Add multiple child elements to this element.
    fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

/// An element for rendering components. An implementation detail of the [`IntoElement`] derive macro
/// for [`RenderOnce`]
#[doc(hidden)]
pub struct Component<C: RenderOnce>(Option<C>);

impl<C: RenderOnce> Component<C> {
    /// Create a new component from the given RenderOnce type.
    pub fn new(component: C) -> Self {
        Component(Some(component))
    }
}

impl<C: RenderOnce> Element for Component<C> {
    type BeforeLayout = AnyElement;
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let mut element = self
            .0
            .take()
            .unwrap()
            .render(cx.deref_mut())
            .into_any_element();
        let layout_id = element.before_layout(cx);
        (layout_id, element)
    }

    fn after_layout(
        &mut self,
        _: Bounds<Pixels>,
        element: &mut AnyElement,
        cx: &mut ElementContext,
    ) {
        element.after_layout(cx);
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        element: &mut Self::BeforeLayout,
        _: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        element.paint(cx)
    }
}

impl<C: RenderOnce> IntoElement for Component<C> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A globally unique identifier for an element, used to track state across frames.
#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct GlobalElementId(SmallVec<[ElementId; 32]>);

trait ElementObject {
    fn inner_element(&mut self) -> &mut dyn Any;

    fn before_layout(&mut self, cx: &mut ElementContext) -> LayoutId;

    fn after_layout(&mut self, cx: &mut ElementContext);

    fn paint(&mut self, cx: &mut ElementContext);

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels>;
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
pub struct Drawable<E: Element> {
    /// The drawn element.
    pub element: E,
    phase: ElementDrawPhase<E::BeforeLayout, E::AfterLayout>,
}

#[derive(Default)]
enum ElementDrawPhase<BeforeLayout, AfterLayout> {
    #[default]
    Start,
    BeforeLayout {
        layout_id: LayoutId,
        before_layout: BeforeLayout,
    },
    LayoutComputed {
        layout_id: LayoutId,
        available_space: Size<AvailableSpace>,
        before_layout: BeforeLayout,
    },
    AfterLayout {
        node_id: DispatchNodeId,
        bounds: Bounds<Pixels>,
        before_layout: BeforeLayout,
        after_layout: AfterLayout,
    },
    Painted,
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
impl<E: Element> Drawable<E> {
    fn new(element: E) -> Self {
        Drawable {
            element,
            phase: ElementDrawPhase::Start,
        }
    }

    fn before_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Start => {
                let (layout_id, before_layout) = self.element.before_layout(cx);
                self.phase = ElementDrawPhase::BeforeLayout {
                    layout_id,
                    before_layout,
                };
                layout_id
            }
            _ => panic!("must call before_layout only once"),
        }
    }

    fn after_layout(&mut self, cx: &mut ElementContext) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::BeforeLayout {
                layout_id,
                mut before_layout,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                mut before_layout,
                ..
            } => {
                let bounds = cx.layout_bounds(layout_id);
                let node_id = cx.window.next_frame.dispatch_tree.push_node();
                let after_layout = self.element.after_layout(bounds, &mut before_layout, cx);
                self.phase = ElementDrawPhase::AfterLayout {
                    node_id,
                    bounds,
                    before_layout,
                    after_layout,
                };
                cx.window.next_frame.dispatch_tree.pop_node();
            }
            _ => panic!("must call before_layout before after_layout"),
        }
    }

    fn paint(&mut self, cx: &mut ElementContext) -> E::BeforeLayout {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::AfterLayout {
                node_id,
                bounds,
                mut before_layout,
                mut after_layout,
                ..
            } => {
                cx.window.next_frame.dispatch_tree.set_active_node(node_id);
                self.element
                    .paint(bounds, &mut before_layout, &mut after_layout, cx);
                self.phase = ElementDrawPhase::Painted;
                before_layout
            }
            _ => panic!("must call after_layout before paint"),
        }
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.before_layout(cx);
        }

        let layout_id = match mem::take(&mut self.phase) {
            ElementDrawPhase::BeforeLayout {
                layout_id,
                before_layout,
            } => {
                cx.compute_layout(layout_id, available_space);
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    before_layout,
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                available_space: prev_available_space,
                before_layout,
            } => {
                if available_space != prev_available_space {
                    cx.compute_layout(layout_id, available_space);
                }
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    before_layout,
                };
                layout_id
            }
            _ => panic!("cannot measure after painting"),
        };

        cx.layout_bounds(layout_id).size
    }
}

impl<E> ElementObject for Drawable<E>
where
    E: Element,
    E::BeforeLayout: 'static,
{
    fn inner_element(&mut self) -> &mut dyn Any {
        &mut self.element
    }

    fn before_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        Drawable::before_layout(self, cx)
    }

    fn after_layout(&mut self, cx: &mut ElementContext) {
        Drawable::after_layout(self, cx);
    }

    fn paint(&mut self, cx: &mut ElementContext) {
        Drawable::paint(self, cx);
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        Drawable::measure(self, available_space, cx)
    }
}

/// A dynamically typed element that can be used to store any element type.
pub struct AnyElement(ArenaBox<dyn ElementObject>);

impl AnyElement {
    pub(crate) fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::BeforeLayout: Any,
    {
        let element = ELEMENT_ARENA
            .with_borrow_mut(|arena| arena.alloc(|| Drawable::new(element)))
            .map(|element| element as &mut dyn ElementObject);
        AnyElement(element)
    }

    /// Attempt to downcast a reference to the boxed element to a specific type.
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.inner_element().downcast_mut::<T>()
    }

    /// Request the layout ID of the element stored in this `AnyElement`.
    /// Used for laying out child elements in a parent element.
    pub fn before_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        self.0.before_layout(cx)
    }

    /// Commits the element bounds of this [AnyElement] for hitbox purposes.
    pub fn after_layout(&mut self, cx: &mut ElementContext) {
        self.0.after_layout(cx)
    }

    /// Paints the element stored in this `AnyElement`.
    pub fn paint(&mut self, cx: &mut ElementContext) {
        self.0.paint(cx)
    }

    /// Initializes this element and performs layout within the given available space to determine its size.
    pub fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        self.0.measure(available_space, cx)
    }

    /// Initializes this element, performs layout if needed and commits its bounds for hitbox purposes.
    pub fn layout(
        &mut self,
        absolute_offset: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        let size = self.measure(available_space, cx);
        cx.with_absolute_element_offset(absolute_offset, |cx| self.after_layout(cx));
        size
    }
}

impl Element for AnyElement {
    type BeforeLayout = ();
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let layout_id = self.before_layout(cx);
        (layout_id, ())
    }

    fn after_layout(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) {
        self.after_layout(cx)
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut Self::BeforeLayout,
        _: &mut Self::AfterLayout,
        cx: &mut ElementContext,
    ) {
        self.paint(cx)
    }
}

impl IntoElement for AnyElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }

    fn into_any_element(self) -> AnyElement {
        self
    }
}

/// The empty element, which renders nothing.
pub struct Empty;

impl IntoElement for Empty {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Empty {
    type BeforeLayout = ();
    type AfterLayout = ();

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        (cx.request_layout(&crate::Style::default(), None), ())
    }

    fn after_layout(
        &mut self,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::BeforeLayout,
        _cx: &mut ElementContext,
    ) {
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        _after_layout: &mut Self::AfterLayout,
        _cx: &mut ElementContext,
    ) {
    }
}
