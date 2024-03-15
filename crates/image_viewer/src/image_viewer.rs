#![allow(unused_imports)]
use gpui::{
    actions, canvas, div, fill, green, img, impl_actions, periwinkle, point, quad, size, white,
    Action, AnyElement, AnyView, AnyWeakView, AppContext, AsyncAppContext, AsyncWindowContext,
    Bounds, Context, Div, DragMoveEvent, Element, ElementContext, Empty, Entity, EntityId,
    EventEmitter, FocusHandle, FocusableView, Global, GlobalPixels, InteractiveElement,
    IntoElement, KeyContext, Keystroke, LayoutId, ManagedView, Model, ModelContext, ParentElement,
    PathPromptOptions, Pixels, Point, PromptLevel, Render, SharedString, SharedUri, Size, Styled,
    Subscription, Task, View, ViewContext, VisualContext, WeakView, WindowContext, WindowHandle,
    WindowOptions,
};
use persistence::IMAGE_VIEWER;
use ui::{
    h_flex,
    prelude::*,
    utils::{DateTimeType, FormatDistance},
    v_flex, ButtonLike, Tab, TabBar, Tooltip,
};

use project::{Project, ProjectEntryId, ProjectPath};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, path::PathBuf};
use util::ResultExt;
use workspace::{
    item::{Item, ProjectItem},
    ItemId, Pane, Workspace, WorkspaceId,
};

const IMAGE_VIEWER_KIND: &str = "ImageView";

pub struct ImageItem {
    path: PathBuf,
    project_path: ProjectPath,
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
            Some(cx.spawn(|mut cx| async move {
                let abs_path = project
                    .read_with(&cx, |project, cx| project.absolute_path(&path, cx))?
                    .ok_or_else(|| anyhow::anyhow!("Failed to find the absolute path"))?;

                cx.new_model(|_| ImageItem {
                    path: abs_path,
                    project_path: path,
                })
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        None
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }
}

pub struct ImageView {
    path: PathBuf,
    focus_handle: FocusHandle,
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
            .file_name()
            .unwrap_or_else(|| self.path.as_os_str())
            .to_string_lossy()
            .to_string()
            .into_any_element()
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        let item_id = cx.entity_id().as_u64();
        let workspace_id = workspace.database_id();
        let image_path = self.path.clone();

        cx.background_executor()
            .spawn({
                let image_path = image_path.clone();
                async move {
                    IMAGE_VIEWER
                        .save_image_path(item_id, workspace_id, image_path)
                        .await
                        .log_err();
                }
            })
            .detach();
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some(IMAGE_VIEWER_KIND)
    }

    fn deserialize(
        _project: Model<Project>,
        _workspace: WeakView<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<anyhow::Result<View<Self>>> {
        cx.spawn(|_pane, mut cx| async move {
            let image_path = IMAGE_VIEWER
                .get_image_path(item_id, workspace_id)?
                .ok_or_else(|| anyhow::anyhow!("No image path found"))?;

            Ok(cx.new_view(|cx| ImageView {
                path: image_path,
                focus_handle: cx.focus_handle(),
            })?)
        })
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
        let im = img(self.path.clone()).into_any();

        //
        // Centered image.
        // checkboard pattern behind wherever transparent
        //
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .child(
                canvas(
                    |_, _| (),
                    |bounds, _, cx| {
                        // let square_size = 10.0;

                        let left_bounds = Bounds::from_corners(
                            bounds.origin,
                            point(bounds.center().x, bounds.bottom()),
                        );
                        let right_bounds = Bounds::from_corners(
                            point(bounds.center().x, bounds.top()),
                            bounds.lower_right(),
                        );

                        cx.paint_quad(fill(left_bounds, periwinkle()));
                        cx.paint_quad(fill(right_bounds, green()));
                    },
                )
                .size_full()
                .absolute()
                .top_0()
                .left_0(),
            )
            .child(
                v_flex()
                    .justify_around()
                    .child(h_flex().justify_around().child(im)),
            )
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
            focus_handle: cx.focus_handle(),
        }
    }
}

pub fn init(cx: &mut AppContext) {
    workspace::register_project_item::<ImageView>(cx);
    workspace::register_deserializable_item::<ImageView>(cx)
}

mod persistence {
    use std::path::PathBuf;

    use db::{define_connection, query, sqlez_macros::sql};
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    define_connection! {
        pub static ref IMAGE_VIEWER: ImageViewerDb<WorkspaceDb> =
            &[sql!(
                CREATE TABLE image_viewers (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    image_path BLOB,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
            )];
    }

    impl ImageViewerDb {
        query! {
           pub async fn update_workspace_id(
                new_id: WorkspaceId,
                old_id: WorkspaceId,
                item_id: ItemId
            ) -> Result<()> {
                UPDATE image_viewers
                SET workspace_id = ?
                WHERE workspace_id = ? AND item_id = ?
            }
        }

        query! {
            pub async fn save_image_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                image_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO image_viewers(item_id, workspace_id, image_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_image_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT image_path
                FROM image_viewers
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
