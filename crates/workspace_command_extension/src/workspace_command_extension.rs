//! Bridges Zed extensions' workspace commands with the editor.
//!
//! Extension workspace commands allow extensions to register named commands that:
//! - Appear in the command palette under their human-readable name
//! - Can be bound to keybindings (`extensions::RunExtensionWorkspaceCommand`)
//! - Receive the path of the currently active file
//! - Return a file to open or a list of candidates for the user to pick from

use std::{path::PathBuf, sync::Arc};

use command_palette_hooks::{
    CommandInterceptItem, CommandInterceptResult, GlobalCommandPaletteInterceptor,
};
use extension::{
    Extension, ExtensionHostProxy, ExtensionWorkspaceCommandProxy, WorkspaceCommand,
    WorkspaceCommandResult,
};
use gpui::{
    Action, App, AppContext, Context, DismissEvent, ParentElement, SharedString, Task, WeakEntity,
    Window,
};
use parking_lot::RwLock;
use picker::{Picker, PickerDelegate};
use schemars::JsonSchema;
use serde::Deserialize;
use ui::{Label, LabelCommon, ListItem, ListItemSpacing, Toggleable};
use workspace::{OpenOptions, Workspace};

/// Key used to identify the workspace command extension's command palette interceptor.
struct WorkspaceCommandInterceptorKey;

/// Initializes workspace command extension support.
pub fn init(cx: &mut App) {
    let proxy = ExtensionHostProxy::default_global(cx);
    let registry = Arc::new(WorkspaceCommandRegistry::default());

    proxy.register_workspace_command_proxy(WorkspaceCommandRegistryProxy {
        registry: registry.clone(),
    });

    // Inject all registered workspace commands into the command palette.
    // Because RunExtensionWorkspaceCommand requires non-default fields it is
    // never emitted by window.available_actions(), so we use the interceptor
    // to surface each command by its human-readable name.
    let registry_for_interceptor = registry.clone();
    GlobalCommandPaletteInterceptor::set::<WorkspaceCommandInterceptorKey>(cx, move |query, _workspace, _cx| {
        let entries = registry_for_interceptor.palette_entries(query);
        Task::ready(CommandInterceptResult {
            results: entries,
            exclusive: false,
        })
    });

    let registry_for_observer = registry.clone();
    cx.observe_new(move |workspace: &mut Workspace, _window, _cx| {
        let registry = registry_for_observer.clone();
        workspace.register_action(
            move |workspace, action: &RunExtensionWorkspaceCommand, window, cx| {
                run_workspace_command(workspace, action.clone(), registry.clone(), window, cx);
            },
        );
    })
    .detach();
}

/// The action dispatched when the user invokes an extension workspace command.
///
/// Users can bind this action in their keybindings:
/// ```json
/// {
///   "context": "Workspace",
///   "bindings": {
///     "ctrl-alt-o": [
///       "extensions::RunExtensionWorkspaceCommand",
///       { "extension_id": "file-ext-switcher", "command_id": "switch-companion-file" }
///     ]
///   }
/// }
/// ```
///
/// The command palette uses the interceptor path instead — each registered
/// command appears under its human-readable name without requiring the user
/// to know the extension or command ID.
#[derive(Clone, Deserialize, PartialEq, JsonSchema, gpui::Action)]
#[action(namespace = extensions)]
#[serde(deny_unknown_fields)]
pub struct RunExtensionWorkspaceCommand {
    /// The ID of the extension that owns this command.
    pub extension_id: Arc<str>,
    /// The command identifier within the extension.
    pub command_id: String,
}

fn run_workspace_command(
    workspace: &mut Workspace,
    action: RunExtensionWorkspaceCommand,
    registry: Arc<WorkspaceCommandRegistry>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let active_file = workspace
        .active_item(cx)
        .and_then(|item| item.project_path(cx))
        .and_then(|project_path| {
            workspace
                .project()
                .read(cx)
                .worktree_for_id(project_path.worktree_id, cx)
                .map(|wt| wt.read(cx).abs_path().join(project_path.path.as_std_path()))
        })
        .map(|p| p.to_string_lossy().into_owned());

    let Some(extension) = registry.extension_for(&action.extension_id, &action.command_id) else {
        log::warn!(
            "No extension found for workspace command: {}::{}",
            action.extension_id,
            action.command_id,
        );
        return;
    };

    let command_id = action.command_id.clone();

    let task = cx.background_spawn(async move {
        extension
            .run_workspace_command(command_id, active_file, None)
            .await
    });

    cx.spawn_in(window, async move |workspace, cx| {
        let result = task.await?;
        workspace.update_in(cx, |workspace, window, cx| match result {
            WorkspaceCommandResult::OpenFile(path) => {
                workspace
                    .open_abs_path(PathBuf::from(path), OpenOptions::default(), window, cx)
                    .detach_and_log_err(cx);
            }
            WorkspaceCommandResult::PickAndOpen(candidates) => {
                let workspace_handle = cx.weak_entity();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let delegate =
                        CandidatePickerDelegate::new(candidates, workspace_handle);
                    Picker::uniform_list(delegate, window, cx)
                });
            }
            WorkspaceCommandResult::None => {}
        })
    })
    .detach_and_log_err(cx);
}

// ─── Candidate picker ────────────────────────────────────────────────────────

/// Delegate for the modal picker shown when an extension returns multiple
/// candidates. Presents each absolute path and opens the selected one.
struct CandidatePickerDelegate {
    /// All candidate absolute paths returned by the extension.
    candidates: Vec<String>,
    /// Indices into `candidates` that survive the current query filter.
    filtered: Vec<usize>,
    selected_ix: usize,
    workspace: WeakEntity<Workspace>,
}

impl CandidatePickerDelegate {
    fn new(candidates: Vec<String>, workspace: WeakEntity<Workspace>) -> Self {
        let filtered = (0..candidates.len()).collect();
        Self {
            candidates,
            filtered,
            selected_ix: 0,
            workspace,
        }
    }
}

impl PickerDelegate for CandidatePickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_ix = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a companion file…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query_lower = query.to_lowercase();
        self.filtered = self
            .candidates
            .iter()
            .enumerate()
            .filter(|(_, path)| {
                query_lower.is_empty() || path.to_lowercase().contains(&query_lower)
            })
            .map(|(i, _)| i)
            .collect();
        self.selected_ix = 0;
        cx.notify();
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(&ix) = self.filtered.get(self.selected_ix) else {
            return;
        };
        let path = PathBuf::from(&self.candidates[ix]);
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace
                    .open_abs_path(path, OpenOptions::default(), window, cx)
                    .detach_and_log_err(cx);
            });
        }
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
        let &candidate_ix = self.filtered.get(ix)?;
        let path = &self.candidates[candidate_ix];

        // Show the file name as the primary label and the directory as a
        // secondary muted label so the user can distinguish files that share
        // a base name but live in different directories.
        let file_name = path.rsplit('/').next().unwrap_or(path);
        let dir = path
            .rsplit_once('/')
            .map(|(d, _)| d)
            .unwrap_or("")
            .to_string();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(SharedString::from(file_name.to_string())))
                .end_slot::<Label>((!dir.is_empty()).then(|| {
                    Label::new(SharedString::from(dir))
                        .color(ui::Color::Muted)
                })),
        )
    }
}

// ─── Registry ────────────────────────────────────────────────────────────────

#[derive(Default)]
struct WorkspaceCommandRegistry {
    entries: RwLock<Vec<RegistryEntry>>,
}

struct RegistryEntry {
    extension_id: Arc<str>,
    command: WorkspaceCommand,
    extension: Arc<dyn Extension>,
}

impl WorkspaceCommandRegistry {
    fn extension_for(
        &self,
        extension_id: &Arc<str>,
        command_id: &str,
    ) -> Option<Arc<dyn Extension>> {
        self.entries
            .read()
            .iter()
            .find(|e| &e.extension_id == extension_id && e.command.id == command_id)
            .map(|e| e.extension.clone())
    }

    /// Returns palette entries for all registered commands that match `query`.
    ///
    /// Matching is case-insensitive substring search against the command's
    /// human-readable name. An empty query returns all commands.
    fn palette_entries(&self, query: &str) -> Vec<CommandInterceptItem> {
        let query_lower = query.to_lowercase();
        self.entries
            .read()
            .iter()
            .filter(|e| {
                query_lower.is_empty()
                    || e.command.name.to_lowercase().contains(&query_lower)
            })
            .map(|e| {
                let positions = match_positions(&e.command.name, &query_lower);
                CommandInterceptItem {
                    string: e.command.name.clone(),
                    action: RunExtensionWorkspaceCommand {
                        extension_id: e.extension_id.clone(),
                        command_id: e.command.id.clone(),
                    }
                    .boxed_clone(),
                    positions,
                }
            })
            .collect()
    }
}

/// Returns the byte positions in the *original* `text` where matched characters
/// begin (case-insensitive substring match, `query_lower` must already be
/// lower-case).
///
/// Each returned value is a char-boundary byte offset into `text`, suitable for
/// use with [`ui::HighlightedLabel`]. Positions are computed by walking `text`
/// and its lowercased form in parallel so that characters whose lower-case form
/// changes the UTF-8 byte length (e.g. `İ` → `i̇`) are mapped back to the
/// correct boundary in the original string.
fn match_positions(text: &str, query_lower: &str) -> Vec<usize> {
    if query_lower.is_empty() {
        return vec![];
    }
    let text_lower = text.to_lowercase();
    let Some(lower_match_start) = text_lower.find(query_lower) else {
        return vec![];
    };
    let lower_match_end = lower_match_start + query_lower.len();

    let mut orig_offset = 0usize;
    let mut lower_offset = 0usize;
    let mut positions = Vec::new();

    for ch in text.chars() {
        let orig_char_len = ch.len_utf8();
        let lower_char_len: usize = ch.to_lowercase().map(|c| c.len_utf8()).sum();

        // Include this char's original byte position if its lowercase form
        // overlaps the matched range in `text_lower`.
        if lower_offset < lower_match_end && lower_offset + lower_char_len > lower_match_start {
            positions.push(orig_offset);
        }

        orig_offset += orig_char_len;
        lower_offset += lower_char_len;

        if lower_offset >= lower_match_end {
            break;
        }
    }

    positions
}

// ─── Proxy ───────────────────────────────────────────────────────────────────

struct WorkspaceCommandRegistryProxy {
    registry: Arc<WorkspaceCommandRegistry>,
}

impl ExtensionWorkspaceCommandProxy for WorkspaceCommandRegistryProxy {
    fn register_workspace_command(
        &self,
        extension: Arc<dyn Extension>,
        command: WorkspaceCommand,
    ) {
        let extension_id = extension.manifest().id.clone();
        self.registry.entries.write().push(RegistryEntry {
            extension_id,
            command,
            extension,
        });
    }

    fn unregister_workspace_commands(&self, extension_id: Arc<str>) {
        self.registry
            .entries
            .write()
            .retain(|e| e.extension_id != extension_id);
    }
}
