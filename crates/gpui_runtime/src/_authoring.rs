//! # Authoring GPUI
//!
//! UI is written at one of two levels: composing the built-in elements into
//! components, or writing a custom element when nothing built in emits what you
//! need. Both levels are public, so code written against them today should keep
//! compiling across minor releases.
//!
//! ## Composing components
//!
//! Most UI is built from [`div()`](crate::div) and the other element
//! constructors, refined with style and interaction builders:
//!
//! - [`Styled`](crate::Styled) for layout and appearance: `.p_4()`, `.bg()`,
//!   `.w_full()`, and the rest of the utility-style methods.
//! - [`InteractiveElement`](crate::InteractiveElement) for listeners:
//!   `.on_click()`, `.on_mouse_down()`, `.on_key_down()`.
//! - [`Render`](crate::Render) and [`RenderOnce`](crate::RenderOnce) to turn
//!   your own types into elements.
//! - [`Entity`](crate::Entity) and [`Context`](crate::Context) for the state
//!   that drives them.
//!
//! This level composes; it does not draw. Painting is decided by the elements
//! being composed.
//!
//! ## Writing custom elements
//!
//! When nothing built in draws what you need, implement
//! [`Element`](crate::Element). Its lifecycle runs in three phases:
//!
//! 1. [`Element::request_layout`](crate::Element::request_layout) asks the layout
//!    engine for a [`LayoutId`](crate::LayoutId), through
//!    [`Window::request_layout`](crate::Window::request_layout) or, for leaf
//!    content whose size only its own measurements know,
//!    [`Window::request_measured_layout`](crate::Window::request_measured_layout).
//! 2. [`Element::prepaint`](crate::Element::prepaint) runs once the element's
//!    [`Bounds`](crate::Bounds) are known. This is where you register hitboxes
//!    with [`Window::insert_hitbox`](crate::Window::insert_hitbox) and stash
//!    whatever state painting will need.
//! 3. [`Element::paint`](crate::Element::paint) emits the drawing:
//!    - [`Window::paint_quad`](crate::Window::paint_quad) draws a
//!      [`PaintQuad`](crate::PaintQuad), the same primitive a `div()` uses for
//!      its background, border and corner radii.
//!    - [`Window::paint_path`](crate::Window::paint_path) draws arbitrary vector
//!      geometry, built with [`PathBuilder`](crate::PathBuilder) into an
//!      engine-level [`Path`](crate::Path).
//!    - [`Window::paint_drop_shadows`](crate::Window::paint_drop_shadows) and
//!      [`Window::paint_inset_shadows`](crate::Window::paint_inset_shadows) draw
//!      [`BoxShadow`](crate::BoxShadow)s, before and after the element's fill
//!      respectively.
//!    - [`Window::paint_image`](crate::Window::paint_image) draws an image.
//!    - [`Window::layout_bounds`](crate::Window::layout_bounds) reads a child's
//!      computed bounds.
//!
//! None of these name a GPU API or an operating system. Each call appends a
//! record to the frame's scene, and the engine's
//! [`SceneRenderer`](crate::SceneRenderer) turns those records into whatever the
//! backend draws, so a custom element keeps working when the renderer is
//! replaced. [`LayoutId`](crate::LayoutId) is likewise an opaque token, so a
//! custom element keeps working when the layout engine is replaced.
//!
//! ## What is not authoring API
//!
//! Part of [`Window`](crate::Window)'s public surface exists for the runtime and
//! for test harnesses rather than for elements:
//!
//! - Frame orchestration: `compute_layout`, `present_if_needed`, damage diffing
//!   and `draw`. `draw` stays public because the visual test harnesses render
//!   frames with it, and is hidden from these docs.
//! - Element storage: the erased `ElementObject`, the element arena, and the
//!   stack-safety wrapper around it.
//! - Frame double-buffering and the retained hit-test tree.
//! - The inspector: the runtime publishes the identity of the element it is
//!   laying out or painting, so elements never handle it themselves.
//!
//! Depend on the authoring surface instead. If something in that list is the
//! only way to express what you need, that is a gap worth reporting.
