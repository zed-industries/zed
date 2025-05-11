mod image_info;
mod image_viewer_settings;

use std::path::PathBuf;

use anyhow::Context as _;
use editor::{EditorSettings, items::entry_git_aware_label_color};
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, Bounds, Context, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ObjectFit, ParentElement, Render, Styled, Task, WeakEntity,
    Window, canvas, div, fill, img, opaque_grey, point, size,
};
use persistence::IMAGE_VIEWER;
use project::{ImageItem, Project, ProjectPath, image_store::ImageItemEvent};
use settings::Settings;
use theme::Theme;
use ui::prelude::*;
use util::paths::PathExt;
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemLocation, Workspace, WorkspaceId, delete_unloaded_items,
    item::{BreadcrumbText, Item, ProjectItem, SerializableItem, TabContentParams},
};

pub use crate::image_info::*;
pub use crate::image_viewer_settings::*;

pub struct ImageView {
    image_item: Entity<ImageItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
}

impl ImageView {
    pub fn new(
        image_item: Entity<ImageItem>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&image_item, Self::on_image_event).detach();
        cx.on_release_in(window, |this, window, cx| {
            let image_data = this.image_item.read(cx).image.clone();
            if let Some(image) = image_data.clone().get_render_image(window, cx) {
                cx.drop_image(image, None);
            }
            image_data.remove_asset(cx);
        })
        .detach();

        Self {
            image_item,
            project,
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_image_event(
        &mut self,
        _: Entity<ImageItem>,
        event: &ImageItemEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            ImageItemEvent::MetadataUpdated
            | ImageItemEvent::FileHandleChanged
            | ImageItemEvent::Reloaded => {
                cx.emit(ImageViewEvent::TitleChanged);
                cx.notify();
            }
            ImageItemEvent::ReloadNeeded => {}
        }
    }
}

pub enum ImageViewEvent {
    TitleChanged,
}

impl EventEmitter<ImageViewEvent> for ImageView {}

impl Item for ImageView {
    type Event = ImageViewEvent;

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        match event {
            ImageViewEvent::TitleChanged => {
                f(workspace::item::ItemEvent::UpdateTab);
                f(workspace::item::ItemEvent::UpdateBreadcrumbs);
            }
        }
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        f(self.image_item.entity_id(), self.image_item.read(cx))
    }

    fn is_singleton(&self, _cx: &App) -> bool {
        true
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let abs_path = self.image_item.read(cx).file.as_local()?.abs_path(cx);
        let file_path = abs_path.compact().to_string_lossy().to_string();
        Some(file_path.into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        let project_path = self.image_item.read(cx).project_path(cx);

        let label_color = if ItemSettings::get_global(cx).git_status {
            let git_status = self
                .project
                .read(cx)
                .project_path_git_status(&project_path, cx)
                .map(|status| status.summary())
                .unwrap_or_default();

            self.project
                .read(cx)
                .entry_for_path(&project_path, cx)
                .map(|entry| {
                    entry_git_aware_label_color(git_status, entry.is_ignored, params.selected)
                })
                .unwrap_or_else(|| params.text_color())
        } else {
            params.text_color()
        };

        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .single_line()
            .color(label_color)
            .when(params.preview, |this| this.italic())
            .into_any_element()
    }

    fn tab_content_text(&self, _: usize, cx: &App) -> SharedString {
        self.image_item
            .read(cx)
            .file
            .file_name(cx)
            .to_string_lossy()
            .to_string()
            .into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = self.image_item.read(cx).path();
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(path, cx))
            .flatten()
            .map(Icon::from_path)
    }

    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation {
        let show_breadcrumb = EditorSettings::get_global(cx).toolbar.breadcrumbs;
        if show_breadcrumb {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, _theme: &Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        let text = breadcrumbs_text_for_image(self.project.read(cx), self.image_item.read(cx), cx);
        Some(vec![BreadcrumbText {
            text,
            highlights: None,
            font: None,
        }])
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| Self {
            image_item: self.image_item.clone(),
            project: self.project.clone(),
            focus_handle: cx.focus_handle(),
        }))
    }
}

fn breadcrumbs_text_for_image(project: &Project, image: &ImageItem, cx: &App) -> String {
    let path = image.file.file_name(cx);
    if project.visible_worktrees(cx).count() <= 1 {
        return path.to_string_lossy().to_string();
    }

    project
        .worktree_for_id(image.project_path(cx).worktree_id, cx)
        .map(|worktree| {
            PathBuf::from(worktree.read(cx).root_name())
                .join(path)
                .to_string_lossy()
                .to_string()
        })
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}

impl SerializableItem for ImageView {
    fn serialized_item_kind() -> &'static str {
        "ImageView"
    }

    fn deserialize(
        project: Entity<Project>,
        _workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            let image_path = IMAGE_VIEWER
                .get_image_path(item_id, workspace_id)?
                .ok_or_else(|| anyhow::anyhow!("No image path found"))?;

            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(image_path.clone(), false, cx)
                })?
                .await
                .context("Path not found")?;
            let worktree_id = worktree.update(cx, |worktree, _cx| worktree.id())?;

            let project_path = ProjectPath {
                worktree_id,
                path: relative_path.into(),
            };

            let image_item = project
                .update(cx, |project, cx| project.open_image(project_path, cx))?
                .await?;

            cx.update(
                |window, cx| Ok(cx.new(|cx| ImageView::new(image_item, project, window, cx))),
            )?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "image_viewers",
            &IMAGE_VIEWER,
            cx,
        )
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let image_path = self.image_item.read(cx).file.as_local()?.abs_path(cx);

        Some(cx.background_spawn({
            async move {
                log::debug!("Saving image at path {image_path:?}");
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
impl Focusable for ImageView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ImageView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let image = self.image_item.read(cx).image.clone();
        let checkered_background = |bounds: Bounds<Pixels>,
                                    _,
                                    window: &mut Window,
                                    _cx: &mut App| {
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

                    window.paint_quad(fill(rect, color));
                    color_swapper = !color_swapper;
                    x += square_size;
                }
                x = start_x;
                color_swapper = !start_swap;
                y += square_size;
            }
        };

        let checkered_background = canvas(|_, _, _| (), checkered_background)
            .border_2()
            .border_color(cx.theme().styles.colors.border)
            .size_full()
            .absolute()
            .top_0()
            .left_0();

        div()
            .track_focus(&self.focus_handle(cx))
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
                            .max_h_full()
                            .id("img"),
                    ),
            )
    }
}

impl ProjectItem for ImageView {
    type Item = ImageItem;

    fn for_project_item(
        project: Entity<Project>,
        _: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self::new(item, project, window, cx)
    }
}

pub fn init(cx: &mut App) {
    ImageViewerSettings::register(cx);
    workspace::register_project_item::<ImageView>(cx);
    workspace::register_serializable_item::<ImageView>(cx);
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
