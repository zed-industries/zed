mod unified_palette_tests;

use std::sync::Arc;

use gpui::{
    AnyElement, App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, Task, Window, WeakEntity,
};
use picker::{Picker, PickerDelegate};
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
    
    // Lazy-loaded sub-pickers (each with their own delegate)
    file_finder: Option<Entity<file_finder::FileFinder>>,
    command_palette: Option<Entity<command_palette::CommandPalette>>,
    project_symbols: Option<project_symbols::ProjectSymbols>,
}

impl UnifiedPalette {
    pub fn new(
        workspace: &mut workspace::Workspace,
        window: &mut Window,
        cx: &mut Context<workspace::Workspace>,
    ) -> Entity<Self> {
        let workspace_handle = cx.entity().downgrade();
        
        cx.new(|cx| {
            let delegate = UnifiedPaletteDelegate::new(workspace_handle.clone(), workspace, window, cx);
            let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
            
            Self {
                picker,
                workspace: workspace_handle,
            }
        })
    }
    
    fn switch_mode(&mut self, mode: PaletteMode, window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.switch_mode(mode, window, cx);
        });
    }
}

impl UnifiedPaletteDelegate {
    fn new(
        workspace: WeakEntity<workspace::Workspace>,
        _workspace_entity: &mut workspace::Workspace,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self {
            mode: PaletteMode::FileFinder,
            workspace,
            file_finder: None,
            command_palette: None,
            project_symbols: None,
        }
    }
    
    fn switch_mode(&mut self, mode: PaletteMode, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        self.mode = mode;
        // TODO: Initialize sub-picker if needed
    }
}

impl PickerDelegate for UnifiedPaletteDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        // TODO: Delegate to active sub-picker
        0
    }

    fn selected_index(&self) -> usize {
        // TODO: Delegate to active sub-picker
        0
    }

    fn set_selected_index(&mut self, _ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        // TODO: Delegate to active sub-picker
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

    fn update_matches(&mut self, _query: String, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> Task<()> {
        // TODO: Delegate to active sub-picker
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        // TODO: Delegate to active sub-picker
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {
        // TODO: Cleanup
    }

    fn render_match(&self, _ix: usize, _selected: bool, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> Option<Self::ListItem> {
        // TODO: Delegate to active sub-picker
        None
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
