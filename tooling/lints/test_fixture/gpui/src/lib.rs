//! Minimal stand-in for the real `gpui` crate. The lints key off the crate
//! name and the type names, so we only need to reproduce the relevant API
//! surface.

use std::marker::PhantomData;

// --- AppContext ---

pub trait AppContext {
    fn as_app_mut(&mut self) -> &mut App;
}

// --- App ---

pub struct App;

impl AppContext for App {
    fn as_app_mut(&mut self) -> &mut App {
        self
    }
}

// --- Context ---

pub struct Context<'a, T> {
    _marker: PhantomData<&'a mut T>,
}

impl<T> AppContext for Context<'_, T> {
    fn as_app_mut(&mut self) -> &mut App {
        unimplemented!()
    }
}

impl<T> Context<'_, T> {
    pub fn notify(&mut self) {}
}

// --- Window ---

pub struct Window;

// --- Entity ---

pub struct Entity<T> {
    _marker: PhantomData<T>,
}

impl<T> Entity<T> {
    pub fn update<R, C: AppContext>(
        &self,
        _cx: &mut C,
        _f: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> R {
        unimplemented!()
    }

    pub fn read<'a>(&self, _cx: &'a App) -> &'a T {
        unimplemented!()
    }

    pub fn read_with<R>(&self, _cx: &App, _f: impl FnOnce(&T, &App) -> R) -> R {
        unimplemented!()
    }

    pub fn downgrade(&self) -> WeakEntity<T> {
        unimplemented!()
    }
}

// --- WeakEntity ---

pub struct WeakEntity<T> {
    _marker: PhantomData<T>,
}

impl<T> WeakEntity<T> {
    pub fn update<R, C: AppContext>(
        &self,
        _cx: &mut C,
        _f: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> Result<R, ()> {
        unimplemented!()
    }
}

// --- Render traits ---

pub trait IntoElement {}

pub trait Render: 'static + Sized {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement;
}

pub trait RenderOnce: 'static {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement;
}

impl IntoElement for () {}
impl IntoElement for &str {}
