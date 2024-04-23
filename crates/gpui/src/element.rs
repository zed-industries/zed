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
    /// The type of state returned from [`Element::request_layout`]. A mutable reference to this state is subsequently
    /// provided to [`Element::prepaint`] and [`Element::paint`].
    type RequestLayoutState: 'static;

    /// The type of state returned from [`Element::prepaint`]. A mutable reference to this state is subsequently
    /// provided to [`Element::paint`].
    type PrepaintState: 'static;

    /// Before an element can be painted, we need to know where it's going to be and how big it is.
    /// Use this method to request a layout from Taffy and initialize the element's state.
    fn request_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::RequestLayoutState);

    /// After laying out an element, we need to commit its bounds to the current frame for hitbox
    /// purposes. The state argument is the same state that was returned from [`Element::request_layout()`].
    fn prepaint(
        &mut self,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut ElementContext,
    ) -> Self::PrepaintState;

    /// Once layout has been completed, this method will be called to paint the element to the screen.
    /// The state argument is the same state that was returned from [`Element::request_layout()`].
    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
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
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn request_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::RequestLayoutState) {
        let mut element = self
            .0
            .take()
            .unwrap()
            .render(cx.deref_mut())
            .into_any_element();
        let layout_id = element.request_layout(cx);
        (layout_id, element)
    }

    fn prepaint(&mut self, _: Bounds<Pixels>, element: &mut AnyElement, cx: &mut ElementContext) {
        element.prepaint(cx);
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
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

    fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId;

    fn prepaint(&mut self, cx: &mut ElementContext);

    fn paint(&mut self, cx: &mut ElementContext);

    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels>;
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
pub struct Drawable<E: Element> {
    /// The drawn element.
    pub element: E,
    phase: ElementDrawPhase<E::RequestLayoutState, E::PrepaintState>,
}

#[derive(Default)]
enum ElementDrawPhase<RequestLayoutState, PrepaintState> {
    #[default]
    Start,
    RequestLayoutState {
        layout_id: LayoutId,
        request_layout: RequestLayoutState,
    },
    LayoutComputed {
        layout_id: LayoutId,
        available_space: Size<AvailableSpace>,
        request_layout: RequestLayoutState,
    },
    PrepaintState {
        node_id: DispatchNodeId,
        bounds: Bounds<Pixels>,
        request_layout: RequestLayoutState,
        prepaint: PrepaintState,
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

    fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Start => {
                let (layout_id, request_layout) = self.element.request_layout(cx);
                self.phase = ElementDrawPhase::RequestLayoutState {
                    layout_id,
                    request_layout,
                };
                layout_id
            }
            _ => panic!("must call request_layout only once"),
        }
    }

    fn prepaint(&mut self, cx: &mut ElementContext) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayoutState {
                layout_id,
                mut request_layout,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                mut request_layout,
                ..
            } => {
                let bounds = cx.layout_bounds(layout_id);
                let node_id = cx.window.next_frame.dispatch_tree.push_node();
                let prepaint = self.element.prepaint(bounds, &mut request_layout, cx);
                self.phase = ElementDrawPhase::PrepaintState {
                    node_id,
                    bounds,
                    request_layout,
                    prepaint,
                };
                cx.window.next_frame.dispatch_tree.pop_node();
            }
            _ => panic!("must call request_layout before prepaint"),
        }
    }

    fn paint(&mut self, cx: &mut ElementContext) -> E::RequestLayoutState {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::PrepaintState {
                node_id,
                bounds,
                mut request_layout,
                mut prepaint,
                ..
            } => {
                cx.window.next_frame.dispatch_tree.set_active_node(node_id);
                self.element
                    .paint(bounds, &mut request_layout, &mut prepaint, cx);
                self.phase = ElementDrawPhase::Painted;
                request_layout
            }
            _ => panic!("must call prepaint before paint"),
        }
    }

    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.request_layout(cx);
        }

        let layout_id = match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayoutState {
                layout_id,
                request_layout,
            } => {
                cx.compute_layout(layout_id, available_space);
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    request_layout,
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                available_space: prev_available_space,
                request_layout,
            } => {
                if available_space != prev_available_space {
                    cx.compute_layout(layout_id, available_space);
                }
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    request_layout,
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
    E::RequestLayoutState: 'static,
{
    fn inner_element(&mut self) -> &mut dyn Any {
        &mut self.element
    }

    fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        Drawable::request_layout(self, cx)
    }

    fn prepaint(&mut self, cx: &mut ElementContext) {
        Drawable::prepaint(self, cx);
    }

    fn paint(&mut self, cx: &mut ElementContext) {
        Drawable::paint(self, cx);
    }

    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        Drawable::layout_as_root(self, available_space, cx)
    }
}

/// A dynamically typed element that can be used to store any element type.
pub struct AnyElement(ArenaBox<dyn ElementObject>);

impl AnyElement {
    pub(crate) fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::RequestLayoutState: Any,
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
    pub fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        self.0.request_layout(cx)
    }

    /// Prepares the element to be painted by storing its bounds, giving it a chance to draw hitboxes and
    /// request autoscroll before the final paint pass is confirmed.
    pub fn prepaint(&mut self, cx: &mut ElementContext) {
        self.0.prepaint(cx)
    }

    /// Paints the element stored in this `AnyElement`.
    pub fn paint(&mut self, cx: &mut ElementContext) {
        self.0.paint(cx)
    }

    /// Performs layout for this element within the given available space and returns its size.
    pub fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        self.0.layout_as_root(available_space, cx)
    }

    /// Prepaints this element at the given absolute origin.
    pub fn prepaint_at(&mut self, origin: Point<Pixels>, cx: &mut ElementContext) {
        cx.with_absolute_element_offset(origin, |cx| self.0.prepaint(cx));
    }

    /// Performs layout on this element in the available space, then prepaints it at the given absolute origin.
    pub fn prepaint_as_root(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) {
        self.layout_as_root(available_space, cx);
        cx.with_absolute_element_offset(origin, |cx| self.0.prepaint(cx));
    }
}

impl Element for AnyElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn request_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.request_layout(cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        cx: &mut ElementContext,
    ) {
        self.prepaint(cx)
    }

    fn paint(
        &mut self,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
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
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn request_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::RequestLayoutState) {
        (cx.request_layout(&crate::Style::default(), None), ())
    }

    fn prepaint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::RequestLayoutState,
        _cx: &mut ElementContext,
    ) {
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _cx: &mut ElementContext,
    ) {
    }
}
