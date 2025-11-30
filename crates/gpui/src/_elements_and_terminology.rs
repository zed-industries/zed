//! GPUI has three ways to create an element; elements participate in laying out and painting the contents of the window.
//!
//! This page outlines some terminology and common patterns for expressing UI in GPUI.
//!
//! ## an Entity (struct `Entity<T>`)
//! Some state `T` owned by the `App`, accessible by references to context.
//!
//! ## Struct `AnyEntity`
//! A dynamically typed handle to an `Entity<T>`
//!
//! (See also struct `AnyView`)
//!
//! ## an Element (`impl Element`)
//! GPUI renders a tree of elements. Elements participate in the laying out and painting of the contents of the window.
//!
//! Elements come about in one of three ways:
//! 1. Implementing `Element` yourself (you likely wont need to; can simply compose gpui's prebuilt primitives: `Div`, etc)
//! 2. Implementing `Render` (a "View")
//! 3. Implementing `RenderOnce` (a "Component")
//!
//! ## Struct `AnyElement`
//! A dynamically typed element
//!
//!
//! ## Trait `IntoElement`
//! - `IntoElement::into_element` statically returns a type which implements Element
//!
//! - `IntoElement::into_any_element` dynamically returns a type which implements Element, namely `AnyElement`
//!
//! - May be derived on any type which implements `RenderOnce`
//! ```rust
//! #[derive(IntoElement)]
//! ```
//!
//! ## a View (`impl Render`)
//! A `View` is an `Entity<T>` where `T: Render`. You implement `Render::render` which is called on each frame and returns an `impl IntoElement`
//!
//!
//! ```rust
//! fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement;
//! ```
//! - Takes `&mut self` allowing it to consume and mutate state across frames.
//! - i.e. Create a View `T` which lives across frames. Store state in `T`'s fields. Access & mutate them inside `Render::render`
//!
//!
//!
//! ## Struct `AnyView`
//! A dynamically typed handle to a View (an `Entity<T>` with `T: Render`)
//!
//! ## a Component (`impl RenderOnce`)
//! A `Component` is a `T` where `T: RenderOnce`. You implement `RenderOnce::render` which is called on each frame and returns an `impl IntoElement`
//!
//!
//! ```rust
//! fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement;
//! ```
//! - Takes `self`, consuming itself. As such instances of `T` don't live across frames and don't store any state. You'll typically instantiate a brand new `T` every frame. They're useful as layout abstractions (i.e. per frame: instantiate a new Button component and pass it a label; let your implementation of Button handle the layout from there)
//!
//!
