mod image_info;
mod image_viewer_settings;

use std::path::Path;

use anyhow::Context as _;
use editor::{EditorSettings, items::entry_git_aware_label_color};
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, Bounds, Context, DispatchPhase, Element, ElementId, Entity, EventEmitter,
    FocusHandle, Focusable, GlobalElementId, InspectorElementId, InteractiveElement, IntoElement,
    LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement, Pixels,
    Point, Render, ScrollDelta, ScrollWheelEvent, Style, Styled, Task, WeakEntity, Window, actions,
    canvas, div, img, opaque_grey, point, px, size,
};
use language::File as _;
use persistence::IMAGE_VIEWER;
use project::{ImageItem, Project, ProjectPath, image_store::ImageItemEvent};
use settings::Settings;
use theme::{Theme, ThemeSettings};
use ui::{Tooltip, prelude::*};
use util::paths::PathExt;
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    WorkspaceId, delete_unloaded_items,
    invalid_item_view::InvalidItemView,
    item::{BreadcrumbText, Item, ItemHandle, ProjectItem, SerializableItem, TabContentParams},
};

pub use crate::image_info::*;
pub use crate::image_viewer_settings::*;

actions!(
    image_viewer,
    [
        /// Zoom in the image.
        ZoomIn,
        /// Zoom out the image.
        ZoomOut,
        /// Reset zoom to 100%.
        ResetZoom,
        /// Fit the image to view.
        FitToView,
        /// Zoom to actual size (100%).
        ZoomToActualSize
    ]
);

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 20.0;
const ZOOM_STEP: f32 = 1.1;
const SCROLL_LINE_MULTIPLIER: f32 = 20.0;
const BASE_SQUARE_SIZE: f32 = 48.0;

pub struct ImageView {
    image_item: Entity<ImageItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    zoom_level: f32,
    pan_offset: Point<Pixels>,
    last_mouse_position: Option<Point<Pixels>>,
    container_bounds: Option<Bounds<Pixels>>,
    image_size: Option<(u32, u32)>,
}

impl ImageView {
    fn is_dragging(&self) -> bool {
        self.last_mouse_position.is_some()
    }

    pub fn new(
        image_item: Entity<ImageItem>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Start loading the image to render in the background to prevent the view
        // from flickering in most cases.
        let _ = image_item.update(cx, |image, cx| {
            image.image.clone().get_render_image(window, cx)
        });

        cx.subscribe(&image_item, Self::on_image_event).detach();
        cx.on_release_in(window, |this, window, cx| {
            let image_data = this.image_item.read(cx).image.clone();
            if let Some(image) = image_data.clone().get_render_image(window, cx) {
                cx.drop_image(image, None);
            }
            image_data.remove_asset(cx);
        })
        .detach();

        let image_size = image_item
            .read(cx)
            .image_metadata
            .map(|m| (m.width, m.height));

        Self {
            image_item,
            project,
            focus_handle: cx.focus_handle(),
            zoom_level: 1.0,
            pan_offset: Point::default(),
            last_mouse_position: None,
            container_bounds: None,
            image_size,
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
                self.image_size = self
                    .image_item
                    .read(cx)
                    .image_metadata
                    .map(|m| (m.width, m.height));
                cx.emit(ImageViewEvent::TitleChanged);
                cx.notify();
            }
            ImageItemEvent::ReloadNeeded => {}
        }
    }

    fn zoom_in(&mut self, _: &ZoomIn, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom_level * ZOOM_STEP, None, cx);
    }

    fn zoom_out(&mut self, _: &ZoomOut, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom_level / ZOOM_STEP, None, cx);
    }

    fn reset_zoom(&mut self, _: &ResetZoom, _window: &mut Window, cx: &mut Context<Self>) {
        self.zoom_level = 1.0;
        self.pan_offset = Point::default();
        cx.notify();
    }

    fn fit_to_view(&mut self, _: &FitToView, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some((bounds, (img_width, img_height))) = self.container_bounds.zip(self.image_size)
        {
            let container_width: f32 = bounds.size.width.into();
            let container_height: f32 = bounds.size.height.into();
            let scale_x = container_width / img_width as f32;
            let scale_y = container_height / img_height as f32;
            self.zoom_level = scale_x.min(scale_y).min(1.0);
            self.pan_offset = Point::default();
            cx.notify();
        }
    }

    fn zoom_to_actual_size(
        &mut self,
        _: &ZoomToActualSize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.zoom_level = 1.0;
        self.pan_offset = Point::default();
        cx.notify();
    }

    fn set_zoom(
        &mut self,
        new_zoom: f32,
        zoom_center: Option<Point<Pixels>>,
        cx: &mut Context<Self>,
    ) {
        let old_zoom = self.zoom_level;
        self.zoom_level = new_zoom.clamp(MIN_ZOOM, MAX_ZOOM);

        if let Some((center, bounds)) = zoom_center.zip(self.container_bounds) {
            let relative_center = point(
                center.x - bounds.origin.x - bounds.size.width / 2.0,
                center.y - bounds.origin.y - bounds.size.height / 2.0,
            );

            let mouse_offset_from_image = relative_center - self.pan_offset;

            let zoom_ratio = self.zoom_level / old_zoom;

            self.pan_offset += mouse_offset_from_image * (1.0 - zoom_ratio);
        }

        cx.notify();
    }

    fn handle_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.modifiers.control || event.modifiers.platform {
            let delta: f32 = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels.y.into(),
                ScrollDelta::Lines(lines) => lines.y * SCROLL_LINE_MULTIPLIER,
            };
            let zoom_factor = if delta > 0.0 {
                1.0 + delta.abs() * 0.01
            } else {
                1.0 / (1.0 + delta.abs() * 0.01)
            };
            self.set_zoom(self.zoom_level * zoom_factor, Some(event.position), cx);
        } else {
            let delta = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels,
                ScrollDelta::Lines(lines) => lines.map(|d| px(d * SCROLL_LINE_MULTIPLIER)),
            };
            self.pan_offset += delta;
            cx.notify();
        }
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.button == MouseButton::Left || event.button == MouseButton::Middle {
            self.last_mouse_position = Some(event.position);
            cx.notify();
        }
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.last_mouse_position = None;
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging() {
            if let Some(last_pos) = self.last_mouse_position {
                let delta = event.position - last_pos;
                self.pan_offset += delta;
            }
            self.last_mouse_position = Some(event.position);
            cx.notify();
        }
    }
}

struct ImageContentElement {
    image_view: Entity<ImageView>,
}

impl ImageContentElement {
    fn new(image_view: Entity<ImageView>) -> Self {
        Self { image_view }
    }
}

impl IntoElement for ImageContentElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ImageContentElement {
    type RequestLayoutState = ();
    type PrepaintState = Option<(AnyElement, bool)>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (
            window.request_layout(
                Style {
                    size: size(relative(1.).into(), relative(1.).into()),
                    ..Default::default()
                },
                [],
                cx,
            ),
            (),
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let image_view = self.image_view.read(cx);
        let image = image_view.image_item.read(cx).image.clone();

        let zoom_level = image_view.zoom_level;
        let pan_offset = image_view.pan_offset;
        let border_color = cx.theme().colors().border;

        let is_dragging = image_view.is_dragging();

        let scaled_size = image_view
            .image_size
            .map(|(w, h)| (px(w as f32 * zoom_level), px(h as f32 * zoom_level)));

        let (mut left, mut top) = (px(0.0), px(0.0));
        let mut scaled_width = px(0.0);
        let mut scaled_height = px(0.0);

        if let Some((width, height)) = scaled_size {
            scaled_width = width;
            scaled_height = height;

            let center_x = bounds.size.width / 2.0;
            let center_y = bounds.size.height / 2.0;

            left = center_x - (scaled_width / 2.0) + pan_offset.x;
            top = center_y - (scaled_height / 2.0) + pan_offset.y;
        }

        self.image_view.update(cx, |this, _| {
            this.container_bounds = Some(bounds);
        });

        let mut image_content = div()
            .relative()
            .size_full()
            .child(
                div()
                    .absolute()
                    .left(left)
                    .top(top)
                    .w(scaled_width)
                    .h(scaled_height)
                    .child(
                        canvas(
                            |_, _, _| {},
                            move |bounds, _, window, _cx| {
                                let bounds_x: f32 = bounds.origin.x.into();
                                let bounds_y: f32 = bounds.origin.y.into();
                                let bounds_width: f32 = bounds.size.width.into();
                                let bounds_height: f32 = bounds.size.height.into();
                                let square_size = BASE_SQUARE_SIZE * zoom_level;
                                let cols = (bounds_width / square_size).ceil() as i32 + 1;
                                let rows = (bounds_height / square_size).ceil() as i32 + 1;
                                for row in 0..rows {
                                    for col in 0..cols {
                                        if (row + col) % 2 == 0 {
                                            continue;
                                        }
                                        let x = bounds_x + col as f32 * square_size;
                                        let y = bounds_y + row as f32 * square_size;
                                        let w = square_size.min(bounds_x + bounds_width - x);
                                        let h = square_size.min(bounds_y + bounds_height - y);
                                        if w > 0.0 && h > 0.0 {
                                            let rect = Bounds::new(
                                                point(px(x), px(y)),
                                                size(px(w), px(h)),
                                            );
                                            window.paint_quad(gpui::fill(
                                                rect,
                                                opaque_grey(0.6, 1.0),
                                            ));
                                        }
                                    }
                                }
                                let border_rect = Bounds::new(
                                    point(px(bounds_x), px(bounds_y)),
                                    size(px(bounds_width), px(bounds_height)),
                                );
                                window.paint_quad(gpui::outline(
                                    border_rect,
                                    border_color,
                                    gpui::BorderStyle::default(),
                                ));
                            },
                        )
                        .size_full()
                        .absolute()
                        .top_0()
                        .left_0()
                        .bg(gpui::rgb(0xCCCCCD)),
                    )
                    .child({
                        img(image)
                            .id(("image-viewer-image", self.image_view.entity_id()))
                            .size_full()
                    }),
            )
            .into_any_element();

        image_content.prepaint_as_root(bounds.origin, bounds.size.into(), window, cx);
        Some((image_content, is_dragging))
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some((mut element, is_dragging)) = prepaint.take() else {
            return;
        };

        if is_dragging {
            let image_view = self.image_view.downgrade();
            window.on_mouse_event(move |_event: &MouseUpEvent, phase, _window, cx| {
                if phase == DispatchPhase::Bubble
                    && let Some(entity) = image_view.upgrade()
                {
                    entity.update(cx, |this, cx| {
                        this.last_mouse_position = None;
                        cx.notify();
                    });
                }
            });
        }

        element.paint(window, cx);
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

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let abs_path = self.image_item.read(cx).abs_path(cx)?;
        let file_path = abs_path.compact().to_string_lossy().into_owned();
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
            .to_string()
            .into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = self.image_item.read(cx).abs_path(cx)?;
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(&path, cx))
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
        let settings = ThemeSettings::get_global(cx);

        Some(vec![BreadcrumbText {
            text,
            highlights: None,
            font: Some(settings.buffer_font.clone()),
        }])
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| Self {
            image_item: self.image_item.clone(),
            project: self.project.clone(),
            focus_handle: cx.focus_handle(),
            zoom_level: self.zoom_level,
            pan_offset: self.pan_offset,
            last_mouse_position: None,
            container_bounds: None,
            image_size: self.image_size,
        })))
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.image_item.read(cx).file.disk_state().is_deleted()
    }
    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }
}

fn breadcrumbs_text_for_image(project: &Project, image: &ImageItem, cx: &App) -> String {
    let mut path = image.file.path().clone();
    if project.visible_worktrees(cx).count() > 1
        && let Some(worktree) = project.worktree_for_id(image.project_path(cx).worktree_id, cx)
    {
        path = worktree.read(cx).root_name().join(&path);
    }

    path.display(project.path_style(cx)).to_string()
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
    ) -> Task<anyhow::Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            let image_path = IMAGE_VIEWER
                .get_image_path(item_id, workspace_id)?
                .context("No image path found")?;

            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(image_path.clone(), false, cx)
                })
                .await
                .context("Path not found")?;
            let worktree_id = worktree.update(cx, |worktree, _cx| worktree.id());

            let project_path = ProjectPath {
                worktree_id,
                path: relative_path,
            };

            let image_item = project
                .update(cx, |project, cx| project.open_image(project_path, cx))
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
    ) -> Task<anyhow::Result<()>> {
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
    ) -> Option<Task<anyhow::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let image_path = self.image_item.read(cx).abs_path(cx)?;

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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle(cx))
            .key_context("ImageViewer")
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::reset_zoom))
            .on_action(cx.listener(Self::fit_to_view))
            .on_action(cx.listener(Self::zoom_to_actual_size))
            .size_full()
            .relative()
            .bg(cx.theme().colors().editor_background)
            .child(
                div()
                    .id("image-container")
                    .size_full()
                    .overflow_hidden()
                    .cursor(if self.is_dragging() {
                        gpui::CursorStyle::ClosedHand
                    } else {
                        gpui::CursorStyle::OpenHand
                    })
                    .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
                    .on_mouse_down(MouseButton::Middle, cx.listener(Self::handle_mouse_down))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                    .on_mouse_up(MouseButton::Middle, cx.listener(Self::handle_mouse_up))
                    .on_mouse_move(cx.listener(Self::handle_mouse_move))
                    .child(ImageContentElement::new(cx.entity())),
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

    fn for_broken_project_item(
        abs_path: &Path,
        is_local: bool,
        e: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView>
    where
        Self: Sized,
    {
        Some(InvalidItemView::new(abs_path, is_local, e, window, cx))
    }
}

pub struct ImageViewToolbarControls {
    image_view: Option<WeakEntity<ImageView>>,
    _subscription: Option<gpui::Subscription>,
}

impl ImageViewToolbarControls {
    pub fn new() -> Self {
        Self {
            image_view: None,
            _subscription: None,
        }
    }
}

impl Render for ImageViewToolbarControls {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(image_view) = self.image_view.as_ref().and_then(|v| v.upgrade()) else {
            return div().into_any_element();
        };

        let zoom_level = image_view.read(cx).zoom_level;
        let zoom_percentage = format!("{}%", (zoom_level * 100.0).round() as i32);

        h_flex()
            .gap_1()
            .child(
                IconButton::new("zoom-out", IconName::Dash)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Zoom Out", &ZoomOut, cx))
                    .on_click({
                        let image_view = image_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = image_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.zoom_out(&ZoomOut, window, cx);
                                });
                            }
                        }
                    }),
            )
            .child(
                Button::new("zoom-level", zoom_percentage)
                    .label_size(LabelSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Reset Zoom", &ResetZoom, cx))
                    .on_click({
                        let image_view = image_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = image_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.reset_zoom(&ResetZoom, window, cx);
                                });
                            }
                        }
                    }),
            )
            .child(
                IconButton::new("zoom-in", IconName::Plus)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Zoom In", &ZoomIn, cx))
                    .on_click({
                        let image_view = image_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = image_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.zoom_in(&ZoomIn, window, cx);
                                });
                            }
                        }
                    }),
            )
            .child(
                IconButton::new("fit-to-view", IconName::Maximize)
                    .icon_size(IconSize::Small)
                    .tooltip(|_window, cx| Tooltip::for_action("Fit to View", &FitToView, cx))
                    .on_click({
                        let image_view = image_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = image_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.fit_to_view(&FitToView, window, cx);
                                });
                            }
                        }
                    }),
            )
            .into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for ImageViewToolbarControls {}

impl ToolbarItemView for ImageViewToolbarControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.image_view = None;
        self._subscription = None;

        if let Some(item) = active_pane_item.and_then(|i| i.downcast::<ImageView>()) {
            self._subscription = Some(cx.observe(&item, |_, _, cx| {
                cx.notify();
            }));
            self.image_view = Some(item.downgrade());
            cx.notify();
            return ToolbarItemLocation::PrimaryRight;
        }

        ToolbarItemLocation::Hidden
    }
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<ImageView>(cx);
    workspace::register_serializable_item::<ImageView>(cx);
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct ImageViewerDb(ThreadSafeConnection);

    impl Domain for ImageViewerDb {
        const NAME: &str = stringify!(ImageViewerDb);

        const MIGRATIONS: &[&str] = &[sql!(
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

    db::static_connection!(IMAGE_VIEWER, ImageViewerDb, [WorkspaceDb]);

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
