mod app;
mod assets;
mod color;
mod element;
mod elements;
mod executor;
mod geometry;
mod image_cache;
mod platform;
mod scene;
mod style;
mod style_helpers;
mod styled;
mod svg_renderer;
mod taffy;
mod text_system;
mod util;
mod view;
mod window;

pub use anyhow::Result;
pub use app::*;
pub use assets::*;
pub use color::*;
pub use element::*;
pub use elements::*;
pub use executor::*;
pub use geometry::*;
pub use gpui3_macros::*;
pub use svg_renderer::*;

pub use platform::*;
pub use refineable::*;
pub use scene::*;
pub use serde;
pub use serde_json;
pub use smallvec;
pub use smol::Timer;
use std::{
    mem,
    ops::{Deref, DerefMut},
    sync::Arc,
};
pub use style::*;
pub use style_helpers::*;
pub use styled::*;
use taffy::TaffyLayoutEngine;
pub use taffy::{AvailableSpace, LayoutId};
pub use text_system::*;
pub use util::arc_cow::ArcCow;
pub use view::*;
pub use window::*;

pub trait Context {
    type EntityContext<'a, 'w, T: 'static + Send + Sync>;
    type Result<T>;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Self::Result<Handle<T>>;

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> Self::Result<R>;
}

#[repr(transparent)]
pub struct MainThread<T>(T);

impl<T> MainThread<T> {
    fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for MainThread<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MainThread<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<C: Context> Context for MainThread<C> {
    type EntityContext<'a, 'w, T: 'static + Send + Sync> = MainThread<C::EntityContext<'a, 'w, T>>;
    type Result<T> = C::Result<T>;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Self::Result<Handle<T>> {
        self.0.entity(|cx| {
            let cx = unsafe {
                mem::transmute::<
                    &mut C::EntityContext<'_, '_, T>,
                    &mut MainThread<C::EntityContext<'_, '_, T>>,
                >(cx)
            };
            build_entity(cx)
        })
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> Self::Result<R> {
        self.0.update_entity(handle, |entity, cx| {
            let cx = unsafe {
                mem::transmute::<
                    &mut C::EntityContext<'_, '_, T>,
                    &mut MainThread<C::EntityContext<'_, '_, T>>,
                >(cx)
            };
            update(entity, cx)
        })
    }
}

pub trait BorrowAppContext {
    fn app_mut(&mut self) -> &mut AppContext;

    fn with_text_style<F, R>(&mut self, style: TextStyleRefinement, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.app_mut().push_text_style(style);
        let result = f(self);
        self.app_mut().pop_text_style();
        result
    }

    fn with_state<T: Send + Sync + 'static, F, R>(&mut self, state: T, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.app_mut().push_state(state);
        let result = f(self);
        self.app_mut().pop_state::<T>();
        result
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

#[derive(Clone, Eq, PartialEq, Hash)]
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

pub(crate) struct MainThreadOnly<T: ?Sized> {
    executor: Executor,
    value: Arc<T>,
}

impl<T: ?Sized> Clone for MainThreadOnly<T> {
    fn clone(&self) -> Self {
        Self {
            executor: self.executor.clone(),
            value: self.value.clone(),
        }
    }
}

/// Allows a value to be accessed only on the main thread, allowing a non-`Send` type
/// to become `Send`.
impl<T: 'static + ?Sized> MainThreadOnly<T> {
    pub(crate) fn new(value: Arc<T>, executor: Executor) -> Self {
        Self { executor, value }
    }

    pub(crate) fn borrow_on_main_thread(&self) -> &T {
        assert!(self.executor.is_main_thread());
        &self.value
    }
}

unsafe impl<T: ?Sized> Send for MainThreadOnly<T> {}
