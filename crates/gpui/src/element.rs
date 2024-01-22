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
//! If an element returns an [`ElementId`] from [`IntoElement::element_id()`], and that element id
//! appears in the same place relative to other views and ElementIds in the frame, then the previous
//! frame's state will be passed to the element's layout and paint methods.
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
    util::FluentBuilder, AppContext, ArenaBox, AvailableSpace, BorrowWindow, Bounds, ContentMask,
    ElementId, ElementStateBox, EntityId, IsZero, LayoutId, Pixels, Point, Size, ViewContext,
    Window, WindowContext, ELEMENT_ARENA,
};
use derive_more::{Deref, DerefMut};
pub(crate) use smallvec::SmallVec;
use std::{
    any::Any,
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    mem,
    ops::DerefMut,
};
use util::post_inc;

/// This context is used for assisting in the implementation of the element trait
#[derive(Deref, DerefMut)]
pub struct ElementContext<'a> {
    pub(crate) cx: WindowContext<'a>,
}

impl<'a> WindowContext<'a> {
    pub(crate) fn into_element_cx(self) -> ElementContext<'a> {
        ElementContext { cx: self }
    }
}

impl<'a> Borrow<AppContext> for ElementContext<'a> {
    fn borrow(&self) -> &AppContext {
        self.cx.borrow()
    }
}

impl<'a> BorrowMut<AppContext> for ElementContext<'a> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        self.cx.borrow_mut()
    }
}

impl<'a> Borrow<Window> for ElementContext<'a> {
    fn borrow(&self) -> &Window {
        self.cx.borrow()
    }
}

impl<'a> BorrowMut<Window> for ElementContext<'a> {
    fn borrow_mut(&mut self) -> &mut Window {
        self.cx.borrow_mut()
    }
}

/// Implemented by types that participate in laying out and painting the contents of a window.
/// Elements form a tree and are laid out according to web-based layout rules, as implemented by Taffy.
/// You can create custom elements by implementing this trait, see the module-level documentation
/// for more details.
pub trait Element: 'static + IntoElement {
    /// The type of state to store for this element between frames. See the module-level documentation
    /// for details.
    type State: 'static;

    /// Before an element can be painted, we need to know where it's going to be and how big it is.
    /// Use this method to request a layout from Taffy and initialize the element's state.
    fn request_layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State);

    /// Once layout has been completed, this method will be called to paint the element to the screen.
    /// The state argument is the same state that was returned from [`Element::request_layout()`].
    fn paint(&mut self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut ElementContext);

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

    /// The [`ElementId`] of self once converted into an [`Element`].
    /// If present, the resulting element's state will be carried across frames.
    fn element_id(&self) -> Option<ElementId>;

    /// Convert self into a type that implements [`Element`].
    fn into_element(self) -> Self::Element;

    /// Convert self into a dynamically-typed [`AnyElement`].
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }

    /// Convert into an element, then draw in the current window at the given origin.
    /// The available space argument is provided to the layout engine to determine the size of the
    // root element.  Once the element is drawn, its associated element state is yielded to the
    // given callback.
    fn draw_and_update_state<T, R>(
        self,
        origin: Point<Pixels>,
        available_space: Size<T>,
        cx: &mut ElementContext,
        f: impl FnOnce(&mut <Self::Element as Element>::State, &mut ElementContext) -> R,
    ) -> R
    where
        T: Clone + Default + Debug + Into<AvailableSpace>,
    {
        let element = self.into_element();
        let element_id = element.element_id();
        let element = DrawableElement {
            element: Some(element),
            phase: ElementDrawPhase::Start,
        };

        let frame_state =
            DrawableElement::draw(element, origin, available_space.map(Into::into), cx);

        if let Some(mut frame_state) = frame_state {
            f(&mut frame_state, cx)
        } else {
            cx.with_element_state(element_id.unwrap(), |element_state, cx| {
                let mut element_state = element_state.unwrap();
                let result = f(&mut element_state, cx);
                (result, element_state)
            })
        }
    }
}

impl<T: IntoElement> FluentBuilder for T {}

/// An object that can be drawn to the screen. This is the trait that distinguishes `Views` from
/// models. Views are drawn to the screen and care about the current window's state, models are not and do not.
pub trait Render: 'static + Sized {
    /// Render this view into an element tree.
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement;
}

impl Render for () {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        ()
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
    type State = AnyElement;

    fn request_layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        let mut element = self
            .0
            .take()
            .unwrap()
            .render(cx.deref_mut())
            .into_any_element();
        let layout_id = element.request_layout(cx);
        (layout_id, element)
    }

    fn paint(&mut self, _: Bounds<Pixels>, element: &mut Self::State, cx: &mut ElementContext) {
        element.paint(cx)
    }
}

impl<C: RenderOnce> IntoElement for Component<C> {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A globally unique identifier for an element, used to track state across frames.
#[derive(Deref, DerefMut, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct GlobalElementId(SmallVec<[ElementId; 32]>);

trait ElementObject {
    fn element_id(&self) -> Option<ElementId>;

    fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId;

    fn paint(&mut self, cx: &mut ElementContext);

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels>;

    fn draw(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    );
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
pub(crate) struct DrawableElement<E: Element> {
    element: Option<E>,
    phase: ElementDrawPhase<E::State>,
}

#[derive(Default)]
enum ElementDrawPhase<S> {
    #[default]
    Start,
    LayoutRequested {
        layout_id: LayoutId,
        frame_state: Option<S>,
    },
    LayoutComputed {
        layout_id: LayoutId,
        available_space: Size<AvailableSpace>,
        frame_state: Option<S>,
    },
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
impl<E: Element> DrawableElement<E> {
    fn new(element: E) -> Self {
        DrawableElement {
            element: Some(element),
            phase: ElementDrawPhase::Start,
        }
    }

    fn element_id(&self) -> Option<ElementId> {
        self.element.as_ref()?.element_id()
    }

    fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        let (layout_id, frame_state) = if let Some(id) = self.element.as_ref().unwrap().element_id()
        {
            let layout_id = cx.with_element_state(id, |element_state, cx| {
                self.element
                    .as_mut()
                    .unwrap()
                    .request_layout(element_state, cx)
            });
            (layout_id, None)
        } else {
            let (layout_id, frame_state) = self.element.as_mut().unwrap().request_layout(None, cx);
            (layout_id, Some(frame_state))
        };

        self.phase = ElementDrawPhase::LayoutRequested {
            layout_id,
            frame_state,
        };
        layout_id
    }

    fn paint(mut self, cx: &mut ElementContext) -> Option<E::State> {
        match self.phase {
            ElementDrawPhase::LayoutRequested {
                layout_id,
                frame_state,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                frame_state,
                ..
            } => {
                let bounds = cx.layout_bounds(layout_id);

                if let Some(mut frame_state) = frame_state {
                    self.element
                        .take()
                        .unwrap()
                        .paint(bounds, &mut frame_state, cx);
                    Some(frame_state)
                } else {
                    let element_id = self
                        .element
                        .as_ref()
                        .unwrap()
                        .element_id()
                        .expect("if we don't have frame state, we should have element state");
                    cx.with_element_state(element_id, |element_state, cx| {
                        let mut element_state = element_state.unwrap();
                        self.element
                            .take()
                            .unwrap()
                            .paint(bounds, &mut element_state, cx);
                        ((), element_state)
                    });
                    None
                }
            }

            _ => panic!("must call layout before paint"),
        }
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.request_layout(cx);
        }

        let layout_id = match &mut self.phase {
            ElementDrawPhase::LayoutRequested {
                layout_id,
                frame_state,
            } => {
                cx.compute_layout(*layout_id, available_space);
                let layout_id = *layout_id;
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    available_space,
                    frame_state: frame_state.take(),
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                available_space: prev_available_space,
                ..
            } => {
                if available_space != *prev_available_space {
                    cx.compute_layout(*layout_id, available_space);
                    *prev_available_space = available_space;
                }
                *layout_id
            }
            _ => panic!("cannot measure after painting"),
        };

        cx.layout_bounds(layout_id).size
    }

    fn draw(
        mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Option<E::State> {
        self.measure(available_space, cx);
        cx.with_absolute_element_offset(origin, |cx| self.paint(cx))
    }
}

impl<E> ElementObject for Option<DrawableElement<E>>
where
    E: Element,
    E::State: 'static,
{
    fn element_id(&self) -> Option<ElementId> {
        self.as_ref().unwrap().element_id()
    }

    fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        DrawableElement::request_layout(self.as_mut().unwrap(), cx)
    }

    fn paint(&mut self, cx: &mut ElementContext) {
        DrawableElement::paint(self.take().unwrap(), cx);
    }

    fn measure(
        &mut self,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) -> Size<Pixels> {
        DrawableElement::measure(self.as_mut().unwrap(), available_space, cx)
    }

    fn draw(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) {
        DrawableElement::draw(self.take().unwrap(), origin, available_space, cx);
    }
}

/// A dynamically typed element that can be used to store any element type.
pub struct AnyElement(ArenaBox<dyn ElementObject>);

impl AnyElement {
    pub(crate) fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::State: Any,
    {
        let element = ELEMENT_ARENA
            .with_borrow_mut(|arena| arena.alloc(|| Some(DrawableElement::new(element))))
            .map(|element| element as &mut dyn ElementObject);
        AnyElement(element)
    }

    /// Request the layout ID of the element stored in this `AnyElement`.
    /// Used for laying out child elements in a parent element.
    pub fn request_layout(&mut self, cx: &mut ElementContext) -> LayoutId {
        self.0.request_layout(cx)
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

    /// Initializes this element and performs layout in the available space, then paints it at the given origin.
    pub fn draw(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        cx: &mut ElementContext,
    ) {
        self.0.draw(origin, available_space, cx)
    }

    /// Returns the element ID of the element stored in this `AnyElement`, if any.
    pub fn inner_id(&self) -> Option<ElementId> {
        self.0.element_id()
    }
}

impl Element for AnyElement {
    type State = ();

    fn request_layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        let layout_id = self.request_layout(cx);
        (layout_id, ())
    }

    fn paint(&mut self, _: Bounds<Pixels>, _: &mut Self::State, cx: &mut ElementContext) {
        self.paint(cx)
    }
}

impl IntoElement for AnyElement {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }

    fn into_any_element(self) -> AnyElement {
        self
    }
}

/// The empty element, which renders nothing.
pub type Empty = ();

impl IntoElement for () {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for () {
    type State = ();

    fn request_layout(
        &mut self,
        _state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        (cx.request_layout(&crate::Style::default(), None), ())
    }

    fn paint(
        &mut self,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::State,
        _cx: &mut ElementContext,
    ) {
    }
}

impl<'a> ElementContext<'a> {
    /// Pushes the given element id onto the global stack and invokes the given closure
    /// with a `GlobalElementId`, which disambiguates the given id in the context of its ancestor
    /// ids. Because elements are discarded and recreated on each frame, the `GlobalElementId` is
    /// used to associate state with identified elements across separate frames.
    fn with_element_id<R>(
        &mut self,
        id: Option<impl Into<ElementId>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if let Some(id) = id.map(Into::into) {
            let window = self.window_mut();
            window.element_id_stack.push(id);
            let result = f(self);
            let window: &mut Window = self.borrow_mut();
            window.element_id_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Invoke the given function with the given content mask after intersecting it
    /// with the current mask.
    fn with_content_mask<R>(
        &mut self,
        mask: Option<ContentMask<Pixels>>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if let Some(mask) = mask {
            let mask = mask.intersect(&self.content_mask());
            self.window_mut().next_frame.content_mask_stack.push(mask);
            let result = f(self);
            self.window_mut().next_frame.content_mask_stack.pop();
            result
        } else {
            f(self)
        }
    }

    /// Invoke the given function with the content mask reset to that
    /// of the window.
    fn break_content_mask<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let mask = ContentMask {
            bounds: Bounds {
                origin: Point::default(),
                size: self.window().viewport_size,
            },
        };
        let new_stacking_order_id =
            post_inc(&mut self.window_mut().next_frame.next_stacking_order_id);
        let new_root_z_index = post_inc(&mut self.window_mut().next_frame.next_root_z_index);
        let old_stacking_order = mem::take(&mut self.window_mut().next_frame.z_index_stack);
        self.window_mut().next_frame.z_index_stack.id = new_stacking_order_id;
        self.window_mut()
            .next_frame
            .z_index_stack
            .push(new_root_z_index);
        self.window_mut().next_frame.content_mask_stack.push(mask);
        let result = f(self);
        self.window_mut().next_frame.content_mask_stack.pop();
        self.window_mut().next_frame.z_index_stack = old_stacking_order;
        result
    }

    /// Called during painting to invoke the given closure in a new stacking context. The given
    /// z-index is interpreted relative to the previous call to `stack`.
    fn with_z_index<R>(&mut self, z_index: u8, f: impl FnOnce(&mut Self) -> R) -> R {
        let new_stacking_order_id =
            post_inc(&mut self.window_mut().next_frame.next_stacking_order_id);
        let old_stacking_order_id = mem::replace(
            &mut self.window_mut().next_frame.z_index_stack.id,
            new_stacking_order_id,
        );
        self.window_mut().next_frame.z_index_stack.id = new_stacking_order_id;
        self.window_mut().next_frame.z_index_stack.push(z_index);
        let result = f(self);
        self.window_mut().next_frame.z_index_stack.id = old_stacking_order_id;
        self.window_mut().next_frame.z_index_stack.pop();
        result
    }

    /// Updates the global element offset relative to the current offset. This is used to implement
    /// scrolling.
    fn with_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        if offset.is_zero() {
            return f(self);
        };

        let abs_offset = self.element_offset() + offset;
        self.with_absolute_element_offset(abs_offset, f)
    }

    /// Updates the global element offset based on the given offset. This is used to implement
    /// drag handles and other manual painting of elements.
    fn with_absolute_element_offset<R>(
        &mut self,
        offset: Point<Pixels>,
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.window_mut()
            .next_frame
            .element_offset_stack
            .push(offset);
        let result = f(self);
        self.window_mut().next_frame.element_offset_stack.pop();
        result
    }

    /// Obtain the current element offset.
    fn element_offset(&self) -> Point<Pixels> {
        self.window()
            .next_frame
            .element_offset_stack
            .last()
            .copied()
            .unwrap_or_default()
    }

    /// Obtain the current content mask.
    fn content_mask(&self) -> ContentMask<Pixels> {
        self.window()
            .next_frame
            .content_mask_stack
            .last()
            .cloned()
            .unwrap_or_else(|| ContentMask {
                bounds: Bounds {
                    origin: Point::default(),
                    size: self.window().viewport_size,
                },
            })
    }

    /// The size of an em for the base font of the application. Adjusting this value allows the
    /// UI to scale, just like zooming a web page.
    fn rem_size(&self) -> Pixels {
        self.window().rem_size
    }

    fn parent_view_id(&self) -> EntityId {
        *self
            .window
            .next_frame
            .view_stack
            .last()
            .expect("a view should always be on the stack while drawing")
    }

    /// Updates or initializes state for an element with the given id that lives across multiple
    /// frames. If an element with this ID existed in the rendered frame, its state will be passed
    /// to the given closure. The state returned by the closure will be stored so it can be referenced
    /// when drawing the next frame.
    pub(crate) fn with_element_state<S, R>(
        &mut self,
        id: ElementId,
        f: impl FnOnce(Option<S>, &mut Self) -> (R, S),
    ) -> R
    where
        S: 'static,
    {
        self.with_element_id(Some(id), |cx| {
            let global_id = cx.window().element_id_stack.clone();

            if let Some(any) = cx
                .window_mut()
                .next_frame
                .element_states
                .remove(&global_id)
                .or_else(|| {
                    cx.window_mut()
                        .rendered_frame
                        .element_states
                        .remove(&global_id)
                })
            {
                let ElementStateBox {
                    inner,
                    parent_view_id,
                    #[cfg(debug_assertions)]
                    type_name
                } = any;
                // Using the extra inner option to avoid needing to reallocate a new box.
                let mut state_box = inner
                    .downcast::<Option<S>>()
                    .map_err(|_| {
                        #[cfg(debug_assertions)]
                        {
                            anyhow::anyhow!(
                                "invalid element state type for id, requested_type {:?}, actual type: {:?}",
                                std::any::type_name::<S>(),
                                type_name
                            )
                        }

                        #[cfg(not(debug_assertions))]
                        {
                            anyhow::anyhow!(
                                "invalid element state type for id, requested_type {:?}",
                                std::any::type_name::<S>(),
                            )
                        }
                    })
                    .unwrap();

                // Actual: Option<AnyElement> <- View
                // Requested: () <- AnyElement
                let state = state_box
                    .take()
                    .expect("element state is already on the stack");
                let (result, state) = f(Some(state), cx);
                state_box.replace(state);
                cx.window_mut()
                    .next_frame
                    .element_states
                    .insert(global_id, ElementStateBox {
                        inner: state_box,
                        parent_view_id,
                        #[cfg(debug_assertions)]
                        type_name
                    });
                result
            } else {
                let (result, state) = f(None, cx);
                let parent_view_id = cx.parent_view_id();
                cx.window_mut()
                    .next_frame
                    .element_states
                    .insert(global_id,
                        ElementStateBox {
                            inner: Box::new(Some(state)),
                            parent_view_id,
                            #[cfg(debug_assertions)]
                            type_name: std::any::type_name::<S>()
                        }

                    );
                result
            }
        })
    }
}
