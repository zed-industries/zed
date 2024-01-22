//! # Welcome to GPUI!
//!
//! GPUI is a hybrid immediate and retained mode, GPU accelerated, UI framework
//! for Rust, designed to support a wide variety of applications. GPUI is currently
//! being actively developed and improved for the [Zed code editor](https://zed.dev/), and new versions
//! will have breaking changes. You'll probably need to use the latest stable version
//! of rust to use GPUI.
//!
//! # Getting started with GPUI
//!
//! TODO!(docs): Write a code sample showing how to create a window and render a simple
//! div
//!
//! # Drawing interesting things
//!
//! TODO!(docs): Expand demo to show how to draw a more interesting scene, with
//! a counter to store state and a button to increment it.
//!
//! # Interacting with your application state
//!
//! TODO!(docs): Expand demo to show GPUI entity interactions, like subscriptions and entities
//! maybe make a network request to show async stuff?
//!
//! # Conclusion
//!
//! TODO!(docs): Wrap up with a conclusion and links to other places? Zed / GPUI website?
//! Discord for chatting about it? Other tutorials or references?

#[macro_use]
mod action;
mod app;

mod arena;
mod assets;
mod color;
mod element;
mod elements;
mod executor;
mod geometry;
mod image_cache;
mod input;
mod interactive;
mod key_dispatch;
mod keymap;
mod platform;
pub mod prelude;
mod scene;
mod shared_string;
mod shared_url;
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
pub use assets::*;
pub use color::*;
pub use ctor::ctor;
pub use element::*;
pub use elements::*;
pub use executor::*;
pub use geometry::*;
pub use gpui_macros::{register_action, test, IntoElement, Render};
use image_cache::*;
pub use input::*;
pub use interactive::*;
use key_dispatch::*;
pub use keymap::*;
pub use platform::*;
pub use refineable::*;
pub use scene::*;
use seal::Sealed;
pub use shared_string::*;
pub use shared_url::*;
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
    fn set_global<T: 'static>(&mut self, global: T);
}

impl<C> BorrowAppContext for C
where
    C: BorrowMut<AppContext>,
{
    fn set_global<G: 'static>(&mut self, global: G) {
        self.borrow_mut().set_global(global)
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
