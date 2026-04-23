use std::any::TypeId;
use std::cmp::min;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
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
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{ContextMenu, WithScrollbar, prelude::*, right_click_menu};
use util::markdown::split_local_url_fragment;
use util::normalize_path;
use workspace::item::{Item, ItemBufferKind, ItemHandle};
use workspace::searchable::{
    Direction, SearchEvent, SearchOptions, SearchToken, SearchableItem, SearchableItemHandle,
};
use workspace::{OpenOptions, OpenVisible, Pane, Workspace};

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

struct EditorState {
    editor: Entity<Editor>,
    _subscription: Subscription,
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
        pane.items_of_type::<MarkdownPreviewView>()
            .find(|view| {
                let view_read = view.read(cx);
                // Only look for independent (Default mode) previews, not Follow previews
                view_read.mode == MarkdownPreviewMode::Default
                    && view_read
                        .active_editor
                        .as_ref()
                        .is_some_and(|active_editor| active_editor.editor == *editor)
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
        cx: &mut Context<Workspace>,
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

            if mode == MarkdownPreviewMode::Follow {
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
                    EditorEvent::SelectionsChanged { .. } => {
                        let (selection_start, editor_is_focused) =
                            editor.update(cx, |editor, cx| {
                                let index = Self::selected_source_index(editor, cx);
                                let focused = editor.focus_handle(cx).is_focused(window);
                                (index, focused)
                            });
                        this.sync_preview_to_source_index(selection_start, editor_is_focused, cx);
                        cx.notify();
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

            let editor_clone = editor.clone();
            let update = view.update(cx, |view, cx| {
                let is_active_editor = view
                    .active_editor
                    .as_ref()
                    .is_some_and(|active_editor| active_editor.editor == editor_clone);
                if !is_active_editor {
                    return None;
                }

                let (contents, selection_start) = editor_clone.update(cx, |editor, cx| {
                    let contents = editor.buffer().read(cx).snapshot(cx).text();
                    let selection_start = Self::selected_source_index(editor, cx);
                    (contents, selection_start)
                });
                Some((SharedString::from(contents), selection_start))
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

    fn selected_source_index(editor: &Editor, cx: &mut App) -> usize {
        editor
            .selections
            .last::<MultiBufferOffset>(&editor.display_snapshot(cx))
            .range()
            .start
            .0
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

    fn render_markdown_element(
        &self,
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

        let mut markdown_element = MarkdownElement::new(
            self.markdown.clone(),
            MarkdownStyle::themed(MarkdownFont::Editor, window, cx),
        )
        .code_block_renderer(CodeBlockRenderer::Default {
            copy_button_visibility: CopyButtonVisibility::VisibleOnHover,
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
                    let task_marker = if new_checked { "[x]" } else { "[ ]" };
                    editor_for_checkbox.update(cx, |editor, cx| {
                        editor.edit(
                            [(
                                MultiBufferOffset(source_range.start)
                                    ..MultiBufferOffset(source_range.end),
                                task_marker,
                            )],
                            cx,
                        );
                    });
                    if let Some(view) = view_handle.upgrade() {
                        cx.update_entity(&view, |this, cx| {
                            this.update_markdown_from_active_editor(false, false, window, cx);
                        });
                    }
                });
        }

        markdown_element
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
    if let Some(path) = resolve_preview_path(url.as_ref(), base_directory.as_deref())
        && let Some(workspace) = workspace.upgrade()
    {
        let _ = workspace.update(cx, |workspace, cx| {
            workspace
                .open_abs_path(
                    normalize_path(path.as_path()),
                    OpenOptions {
                        visible: Some(OpenVisible::None),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
                .detach();
        });
        return;
    }

    cx.open_url(url.as_ref());
}

fn resolve_preview_path(url: &str, base_directory: Option<&Path>) -> Option<PathBuf> {
    if url.starts_with("http://") || url.starts_with("https://") {
        return None;
    }

    let decoded_url = urlencoding::decode(url)
        .map(|decoded| decoded.into_owned())
        .unwrap_or_else(|_| url.to_string());
    let candidate = PathBuf::from(&decoded_url);

    if candidate.is_absolute() && candidate.exists() {
        return Some(candidate);
    }

    let base_directory = base_directory?;
    let resolved = base_directory.join(decoded_url);
    if resolved.exists() {
        Some(resolved)
    } else {
        None
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

    let decoded_path = Path::new(&decoded);

    if let Ok(relative_path) = decoded_path.strip_prefix("/") {
        if let Some(root) = workspace_directory {
            let absolute_path = root.join(relative_path);
            if absolute_path.exists() {
                return Some(ImageSource::Resource(Resource::Path(Arc::from(
                    absolute_path.as_path(),
                ))));
            }
        }
    }

    let path = if Path::new(&decoded).is_absolute() {
        PathBuf::from(decoded)
    } else {
        base_directory?.join(decoded)
    };

    Some(ImageSource::Resource(Resource::Path(Arc::from(
        path.as_path(),
    ))))
}

impl Focusable for MarkdownPreviewView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for MarkdownPreviewView {}
impl EventEmitter<SearchEvent> for MarkdownPreviewView {}

impl Item for MarkdownPreviewView {
    type Event = ();

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

    fn to_item_events(_event: &Self::Event, _f: &mut dyn FnMut(workspace::item::ItemEvent)) {}

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
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                div()
                    .id("markdown-preview-scroll-container")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .p_4()
                    .child({
                        let markdown_element = self.render_markdown_element(window, cx);
                        let markdown = self.markdown.clone();
                        right_click_menu("markdown-preview-context-menu")
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
                            })
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
        _ignore_settings: bool,
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

#[cfg(test)]
mod tests {
    use crate::markdown_preview_view::ImageSource;
    use crate::markdown_preview_view::Resource;
    use crate::markdown_preview_view::resolve_preview_image;
    use anyhow::Result;
    use std::fs;
    use tempfile::TempDir;

    use super::resolve_preview_path;

    #[test]
    fn resolves_relative_preview_paths() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let base_directory = temp_dir.path();
        let file = base_directory.join("notes.md");
        fs::write(&file, "# Notes")?;

        assert_eq!(
            resolve_preview_path("notes.md", Some(base_directory)),
            Some(file)
        );
        assert_eq!(
            resolve_preview_path("nonexistent.md", Some(base_directory)),
            None
        );
        assert_eq!(resolve_preview_path("notes.md", None), None);

        Ok(())
    }

    #[test]
    fn resolves_urlencoded_preview_paths() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let base_directory = temp_dir.path();
        let file = base_directory.join("release notes.md");
        fs::write(&file, "# Release Notes")?;

        assert_eq!(
            resolve_preview_path("release%20notes.md", Some(base_directory)),
            Some(file)
        );

        Ok(())
    }

    #[test]
    fn resolves_workspace_absolute_preview_images() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let workspace_directory = temp_dir.path();

        let base_directory = workspace_directory.join("docs");
        fs::create_dir_all(&base_directory)?;

        let image_file = workspace_directory.join("test_image.png");
        fs::write(&image_file, "mock data")?;

        let resolved_success = resolve_preview_image(
            "/test_image.png",
            Some(&base_directory),
            Some(workspace_directory),
        );

        match resolved_success {
            Some(ImageSource::Resource(Resource::Path(p))) => {
                assert_eq!(p.as_ref(), image_file.as_path());
            }
            _ => panic!("Expected successful resolution to be a Resource::Path"),
        }

        let resolved_missing = resolve_preview_image(
            "/missing_image.png",
            Some(&base_directory),
            Some(workspace_directory),
        );

        let expected_missing_path = if std::path::Path::new("/missing_image.png").is_absolute() {
            std::path::PathBuf::from("/missing_image.png")
        } else {
            // join is to retain windows path prefix C:/
            #[expect(clippy::join_absolute_paths)]
            base_directory.join("/missing_image.png")
        };

        match resolved_missing {
            Some(ImageSource::Resource(Resource::Path(p))) => {
                assert_eq!(p.as_ref(), expected_missing_path.as_path());
            }
            _ => panic!("Expected missing file to fallback to a Resource::Path"),
        }

        Ok(())
    }

    #[test]
    fn does_not_treat_web_links_as_preview_paths() {
        assert_eq!(resolve_preview_path("https://zed.dev", None), None);
        assert_eq!(resolve_preview_path("http://example.com", None), None);
    }
}
