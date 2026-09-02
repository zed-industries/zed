mod image_info;
mod image_viewer_settings;

use std::{path::Path, sync::Arc};

use anyhow::Context as _;
use editor::{
    Editor, EditorEvent, EditorSettings, RevealInFileManager, actions::SelectAll,
    items::entry_git_aware_label_color,
};
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, Bounds, Context, DispatchPhase, Element, ElementId, Entity, EventEmitter,
    FocusHandle, Focusable, Font, GlobalElementId, InspectorElementId, InteractiveElement,
    IntoElement, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement, PinchEvent, Pixels, Point, Render, RenderImage, ScrollDelta, ScrollWheelEvent,
    Style, Styled, Subscription, Task, WeakEntity, Window, actions, checkerboard, div, img, point,
    px, size,
};
use language::File as _;
use persistence::ImageViewerDb;
use project::{
    ImageItem, Project, ProjectPath, git_store::GitStoreEvent, image_store::ImageItemEvent,
};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{Divider, Tooltip, prelude::*};
use util::{ResultExt as _, paths::PathExt};
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    WorkspaceId, delete_unloaded_items,
    invalid_item_view::InvalidItemView,
    item::{HighlightedText, Item, ItemHandle, ProjectItem, SerializableItem, TabContentParams},
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
        ZoomToActualSize,
        /// Go to next page (PDF).
        NextPage,
        /// Go to previous page (PDF).
        PreviousPage,
        /// Go to first page (PDF).
        FirstPage,
        /// Go to last page (PDF).
        LastPage,
        /// Copy text from PDF.
        CopyText,
        /// Toggle text inspection panel.
        ToggleTextPanel
    ]
);

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 20.0;
const ZOOM_STEP: f32 = 1.1;
const SCROLL_LINE_MULTIPLIER: f32 = 20.0;
const BASE_SQUARE_SIZE: f32 = 32.0;
const ZOOM_EDITOR_MIN_DIGITS: usize = 3; // Reserve room for common values like 100%.
const ZOOM_EDITOR_MAX_DIGITS: usize = 4; // MAX_ZOOM is 2000%.
const ZOOM_EDITOR_APPROX_CHAR_WIDTH: f32 = 8.0; // Approximate width of one small UI digit.
const ZOOM_EDITOR_HORIZONTAL_PADDING: f32 = 12.0; // Extra room for cursor and editor edge padding.

pub struct ImageView {
    image_item: Entity<ImageItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    zoom_level: f32,
    pan_offset: Point<Pixels>,
    last_mouse_position: Option<Point<Pixels>>,
    container_bounds: Option<Bounds<Pixels>>,
    image_size: Option<(u32, u32)>,
    pending_image: Option<Arc<gpui::Image>>,
    displayed_image: Option<DisplayedImage>,
    pdf_scroll_handle: gpui::ScrollHandle,
    show_text_panel: bool,
    text_buffer: Option<Entity<language::Buffer>>,
    text_editor: Option<Entity<Editor>>,
}

struct DisplayedImage {
    source_image: Arc<gpui::Image>,
    render_image: Arc<RenderImage>,
}

impl DisplayedImage {
    fn drop_atlas_entry(&self, window: &mut Window) {
        window.drop_image(self.render_image.clone()).log_err();
    }

    fn release(self, window: &mut Window, cx: &mut App) {
        self.drop_atlas_entry(window);
        self.source_image.remove_asset(cx);
    }
}

impl ImageView {
    fn is_dragging(&self) -> bool {
        self.last_mouse_position.is_some()
    }

    fn update_displayed_image(
        &mut self,
        image: &Arc<gpui::Image>,
        render_image: Option<Arc<RenderImage>>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(pending_image) = self.pending_image.take() {
            if pending_image.id() != image.id() {
                pending_image.remove_asset(cx);
            } else if render_image.is_none() {
                self.pending_image = Some(pending_image);
            }
        }

        let Some(render_image) = render_image else {
            self.pending_image.get_or_insert_with(|| image.clone());
            return;
        };

        if self
            .displayed_image
            .as_ref()
            .is_some_and(|displayed_image| displayed_image.render_image.id == render_image.id)
        {
            return;
        }

        if let Some(previous) = self.displayed_image.take() {
            if previous.source_image.id() == image.id() {
                previous.drop_atlas_entry(window);
            } else {
                previous.release(window, cx);
            }
        }

        self.displayed_image = Some(DisplayedImage {
            source_image: image.clone(),
            render_image,
        });
    }

    pub fn new(
        image_item: Entity<ImageItem>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Start loading the image to render in the background to prevent the view
        // from flickering in most cases.
        let pending_image = image_item.read(cx).image.clone();
        let _render_image = pending_image.clone().get_render_image(window, cx);

        cx.subscribe(&image_item, Self::on_image_event).detach();
        let git_store = project.read(cx).git_store().clone();
        cx.subscribe(&git_store, |_, _, event, cx| {
            if matches!(event, GitStoreEvent::DiffBaseChanged(_)) {
                cx.emit(ImageViewEvent::TitleChanged);
            }
        })
        .detach();
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
            pending_image: Some(pending_image),
            displayed_image: None,
            pdf_scroll_handle: gpui::ScrollHandle::new(),
            show_text_panel: false,
            text_buffer: None,
            text_editor: None,
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
                let image = self.image_item.read(cx).image.clone();
                self.pending_image = Some(image);
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
        if let Some((bounds, image_size)) = self.container_bounds.zip(self.image_size) {
            self.zoom_level = ImageView::compute_fit_to_view_zoom(bounds, image_size);
            self.pan_offset = Point::default();
            cx.notify();
        }
    }

    fn compute_fit_to_view_zoom(container_bounds: Bounds<Pixels>, image_size: (u32, u32)) -> f32 {
        let (image_width, image_height) = image_size;
        let container_width: f32 = container_bounds.size.width.into();
        let container_height: f32 = container_bounds.size.height.into();
        let scale_x = container_width / image_width as f32;
        let scale_y = container_height / image_height as f32;
        scale_x.min(scale_y).min(1.0)
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

    fn next_page(&mut self, _: &NextPage, _window: &mut Window, cx: &mut Context<Self>) {
        let changed = self.image_item.update(cx, |item, cx| {
            item.next_page(cx)
        });
        if changed {
            let cur = self.image_item.read(cx).current_page();
            self.pdf_scroll_handle.scroll_to_top_of_item(cur);
            cx.notify();
        }
    }

    fn previous_page(&mut self, _: &PreviousPage, _window: &mut Window, cx: &mut Context<Self>) {
        let changed = self.image_item.update(cx, |item, cx| {
            item.previous_page(cx)
        });
        if changed {
            let cur = self.image_item.read(cx).current_page();
            self.pdf_scroll_handle.scroll_to_top_of_item(cur);
            cx.notify();
        }
    }

    fn first_page(&mut self, _: &FirstPage, _window: &mut Window, cx: &mut Context<Self>) {
        let changed = self.image_item.update(cx, |item, cx| {
            item.first_page(cx)
        });
        if changed {
            self.pdf_scroll_handle.scroll_to_top_of_item(0);
            cx.notify();
        }
    }

    fn last_page(&mut self, _: &LastPage, _window: &mut Window, cx: &mut Context<Self>) {
        let changed = self.image_item.update(cx, |item, cx| {
            item.last_page(cx)
        });
        if changed {
            let cur = self.image_item.read(cx).current_page();
            self.pdf_scroll_handle.scroll_to_top_of_item(cur);
            cx.notify();
        }
    }

    fn copy_text(&mut self, _: &CopyText, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = self.image_item.read(cx).extract_text() {
            if !text.is_empty() {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
            }
        }
    }

    fn toggle_text_panel(&mut self, _: &ToggleTextPanel, window: &mut Window, cx: &mut Context<Self>) {
        self.show_text_panel = !self.show_text_panel;
        if self.show_text_panel && self.text_editor.is_none() {
            if let Some(text) = self.image_item.read(cx).extract_text() {
                let buffer = cx.new(|cx| language::Buffer::local(text, cx));
                let editor = cx.new(|cx| {
                    let mut editor = Editor::for_buffer(buffer.clone(), Some(self.project.clone()), window, cx);
                    editor.set_read_only(true);
                    editor
                });
                self.text_buffer = Some(buffer);
                self.text_editor = Some(editor);
            }
        }
        cx.notify();
    }

    fn reveal_in_file_manager(
        &mut self,
        _: &RevealInFileManager,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = self.image_item.read(cx).abs_path(cx) {
            self.project
                .update(cx, |project, cx| project.reveal_path(&path, cx));
        }
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

    fn handle_pinch(&mut self, event: &PinchEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let zoom_factor = 1.0 + event.delta;
        self.set_zoom(self.zoom_level * zoom_factor, Some(event.position), cx);
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

        self.image_view.update(cx, |this, cx| {
            let render_image = image.clone().use_render_image(window, cx);
            this.update_displayed_image(&image, render_image, window, cx);
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
                        div()
                            .size_full()
                            .absolute()
                            .top_0()
                            .left_0()
                            .child(div().size_full().bg(checkerboard(
                                cx.theme().colors().panel_background,
                                BASE_SQUARE_SIZE * zoom_level,
                            )))
                            .border_1()
                            .border_color(border_color),
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

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
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
                .git_store()
                .read(cx)
                .display_status_for_project_path(&project_path, cx)
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

    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)> {
        let text = breadcrumbs_text_for_image(self.project.read(cx), self.image_item.read(cx), cx);
        let font = ThemeSettings::get_global(cx).buffer_font.clone();

        Some((
            vec![HighlightedText {
                text: text.into(),
                highlights: vec![],
            }],
            Some(font),
        ))
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _window: &mut Window,
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
            pending_image: None,
            displayed_image: None,
            pdf_scroll_handle: gpui::ScrollHandle::new(),
            show_text_panel: self.show_text_panel,
            text_buffer: None,
            text_editor: None,
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
    let mut path = image.file.path().to_rel_path_buf();
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
        let db = ImageViewerDb::global(cx);
        window.spawn(cx, async move |cx| {
            let image_path = db
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
        let db = ImageViewerDb::global(cx);
        delete_unloaded_items(alive_items, workspace_id, "image_viewers", &db, cx)
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<anyhow::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let image_path = self.image_item.read(cx).abs_path(cx)?;

        let db = ImageViewerDb::global(cx);
        Some(cx.background_spawn({
            async move {
                log::debug!("Saving image at path {image_path:?}");
                db.save_image_path(item_id, workspace_id, image_path).await
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
        let pdf_info = self.image_item.read(cx).pdf_info.clone();

        div()
            .track_focus(&self.focus_handle(cx))
            .key_context("ImageViewer")
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::reset_zoom))
            .on_action(cx.listener(Self::fit_to_view))
            .on_action(cx.listener(Self::zoom_to_actual_size))
            .on_action(cx.listener(Self::reveal_in_file_manager))
            .on_action(cx.listener(Self::next_page))
            .on_action(cx.listener(Self::previous_page))
            .on_action(cx.listener(Self::first_page))
            .on_action(cx.listener(Self::last_page))
            .on_action(cx.listener(Self::copy_text))
            .on_action(cx.listener(Self::toggle_text_panel))
            .size_full()
            .relative()
            .bg(cx.theme().colors().editor_background)
            .child({
                if let Some(pdf_info) = pdf_info.filter(|info| !info.pages.is_empty()) {
                    let border_color = cx.theme().colors().border;
                    let zoom_level = self.zoom_level;
                    let total_pages = pdf_info.total_pages;

                    let main_pdf_view = div()
                        .id("pdf-scroll-container")
                        .track_scroll(&self.pdf_scroll_handle)
                        .overflow_y_scroll()
                        .size_full()
                        .child(
                            v_flex()
                                .w_full()
                                .items_center()
                                .py_6()
                                .gap_8()
                                .children(pdf_info.pages.into_iter().map(|page| {
                                    let page_w = px(page.width as f32 * zoom_level);
                                    let page_h = px(page.height as f32 * zoom_level);
                                    let page_idx = page.page_index;
                                    v_flex()
                                        .id(("pdf-page-container", page_idx))
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .id(("pdf-page-canvas", page_idx))
                                                .w(page_w)
                                                .h(page_h)
                                                .shadow_md()
                                                .border_1()
                                                .border_color(border_color)
                                                .bg(gpui::white())
                                                .child(
                                                    img(page.image)
                                                        .id(("pdf-page-img", page_idx))
                                                        .size_full()
                                                )
                                        )
                                        .child(
                                            Label::new(format!("Page {} of {}", page_idx + 1, total_pages))
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted)
                                        )
                                }))
                        );

                    if self.show_text_panel {
                        h_flex()
                            .size_full()
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .child(main_pdf_view)
                            )
                            .child(Divider::vertical())
                            .child(
                                div()
                                    .w_96()
                                    .h_full()
                                    .p_2()
                                    .bg(cx.theme().colors().panel_background)
                                    .child(
                                        if let Some(editor) = &self.text_editor {
                                            div().size_full().child(editor.clone()).into_any_element()
                                        } else {
                                            div().size_full().child(Label::new("Extracting text...")).into_any_element()
                                        }
                                    )
                            )
                            .into_any_element()
                    } else {
                        main_pdf_view.into_any_element()
                    }
                } else {
                    let container = div()
                        .id("image-container")
                        .size_full()
                        .overflow_hidden()
                        .cursor(if self.is_dragging() {
                            gpui::CursorStyle::ClosedHand
                        } else {
                            gpui::CursorStyle::OpenHand
                        })
                        .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
                        .on_pinch(cx.listener(Self::handle_pinch))
                        .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
                        .on_mouse_down(MouseButton::Middle, cx.listener(Self::handle_mouse_down))
                        .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                        .on_mouse_up(MouseButton::Middle, cx.listener(Self::handle_mouse_up))
                        .on_mouse_move(cx.listener(Self::handle_mouse_move))
                        .child(ImageContentElement::new(cx.entity()));

                    container.into_any_element()
                }
            })
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
    zoom_editor: Option<Entity<Editor>>,
    _zoom_subscription: Option<Subscription>,
}

impl ImageViewToolbarControls {
    pub fn new() -> Self {
        Self {
            image_view: None,
            _subscription: None,
            zoom_editor: None,
            _zoom_subscription: None,
        }
    }

    fn start_editing_zoom(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(image_view) = self.image_view.as_ref().and_then(|v| v.upgrade()) else {
            return;
        };
        let zoom_level = image_view.read(cx).zoom_level;
        let zoom_percentage = (zoom_level * 100.0).round() as i32;

        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(zoom_percentage.to_string(), window, cx);
            editor.set_text_style_refinement(gpui::TextStyleRefinement {
                color: Some(cx.theme().colors().text),
                text_align: Some(gpui::TextAlign::Center),
                font_size: Some(TextSize::Small.rems(cx).into()),
                ..Default::default()
            });
            editor.select_all(&SelectAll, window, cx);
            editor
        });

        let subscription = cx.subscribe_in(&editor, window, {
            move |this, editor, event, window, cx| match event {
                EditorEvent::Blurred => this.commit_edit(cx),
                EditorEvent::Edited { .. } => {
                    let text = editor.read(cx).text(cx);
                    let sanitized = text
                        .chars()
                        .filter(|ch| ch.is_ascii_digit())
                        .take(ZOOM_EDITOR_MAX_DIGITS)
                        .collect::<String>();
                    if sanitized != text {
                        editor.update(cx, |editor, cx| editor.set_text(sanitized, window, cx));
                    }
                    cx.notify();
                }
                _ => {}
            }
        });

        editor.focus_handle(cx).focus(window, cx);

        self.zoom_editor = Some(editor);
        self._zoom_subscription = Some(subscription);

        cx.notify();
    }

    fn commit_edit(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.zoom_editor.as_ref() else {
            self.cancel_edit(cx);
            return;
        };

        let input = editor.read(cx).text(cx);
        let parsed = input
            .trim()
            .parse::<u32>()
            .ok()
            .filter(|parsed| *parsed > 0);

        let Some(parsed) = parsed else {
            self.cancel_edit(cx);
            return;
        };

        self._zoom_subscription = None;
        self.zoom_editor = None;

        let new_zoom = (parsed as f32 / 100.0).clamp(MIN_ZOOM, MAX_ZOOM);
        if let Some(image_view) = self.image_view.as_ref().and_then(|v| v.upgrade()) {
            image_view.update(cx, |this, cx| {
                this.set_zoom(new_zoom, None, cx);
            });
        }

        cx.notify();
    }

    fn cancel_edit(&mut self, cx: &mut Context<Self>) {
        self.zoom_editor = None;
        self._zoom_subscription = None;
        cx.notify();
    }
}

impl Render for ImageViewToolbarControls {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(image_view) = self.image_view.as_ref().and_then(|v| v.upgrade()) else {
            return div().into_any_element();
        };

        let image_item = image_view.read(cx).image_item.clone();
        let (is_pdf, current_page, total_pages, show_text_panel) = {
            let item = image_item.read(cx);
            let show_text = image_view.read(cx).show_text_panel;
            (item.is_pdf(), item.current_page(), item.total_pages(), show_text)
        };

        h_flex()
            .gap_1()
            .when(is_pdf && total_pages > 1, |this| {
                let image_view_prev = image_view.downgrade();
                let image_view_next = image_view.downgrade();
                this.child(
                    IconButton::new("prev-page", IconName::ChevronLeft)
                        .icon_size(IconSize::Small)
                        .disabled(current_page == 0)
                        .tooltip(|_window, cx| Tooltip::for_action("Previous Page", &PreviousPage, cx))
                        .on_click(move |_, window, cx| {
                            if let Some(view) = image_view_prev.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.previous_page(&PreviousPage, window, cx);
                                });
                            }
                        }),
                )
                .child(
                    h_flex()
                        .px_1()
                        .child(Label::new(format!("{} / {}", current_page + 1, total_pages)).size(LabelSize::Small))
                )
                .child(
                    IconButton::new("next-page", IconName::ChevronRight)
                        .icon_size(IconSize::Small)
                        .disabled(current_page + 1 >= total_pages)
                        .tooltip(|_window, cx| Tooltip::for_action("Next Page", &NextPage, cx))
                        .on_click(move |_, window, cx| {
                            if let Some(view) = image_view_next.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.next_page(&NextPage, window, cx);
                                });
                            }
                        }),
                )
                .child(Divider::vertical())
            })
            .when(is_pdf, |this| {
                let image_view_copy = image_view.downgrade();
                let image_view_text = image_view.downgrade();
                this.child(
                    IconButton::new("copy-text", IconName::Copy)
                        .icon_size(IconSize::Small)
                        .tooltip(|_window, cx| Tooltip::for_action("Copy Text", &CopyText, cx))
                        .on_click(move |_, window, cx| {
                            if let Some(view) = image_view_copy.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.copy_text(&CopyText, window, cx);
                                });
                            }
                        }),
                )
                .child(
                    IconButton::new("toggle-text-panel", IconName::FileDoc)
                        .icon_size(IconSize::Small)
                        .toggle_state(show_text_panel)
                        .tooltip(|_window, cx| Tooltip::for_action("Toggle Text Panel", &ToggleTextPanel, cx))
                        .on_click(move |_, window, cx| {
                            if let Some(view) = image_view_text.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.toggle_text_panel(&ToggleTextPanel, window, cx);
                                });
                            }
                        }),
                )
                .child(Divider::vertical())
            })
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
            .child(if let Some(editor) = self.zoom_editor.as_ref() {
                // Grow with input, defaulting to 3-digit zoom.
                let editor_width = px((editor
                    .read(cx)
                    .text(cx)
                    .chars()
                    .count()
                    .clamp(ZOOM_EDITOR_MIN_DIGITS, ZOOM_EDITOR_MAX_DIGITS)
                    as f32
                    * ZOOM_EDITOR_APPROX_CHAR_WIDTH)
                    + ZOOM_EDITOR_HORIZONTAL_PADDING);

                h_flex()
                    .w(editor_width)
                    .capture_key_down(|event, _window, cx| {
                        if event.keystroke.modifiers.control || event.keystroke.modifiers.platform {
                            return;
                        }

                        // Only allow digits to be entered
                        if let Some(text) = event.keystroke.key_char.as_deref()
                            && !text.chars().all(|ch| ch.is_ascii_digit())
                        {
                            cx.stop_propagation();
                        }
                    })
                    .child(editor.clone())
                    .on_action::<menu::Confirm>({
                        move |_: &menu::Confirm, window, cx| {
                            window.blur(cx);
                        }
                    })
                    .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| {
                        this.cancel_edit(cx);
                    }))
                    .into_any_element()
            } else {
                let zoom_level = image_view.read(cx).zoom_level;
                let zoom_percentage = format!("{}%", (zoom_level * 100.0).round() as i32);
                h_flex()
                    .px_1()
                    .cursor_pointer()
                    .child(Label::new(zoom_percentage).size(LabelSize::Small))
                    .id("zoom-label")
                    .tooltip(|_window, cx| {
                        Tooltip::with_meta("Edit Zoom", None, "Right-click to reset to 100%.", cx)
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.start_editing_zoom(window, cx);
                    }))
                    .on_mouse_down(MouseButton::Right, {
                        let image_view = image_view.downgrade();
                        move |_, window, cx| {
                            if let Some(view) = image_view.upgrade() {
                                view.update(cx, |this, cx| {
                                    this.reset_zoom(&ResetZoom, window, cx);
                                });
                            }
                        }
                    })
                    .into_any_element()
            })
            .child(
                IconButton::new("zoom-in", IconName::Plus)
                    .icon_size(IconSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Zoom In", &ZoomIn, cx))
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
        self.zoom_editor = None;
        self._zoom_subscription = None;

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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::{FakeFs, Fs as _};
    use gpui::{TestAppContext, VisualTestContext};
    use settings::SettingsStore;
    use util::rel_path::rel_path;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
    }

    fn test_image(red: u8) -> Arc<gpui::Image> {
        let bytes = format!("P3\n1 1\n255\n{red} 0 0\n").into_bytes();
        Arc::new(gpui::Image::from_bytes(gpui::ImageFormat::Pnm, bytes))
    }

    async fn open_test_image(cx: &mut TestAppContext) -> (Entity<Project>, Entity<ImageItem>) {
        let fs = FakeFs::new(cx.executor());
        fs.create_dir(Path::new("/root"))
            .await
            .expect("test root should be created");
        fs.insert_file("/root/image.ppm", test_image(0).bytes.clone())
            .await;

        let project = Project::test(fs, [Path::new("/root")], cx).await;
        let worktree_id = cx.update(|cx| {
            project
                .read(cx)
                .worktrees(cx)
                .next()
                .expect("test project should contain a worktree")
                .read(cx)
                .id()
        });
        let image_item = project
            .update(cx, |project, cx| {
                project.open_image(
                    ProjectPath {
                        worktree_id,
                        path: rel_path("image.ppm").into(),
                    },
                    cx,
                )
            })
            .await
            .expect("test image should open");

        (project, image_item)
    }

    fn draw_window(cx: &mut VisualTestContext) {
        cx.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
        });
    }

    fn displayed_source_id(image_view: &Entity<ImageView>, cx: &VisualTestContext) -> Option<u64> {
        cx.read(|cx| {
            image_view
                .read(cx)
                .displayed_image
                .as_ref()
                .map(|displayed_image| displayed_image.source_image.id())
        })
    }

    fn displayed_render_image(
        image_view: &Entity<ImageView>,
        cx: &VisualTestContext,
    ) -> Option<Arc<RenderImage>> {
        cx.read(|cx| {
            image_view
                .read(cx)
                .displayed_image
                .as_ref()
                .map(|displayed_image| displayed_image.render_image.clone())
        })
    }

    fn replace_image(
        image_item: &Entity<ImageItem>,
        image: Arc<gpui::Image>,
        cx: &mut VisualTestContext,
    ) {
        cx.update(|_window, cx| {
            image_item.update(cx, |image_item, cx| {
                image_item.image = image;
                cx.emit(ImageItemEvent::Reloaded);
            });
        });
    }

    fn replace_image_and_draw(
        image_item: &Entity<ImageItem>,
        image: Arc<gpui::Image>,
        cx: &mut VisualTestContext,
    ) {
        replace_image(image_item, image, cx);
        draw_window(cx);
    }

    fn image_is_cached(image: &Arc<gpui::Image>, cx: &VisualTestContext) -> bool {
        cx.read(|cx| image.is_asset_cached(cx))
    }

    #[gpui::test]
    async fn test_reloading_removes_replaced_image_from_asset_cache(cx: &mut TestAppContext) {
        init_test(cx);
        let (project, image_item) = open_test_image(cx).await;
        let original_image = cx.read(|cx| image_item.read(cx).image.clone());

        let (image_view, cx) = cx
            .add_window_view(|window, cx| ImageView::new(image_item.clone(), project, window, cx));

        cx.run_until_parked();
        draw_window(cx);
        assert_eq!(
            displayed_source_id(&image_view, cx),
            Some(original_image.id()),
            "the original image should finish decoding and be displayed"
        );

        let reloaded_image = test_image(1);
        replace_image_and_draw(&image_item, reloaded_image.clone(), cx);
        cx.run_until_parked();
        draw_window(cx);

        assert_eq!(
            displayed_source_id(&image_view, cx),
            Some(reloaded_image.id()),
            "the reloaded image should replace the original"
        );
        assert!(
            !image_is_cached(&original_image, cx),
            "the replaced image remained in GPUI's asset cache"
        );
    }

    #[gpui::test]
    async fn test_superseded_in_flight_image_is_removed_from_asset_cache(cx: &mut TestAppContext) {
        init_test(cx);
        let (project, image_item) = open_test_image(cx).await;

        let (image_view, cx) = cx
            .add_window_view(|window, cx| ImageView::new(image_item.clone(), project, window, cx));

        let superseded_image = test_image(1);
        replace_image_and_draw(&image_item, superseded_image.clone(), cx);
        assert_ne!(
            displayed_source_id(&image_view, cx),
            Some(superseded_image.id()),
            "the superseded image should still be decoding"
        );

        let current_image = test_image(2);
        replace_image_and_draw(&image_item, current_image.clone(), cx);
        assert_ne!(
            displayed_source_id(&image_view, cx),
            Some(current_image.id()),
            "the current image should start decoding before the superseded decode completes"
        );

        cx.run_until_parked();
        draw_window(cx);

        assert_eq!(
            displayed_source_id(&image_view, cx),
            Some(current_image.id()),
            "the current image should finish decoding"
        );
        assert!(
            !image_is_cached(&superseded_image, cx),
            "the superseded image remained in GPUI's asset cache"
        );
    }

    #[gpui::test]
    async fn test_superseded_constructor_prefetch_is_removed_from_asset_cache(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let (project, image_item) = open_test_image(cx).await;
        let prefetched_image = cx.read(|cx| image_item.read(cx).image.clone());

        let cx = cx.add_empty_window();
        let image_view = cx.update(|window, cx| {
            cx.new(|cx| ImageView::new(image_item.clone(), project, window, cx))
        });

        let current_image = test_image(1);
        replace_image(&image_item, current_image, cx);
        cx.draw(point(px(0.), px(0.)), size(px(1.), px(1.)), |_, _| {
            image_view.clone().into_any_element()
        });
        cx.run_until_parked();

        assert!(
            !image_is_cached(&prefetched_image, cx),
            "the superseded constructor prefetch remained in GPUI's asset cache"
        );
    }

    #[gpui::test]
    async fn test_releasing_one_split_keeps_shared_atlas_entry(cx: &mut TestAppContext) {
        init_test(cx);
        let (project, image_item) = open_test_image(cx).await;

        let cx = cx.add_empty_window();
        let original_image_view = cx.update(|window, cx| {
            cx.new(|cx| ImageView::new(image_item.clone(), project, window, cx))
        });
        let split_image_view = original_image_view
            .update_in(cx, |image_view, window, cx| {
                image_view.clone_on_split(None, window, cx)
            })
            .await
            .expect("image view should support splitting");

        let draw_split_views = |cx: &mut VisualTestContext| {
            cx.draw(point(px(0.), px(0.)), size(px(2.), px(1.)), |_, _| {
                div()
                    .size_full()
                    .child(original_image_view.clone())
                    .child(split_image_view.clone())
            });
        };

        draw_split_views(cx);
        cx.run_until_parked();
        draw_split_views(cx);

        let original_render_image = displayed_render_image(&original_image_view, cx)
            .expect("the original image view should finish decoding");
        let split_render_image = displayed_render_image(&split_image_view, cx)
            .expect("the split image view should finish decoding");
        assert_eq!(
            original_render_image.id, split_render_image.id,
            "both image views should share the decoded render image"
        );
        assert!(
            cx.update(|window, _| window.has_image_atlas_entry(&original_render_image)),
            "the shared image should be present in the window atlas"
        );

        drop(original_image_view);
        cx.update(|_, _| {});
        cx.run_until_parked();

        assert!(
            cx.update(|window, _| window.has_image_atlas_entry(&split_render_image)),
            "releasing one image view removed an atlas entry still used by its split"
        );

        cx.draw(point(px(0.), px(0.)), size(px(1.), px(1.)), |_, _| {
            split_image_view.clone().into_any_element()
        });
    }
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

    db::static_connection!(ImageViewerDb, [WorkspaceDb]);

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
