mod pdf_renderer;
pub use pdf_renderer::{PdfDocument, PdfLoadError, RenderedPage};

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use anyhow::Context as _;
use editor::{EditorSettings, items::entry_git_aware_label_color};
use file_icons::FileIcons;
use gpui::*;
use language::File as _;
use project::pdf_store::PdfItemEvent;
use project::{PdfItem, Project, ProjectPath};
use settings::Settings;
use theme::ThemeSettings;
use ui::prelude::*;
use util::ResultExt;
use util::paths::PathExt;
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemLocation, Workspace, WorkspaceId,
    invalid_item_view::InvalidItemView,
    item::{HighlightedText, Item, ProjectItem, SerializableItem, TabContentParams},
};

actions!(
    pdf_viewer,
    [
        /// Zoom in the PDF view.
        ZoomIn,
        /// Zoom out the PDF view.
        ZoomOut,
        /// Reset zoom to 100%.
        ResetZoom,
        /// Fit the PDF page to the view.
        FitToView,
        /// Copy the text content of the current page.
        CopyPageText,
        /// Toggle between continuous scroll and single page mode.
        ToggleViewMode,
    ]
);

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 10.0;
const ZOOM_STEP: f32 = 1.1;
const SCROLL_LINE_MULTIPLIER: f32 = 40.0;
const DEFAULT_DPI: f32 = 144.0;
const PAGE_GAP: f32 = 12.0;
const PAGE_SHADOW_SIZE: f32 = 2.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    ContinuousScroll,
    SinglePage,
}

impl Default for ViewMode {
    fn default() -> Self {
        ViewMode::ContinuousScroll
    }
}

pub enum PdfViewEvent {
    TitleChanged,
}

impl EventEmitter<PdfViewEvent> for PdfView {}
impl EventEmitter<()> for PdfView {}

pub struct PdfView {
    pdf_item: Entity<PdfItem>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    document: Option<Arc<PdfDocument>>,
    load_error: Option<String>,
    zoom_level: f32,
    scroll_offset: f32,
    current_page: u32,
    view_mode: ViewMode,
    page_cache: HashMap<u32, Arc<RenderImage>>,
    cache_generation: u64,
    container_size: Option<Size<Pixels>>,
    _subscription: gpui::Subscription,
}

impl PdfView {
    pub fn new(
        pdf_item: Entity<PdfItem>,
        project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.subscribe(&pdf_item, Self::on_pdf_event);

        let mut this = Self {
            pdf_item,
            project,
            focus_handle: cx.focus_handle(),
            document: None,
            load_error: None,
            zoom_level: 1.0,
            scroll_offset: 0.0,
            current_page: 0,
            view_mode: ViewMode::default(),
            page_cache: HashMap::new(),
            cache_generation: 0,
            container_size: None,
            _subscription: subscription,
        };
        this.parse_document(cx);
        this
    }

    fn parse_document(&mut self, cx: &mut Context<Self>) {
        let data = self.pdf_item.read(cx).data.clone();

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { PdfDocument::open(data) })
                .await;

            this.update(cx, |this, cx| match result {
                Ok(document) => {
                    this.document = Some(Arc::new(document));
                    this.load_error = None;
                    this.page_cache.clear();
                    this.cache_generation += 1;
                    cx.notify();
                }
                Err(error) => {
                    this.load_error = Some(error.to_string());
                    this.document = None;
                    cx.notify();
                }
            })
            .log_err();
        })
        .detach();
    }

    fn on_pdf_event(&mut self, _: Entity<PdfItem>, event: &PdfItemEvent, cx: &mut Context<Self>) {
        match event {
            PdfItemEvent::Reloaded => {
                self.page_cache.clear();
                self.cache_generation += 1;
                self.parse_document(cx);
            }
            PdfItemEvent::FileHandleChanged => {
                cx.emit(PdfViewEvent::TitleChanged);
                cx.notify();
            }
        }
    }

    fn page_count(&self) -> u32 {
        self.document
            .as_ref()
            .map(|document| document.page_count())
            .unwrap_or(0)
    }

    fn effective_dpi(&self) -> f32 {
        DEFAULT_DPI * self.zoom_level
    }

    fn page_pixel_size(&self, page_index: u32) -> Option<(f32, f32)> {
        let document = self.document.as_ref()?;
        let (width_points, height_points) = document.page_dimensions(page_index).ok()?;
        let scale = self.zoom_level * DEFAULT_DPI / 72.0;
        Some((width_points as f32 * scale, height_points as f32 * scale))
    }

    fn visible_page_range(&self) -> std::ops::Range<u32> {
        let page_count = self.page_count();
        if page_count == 0 {
            return 0..0;
        }

        if self.view_mode == ViewMode::SinglePage {
            let page = self.current_page.min(page_count.saturating_sub(1));
            return page..page + 1;
        }

        let container_height = self
            .container_size
            .map(|size| f32::from(size.height))
            .unwrap_or(800.0);

        let scroll = self.scroll_offset;
        let mut accumulated_height: f32 = 0.0;
        let mut first_visible = None;
        let mut last_visible = 0u32;

        for page_index in 0..page_count {
            let (_, page_height) = self.page_pixel_size(page_index).unwrap_or((600.0, 800.0));
            let page_top = accumulated_height;
            let page_bottom = accumulated_height + page_height;
            accumulated_height = page_bottom + PAGE_GAP;

            if page_bottom > scroll && page_top < scroll + container_height {
                if first_visible.is_none() {
                    first_visible = Some(page_index);
                }
                last_visible = page_index;
            }
        }

        let first = first_visible.unwrap_or(0);
        let buffer_start = first.saturating_sub(1);
        let buffer_end = (last_visible + 2).min(page_count);

        buffer_start..buffer_end
    }

    fn total_content_height(&self) -> f32 {
        let page_count = self.page_count();
        if page_count == 0 {
            return 0.0;
        }

        let mut total: f32 = 0.0;
        for page_index in 0..page_count {
            let (_, page_height) = self.page_pixel_size(page_index).unwrap_or((600.0, 800.0));
            total += page_height;
        }
        total += PAGE_GAP * (page_count.saturating_sub(1)) as f32;
        total
    }

    fn render_visible_pages(&mut self, cx: &mut Context<Self>) {
        let Some(document) = self.document.clone() else {
            return;
        };

        let visible_range = self.visible_page_range();
        let dpi = self.effective_dpi();
        let generation = self.cache_generation;

        for page_index in visible_range.clone() {
            if self.page_cache.contains_key(&page_index) {
                continue;
            }

            let document = document.clone();
            cx.spawn({
                let page_index = page_index;
                async move |this, cx| {
                    let rendered = cx
                        .background_executor()
                        .spawn(async move { document.render_page(page_index, dpi) })
                        .await;

                    if let Ok(rendered_page) = rendered {
                        let image = rendered_page.into_render_image();
                        this.update(cx, |this, cx| {
                            if this.cache_generation == generation {
                                this.page_cache.insert(page_index, image);
                                cx.notify();
                            }
                        })
                        .log_err();
                    }
                }
            })
            .detach();
        }

        let pages_to_evict: Vec<u32> = self
            .page_cache
            .keys()
            .copied()
            .filter(|page_index| !visible_range.contains(page_index))
            .collect();
        for page_index in pages_to_evict {
            self.page_cache.remove(&page_index);
        }
    }

    fn set_zoom(&mut self, new_zoom: f32, cx: &mut Context<Self>) {
        let old_zoom = self.zoom_level;
        self.zoom_level = new_zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        if (self.zoom_level - old_zoom).abs() > f32::EPSILON {
            self.page_cache.clear();
            self.cache_generation += 1;
            let ratio = self.zoom_level / old_zoom;
            self.scroll_offset *= ratio;
            cx.notify();
        }
    }

    fn zoom_in(&mut self, _: &ZoomIn, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom_level * ZOOM_STEP, cx);
    }

    fn zoom_out(&mut self, _: &ZoomOut, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom_level / ZOOM_STEP, cx);
    }

    fn reset_zoom(&mut self, _: &ResetZoom, _window: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(1.0, cx);
    }

    fn fit_to_view(&mut self, _: &FitToView, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(container_size) = self.container_size else {
            return;
        };
        let page_index = self.current_page.min(self.page_count().saturating_sub(1));
        let Some((page_width_points, _)) = self
            .document
            .as_ref()
            .and_then(|document| document.page_dimensions(page_index).ok())
        else {
            return;
        };

        let container_width: f32 = container_size.width.into();
        let page_width_at_zoom_1 = page_width_points as f32 * DEFAULT_DPI / 72.0;
        if page_width_at_zoom_1 > 0.0 {
            let fit_zoom = (container_width - PAGE_SHADOW_SIZE * 2.0 - 20.0) / page_width_at_zoom_1;
            self.set_zoom(fit_zoom.min(MAX_ZOOM), cx);
        }
    }

    fn toggle_view_mode(
        &mut self,
        _: &ToggleViewMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.view_mode = match self.view_mode {
            ViewMode::ContinuousScroll => ViewMode::SinglePage,
            ViewMode::SinglePage => ViewMode::ContinuousScroll,
        };
        cx.notify();
    }

    fn copy_page_text(&mut self, _: &CopyPageText, _window: &mut Window, cx: &mut Context<Self>) {
        self.copy_all_text(cx);
    }

    fn copy_all_text(&mut self, cx: &mut Context<Self>) {
        let Some(document) = self.document.as_ref() else {
            return;
        };
        let mut all_text = String::new();
        for page_index in 0..document.page_count() {
            if let Ok(text) = document.extract_page_text(page_index) {
                if !text.is_empty() {
                    if !all_text.is_empty() {
                        all_text.push('\n');
                    }
                    all_text.push_str(&text);
                }
            }
        }
        if !all_text.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(all_text));
        }
    }

    fn handle_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Ctrl+scroll or Cmd+scroll = zoom (like typical PDF viewers)
        if event.modifiers.control || event.modifiers.platform {
            let delta: f32 = match event.delta {
                ScrollDelta::Pixels(pixels) => pixels.y.into(),
                ScrollDelta::Lines(lines) => lines.y * SCROLL_LINE_MULTIPLIER,
            };
            let zoom_factor = if delta > 0.0 {
                ZOOM_STEP
            } else {
                1.0 / ZOOM_STEP
            };
            self.set_zoom(self.zoom_level * zoom_factor, cx);
        }
        // Plain scroll is handled natively by GPUI's overflow_y_scroll
    }

    fn update_current_page_from_scroll(&mut self) {
        let page_count = self.page_count();
        if page_count == 0 {
            return;
        }

        let viewport_center = self.scroll_offset
            + self
                .container_size
                .map(|size| f32::from(size.height) / 2.0)
                .unwrap_or(400.0);

        let mut accumulated_height: f32 = 0.0;
        for page_index in 0..page_count {
            let (_, page_height) = self.page_pixel_size(page_index).unwrap_or((600.0, 800.0));
            let page_center = accumulated_height + page_height / 2.0;
            accumulated_height += page_height + PAGE_GAP;

            if page_center >= viewport_center {
                self.current_page = page_index;
                return;
            }
        }
        self.current_page = page_count.saturating_sub(1);
    }

    fn render_toolbar(&self, cx: &Context<Self>) -> AnyElement {
        let page_count = self.page_count();
        let current_display = if page_count > 0 {
            format!("Page {} / {}", self.current_page + 1, page_count)
        } else {
            "Loading...".to_string()
        };

        let zoom_percentage = format!("{}%", (self.zoom_level * 100.0).round() as i32);

        let mode_label = match self.view_mode {
            ViewMode::ContinuousScroll => "Scroll",
            ViewMode::SinglePage => "Single",
        };

        let colors = cx.theme().colors();

        h_flex()
            .w_full()
            .justify_between()
            .px_2()
            .py_1()
            .bg(colors.title_bar_background)
            .border_b_1()
            .border_color(colors.border)
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new(current_display)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(mode_label)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                h_flex().gap_1().child(
                    Label::new(zoom_percentage)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
            .into_any_element()
    }

    fn render_pages(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let page_count = self.page_count();
        if page_count == 0 {
            if let Some(error) = &self.load_error {
                return div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Label::new(error.clone()).color(Color::Error))
                    .into_any_element();
            }
            return div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("Loading PDF...").color(Color::Muted))
                .into_any_element();
        }

        self.render_visible_pages(cx);

        match self.view_mode {
            ViewMode::ContinuousScroll => self.render_continuous_scroll(cx),
            ViewMode::SinglePage => self.render_single_page(cx),
        }
    }

    fn render_continuous_scroll(&self, _cx: &mut Context<Self>) -> AnyElement {
        let page_count = self.page_count();
        let mut pages_container = v_flex().w_full().items_center().p_2();

        for page_index in 0..page_count {
            let (page_width, page_height) =
                self.page_pixel_size(page_index).unwrap_or((600.0, 800.0));

            let page_element = self.render_page_element(page_index, page_width, page_height);
            pages_container = pages_container
                .child(page_element)
                .child(div().h(px(PAGE_GAP)));
        }

        div()
            .id("pdf-scroll-container")
            .size_full()
            .overflow_y_scroll()
            .child(pages_container)
            .into_any_element()
    }

    fn render_single_page(&self, _cx: &mut Context<Self>) -> AnyElement {
        let page_index = self.current_page.min(self.page_count().saturating_sub(1));
        let (page_width, page_height) = self.page_pixel_size(page_index).unwrap_or((600.0, 800.0));

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(self.render_page_element(page_index, page_width, page_height))
            .into_any_element()
    }

    fn render_page_element(
        &self,
        page_index: u32,
        page_width: f32,
        page_height: f32,
    ) -> AnyElement {
        let page_content = if let Some(cached_image) = self.page_cache.get(&page_index) {
            div()
                .w(px(page_width))
                .h(px(page_height))
                .child(
                    img(ImageSource::Render(cached_image.clone()))
                        .w(px(page_width))
                        .h(px(page_height)),
                )
                .into_any_element()
        } else {
            div()
                .w(px(page_width))
                .h(px(page_height))
                .flex()
                .items_center()
                .justify_center()
                .child(Label::new("Rendering...").color(Color::Muted))
                .into_any_element()
        };

        div()
            .bg(gpui::white())
            .shadow_md()
            .rounded(px(PAGE_SHADOW_SIZE))
            .child(page_content)
            .into_any_element()
    }
}

impl Focusable for PdfView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PdfView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.container_size = None;

        let toolbar = self.render_toolbar(cx);
        let pages = self.render_pages(cx);

        div()
            .id("pdf-viewer")
            .track_focus(&self.focus_handle(cx))
            .key_context("PdfViewer")
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::reset_zoom))
            .on_action(cx.listener(Self::fit_to_view))
            .on_action(cx.listener(Self::copy_page_text))
            .on_action(cx.listener(Self::toggle_view_mode))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex().size_full().child(toolbar).child(
                    div()
                        .id("pdf-pages-container")
                        .flex_1()
                        .overflow_hidden()
                        .relative()
                        .on_scroll_wheel(cx.listener(Self::handle_scroll_wheel))
                        .child(pages)
                        .child(
                            // Floating copy button in top-right corner
                            div()
                                .absolute()
                                .top_2()
                                .right_2()
                                .child(
                                    div()
                                        .id("pdf-copy-button")
                                        .rounded_md()
                                        .px_3()
                                        .py_1()
                                        .bg(cx.theme().colors().element_background)
                                        .border_1()
                                        .border_color(cx.theme().colors().border)
                                        .hover(|style| style.bg(cx.theme().colors().element_hover))
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _, _window, cx| {
                                            this.copy_all_text(cx);
                                        }))
                                        .child(
                                            Label::new("Copy All Text")
                                                .size(LabelSize::Small)
                                        )
                                ),
                        ),
                ),
            )
    }
}

impl Item for PdfView {
    type Event = PdfViewEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        match event {
            PdfViewEvent::TitleChanged => {
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
        f(self.pdf_item.entity_id(), self.pdf_item.read(cx))
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let abs_path = self.pdf_item.read(cx).abs_path(cx)?;
        let file_path = abs_path.compact().to_string_lossy().into_owned();
        Some(file_path.into())
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        let project_path = self.pdf_item.read(cx).project_path(cx);

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
        self.pdf_item
            .read(cx)
            .file_name(cx)
            .to_string()
            .into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = self.pdf_item.read(cx).abs_path(cx)?;
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
        let text = breadcrumbs_text_for_pdf(self.project.read(cx), self.pdf_item.read(cx), cx);
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
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        let pdf_item = self.pdf_item.clone();
        let project = self.project.clone();
        let document = self.document.clone();
        let zoom_level = self.zoom_level;
        let current_page = self.current_page;
        let view_mode = self.view_mode;

        Task::ready(Some(cx.new(|cx| {
            let subscription = cx.subscribe(&pdf_item, Self::on_pdf_event);
            Self {
                pdf_item,
                project,
                focus_handle: cx.focus_handle(),
                document,
                load_error: None,
                zoom_level,
                scroll_offset: 0.0,
                current_page,
                view_mode,
                page_cache: HashMap::new(),
                cache_generation: 0,
                container_size: None,
                _subscription: subscription,
            }
        })))
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.pdf_item.read(cx).file.disk_state().is_deleted()
    }

    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }
}

fn breadcrumbs_text_for_pdf(project: &Project, pdf: &PdfItem, cx: &App) -> String {
    let mut path = pdf.file.path().clone();
    if project.visible_worktrees(cx).count() > 1
        && let Some(worktree) = project.worktree_for_id(pdf.project_path(cx).worktree_id, cx)
    {
        path = worktree.read(cx).root_name().join(&path);
    }

    path.display(project.path_style(cx)).to_string()
}

impl ProjectItem for PdfView {
    type Item = PdfItem;

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
        error: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView>
    where
        Self: Sized,
    {
        Some(InvalidItemView::new(abs_path, is_local, error, window, cx))
    }
}

impl SerializableItem for PdfView {
    fn serialized_item_kind() -> &'static str {
        "PdfView"
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
            let pdf_path = persistence::PDF_VIEWER
                .get_pdf_path(item_id, workspace_id)?
                .context("No PDF path found")?;

            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(pdf_path.clone(), false, cx)
                })
                .await
                .context("Path not found")?;
            let worktree_id = worktree.update(cx, |worktree, _cx| worktree.id());

            let project_path = ProjectPath {
                worktree_id,
                path: relative_path,
            };

            let pdf_item = project
                .update(cx, |project, cx| project.open_pdf(project_path, cx))
                .await?;

            cx.update(
                |window, cx| Ok(cx.new(|cx| PdfView::new(pdf_item, project, window, cx))),
            )?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "pdf_viewers",
            &persistence::PDF_VIEWER,
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
        let pdf_path = self.pdf_item.read(cx).abs_path(cx)?;

        Some(cx.background_spawn(async move {
            persistence::PDF_VIEWER
                .save_pdf_path(item_id, workspace_id, pdf_path)
                .await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<PdfView>(cx);
    workspace::register_serializable_item::<PdfView>(cx);
}

mod persistence {
    use std::path::PathBuf;
    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct PdfViewerDb(ThreadSafeConnection);

    impl Domain for PdfViewerDb {
        const NAME: &str = stringify!(PdfViewerDb);
        const MIGRATIONS: &[&str] = &[sql!(
            CREATE TABLE pdf_viewers (
                workspace_id INTEGER,
                item_id INTEGER UNIQUE,
                pdf_path BLOB,
                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
    }

    db::static_connection!(PDF_VIEWER, PdfViewerDb, [WorkspaceDb]);

    impl PdfViewerDb {
        query! {
            pub async fn save_pdf_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                pdf_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO pdf_viewers(item_id, workspace_id, pdf_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_pdf_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT pdf_path
                FROM pdf_viewers
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
