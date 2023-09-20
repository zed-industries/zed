mod app;
mod color;
mod element;
mod elements;
mod executor;
mod fonts;
mod geometry;
mod platform;
mod renderer;
mod scene;
mod style;
mod styled;
mod taffy;
mod text;
mod util;
mod window;

use anyhow::Result;
pub use app::*;
pub use color::*;
pub use element::*;
pub use elements::*;
pub use executor::*;
pub use fonts::*;
pub use geometry::*;
pub use platform::*;
pub use refineable::*;
pub use scene::*;
pub use smallvec;
pub use smol::Timer;
use std::ops::{Deref, DerefMut};
pub use style::*;
pub use styled::*;
pub use taffy::LayoutId;
use taffy::TaffyLayoutEngine;
use text::*;
pub use util::arc_cow::ArcCow;
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

#[cfg(test)]
mod tests {
    use super::*;

    struct Workspace {
        left_panel: AnyView<Self>,
    }

    fn workspace(cx: &mut WindowContext) -> View<Workspace> {
        let workspace = cx.entity(|cx| Workspace {
            left_panel: collab_panel(cx).into_any(),
        });
        view(workspace, |workspace, _cx| {
            div().child(workspace.left_panel.clone())
        })
    }

    struct CollabPanel {
        filter_editor: Handle<editor::Editor>,
    }

    fn collab_panel(cx: &mut WindowContext) -> View<CollabPanel> {
        let panel = cx.entity(|cx| CollabPanel::new(cx));
        view(panel, |panel, _cx| {
            div().child(div()).child(
                field(panel.filter_editor.clone()).placeholder_text("Search channels, contacts"),
            )
        })
    }

    impl CollabPanel {
        fn new(cx: &mut ViewContext<Self>) -> Self {
            Self {
                filter_editor: cx.entity(|cx| editor::Editor::new(cx)),
            }
        }
    }

    #[test]
    fn test() {
        let mut cx = AppContext::test();

        cx.open_window(WindowOptions::default(), |cx| workspace(cx));
    }
}
