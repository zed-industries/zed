mod app;
mod color;
mod element;
mod elements;
mod executor;
mod geometry;
mod platform;
mod renderer;
mod scene;
mod style;
mod style_helpers;
mod styled;
mod taffy;
mod text_system;
mod util;
mod view;
mod window;

pub use anyhow::Result;
pub use app::*;
pub use color::*;
pub use element::*;
pub use elements::*;
pub use executor::*;
pub use geometry::*;
pub use gpui3_macros::*;
pub use platform::*;
pub use refineable::*;
pub use scene::*;
pub use serde;
pub use serde_json;
pub use smallvec;
pub use smol::Timer;
use std::ops::{Deref, DerefMut};
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
    type EntityContext<'a, 'w, T: 'static>;

    fn entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T>;

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R;
}

#[derive(Clone, Eq, PartialEq)]
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
