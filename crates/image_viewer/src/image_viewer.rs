use anyhow::Context as _;
use futures::StreamExt;
use gpui::{
    canvas, div, fill, hash, img, opaque_grey, point, size, AnyElement, AppContext,
    AsyncAppContext, Bounds, Context, EventEmitter, FocusHandle, FocusableView, Img,
    InteractiveElement, IntoElement, Model, ObjectFit, ParentElement, Render, Styled, Task, View,
    ViewContext, VisualContext, WeakView, WindowContext,
};
use persistence::IMAGE_VIEWER;
use ui::prelude::*;

use file_icons::FileIcons;
use project::{Fs, PathEventKind, Project, ProjectEntryId, ProjectPath};
use settings::Settings;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use util::ResultExt;
use workspace::{
    item::{Item, ProjectItem, SerializableItem, TabContentParams},
    ItemId, ItemSettings, Pane, Workspace, WorkspaceId,
};

const IMAGE_VIEWER_KIND: &str = "ImageView";

pub struct ImageItem {
    id: ProjectEntryId,
    abs_path: PathBuf,
    project_path: ProjectPath,
    image: Arc<gpui::Image>,
}

impl project::Item for ImageItem {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let project_path = path.clone();
        let project = project.clone();

        let ext = project_path
            .path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase)
            .unwrap_or_default();
        let ext = ext.as_str();

        // Only open the item if it's a binary image (no SVGs, etc.)
        // Since we do not have a way to toggle to an editor
        if Img::extensions().contains(&ext) && !ext.contains("svg") {
            Some(cx.spawn(|mut cx| async move {
                let abs_path = project
                    .read_with(&cx, |project, cx| project.absolute_path(&project_path, cx))?
                    .ok_or_else(|| anyhow::anyhow!("Failed to find the absolute path"))?;

                create_image_item(abs_path, project_path, project, &mut cx).await
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        Some(self.id)
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }
}

async fn create_image_item(
    abs_path: PathBuf,
    project_path: ProjectPath,
    project: Model<Project>,
    cx: &mut AsyncAppContext,
) -> anyhow::Result<Model<ImageItem>> {
    let fs = project.read_with(cx, |project, _| project.fs().clone())?;
    let image = load_image(fs.clone(), &abs_path).await?;

    let id = project
        .update(cx, |project, cx| project.entry_for_path(&project_path, cx))?
        .context("Entry not found")?
        .id;

    cx.update(|cx| {
        cx.new_model(|cx| {
            cx.spawn({
                let abs_path = abs_path.to_path_buf();
                |image_item, mut cx| async move {
                    let (mut image_file_events, _watcher) =
                        fs.watch(&abs_path, Duration::from_millis(100)).await;
                    while let Some(events) = image_file_events.next().await {
                        if let Some(event) = events.last() {
                            if let Some(PathEventKind::Removed) = event.kind {
                                continue;
                            }
                        }

                        if let Some(new_image) = load_image(fs.clone(), &abs_path).await.log_err() {
                            image_item
                                .update(&mut cx, |image_item: &mut ImageItem, _| {
                                    image_item.image = new_image;
                                })
                                .ok();
                        }
                    }
                }
            })
            .detach();
            ImageItem {
                id,
                abs_path,
                project_path,
                image,
            }
        })
    })
}

async fn load_image(fs: Arc<dyn Fs>, abs_path: &Path) -> anyhow::Result<Arc<gpui::Image>> {
    let new_contents = fs
        .load_bytes(&abs_path)
        .await
        .with_context(|| format!("failed to load bytes for path {abs_path:?}"))?;

    let format = image::guess_format(&new_contents)?;

    Ok(Arc::new(gpui::Image {
        id: hash(&new_contents),
        format: match format {
            image::ImageFormat::Png => gpui::ImageFormat::Png,
            image::ImageFormat::Jpeg => gpui::ImageFormat::Jpeg,
            image::ImageFormat::Gif => gpui::ImageFormat::Gif,
            image::ImageFormat::WebP => gpui::ImageFormat::Webp,
            image::ImageFormat::Tiff => gpui::ImageFormat::Tiff,
            image::ImageFormat::Bmp => gpui::ImageFormat::Bmp,
            _ => {
                log::error!("Image format not supported");
                Err(anyhow::anyhow!("Image format not supported"))?
            }
        },
        bytes: new_contents,
    }))
}

pub struct ImageView {
    item: Model<ImageItem>,
    focus_handle: FocusHandle,
}

impl Item for ImageView {
    type Event = ();

    fn for_each_project_item(
        &self,
        cx: &AppContext,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::Item),
    ) {
        f(self.item.entity_id(), self.item.read(cx))
    }

    fn is_singleton(&self, _cx: &AppContext) -> bool {
        true
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        let path = &self.item.read(cx).abs_path;
        let title = path
            .file_name()
            .unwrap_or_else(|| path.as_os_str())
            .to_string_lossy()
            .to_string();
        Label::new(title)
            .single_line()
            .color(params.text_color())
            .italic(params.preview)
            .into_any_element()
    }

    fn tab_icon(&self, cx: &WindowContext) -> Option<Icon> {
        let path = &self.item.read(cx).abs_path;
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(path.as_path(), cx))
            .flatten()
            .map(Icon::from_path)
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
            item: self.item.clone(),
            focus_handle: cx.focus_handle(),
        }))
    }
}

impl SerializableItem for ImageView {
    fn serialized_item_kind() -> &'static str {
        IMAGE_VIEWER_KIND
    }

    fn deserialize(
        project: Model<Project>,
        _workspace: WeakView<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<gpui::Result<View<Self>>> {
        cx.spawn(|_pane, mut cx| async move {
            let image_path = IMAGE_VIEWER
                .get_image_path(item_id, workspace_id)?
                .ok_or_else(|| anyhow::anyhow!("No image path found"))?;

            let (worktree, relative_path) = project
                .update(&mut cx, |project, cx| {
                    project.find_or_create_worktree(image_path.clone(), false, cx)
                })?
                .await
                .context("Path not found")?;
            let worktree_id = worktree.update(&mut cx, |worktree, _cx| worktree.id())?;

            let project_path = ProjectPath {
                worktree_id,
                path: relative_path.into(),
            };

            let item = create_image_item(image_path, project_path, project, &mut cx).await?;

            cx.update(|cx| {
                Ok(cx.new_view(|cx| ImageView {
                    item,
                    focus_handle: cx.focus_handle(),
                }))
            })?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        cx: &mut WindowContext,
    ) -> Task<gpui::Result<()>> {
        cx.spawn(|_| IMAGE_VIEWER.delete_unloaded_items(workspace_id, alive_items))
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        cx: &mut ViewContext<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;

        Some(cx.background_executor().spawn({
            let image_path = self.item.read(cx).abs_path.clone();
            async move {
                IMAGE_VIEWER
                    .save_image_path(item_id, workspace_id, image_path)
                    .await
            }
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
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
        let image = self.item.read(cx).image.clone();
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
                        img(image)
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
            item,
            focus_handle: cx.focus_handle(),
        }
    }
}

pub fn init(cx: &mut AppContext) {
    workspace::register_project_item::<ImageView>(cx);
    workspace::register_serializable_item::<ImageView>(cx);
}

mod persistence {
    use anyhow::Result;
    use std::path::PathBuf;

    use db::{define_connection, query, sqlez::statement::Statement, sqlez_macros::sql};
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

        pub async fn delete_unloaded_items(
            &self,
            workspace: WorkspaceId,
            alive_items: Vec<ItemId>,
        ) -> Result<()> {
            let placeholders = alive_items
                .iter()
                .map(|_| "?")
                .collect::<Vec<&str>>()
                .join(", ");

            let query = format!("DELETE FROM image_viewers WHERE workspace_id = ? AND item_id NOT IN ({placeholders})");

            self.write(move |conn| {
                let mut statement = Statement::prepare(conn, query)?;
                let mut next_index = statement.bind(&workspace, 1)?;
                for id in alive_items {
                    next_index = statement.bind(&id, next_index)?;
                }
                statement.exec()
            })
            .await
        }
    }
}
