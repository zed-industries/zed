#[macro_use]
mod action;
mod app;
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

mod private {
    /// A mechanism for restricting implementations of a trait to only those in GPUI.
    /// See: https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/
    pub trait Sealed {}
}

pub use action::*;
pub use anyhow::Result;
pub use app::*;
pub use assets::*;
pub use color::*;
pub use ctor::ctor;
pub use element::*;
pub use elements::*;
pub use executor::*;
pub use geometry::*;
pub use gpui2_macros::*;
pub use image_cache::*;
pub use input::*;
pub use interactive::*;
pub use key_dispatch::*;
pub use keymap::*;
pub use linkme;
pub use platform::*;
use private::Sealed;
pub use refineable::*;
pub use scene::*;
pub use serde;
pub use serde_derive;
pub use serde_json;
pub use smallvec;
pub use smol::Timer;
pub use style::*;
pub use styled::*;
pub use subscription::*;
pub use svg_renderer::*;
pub use taffy::{AvailableSpace, LayoutId};
#[cfg(any(test, feature = "test-support"))]
pub use test::*;
pub use text_system::*;
pub use util::arc_cow::ArcCow;
pub use view::*;
pub use window::*;

use derive_more::{Deref, DerefMut};
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut},
};
use taffy::TaffyLayoutEngine;

pub trait Context {
    type Result<T>;

    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    fn update_model<T, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T;

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static;
}

pub trait VisualContext: Context {
    fn build_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render;

    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Self::Result<R>;

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render;

    fn focus_view<V>(&mut self, view: &View<V>) -> Self::Result<()>
    where
        V: FocusableView;

    fn dismiss_view<V>(&mut self, view: &View<V>) -> Self::Result<()>
    where
        V: ManagedView;
}

pub trait Entity<T>: Sealed {
    type Weak: 'static;

    fn entity_id(&self) -> EntityId;
    fn downgrade(&self) -> Self::Weak;
    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized;
}

pub trait EventEmitter<E: Any>: 'static {}

pub enum GlobalKey {
    Numeric(usize),
    View(EntityId),
    Type(TypeId),
}

pub trait BorrowAppContext {
    fn with_text_style<F, R>(&mut self, style: Option<TextStyleRefinement>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R;

    fn set_global<T: 'static>(&mut self, global: T);
}

impl<C> BorrowAppContext for C
where
    C: BorrowMut<AppContext>,
{
    fn with_text_style<F, R>(&mut self, style: Option<TextStyleRefinement>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        if let Some(style) = style {
            self.borrow_mut().push_text_style(style);
            let result = f(self);
            self.borrow_mut().pop_text_style();
            result
        } else {
            f(self)
        }
    }

    fn set_global<G: 'static>(&mut self, global: G) {
        self.borrow_mut().set_global(global)
    }
}

pub trait Flatten<T> {
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

#[derive(Deref, DerefMut, Eq, PartialEq, Hash, Clone)]
pub struct SharedString(ArcCow<'static, str>);

impl Default for SharedString {
    fn default() -> Self {
        Self(ArcCow::Owned("".into()))
    }
}

impl AsRef<str> for SharedString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for SharedString {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl std::fmt::Debug for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

impl<T: Into<ArcCow<'static, str>>> From<T> for SharedString {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
