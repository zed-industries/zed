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
use std::marker::PhantomData;

pub use app::*;
pub use element::*;
pub use elements::*;
pub use geometry::*;
pub use style::*;
use taffy::TaffyLayoutEngine;
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
    filter_editor: Handle<Editor>,
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

fn field<S>(editor: Handle<Editor>) -> EditorElement<S> {
    EditorElement {
        editor,
        field: true,
        placeholder_text: None,
        parent_state: PhantomData,
    }
}

struct EditorElement<S> {
    editor: Handle<Editor>,
    field: bool,
    placeholder_text: Option<SharedString>,
    parent_state: PhantomData<S>,
}

impl<S> EditorElement<S> {
    pub fn field(mut self) -> Self {
        self.field = true;
        self
    }

    pub fn placeholder_text(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder_text = Some(text.into());
        self
    }
}

impl<S: 'static> Element for EditorElement<S> {
    type State = S;
    type FrameState = ();

    fn layout(
        &mut self,
        _: &mut Self::State,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<(LayoutId, Self::FrameState)> {
        self.editor.update(cx, |editor, cx| todo!())
    }

    fn paint(
        &mut self,
        layout: Layout,
        state: &mut Self::State,
        frame_state: &mut Self::FrameState,
        cx: &mut ViewContext<Self::State>,
    ) -> Result<()> {
        self.editor.update(cx, |editor, cx| todo!())
    }
}

struct Editor {}

impl Editor {
    pub fn new(_: &mut ViewContext<Self>) -> Self {
        Editor {}
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
