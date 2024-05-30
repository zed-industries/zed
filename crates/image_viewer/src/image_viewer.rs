use gpui::{
    canvas, div, fill, img, opaque_grey, point, size, AnyElement, AppContext, Bounds, Context,
    EventEmitter, FocusHandle, FocusableView, Img, InteractiveElement, IntoElement, Model,
    ObjectFit, ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};
use persistence::IMAGE_VIEWER;
use ui::prelude::*;

use project::{Project, ProjectEntryId, ProjectPath};
use std::{ffi::OsStr, path::PathBuf};
use util::ResultExt;
use workspace::{
    item::{Item, ProjectItem, TabContentParams},
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

        // Only open the item if it's a binary image (no SVGs, etc.)
        // Since we do not have a way to toggle to an editor
        if Img::extensions().contains(&ext) && !ext.contains("svg") {
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

    fn tab_content(&self, params: TabContentParams, _cx: &WindowContext) -> AnyElement {
        let title = self
            .path
            .file_name()
            .unwrap_or_else(|| self.path.as_os_str())
            .to_string_lossy()
            .to_string();
        Label::new(title)
            .single_line()
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .italic(params.preview)
            .into_any_element()
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        let item_id = cx.entity_id().as_u64();
        let workspace_id = workspace.database_id();
        let image_path = self.path.clone();

        if let Some(workspace_id) = workspace_id {
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

            cx.new_view(|cx| ImageView {
                path: image_path,
                focus_handle: cx.focus_handle(),
            })
        })
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(|cx| Self {
            path: self.path.clone(),
            focus_handle: cx.focus_handle(),
        }))
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
        let checkered_background = |bounds: Bounds<Pixels>, _, cx: &mut WindowContext| {
            let square_size = 32.0;

            let start_y = bounds.origin.y.0;
            let height = bounds.size.height.0;
            let start_x = bounds.origin.x.0;
            let width = bounds.size.width.0;

            let mut y = start_y;
            let mut x = start_x;
            let mut color_swapper = true;
            // draw checkerboard pattern
            while y <= start_y + height {
                // Keeping track of the grid in order to be resilient to resizing
                let start_swap = color_swapper;
                while x <= start_x + width {
                    let rect =
                        Bounds::new(point(px(x), px(y)), size(px(square_size), px(square_size)));

                    let color = if color_swapper {
                        opaque_grey(0.6, 0.4)
                    } else {
                        opaque_grey(0.7, 0.4)
                    };

                    cx.paint_quad(fill(rect, color));
                    color_swapper = !color_swapper;
                    x += square_size;
                }
                x = start_x;
                color_swapper = !start_swap;
                y += square_size;
            }
        };

        let checkered_background = canvas(|_, _| (), checkered_background)
            .border_2()
            .border_color(cx.theme().styles.colors.border)
            .size_full()
            .absolute()
            .top_0()
            .left_0();

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .child(checkered_background)
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .w_full()
                    // TODO: In browser based Tailwind & Flex this would be h-screen and we'd use w-full
                    .h_full()
                    .child(
                        img(self.path.clone())
                            .object_fit(ObjectFit::ScaleDown)
                            .max_w_full()
                            .max_h_full(),
                    ),
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
