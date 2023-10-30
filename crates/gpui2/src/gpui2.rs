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
    mem,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use taffy::TaffyLayoutEngine;

type AnyBox = Box<dyn Any + Send>;

pub trait Context {
    type EntityContext<'a, T>;
    type Result<T>;

    fn entity<T>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, T>) -> T,
    ) -> Self::Result<Handle<T>>
    where
        T: 'static + Send;

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, T>) -> R,
    ) -> Self::Result<R>;
}

pub trait VisualContext: Context {
    type ViewContext<'a, 'w, V>;

    fn build_view<E, V>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::ViewContext<'_, '_, V>) -> V,
        render: impl Fn(&mut V, &mut ViewContext<'_, '_, V>) -> E + Send + 'static,
    ) -> Self::Result<View<V>>
    where
        E: Component<V>,
        V: 'static + Send;

    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut Self::ViewContext<'_, '_, V>) -> R,
    ) -> Self::Result<R>;
}

pub enum GlobalKey {
    Numeric(usize),
    View(EntityId),
    Type(TypeId),
}

#[repr(transparent)]
pub struct MainThread<T>(T);

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
    type EntityContext<'a, T> = MainThread<C::EntityContext<'a, T>>;
    type Result<T> = C::Result<T>;

    fn entity<T>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, T>) -> T,
    ) -> Self::Result<Handle<T>>
    where
        T: 'static + Send,
    {
        self.0.entity(|cx| {
            let cx = unsafe {
                mem::transmute::<
                    &mut C::EntityContext<'_, T>,
                    &mut MainThread<C::EntityContext<'_, T>>,
                >(cx)
            };
            build_entity(cx)
        })
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        self.0.update_entity(handle, |entity, cx| {
            let cx = unsafe {
                mem::transmute::<
                    &mut C::EntityContext<'_, T>,
                    &mut MainThread<C::EntityContext<'_, T>>,
                >(cx)
            };
            update(entity, cx)
        })
    }
}

impl<C: VisualContext> VisualContext for MainThread<C> {
    type ViewContext<'a, 'w, V> = MainThread<C::ViewContext<'a, 'w, V>>;

    fn build_view<E, V>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::ViewContext<'_, '_, V>) -> V,
        render: impl Fn(&mut V, &mut ViewContext<'_, '_, V>) -> E + Send + 'static,
    ) -> Self::Result<View<V>>
    where
        E: Component<V>,
        V: 'static + Send,
    {
        self.0.build_view(
            |cx| {
                let cx = unsafe {
                    mem::transmute::<
                        &mut C::ViewContext<'_, '_, V>,
                        &mut MainThread<C::ViewContext<'_, '_, V>>,
                    >(cx)
                };
                build_entity(cx)
            },
            render,
        )
    }

    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut Self::ViewContext<'_, '_, V>) -> R,
    ) -> Self::Result<R> {
        self.0.update_view(view, |view_state, cx| {
            let cx = unsafe {
                mem::transmute::<
                    &mut C::ViewContext<'_, '_, V>,
                    &mut MainThread<C::ViewContext<'_, '_, V>>,
                >(cx)
            };
            update(view_state, cx)
        })
    }
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
