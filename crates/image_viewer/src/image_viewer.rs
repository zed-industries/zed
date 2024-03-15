#![allow(unused_imports)]
use gpui::{
    actions, canvas, div, impl_actions, point, size, Action, AnyElement, AnyView, AnyWeakView,
    AppContext, AsyncAppContext, AsyncWindowContext, Bounds, Context, Div, DragMoveEvent, Element,
    ElementContext, Empty, Entity, EntityId, EventEmitter, FocusHandle, FocusableView, Global,
    GlobalPixels, InteractiveElement, IntoElement, KeyContext, Keystroke, LayoutId, ManagedView,
    Model, ModelContext, ParentElement, PathPromptOptions, Pixels, Point, PromptLevel, Render,
    SharedString, Size, Styled, Subscription, Task, View, ViewContext, VisualContext, WeakView,
    WindowContext, WindowHandle, WindowOptions,
};

use std::ffi::OsStr;

use project::{Project, ProjectEntryId, ProjectPath};
use workspace::item::{Item, ProjectItem};

const TEST_PNG_KIND: &str = "TestPngItemView";

pub struct ImageView {
    focus_handle: FocusHandle,
}

pub struct ImageItem {}

impl project::Item for ImageItem {
    fn try_open(
        _project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let ext = path
            .path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();
        if ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "ico"].contains(&ext) {
            Some(cx.spawn(|mut cx| async move { cx.new_model(|_| ImageItem {}) }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        None
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        None
    }
}

impl Item for ImageView {
    type Event = ();

    fn serialized_item_kind() -> Option<&'static str> {
        Some(TEST_PNG_KIND)
    }
}
impl EventEmitter<()> for ImageView {}
impl FocusableView for ImageView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ImageView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        Empty
    }
}

impl ProjectItem for ImageView {
    type Item = ImageItem;

    fn for_project_item(
        _project: Model<Project>,
        _item: Model<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

pub fn init(cx: &mut AppContext) {
    workspace::register_project_item::<ImageView>(cx);
}
