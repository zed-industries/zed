mod unified_palette_tests;

use std::sync::Arc;

use gpui::{
    AnyElement, App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, Task, Window, WeakEntity, div, prelude::*,
};
use picker::{Picker, PickerDelegate};
use ui::prelude::*;
use workspace::ModalView;

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
    
    // Sub-pickers (lazy-loaded)
    file_finder: Option<Entity<file_finder::FileFinder>>,
    command_palette: Option<Entity<command_palette::CommandPalette>>,
    project_symbols: Option<project_symbols::ProjectSymbols>,
    
    selected_index: usize,
}

impl UnifiedPalette {
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
            file_finder: None,
            command_palette: None,
            project_symbols: None,
            selected_index: 0,
        }
    }
    
    fn ensure_file_finder(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.file_finder.is_some() {
            return;
        }
        
        // TODO: Create FileFinder
        // Problem: FileFinder::new() is private, need to use workspace.toggle_modal
        // For now, leave as None
    }
}

impl PickerDelegate for UnifiedPaletteDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        match self.mode {
            PaletteMode::FileFinder => {
                // TODO: Read from file_finder
                0
            }
            PaletteMode::CommandPalette => {
                // TODO: Read from command_palette
                0
            }
            PaletteMode::ProjectSymbols => {
                // TODO: Read from project_symbols
                0
            }
            _ => 0,
        }
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

    fn update_matches(&mut self, query: String, window: &mut Window, cx: &mut Context<Picker<Self>>) -> Task<()> {
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
            cx.notify();
        }
        
        // Ensure picker exists for current mode
        match self.mode {
            PaletteMode::FileFinder => {
                self.ensure_file_finder(window, cx);
                // TODO: Forward query to file_finder
            }
            PaletteMode::CommandPalette => {
                // TODO: Ensure and forward to command_palette
            }
            PaletteMode::ProjectSymbols => {
                // TODO: Ensure and forward to project_symbols
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

    fn render_match(&self, _ix: usize, selected: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<Self::ListItem> {
        // TODO: Forward to active sub-picker
        Some(
            div()
                .px_2()
                .py_1()
                .when(selected, |el| el.bg(cx.theme().colors().element_selected))
                .child("TODO: Render from sub-picker")
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
