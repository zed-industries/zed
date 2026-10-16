use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use anyhow::{Result, anyhow};
use collections::{HashMap, HashSet};
use editor::{
    Anchor, Editor, EditorElement, EditorEvent, EditorMode, EditorStyle, HighlightKey, Inlay,
    MultiBufferOffset, MultiBufferRow, SizingBehavior,
    display_map::{BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId},
};
use extension::{
    ExtensionCommandContext, ExtensionCommandDispatcher as _, ExtensionConsoleInputDispatcher as _,
    ExtensionConsoleOutputKind, ExtensionConsoleProxy, ExtensionDecoration,
    ExtensionDecorationStyle, ExtensionEditorProxy, ExtensionEditorState,
    ExtensionEventsDispatcher as _, ExtensionHostProxy, ExtensionNotificationLevel,
    ExtensionQuickPickItem, ExtensionTextEdit, ExtensionUiProxy,
    ExtensionUiResponseDispatcher as _,
};
use extension_host::ExtensionStore;
use zed_actions::{RunExtensionCommand, extensions::RunCommand};
use gpui::{
    App, AppContext as _, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Global, HighlightStyle, InteractiveElement as _, IntoElement, ParentElement as _, Render,
    SharedString, Styled as _, Subscription, Task, TaskExt as _, TextStyle, WeakEntity, Window,
    div,
};
use language::Point;
use menu::Confirm;
use picker::{Picker, PickerDelegate};
use project::InlayId;
use settings::Settings as _;
use theme::ActiveTheme as _;
use theme_settings::ThemeSettings;
use ui::{
    Color, FluentBuilder as _, Icon, IconName, Label, LabelCommon as _, LabelSize, ListItem,
    ListItemSpacing, StyledExt as _, Toggleable as _, h_flex, v_flex,
};
use util::ResultExt as _;
use workspace::{
    DismissDecision, HideStatusItem, ModalView, OpenOptions, SplitDirection, StatusItemView, Toast,
    Workspace, WorkspaceStore,
    item::{Item, ItemEvent, ItemHandle},
    notifications::NotificationId,
};

static NEXT_HIGHLIGHT_ID: AtomicUsize = AtomicUsize::new(1);

struct ExtensionGutter;

fn gutter_info_color(cx: &App) -> gpui::Hsla {
    cx.theme().status().info
}
fn gutter_success_color(cx: &App) -> gpui::Hsla {
    cx.theme().status().success
}
fn gutter_error_color(cx: &App) -> gpui::Hsla {
    cx.theme().status().error
}
fn gutter_warning_color(cx: &App) -> gpui::Hsla {
    cx.theme().status().warning
}
fn gutter_muted_color(cx: &App) -> gpui::Hsla {
    cx.theme().colors().text_muted
}

#[derive(Default)]
struct EditorDecorationRecords {
    inlays: Vec<InlayId>,
    blocks: HashSet<CustomBlockId>,
    highlights: Vec<HighlightKey>,
    gutters: Vec<std::ops::Range<Anchor>>,
}

pub struct ZedExtensionEditorProxy {
    workspace_store: Entity<WorkspaceStore>,
    decorations: parking_lot::Mutex<HashMap<String, Vec<(String, WeakEntity<Editor>, EditorDecorationRecords)>>>,
}

fn active_or_first_workspace(
    workspace_store: &Entity<WorkspaceStore>,
    cx: &App,
) -> Option<(gpui::AnyWindowHandle, Entity<Workspace>)> {
    let active_window = cx.active_window();
    let store = workspace_store.read(cx);
    if let Some(active_window) = active_window {
        if let Some(workspace) = store
            .workspaces_with_windows()
            .find(|(window, _)| *window == active_window)
            .and_then(|(window, weak)| weak.upgrade().map(|w| (window, w)))
        {
            return Some(workspace);
        }
    }
    store
        .workspaces_with_windows()
        .find_map(|(window, weak)| weak.upgrade().map(|w| (window, w)))
}

impl ZedExtensionEditorProxy {
    pub fn new(workspace_store: Entity<WorkspaceStore>) -> Self {
        Self {
            workspace_store,
            decorations: parking_lot::Mutex::new(HashMap::default()),
        }
    }

    fn active_or_first_workspace(
        &self,
        cx: &App,
    ) -> Option<(gpui::AnyWindowHandle, Entity<Workspace>)> {
        active_or_first_workspace(&self.workspace_store, cx)
    }

    fn find_editors(&self, path: &str, cx: &App) -> Vec<Entity<Editor>> {
        if path.is_empty() {
            if let Some((_, workspace)) = self.active_or_first_workspace(cx) {
                if let Some(editor) = workspace.read(cx).active_item_as::<Editor>(cx) {
                    return vec![editor];
                }
            }
            return Vec::new();
        }

        let target_path = Path::new(path);
        let mut matching_editors = Vec::new();
        let workspaces: Vec<_> = self
            .workspace_store
            .read(cx)
            .workspaces_with_windows()
            .filter_map(|(_, weak)| weak.upgrade())
            .collect();

        for workspace in workspaces {
            for editor in workspace.read(cx).items_of_type::<Editor>(cx) {
                if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                    let matches = buffer.read(cx).file().is_some_and(|file| {
                        file.file_system_abs_path(cx).as_deref() == Some(target_path)
                            || file.full_path(cx) == target_path
                    });
                    if matches {
                        matching_editors.push(editor);
                    }
                }
            }
        }
        matching_editors
    }
}

pub fn active_editor_state(editor: &Entity<Editor>, cx: &mut App) -> Option<ExtensionEditorState> {
    let multibuffer = editor.read(cx).buffer().clone();
    let buffer = multibuffer.read(cx).as_singleton()?;

    let path = buffer
        .read(cx)
        .file()
        .and_then(|file| file.file_system_abs_path(cx).or_else(|| Some(file.full_path(cx))))
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();

    let language = buffer
        .read(cx)
        .language()
        .map(|lang| lang.name().to_string())
        .unwrap_or_default();

    let text = buffer.read(cx).as_rope().to_string();

    let display_snapshot = editor.update(cx, |editor, cx| editor.display_snapshot(cx));
    let selections = editor
        .read(cx)
        .selections
        .all::<MultiBufferOffset>(&display_snapshot)
        .into_iter()
        .map(|selection| (selection.start.0 as u32)..(selection.end.0 as u32))
        .collect();

    let version = buffer
        .read(cx)
        .version()
        .iter()
        .map(|lamport| lamport.value as u64)
        .sum::<u64>();

    Some(ExtensionEditorState {
        path,
        language,
        text,
        selections,
        version,
    })
}

impl ExtensionEditorProxy for ZedExtensionEditorProxy {
    fn active_editor(&self, cx: &mut App) -> Result<Option<ExtensionEditorState>> {
        let Some((_, workspace)) = self.active_or_first_workspace(cx) else {
            return Ok(None);
        };
        let Some(editor) = workspace.read(cx).active_item_as::<Editor>(cx) else {
            return Ok(None);
        };
        Ok(active_editor_state(&editor, cx))
    }

    fn apply_edits(&self, path: &str, edits: Vec<ExtensionTextEdit>, cx: &mut App) -> Result<()> {
        let target_path = Path::new(path);

        if path.is_empty() {
            if let Some((_, workspace)) = self.active_or_first_workspace(cx) {
                if let Some(editor) = workspace.read(cx).active_item_as::<Editor>(cx) {
                    if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                        buffer.update(cx, |buffer, cx| {
                            buffer.edit(
                                edits.into_iter().map(|edit| {
                                    (edit.start as usize..edit.end as usize, edit.text)
                                }),
                                None,
                                cx,
                            );
                        });
                        return Ok(());
                    }
                }
            }
        }

        let workspaces: Vec<_> = self
            .workspace_store
            .read(cx)
            .workspaces_with_windows()
            .filter_map(|(handle, weak)| weak.upgrade().map(|w| (handle, w)))
            .collect();

        for (_, workspace) in &workspaces {
            let editors: Vec<_> = workspace.read(cx).items_of_type::<Editor>(cx).collect();
            for editor in editors {
                if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                    let matches = buffer.read(cx).file().is_some_and(|file| {
                        file.file_system_abs_path(cx).as_deref() == Some(target_path)
                            || file.full_path(cx) == target_path
                    });
                    if matches {
                        buffer.update(cx, |buffer, cx| {
                            buffer.edit(
                                edits.into_iter().map(|edit| {
                                    (edit.start as usize..edit.end as usize, edit.text)
                                }),
                                None,
                                cx,
                            );
                        });
                        return Ok(());
                    }
                }
            }

            let buffers: Vec<_> = workspace
                .read(cx)
                .project()
                .read(cx)
                .buffer_store()
                .read(cx)
                .buffers()
                .collect();
            for buffer in buffers {
                let matches = buffer.read(cx).file().is_some_and(|file| {
                    file.file_system_abs_path(cx).as_deref() == Some(target_path)
                        || file.full_path(cx) == target_path
                });
                if matches {
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit(
                            edits.into_iter().map(|edit| {
                                (edit.start as usize..edit.end as usize, edit.text)
                            }),
                            None,
                            cx,
                        );
                    });
                    return Ok(());
                }
            }
        }

        let Some((window_handle, workspace)) = self.active_or_first_workspace(cx) else {
            return Err(anyhow!("no workspace available to open buffer: {path}"));
        };

        window_handle.update(cx, |_, window, cx| {
            let path_buf = target_path.to_path_buf();
            let open_task = workspace.update(cx, |workspace, cx| {
                workspace.open_abs_path(path_buf, OpenOptions::default(), window, cx)
            });

            window
                .spawn(cx, async move |cx| {
                    let item = open_task.await.log_err()?;
                    let editor = item.downcast::<Editor>()?;
                    editor
                        .update_in(cx, |editor, _window, cx| {
                            if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                                buffer.update(cx, |buffer, cx| {
                                    buffer.edit(
                                        edits.into_iter().map(|edit| {
                                            (edit.start as usize..edit.end as usize, edit.text)
                                        }),
                                        None,
                                        cx,
                                    );
                                });
                            }
                        })
                        .log_err();
                    Some(())
                })
                .detach();
        })?;

        Ok(())
    }

    fn set_selections(
        &self,
        path: &str,
        selections: Vec<std::ops::Range<u32>>,
        cx: &mut App,
    ) -> Result<()> {
        let target_path = Path::new(path);

        if let Some((window_handle, workspace)) = self.active_or_first_workspace(cx) {
            if let Some(editor) = workspace.read(cx).active_item_as::<Editor>(cx) {
                let is_match = path.is_empty()
                    || editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .is_some_and(|buffer| {
                            buffer.read(cx).file().is_some_and(|file| {
                                file.file_system_abs_path(cx).as_deref() == Some(target_path)
                                    || file.full_path(cx) == target_path
                            })
                        });

                if is_match {
                    window_handle.update(cx, |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            editor.change_selections(Default::default(), window, cx, |s| {
                                s.select_ranges(
                                    selections.into_iter().map(|range| {
                                        MultiBufferOffset(range.start as usize)
                                            ..MultiBufferOffset(range.end as usize)
                                    }),
                                );
                            });
                        });
                    })?;
                    return Ok(());
                }
            }
        }

        let workspaces: Vec<_> = self
            .workspace_store
            .read(cx)
            .workspaces_with_windows()
            .filter_map(|(handle, weak)| weak.upgrade().map(|w| (handle, w)))
            .collect();

        for (window_handle, workspace) in workspaces {
            let editors: Vec<_> = workspace.read(cx).items_of_type::<Editor>(cx).collect();
            for editor in editors {
                let matches = editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .is_some_and(|buffer| {
                        buffer.read(cx).file().is_some_and(|file| {
                            file.file_system_abs_path(cx).as_deref() == Some(target_path)
                                || file.full_path(cx) == target_path
                        })
                    });

                if matches {
                    window_handle.update(cx, |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            editor.change_selections(Default::default(), window, cx, |s| {
                                s.select_ranges(
                                    selections
                                        .into_iter()
                                        .map(|range| {
                                            MultiBufferOffset(range.start as usize)
                                                ..MultiBufferOffset(range.end as usize)
                                        }),
                                );
                            });
                        });
                    })?;
                    return Ok(());
                }
            }
        }

        Err(anyhow!("no editor found for path: {path}"))
    }

    fn set_decorations(
        &self,
        path: &str,
        key: &str,
        decorations: Vec<ExtensionDecoration>,
        cx: &mut App,
    ) -> Result<()> {
        self.clear_decorations(Some(path), key, cx)?;

        let editors = self.find_editors(path, cx);
        if editors.is_empty() {
            return Ok(());
        }

        static NEXT_INLAY_ID: AtomicUsize = AtomicUsize::new(100_000);

        for editor in editors {
            let mut records = EditorDecorationRecords::default();

            editor.update(cx, |editor, cx| {
                let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                let max_point = buffer_snapshot.max_point();

                for decoration in &decorations {
                    match decoration {
                        ExtensionDecoration::InlineAfter { range, text, .. } => {
                            let end_offset = (range.end as usize).min(buffer_snapshot.len().0);
                            let point = buffer_snapshot.offset_to_point(MultiBufferOffset(end_offset));
                            let line_len = buffer_snapshot.line_len(MultiBufferRow(point.row));
                            let line_end = Point::new(point.row, line_len);
                            let anchor = buffer_snapshot.anchor_after(line_end);
                            let inlay_id = NEXT_INLAY_ID.fetch_add(1, Ordering::SeqCst);
                            let inlay = Inlay::repl_result(inlay_id, anchor, format!("  {text}"));
                            editor.splice_inlays(&[], vec![inlay], cx);
                            records.inlays.push(InlayId::ReplResult(inlay_id));
                        }
                        ExtensionDecoration::Block { row, text, style } => {
                            let actual_row = (*row).min(max_point.row);
                            let line_len = buffer_snapshot.line_len(MultiBufferRow(actual_row));
                            let point = Point::new(actual_row, line_len);
                            let placement_anchor = buffer_snapshot.anchor_before(point);
                            let text = text.clone();
                            let style = *style;
                            let height = text.lines().count().max(1) as u32;
                            let block = BlockProperties {
                                placement: BlockPlacement::Below(placement_anchor),
                                height: Some(height),
                                style: BlockStyle::Sticky,
                                render: Arc::new(move |cx: &mut BlockContext| {
                                    let color = match style {
                                        ExtensionDecorationStyle::Info => cx.theme().status().info,
                                        ExtensionDecorationStyle::Success => cx.theme().status().success,
                                        ExtensionDecorationStyle::Error => cx.theme().status().error,
                                        ExtensionDecorationStyle::Warning => cx.theme().status().warning,
                                        ExtensionDecorationStyle::Muted => cx.theme().colors().text_muted,
                                    };
                                    div()
                                        .px_2()
                                        .py_1()
                                        .text_sm()
                                        .text_color(color)
                                        .child(text.clone())
                                        .into_any_element()
                                }),
                                priority: 0,
                            };
                            let block_ids = editor.insert_blocks([block], None, cx);
                            records.blocks.extend(block_ids);
                        }
                        ExtensionDecoration::Highlight { range, style } => {
                            let start_offset = (range.start as usize).min(buffer_snapshot.len().0);
                            let end_offset = (range.end as usize).min(buffer_snapshot.len().0);
                            let start = buffer_snapshot.anchor_after(MultiBufferOffset(start_offset));
                            let end = buffer_snapshot.anchor_after(MultiBufferOffset(end_offset));
                            let highlight_id = NEXT_HIGHLIGHT_ID.fetch_add(1, Ordering::SeqCst);
                            let highlight_key = HighlightKey::Extension(highlight_id);
                            let style = *style;
                            editor.highlight_background(
                                highlight_key,
                                &[start..end],
                                move |_, theme| {
                                    let color = match style {
                                        ExtensionDecorationStyle::Info => theme.status().info,
                                        ExtensionDecorationStyle::Success => theme.status().success,
                                        ExtensionDecorationStyle::Error => theme.status().error,
                                        ExtensionDecorationStyle::Warning => theme.status().warning,
                                        ExtensionDecorationStyle::Muted => theme.colors().text_muted,
                                    };
                                    color.opacity(0.2)
                                },
                                cx,
                            );
                            records.highlights.push(highlight_key);
                        }
                        ExtensionDecoration::Gutter { row, style } => {
                            let actual_row = (*row).min(max_point.row);
                            let line_len = buffer_snapshot.line_len(MultiBufferRow(actual_row));
                            let start = buffer_snapshot.anchor_before(Point::new(actual_row, 0));
                            let end = buffer_snapshot.anchor_after(Point::new(actual_row, line_len));
                            let anchor_range = start..end;
                            let color_fn = match style {
                                ExtensionDecorationStyle::Info => gutter_info_color,
                                ExtensionDecorationStyle::Success => gutter_success_color,
                                ExtensionDecorationStyle::Error => gutter_error_color,
                                ExtensionDecorationStyle::Warning => gutter_warning_color,
                                ExtensionDecorationStyle::Muted => gutter_muted_color,
                            };
                            editor.insert_gutter_highlight::<ExtensionGutter>(
                                anchor_range.clone(),
                                color_fn,
                                cx,
                            );
                            records.gutters.push(anchor_range);
                        }
                    }
                }
            });

            self.decorations
                .lock()
                .entry(key.to_string())
                .or_default()
                .push((path.to_string(), editor.downgrade(), records));
        }

        Ok(())
    }

    fn clear_decorations(&self, path: Option<&str>, key: &str, cx: &mut App) -> Result<()> {
        let to_clear: Vec<(WeakEntity<Editor>, EditorDecorationRecords)> = {
            let mut decorations = self.decorations.lock();
            if let Some(path) = path {
                if let Some(entries) = decorations.get_mut(key) {
                    let mut kept = Vec::new();
                    let mut removed = Vec::new();
                    for (entry_path, weak_editor, records) in entries.drain(..) {
                        if entry_path == path || Path::new(&entry_path) == Path::new(path) {
                            removed.push((weak_editor, records));
                        } else {
                            kept.push((entry_path, weak_editor, records));
                        }
                    }
                    if kept.is_empty() {
                        decorations.remove(key);
                    } else {
                        *entries = kept;
                    }
                    removed
                } else {
                    Vec::new()
                }
            } else {
                decorations
                    .remove(key)
                    .map(|entries| {
                        entries
                            .into_iter()
                            .map(|(_, weak_editor, records)| (weak_editor, records))
                            .collect()
                    })
                    .unwrap_or_default()
            }
        };

        for (weak_editor, mut records) in to_clear {
            if let Some(editor) = weak_editor.upgrade() {
                clear_records(&mut records, &editor, cx);
            }
        }

        Ok(())
    }
}

fn clear_records(records: &mut EditorDecorationRecords, editor: &Entity<Editor>, cx: &mut App) {
    editor.update(cx, |editor, cx| {
        if !records.inlays.is_empty() {
            editor.splice_inlays(&records.inlays, Vec::new(), cx);
            records.inlays.clear();
        }
        if !records.blocks.is_empty() {
            editor.remove_blocks(records.blocks.clone(), None, cx);
            records.blocks.clear();
        }
        for highlight in records.highlights.drain(..) {
            editor.clear_background_highlights(highlight, cx);
        }
        if !records.gutters.is_empty() {
            editor.remove_gutter_highlights::<ExtensionGutter>(
                std::mem::take(&mut records.gutters),
                cx,
            );
        }
    });
}

#[derive(Default)]
pub struct ExtensionStatusRegistry {
    statuses: HashMap<String, String>,
}

#[derive(Clone)]
pub struct GlobalExtensionStatusRegistry(pub Entity<ExtensionStatusRegistry>);

impl Global for GlobalExtensionStatusRegistry {}

pub struct ExtensionStatusItemView {
    registry: Entity<ExtensionStatusRegistry>,
}

impl ExtensionStatusItemView {
    pub fn new(registry: Entity<ExtensionStatusRegistry>, cx: &mut Context<Self>) -> Self {
        cx.observe(&registry, |_, _, cx| cx.notify()).detach();
        Self { registry }
    }
}

impl Render for ExtensionStatusItemView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let registry = self.registry.read(cx);
        if registry.statuses.is_empty() {
            return div();
        }
        let mut values: Vec<_> = registry.statuses.values().cloned().collect();
        values.sort();
        let text = values.join(" | ");
        div().child(Label::new(text).size(LabelSize::Small))
    }
}

impl StatusItemView for ExtensionStatusItemView {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _cx: &App) -> Option<HideStatusItem> {
        None
    }
}

struct ExtensionQuickPickDelegate {
    extension_id: Arc<str>,
    request_id: u64,
    items: Vec<ExtensionQuickPickItem>,
    matches: Vec<usize>,
    selected_index: usize,
    placeholder: Option<String>,
    responded: bool,
}

impl ExtensionQuickPickDelegate {
    fn new(
        extension_id: Arc<str>,
        request_id: u64,
        items: Vec<ExtensionQuickPickItem>,
        placeholder: Option<String>,
    ) -> Self {
        let matches = (0..items.len()).collect();
        Self {
            extension_id,
            request_id,
            items,
            matches,
            selected_index: 0,
            placeholder,
            responded: false,
        }
    }

    fn respond(&mut self, selected: Option<String>, cx: &mut App) {
        if !self.responded {
            self.responded = true;
            ExtensionHostProxy::global(cx)
                .ui_response(self.extension_id.clone(), self.request_id, selected, cx)
                .detach_and_log_err(cx);
        }
    }
}

impl PickerDelegate for ExtensionQuickPickDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "ExtensionQuickPick"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder
            .as_deref()
            .unwrap_or("Select an item…")
            .into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query_lower = query.trim().to_lowercase();
        self.matches = if query_lower.is_empty() {
            (0..self.items.len()).collect()
        } else {
            self.items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    item.label.to_lowercase().contains(&query_lower)
                        || item
                            .detail
                            .as_deref()
                            .map_or(false, |detail| detail.to_lowercase().contains(&query_lower))
                })
                .map(|(ix, _)| ix)
                .collect()
        };
        self.selected_index = self
            .selected_index
            .min(self.matches.len().saturating_sub(1));
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(&ix) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(item) = self.items.get(ix) else {
            return;
        };
        self.respond(Some(item.label.clone()), cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.respond(None, cx);
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let &item_ix = self.matches.get(ix)?;
        let item = self.items.get(item_ix)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Label::new(item.label.clone()).size(LabelSize::Small))
                        .when_some(item.detail.clone(), |this, detail| {
                            this.child(
                                Label::new(detail)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        }),
                ),
        )
    }
}

pub struct ExtensionInputBoxModal {
    extension_id: Arc<str>,
    request_id: u64,
    prompt: String,
    editor: Entity<Editor>,
    responded: bool,
}

impl EventEmitter<DismissEvent> for ExtensionInputBoxModal {}

impl ModalView for ExtensionInputBoxModal {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DismissDecision {
        if !self.responded {
            self.responded = true;
            ExtensionHostProxy::global(cx)
                .ui_response(self.extension_id.clone(), self.request_id, None, cx)
                .detach_and_log_err(cx);
        }
        DismissDecision::Dismiss(true)
    }
}

impl Focusable for ExtensionInputBoxModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl ExtensionInputBoxModal {
    pub fn new(
        extension_id: Arc<str>,
        request_id: u64,
        prompt: String,
        default_value: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if let Some(default_value) = default_value {
                editor.set_text(default_value, window, cx);
                editor.select_all(&editor::actions::SelectAll, window, cx);
            }
            editor
        });

        Self {
            extension_id,
            request_id,
            prompt,
            editor,
            responded: false,
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.responded {
            self.responded = true;
            ExtensionHostProxy::global(cx)
                .ui_response(self.extension_id.clone(), self.request_id, None, cx)
                .detach_and_log_err(cx);
        }
        cx.emit(DismissEvent);
    }

    fn confirm(&mut self, _: &menu::Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        let text = self.editor.read(cx).text(cx);
        self.responded = true;
        ExtensionHostProxy::global(cx)
            .ui_response(self.extension_id.clone(), self.request_id, Some(text), cx)
            .detach_and_log_err(cx);
        cx.emit(DismissEvent);
    }
}

impl Render for ExtensionInputBoxModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .key_context("ExtensionInputBoxModal")
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .elevation_3(cx)
            .w_96()
            .overflow_hidden()
            .child(
                div()
                    .p_2()
                    .border_b_1()
                    .border_color(theme.colors().border_variant)
                    .child(self.editor.clone()),
            )
            .when(!self.prompt.is_empty(), |this| {
                this.child(
                    h_flex()
                        .bg(theme.colors().editor_background)
                        .rounded_b_sm()
                        .w_full()
                        .p_2()
                        .gap_1()
                        .child(
                            Label::new(self.prompt.clone())
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                )
            })
    }
}

pub struct ZedExtensionUiProxy {
    workspace_store: Entity<WorkspaceStore>,
    status_registry: Entity<ExtensionStatusRegistry>,
}

impl ZedExtensionUiProxy {
    pub fn new(workspace_store: Entity<WorkspaceStore>, cx: &mut App) -> Self {
        let status_registry = if cx.has_global::<GlobalExtensionStatusRegistry>() {
            cx.global::<GlobalExtensionStatusRegistry>().0.clone()
        } else {
            let registry = cx.new(|_| ExtensionStatusRegistry::default());
            cx.set_global(GlobalExtensionStatusRegistry(registry.clone()));
            let registry_clone = registry.clone();
            cx.observe_new(
                move |workspace: &mut Workspace, window, cx: &mut Context<Workspace>| {
                    if let Some(window) = window {
                        let item =
                            cx.new(|cx| ExtensionStatusItemView::new(registry_clone.clone(), cx));
                        workspace.status_bar().update(cx, |status_bar, cx| {
                            status_bar.add_right_item(item, window, cx);
                        });
                    }
                },
            )
            .detach();
            registry
        };

        cx.observe_new(
            |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
                register_actions(workspace);
            },
        )
        .detach();

        Self {
            workspace_store,
            status_registry,
        }
    }
}

impl ExtensionUiProxy for ZedExtensionUiProxy {
    fn notify(&self, level: ExtensionNotificationLevel, message: &str, cx: &mut App) -> Result<()> {
        let Some((_, workspace)) = active_or_first_workspace(&self.workspace_store, cx) else {
            return Err(anyhow!("no active workspace for notification"));
        };

        static NOTIFICATION_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = NotificationId::named(
            format!(
                "extension-notification-{}",
                NOTIFICATION_COUNTER.fetch_add(1, Ordering::Relaxed)
            )
            .into(),
        );

        let formatted = match level {
            ExtensionNotificationLevel::Info => message.to_string(),
            ExtensionNotificationLevel::Warning => format!("Warning: {message}"),
            ExtensionNotificationLevel::Error => format!("Error: {message}"),
        };

        workspace.update(cx, |workspace, cx| {
            let mut toast = Toast::new(id, formatted);
            if level != ExtensionNotificationLevel::Error {
                toast = toast.autohide();
            }
            workspace.show_toast(toast, cx);
        });

        Ok(())
    }

    fn quick_pick(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        items: Vec<ExtensionQuickPickItem>,
        placeholder: Option<String>,
        cx: &mut App,
    ) -> Result<()> {
        let Some((window_handle, workspace)) =
            active_or_first_workspace(&self.workspace_store, cx)
        else {
            return Err(anyhow!("no active workspace for quick pick"));
        };

        window_handle.update(cx, |_, window, cx| {
            workspace.update(cx, |workspace, cx| {
                let delegate = ExtensionQuickPickDelegate::new(
                    extension_id,
                    request_id,
                    items,
                    placeholder,
                );
                workspace.toggle_modal(window, cx, move |window, cx| {
                    Picker::uniform_list(delegate, window, cx)
                });
            });
        })?;

        Ok(())
    }

    fn input_box(
        &self,
        extension_id: Arc<str>,
        request_id: u64,
        prompt: &str,
        default_value: Option<String>,
        cx: &mut App,
    ) -> Result<()> {
        let Some((window_handle, workspace)) =
            active_or_first_workspace(&self.workspace_store, cx)
        else {
            return Err(anyhow!("no active workspace for input box"));
        };

        let prompt = prompt.to_string();
        window_handle.update(cx, |_, window, cx| {
            workspace.update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, move |window, cx| {
                    ExtensionInputBoxModal::new(
                        extension_id,
                        request_id,
                        prompt,
                        default_value,
                        window,
                        cx,
                    )
                });
            });
        })?;

        Ok(())
    }

    fn set_status(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: Option<String>,
        cx: &mut App,
    ) -> Result<()> {
        let key = format!("{extension_id}:{id}");
        self.status_registry.update(cx, |registry, cx| {
            if let Some(text) = text {
                registry.statuses.insert(key, text);
            } else {
                registry.statuses.remove(&key);
            }
            cx.notify();
        });
        Ok(())
    }

    fn write_clipboard(&self, text: &str, cx: &mut App) -> Result<()> {
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.to_string()));
        Ok(())
    }

    fn open_path(
        &self,
        path: &str,
        row: Option<u32>,
        column: Option<u32>,
        cx: &mut App,
    ) -> Result<()> {
        let Some((window_handle, workspace)) =
            active_or_first_workspace(&self.workspace_store, cx)
        else {
            return Err(anyhow!("no active workspace to open path: {path}"));
        };

        if path.is_empty() {
            if let Some(editor) = workspace.read(cx).active_item_as::<Editor>(cx) {
                if row.is_some() || column.is_some() {
                    let point = Point::new(row.unwrap_or(0), column.unwrap_or(0));
                    window_handle.update(cx, |_, window, cx| {
                        editor.update(cx, |editor, cx| {
                            editor.go_to_singleton_buffer_point(point, window, cx);
                        });
                    })?;
                }
                return Ok(());
            }
        }

        let target_path = Path::new(path);
        let path_buf = if target_path.is_absolute() {
            target_path.to_path_buf()
        } else if let Some(worktree) = workspace
            .read(cx)
            .project()
            .read(cx)
            .visible_worktrees(cx)
            .next()
        {
            worktree.read(cx).abs_path().join(target_path)
        } else {
            target_path.to_path_buf()
        };

        window_handle.update(cx, |_, window, cx| {
            let open_task = workspace.update(cx, |workspace, cx| {
                workspace.open_abs_path(path_buf, OpenOptions::default(), window, cx)
            });

            window
                .spawn(cx, async move |cx| {
                    let item = open_task.await.log_err()?;
                    let editor = item.downcast::<Editor>()?;
                    if row.is_some() || column.is_some() {
                        let point = Point::new(row.unwrap_or(0), column.unwrap_or(0));
                        editor
                            .update_in(cx, |editor, window, cx| {
                                editor.go_to_singleton_buffer_point(point, window, cx);
                            })
                            .log_err();
                    }
                    Some(())
                })
                .detach();
        })?;

        Ok(())
    }
}

pub struct ZedExtensionConsoleProxy {
    workspace_store: Entity<WorkspaceStore>,
}

impl ZedExtensionConsoleProxy {
    pub fn new(workspace_store: Entity<WorkspaceStore>) -> Self {
        Self { workspace_store }
    }

    fn find_console(
        &self,
        extension_id: &str,
        id: &str,
        cx: &App,
    ) -> Option<(gpui::AnyWindowHandle, Entity<Workspace>, Entity<ExtensionConsole>)> {
        let store = self.workspace_store.read(cx);
        if let Some(active_window) = cx.active_window() {
            if let Some((window, weak)) = store
                .workspaces_with_windows()
                .find(|(window, _)| *window == active_window)
            {
                if let Some(workspace) = weak.upgrade() {
                    if let Some(console) = workspace
                        .read(cx)
                        .items_of_type::<ExtensionConsole>(cx)
                        .find(|console| {
                            let console = console.read(cx);
                            console.extension_id.as_ref() == extension_id && console.id == id
                        })
                    {
                        return Some((window, workspace, console));
                    }
                }
            }
        }

        store.workspaces_with_windows().find_map(|(window, weak)| {
            let workspace = weak.upgrade()?;
            let console = workspace
                .read(cx)
                .items_of_type::<ExtensionConsole>(cx)
                .find(|console| {
                    let console = console.read(cx);
                    console.extension_id.as_ref() == extension_id && console.id == id
                })?;
            Some((window, workspace, console))
        })
    }
}

impl ExtensionConsoleProxy for ZedExtensionConsoleProxy {
    fn open_console(
        &self,
        extension_id: Arc<str>,
        id: &str,
        title: &str,
        cx: &mut App,
    ) -> Result<()> {
        if let Some((window_handle, workspace, console)) = self.find_console(&extension_id, id, cx) {
            window_handle.update(cx, |_, window, cx| {
                workspace.update(cx, |workspace, cx| {
                    workspace.activate_item(&console, true, true, window, cx);
                });
                console.update(cx, |console, cx| {
                    if !title.is_empty() {
                        console.set_title(title, cx);
                    }
                    console.focus_input(window, cx);
                });
            })?;
            return Ok(());
        }

        let Some((window_handle, workspace)) = active_or_first_workspace(&self.workspace_store, cx) else {
            return Err(anyhow!("no active workspace to open console"));
        };

        window_handle.update(cx, |_, window, cx| {
            let title = if title.is_empty() {
                format!("Console ({id})")
            } else {
                title.to_string()
            };
            let console = cx.new(|cx| {
                ExtensionConsole::new(extension_id, id.to_string(), title, window, cx)
            });
            workspace.update(cx, |workspace, cx| {
                workspace.split_item(SplitDirection::Right, Box::new(console.clone()), window, cx);
            });
            console.update(cx, |console, cx| {
                console.focus_input(window, cx);
            });
        })?;

        Ok(())
    }

    fn append(
        &self,
        extension_id: Arc<str>,
        id: &str,
        text: &str,
        kind: ExtensionConsoleOutputKind,
        cx: &mut App,
    ) -> Result<()> {
        let Some((window_handle, _, console)) = self.find_console(&extension_id, id, cx) else {
            return Err(anyhow!("console not found for extension {extension_id} id {id}"));
        };
        window_handle.update(cx, |_, window, cx| {
            console.update(cx, |console, cx| {
                console.append(text, kind, Some(window), cx);
            });
        })?;
        Ok(())
    }

    fn clear(&self, extension_id: Arc<str>, id: &str, cx: &mut App) -> Result<()> {
        let Some((window_handle, _, console)) = self.find_console(&extension_id, id, cx) else {
            return Err(anyhow!("console not found for extension {extension_id} id {id}"));
        };
        window_handle.update(cx, |_, window, cx| {
            console.update(cx, |console, cx| {
                console.clear(Some(window), cx);
            });
        })?;
        Ok(())
    }

    fn set_prompt(&self, extension_id: Arc<str>, id: &str, text: &str, cx: &mut App) -> Result<()> {
        let Some((window_handle, _, console)) = self.find_console(&extension_id, id, cx) else {
            return Err(anyhow!("console not found for extension {extension_id} id {id}"));
        };
        window_handle.update(cx, |_, _window, cx| {
            console.update(cx, |console, cx| {
                console.set_prompt(text, cx);
            });
        })?;
        Ok(())
    }
}

pub struct ExtensionConsole {
    extension_id: Arc<str>,
    id: String,
    title: String,
    prompt: String,
    transcript: Entity<Editor>,
    input: Entity<Editor>,
    focus_handle: FocusHandle,
    highlight_key: HighlightKey,
    _subscriptions: Vec<Subscription>,
}

impl ExtensionConsole {
    pub fn new(
        extension_id: Arc<str>,
        id: String,
        title: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let transcript = cx.new(|cx| {
            let mut editor = Editor::multi_line(window, cx);
            editor.set_mode(EditorMode::Full {
                scale_ui_elements_with_buffer_font_size: true,
                show_active_line_background: false,
                sizing_behavior: SizingBehavior::ExcludeOverscrollMargin,
            });
            editor.set_read_only(true);
            editor.set_show_gutter(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_bookmarks(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_edit_predictions(Some(false), window, cx);
            editor.set_autoindent(false);
            editor.set_input_enabled(false);
            editor.set_use_autoclose(false);
            editor.set_use_modal_editing(false);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor
        });

        let input = cx.new(|cx| {
            let mut editor = Editor::multi_line(window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_bookmarks(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_edit_predictions(Some(false), window, cx);
            editor.set_autoindent(false);
            editor.set_use_autoclose(false);
            editor.set_use_modal_editing(false);
            editor
        });

        let focus_handle = cx.focus_handle();
        let mut subscriptions = vec![cx.on_focus(&focus_handle, window, |this, window, cx| {
            this.input.focus_handle(cx).focus(window, cx);
        })];

        let this = cx.entity().downgrade();
        let action_sub = input.update(cx, |editor, _cx| {
            let this = this.clone();
            editor.register_action(move |_: &editor::actions::Newline, window, cx| {
                if window.modifiers().shift {
                    cx.propagate();
                } else if let Some(this) = this.upgrade() {
                    this.update(cx, |console, cx| {
                        console.send(window, cx);
                    });
                }
            })
        });
        subscriptions.push(action_sub);

        let confirm_sub = input.update(cx, |editor, _cx| {
            let this = this.clone();
            editor.register_action(move |_: &Confirm, window, cx| {
                if window.modifiers().shift {
                    cx.propagate();
                } else if let Some(this) = this.upgrade() {
                    this.update(cx, |console, cx| {
                        console.send(window, cx);
                    });
                }
            })
        });
        subscriptions.push(confirm_sub);

        let highlight_key =
            HighlightKey::Extension(NEXT_HIGHLIGHT_ID.fetch_add(1, Ordering::SeqCst));

        Self {
            extension_id,
            id,
            title,
            prompt: "> ".to_string(),
            transcript,
            input,
            focus_handle,
            highlight_key,
            _subscriptions: subscriptions,
        }
    }

    pub fn set_title(&mut self, title: &str, cx: &mut Context<Self>) {
        self.title = title.to_string();
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    pub fn set_prompt(&mut self, prompt: &str, cx: &mut Context<Self>) {
        self.prompt = prompt.to_string();
        cx.notify();
    }

    pub fn focus_input(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.focus_handle(cx).focus(window, cx);
        });
    }

    pub fn clear(&mut self, window: Option<&mut Window>, cx: &mut Context<Self>) {
        self.transcript.update(cx, |editor, cx| {
            editor.set_read_only(false);
            if let Some(window) = window {
                editor.clear(window, cx);
            } else {
                let multibuffer = editor.buffer().clone();
                multibuffer.update(cx, |multibuffer, cx| {
                    let len = multibuffer.len(cx);
                    multibuffer.edit([(MultiBufferOffset(0)..len, "")], None, cx);
                });
            }
            editor.highlight_text(self.highlight_key, vec![], HighlightStyle::default(), cx);
            editor.set_read_only(true);
        });
    }

    pub fn append(
        &mut self,
        text: &str,
        kind: ExtensionConsoleOutputKind,
        window: Option<&mut Window>,
        cx: &mut Context<Self>,
    ) {
        if text.is_empty() {
            return;
        }
        self.transcript.update(cx, |editor, cx| {
            let prev_len = editor.buffer().read(cx).len(cx).0;
            editor.set_read_only(false);
            if let Some(window) = window {
                editor.move_to_end(&editor::actions::MoveToEnd, window, cx);
                editor.insert(text, window, cx);
            } else {
                let multibuffer = editor.buffer().clone();
                multibuffer.update(cx, |multibuffer, cx| {
                    let len = multibuffer.len(cx);
                    multibuffer.edit([(len..len, text)], None, cx);
                });
            }
            if matches!(kind, ExtensionConsoleOutputKind::Error) {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                let new_len = snapshot.len().0;
                let start = snapshot.anchor_after(MultiBufferOffset(prev_len));
                let end = snapshot.anchor_after(MultiBufferOffset(new_len));
                let error_style = HighlightStyle {
                    color: Some(cx.theme().status().error),
                    ..Default::default()
                };
                editor.highlight_text_key(self.highlight_key, vec![start..end], error_style, true, cx);
            }
            editor.set_read_only(true);
        });
    }

    fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.input.read(cx).text(cx);
        self.input.update(cx, |editor, cx| editor.clear(window, cx));
        let task = ExtensionHostProxy::global(cx).console_input(
            self.extension_id.clone(),
            &self.id,
            &text,
            cx,
        );
        task.detach_and_log_err(cx);
    }

    fn editor_style(_editor: &Entity<Editor>, cx: &App) -> EditorStyle {
        let settings = ThemeSettings::get_global(cx);
        let theme = cx.theme();
        let text = TextStyle {
            color: theme.colors().text,
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: settings.buffer_font_size(cx).into(),
            font_weight: settings.buffer_font.weight,
            line_height: gpui::relative(settings.buffer_line_height.value()),
            ..Default::default()
        };
        EditorStyle {
            background: theme.colors().editor_background,
            local_player: theme.players().local(),
            text,
            syntax: theme.syntax().clone(),
            ..Default::default()
        }
    }
}

impl EventEmitter<ItemEvent> for ExtensionConsole {}

impl Focusable for ExtensionConsole {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ExtensionConsole {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.title.clone().into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Terminal))
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Extension Console Opened")
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event);
    }
}

impl Render for ExtensionConsole {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .key_context("ExtensionConsole")
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                div().flex_1().px_2().pt_1().child(EditorElement::new(
                    &self.transcript,
                    Self::editor_style(&self.transcript, cx),
                )),
            )
            .child(
                h_flex()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Label::new(self.prompt.clone())
                            .color(Color::Accent)
                            .buffer_font(cx),
                    )
                    .child(
                        div().flex_1().child(EditorElement::new(
                            &self.input,
                            Self::editor_style(&self.input, cx),
                        )),
                    ),
            )
    }
}

#[derive(Clone)]
struct ExtensionCommandItem {
    extension_id: Arc<str>,
    command_id: Arc<str>,
    title: String,
    extension_name: String,
}

struct ExtensionCommandPickerDelegate {
    commands: Vec<ExtensionCommandItem>,
    matches: Vec<usize>,
    selected_index: usize,
}

impl ExtensionCommandPickerDelegate {
    fn new(commands: Vec<ExtensionCommandItem>) -> Self {
        let matches = (0..commands.len()).collect();
        Self {
            commands,
            matches,
            selected_index: 0,
        }
    }
}

impl PickerDelegate for ExtensionCommandPickerDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "ExtensionCommandPicker"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select an extension command…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query_lower = query.trim().to_lowercase();
        self.matches = if query_lower.is_empty() {
            (0..self.commands.len()).collect()
        } else {
            self.commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    cmd.title.to_lowercase().contains(&query_lower)
                        || cmd.extension_name.to_lowercase().contains(&query_lower)
                        || cmd.command_id.to_lowercase().contains(&query_lower)
                })
                .map(|(ix, _)| ix)
                .collect()
        };
        self.selected_index = self
            .selected_index
            .min(self.matches.len().saturating_sub(1));
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(&ix) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(item) = self.commands.get(ix) else {
            return;
        };
        let action = RunExtensionCommand {
            extension: item.extension_id.to_string(),
            command: item.command_id.to_string(),
        };
        window.dispatch_action(Box::new(action), cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let &item_ix = self.matches.get(ix)?;
        let item = self.commands.get(item_ix)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .gap_2()
                        .child(Label::new(item.title.clone()).size(LabelSize::Small))
                        .child(
                            Label::new(item.extension_name.clone())
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        )
    }
}

pub fn register_actions(workspace: &mut Workspace) {
    workspace
        .register_action(run_extension_command)
        .register_action(run_command_picker);
}

fn run_extension_command(
    workspace: &mut Workspace,
    action: &RunExtensionCommand,
    _window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let editor = workspace
        .active_item_as::<Editor>(cx)
        .and_then(|editor| active_editor_state(&editor, cx));

    let context = ExtensionCommandContext { editor };
    let task = ExtensionHostProxy::global(cx).run_command(
        action.extension.clone().into(),
        &action.command,
        context,
        cx,
    );
    task.detach_and_log_err(cx);
}

fn run_command_picker(
    workspace: &mut Workspace,
    _: &RunCommand,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(store) = ExtensionStore::try_global(cx) else {
        return;
    };
    let mut commands = Vec::new();
    for (extension_id, entry) in store.read(cx).installed_extensions() {
        for (command_id, command_entry) in &entry.manifest.commands {
            commands.push(ExtensionCommandItem {
                extension_id: extension_id.clone(),
                command_id: command_id.clone(),
                title: if command_entry.title.is_empty() {
                    command_id.to_string()
                } else {
                    command_entry.title.clone()
                },
                extension_name: entry.manifest.name.clone(),
            });
        }
    }
    commands.sort_by(|a, b| a.title.cmp(&b.title));
    let delegate = ExtensionCommandPickerDelegate::new(commands);
    workspace.toggle_modal(window, cx, move |window, cx| {
        Picker::uniform_list(delegate, window, cx)
    });
}

fn editor_path(editor: &Editor, cx: &App) -> String {
    let Some(buffer) = editor.buffer().read(cx).as_singleton() else {
        return String::new();
    };
    let Some(file) = buffer.read(cx).file() else {
        return String::new();
    };
    if let Some(abs_path) = file.file_system_abs_path(cx) {
        abs_path.to_string_lossy().into_owned()
    } else {
        file.full_path(cx).to_string_lossy().into_owned()
    }
}

fn editor_selections(
    editor: &Entity<Editor>,
    cx: &mut App,
) -> (Vec<std::ops::Range<u32>>, Vec<[u32; 2]>) {
    let display_snapshot = editor.update(cx, |editor, cx| editor.display_snapshot(cx));
    let ranges: Vec<std::ops::Range<u32>> = editor
        .read(cx)
        .selections
        .all::<MultiBufferOffset>(&display_snapshot)
        .into_iter()
        .map(|selection| (selection.start.0 as u32)..(selection.end.0 as u32))
        .collect();
    let offsets: Vec<[u32; 2]> = ranges.iter().map(|range| [range.start, range.end]).collect();
    (ranges, offsets)
}

fn dispatch_extension_event(kind: &str, payload: serde_json::Value, cx: &mut App) {
    let proxy = ExtensionHostProxy::global(cx);
    let payload_str = payload.to_string();
    let task = proxy.dispatch_event(kind, &payload_str, cx);
    task.detach_and_log_err(cx);
}

#[allow(dead_code)]
pub struct GlobalExtensionEventsWatcher(pub Entity<ExtensionEventsWatcher>);
impl Global for GlobalExtensionEventsWatcher {}

pub fn init_extension_events(workspace_store: Entity<WorkspaceStore>, cx: &mut App) {
    command_palette_hooks::GlobalCommandPaletteExtraCommands::set(cx, |cx| {
        let Some(store) = ExtensionStore::try_global(cx) else {
            return Vec::new();
        };
        let mut commands = Vec::new();
        for (extension_id, entry) in store.read(cx).installed_extensions() {
            for (command_id, command_entry) in &entry.manifest.commands {
                let title = if command_entry.title.is_empty() {
                    format!("{}: {}", entry.manifest.name, command_id)
                } else {
                    command_entry.title.clone()
                };
                let action: Box<dyn gpui::Action> = Box::new(RunExtensionCommand {
                    extension: extension_id.to_string(),
                    command: command_id.to_string(),
                });
                commands.push((title, action));
            }
        }
        commands
    });
    let watcher = cx.new(|cx| ExtensionEventsWatcher::new(workspace_store, cx));
    cx.set_global(GlobalExtensionEventsWatcher(watcher));
}

pub struct ExtensionEventsWatcher {
    workspace_store: Entity<WorkspaceStore>,
    active_editor: Option<WeakEntity<Editor>>,
    active_editor_sub: Option<Subscription>,
    debounce_task: Option<Task<()>>,
}

// These callbacks run while the workspace is being updated, and syncing reads it.
fn defer_sync_active_editor(watcher: WeakEntity<ExtensionEventsWatcher>, cx: &mut App) {
    cx.defer(move |cx| {
        if let Some(watcher) = watcher.upgrade() {
            watcher.update(cx, |watcher, cx| watcher.sync_active_editor(cx));
        }
    });
}

impl ExtensionEventsWatcher {
    fn new(workspace_store: Entity<WorkspaceStore>, cx: &mut Context<Self>) -> Self {
        cx.observe_new(|_editor: &mut Editor, _window, cx| {
            cx.subscribe_self(move |editor, event: &EditorEvent, cx| {
                if let EditorEvent::Saved = event {
                    let path = editor_path(editor, cx);
                    if !path.is_empty() {
                        let payload = serde_json::json!({ "path": path });
                        dispatch_extension_event("buffer-saved", payload, &mut *cx);
                    }
                }
            })
            .detach();
        })
        .detach();

        let watcher_handle = cx.entity().downgrade();
        cx.observe_new(move |_workspace: &mut Workspace, window, cx: &mut Context<Workspace>| {
            let watcher = watcher_handle.clone();
            cx.subscribe_self(move |_workspace, event: &workspace::Event, cx| {
                if let workspace::Event::ActiveItemChanged = event {
                    defer_sync_active_editor(watcher.clone(), cx);
                }
            })
            .detach();

            if let Some(window) = window {
                let watcher = watcher_handle.clone();
                cx.observe_window_activation(window, move |_workspace, window, cx| {
                    if window.is_window_active() {
                        defer_sync_active_editor(watcher.clone(), cx);
                    }
                })
                .detach();
            }

            defer_sync_active_editor(watcher_handle.clone(), cx);
        })
        .detach();

        let watcher_handle = cx.entity().downgrade();
        cx.defer(move |cx| {
            if let Some(watcher) = watcher_handle.upgrade() {
                watcher.update(cx, |watcher, cx| {
                    watcher.sync_active_editor(cx);
                });
            }
        });

        Self {
            workspace_store,
            active_editor: None,
            active_editor_sub: None,
            debounce_task: None,
        }
    }

    fn sync_active_editor(&mut self, cx: &mut Context<Self>) {
        let current_editor = active_or_first_workspace(&self.workspace_store, cx)
            .and_then(|(_, workspace)| workspace.read(cx).active_item_as::<Editor>(cx));

        let same_editor = match (&self.active_editor, &current_editor) {
            (Some(old), Some(new)) => old == &new.downgrade(),
            (None, None) => true,
            _ => false,
        };

        if same_editor {
            return;
        }

        self.active_editor = current_editor.as_ref().map(|editor| editor.downgrade());
        self.active_editor_sub = None;
        self.debounce_task = None;

        let path = current_editor
            .as_ref()
            .map(|editor| editor_path(editor.read(cx), cx))
            .unwrap_or_default();

        let payload = serde_json::json!({ "path": path });
        dispatch_extension_event("active-editor-changed", payload, &mut *cx);

        if let Some(editor) = current_editor {
            let editor_handle = editor.clone();
            self.active_editor_sub = Some(cx.subscribe(&editor, move |this, _editor, event: &EditorEvent, cx| {
                if let EditorEvent::SelectionsChanged { .. } = event {
                    this.on_selection_changed(&editor_handle, cx);
                }
            }));
        }
    }

    fn on_selection_changed(&mut self, editor: &Entity<Editor>, cx: &mut Context<Self>) {
        let editor = editor.clone();
        self.debounce_task = Some(cx.spawn({
            let editor = editor.downgrade();
            async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(150))
                    .await;
                this.update(cx, |_this, cx| {
                    if let Some(editor) = editor.upgrade() {
                        let path = editor_path(editor.read(cx), cx);
                        let (selections, offsets) = editor_selections(&editor, &mut *cx);
                        let payload = serde_json::json!({
                            "path": path,
                            "selections": selections,
                            "offsets": offsets,
                        });
                        dispatch_extension_event("selection-changed", payload, &mut *cx);
                    }
                })
                .log_err();
            }
        }));
    }
}

