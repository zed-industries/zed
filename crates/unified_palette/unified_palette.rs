mod unified_palette_tests;

use std::sync::Arc;

use gpui::{
    actions, Action, App, AppContext, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, IntoElement, Render, SharedString, StyledText, 
    Task, TextStyle, Window, WeakEntity, prelude::*, relative,
};
use gpui_util::ResultExt;
use language::{ToPointUtf16, Unclipped};
use lsp;
use picker::{Picker, PickerDelegate};
use project::{ProjectPath, WorktreeId};
use settings::Settings;
use theme::ActiveTheme;
use theme_settings::ThemeSettings;
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
    Symbol(SymbolMatch),
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

#[derive(Clone)]
struct SymbolMatch {
    symbol: project::Symbol,
    highlight_ranges: Vec<(std::ops::Range<usize>, gpui::HighlightStyle)>,
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
    
    fn search_project_symbols(&mut self, query: &str, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if query.is_empty() {
            self.matches.clear();
            self.selected_index = 0;
            return;
        }

        let project = self.project.clone();
        let query_string = query.to_string();
        
        let symbols_task = project.update(cx, |project, cx| {
            project.symbols(&query_string, cx)
        });
        
        cx.spawn_in(window, async move |picker, cx| {
            if let Ok(symbols) = symbols_task.await {
                picker.update_in(cx, |picker, _window, cx| {
                    let delegate = &mut picker.delegate;
                    
                    // Convert symbols to matches (limit to 100)
                    delegate.matches = symbols
                        .into_iter()
                        .take(100)
                        .map(|symbol| Match::Symbol(SymbolMatch { 
                            symbol,
                            highlight_ranges: Vec::new(), // Project symbols don't have highlights
                        }))
                        .collect();
                    
                    delegate.selected_index = 0;
                    cx.notify();
                }).ok();
            }
        }).detach();
    }
    
    fn search_outline(&mut self, query: &str, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        // Get active editor from workspace
        let Some(workspace) = self.workspace.upgrade() else {
            self.matches.clear();
            self.selected_index = 0;
            return;
        };
        
        let editor = workspace.read(cx).active_item(cx)
            .and_then(|item| item.downcast::<editor::Editor>());
        
        let Some(editor) = editor else {
            log::warn!("UnifiedPalette: No active editor for outline mode");
            self.matches.clear();
            self.selected_index = 0;
            return;
        };
        
        // Get buffer and outline items
        let multibuffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        let Some(buffer_snapshot) = multibuffer.as_singleton() else {
            log::warn!("UnifiedPalette: Active editor has multiple buffers");
            self.matches.clear();
            self.selected_index = 0;
            return;
        };
        
        let buffer_id = buffer_snapshot.remote_id();
        let file_path = buffer_snapshot.file().map(|f| f.path().clone());
        let outline_task = editor.update(cx, |editor, cx| {
            editor.buffer_outline_items(buffer_id, cx)
        });
        
        let query_lower = query.to_lowercase();
        let project = self.project.clone();
        
        cx.spawn_in(window, async move |picker, cx| {
            let items = outline_task.await;
            
            picker.update_in(cx, |picker, _window, cx| {
                let delegate = &mut picker.delegate;
                let buffer_snapshot = multibuffer.as_singleton();
                
                // Filter items by query and convert to Symbol matches
                delegate.matches = items
                    .into_iter()
                    .filter(|item| {
                        query_lower.is_empty() || item.text.to_lowercase().contains(&query_lower)
                    })
                    .take(100)
                    .filter_map(|item| {
                        let buffer_snapshot = buffer_snapshot.as_ref()?;
                        let file_path = file_path.as_ref()?;
                        
                        // Convert anchor range to PointUtf16
                        let start_point = item.range.start.to_point_utf16(buffer_snapshot);
                        let end_point = item.range.end.to_point_utf16(buffer_snapshot);
                        
                        // Get worktree_id from project
                        let worktree_id = project.read(cx)
                            .worktrees(cx)
                            .next()?
                            .read(cx)
                            .id();
                        
                        // Create a Symbol from the outline item
                        let symbol = project::Symbol {
                            language_server_name: language::LanguageServerName(SharedString::from("outline")),
                            source_worktree_id: worktree_id,
                            source_language_server_id: language::LanguageServerId(0),
                            path: project::lsp_store::SymbolLocation::InProject(ProjectPath {
                                worktree_id,
                                path: file_path.clone(),
                            }),
                            label: language::CodeLabel {
                                text: item.text.clone(),
                                runs: Vec::new(),
                                filter_range: 0..item.text.len(),
                            },
                            name: item.text.clone(),
                            kind: lsp::SymbolKind::FUNCTION,
                            range: Unclipped(start_point)..Unclipped(end_point),
                            container_name: None,
                        };
                        Some(Match::Symbol(SymbolMatch { 
                            symbol,
                            highlight_ranges: item.highlight_ranges.clone(), // Preserve syntax highlighting
                        }))
                    })
                    .collect();
                
                delegate.selected_index = 0;
            }).ok();
        }).detach();
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

    fn set_selected_index(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        
        // For outline mode, navigate to the symbol as you move through the list
        if self.mode == PaletteMode::Outline {
            if let Some(Match::Symbol(symbol_match)) = self.matches.get(ix) {
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        if let Some(active_item) = workspace.active_item(cx) {
                            if let Some(editor) = active_item.downcast::<editor::Editor>() {
                                let symbol = &symbol_match.symbol;
                                let position = symbol.range.start.0;
                                
                                editor.update(cx, |editor, cx| {
                                    // Move selection to the symbol with autoscroll
                                    editor.change_selections(
                                        editor::SelectionEffects::scroll(editor::scroll::Autoscroll::center()),
                                        window,
                                        cx,
                                        |s| s.select_ranges([position..position]),
                                    );
                                });
                            }
                        }
                    });
                }
            }
        }
        
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
                self.search_outline(&stripped_query, window, cx);
                log::debug!("UnifiedPalette: Found {} outline matches", self.matches.len());
            }
        }
        
        cx.notify();
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        log::info!("UnifiedPalette: Confirm called in {:?} mode (secondary: {})", self.mode, secondary);
        
        // Don't confirm if there are no matches
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
            Match::Symbol(symbol_match) => {
                log::info!("UnifiedPalette: Navigating to symbol: {}", symbol_match.symbol.label.text);
                
                // Check if this is an outline symbol (from current file)
                if self.mode == PaletteMode::Outline {
                    // For outline mode, navigate within the current editor
                    workspace.update(cx, |workspace, cx| {
                        if let Some(active_item) = workspace.active_item(cx) {
                            if let Some(editor) = active_item.downcast::<editor::Editor>() {
                                let symbol = &symbol_match.symbol;
                                let position = symbol.range.start.0; // Unwrap Unclipped
                                
                                editor.update(cx, |editor, cx| {
                                    editor.change_selections(
                                        editor::SelectionEffects::scroll(editor::scroll::Autoscroll::center()),
                                        window,
                                        cx,
                                        |s| s.select_ranges([position..position]),
                                    );
                                });
                            }
                        }
                    });
                    self.unified_palette.update(cx, |_, cx| cx.emit(DismissEvent)).log_err();
                } else {
                    // For project symbols, open the buffer and navigate
                    let symbol = symbol_match.symbol.clone();
                    let buffer = self.project.update(cx, |project, cx| {
                        project.open_buffer_for_symbol(&symbol, cx)
                    });
                    
                    let workspace = self.workspace.clone();
                    let palette = self.unified_palette.clone();
                    
                    cx.spawn_in(window, async move |_, cx| {
                        let buffer = buffer.await.log_err()?;
                        workspace.update_in(cx, |workspace, window, cx| {
                            let position = buffer
                                .read(cx)
                                .clip_point_utf16(symbol.range.start, editor::Bias::Left);
                            
                            let pane = if secondary {
                                workspace.adjacent_pane(window, cx)
                            } else {
                                workspace.active_pane().clone()
                            };
                            
                            let editor = workspace.open_project_item::<editor::Editor>(
                                pane, buffer, true, true, true, true, window, cx,
                            );
                            
                            editor.update(cx, |editor, cx| {
                                let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                                let Some(buffer_snapshot) = multibuffer_snapshot.as_singleton() else {
                                    return;
                                };
                                let text_anchor = buffer_snapshot.anchor_before(position);
                                let Some(anchor) = multibuffer_snapshot.anchor_in_buffer(text_anchor) else {
                                    return;
                                };
                                editor.change_selections(
                                    editor::SelectionEffects::scroll(editor::scroll::Autoscroll::center()),
                                    window,
                                    cx,
                                    |s| s.select_ranges([anchor..anchor]),
                                );
                            });
                        }).log_err();
                        
                        palette.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
                        Some(())
                    }).detach();
                }
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        log::info!("UnifiedPalette: Modal dismissed");
    }

    fn render_match(&self, ix: usize, selected: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<Self::ListItem> {
        let match_item = self.matches.get(ix)?;
        
        match match_item {
            Match::File(file_match) => {
                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(Label::new(file_match.display_path.clone()))
                )
            }
            Match::Command(command_match) => {
                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(Label::new(command_match.name.clone()))
                )
            }
            Match::Line(line_match) => {
                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(Label::new(format!("Go to line {}", line_match.line_number)))
                )
            }
            Match::Symbol(symbol_match) => {
                let symbol = &symbol_match.symbol;
                
                // For outline mode, show simple single-line format with syntax highlighting
                if self.mode == PaletteMode::Outline {
                    let settings = ThemeSettings::get_global(cx);
                    let text_style = TextStyle {
                        color: cx.theme().colors().text,
                        font_family: settings.buffer_font.family.clone(),
                        font_features: settings.buffer_font.features.clone(),
                        font_fallbacks: settings.buffer_font.fallbacks.clone(),
                        font_size: settings.buffer_font_size(cx).into(),
                        font_weight: settings.buffer_font.weight,
                        line_height: relative(1.),
                        ..Default::default()
                    };
                    
                    Some(
                        ListItem::new(ix)
                            .inset(true)
                            .spacing(ListItemSpacing::Sparse)
                            .toggle_state(selected)
                            .child(
                                StyledText::new(symbol.label.text.clone())
                                    .with_default_highlights(&text_style, symbol_match.highlight_ranges.iter().cloned())
                            )
                    )
                } else {
                    // For project symbols, show two-line format with path and line number
                    let path = match &symbol.path {
                        project::lsp_store::SymbolLocation::InProject(path) => {
                            format!("{:?}", path.path).trim_matches('"').to_string()
                        }
                        project::lsp_store::SymbolLocation::OutsideProject { abs_path, .. } => {
                            abs_path.display().to_string()
                        }
                    };
                    let line_number = symbol.range.start.0.row + 1;
                    
                    Some(
                        ListItem::new(ix)
                            .inset(true)
                            .spacing(ListItemSpacing::Sparse)
                            .toggle_state(selected)
                            .child(
                                v_flex()
                                    .child(Label::new(symbol.label.text.clone()))
                                    .child(
                                        h_flex()
                                            .child(Label::new(path).size(LabelSize::Small).color(Color::Muted))
                                            .child(
                                                Label::new(format!(":{}", line_number))
                                                    .size(LabelSize::Small)
                                                    .color(Color::Placeholder)
                                            )
                                    )
                            )
                    )
                }
            }
        }
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
