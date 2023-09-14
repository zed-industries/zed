mod app;
mod element;
mod elements;
mod geometry;
mod style;
mod taffy;
mod window;

use anyhow::Result;
pub use gpui2::ArcCow;
use gpui2::Reference;

pub use app::*;
pub use element::*;
pub use elements::*;
pub use geometry::*;
pub use style::*;
pub use taffy::LayoutId;
use taffy::TaffyLayoutEngine;
pub use window::*;

use self::editor::Editor;

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

pub struct SharedString(ArcCow<'static, str>);

impl<T: Into<ArcCow<'static, str>>> From<T> for SharedString {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

struct Workspace {
    left_panel: AnyView<Self>,
}

fn workspace(cx: &mut WindowContext) -> View<Workspace> {
    let workspace = cx.entity(|cx| Workspace {
        left_panel: collab_panel(cx).into_any(),
    });
    view(workspace, |workspace, cx| {
        div().child(workspace.left_panel.clone())
    })
}

struct CollabPanel {
    filter_editor: Handle<editor::Editor>,
}

fn collab_panel(cx: &mut WindowContext) -> View<CollabPanel> {
    let panel = cx.entity(|cx| CollabPanel::new(cx));
    view(panel, |panel, cx| {
        div()
            .child(div())
            .child(field(panel.filter_editor.clone()).placeholder_text("Search channels, contacts"))
    })
}

impl CollabPanel {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            filter_editor: cx.entity(|cx| Editor::new(cx)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut cx = AppContext::new();
        cx.open_window(|cx| workspace(cx));
    }
}
