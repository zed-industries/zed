use std::any::TypeId;
use std::borrow::Cow;
use std::cmp::min;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use editor::scroll::Autoscroll;
use editor::{Editor, EditorEvent, MultiBufferOffset, SelectionEffects};
use gpui::{
    App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable, ImageSource,
    InteractiveElement, IntoElement, IsZero, Pixels, Render, Resource, RetainAllImageCache,
    ScrollHandle, SharedString, SharedUri, Subscription, Task, WeakEntity, Window, point,
};
use language::LanguageRegistry;
use markdown::{
    CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownFont,
    MarkdownOptions, MarkdownStyle,
};
use project::search::SearchQuery;
use project::{Project, ProjectPath};
use settings::{SeedQuerySetting, Settings};
use theme::{SystemAppearance, Theme, ThemeRegistry};
use theme_settings::ThemeSettings;
use ui::{ContextMenu, WithScrollbar, prelude::*, right_click_menu};
use util::markdown::split_local_url_fragment;
use workspace::item::{Item, ItemBufferKind, ItemHandle, SaveOptions, SerializableItem};
use workspace::searchable::{
    Direction, SearchEvent, SearchOptions, SearchToken, SearchableItem, SearchableItemHandle,
};
use workspace::{ItemId, Pane, Workspace, WorkspaceId, delete_unloaded_items};

use crate::markdown_preview_settings::MarkdownPreviewSettings;
use crate::{
    OpenFollowingPreview, OpenPreview, OpenPreviewToTheSide, ScrollDown, ScrollDownByItem,
};
use crate::{ScrollPageDown, ScrollPageUp, ScrollToBottom, ScrollToTop, ScrollUp, ScrollUpByItem};

const REPARSE_DEBOUNCE: Duration = Duration::from_millis(200);

pub struct MarkdownPreviewView {
    workspace: WeakEntity<Workspace>,
    active_editor: Option<EditorState>,
    focus_handle: FocusHandle,
    markdown: Entity<Markdown>,
    _markdown_subscription: Subscription,
    active_source_index: Option<usize>,
    scroll_handle: ScrollHandle,
    image_cache: Entity<RetainAllImageCache>,
    base_directory: Option<PathBuf>,
    pending_update_task: Option<Task<Result<()>>>,
    mode: MarkdownPreviewMode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MarkdownPreviewMode {
    /// The preview will always show the contents of the provided editor.
    Default,
    /// The preview will "follow" the currently active editor.
    Follow,
}

impl MarkdownPreviewMode {
    fn to_db(self) -> i64 {
        match self {
            Self::Default => 0,
            Self::Follow => 1,
        }
    }

    fn from_db(value: i64) -> Self {
        match value {
            1 => Self::Follow,
            _ => Self::Default,
        }
    }
}

struct EditorState {
    editor: Entity<Editor>,
    _subscription: Subscription,
}

#[derive(Clone, Copy, Debug)]
pub enum MarkdownPreviewEvent {
    SourceEditorChanged,
    SourceFileHandleChanged,
}

impl MarkdownPreviewView {
    pub fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(move |workspace, _: &OpenPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                let view = Self::create_markdown_view(workspace, editor.clone(), window, cx);
                workspace.active_pane().update(cx, |pane, cx| {
                    if let Some(existing_view_idx) =
                        Self::find_existing_independent_preview_item_idx(pane, &editor, cx)
                    {
                        pane.activate_item(existing_view_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view.clone()), true, true, None, window, cx)
                    }
                });
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenPreviewToTheSide, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                let view = Self::create_markdown_view(workspace, editor.clone(), window, cx);
                let pane = workspace
                    .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                    .unwrap_or_else(|| {
                        workspace.split_pane(
                            workspace.active_pane().clone(),
                            workspace::SplitDirection::Right,
                            window,
                            cx,
                        )
                    });
                pane.update(cx, |pane, cx| {
                    if let Some(existing_view_idx) =
                        Self::find_existing_independent_preview_item_idx(pane, &editor, cx)
                    {
                        pane.activate_item(existing_view_idx, true, true, window, cx);
                    } else {
                        pane.add_item(Box::new(view.clone()), false, false, None, window, cx)
                    }
                });
                editor.focus_handle(cx).focus(window, cx);
                cx.notify();
            }
        });

        workspace.register_action(move |workspace, _: &OpenFollowingPreview, window, cx| {
            if let Some(editor) = Self::resolve_active_item_as_markdown_editor(workspace, cx) {
                // Check if there's already a following preview
                let existing_follow_view_idx = {
                    let active_pane = workspace.active_pane().read(cx);
                    active_pane
                        .items_of_type::<MarkdownPreviewView>()
                        .find(|view| view.read(cx).mode == MarkdownPreviewMode::Follow)
                        .and_then(|view| active_pane.index_for_item(&view))
                };

                if let Some(existing_follow_view_idx) = existing_follow_view_idx {
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.activate_item(existing_follow_view_idx, true, true, window, cx);
                    });
                } else {
                    let view = Self::create_following_markdown_view(workspace, editor, window, cx);
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.add_item(Box::new(view.clone()), true, true, None, window, cx)
                    });
                }
                cx.notify();
            }
        });
    }

    fn find_existing_independent_preview_item_idx(
        pane: &Pane,
        editor: &Entity<Editor>,
        cx: &App,
    ) -> Option<usize> {
        let target_buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
        pane.items_of_type::<MarkdownPreviewView>()
            .find(|view| {
                let view_read = view.read(cx);
                // Only look for independent (Default mode) previews, not Follow previews.
                // Match by buffer entity rather than editor entity so the lookup survives
                // workspace restoration, where the preview's bound editor may differ from
                // the editor the user is currently invoking the action on even though both
                // wrap the same source buffer.
                view_read.mode == MarkdownPreviewMode::Default
                    && view_read
                        .active_editor
                        .as_ref()
                        .is_some_and(|active_editor| {
                            active_editor
                                .editor
                                .read(cx)
                                .buffer()
                                .read(cx)
                                .as_singleton()
                                .as_ref()
                                == Some(&target_buffer)
                        })
            })
            .and_then(|view| pane.index_for_item(&view))
    }

    pub fn resolve_active_item_as_markdown_editor(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<Editor>> {
        if let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            && Self::is_markdown_file(&editor, cx)
        {
            return Some(editor);
        }
        None
    }

    fn create_markdown_view(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<MarkdownPreviewView> {
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        MarkdownPreviewView::new(
            MarkdownPreviewMode::Default,
            editor,
            workspace_handle,
            language_registry,
            window,
            cx,
        )
    }

    fn create_following_markdown_view(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<MarkdownPreviewView> {
        let language_registry = workspace.project().read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        MarkdownPreviewView::new(
            MarkdownPreviewMode::Follow,
            editor,
            workspace_handle,
            language_registry,
            window,
            cx,
        )
    }

    pub fn new(
        mode: MarkdownPreviewMode,
        active_editor: Entity<Editor>,
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let markdown = cx.new(|cx| {
                Markdown::new_with_options(
                    SharedString::default(),
                    Some(language_registry),
                    None,
                    MarkdownOptions {
                        parse_html: true,
                        render_mermaid_diagrams: true,
                        parse_heading_slugs: true,
                        render_metadata_blocks: true,
                        ..Default::default()
                    },
                    cx,
                )
            });
            let mut this = Self {
                active_editor: None,
                focus_handle: cx.focus_handle(),
                workspace: workspace.clone(),
                _markdown_subscription: cx.observe(
                    &markdown,
                    |this: &mut Self, _: Entity<Markdown>, cx| {
                        this.sync_active_root_block(cx);
                    },
                ),
                markdown,
                active_source_index: None,
                scroll_handle: ScrollHandle::new(),
                image_cache: RetainAllImageCache::new(cx),
                base_directory: None,
                pending_update_task: None,
                mode,
            };

            this.set_editor(active_editor, window, cx);

            match mode {
                MarkdownPreviewMode::Follow => {
                    if let Some(workspace) = &workspace.upgrade() {
                        cx.observe_in(workspace, window, |this, workspace, window, cx| {
                            let item = workspace.read(cx).active_item(cx);
                            this.workspace_updated(item, window, cx);
                        })
                        .detach();
                    } else {
                        log::error!("Failed to listen to workspace updates");
                    }
                }
                MarkdownPreviewMode::Default => {
                    // After workspace restoration the bound editor may be an orphan that
                    // wraps the right buffer but isn't the canonical Editor instance in
                    // any pane. Re-binding to the workspace's editor for our buffer is
                    // what restores cursor-driven scroll sync — `SelectionsChanged` only
                    // fires from the editor the user actually interacts with.
                    //
                    // Subscribing to `workspace::Event` (rather than `observe`) keeps the
                    // rebind check off the cursor-move hot path; `observe` would fire on
                    // every workspace `cx.notify`.
                    if let Some(workspace) = &workspace.upgrade() {
                        cx.subscribe_in(workspace, window, Self::on_workspace_event)
                            .detach();
                    }
                }
            }

            this
        })
    }

    fn workspace_updated(
        &mut self,
        active_item: Option<Box<dyn ItemHandle>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(item) = active_item
            && item.item_id() != cx.entity_id()
            && let Some(editor) = item.act_as::<Editor>(cx)
            && Self::is_markdown_file(&editor, cx)
        {
            self.set_editor(editor, window, cx);
        }
    }

    pub fn is_markdown_file<V>(editor: &Entity<Editor>, cx: &mut Context<V>) -> bool {
        let buffer = editor.read(cx).buffer().read(cx);
        if let Some(buffer) = buffer.as_singleton()
            && let Some(language) = buffer.read(cx).language()
        {
            return language.name() == "Markdown";
        }
        false
    }

    fn set_editor(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active) = &self.active_editor
            && active.editor == editor
        {
            return;
        }

        let had_active_editor = self.active_editor.is_some();
        let subscription = cx.subscribe_in(
            &editor,
            window,
            |this, editor, event: &EditorEvent, window, cx| {
                match event {
                    EditorEvent::Edited { .. }
                    | EditorEvent::BufferEdited { .. }
                    | EditorEvent::DirtyChanged
                    | EditorEvent::BuffersEdited { .. } => {
                        this.update_markdown_from_active_editor(true, false, window, cx);
                    }
                    EditorEvent::FileHandleChanged => {
                        this.base_directory =
                            Self::get_folder_for_active_editor(editor.read(cx), cx);
                        this.update_markdown_from_active_editor(false, false, window, cx);
                        cx.emit(MarkdownPreviewEvent::SourceFileHandleChanged);
                    }
                    EditorEvent::SelectionsChanged { .. } => {
                        let (selection_start, editor_is_focused) =
                            editor.update(cx, |editor, cx| {
                                let index = Self::selected_source_index(editor, cx);
                                let focused = editor.focus_handle(cx).is_focused(window);
                                (index, focused)
                            });
                        if let Some(selection_start) = selection_start {
                            this.sync_preview_to_source_index(
                                selection_start,
                                editor_is_focused,
                                cx,
                            );
                            cx.notify();
                        }
                    }
                    _ => {}
                };
            },
        );

        self.base_directory = Self::get_folder_for_active_editor(editor.read(cx), cx);
        self.active_editor = Some(EditorState {
            editor,
            _subscription: subscription,
        });
        self.update_markdown_from_active_editor(false, true, window, cx);
        if had_active_editor {
            cx.emit(MarkdownPreviewEvent::SourceEditorChanged);
        }
    }

    fn on_workspace_event(
        &mut self,
        workspace: &Entity<Workspace>,
        event: &workspace::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !matches!(
            event,
            workspace::Event::ItemAdded { .. } | workspace::Event::ItemRemoved { .. }
        ) {
            return;
        }
        let candidate = self.find_canonical_editor(workspace.read(cx), cx);
        if let Some(editor) = candidate
            && self
                .active_editor
                .as_ref()
                .is_none_or(|s| s.editor != editor)
        {
            self.set_editor(editor, window, cx);
        }
    }

    fn find_canonical_editor(&self, workspace: &Workspace, cx: &App) -> Option<Entity<Editor>> {
        let current = self.active_editor.as_ref()?.editor.clone();
        let our_buffer = current.read(cx).buffer().read(cx).as_singleton()?;
        let mut fallback = None;
        for editor in workspace.items_of_type::<Editor>(cx) {
            if editor.read(cx).buffer().read(cx).as_singleton().as_ref() != Some(&our_buffer) {
                continue;
            }
            if editor == current {
                return Some(current);
            }
            fallback.get_or_insert(editor);
        }
        fallback
    }

    fn update_markdown_from_active_editor(
        &mut self,
        wait_for_debounce: bool,
        should_reveal: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(state) = &self.active_editor {
            // if there is already a task to update the ui and the current task is also debounced (not high priority), do nothing
            if wait_for_debounce && self.pending_update_task.is_some() {
                return;
            }
            self.pending_update_task = Some(self.schedule_markdown_update(
                wait_for_debounce,
                should_reveal,
                state.editor.clone(),
                window,
                cx,
            ));
        }
    }

    fn schedule_markdown_update(
        &mut self,
        wait_for_debounce: bool,
        should_reveal_selection: bool,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        cx.spawn_in(window, async move |view, cx| {
            if wait_for_debounce {
                // Wait for the user to stop typing
                cx.background_executor().timer(REPARSE_DEBOUNCE).await;
            }

            let update = view.update(cx, |view, cx| {
                let is_active_editor = view
                    .active_editor
                    .as_ref()
                    .is_some_and(|active_editor| active_editor.editor == editor);
                if !is_active_editor {
                    return None;
                }

                editor.update(cx, |editor, cx| {
                    let contents = editor
                        .buffer()
                        .read(cx)
                        .as_singleton()?
                        .read(cx)
                        .as_rope()
                        .to_string()
                        .into();
                    let selection_start = Self::selected_source_index(editor, cx)?;
                    Some((contents, selection_start))
                })
            })?;

            view.update(cx, move |view, cx| {
                if let Some((contents, selection_start)) = update {
                    view.markdown.update(cx, |markdown, cx| {
                        markdown.reset(contents, cx);
                    });
                    view.sync_preview_to_source_index(selection_start, should_reveal_selection, cx);
                    cx.emit(SearchEvent::MatchesInvalidated);
                }
                view.pending_update_task = None;
                cx.notify();
            })
        })
    }

    fn selected_source_index(editor: &Editor, cx: &mut App) -> Option<usize> {
        let display_snapshot = editor.display_snapshot(cx);
        let source_offset = editor
            .selections
            .last::<MultiBufferOffset>(&display_snapshot)
            .range()
            .start;
        let buffer = editor.buffer().read(cx).as_singleton()?;
        let buffer_id = buffer.read(cx).remote_id();
        let (buffer_snapshot, buffer_offset) = display_snapshot
            .buffer_snapshot()
            .point_to_buffer_offset(source_offset)?;

        if buffer_snapshot.remote_id() == buffer_id {
            Some(buffer_offset.0)
        } else {
            None
        }
    }

    fn sync_preview_to_source_index(
        &mut self,
        source_index: usize,
        reveal: bool,
        cx: &mut Context<Self>,
    ) {
        self.active_source_index = Some(source_index);
        self.sync_active_root_block(cx);
        self.markdown.update(cx, |markdown, cx| {
            if reveal {
                markdown.request_autoscroll_to_source_index(source_index, cx);
            }
        });
    }

    fn sync_active_root_block(&mut self, cx: &mut Context<Self>) {
        self.markdown.update(cx, |markdown, cx| {
            markdown.set_active_root_for_source_index(self.active_source_index, cx);
        });
    }

    fn move_cursor_to_source_index(
        editor: &Entity<Editor>,
        source_index: usize,
        window: &mut Window,
        cx: &mut App,
    ) {
        editor.update(cx, |editor, cx| {
            let selection = MultiBufferOffset(source_index)..MultiBufferOffset(source_index);
            editor.change_selections(
                SelectionEffects::scroll(Autoscroll::center()),
                window,
                cx,
                |selections| selections.select_ranges(vec![selection]),
            );
            window.focus(&editor.focus_handle(cx), cx);
        });
    }

    /// The absolute path of the file that is currently being previewed.
    fn get_folder_for_active_editor(editor: &Editor, cx: &App) -> Option<PathBuf> {
        if let Some(file) = editor.file_at(MultiBufferOffset(0), cx) {
            if let Some(file) = file.as_local() {
                file.abs_path(cx).parent().map(|p| p.to_path_buf())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn line_scroll_amount(&self, cx: &App) -> Pixels {
        let settings = ThemeSettings::get_global(cx);
        settings.buffer_font_size(cx) * settings.buffer_line_height.value()
    }

    fn scroll_by_amount(&self, distance: Pixels) {
        let offset = self.scroll_handle.offset();
        self.scroll_handle
            .set_offset(point(offset.x, offset.y - distance));
    }

    fn scroll_page_up(&mut self, _: &ScrollPageUp, _window: &mut Window, cx: &mut Context<Self>) {
        let viewport_height = self.scroll_handle.bounds().size.height;
        if viewport_height.is_zero() {
            return;
        }

        self.scroll_by_amount(-viewport_height);
        cx.notify();
    }

    fn scroll_page_down(
        &mut self,
        _: &ScrollPageDown,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let viewport_height = self.scroll_handle.bounds().size.height;
        if viewport_height.is_zero() {
            return;
        }

        self.scroll_by_amount(viewport_height);
        cx.notify();
    }

    fn scroll_up(&mut self, _: &ScrollUp, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(bounds) = self
            .scroll_handle
            .bounds_for_item(self.scroll_handle.top_item())
        {
            let item_height = bounds.size.height;
            // Scroll no more than the rough equivalent of a large headline
            let max_height = window.rem_size() * 2;
            let scroll_height = min(item_height, max_height);
            self.scroll_by_amount(-scroll_height);
        } else {
            let scroll_height = self.line_scroll_amount(cx);
            if !scroll_height.is_zero() {
                self.scroll_by_amount(-scroll_height);
            }
        }
        cx.notify();
    }

    fn scroll_down(&mut self, _: &ScrollDown, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(bounds) = self
            .scroll_handle
            .bounds_for_item(self.scroll_handle.top_item())
        {
            let item_height = bounds.size.height;
            // Scroll no more than the rough equivalent of a large headline
            let max_height = window.rem_size() * 2;
            let scroll_height = min(item_height, max_height);
            self.scroll_by_amount(scroll_height);
        } else {
            let scroll_height = self.line_scroll_amount(cx);
            if !scroll_height.is_zero() {
                self.scroll_by_amount(scroll_height);
            }
        }
        cx.notify();
    }

    fn scroll_up_by_item(
        &mut self,
        _: &ScrollUpByItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self
            .scroll_handle
            .bounds_for_item(self.scroll_handle.top_item())
        {
            self.scroll_by_amount(-bounds.size.height);
        }
        cx.notify();
    }

    fn scroll_down_by_item(
        &mut self,
        _: &ScrollDownByItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self
            .scroll_handle
            .bounds_for_item(self.scroll_handle.top_item())
        {
            self.scroll_by_amount(bounds.size.height);
        }
        cx.notify();
    }

    fn scroll_to_top(&mut self, _: &ScrollToTop, _window: &mut Window, cx: &mut Context<Self>) {
        self.scroll_handle.scroll_to_item(0);
        cx.notify();
    }

    fn scroll_to_bottom(
        &mut self,
        _: &ScrollToBottom,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.scroll_handle.scroll_to_bottom();
        cx.notify();
    }

    /// Returns the theme chosen in `markdown_preview_theme`, or `None` if the
    /// user hasn't set one or it can't be resolved.
    fn resolve_preview_theme(&self, cx: &App) -> Option<Arc<Theme>> {
        let theme_settings = ThemeSettings::get_global(cx);
        let theme_selection = theme_settings.markdown_preview_theme.as_ref()?;
        let theme_name = theme_selection.name(SystemAppearance::global(cx).0);
        ThemeRegistry::global(cx).get(&theme_name.0).ok()
    }

    fn render_markdown_element(
        &self,
        preview_theme: &Option<Arc<Theme>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> MarkdownElement {
        let active_editor = self
            .active_editor
            .as_ref()
            .map(|state| state.editor.clone());

        let mut workspace_directory = None;
        if let Some(workspace_entity) = self.workspace.upgrade() {
            let project = workspace_entity.read(cx).project();
            if let Some(tree) = project.read(cx).worktrees(cx).next() {
                workspace_directory = Some(tree.read(cx).abs_path().to_path_buf());
            }
        }

        let markdown_style = if let Some(theme) = preview_theme {
            MarkdownStyle::themed_with_overrides(
                MarkdownFont::Preview,
                theme.colors(),
                theme.syntax(),
                window,
                cx,
            )
        } else {
            MarkdownStyle::themed(MarkdownFont::Preview, window, cx)
        };

        let mut markdown_element = MarkdownElement::new(self.markdown.clone(), markdown_style)
            .code_block_renderer(CodeBlockRenderer::Default {
                copy_button_visibility: CopyButtonVisibility::VisibleOnHover,
                wrap_button_visibility: markdown::WrapButtonVisibility::Hidden,
                border: false,
            })
            .scroll_handle(self.scroll_handle.clone())
            .show_root_block_markers()
            .image_resolver({
                let base_directory = self.base_directory.clone();
                move |dest_url| {
                    resolve_preview_image(
                        dest_url,
                        base_directory.as_deref(),
                        workspace_directory.as_deref(),
                    )
                }
            })
            .on_url_click({
                let view_handle = cx.entity().downgrade();
                let workspace = self.workspace.clone();
                let base_directory = self.base_directory.clone();
                move |url, window, cx| {
                    handle_url_click(
                        url,
                        &view_handle,
                        base_directory.clone(),
                        &workspace,
                        window,
                        cx,
                    );
                }
            });

        if let Some(active_editor) = active_editor {
            let editor_for_checkbox = active_editor.clone();
            let view_handle = cx.entity().downgrade();
            markdown_element = markdown_element
                .on_source_click(move |source_index, click_count, window, cx| {
                    if click_count == 2 {
                        Self::move_cursor_to_source_index(&active_editor, source_index, window, cx);
                        true
                    } else {
                        false
                    }
                })
                .on_checkbox_toggle(move |source_range, new_checked, window, cx| {
                    Self::apply_checkbox_toggle_to_editor(
                        &editor_for_checkbox,
                        source_range,
                        new_checked,
                        cx,
                    );
                    Self::refresh_preview(view_handle.clone(), window, cx);
                });
        }

        markdown_element
    }

    fn apply_checkbox_toggle_to_editor(
        editor: &Entity<Editor>,
        source_range: std::ops::Range<usize>,
        new_checked: bool,
        cx: &mut App,
    ) {
        let task_marker = if new_checked { "[x]" } else { "[ ]" };
        let expected_existing_marker = if new_checked { "[ ]" } else { "[x]" };

        editor.update(cx, |editor, cx| {
            let existing_marker: String = editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .text_for_range(
                    MultiBufferOffset(source_range.start)..MultiBufferOffset(source_range.end),
                )
                .collect();

            debug_assert_eq!(existing_marker, expected_existing_marker);

            editor.edit(
                [(
                    MultiBufferOffset(source_range.start)..MultiBufferOffset(source_range.end),
                    task_marker,
                )],
                cx,
            );
        });
    }

    fn refresh_preview(view_handle: WeakEntity<Self>, window: &mut Window, cx: &mut App) {
        if let Some(view) = view_handle.upgrade() {
            let preview_is_focused = view.read(cx).focus_handle.contains_focused(window, cx);
            if !preview_is_focused {
                return;
            }

            cx.update_entity(&view, |this, cx| {
                this.update_markdown_from_active_editor(false, false, window, cx);
            });
        }
    }
}

fn handle_url_click(
    url: SharedString,
    view: &WeakEntity<MarkdownPreviewView>,
    base_directory: Option<PathBuf>,
    workspace: &WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let (path_part, fragment) = split_local_url_fragment(url.as_ref());

    if path_part.is_empty() {
        if let Some(fragment) = fragment {
            let view = view.clone();
            let slug = SharedString::from(fragment.to_string());
            window.defer(cx, move |window, cx| {
                if let Some(view) = view.upgrade() {
                    let markdown = view.read(cx).markdown.clone();
                    let active_editor = view
                        .read(cx)
                        .active_editor
                        .as_ref()
                        .map(|state| state.editor.clone());

                    let source_index =
                        markdown.update(cx, |markdown, cx| markdown.scroll_to_heading(&slug, cx));

                    if let Some(source_index) = source_index {
                        if let Some(editor) = active_editor {
                            MarkdownPreviewView::move_cursor_to_source_index(
                                &editor,
                                source_index,
                                window,
                                cx,
                            );
                        }
                    }
                }
            });
        }
    } else {
        open_preview_url(
            SharedString::from(path_part.to_string()),
            base_directory,
            workspace,
            window,
            cx,
        );
    }
}

fn open_preview_url(
    url: SharedString,
    base_directory: Option<PathBuf>,
    workspace: &WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let (path_text, _) = split_preview_url(url.as_ref());

    // URL-decode the path for proper handling of encoded characters
    let decoded_path = urlencoding::decode(path_text).unwrap_or_else(|_| Cow::Borrowed(path_text));

    if let Some(workspace) = workspace.upgrade() {
        workspace.update(cx, |workspace, cx| {
            workspace.open_url_or_file(&decoded_path, base_directory.as_deref(), window, cx);
        });
    } else {
        cx.open_url(url.as_ref());
    }
}

fn split_preview_url(url: &str) -> (&str, Option<&str>) {
    match url.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (url, None),
    }
}

fn resolve_preview_image(
    dest_url: &str,
    base_directory: Option<&Path>,
    workspace_directory: Option<&Path>,
) -> Option<ImageSource> {
    if dest_url.starts_with("data:") {
        return None;
    }

    if dest_url.starts_with("http://") || dest_url.starts_with("https://") {
        return Some(ImageSource::Resource(Resource::Uri(SharedUri::from(
            dest_url.to_string(),
        ))));
    }

    let decoded = urlencoding::decode(dest_url)
        .map(|decoded| decoded.into_owned())
        .unwrap_or_else(|_| dest_url.to_string());

    if let Some(stripped) = ['/', '\\']
        .iter()
        .find_map(|prefix| decoded.strip_prefix(*prefix))
    {
        if let Some(root) = workspace_directory {
            let absolute_path = root.join(stripped);
            if absolute_path.exists() {
                return Some(ImageSource::Resource(Resource::Path(Arc::from(
                    absolute_path.as_path(),
                ))));
            } else {
                return None;
            }
        }
    }

    let path = if Path::new(&decoded).is_absolute() {
        PathBuf::from(decoded)
    } else {
        base_directory?.join(decoded)
    };

    path.exists()
        .then(|| ImageSource::Resource(Resource::Path(Arc::from(path.as_path()))))
}

impl Focusable for MarkdownPreviewView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<MarkdownPreviewEvent> for MarkdownPreviewView {}
impl EventEmitter<SearchEvent> for MarkdownPreviewView {}

impl Item for MarkdownPreviewView {
    type Event = MarkdownPreviewEvent;

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            self.active_editor
                .as_ref()
                .map(|state| state.editor.clone().into())
        } else {
            None
        }
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileDoc))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.active_editor
            .as_ref()
            .map(|editor_state| {
                let buffer = editor_state.editor.read(cx).buffer().read(cx);
                let title = buffer.title(cx);
                format!("Preview {}", title).into()
            })
            .unwrap_or_else(|| SharedString::from("Markdown Preview"))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Markdown Preview Opened")
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.mode != MarkdownPreviewMode::Default {
            return;
        }
        if let Some(editor) = self.find_canonical_editor(workspace, cx)
            && self
                .active_editor
                .as_ref()
                .is_none_or(|s| s.editor != editor)
        {
            self.set_editor(editor, window, cx);
        }
    }

    fn can_save(&self, cx: &App) -> bool {
        self.active_editor
            .as_ref()
            .is_some_and(|editor_state| editor_state.editor.read(cx).can_save(cx))
    }

    fn can_save_as(&self, cx: &App) -> bool {
        self.active_editor
            .as_ref()
            .is_some_and(|editor_state| editor_state.editor.read(cx).can_save_as(cx))
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.active_editor
            .as_ref()
            .map(|editor_state| {
                editor_state
                    .editor
                    .update(cx, |editor, cx| editor.save(options, project, window, cx))
            })
            .unwrap_or_else(|| Task::ready(Ok(())))
    }

    fn save_as(
        &mut self,
        project: Entity<Project>,
        path: project::ProjectPath,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.active_editor
            .as_ref()
            .map(|editor_state| {
                editor_state
                    .editor
                    .update(cx, |editor, cx| editor.save_as(project, path, window, cx))
            })
            .unwrap_or_else(|| Task::ready(Ok(())))
    }

    fn reload(
        &mut self,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        // The preview is not the owner of the source editor's buffer, so force-closing it should not discard editor changes.
        Task::ready(Ok(()))
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        match event {
            MarkdownPreviewEvent::SourceEditorChanged
            | MarkdownPreviewEvent::SourceFileHandleChanged => {
                f(workspace::item::ItemEvent::UpdateTab);
                f(workspace::item::ItemEvent::UpdateBreadcrumbs);
            }
        }
    }

    fn buffer_kind(&self, _cx: &App) -> ItemBufferKind {
        ItemBufferKind::Singleton
    }

    fn as_searchable(
        &self,
        handle: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }
}

impl Render for MarkdownPreviewView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let preview_theme = self.resolve_preview_theme(cx);
        let bg_color = preview_theme
            .as_ref()
            .map(|theme| theme.colors().editor_background)
            .unwrap_or_else(|| cx.theme().colors().editor_background);
        div()
            .image_cache(self.image_cache.clone())
            .id("MarkdownPreview")
            .key_context("MarkdownPreview")
            .track_focus(&self.focus_handle(cx))
            .on_action(cx.listener(MarkdownPreviewView::scroll_page_up))
            .on_action(cx.listener(MarkdownPreviewView::scroll_page_down))
            .on_action(cx.listener(MarkdownPreviewView::scroll_up))
            .on_action(cx.listener(MarkdownPreviewView::scroll_down))
            .on_action(cx.listener(MarkdownPreviewView::scroll_up_by_item))
            .on_action(cx.listener(MarkdownPreviewView::scroll_down_by_item))
            .on_action(cx.listener(MarkdownPreviewView::scroll_to_top))
            .on_action(cx.listener(MarkdownPreviewView::scroll_to_bottom))
            .w_full()
            .flex_1()
            .min_h_0()
            .bg(bg_color)
            .child(
                div()
                    .id("markdown-preview-scroll-container")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .p_4()
                    .child({
                        let markdown_element =
                            self.render_markdown_element(&preview_theme, window, cx);
                        let markdown = self.markdown.clone();
                        let max_width = MarkdownPreviewSettings::get_global(cx).max_width;
                        let content = right_click_menu("markdown-preview-context-menu")
                            .trigger(move |_, _, _| markdown_element)
                            .menu(move |window, cx| {
                                let focus = window.focused(cx);
                                let context_menu_link =
                                    markdown.read(cx).context_menu_link().cloned();
                                ContextMenu::build(window, cx, move |menu, _, _cx| {
                                    menu.when_some(focus, |menu, focus| menu.context(focus))
                                        .when_some(context_menu_link, |menu, url| {
                                            menu.entry("Copy Link", None, move |_, cx| {
                                                cx.write_to_clipboard(ClipboardItem::new_string(
                                                    url.to_string(),
                                                ));
                                            })
                                        })
                                })
                            });
                        div()
                            .w_full()
                            .when_some(max_width, |this, max_width| this.max_w(max_width).mx_auto())
                            .child(content)
                    }),
            )
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
    }
}

impl SearchableItem for MarkdownPreviewView {
    type Match = Range<usize>;

    fn supported_options(&self) -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
            replacement: false,
            selection: false,
            select_all: false,
            find_in_results: false,
        }
    }

    fn get_matches(&self, _window: &mut Window, cx: &mut App) -> (Vec<Self::Match>, SearchToken) {
        (
            self.markdown.read(cx).search_highlights().to_vec(),
            SearchToken::default(),
        )
    }

    fn clear_matches(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let had_highlights = !self.markdown.read(cx).search_highlights().is_empty();
        self.markdown.update(cx, |markdown, cx| {
            markdown.clear_search_highlights(cx);
        });
        if had_highlights {
            cx.emit(SearchEvent::MatchesInvalidated);
        }
    }

    fn update_matches(
        &mut self,
        matches: &[Self::Match],
        active_match_index: Option<usize>,
        _token: SearchToken,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        debug_assert!(
            matches
                .windows(2)
                .all(|ranges| (ranges[0].start, ranges[0].end) <= (ranges[1].start, ranges[1].end))
        );
        let old_highlights = self.markdown.read(cx).search_highlights();
        let changed = old_highlights != matches;
        self.markdown.update(cx, |markdown, cx| {
            markdown.set_search_highlights(matches.to_vec(), active_match_index, cx);
        });
        if changed {
            cx.emit(SearchEvent::MatchesInvalidated);
        }
    }

    fn query_suggestion(
        &mut self,
        _seed_query_override: Option<SeedQuerySetting>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> String {
        self.markdown.read(cx).selected_text().unwrap_or_default()
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        _token: SearchToken,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(match_range) = matches.get(index) {
            let start = match_range.start;
            self.markdown.update(cx, |markdown, cx| {
                markdown.set_active_search_highlight(Some(index), cx);
                markdown.request_autoscroll_to_source_index(start, cx);
            });
        }
    }

    fn select_matches(
        &mut self,
        _matches: &[Self::Match],
        _token: SearchToken,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn replace(
        &mut self,
        _: &Self::Match,
        _: &SearchQuery,
        _token: SearchToken,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    fn find_matches(
        &mut self,
        query: Arc<SearchQuery>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Self::Match>> {
        let source = self.markdown.read(cx).source().to_string();
        cx.background_spawn(async move { query.search_str(&source) })
    }

    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Self::Match],
        _token: SearchToken,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        if matches.is_empty() {
            return None;
        }

        let markdown = self.markdown.read(cx);
        let current_source_index = markdown
            .active_search_highlight()
            .and_then(|i| markdown.search_highlights().get(i))
            .map(|m| m.start)
            .or(self.active_source_index)
            .unwrap_or(0);

        match direction {
            Direction::Next => matches
                .iter()
                .position(|m| m.start >= current_source_index)
                .or(Some(0)),
            Direction::Prev => matches
                .iter()
                .rposition(|m| m.start <= current_source_index)
                .or(Some(matches.len().saturating_sub(1))),
        }
    }
}

impl SerializableItem for MarkdownPreviewView {
    fn serialized_item_kind() -> &'static str {
        "MarkdownPreviewView"
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let db = persistence::MarkdownPreviewDb::global(cx);
        window.spawn(cx, async move |cx| {
            let (abs_path, mode_value) = db
                .get_preview(item_id, workspace_id)?
                .context("No markdown preview entry found")?;
            let mode = MarkdownPreviewMode::from_db(mode_value);

            let (worktree, relative_path) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(abs_path.clone(), false, cx)
                })
                .await
                .context("Path not found")?;
            let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());

            let project_path = ProjectPath {
                worktree_id,
                path: relative_path,
            };

            let buffer = project
                .update(cx, |project, cx| project.open_buffer(project_path, cx))
                .await?;

            cx.update(|window, cx| {
                let language_registry = project.read(cx).languages().clone();
                let editor =
                    cx.new(|cx| Editor::for_buffer(buffer, Some(project.clone()), window, cx));
                MarkdownPreviewView::new(mode, editor, workspace, language_registry, window, cx)
            })
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let db = persistence::MarkdownPreviewDb::global(cx);
        delete_unloaded_items(alive_items, workspace_id, "markdown_previews", &db, cx)
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let workspace_id = workspace.database_id()?;
        let editor = self.active_editor.as_ref()?.editor.clone();
        let buffer = editor.read(cx).buffer().read(cx).as_singleton()?;
        let file = buffer.read(cx).file()?;
        let worktree_id = file.worktree_id(cx);
        let abs_path = workspace
            .project()
            .read(cx)
            .worktree_for_id(worktree_id, cx)?
            .read(cx)
            .absolutize(file.path());
        let mode = self.mode.to_db();
        let db = persistence::MarkdownPreviewDb::global(cx);
        Some(cx.background_spawn(async move {
            db.save_preview(item_id, workspace_id, abs_path, mode).await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        matches!(
            event,
            MarkdownPreviewEvent::SourceEditorChanged
                | MarkdownPreviewEvent::SourceFileHandleChanged
        )
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

    pub struct MarkdownPreviewDb(ThreadSafeConnection);

    impl Domain for MarkdownPreviewDb {
        const NAME: &str = stringify!(MarkdownPreviewDb);

        const MIGRATIONS: &[&str] = &[sql!(
            CREATE TABLE markdown_previews (
                workspace_id INTEGER,
                item_id INTEGER,
                abs_path BLOB,
                mode INTEGER NOT NULL DEFAULT 0,

                PRIMARY KEY(workspace_id, item_id),
                FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                ON DELETE CASCADE
            ) STRICT;
        )];
    }

    db::static_connection!(MarkdownPreviewDb, [WorkspaceDb]);

    impl MarkdownPreviewDb {
        query! {
            pub async fn save_preview(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                abs_path: PathBuf,
                mode: i64
            ) -> Result<()> {
                INSERT OR REPLACE INTO markdown_previews(item_id, workspace_id, abs_path, mode)
                VALUES (?, ?, ?, ?)
            }
        }

        query! {
            pub fn get_preview(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<(PathBuf, i64)>> {
                SELECT abs_path, mode
                FROM markdown_previews
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown_preview_view::ImageSource;
    use crate::markdown_preview_view::Resource;
    use crate::markdown_preview_view::resolve_preview_image;
    use buffer_diff::BufferDiff;
    use editor::Editor;
    use gpui::{AppContext as _, Entity, TestAppContext};
    use serde_json::json;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::Duration;
    use util::path;
    use util::rel_path::{RelPath, rel_path};
    use util::test::TempTree;
    use workspace::item::SerializableItem;
    use workspace::{
        AppState, ItemId, MultiWorkspace, SaveIntent, Workspace, WorkspaceId, open_paths,
    };

    use super::MarkdownPreviewView;

    #[test]
    fn resolves_workspace_absolute_preview_image_path_and_rejects_missing() {
        let tree = TempTree::new(json!({
            "docs": {},
            "test_image.png": "mock data"
        }));
        let workspace_directory = tree.path();
        let base_directory = markdown_fixture_directory(&tree);
        let image_file = workspace_directory.join("test_image.png");

        for workspace_root_relative_path in ["/test_image.png", "\\test_image.png"] {
            let resolved = resolve_preview_image(
                workspace_root_relative_path,
                Some(&base_directory),
                Some(workspace_directory),
            );
            assert_resolved_preview_image_path(resolved, image_file.as_path());
        }

        let missing = resolve_preview_image(
            "/missing_image.png",
            Some(&base_directory),
            Some(workspace_directory),
        );
        assert!(missing.is_none());
    }

    #[gpui::test]
    async fn toggles_task_checkbox_and_saves_when_preview_is_active(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "todo.md": "- [ ] Finish work\n"
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir/todo.md"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let preview = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                let editor: Entity<Editor> = workspace
                    .read(cx)
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                    .unwrap();

                workspace.update(cx, |workspace, cx| {
                    let preview = MarkdownPreviewView::create_markdown_view(
                        workspace,
                        editor.clone(),
                        window,
                        cx,
                    );
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.add_item(Box::new(preview.clone()), true, true, None, window, cx)
                    });
                    preview
                })
            })
            .unwrap();
        cx.run_until_parked();

        let save_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace: Entity<Workspace> = multi_workspace.workspace().clone();
                let view_handle = preview.downgrade();
                assert!(preview.read(cx).focus_handle.contains_focused(window, cx));
                preview.update(cx, |preview, cx| {
                    let editor = preview.active_editor.as_ref().unwrap().editor.clone();
                    MarkdownPreviewView::apply_checkbox_toggle_to_editor(&editor, 2..5, true, cx);
                });
                MarkdownPreviewView::refresh_preview(view_handle, window, cx);

                workspace.update(cx, |workspace: &mut Workspace, cx| {
                    workspace.save_active_item(SaveIntent::Save, window, cx)
                })
            })
            .unwrap();

        save_task.await.unwrap();
        cx.run_until_parked();

        assert_eq!(
            app_state
                .fs
                .load(path!("/dir/todo.md").as_ref())
                .await
                .unwrap(),
            "- [x] Finish work\n"
        );
    }

    #[gpui::test]
    async fn preview_uses_buffer_contents_instead_of_diff_contents(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "note.md": "new\n"
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir/note.md"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let preview = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let editor: Entity<Editor> = workspace
                        .active_item(cx)
                        .and_then(|item| item.act_as::<Editor>(cx))
                        .unwrap();
                    let buffer = editor.read(cx).buffer().read(cx).as_singleton().unwrap();
                    let diff = cx.new(|cx| {
                        BufferDiff::new_with_base_text(
                            "old\n",
                            &buffer.read(cx).text_snapshot(),
                            cx,
                        )
                    });
                    let multibuffer = editor.read(cx).buffer().clone();
                    multibuffer.update(cx, |multibuffer, cx| {
                        multibuffer.add_diff(diff, cx);
                        multibuffer.set_all_diff_hunks_expanded(cx);
                    });

                    let diff_text = multibuffer.read(cx).snapshot(cx).text();
                    assert!(diff_text.contains("old"));
                    assert!(diff_text.contains("new"));

                    let preview =
                        MarkdownPreviewView::create_markdown_view(workspace, editor, window, cx);
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.add_item(Box::new(preview.clone()), true, true, None, window, cx)
                    });
                    preview
                })
            })
            .unwrap();
        cx.run_until_parked();

        assert_eq!(
            preview.read_with(cx, |preview, cx| preview
                .markdown
                .read(cx)
                .source()
                .to_string()),
            "new\n"
        );
    }

    #[gpui::test]
    async fn force_closing_preview_preserves_source_editor_changes(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "todo.md": "- [ ] Finish work\n"
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir/todo.md"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let (preview, editor) = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                let editor: Entity<Editor> = workspace
                    .read(cx)
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
                    .unwrap();

                let preview = workspace.update(cx, |workspace, cx| {
                    let preview = MarkdownPreviewView::create_markdown_view(
                        workspace,
                        editor.clone(),
                        window,
                        cx,
                    );
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.add_item(Box::new(preview.clone()), true, true, None, window, cx)
                    });
                    preview
                });

                (preview, editor)
            })
            .unwrap();
        cx.run_until_parked();

        multi_workspace
            .update(cx, |_, window, cx| {
                let view_handle = preview.downgrade();
                assert!(preview.read(cx).focus_handle.contains_focused(window, cx));
                MarkdownPreviewView::apply_checkbox_toggle_to_editor(&editor, 2..5, true, cx);
                MarkdownPreviewView::refresh_preview(view_handle, window, cx);
            })
            .unwrap();

        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.buffer().read(cx).read(cx).text()),
            "- [x] Finish work\n"
        );

        let close_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.close_item_by_id(preview.entity_id(), SaveIntent::Skip, window, cx)
                    })
                })
            })
            .unwrap();

        close_task.await.unwrap();
        cx.run_until_parked();

        assert_eq!(
            editor.read_with(cx, |editor, cx| editor.buffer().read(cx).read(cx).text()),
            "- [x] Finish work\n"
        );
    }

    #[gpui::test]
    async fn preview_serialized_path_updates_when_source_file_is_renamed(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "todo.md": "![image](image.png)\n",
                    "subdir": {},
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let open_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let worktree_id = workspace
                        .project()
                        .read(cx)
                        .worktrees(cx)
                        .next()
                        .unwrap()
                        .read(cx)
                        .id();
                    workspace.open_path((worktree_id, rel_path("todo.md")), None, true, window, cx)
                })
            })
            .unwrap();
        open_task.await.unwrap();
        cx.run_until_parked();

        let (preview, project, workspace_id) = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    workspace.set_random_database_id();
                    let workspace_id = workspace.database_id().unwrap();
                    let project = workspace.project().clone();
                    let editor: Entity<Editor> = workspace
                        .active_item(cx)
                        .and_then(|item| item.act_as::<Editor>(cx))
                        .unwrap();
                    let preview =
                        MarkdownPreviewView::create_markdown_view(workspace, editor, window, cx);
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.add_item(Box::new(preview.clone()), true, true, None, window, cx)
                    });
                    (preview, project, workspace_id)
                })
            })
            .unwrap();
        let workspace_serialization_tasks = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.flush_all_serialization(window, cx)
            })
            .unwrap();
        for task in workspace_serialization_tasks {
            task.await;
        }

        let serialize_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    preview
                        .update(cx, |preview, cx| {
                            preview.serialize(workspace, cx.entity_id().as_u64(), false, window, cx)
                        })
                        .unwrap()
                })
            })
            .unwrap();
        serialize_task.await.unwrap();

        assert_eq!(
            saved_preview_path(cx, preview.entity_id().as_u64(), workspace_id),
            PathBuf::from(path!("/dir/todo.md"))
        );

        let (entry_id, worktree_id, destination_path) = preview.read_with(cx, |preview, cx| {
            let editor = &preview.active_editor.as_ref().unwrap().editor;
            let buffer = editor.read(cx).buffer().read(cx).as_singleton().unwrap();
            let buffer = buffer.read(cx);
            let file = buffer.file().unwrap();
            let worktree_id = file.worktree_id(cx);
            let source_path = file.path();
            let mut destination_path = source_path.to_rel_path_buf();
            destination_path.pop();
            destination_path.push(rel_path("subdir/renamed.md"));
            let worktree = project.read(cx).worktree_for_id(worktree_id, cx).unwrap();
            let entry_id = worktree.read(cx).entry_for_path(source_path).unwrap().id;
            (
                entry_id,
                worktree_id,
                destination_path.as_rel_path().into_arc(),
            )
        });
        project
            .update(cx, |project, cx| {
                project.rename_entry(entry_id, (worktree_id, destination_path).into(), cx)
            })
            .await
            .unwrap();
        wait_for_preview_serialization(cx).await;

        assert_eq!(
            preview.read_with(cx, |preview, _| preview.base_directory.clone()),
            Some(PathBuf::from(path!("/dir/subdir")))
        );
        assert_eq!(
            saved_preview_path(cx, preview.entity_id().as_u64(), workspace_id),
            PathBuf::from(path!("/dir/subdir/renamed.md"))
        );
    }

    #[gpui::test]
    async fn follow_preview_serialized_path_updates_when_followed_editor_changes(
        cx: &mut TestAppContext,
    ) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "a.md": "# A\n",
                    "b.md": "# B\n",
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let worktree_id = multi_workspace
            .update(cx, |multi_workspace, _, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .project()
                    .read(cx)
                    .worktrees(cx)
                    .next()
                    .unwrap()
                    .read(cx)
                    .id()
            })
            .unwrap();

        let open_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.open_path((worktree_id, rel_path("a.md")), None, true, window, cx)
                })
            })
            .unwrap();
        let opened_item = open_task.await.unwrap();
        cx.run_until_parked();
        let editor_a = cx.update(|cx| opened_item.act_as::<Editor>(cx).unwrap());

        let open_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.open_path((worktree_id, rel_path("b.md")), None, true, window, cx)
                })
            })
            .unwrap();
        let opened_item = open_task.await.unwrap();
        cx.run_until_parked();
        let editor_b = cx.update(|cx| opened_item.act_as::<Editor>(cx).unwrap());
        let editor_b_path = editor_source_path(cx, &editor_b);
        assert_eq!(editor_b_path.as_ref(), rel_path("b.md"));

        let (preview, workspace_id) = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.workspace().update(cx, |workspace, cx| {
                    workspace.set_random_database_id();
                    let workspace_id = workspace.database_id().unwrap();
                    let preview = MarkdownPreviewView::create_following_markdown_view(
                        workspace, editor_a, window, cx,
                    );
                    workspace.active_pane().update(cx, |pane, cx| {
                        pane.add_item(Box::new(preview.clone()), true, true, None, window, cx)
                    });
                    (preview, workspace_id)
                })
            })
            .unwrap();
        let workspace_serialization_tasks = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                multi_workspace.flush_all_serialization(window, cx)
            })
            .unwrap();
        for task in workspace_serialization_tasks {
            task.await;
        }
        wait_for_preview_serialization(cx).await;

        let serialize_task = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    preview
                        .update(cx, |preview, cx| {
                            preview.serialize(workspace, cx.entity_id().as_u64(), false, window, cx)
                        })
                        .unwrap()
                })
            })
            .unwrap();
        serialize_task.await.unwrap();

        assert_eq!(
            saved_preview_path(cx, preview.entity_id().as_u64(), workspace_id),
            PathBuf::from(path!("/dir/a.md"))
        );

        multi_workspace
            .update(cx, |_, window, cx| {
                preview.update(cx, |preview, cx| {
                    preview.set_editor(editor_b, window, cx);
                });
            })
            .unwrap();
        wait_for_preview_serialization(cx).await;

        let followed_path = preview_source_path(cx, &preview);
        assert_eq!(followed_path.as_ref(), rel_path("b.md"));

        assert_eq!(
            saved_preview_path(cx, preview.entity_id().as_u64(), workspace_id),
            PathBuf::from(path!("/dir/b.md")),
            "a Follow preview should persist the source editor it most recently followed"
        );
    }

    #[gpui::test]
    async fn default_preview_stays_bound_to_invoking_editor_across_splits(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "todo.md": "- [ ] Finish work\n"
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from(path!("/dir/todo.md"))],
                app_state.clone(),
                workspace::OpenOptions::default(),
                cx,
            )
        })
        .await
        .unwrap();

        let multi_workspace = cx.update(|cx| cx.windows()[0].downcast::<MultiWorkspace>().unwrap());
        let (preview, second_editor) = multi_workspace
            .update(cx, |multi_workspace, window, cx| {
                let workspace = multi_workspace.workspace().clone();
                workspace.update(cx, |workspace, cx| {
                    let first_editor: Entity<Editor> = workspace
                        .active_item(cx)
                        .and_then(|item| item.act_as::<Editor>(cx))
                        .unwrap();
                    let buffer = first_editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .unwrap();
                    let project = workspace.project().clone();

                    let second_editor =
                        cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
                    let new_pane = workspace.split_pane(
                        workspace.active_pane().clone(),
                        workspace::SplitDirection::Right,
                        window,
                        cx,
                    );
                    new_pane.update(cx, |pane, cx| {
                        pane.add_item(
                            Box::new(second_editor.clone()),
                            true,
                            true,
                            None,
                            window,
                            cx,
                        )
                    });

                    let preview = MarkdownPreviewView::create_markdown_view(
                        workspace,
                        second_editor.clone(),
                        window,
                        cx,
                    );
                    new_pane.update(cx, |pane, cx| {
                        pane.add_item(Box::new(preview.clone()), true, true, None, window, cx)
                    });
                    (preview, second_editor)
                })
            })
            .unwrap();
        cx.run_until_parked();

        let bound_editor = preview.read_with(cx, |preview, _| {
            preview.active_editor.as_ref().unwrap().editor.clone()
        });
        assert_eq!(
            bound_editor, second_editor,
            "a Default preview must stay bound to the editor it was opened from, not another \
             editor that happens to share the same buffer in a different split"
        );
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            editor::init(cx);
            crate::init(cx);
            state
        })
    }

    async fn wait_for_preview_serialization(cx: &mut TestAppContext) {
        cx.run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(250));
        cx.run_until_parked();
    }

    fn saved_preview_path(
        cx: &mut TestAppContext,
        item_id: ItemId,
        workspace_id: WorkspaceId,
    ) -> PathBuf {
        cx.update(|cx| {
            super::persistence::MarkdownPreviewDb::global(cx)
                .get_preview(item_id, workspace_id)
                .unwrap()
                .unwrap()
                .0
        })
    }

    fn preview_source_path(
        cx: &mut TestAppContext,
        preview: &Entity<MarkdownPreviewView>,
    ) -> Arc<RelPath> {
        let editor = preview.read_with(cx, |preview, _| {
            preview.active_editor.as_ref().unwrap().editor.clone()
        });
        editor_source_path(cx, &editor)
    }

    fn editor_source_path(cx: &mut TestAppContext, editor: &Entity<Editor>) -> Arc<RelPath> {
        editor.read_with(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            buffer.read(cx).file().unwrap().path().clone()
        })
    }

    fn markdown_fixture_directory(tree: &TempTree) -> PathBuf {
        tree.path().join("docs")
    }

    #[track_caller]
    fn assert_resolved_preview_image_path(
        resolved: Option<ImageSource>,
        expected_path: &std::path::Path,
    ) {
        match resolved {
            Some(ImageSource::Resource(Resource::Path(path))) => {
                assert_eq!(path.as_ref(), expected_path);
            }
            _ => panic!("Expected preview image to resolve to a local path"),
        }
    }
}
