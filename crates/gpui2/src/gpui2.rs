mod action;
mod app;
mod assets;
mod color;
mod element;
mod elements;
mod executor;
mod focusable;
mod geometry;
mod image_cache;
mod interactive;
mod keymap;
mod platform;
mod scene;
mod style;
mod styled;
mod subscription;
mod svg_renderer;
mod taffy;
#[cfg(any(test, feature = "test-support"))]
mod test;
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
pub use element::*;
pub use elements::*;
pub use executor::*;
pub use focusable::*;
pub use geometry::*;
pub use gpui2_macros::*;
pub use image_cache::*;
pub use interactive::*;
pub use keymap::*;
pub use platform::*;
use private::Sealed;
pub use refineable::*;
pub use scene::*;
pub use serde;
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
    ops::{Deref, DerefMut},
};
use taffy::TaffyLayoutEngine;

type AnyBox = Box<dyn Any>;

pub trait Context {
    type ModelContext<'a, T>;
    type Result<T>;

    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut Self::ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    fn update_model<T: 'static, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut Self::ModelContext<'_, T>) -> R,
    ) -> Self::Result<R>;
}

pub trait VisualContext: Context {
    type ViewContext<'a, 'w, V>;

    fn build_view<V>(
        &mut self,
        build_view_state: impl FnOnce(&mut Self::ViewContext<'_, '_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static;

    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut Self::ViewContext<'_, '_, V>) -> R,
    ) -> Self::Result<R>;
}

pub trait Entity<T>: Sealed {
    type Weak: 'static + Send;

    fn entity_id(&self) -> EntityId;
    fn downgrade(&self) -> Self::Weak;
    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized;
}

pub enum GlobalKey {
    Numeric(usize),
    View(EntityId),
    Type(TypeId),
}

pub trait BorrowAppContext {
    fn with_text_style<F, R>(&mut self, style: TextStyleRefinement, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R;

    fn set_global<T: Send + 'static>(&mut self, global: T);
}

impl<C> BorrowAppContext for C
where
    C: BorrowMut<AppContext>,
{
    fn with_text_style<F, R>(&mut self, style: TextStyleRefinement, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.borrow_mut().push_text_style(style);
        let result = f(self);
        self.borrow_mut().pop_text_style();
        result
    }

    fn set_global<G: 'static + Send>(&mut self, global: G) {
        self.borrow_mut().set_global(global)
    }
}

pub trait EventEmitter: 'static {
    type Event: Any;
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

pub enum Reference<'a, T> {
    Immutable(&'a T),
    Mutable(&'a mut T),
}

impl<'a, T> Deref for Reference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Reference::Immutable(target) => target,
            Reference::Mutable(target) => target,
        }
    }
}

impl<'a, T> DerefMut for Reference<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Reference::Immutable(_) => {
                panic!("cannot mutably deref an immutable reference. this is a bug in GPUI.");
            }
            Reference::Mutable(target) => target,
        }
    }
}
