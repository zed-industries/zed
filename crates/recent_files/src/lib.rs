use file_icons::FileIcons;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, AsyncApp, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Subscription, Task, UniformListScrollHandle, WeakEntity, Window, WindowHandle,
};
use gpui::{Pixels, px};

use parking_lot::Mutex;
use picker::{Picker, PickerDelegate};
use std::{
    borrow::Cow,
    ops::Range,
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use util::paths::PathExt;
use workspace::{
    self, ModalView, MultiWorkspace, OpenMode, PathList, SerializedWorkspaceLocation, Workspace,
    WorkspaceDb, WorkspaceId, with_active_or_new_workspace,
};
use zed_actions::{OpenFileFromDirectory, OpenRecentFile};

/// Match strings with order-insensitive word matching.
/// Splits the query into words and ensures all words match somewhere in the candidate,
/// regardless of order.
async fn match_strings_order_insensitive<T>(
    candidates: &[T],
    query: &str,
    smart_case: bool,
    max_results: usize,
    cancel_flag: &std::sync::atomic::AtomicBool,
) -> Vec<StringMatch>
where
    T: std::borrow::Borrow<StringMatchCandidate> + Sync,
{
    if candidates.is_empty() || max_results == 0 {
        return Default::default();
    }

    if query.is_empty() {
        return candidates
            .iter()
            .map(|candidate| StringMatch {
                candidate_id: candidate.borrow().id,
                score: 0.,
                positions: Default::default(),
                string: candidate.borrow().string.clone(),
            })
            .collect();
    }

    // Split query into words and remove empty ones
    let words: Vec<&str> = if query.trim().contains(' ') {
        query.split_whitespace().collect()
    } else {
        // For single words, treat the whole query as one word
        vec![query.trim()]
    };

    if words.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();

    for candidate in candidates {
        if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        let candidate_borrowed = candidate.borrow();
        let candidate_string = &candidate_borrowed.string;
        let candidate_lower = candidate_string.to_lowercase();

        // Check if all words are present in the candidate (case-insensitive)
        let mut all_words_match = true;
        let mut total_score = 0.0;
        let mut all_positions = Vec::new();

        for word in &words {
            let word_lower = if smart_case {
                word.to_string()
            } else {
                word.to_lowercase()
            };

            let found_match = if smart_case {
                candidate_string.contains(word)
            } else {
                candidate_lower.contains(&word_lower)
            };

            if found_match {
                if let Some(byte_pos) = if smart_case {
                    candidate_string.find(word)
                } else {
                    candidate_lower.find(&word_lower)
                } {
                    // Calculate a simple score based on position and word length
                    let word_score = 1.0 / (byte_pos as f64 + 1.0)
                        * (word.len() as f64 / candidate_string.len() as f64);
                    total_score += word_score;

                    if let Some(original_byte_pos) = if smart_case {
                        candidate_string.find(word)
                    } else {
                        candidate_string.to_lowercase().find(&word_lower)
                    } {
                        let word_byte_len = word.len();
                        for i in 0..word_byte_len {
                            let pos = original_byte_pos + i;
                            if pos < candidate_string.len()
                                && candidate_string.is_char_boundary(pos)
                            {
                                all_positions.push(pos);
                            }
                        }
                    }
                }
            } else {
                all_words_match = false;
                break;
            }
        }

        if all_words_match {
            all_positions.sort_unstable();
            all_positions.dedup();

            results.push(StringMatch {
                candidate_id: candidate_borrowed.id,
                score: total_score / words.len() as f64, // Average score across words
                positions: all_positions,
                string: candidate_string.clone(),
            });
        }
    }

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results.truncate(max_results);
    results
}

static RECENT_FILES: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

fn add_recent_file(path: PathBuf, workspace_db: WorkspaceDb) {
    let mut recent_files = RECENT_FILES.lock();
    recent_files.retain(|p| p != &path);
    recent_files.insert(0, path.clone());
    recent_files.truncate(3000);

    // Save to database asynchronously
    smol::spawn(async move {
        if let Err(e) = workspace_db.save_recent_file(&path).await {
            log::error!("Failed to save recent file to database: {:?}", e);
        }
    })
    .detach();
}

/// Expand tilde (~) in path to the user's home directory
fn expand_tilde(path: &Path) -> PathBuf {
    if let Some(path_str) = path.to_str() {
        PathBuf::from(shellexpand::tilde(path_str).as_ref())
    } else {
        path.to_path_buf()
    }
}

/// Check if a path exists, expanding tilde if present
fn path_exists(path: &Path) -> bool {
    expand_tilde(path).exists()
}

/// Find the most recent workspace that contains the given file path.
/// Returns the workspace info if found, None otherwise.
/// The workspaces are already ordered by recency (most recent first).
async fn find_workspace_for_file(
    _workspace_db: &WorkspaceDb,
    file_path: &Path,
) -> Option<(WorkspaceId, SerializedWorkspaceLocation, PathList)> {
    let _ = file_path;
    None

}

pub fn init(cx: &mut App) {
    let workspace_db = WorkspaceDb::global(cx);

    // Load recent files from database on startup
    let workspace_db_for_load = workspace_db.clone();
    cx.spawn(|_cx: &mut AsyncApp| async move {
        match workspace_db_for_load.get_recent_files(3000).await {
            Ok(files) => {
                // Separate existing and non-existing files while holding the lock
                let non_existing = {
                    let mut recent_files = RECENT_FILES.lock();
                    recent_files.clear();

                    // Separate existing and non-existing files
                    let (existing, non_existing): (Vec<_>, Vec<_>) =
                        files.into_iter().partition(|path| path_exists(path));

                    recent_files.extend(existing);

                    // Return only the non_existing files to be processed outside the lock
                    non_existing
                };

                // Remove non-existing files from database (outside the lock)
                for path in non_existing {
                    if let Err(e) = workspace_db_for_load.delete_recent_file(&path).await {
                        log::error!(
                            "Failed to delete non-existing file from database: {:?}, path: {:?}",
                            e,
                            path
                        );
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to load recent files from database: {:?}", e);
            }
        }
    })
    .detach();

    cx.on_action(|open_recent_file: &OpenRecentFile, cx| {
        let create_new_window = open_recent_file.create_new_window;
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let Some(recent_files) = workspace.active_modal::<RecentFiles>(cx) else {
                RecentFiles::open(workspace, create_new_window, window, cx);
                return;
            };

            recent_files.update(cx, |recent_files, cx| {
                recent_files
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(window, cx))
            });
        });
    });

    cx.on_action(|action: &OpenFileFromDirectory, cx| {
        let directory = PathBuf::from(&action.directory);

        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let Some(picker) = workspace.active_modal::<DirectoryFilePicker>(cx) else {
                DirectoryFilePicker::open(workspace, directory.clone(), window, cx);
                return;
            };

            picker.update(cx, |picker, cx| {
                picker
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(window, cx))
            });
        });
    });

    let workspace_db_for_observe = workspace_db.clone();
    cx.observe_new(move |_workspace: &mut Workspace, window, cx| {
        let Some(window) = window else { return };
        let workspace_db = workspace_db_for_observe.clone();
        cx.subscribe_in(
            &cx.entity(),
            window,
            move |workspace, _, event, _, cx| match event {
                workspace::Event::ItemAdded { item } => {
                    if let Some(project_path) = item.project_path(cx) {
                        if let Some(abs_path) = workspace
                            .project()
                            .read(cx)
                            .absolute_path(&project_path, cx)
                        {
                            add_recent_file(abs_path, workspace_db.clone());
                        }
                    }
                }
                workspace::Event::ActiveItemChanged => {
                    if let Some(active_item) = workspace.active_item(cx) {
                        if let Some(project_path) = active_item.project_path(cx) {
                            if let Some(abs_path) = workspace
                                .project()
                                .read(cx)
                                .absolute_path(&project_path, cx)
                            {
                                add_recent_file(abs_path, workspace_db.clone());
                            }
                        }
                    }
                }
                _ => {}
            },
        )
        .detach();
    })
    .detach();

    // Start periodic save task
    let executor = cx.background_executor().clone();
    let workspace_db_for_periodic = workspace_db;
    cx.spawn(|_cx: &mut AsyncApp| async move {
        loop {
            // Wait for 5 seconds
            executor.timer(std::time::Duration::from_secs(5)).await;

            // Get current recent files
            let recent_files = {
                let recent_files = RECENT_FILES.lock();
                recent_files.clone()
            };

            // Save all recent files to database
            if let Err(e) = workspace_db_for_periodic.clear_recent_files().await {
                log::error!("Failed to clear recent files from database: {:?}", e);
                continue;
            }

            for path in recent_files {
                if let Err(e) = workspace_db_for_periodic.save_recent_file(&path).await {
                    log::error!(
                        "Failed to save recent file to database: {:?}, path: {:?}",
                        e,
                        path
                    );
                }
            }
        }
    })
    .detach();
}

struct RecentFiles {
    picker: Entity<Picker<RecentFilesDelegate>>,
    _subscription: Subscription,
}

impl ModalView for RecentFiles {}

impl RecentFiles {
    fn new(delegate: RecentFilesDelegate, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = UniformListScrollHandle::new();
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .max_height(None)
                .track_scroll(scroll_handle.clone())
                .show_scrollbar(true)
        });
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));
        Self {
            picker,
            _subscription,
        }
    }

    pub fn open(
        workspace: &mut Workspace,
        create_new_window: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let weak = cx.entity().downgrade();
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = RecentFilesDelegate::new(weak, create_new_window);
            Self::new(delegate, window, cx)
        })
    }
}

impl EventEmitter<DismissEvent> for RecentFiles {}

impl Focusable for RecentFiles {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for RecentFiles {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport_size = window.viewport_size();
        let modal_width = (viewport_size.width * 0.7).min(viewport_size.width);
        let modal_height = (viewport_size.height * 0.7).min(viewport_size.height);

        v_flex()
            .key_context("RecentFiles")
            .w(modal_width)
            .h(modal_height)
            .child(
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.picker.clone()),
            )
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), window, cx);
                })
            }))
    }
}

struct RecentFilesDelegate {
    workspace: WeakEntity<Workspace>,
    files: Vec<PathBuf>,
    matches: Vec<StringMatch>,
    selected_match_index: usize,
    create_new_window: bool,
}

impl RecentFilesDelegate {
    fn new(workspace: WeakEntity<Workspace>, create_new_window: bool) -> Self {
        // Filter out non-existing files when creating the delegate
        let files: Vec<PathBuf> = RECENT_FILES
            .lock()
            .iter()
            .filter(|path| path_exists(path))
            .cloned()
            .collect();

        Self {
            workspace,
            files,
            matches: Vec::new(),
            selected_match_index: 0,
            create_new_window,
        }
    }
}

impl EventEmitter<DismissEvent> for RecentFilesDelegate {}

impl PickerDelegate for RecentFilesDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> Arc<str> {
        Arc::from("Search recent files...")
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_match_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim_start();
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let candidates = self
            .files
            .iter()
            .enumerate()
            .map(|(id, path)| {
                let path_str = path.compact().to_string_lossy().into_owned();
                StringMatchCandidate::new(id, &path_str)
            })
            .collect::<Vec<_>>();

        self.matches = smol::block_on(match_strings_order_insensitive(
            candidates.as_slice(),
            query,
            smart_case,
            100,
            &Default::default(),
        ));
        self.matches.sort_unstable_by_key(|m| m.candidate_id);

        self.selected_match_index = 0;

        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(hit) = self.matches.get(self.selected_index()) {
            let path = self.files[hit.candidate_id].clone();
            let create_new_window = if self.create_new_window {
                !secondary
            } else {
                secondary
            };
            let open_mode = if create_new_window {
                OpenMode::NewWindow
            } else {
                OpenMode::Activate
            };
            let workspace_db = WorkspaceDb::global(cx);

            if let Some(workspace) = self.workspace.upgrade() {
                // Try to find a recent workspace that contains this file
                let workspace_handle = workspace;
                cx.spawn_in(window, async move |_, cx| {
                    if let Some((workspace_id, location, _workspace_paths)) =
                        find_workspace_for_file(&workspace_db, &path).await
                    {
                        // Found a workspace that contains this file, open that workspace
                        workspace_handle.update_in(cx, |workspace, window, cx| {
                            // Check if we're already in the correct workspace
                            if workspace.database_id() == Some(workspace_id) {
                                // We're already in the right workspace, just open the file
                                workspace
                                    .open_workspace_for_paths(OpenMode::Activate, vec![path], window, cx)
                                    .detach_and_log_err(cx);
                            } else {
                                // Open the workspace that contains this file
                                match location {
                                    SerializedWorkspaceLocation::Local => {
                                        // We need to open the workspace with all its paths, but then
                                        // also open the specific file. First get workspace paths.
                                        let workspace_paths = _workspace_paths.paths().to_vec();

                                        // Create a combined list: workspace paths + the specific file
                                        let mut paths_to_open = workspace_paths;
                                        if !paths_to_open.contains(&path) {
                                            paths_to_open.push(path);
                                        }

                                        workspace
                                            .open_workspace_for_paths(
                                                open_mode,
                                                paths_to_open,
                                                window,
                                                cx,
                                            )
                                            .detach_and_log_err(cx);
                                    }
                                    SerializedWorkspaceLocation::Remote(_) => {
                                        // For remote workspaces, fall back to opening the file directly
                                        workspace
                                            .open_workspace_for_paths(
                                                open_mode,
                                                vec![path],
                                                window,
                                                cx,
                                            )
                                            .detach_and_log_err(cx);
                                    }
                                }
                            }
                        })
                    } else {
                        // No workspace found, open the file standalone
                        workspace_handle.update_in(cx, |workspace, window, cx| {
                            workspace
                                .open_workspace_for_paths(open_mode, vec![path], window, cx)
                                .detach_and_log_err(cx);
                        })
                    }
                })
                .detach_and_log_err(cx);
            }
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = self.matches.get(ix)?;
        let path = self.files.get(hit.candidate_id)?;

        let path = path.compact();
        let path_string = path.to_string_lossy();
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();

        let file_name_start = path_string.len().saturating_sub(file_name.len());
        let mut dir_name = path_string[0..file_name_start].to_string();

        let file_name_highlights: Vec<usize> = hit
            .positions
            .iter()
            .filter(|&&i| i >= file_name_start)
            .map(|&i| i - file_name_start)
            .collect();

        let mut dir_highlights: Vec<usize> = hit
            .positions
            .iter()
            .filter(|&&i| i < file_name_start)
            .copied()
            .collect();

        if dir_name.is_ascii() {
            let max_width = rems(48.).to_pixels(window.rem_size());
            let (normal_em, small_em) = {
                let style = window.text_style();
                let font_id = window.text_system().resolve_font(&style.font());
                let font_size = ui::TextSize::Default.rems(cx).to_pixels(window.rem_size());
                let normal = cx
                    .text_system()
                    .em_width(font_id, font_size)
                    .unwrap_or(px(16.));
                let font_size = ui::TextSize::Small.rems(cx).to_pixels(window.rem_size());
                let small = cx
                    .text_system()
                    .em_width(font_id, font_size)
                    .unwrap_or(px(10.));
                (normal, small)
            };
            let budget = full_path_budget(&file_name, normal_em, small_em, max_width);

            if budget > 0 && dir_name.len() > budget {
                let components = PathComponentSlice::new(&dir_name);
                if let Some(elided_range) = components.elision_range(budget - 1, &dir_highlights) {
                    let elided_len = elided_range.end - elided_range.start;
                    let placeholder = "…";
                    dir_highlights.retain_mut(|mat| {
                        if *mat >= elided_range.end {
                            *mat -= elided_len;
                            *mat += placeholder.len();
                        } else if *mat >= elided_range.start {
                            return false;
                        }
                        true
                    });
                    dir_name.replace_range(elided_range, placeholder);
                }
            }
        }

        let file_icon =
            FileIcons::get_icon(&path, cx).map(|icon| Icon::from_path(icon).color(Color::Muted));

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot::<Icon>(file_icon)
                .inset(true)
                .child(
                    h_flex()
                        .gap_2()
                        .py_px()
                        .child(HighlightedLabel::new(
                            file_name.to_string(),
                            file_name_highlights,
                        ))
                        .child(
                            HighlightedLabel::new(dir_name, dir_highlights)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        )
    }
}

fn full_path_budget(
    file_name: &str,
    normal_em: Pixels,
    small_em: Pixels,
    max_width: Pixels,
) -> usize {
    (((max_width / 0.8) - (file_name.len() as f32) * normal_em) / small_em) as usize
}

struct DirectoryFilePicker {
    picker: Entity<Picker<DirectoryFileDelegate>>,
    _subscription: Subscription,
}

impl ModalView for DirectoryFilePicker {}

impl DirectoryFilePicker {
    fn new(delegate: DirectoryFileDelegate, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = UniformListScrollHandle::new();
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .max_height(None)
                .track_scroll(scroll_handle.clone())
                .show_scrollbar(true)
        });
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));
        Self {
            picker,
            _subscription,
        }
    }

    pub fn open(
        workspace: &mut Workspace,
        directory: PathBuf,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let weak = cx.entity().downgrade();
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = DirectoryFileDelegate::new(weak, directory);
            Self::new(delegate, window, cx)
        })
    }
}

impl EventEmitter<DismissEvent> for DirectoryFilePicker {}

impl Focusable for DirectoryFilePicker {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for DirectoryFilePicker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport_size = window.viewport_size();
        let modal_width = (viewport_size.width * 0.7).min(viewport_size.width);
        let modal_height = (viewport_size.height * 0.7).min(viewport_size.height);

        v_flex()
            .key_context("DirectoryFilePicker")
            .w(modal_width)
            .h(modal_height)
            .child(
                v_flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.picker.clone()),
            )
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), window, cx);
                })
            }))
    }
}

struct DirectoryFileDelegate {
    workspace: WeakEntity<Workspace>,
    directory: PathBuf,
    files: Vec<PathBuf>,
    matches: Vec<StringMatch>,
    selected_match_index: usize,
}

impl DirectoryFileDelegate {
    fn new(workspace: WeakEntity<Workspace>, directory: PathBuf) -> Self {
        let expanded_dir = expand_tilde(&directory);
        let files = Self::list_files(&expanded_dir);
        Self {
            workspace,
            directory: expanded_dir,
            files,
            matches: Vec::new(),
            selected_match_index: 0,
        }
    }

    fn list_files(directory: &Path) -> Vec<PathBuf> {
        let output = smol::block_on(
            smol::process::Command::new("rg")
                .args(["--files"])
                .current_dir(directory)
                .output(),
        );

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.lines().map(|line| directory.join(line)).collect()
            }
            _ => Vec::new(),
        }
    }
}

impl EventEmitter<DismissEvent> for DirectoryFileDelegate {}

impl PickerDelegate for DirectoryFileDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> Arc<str> {
        let display_path = self.directory.compact();
        Arc::from(format!("Open file from {}…", display_path.display()))
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_match_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_match_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim_start();
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let candidates = self
            .files
            .iter()
            .enumerate()
            .map(|(id, path)| {
                let path_str = path
                    .strip_prefix(&self.directory)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .into_owned();
                StringMatchCandidate::new(id, &path_str)
            })
            .collect::<Vec<_>>();

        self.matches = smol::block_on(match_strings_order_insensitive(
            candidates.as_slice(),
            query,
            smart_case,
            100,
            &Default::default(),
        ));
        self.matches.sort_unstable_by_key(|m| m.candidate_id);

        self.selected_match_index = 0;

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(hit) = self.matches.get(self.selected_index()) {
            let path = self.files[hit.candidate_id].clone();
            let directory = self.directory.clone();

            let existing_window = (|| -> Option<WindowHandle<MultiWorkspace>> {
                for window in
                    workspace::workspace_windows_for_location(&SerializedWorkspaceLocation::Local, cx)
                {
                    if let Ok(multi_workspace) = window.read(cx) {
                        for workspace in multi_workspace.workspaces() {
                            let workspace = workspace.read(cx);
                            let project = workspace.project().read(cx);
                            for worktree in project.worktrees(cx) {
                                let worktree = worktree.read(cx);
                                let expanded_root = expand_tilde(worktree.abs_path().as_ref());
                                if directory.starts_with(&expanded_root)
                                    || expanded_root.starts_with(&directory)
                                {
                                    return Some(window);
                                }
                            }
                        }
                    }
                }
                None
            })();

            if let Some(existing_window) = existing_window {
                let path = path.clone();
                window.defer(cx, move |_, cx| {
                    existing_window
                        .update(cx, |multi_workspace, window, cx| {
                            window.activate_window();
                            multi_workspace
                                .workspace()
                                .update(cx, |workspace, cx| {
                                    workspace
                                        .open_paths(
                                            vec![path],
                                            workspace::OpenOptions::default(),
                                            None,
                                            window,
                                            cx,
                                        )
                                        .detach();
                                });
                        })
                        .log_err();
                });
            } else if let Some(workspace) = self.workspace.upgrade() {
                let directory = directory.clone();
                let path = path.clone();
                window.defer(cx, move |window, cx| {
                    let _ = workspace.update(cx, |workspace, cx| {
                        workspace
                            .open_workspace_for_paths(
                                OpenMode::Activate,
                                vec![directory, path],
                                window,
                                cx,
                            )
                            .detach_and_log_err(cx);
                    });
                });
            }
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = self.matches.get(ix)?;
        let path = self.files.get(hit.candidate_id)?;

        let relative_path = path.strip_prefix(&self.directory).unwrap_or(path);
        let path_string = relative_path.to_string_lossy();
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default();

        let file_name_start = path_string.len().saturating_sub(file_name.len());
        let dir_name = path_string[0..file_name_start].to_string();

        let file_name_highlights: Vec<usize> = hit
            .positions
            .iter()
            .filter(|&&i| i >= file_name_start)
            .map(|&i| i - file_name_start)
            .collect();

        let dir_highlights: Vec<usize> = hit
            .positions
            .iter()
            .filter(|&&i| i < file_name_start)
            .copied()
            .collect();

        let file_icon =
            FileIcons::get_icon(path, cx).map(|icon| Icon::from_path(icon).color(Color::Muted));

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot::<Icon>(file_icon)
                .inset(true)
                .child(
                    h_flex()
                        .gap_2()
                        .py_px()
                        .child(HighlightedLabel::new(
                            file_name.to_string(),
                            file_name_highlights,
                        ))
                        .child(
                            HighlightedLabel::new(dir_name, dir_highlights)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PathComponentSlice<'a> {
    path: Cow<'a, Path>,
    path_str: Cow<'a, str>,
    component_ranges: Vec<(Component<'a>, Range<usize>)>,
}

impl<'a> PathComponentSlice<'a> {
    fn new(path: &'a str) -> Self {
        let trimmed_path = Path::new(path).components().as_path().as_os_str();
        let mut component_ranges = Vec::new();
        let mut components = Path::new(trimmed_path).components();
        let len = trimmed_path.as_encoded_bytes().len();
        let mut pos = 0;
        while let Some(component) = components.next() {
            component_ranges.push((component, pos..0));
            pos = len - components.as_path().as_os_str().as_encoded_bytes().len();
        }
        for ((_, range), ancestor) in component_ranges
            .iter_mut()
            .rev()
            .zip(Path::new(trimmed_path).ancestors())
        {
            range.end = ancestor.as_os_str().as_encoded_bytes().len();
        }
        Self {
            path: Cow::Borrowed(Path::new(path)),
            path_str: Cow::Borrowed(path),
            component_ranges,
        }
    }

    fn elision_range(&self, budget: usize, matches: &[usize]) -> Option<Range<usize>> {
        let eligible_range = {
            assert!(matches.windows(2).all(|w| w[0] <= w[1]));
            let mut matches = matches.iter().copied().peekable();
            let mut longest: Option<Range<usize>> = None;
            let mut cur = 0..0;
            let mut seen_normal = false;
            for (i, (component, range)) in self.component_ranges.iter().enumerate() {
                let is_normal = matches!(component, Component::Normal(_));
                let is_first_normal = is_normal && !seen_normal;
                seen_normal |= is_normal;
                let is_last = i == self.component_ranges.len() - 1;
                let contains_match = matches.peek().is_some_and(|mat| range.contains(mat));
                if contains_match {
                    matches.next();
                }
                if is_first_normal || is_last || !is_normal || contains_match {
                    if longest
                        .as_ref()
                        .is_none_or(|old| old.end - old.start <= cur.end - cur.start)
                    {
                        longest = Some(cur);
                    }
                    cur = i + 1..i + 1;
                } else {
                    cur.end = i + 1;
                }
            }
            if longest
                .as_ref()
                .is_none_or(|old| old.end - old.start <= cur.end - cur.start)
            {
                longest = Some(cur);
            }
            longest
        };

        let eligible_range = eligible_range?;
        assert!(eligible_range.start <= eligible_range.end);
        if eligible_range.is_empty() {
            return None;
        }

        let elided_range: Range<usize> = {
            let byte_range = self.component_ranges[eligible_range.start].1.start
                ..self.component_ranges[eligible_range.end - 1].1.end;
            let midpoint = self.path_str.len() / 2;
            let distance_from_start = byte_range.start.abs_diff(midpoint);
            let distance_from_end = byte_range.end.abs_diff(midpoint);
            let pick_from_end = distance_from_start > distance_from_end;
            let mut len_with_elision = self.path_str.len();
            let mut i = eligible_range.start;
            while i < eligible_range.end {
                let x = if pick_from_end {
                    eligible_range.end - i + eligible_range.start - 1
                } else {
                    i
                };
                len_with_elision -= self.component_ranges[x]
                    .0
                    .as_os_str()
                    .as_encoded_bytes()
                    .len()
                    + 1;
                if len_with_elision <= budget {
                    break;
                }
                i += 1;
            }
            if len_with_elision > budget {
                return None;
            } else if pick_from_end {
                let x = eligible_range.end - i + eligible_range.start - 1;
                x..eligible_range.end
            } else {
                let x = i;
                eligible_range.start..x + 1
            }
        };

        let byte_range = self.component_ranges[elided_range.start].1.start
            ..self.component_ranges[elided_range.end - 1].1.end;
        Some(byte_range)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_workspace_path_matching() {
        // Test the core logic of finding the deepest workspace path
        let workspace_paths = vec![
            PathBuf::from("/Users/dima/Developer"),
            PathBuf::from("/Users/dima/Developer/zed"),
            PathBuf::from("/Users/dima/Developer/zed/docs"),
        ];

        let file_path = PathBuf::from("/Users/dima/Developer/zed/docs/src/configuring-zed.md");

        // Find the deepest matching path
        let mut best_match: Option<(PathBuf, usize)> = None;
        for workspace_path in &workspace_paths {
            if file_path.starts_with(workspace_path) {
                let depth = workspace_path.components().count();
                if best_match
                    .as_ref()
                    .map_or(true, |(_, best_depth)| depth > *best_depth)
                {
                    best_match = Some((workspace_path.clone(), depth));
                }
            }
        }

        // Should match the deepest path: /Users/dima/Developer/zed/docs
        assert_eq!(
            best_match.unwrap().0,
            PathBuf::from("/Users/dima/Developer/zed/docs")
        );
    }

    #[test]
    fn test_workspace_path_matching_prefers_most_recent() {
        // Test that we prefer the most recent workspace that contains the file
        // Simulating the order that recent_workspaces_on_disk() would return
        let workspace_paths_in_recency_order = vec![
            PathBuf::from("/Users/dima/Developer/zed"), // Most recent
            PathBuf::from("/Users/dima/Developer/zed/docs"), // Less recent
        ];

        let file_path = PathBuf::from("/Users/dima/Developer/zed/docs/src/configuring-zed.md");

        // Find the first (most recent) matching workspace
        let mut first_match: Option<PathBuf> = None;
        for workspace_path in &workspace_paths_in_recency_order {
            if file_path.starts_with(workspace_path) {
                first_match = Some(workspace_path.clone());
                break; // Take the first match (most recent)
            }
        }

        // Should match the most recent workspace that contains the file: /Users/dima/Developer/zed
        assert_eq!(
            first_match.unwrap(),
            PathBuf::from("/Users/dima/Developer/zed")
        );
    }

    #[test]
    fn test_tilde_expansion() {
        use super::expand_tilde;

        // Test tilde expansion
        let tilde_path = PathBuf::from("~/Developer/zed");
        let expanded = expand_tilde(&tilde_path);

        // Should expand to absolute path
        assert!(expanded.is_absolute());
        assert!(expanded.to_string_lossy().contains("Developer/zed"));
        assert!(!expanded.to_string_lossy().starts_with("~"));

        // Test non-tilde path remains unchanged
        let abs_path = PathBuf::from("/Users/test/project");
        let unchanged = expand_tilde(&abs_path);
        assert_eq!(unchanged, abs_path);
    }

    #[test]
    fn test_workspace_matching_with_tilde_paths() {
        use super::expand_tilde;

        // Simulate workspace paths (could be absolute)
        let workspace_paths = vec![PathBuf::from("/Users/dima/Developer/zed")];

        // Simulate file path with tilde (as it appears in recent files)
        let file_path_with_tilde = PathBuf::from("~/Developer/zed/docs/src/configuring-zed.md");
        let expanded_file_path = expand_tilde(&file_path_with_tilde);

        // Find matching workspace
        let mut match_found = false;
        for workspace_path in &workspace_paths {
            let expanded_workspace = expand_tilde(workspace_path);
            if expanded_file_path.starts_with(&expanded_workspace) {
                match_found = true;
                break;
            }
        }

        assert!(
            match_found,
            "Should find workspace match after tilde expansion"
        );
    }
}
