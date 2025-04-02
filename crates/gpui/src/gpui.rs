//! # Welcome to GPUI!
//!
//! GPUI is a hybrid immediate and retained mode, GPU accelerated, UI framework
//! for Rust, designed to support a wide variety of applications.
//!
//! ## Getting Started
//!
//! GPUI is still in active development as we work on the Zed code editor and isn't yet on crates.io.
//! You'll also need to use the latest version of stable rust. Add the following to your Cargo.toml:
//!
//! ```
//! gpui = { git = "https://github.com/zed-industries/zed" }
//! ```
//!
//! Everything in GPUI starts with an [`Application`]. You can create one with [`Application::new`], and
//! kick off your application by passing a callback to [`Application::run`]. Inside this callback,
//! you can create a new window with [`App::open_window`], and register your first root
//! view. See [gpui.rs](https://www.gpui.rs/) for a complete example.
//!
//! ## The Big Picture
//!
//! GPUI offers three different [registers](https://en.wikipedia.org/wiki/Register_(sociolinguistics)) depending on your needs:
//!
//! - State management and communication with [`Entity`]'s. Whenever you need to store application state
//!   that communicates between different parts of your application, you'll want to use GPUI's
//!   entities. Entities are owned by GPUI and are only accessible through an owned smart pointer
//!   similar to an [`std::rc::Rc`]. See the [`app::context`] module for more information.
//!
//! - High level, declarative UI with views. All UI in GPUI starts with a view. A view is simply
//!   a [`Entity`] that can be rendered, by implementing the [`Render`] trait. At the start of each frame, GPUI
//!   will call this render method on the root view of a given window. Views build a tree of
//!   [`Element`]s, lay them out and style them with a tailwind-style API, and then give them to
//!   GPUI to turn into pixels. See the [`elements::Div`] element for an all purpose swiss-army
//!   knife for UI.
//!
//! - Low level, imperative UI with Elements. Elements are the building blocks of UI in GPUI, and they
//!   provide a nice wrapper around an imperative API that provides as much flexibility and control as
//!   you need. Elements have total control over how they and their child elements are rendered and
//!   can be used for making efficient views into large lists, implement custom layouting for a code editor,
//!   and anything else you can think of. See the [`element`] module for more information.
//!
//!  Each of these registers has one or more corresponding contexts that can be accessed from all GPUI services.
//!  This context is your main interface to GPUI, and is used extensively throughout the framework.
//!
//! ## Other Resources
//!
//! In addition to the systems above, GPUI provides a range of smaller services that are useful for building
//! complex applications:
//!
//! - Actions are user-defined structs that are used for converting keystrokes into logical operations in your UI.
//!   Use this for implementing keyboard shortcuts, such as cmd-q (See `action` module for more information).
//! - Platform services, such as `quit the app` or `open a URL` are available as methods on the [`app::App`].
//! - An async executor that is integrated with the platform's event loop. See the [`executor`] module for more information.,
//! - The [`gpui::test`](test) macro provides a convenient way to write tests for your GPUI applications. Tests also have their
//!   own kind of context, a [`TestAppContext`] which provides ways of simulating common platform input. See [`app::test_context`]
//!   and [`test`] modules for more details.
//!
//! Currently, the best way to learn about these APIs is to read the Zed source code, ask us about it at a fireside hack, or drop
//! a question in the [Zed Discord](https://zed.dev/community-links). We're working on improving the documentation, creating more examples,
//! and will be publishing more guides to GPUI on our [blog](https://zed.dev/blog).

#![deny(missing_docs)]
#![allow(clippy::type_complexity)] // Not useful, GPUI makes heavy use of callbacks
#![allow(clippy::collapsible_else_if)] // False positives in platform specific code
#![allow(unused_mut)] // False positives in platform specific code

#[macro_use]
mod action;
mod app;

mod arena;
mod asset_cache;
mod assets;
mod bounds_tree;
mod color;
mod element;
mod elements;
mod executor;
mod geometry;
mod global;
mod input;
mod interactive;
mod key_dispatch;
mod keymap;
mod path_builder;
mod platform;
pub mod prelude;
mod scene;
mod shared_string;
mod shared_uri;
mod style;
mod styled;
mod subscription;
mod svg_renderer;
mod taffy;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
mod text_system;
mod util;
mod view;
mod window;

/// Do not touch, here be dragons for use by gpui_macros and such.
#[doc(hidden)]
pub mod private {
    pub use anyhow;
    pub use inventory;
    pub use schemars;
    pub use serde;
    pub use serde_derive;
    pub use serde_json;
}

mod seal {
    /// A mechanism for restricting implementations of a trait to only those in GPUI.
    /// See: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/
    pub trait Sealed {}
}

pub use action::*;
pub use anyhow::Result;
pub use app::*;
pub(crate) use arena::*;
pub use asset_cache::*;
pub use assets::*;
pub use color::*;
pub use ctor::ctor;
pub use element::*;
pub use elements::*;
pub use executor::*;
pub use geometry::*;
pub use global::*;
pub use gpui_macros::{AppContext, IntoElement, Render, VisualContext, register_action, test};
pub use http_client;
pub use input::*;
pub use interactive::*;
use key_dispatch::*;
pub use keymap::*;
pub use path_builder::*;
pub use platform::*;
pub use refineable::*;
pub use scene::*;
pub use shared_string::*;
pub use shared_uri::*;
pub use smol::Timer;
pub use style::*;
pub use styled::*;
pub use subscription::*;
use svg_renderer::*;
pub use taffy::{AvailableSpace, LayoutId};
#[cfg(any(test, feature = "test-support"))]
pub use test::*;
pub use text_system::*;
pub use util::arc_cow::ArcCow;
pub use view::*;
pub use window::*;

use std::{any::Any, borrow::BorrowMut, future::Future};
use taffy::TaffyLayoutEngine;

/// The context trait, allows the different contexts in GPUI to be used
/// interchangeably for certain operations.
pub trait AppContext {
    /// The result type for this context, used for async contexts that
    /// can't hold a direct reference to the application context.
    type Result<T>;

    /// Create a new entity in the app context.
    fn new<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>>;

    /// Reserve a slot for a entity to be inserted later.
    /// The returned [Reservation] allows you to obtain the [EntityId] for the future entity.
    fn reserve_entity<T: 'static>(&mut self) -> Self::Result<Reservation<T>>;

    /// Insert a new entity in the app context based on a [Reservation] previously obtained from [`reserve_entity`].
    ///
    /// [`reserve_entity`]: Self::reserve_entity
    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>>;

    /// Update a entity in the app context.
    fn update_entity<T, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;

    /// Read a entity from the app context.
    fn read_entity<T, R>(
        &self,
        handle: &Entity<T>,
        read: impl FnOnce(&T, &App) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;

    /// Update a window for the given handle.
    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T;

    /// Read a window off of the application context.
    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static;

    /// Spawn a future on a background thread
    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static;

    /// Read a global from this app context
    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> Self::Result<R>
    where
        G: Global;
}

/// Returned by [Context::reserve_entity] to later be passed to [Context::insert_entity].
/// Allows you to obtain the [EntityId] for a entity before it is created.
pub struct Reservation<T>(pub(crate) Slot<T>);

impl<T: 'static> Reservation<T> {
    /// Returns the [EntityId] that will be associated with the entity once it is inserted.
    pub fn entity_id(&self) -> EntityId {
        self.0.entity_id()
    }
}

/// This trait is used for the different visual contexts in GPUI that
/// require a window to be present.
pub trait VisualContext: AppContext {
    /// Returns the handle of the window associated with this context.
    fn window_handle(&self) -> AnyWindowHandle;

    /// Update a view with the given callback
    fn update_window_entity<T: 'static, R>(
        &mut self,
        entity: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R,
    ) -> Self::Result<R>;

    /// Update a view with the given callback
    fn new_window_entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Window, &mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>>;

    /// Replace the root view of a window with a new view.
    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V,
    ) -> Self::Result<Entity<V>>
    where
        V: 'static + Render;

    /// Focus a entity in the window, if it implements the [`Focusable`] trait.
    fn focus<V>(&mut self, entity: &Entity<V>) -> Self::Result<()>
    where
        V: Focusable;
}

/// A trait for tying together the types of a GPUI entity and the events it can
/// emit.
pub trait EventEmitter<E: Any>: 'static {}

/// A helper trait for auto-implementing certain methods on contexts that
/// can be used interchangeably.
pub trait BorrowAppContext {
    /// Set a global value on the context.
    fn set_global<T: Global>(&mut self, global: T);
    /// Updates the global state of the given type.
    fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: Global;
    /// Updates the global state of the given type, creating a default if it didn't exist before.
    fn update_default_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: Global + Default;
}

impl<C> BorrowAppContext for C
where
    C: BorrowMut<App>,
{
    fn set_global<G: Global>(&mut self, global: G) {
        self.borrow_mut().set_global(global)
    }

    #[track_caller]
    fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: Global,
    {
        let mut global = self.borrow_mut().lease_global::<G>();
        let result = f(&mut global, self);
        self.borrow_mut().end_global_lease(global);
        result
    }

    fn update_default_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: Global + Default,
    {
        self.borrow_mut().default_global::<G>();
        self.update_global(f)
    }
}

/// A flatten equivalent for anyhow `Result`s.
pub trait Flatten<T> {
    /// Convert this type into a simple `Result<T>`.
    fn flatten(self) -> Result<T>;
}

impl<T> Flatten<T> for Result<Result<T>> {
    fn flatten(self) -> Result<T> {
        self?
    }
}

impl<T> Flatten<T> for Result<T> {
    fn flatten(self) -> Result<T> {
        self
    }
}

/// Information about the GPU GPUI is running on.
#[derive(Default, Debug)]
pub struct GpuSpecs {
    /// Whether the GPU is really a fake (like `llvmpipe`) running on the CPU.
    pub is_software_emulated: bool,
    /// The name of the device, as reported by Vulkan.
    pub device_name: String,
    /// The name of the driver, as reported by Vulkan.
    pub driver_name: String,
    /// Further information about the driver, as reported by Vulkan.
    pub driver_info: String,
}
