mod image_info;
mod image_viewer_settings;
pub mod zoom_controls;

use std::path::PathBuf;

use anyhow::Context as _;
use editor::items::entry_git_aware_label_color;
use file_icons::FileIcons;
use gpui::{
    canvas, div, fill, img, opaque_grey, point, size, AnyElement, App, Bounds, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ObjectFit, ParentElement, Point, Render,
    ScrollDelta, ScrollWheelEvent, Size, Styled, Task, WeakEntity, Window,
};
use persistence::IMAGE_VIEWER;
use project::{image_store::ImageItemEvent, ImageItem, Project, ProjectPath};
use settings::Settings;
use theme::Theme;
use ui::prelude::*;
use util::paths::PathExt;
use workspace::{
    item::{BreadcrumbText, Item, ProjectItem, SerializableItem, TabContentParams},
    ItemId, ItemSettings, ToolbarItemLocation, Workspace, WorkspaceId,
};

pub use crate::image_info::*;
pub use crate::image_viewer_settings::*;

pub struct ImageView {
    image_item: Entity<ImageItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    pan_offset: Point<Pixels>,
    zoom_level: f32,
    is_panning: bool,
    initial_layout: Option<Size<Pixels>>,
    last_mouse_position: Option<Point<Pixels>>,
}

impl ImageView {
    pub fn new(
        image_item: Entity<ImageItem>,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(&image_item, Self::on_image_event).detach();
        Self {
            image_item,
            project,
            focus_handle: cx.focus_handle(),
            pan_offset: Point::default(),
            zoom_level: 1.0,
            is_panning: false,
            initial_layout: None,
            last_mouse_position: None,
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

    pub fn update_zoom(&mut self, new_zoom: f32, center: Point<Pixels>, cx: &mut Context<Self>) {
        let content_size = self.content_size(cx);
        let scaled_width = content_size.width.0 * new_zoom;
        let scaled_height = content_size.height.0 * new_zoom;

        self.pan_offset = Point {
            x: center.x - px(scaled_width / 2.0),
            y: center.y - px(scaled_height / 2.0),
        };
        self.zoom_level = new_zoom;
        cx.refresh_windows();
    }

    pub fn get_center_point(&self, window_size: Size<Pixels>) -> Point<Pixels> {
        Point::new(window_size.width / 2.0, window_size.height / 2.0)
    }

    pub fn reset_view(&mut self, window_size: Size<Pixels>, cx: &mut Context<Self>) {
        let content_size = self.content_size(cx);
        let center = Point::new(window_size.width / 2.0, window_size.height / 2.0);

        self.pan_offset = Point {
            x: center.x - content_size.width / 2.0,
            y: center.y - content_size.height / 2.0,
        };
        self.zoom_level = 1.0;
        cx.refresh_windows();
    }

    fn content_size(&self, cx: &mut Context<Self>) -> Size<Pixels> {
        self.image_item
            .read(cx)
            .image_metadata
            .as_ref()
            .map(|metadata| Size {
                width: px(metadata.width as f32),
                height: px(metadata.height as f32),
            })
            .unwrap_or_default()
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

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> AnyElement {
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

        let title = self
            .image_item
            .read(cx)
            .file
            .file_name(cx)
            .to_string_lossy()
            .to_string();
        Label::new(title)
            .single_line()
            .color(label_color)
            .italic(params.preview)
            .into_any_element()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = self.image_item.read(cx).path();
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(path, cx))
            .flatten()
            .map(Icon::from_path)
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
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
            initial_layout: self.initial_layout,
            zoom_level: self.zoom_level,
            is_panning: self.is_panning,
            last_mouse_position: self.last_mouse_position,
            pan_offset: self.pan_offset,
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
        window.spawn(cx, |mut cx| async move {
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

            let image_item = project
                .update(&mut cx, |project, cx| project.open_image(project_path, cx))?
                .await?;

            cx.update(|_, cx| Ok(cx.new(|cx| ImageView::new(image_item, project, cx))))?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        window.spawn(cx, |_| {
            IMAGE_VIEWER.delete_unloaded_items(workspace_id, alive_items)
        })
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

        Some(cx.background_executor().spawn({
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
impl Focusable for ImageView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ImageView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let image = self.image_item.read(cx).image.clone();
        let metadata = self.image_item.read(cx).image_metadata.as_ref();
        let bounds = window.bounds();

        let (rendered_width, rendered_height) = if let Some(meta) = metadata {
            (
                px(meta.width as f32 * self.zoom_level),
                px(meta.height as f32 * self.zoom_level),
            )
        } else {
            (px(0.0), px(0.0))
        };

        if self.initial_layout.is_none() {
            if let Some(_) = self.image_item.read(cx).image_metadata.as_ref() {
                self.reset_view(bounds.size, cx);
                self.initial_layout = Some(window.bounds().size);
            }
        }

        fn create_checkered_background(cx: &mut Context<ImageView>) -> impl IntoElement {
            let checkered_background_fn =
                |bounds: Bounds<Pixels>, _, window: &mut Window, _cx: &mut App| {
                    let square_size = 32.0;

                    let start_y = bounds.origin.y.0;
                    let height = bounds.size.height.0;
                    let start_x = bounds.origin.x.0;
                    let width = bounds.size.width.0;

                    let mut y = start_y;
                    let mut x = start_x;
                    let mut color_swapper = true;

                    while y <= start_y + height {
                        let start_swap = color_swapper;
                        while x <= start_x + width {
                            let rect = Bounds::new(
                                point(px(x), px(y)),
                                size(px(square_size), px(square_size)),
                            );

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

            canvas(|_, _, _| (), checkered_background_fn)
                .border_2()
                .border_color(cx.theme().styles.colors.border)
                .size_full()
                .absolute()
                .top_0()
                .left_0()
        }

        div()
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .child(create_checkered_background(cx))
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .w_full()
                    .h_full()
                    .child(
                        img(image.clone())
                            .object_fit(ObjectFit::ScaleDown)
                            .max_w_full()
                            .max_h_full()
                            .id("img"),
                    ),
            );

        div()
            .track_focus(&self.focus_handle(cx))
            .size_full()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this: &mut ImageView, event: &MouseDownEvent, _, cx| {
                    this.is_panning = true;
                    this.last_mouse_position = Some(event.position);
                    cx.refresh_windows();
                }),
            )
            .on_mouse_move(
                cx.listener(|this: &mut ImageView, event: &MouseMoveEvent, _, cx| {
                    if this.is_panning {
                        if let Some(last_pos) = this.last_mouse_position {
                            let delta = event.position - last_pos;
                            this.pan_offset += delta;
                            this.last_mouse_position = Some(event.position);
                            cx.refresh_windows();
                        }
                    }
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this: &mut ImageView, _: &MouseUpEvent, _, cx| {
                    this.is_panning = false;
                    this.last_mouse_position = None;
                    cx.refresh_windows();
                }),
            )
            .on_scroll_wheel(cx.listener(
                |this: &mut ImageView, event: &ScrollWheelEvent, _, cx| {
                    let sensitivity = 0.1;
                    let delta = match event.delta {
                        ScrollDelta::Lines(delta) => delta.y,
                        ScrollDelta::Pixels(pixels) => pixels.y.0,
                    };

                    let old_zoom = this.zoom_level;
                    let new_zoom = (old_zoom * (1.0 + delta * sensitivity)).clamp(0.1, 10.0);

                    if let Some(_) = this.image_item.read(cx).image_metadata.as_ref() {
                        let mouse_pos = event.position;
                        let image_x = (mouse_pos.x - this.pan_offset.x) / old_zoom;
                        let image_y = (mouse_pos.y - this.pan_offset.y) / old_zoom;

                        this.pan_offset.x = mouse_pos.x - image_x * new_zoom;
                        this.pan_offset.y = mouse_pos.y - image_y * new_zoom;
                        this.zoom_level = new_zoom;

                        cx.refresh_windows();
                    }
                },
            ))
            .child(create_checkered_background(cx))
            .child(
                div().size_full().overflow_hidden().relative().child(
                    img(image)
                        .absolute()
                        .left(self.pan_offset.x)
                        .top(self.pan_offset.y)
                        .w(rendered_width)
                        .h(rendered_height),
                ),
            )
    }
}

impl ProjectItem for ImageView {
    type Item = ImageItem;

    fn for_project_item(
        project: Entity<Project>,
        item: Entity<Self::Item>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self::new(item, project, cx)
    }
}

pub fn init(cx: &mut App) {
    ImageViewerSettings::register(cx);
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
