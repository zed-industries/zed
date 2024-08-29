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
//! Everything in GPUI starts with an [`App`]. You can create one with [`App::new`], and
//! kick off your application by passing a callback to [`App::run`]. Inside this callback,
//! you can create a new window with [`AppContext::open_window`], and register your first root
//! view. See [gpui.rs](https://www.gpui.rs/) for a complete example.
//!
//! ## The Big Picture
//!
//! GPUI offers three different [registers](https://en.wikipedia.org/wiki/Register_(sociolinguistics)) depending on your needs:
//!
//! - State management and communication with Models. Whenever you need to store application state
//!   that communicates between different parts of your application, you'll want to use GPUI's
//!   models. Models are owned by GPUI and are only accessible through an owned smart pointer
//!   similar to an [`Rc`]. See the [`app::model_context`] module for more information.
//!
//! - High level, declarative UI with Views. All UI in GPUI starts with a View. A view is simply
//!   a model that can be rendered, via the [`Render`] trait. At the start of each frame, GPUI
//!   will call this render method on the root view of a given window. Views build a tree of
//!   `elements`, lay them out and style them with a tailwind-style API, and then give them to
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
//!   Use this for implementing keyboard shortcuts, such as cmd-q. See the [`action`] module for more information.
//! - Platform services, such as `quit the app` or `open a URL` are available as methods on the [`app::AppContext`].
//! - An async executor that is integrated with the platform's event loop. See the [`executor`] module for more information.,
//! - The [gpui::test] macro provides a convenient way to write tests for your GPUI applications. Tests also have their
//!   own kind of context, a [`TestAppContext`] which provides ways of simulating common platform input. See [`app::test_context`]
//!   and [`test`] modules for more details.
//!
//! Currently, the best way to learn about these APIs is to read the Zed source code, ask us about it at a fireside hack, or drop
//! a question in the [Zed Discord](https://discord.gg/zed-community). We're working on improving the documentation, creating more examples,
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
    pub use linkme;
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
pub use gpui_macros::{register_action, test, IntoElement, Render};
pub use input::*;
pub use interactive::*;
use key_dispatch::*;
pub use keymap::*;
pub use platform::*;
pub use refineable::*;
pub use scene::*;
use seal::Sealed;
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

use std::{any::Any, borrow::BorrowMut};
use taffy::TaffyLayoutEngine;

/// The context trait, allows the different contexts in GPUI to be used
/// interchangeably for certain operations.
pub trait Context {
    /// The result type for this context, used for async contexts that
    /// can't hold a direct reference to the application context.
    type Result<T>;

    /// Create a new model in the app context.
    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    /// Reserve a slot for a model to be inserted later.
    /// The returned [Reservation] allows you to obtain the [EntityId] for the future model.
    fn reserve_model<T: 'static>(&mut self) -> Self::Result<Reservation<T>>;

    /// Insert a new model in the app context based on a [Reservation] previously obtained from [`reserve_model`].
    ///
    /// [`reserve_model`]: Self::reserve_model
    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    /// Update a model in the app context.
    fn update_model<T, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;

    /// Read a model from the app context.
    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;

    /// Update a window for the given handle.
    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T;

    /// Read a window off of the application context.
    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static;
}

/// Returned by [Context::reserve_model] to later be passed to [Context::insert_model].
/// Allows you to obtain the [EntityId] for a model before it is created.
pub struct Reservation<T>(pub(crate) Slot<T>);

impl<T: 'static> Reservation<T> {
    /// Returns the [EntityId] that will be associated with the model once it is inserted.
    pub fn entity_id(&self) -> EntityId {
        self.0.entity_id()
    }
}

/// This trait is used for the different visual contexts in GPUI that
/// require a window to be present.
pub trait VisualContext: Context {
    /// Construct a new view in the window referenced by this context.
    fn new_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render;

    /// Update a view with the given callback
    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Self::Result<R>;

    /// Replace the root view of a window with a new view.
    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render;

    /// Focus a view in the window, if it implements the [`FocusableView`] trait.
    fn focus_view<V>(&mut self, view: &View<V>) -> Self::Result<()>
    where
        V: FocusableView;

    /// Dismiss a view in the window, if it implements the [`ManagedView`] trait.
    fn dismiss_view<V>(&mut self, view: &View<V>) -> Self::Result<()>
    where
        V: ManagedView;
}

/// A trait that allows models and views to be interchangeable in certain operations
pub trait Entity<T>: Sealed {
    /// The weak reference type for this entity.
    type Weak: 'static;

    /// The ID for this entity
    fn entity_id(&self) -> EntityId;

    /// Downgrade this entity to a weak reference.
    fn downgrade(&self) -> Self::Weak;

    /// Upgrade this entity from a weak reference.
    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized;
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
    C: BorrowMut<AppContext>,
{
    fn set_global<G: Global>(&mut self, global: G) {
        self.borrow_mut().set_global(global)
    }

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

#[derive(Default, Debug)]
/// Information about the GPU GPUI is running on
pub struct GPUSpecs {
    /// true if the GPU is really a fake (like llvmpipe) running on the CPU
    pub is_software_emulated: bool,
    /// Name of the device as reported by vulkan
    pub device_name: String,
    /// Name of the driver as reported by vulkan
    pub driver_name: String,
    /// Further driver info as reported by vulkan
    pub driver_info: String,
}
