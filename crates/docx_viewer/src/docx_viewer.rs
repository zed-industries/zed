mod docx_document;
mod docx_item;

use std::path::Path;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use editor::{EditorSettings, RevealInFileManager, items::entry_git_aware_label_color};
use file_icons::FileIcons;
use gpui::{
    AnyElement, App, AppContext as _, ClipboardItem, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Font, FontStyle, FontWeight, HighlightStyle, Hsla, KeyDownEvent, MouseButton,
    PinchEvent, Pixels, ScrollDelta, ScrollEvent, ScrollHandle, StrikethroughStyle, StyledText,
    Subscription, Task, UnderlineStyle, WeakEntity, Window, actions, div, point, px,
};
use project::{Project, ProjectPath, git_store::GitStoreEvent};
use settings::Settings as _;
use theme_settings::ThemeSettings;
use ui::{Scrollbars, Tooltip, WithScrollbar as _, prelude::*};
use util::paths::PathExt as _;
use workspace::{
    ItemId, ItemSettings, Pane, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    WorkspaceId, delete_unloaded_items,
    invalid_item_view::InvalidItemView,
    item::{HighlightedText, Item, ItemHandle, ProjectItem, SerializableItem, TabContentParams},
};

use crate::docx_document::{Block, DocxDocument, Run};
pub use crate::docx_item::{DocxItem, DocxItemEvent, is_docx_file};

actions!(
    docx_viewer,
    [
        /// Zoom in the document.
        ZoomIn,
        /// Zoom out the document.
        ZoomOut,
        /// Reset the text size to 100%.
        ResetZoom,
        /// Copy the document text to the clipboard.
        CopyAllText,
    ]
);

const MIN_ZOOM: f32 = 0.5;
const MAX_ZOOM: f32 = 4.0;
const ZOOM_STEP: f32 = 1.1;
const SCROLL_ZOOM_SENSITIVITY: f32 = 0.01;
const SCROLL_LINE_HEIGHT: f32 = 20.0;
const KEY_SCROLL_STEP: Pixels = px(40.0);
/// Text size of body text at 100% zoom.
const BASE_FONT_SIZE: f32 = 16.0;
/// Body text size multipliers for heading levels 1-6.
const HEADING_SCALE: [f32; 6] = [2.0, 1.6, 1.4, 1.2, 1.1, 1.0];
/// Maximum width of the document column; wider views keep the text centered.
const CONTENT_MAX_WIDTH: Pixels = px(880.0);

pub struct DocxViewer {
    docx_item: Entity<DocxItem>,
    project: Entity<Project>,
    document: Arc<DocxDocument>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    zoom: f32,
    _subscriptions: Vec<Subscription>,
}

pub enum DocxViewEvent {
    TitleChanged,
}

impl DocxViewer {
    pub fn new(
        docx_item: Entity<DocxItem>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let document = docx_item.read(cx).document().clone();
        let git_store = project.read(cx).git_store().clone();
        let subscriptions = vec![
            cx.subscribe_in(&docx_item, window, Self::on_docx_item_event),
            cx.subscribe(&git_store, |_, _, event, cx| {
                if matches!(event, GitStoreEvent::DiffBaseChanged(_)) {
                    cx.emit(DocxViewEvent::TitleChanged);
                }
            }),
        ];
        Self {
            docx_item,
            project,
            document,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            zoom: 1.0,
            _subscriptions: subscriptions,
        }
    }

    fn on_docx_item_event(
        &mut self,
        docx_item: &Entity<DocxItem>,
        event: &DocxItemEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            DocxItemEvent::Reloaded => {
                self.document = docx_item.read(cx).document().clone();
                cx.notify();
            }
            DocxItemEvent::FileHandleChanged => {
                cx.emit(DocxViewEvent::TitleChanged);
                cx.notify();
            }
        }
    }

    fn set_zoom(&mut self, zoom: f32, cx: &mut Context<Self>) {
        let zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
        if (zoom - self.zoom).abs() < f32::EPSILON {
            return;
        }
        self.zoom = zoom;
        cx.notify();
    }

    fn zoom_in(&mut self, _: &ZoomIn, _: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom * ZOOM_STEP, cx);
    }

    fn zoom_out(&mut self, _: &ZoomOut, _: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom / ZOOM_STEP, cx);
    }

    fn reset_zoom(&mut self, _: &ResetZoom, _: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(1.0, cx);
    }

    fn reveal_in_file_manager(
        &mut self,
        _: &RevealInFileManager,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(path) = self.docx_item.read(cx).abs_path(cx) {
            self.project
                .update(cx, |project, cx| project.reveal_path(&path, cx));
        }
    }

    fn copy_all_text(&mut self, _: &CopyAllText, _: &mut Window, cx: &mut Context<Self>) {
        let text = self.document.extract_text();
        if !text.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(text));
        }
    }

    fn handle_scroll(&mut self, event: &ScrollEvent, _: &mut Window, cx: &mut Context<Self>) {
        if !(event.modifiers.control || event.modifiers.platform) {
            return;
        }
        cx.stop_propagation();
        let delta = match event.delta {
            ScrollDelta::Pixels(pixels) => f32::from(pixels.y),
            ScrollDelta::Lines(lines) => lines.y * SCROLL_LINE_HEIGHT,
        };
        let factor = 1.0 + delta.abs() * SCROLL_ZOOM_SENSITIVITY;
        let zoom = if delta > 0.0 {
            self.zoom * factor
        } else {
            self.zoom / factor
        };
        self.set_zoom(zoom, cx);
    }

    fn handle_pinch(&mut self, event: &PinchEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.set_zoom(self.zoom * (1.0 + event.delta), cx);
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let modifiers = &event.keystroke.modifiers;
        if modifiers.control || modifiers.platform || modifiers.alt {
            return;
        }
        let viewport = self.scroll_handle.bounds().size;
        let page_step = viewport.height * 0.9;
        let offset = self.scroll_handle.offset();
        let max_offset = self.scroll_handle.max_offset();
        let new_y = match (event.keystroke.key.as_str(), modifiers.shift) {
            ("up", _) => offset.y + KEY_SCROLL_STEP,
            ("down", _) => offset.y - KEY_SCROLL_STEP,
            ("pageup", _) | ("space", true) => offset.y + page_step,
            ("pagedown", _) | ("space", false) => offset.y - page_step,
            ("home", _) => Pixels::ZERO,
            ("end", _) => -max_offset.y,
            _ => return,
        };
        cx.stop_propagation();
        self.scroll_handle
            .set_offset(point(offset.x, new_y.clamp(-max_offset.y, Pixels::ZERO)));
        cx.notify();
    }

    fn render_block(&self, block: &Block, base: f32, muted: Hsla, border: Hsla) -> AnyElement {
        match block {
            Block::Heading { level, runs } => {
                let level = (*level).clamp(1, 6) as usize;
                let size = base * HEADING_SCALE[level - 1];
                div()
                    .mb(px(base * 0.4))
                    .when(level > 1, |element| element.mt(px(base * 0.4)))
                    .text_size(px(size))
                    .font_weight(FontWeight::BOLD)
                    .line_height(px(size * 1.3))
                    .child(styled_runs(runs))
                    .into_any_element()
            }
            Block::Paragraph { runs } => {
                if runs.is_empty() {
                    return div().h(px(base * 0.5)).into_any_element();
                }
                div()
                    .mb(px(base * 0.55))
                    .line_height(px(base * 1.5))
                    .child(styled_runs(runs))
                    .into_any_element()
            }
            Block::ListItem {
                indent,
                marker,
                runs,
                ..
            } => div()
                .mb(px(base * 0.3))
                .ml(px(*indent as f32 * base * 1.75))
                .line_height(px(base * 1.5))
                .child(
                    h_flex()
                        .child(
                            div()
                                .w(px(base * 1.6))
                                .flex_none()
                                .text_color(muted)
                                .child(marker.clone()),
                        )
                        .child(div().flex_1().min_w_0().child(styled_runs(runs))),
                )
                .into_any_element(),
            Block::Table { rows } => {
                let mut table = div()
                    .flex()
                    .flex_col()
                    .mb(px(base * 0.6))
                    .border_1()
                    .border_color(border)
                    .rounded_md()
                    .overflow_hidden();
                for (row_index, row) in rows.iter().enumerate() {
                    let mut row_element = h_flex().min_w_0();
                    if row_index > 0 {
                        row_element = row_element.border_t_1().border_color(border);
                    }
                    for (cell_index, cell) in row.iter().enumerate() {
                        let mut cell_element = div()
                            .flex_1()
                            .min_w_0()
                            .p(px(base * 0.35))
                            .text_size(px(base * 0.9))
                            .line_height(px(base * 0.9 * 1.4));
                        if cell_index > 0 {
                            cell_element = cell_element.border_l_1().border_color(border);
                        }
                        cell_element = cell_element.children(
                            cell.paragraphs
                                .iter()
                                .map(|runs| styled_runs(runs).into_any_element()),
                        );
                        row_element = row_element.child(cell_element);
                    }
                    table = table.child(row_element);
                }
                table.into_any_element()
            }
        }
    }
}

/// Builds a styled text element from DOCX runs, mapping run properties to text highlights.
fn styled_runs(runs: &[Run]) -> StyledText {
    let mut text = String::new();
    let mut highlights = Vec::new();
    for run in runs {
        if run.text.is_empty() {
            continue;
        }
        let start = text.len();
        text.push_str(&run.text);
        let highlight = HighlightStyle {
            font_weight: run.props.bold.then_some(FontWeight::BOLD),
            font_style: run.props.italic.then_some(FontStyle::Italic),
            underline: run.props.underline.then(|| UnderlineStyle {
                thickness: px(1.0),
                color: None,
                wavy: false,
            }),
            strikethrough: run.props.strikethrough.then(|| StrikethroughStyle {
                thickness: px(1.0),
                color: None,
            }),
            ..Default::default()
        };
        highlights.push((start..text.len(), highlight));
    }
    StyledText::new(text).with_highlights(highlights)
}

fn format_word_count(count: usize) -> String {
    let digits = count.to_string();
    let mut formatted = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(digit);
    }
    format!("{formatted} words")
}

impl EventEmitter<DocxViewEvent> for DocxViewer {}

impl Focusable for DocxViewer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DocxViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let base = BASE_FONT_SIZE * self.zoom;
        let text_color = cx.theme().colors().text;
        let muted_color = cx.theme().colors().text_muted;
        let border_color = cx.theme().colors().border;

        let content = if self.document.blocks.is_empty() {
            div()
                .flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("This document has no readable text.").color(Color::Muted))
                .into_any_element()
        } else {
            let mut column = div()
                .w_full()
                .max_w(CONTENT_MAX_WIDTH)
                .text_color(text_color);
            for block in &self.document.blocks {
                column = column.child(self.render_block(block, base, muted_color, border_color));
            }
            div()
                .flex()
                .justify_center()
                .py_8()
                .px_8()
                .child(column)
                .into_any_element()
        };

        div()
            .key_context("DocxViewer")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(Self::zoom_in))
            .on_action(cx.listener(Self::zoom_out))
            .on_action(cx.listener(Self::reset_zoom))
            .on_action(cx.listener(Self::reveal_in_file_manager))
            .on_action(cx.listener(Self::copy_all_text))
            .on_key_down(cx.listener(Self::handle_key_down))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                div()
                    .id("docx-content")
                    .size_full()
                    .overflow_scroll()
                    .track_scroll(&self.scroll_handle)
                    .on_scroll(cx.listener(Self::handle_scroll))
                    .on_pinch(cx.listener(Self::handle_pinch))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, window, cx| {
                            this.focus_handle.focus(window, cx);
                        }),
                    )
                    .child(content)
                    .custom_scrollbars(
                        Scrollbars::new(ui::ScrollAxes::Both)
                            .tracked_scroll_handle(&self.scroll_handle)
                            .id("docx-scrollbars"),
                        window,
                        cx,
                    ),
            )
    }
}

impl Item for DocxViewer {
    type Event = DocxViewEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        match event {
            DocxViewEvent::TitleChanged => {
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
        f(self.docx_item.entity_id(), self.docx_item.read(cx))
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let abs_path = self.docx_item.read(cx).abs_path(cx)?;
        Some(abs_path.compact().to_string_lossy().into_owned().into())
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> AnyElement {
        let project_path = self.docx_item.read(cx).project_path(cx);
        let label_color = if ItemSettings::get_global(cx).git_status {
            let project = self.project.read(cx);
            let git_status = project
                .git_store()
                .read(cx)
                .display_status_for_project_path(&project_path, cx)
                .map(|status| status.summary())
                .unwrap_or_default();
            project
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
        self.docx_item.read(cx).file_name(cx).into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let path = self.docx_item.read(cx).abs_path(cx)?;
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(&path, cx))
            .flatten()
            .map(Icon::from_path)
    }

    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation {
        if EditorSettings::get_global(cx).toolbar.breadcrumbs {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)> {
        let project = self.project.read(cx);
        let docx_item = self.docx_item.read(cx);
        let mut path = docx_item.file().path.to_rel_path_buf();
        if project.visible_worktrees(cx).count() > 1
            && let Some(worktree) =
                project.worktree_for_id(docx_item.project_path(cx).worktree_id, cx)
        {
            path = worktree.read(cx).root_name().join(&path);
        }
        let text = path.display(project.path_style(cx)).to_string();
        Some((
            vec![HighlightedText {
                text: text.into(),
                highlights: vec![],
            }],
            Some(ThemeSettings::get_global(cx).buffer_font.clone()),
        ))
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>> {
        let docx_item = self.docx_item.clone();
        let project = self.project.clone();
        let zoom = self.zoom;
        Task::ready(Some(cx.new(|cx| {
            let mut viewer = Self::new(docx_item, project, window, cx);
            viewer.zoom = zoom;
            viewer
        })))
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.docx_item.read(cx).file().disk_state.is_deleted()
    }

    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }
}

impl ProjectItem for DocxViewer {
    type Item = DocxItem;

    fn for_project_item(
        project: Entity<Project>,
        _: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(item, project, window, cx)
    }

    fn for_broken_project_item(
        abs_path: &Path,
        is_local: bool,
        error: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView> {
        Some(InvalidItemView::new(abs_path, is_local, error, window, cx))
    }
}

impl SerializableItem for DocxViewer {
    fn serialized_item_kind() -> &'static str {
        "DocxViewer"
    }

    fn deserialize(
        project: Entity<Project>,
        _: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let db = DocxViewerDb::global(cx);
        window.spawn(cx, async move |cx| {
            let docx_path = db
                .get_docx_path(item_id, workspace_id)?
                .context("No DOCX path found")?;
            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(docx_path, false, cx)
                })
                .await
                .context("Path not found")?;
            let project_path = ProjectPath {
                worktree_id: worktree.update(cx, |worktree, _| worktree.id()),
                path: relative_path,
            };
            let docx_item = cx
                .update(|_, cx| DocxItem::load(&project, &project_path, cx))?
                .await?;
            cx.update(|window, cx| cx.new(|cx| Self::new(docx_item, project, window, cx)))
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let db = DocxViewerDb::global(cx);
        delete_unloaded_items(alive_items, workspace_id, "docx_viewers", &db, cx)
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let docx_path = self.docx_item.read(cx).abs_path(cx)?;
        let db = DocxViewerDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_docx_path(item_id, workspace_id, docx_path).await
        }))
    }

    fn should_serialize(&self, _: &Self::Event) -> bool {
        false
    }
}

pub struct DocxViewToolbarControls {
    docx_viewer: Option<WeakEntity<DocxViewer>>,
    _subscription: Option<Subscription>,
}

impl DocxViewToolbarControls {
    pub fn new() -> Self {
        Self {
            docx_viewer: None,
            _subscription: None,
        }
    }
}

impl Render for DocxViewToolbarControls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(docx_viewer) = self
            .docx_viewer
            .as_ref()
            .and_then(|viewer| viewer.upgrade())
        else {
            return div().into_any_element();
        };
        let viewer = docx_viewer.read(cx);
        let words_label = format_word_count(viewer.document.word_count);
        let zoom_label = format!("{}%", (viewer.zoom * 100.0).round() as i32);
        let docx_viewer = docx_viewer.downgrade();
        let dispatch = move |action: fn(&mut DocxViewer, &mut Window, &mut Context<DocxViewer>)| {
            let docx_viewer = docx_viewer.clone();
            move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                docx_viewer
                    .update(cx, |viewer, cx| action(viewer, window, cx))
                    .ok();
            }
        };

        h_flex()
            .gap_1()
            .child(
                Label::new(words_label)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                IconButton::new("zoom-out", IconName::Dash)
                    .icon_size(IconSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Zoom Out", &ZoomOut, cx))
                    .on_click(dispatch(|viewer, window, cx| {
                        viewer.zoom_out(&ZoomOut, window, cx)
                    })),
            )
            .child(
                Button::new("zoom-level", zoom_label)
                    .label_size(LabelSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Reset Zoom", &ResetZoom, cx))
                    .on_click(dispatch(|viewer, window, cx| {
                        viewer.reset_zoom(&ResetZoom, window, cx)
                    })),
            )
            .child(
                IconButton::new("zoom-in", IconName::Plus)
                    .icon_size(IconSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Zoom In", &ZoomIn, cx))
                    .on_click(dispatch(|viewer, window, cx| {
                        viewer.zoom_in(&ZoomIn, window, cx)
                    })),
            )
            .child(
                IconButton::new("copy-text", IconName::Copy)
                    .icon_size(IconSize::Small)
                    .tooltip(|_, cx| Tooltip::for_action("Copy All Text", &CopyAllText, cx))
                    .on_click(dispatch(|viewer, window, cx| {
                        viewer.copy_all_text(&CopyAllText, window, cx)
                    })),
            )
            .into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for DocxViewToolbarControls {}

impl ToolbarItemView for DocxViewToolbarControls {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.docx_viewer = None;
        self._subscription = None;

        if let Some(item) = active_pane_item.and_then(|item| item.downcast::<DocxViewer>()) {
            self._subscription = Some(cx.observe(&item, |_, _, cx| cx.notify()));
            self.docx_viewer = Some(item.downgrade());
            cx.notify();
            return ToolbarItemLocation::PrimaryRight;
        }

        ToolbarItemLocation::Hidden
    }
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<DocxViewer>(cx);
    workspace::register_serializable_item::<DocxViewer>(cx);
}

mod persistence {
    use std::path::PathBuf;

    use db::{
        query,
        sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
        sqlez_macros::sql,
    };
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    pub struct DocxViewerDb(ThreadSafeConnection);

    impl Domain for DocxViewerDb {
        const NAME: &str = stringify!(DocxViewerDb);

        const MIGRATIONS: &[&str] = &[sql!(
                CREATE TABLE docx_viewers (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    docx_path BLOB,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
        )];
    }

    db::static_connection!(DocxViewerDb, [WorkspaceDb]);

    impl DocxViewerDb {
        query! {
            pub async fn save_docx_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                docx_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO docx_viewers(item_id, workspace_id, docx_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_docx_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT docx_path
                FROM docx_viewers
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

use persistence::DocxViewerDb;

#[cfg(test)]
mod tests {
    use fs::{FakeFs, Fs as _};
    use gpui::{TestAppContext, VisualTestContext};
    use settings::SettingsStore;
    use util::{path, rel_path::rel_path};
    use workspace::item::ItemHandle;

    use super::*;
    use crate::docx_document::tests::{build_docx, document_xml};

    fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            cx.set_global(db::AppDatabase::test_new());
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            init(cx);
        });
        FakeFs::new(cx.executor())
    }

    async fn open_in_workspace<'a>(
        fs: Arc<FakeFs>,
        file_name: &str,
        cx: &'a mut TestAppContext,
    ) -> (Result<Box<dyn ItemHandle>>, &'a mut VisualTestContext) {
        let project = Project::test(fs, [Path::new(path!("/root"))], cx).await;
        let worktree_id = cx.update(|cx| {
            project
                .read(cx)
                .worktrees(cx)
                .next()
                .expect("test project should contain a worktree")
                .read(cx)
                .id()
        });
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let item = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path(file_name)), None, true, window, cx)
            })
            .await;
        cx.run_until_parked();
        (item, cx)
    }

    async fn insert_docx(fs: &FakeFs, file_name: &str, body: &str) {
        fs.create_dir(Path::new(path!("/root")))
            .await
            .expect("test root should be created");
        let document = document_xml(body);
        fs.insert_file(
            Path::new(path!("/root")).join(file_name),
            build_docx(&[("word/document.xml", &document)]),
        )
        .await;
    }

    #[gpui::test]
    async fn test_docx_files_open_in_the_docx_viewer(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        insert_docx(
            &fs,
            "Report.DOCX",
            r#"
<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Quarterly report</w:t></w:r></w:p>
<w:p><w:r><w:t>Revenue grew, </w:t></w:r><w:r><w:rPr><w:b/></w:rPr><w:t>profit doubled</w:t></w:r></w:p>"#,
        )
        .await;

        let (item, cx) = open_in_workspace(fs, "Report.DOCX", cx).await;
        let docx_viewer = item
            .expect("DOCX should open")
            .downcast::<DocxViewer>()
            .expect("DOCX should open in the DOCX viewer");

        docx_viewer.read_with(cx, |docx_viewer, cx| {
            assert_eq!(docx_viewer.tab_content_text(0, cx), "Report.DOCX");
            assert_eq!(docx_viewer.document.blocks.len(), 2);
            assert_eq!(docx_viewer.document.word_count, 6);
            assert!(
                matches!(
                    &docx_viewer.document.blocks[0],
                    Block::Heading { level: 1, .. }
                ),
                "first block should be a level 1 heading, got {:?}",
                docx_viewer.document.blocks[0]
            );
        });
    }

    #[gpui::test]
    async fn test_invalid_docx_shows_an_error_item(cx: &mut TestAppContext) {
        let fs = init_test(cx);
        fs.create_dir(Path::new(path!("/root")))
            .await
            .expect("test root should be created");
        fs.insert_file(
            Path::new(path!("/root/broken.docx")),
            b"this is not a zip archive".to_vec(),
        )
        .await;

        let (item, _cx) = open_in_workspace(fs, "broken.docx", cx).await;
        let item = item.expect("broken DOCX should still open a pane item");
        assert!(
            item.downcast::<InvalidItemView>().is_some(),
            "broken DOCX should show the invalid item view"
        );
    }
}
