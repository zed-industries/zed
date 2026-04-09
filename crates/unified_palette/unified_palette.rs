mod unified_palette_tests;

use std::sync::Arc;

use gpui::{
    AnyElement, App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, Task, Window, WeakEntity, div, prelude::*,
};
use picker::{Picker, PickerDelegate};
use ui::prelude::*;
use workspace::ModalView;

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
    workspace: WeakEntity<workspace::Workspace>,
}

pub struct UnifiedPaletteDelegate {
    mode: PaletteMode,
    workspace: WeakEntity<workspace::Workspace>,
    project: Entity<project::Project>,
    
    // Match data (instead of storing sub-pickers)
    matches: Vec<String>,
    selected_index: usize,
}

impl UnifiedPalette {
    fn register(
        workspace: &mut workspace::Workspace,
        _window: Option<&mut Window>,
        cx: &mut Context<workspace::Workspace>,
    ) {
        workspace.register_action(
            |workspace, _action: &workspace::ToggleFileFinder, window, cx| {
                let project = workspace.project().clone();
                let workspace_handle = cx.entity().downgrade();
                
                workspace.toggle_modal(window, cx, move |window, cx| {
                    let delegate = UnifiedPaletteDelegate::new(workspace_handle.clone(), project, cx);
                    let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
                    
                    UnifiedPalette {
                        picker,
                        workspace: workspace_handle,
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
            let delegate = UnifiedPaletteDelegate::new(workspace_handle.clone(), project, cx);
            let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
            
            Self {
                picker,
                workspace: workspace_handle,
            }
        })
    }
}

impl UnifiedPaletteDelegate {
    fn new(
        workspace: WeakEntity<workspace::Workspace>,
        project: Entity<project::Project>,
        _cx: &mut App,
    ) -> Self {
        Self {
            mode: PaletteMode::FileFinder,
            workspace,
            project,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
    
    fn search_files(&mut self, query: &str, cx: &mut Context<Picker<Self>>) {
        if query.is_empty() {
            self.matches.clear();
            return;
        }
        
        // Simple file search - just get file names from project
        let project = self.project.read(cx);
        let mut files = Vec::new();
        
        for worktree in project.worktrees(cx) {
            let worktree = worktree.read(cx);
            for entry in worktree.files(false, 0) {
                let path_str = format!("{:?}", entry.path).trim_matches('"').to_string();
                if path_str.to_lowercase().contains(&query.to_lowercase()) {
                    files.push(path_str);
                    if files.len() >= 100 {
                        break;
                    }
                }
            }
        }
        
        self.matches = files;
        self.selected_index = 0;
    }
}

impl PickerDelegate for UnifiedPaletteDelegate {
    type ListItem = AnyElement;

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
        match self.mode {
            PaletteMode::FileFinder => "Go to file...".into(),
            PaletteMode::CommandPalette => "Execute a command...".into(),
            PaletteMode::ProjectSymbols => "Go to symbol...".into(),
            PaletteMode::Outline => "Go to symbol in editor...".into(),
            PaletteMode::GoToLine => "Go to line...".into(),
        }
    }

    fn update_matches(&mut self, query: String, _window: &mut Window, cx: &mut Context<Picker<Self>>) -> Task<()> {
        // Detect mode from prefix
        let (new_mode, stripped_query) = if let Some(detected_mode) = detect_mode_from_query(&query) {
            let stripped = query.chars().skip(1).collect::<String>();
            (detected_mode, stripped)
        } else {
            (PaletteMode::FileFinder, query.clone())
        };
        
        // Switch mode if changed
        if new_mode != self.mode {
            self.mode = new_mode;
            self.matches.clear();
            cx.notify();
        }
        
        // Search based on mode
        match self.mode {
            PaletteMode::FileFinder => {
                self.search_files(&stripped_query, cx);
                cx.notify();
            }
            PaletteMode::CommandPalette => {
                // TODO: Search commands
                self.matches.clear();
                cx.notify();
            }
            PaletteMode::ProjectSymbols => {
                // TODO: Search symbols
                self.matches.clear();
                cx.notify();
            }
            _ => {}
        }
        
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        // TODO: Forward to active sub-picker
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        // Cleanup
    }

    fn render_match(&self, ix: usize, selected: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<Self::ListItem> {
        let match_text = self.matches.get(ix)?;
        
        Some(
            div()
                .px_2()
                .py_1()
                .when(selected, |el| el.bg(cx.theme().colors().element_selected))
                .child(match_text.clone())
                .into_any_element()
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
        self.picker.clone()
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
