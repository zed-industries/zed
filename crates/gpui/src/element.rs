//! Elements are the workhorses of GPUI. They are responsible for laying out and painting all of
//! the contents of a window. Elements form a tree and are laid out according to the web layout
//! standards as implemented by [taffy](https://github.com/DioxusLabs/taffy). Most of the time,
//! you won't need to interact with this module or these APIs directly. Elements provide their
//! own APIs and GPUI, or other element implementation, uses the APIs in this module to convert
//! that element tree into the pixels you see on the screen.
//!
//! # Element Basics
//!
//! Elements are constructed by calling [`Render::render()`] on the root view of the window,
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
    util::FluentBuilder, ArenaBox, AvailableSpace, Bounds, DispatchNodeId, ElementId, FocusHandle,
    LayoutId, Pixels, Point, Size, Style, ViewContext, WindowContext, ELEMENT_ARENA,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{any::Any, fmt::Debug, mem};

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

    /// If this element has a unique identifier, return it here. This is used to track elements across frames, and
    /// will cause a GlobalElementId to be passed to the request_layout, prepaint, and paint methods.
    ///
    /// The global id can in turn be used to access state that's connected to an element with the same id across
    /// frames. This id must be unique among children of the first containing element with an id.
    fn id(&self) -> Option<ElementId>;

    /// Before an element can be painted, we need to know where it's going to be and how big it is.
    /// Use this method to request a layout from Taffy and initialize the element's state.
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState);

    /// After laying out an element, we need to commit its bounds to the current frame for hitbox
    /// purposes. The state argument is the same state that was returned from [`Element::request_layout()`].
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState;

    /// Once layout has been completed, this method will be called to paint the element to the screen.
    /// The state argument is the same state that was returned from [`Element::request_layout()`].
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
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
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>);

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

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut element = self.0.take().unwrap().render(cx).into_any_element();
        let layout_id = element.request_layout(cx);
        (layout_id, element)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        element: &mut AnyElement,
        cx: &mut WindowContext,
    ) {
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

impl<C: RenderOnce> IntoElement for Component<C> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A globally unique identifier for an element, used to track state across frames.
#[derive(Deref, DerefMut, Default, Debug, Eq, PartialEq, Hash)]
pub struct GlobalElementId(pub(crate) SmallVec<[ElementId; 32]>);

trait ElementObject {
    fn inner_element(&mut self) -> &mut dyn Any;

    fn request_layout(&mut self, cx: &mut WindowContext) -> LayoutId;

    fn prepaint(&mut self, cx: &mut WindowContext);

    fn paint(&mut self, cx: &mut WindowContext);

    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
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
    RequestLayout {
        layout_id: LayoutId,
        global_id: Option<GlobalElementId>,
        request_layout: RequestLayoutState,
    },
    LayoutComputed {
        layout_id: LayoutId,
        global_id: Option<GlobalElementId>,
        available_space: Size<AvailableSpace>,
        request_layout: RequestLayoutState,
    },
    Prepaint {
        node_id: DispatchNodeId,
        global_id: Option<GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: RequestLayoutState,
        prepaint: PrepaintState,
    },
    Painted,
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
impl<E: Element> Drawable<E> {
    pub(crate) fn new(element: E) -> Self {
        Drawable {
            element,
            phase: ElementDrawPhase::Start,
        }
    }

    fn request_layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Start => {
                let global_id = self.element.id().map(|element_id| {
                    cx.window.element_id_stack.push(element_id);
                    GlobalElementId(cx.window.element_id_stack.clone())
                });

                let (layout_id, request_layout) =
                    self.element.request_layout(global_id.as_ref(), cx);

                if global_id.is_some() {
                    cx.window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::RequestLayout {
                    layout_id,
                    global_id,
                    request_layout,
                };
                layout_id
            }
            _ => panic!("must call request_layout only once"),
        }
    }

    pub(crate) fn prepaint(&mut self, cx: &mut WindowContext) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayout {
                layout_id,
                global_id,
                mut request_layout,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                global_id,
                mut request_layout,
                ..
            } => {
                if let Some(element_id) = self.element.id() {
                    cx.window.element_id_stack.push(element_id);
                    debug_assert_eq!(global_id.as_ref().unwrap().0, cx.window.element_id_stack);
                }

                let bounds = cx.layout_bounds(layout_id);
                let node_id = cx.window.next_frame.dispatch_tree.push_node();
                let prepaint =
                    self.element
                        .prepaint(global_id.as_ref(), bounds, &mut request_layout, cx);
                cx.window.next_frame.dispatch_tree.pop_node();

                if global_id.is_some() {
                    cx.window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::Prepaint {
                    node_id,
                    global_id,
                    bounds,
                    request_layout,
                    prepaint,
                };
            }
            _ => panic!("must call request_layout before prepaint"),
        }
    }

    pub(crate) fn paint(
        &mut self,
        cx: &mut WindowContext,
    ) -> (E::RequestLayoutState, E::PrepaintState) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Prepaint {
                node_id,
                global_id,
                bounds,
                mut request_layout,
                mut prepaint,
                ..
            } => {
                if let Some(element_id) = self.element.id() {
                    cx.window.element_id_stack.push(element_id);
                    debug_assert_eq!(global_id.as_ref().unwrap().0, cx.window.element_id_stack);
                }

                cx.window.next_frame.dispatch_tree.set_active_node(node_id);
                self.element.paint(
                    global_id.as_ref(),
                    bounds,
                    &mut request_layout,
                    &mut prepaint,
                    cx,
                );

                if global_id.is_some() {
                    cx.window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::Painted;
                (request_layout, prepaint)
            }
            _ => panic!("must call prepaint before paint"),
        }
    }

    pub(crate) fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.request_layout(cx);
        }

        let layout_id = match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayout {
                layout_id,
                global_id,
                request_layout,
            } => {
                cx.compute_layout(layout_id, available_space);
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    global_id,
                    available_space,
                    request_layout,
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                global_id,
                available_space: prev_available_space,
                request_layout,
            } => {
                if available_space != prev_available_space {
                    cx.compute_layout(layout_id, available_space);
                }
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    global_id,
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

    fn request_layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        Drawable::request_layout(self, cx)
    }

    fn prepaint(&mut self, cx: &mut WindowContext) {
        Drawable::prepaint(self, cx);
    }

    fn paint(&mut self, cx: &mut WindowContext) {
        Drawable::paint(self, cx);
    }

    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
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
    pub fn request_layout(&mut self, cx: &mut WindowContext) -> LayoutId {
        self.0.request_layout(cx)
    }

    /// Prepares the element to be painted by storing its bounds, giving it a chance to draw hitboxes and
    /// request autoscroll before the final paint pass is confirmed.
    pub fn prepaint(&mut self, cx: &mut WindowContext) -> Option<FocusHandle> {
        let focus_assigned = cx.window.next_frame.focus.is_some();

        self.0.prepaint(cx);

        if !focus_assigned {
            if let Some(focus_id) = cx.window.next_frame.focus {
                return FocusHandle::for_id(focus_id, &cx.window.focus_handles);
            }
        }

        None
    }

    /// Paints the element stored in this `AnyElement`.
    pub fn paint(&mut self, cx: &mut WindowContext) {
        self.0.paint(cx);
    }

    /// Performs layout for this element within the given available space and returns its size.
    pub fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Size<Pixels> {
        self.0.layout_as_root(available_space, cx)
    }

    /// Prepaints this element at the given absolute origin.
    /// If any element in the subtree beneath this element is focused, its FocusHandle is returned.
    pub fn prepaint_at(
        &mut self,
        origin: Point<Pixels>,
        cx: &mut WindowContext,
    ) -> Option<FocusHandle> {
        cx.with_absolute_element_offset(origin, |cx| self.prepaint(cx))
    }

    /// Performs layout on this element in the available space, then prepaints it at the given absolute origin.
    /// If any element in the subtree beneath this element is focused, its FocusHandle is returned.
    pub fn prepaint_as_root(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut WindowContext,
    ) -> Option<FocusHandle> {
        self.layout_as_root(available_space, cx);
        cx.with_absolute_element_offset(origin, |cx| self.prepaint(cx))
    }
}

impl Element for AnyElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.request_layout(cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) {
        self.prepaint(cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        self.paint(cx);
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

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (cx.request_layout(Style::default(), None), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::RequestLayoutState,
        _cx: &mut WindowContext,
    ) {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _cx: &mut WindowContext,
    ) {
    }
}
