mod unified_palette_tests;

use std::sync::Arc;

use gpui::{
    actions, Action, App, AppContext, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, Task, Window, WeakEntity, prelude::*,
};
use gpui_util::ResultExt;
use picker::{Picker, PickerDelegate};
use project::{ProjectPath, WorktreeId};
use ui::{prelude::*, Label, ListItem, ListItemSpacing};
use util::rel_path::RelPath;
use workspace::{ModalView, Workspace};

actions!(unified_palette, [ToggleUnifiedPalette]);

pub fn init(cx: &mut App) {
    cx.observe_new(UnifiedPalette::register).detach();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteMode {
    FileFinder,
    CommandPalette,
    ProjectSymbols,
    Outline,
    GoToLine,
}

pub struct UnifiedPalette {
    picker: Entity<Picker<UnifiedPaletteDelegate>>,
    _workspace: WeakEntity<workspace::Workspace>,
    _subscription: gpui::Subscription,
}

#[derive(Clone)]
enum Match {
    File(FileMatch),
    Command(CommandMatch),
    Line(LineMatch),
}

#[derive(Clone)]
struct FileMatch {
    worktree_id: WorktreeId,
    path: Arc<RelPath>,
    display_path: String,
}

#[derive(Clone)]
struct CommandMatch {
    name: String,
    action: Arc<dyn Action>,
}

#[derive(Clone)]
struct LineMatch {
    line_number: u32,
}

pub struct UnifiedPaletteDelegate {
    mode: PaletteMode,
    workspace: WeakEntity<Workspace>,
    project: Entity<project::Project>,
    unified_palette: WeakEntity<UnifiedPalette>,
    
    // Match data
    matches: Vec<Match>,
    selected_index: usize,
    last_query: String,
}

impl UnifiedPalette {
    fn register(
        workspace: &mut workspace::Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<workspace::Workspace>,
    ) {
        workspace.register_action(
            |workspace, _action: &workspace::ToggleFileFinder, window, cx| {
                let project = workspace.project().clone();
                let workspace_handle = cx.entity().downgrade();
                
                workspace.toggle_modal(window, cx, move |window, cx| {
                    let delegate = UnifiedPaletteDelegate::new(
                        workspace_handle.clone(),
                        project,
                        cx.entity().downgrade(),
                        cx,
                    );
                    let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
                    
                    let subscription = cx.subscribe(&picker, |_this, _, _: &DismissEvent, cx| {
                        cx.emit(DismissEvent);
                    });
                    
                    UnifiedPalette {
                        picker,
                        _workspace: workspace_handle,
                        _subscription: subscription,
                    }
                });
            },
        );
    }
    
    pub fn new(
        workspace: &mut workspace::Workspace,
        window: &mut Window,
        cx: &mut Context<workspace::Workspace>,
    ) -> Entity<Self> {
        let workspace_handle = cx.entity().downgrade();
        let project = workspace.project().clone();
        
        cx.new(|cx| {
            let delegate = UnifiedPaletteDelegate::new(
                workspace_handle.clone(),
                project,
                cx.entity().downgrade(),
                cx,
            );
            let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
            
            let subscription = cx.subscribe(&picker, |_this, _, _: &DismissEvent, cx| {
                cx.emit(DismissEvent);
            });
            
            Self {
                picker,
                _workspace: workspace_handle,
                _subscription: subscription,
            }
        })
    }
}

impl UnifiedPaletteDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<project::Project>,
        unified_palette: WeakEntity<UnifiedPalette>,
        _cx: &mut App,
    ) -> Self {
        Self {
            mode: PaletteMode::FileFinder,
            workspace,
            project,
            unified_palette,
            matches: Vec::new(),
            selected_index: 0,
            last_query: String::new(),
        }
    }
    
    fn search_files(&mut self, query: &str, cx: &mut Context<Picker<Self>>) {
        if query.is_empty() {
            self.matches.clear();
            return;
        }
        
        let project = self.project.read(cx);
        let mut files = Vec::new();
        
        for worktree in project.worktrees(cx) {
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            
            for entry in worktree.files(false, 0) {
                let path_str = format!("{:?}", entry.path).trim_matches('"').to_string();
                if path_str.to_lowercase().contains(&query.to_lowercase()) {
                    files.push(Match::File(FileMatch {
                        worktree_id,
                        path: entry.path.clone(),
                        display_path: path_str,
                    }));
                    if files.len() >= 100 {
                        break;
                    }
                }
            }
        }
        
        self.matches = files;
        self.selected_index = 0;
    }
    
    fn search_commands(&mut self, query: &str, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let actions = window.available_actions(cx);
        let mut commands = Vec::new();
        
        for action in actions {
            let name = action.name();
            if query.is_empty() || name.to_lowercase().contains(&query.to_lowercase()) {
                commands.push(Match::Command(CommandMatch {
                    name: name.to_string(),
                    action: Arc::from(action),
                }));
                if commands.len() >= 100 {
                    break;
                }
            }
        }
        
        self.matches = commands;
        self.selected_index = 0;
    }
    
    fn search_line(&mut self, query: &str, _cx: &mut Context<Picker<Self>>) {
        if let Ok(line_number) = query.parse::<u32>() {
            self.matches = vec![Match::Line(LineMatch { line_number })];
        } else {
            self.matches.clear();
        }
        self.selected_index = 0;
    }
    
    fn search_project_symbols(&mut self, query: &str, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        // Project symbols search requires complex async handling
        // For now, show a placeholder message
        if !query.is_empty() {
            log::info!("UnifiedPalette: Project symbols search for '{}' - full implementation pending", query);
        }
        self.matches.clear();
        self.selected_index = 0;
    }
    
    fn search_outline(&mut self, _query: &str, _cx: &mut Context<Picker<Self>>) {
        // Outline mode requires more complex integration with editor
        // For now, just clear matches and log
        log::warn!("UnifiedPalette: Outline mode not fully implemented yet");
        self.matches.clear();
        self.selected_index = 0;
    }
}

impl PickerDelegate for UnifiedPaletteDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        let text = match self.mode {
            PaletteMode::FileFinder => "Go to file...".into(),
            PaletteMode::CommandPalette => "Execute a command...".into(),
            PaletteMode::ProjectSymbols => "Go to symbol...".into(),
            PaletteMode::Outline => "Go to symbol in editor...".into(),
            PaletteMode::GoToLine => "Go to line...".into(),
        };
        log::trace!("UnifiedPalette: Placeholder text for {:?}: {}", self.mode, text);
        text
    }

    fn update_matches(&mut self, query: String, window: &mut Window, cx: &mut Context<Picker<Self>>) -> Task<()> {
        // Store the raw query
        self.last_query = query.clone();
        
        // Detect mode from prefix
        let (new_mode, stripped_query) = if let Some(detected_mode) = detect_mode_from_query(&query) {
            let stripped = query.chars().skip(1).collect::<String>();
            (detected_mode, stripped)
        } else {
            (PaletteMode::FileFinder, query.clone())
        };
        
        // Switch mode if changed
        if new_mode != self.mode {
            log::info!("UnifiedPalette: Mode changed from {:?} to {:?}", self.mode, new_mode);
            self.mode = new_mode;
            self.matches.clear();
            cx.notify();
        }
        
        log::debug!("UnifiedPalette: Searching in {:?} mode with query: '{}'", self.mode, stripped_query);
        
        // Search based on mode
        match self.mode {
            PaletteMode::FileFinder => {
                self.search_files(&stripped_query, cx);
                log::debug!("UnifiedPalette: Found {} file matches", self.matches.len());
            }
            PaletteMode::CommandPalette => {
                self.search_commands(&stripped_query, window, cx);
                log::debug!("UnifiedPalette: Found {} command matches", self.matches.len());
            }
            PaletteMode::GoToLine => {
                self.search_line(&stripped_query, cx);
                log::debug!("UnifiedPalette: Found {} line matches", self.matches.len());
            }
            PaletteMode::ProjectSymbols => {
                self.search_project_symbols(&stripped_query, window, cx);
                log::debug!("UnifiedPalette: Searching for project symbols with query: '{}'", stripped_query);
            }
            PaletteMode::Outline => {
                self.search_outline(&stripped_query, cx);
                log::debug!("UnifiedPalette: Found {} outline matches", self.matches.len());
            }
        }
        
        cx.notify();
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        log::info!("UnifiedPalette: Confirm called in {:?} mode (secondary: {})", self.mode, secondary);
        
        // Don't confirm if query is just a prefix (>, #, @, :) with no actual search text
        if self.last_query.len() == 1 && matches!(self.last_query.chars().next(), Some('>') | Some('#') | Some('@') | Some(':')) {
            log::warn!("UnifiedPalette: Query is just a prefix, ignoring confirm");
            return;
        }
        
        let Some(selected_match) = self.matches.get(self.selected_index).cloned() else {
            log::warn!("UnifiedPalette: No match selected, ignoring confirm");
            return;
        };
        
        let Some(workspace) = self.workspace.upgrade() else {
            log::error!("UnifiedPalette: Workspace no longer exists, dismissing");
            self.unified_palette.update(cx, |_, cx| cx.emit(DismissEvent)).log_err();
            return;
        };
        
        match selected_match {
            Match::File(file_match) => {
                log::info!("UnifiedPalette: Opening file: {}", file_match.display_path);
                let project_path = ProjectPath {
                    worktree_id: file_match.worktree_id,
                    path: file_match.path,
                };
                
                let open_task = workspace.update(cx, |workspace, cx| {
                    if secondary {
                        workspace.split_path_preview(project_path, false, None, window, cx)
                    } else {
                        workspace.open_path_preview(project_path, None, true, false, true, window, cx)
                    }
                });
                
                let palette = self.unified_palette.clone();
                cx.spawn_in(window, async move |_, cx| {
                    open_task.await.log_err();
                    log::debug!("UnifiedPalette: File opened, dismissing modal");
                    palette.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                }).detach();
            }
            Match::Command(command_match) => {
                log::info!("UnifiedPalette: Executing command: {}", command_match.name);
                window.dispatch_action(command_match.action.as_ref().boxed_clone(), cx);
                log::debug!("UnifiedPalette: Command dispatched, dismissing modal");
                self.unified_palette.update(cx, |_, cx| cx.emit(DismissEvent)).log_err();
            }
            Match::Line(line_match) => {
                log::info!("UnifiedPalette: Going to line {}", line_match.line_number);
                workspace.update(cx, |workspace, cx| {
                    if let Some(active_item) = workspace.active_item(cx) {
                        if let Some(editor) = active_item.downcast::<editor::Editor>() {
                            editor.update(cx, |editor, cx| {
                                let point = language::Point::new(line_match.line_number.saturating_sub(1), 0);
                                editor.change_selections(
                                    editor::SelectionEffects::default(),
                                    window,
                                    cx,
                                    |s| {
                                        s.select_ranges([point..point]);
                                    },
                                );
                                log::debug!("UnifiedPalette: Selection changed to line {}", line_match.line_number);
                            });
                        } else {
                            log::warn!("UnifiedPalette: Active item is not an editor");
                        }
                    } else {
                        log::warn!("UnifiedPalette: No active item in workspace");
                    }
                });
                log::debug!("UnifiedPalette: Dismissing modal after go-to-line");
                self.unified_palette.update(cx, |_, cx| cx.emit(DismissEvent)).log_err();
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        log::info!("UnifiedPalette: Modal dismissed");
    }

    fn render_match(&self, ix: usize, selected: bool, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> Option<Self::ListItem> {
        let match_item = self.matches.get(ix)?;
        
        let display_text = match match_item {
            Match::File(file_match) => file_match.display_path.clone(),
            Match::Command(command_match) => command_match.name.clone(),
            Match::Line(line_match) => format!("Go to line {}", line_match.line_number),
        };
        
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(display_text))
        )
    }
}

impl EventEmitter<DismissEvent> for UnifiedPalette {}
impl ModalView for UnifiedPalette {}

impl Focusable for UnifiedPalette {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.read(cx).focus_handle(cx)
    }
}

impl Render for UnifiedPalette {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .min_w(rems(34.))
            .child(self.picker.clone())
    }
}

// Helper functions
pub fn detect_mode_from_query(query: &str) -> Option<PaletteMode> {
    if query.starts_with('>') {
        Some(PaletteMode::CommandPalette)
    } else if query.starts_with('#') {
        Some(PaletteMode::ProjectSymbols)
    } else if query.starts_with('@') {
        Some(PaletteMode::Outline)
    } else if query.starts_with(':') {
        Some(PaletteMode::GoToLine)
    } else {
        None
    }
}

pub fn is_mode_available(mode: PaletteMode, has_active_editor: bool) -> bool {
    match mode {
        PaletteMode::Outline | PaletteMode::GoToLine => has_active_editor,
        _ => true,
    }
}
