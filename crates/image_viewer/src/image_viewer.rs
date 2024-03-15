#![allow(unused_imports)]
use gpui::{
    actions, canvas, div, img, impl_actions, periwinkle, point, size, Action, AnyElement, AnyView,
    AnyWeakView, AppContext, AsyncAppContext, AsyncWindowContext, Bounds, Context, Div,
    DragMoveEvent, Element, ElementContext, Empty, Entity, EntityId, EventEmitter, FocusHandle,
    FocusableView, Global, GlobalPixels, InteractiveElement, IntoElement, KeyContext, Keystroke,
    LayoutId, ManagedView, Model, ModelContext, ParentElement, PathPromptOptions, Pixels, Point,
    PromptLevel, Render, SharedString, SharedUri, Size, Styled, Subscription, Task, View,
    ViewContext, VisualContext, WeakView, WindowContext, WindowHandle, WindowOptions,
};
use ui::{
    h_flex,
    prelude::*,
    utils::{DateTimeType, FormatDistance},
    v_flex, ButtonLike, Tab, TabBar, Tooltip,
};

use project::{Project, ProjectEntryId, ProjectPath};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use workspace::item::{Item, ProjectItem};

const _IMAGE_VIEWER_KIND: &str = "ImageView";

pub struct ImageView {
    path: ProjectPath,
    project: Model<Project>,
    focus_handle: FocusHandle,
}

pub struct ImageItem {
    path: ProjectPath,
    project: Model<Project>,
}

impl project::Item for ImageItem {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let path = path.clone();
        let project = project.clone();

        let ext = path
            .path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();
        if ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "ico"].contains(&ext) {
            Some(cx.spawn(|mut cx| async move { cx.new_model(|_| ImageItem { path, project }) }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        None
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        Some(self.path.clone())
    }
}

impl Item for ImageView {
    type Event = ();

    fn tab_content(
        &self,
        _detail: Option<usize>,
        _selected: bool,
        _cx: &WindowContext,
    ) -> AnyElement {
        self.path
            .path
            .file_name()
            .unwrap_or_else(|| self.path.path.as_os_str())
            .to_string_lossy()
            .to_string()
            .into_any_element()
    }
}

impl EventEmitter<()> for ImageView {}
impl FocusableView for ImageView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ImageView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let path = self.project.read(cx).absolute_path(&self.path, cx);

        let im = path
            .map(|path| img(path).into_any())
            .unwrap_or_else(|| "No image found".into_any());

        div().size_full().bg(periwinkle()).child(im)
    }
}

impl ProjectItem for ImageView {
    type Item = ImageItem;

    fn for_project_item(
        _project: Model<Project>,
        item: Model<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            path: item.read(cx).path.clone(),
            project: item.read(cx).project.clone(),
            focus_handle: cx.focus_handle(),
        }
    }
}

pub fn init(cx: &mut AppContext) {
    workspace::register_project_item::<ImageView>(cx);
}
