use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::Context;
use collections::{HashMap, HashSet};
use editor::EditorSettings;
use gpui::{Entity, FocusHandle, WeakEntity};
use project::search_history::SearchHistoryCursor;
use project::{Project, ProjectPath};
use settings::Settings;
use ui::{ContextMenu, PopoverMenuHandle};
use workspace::Workspace;

use super::SearchMatch;
use crate::SearchOptions;
use crate::project_search_picker::TextPicker;

mod impl_delegate;

pub struct TextPickerDelegate {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) workspace: WeakEntity<Workspace>,
    pub(crate) project: Entity<Project>,
    pub(crate) matches: Vec<SearchMatch>,
    pub(crate) selected_index: usize,
    pub(crate) cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    pub(crate) last_selection_change_time: Option<std::time::Instant>,
    pub(crate) last_click: Option<(usize, std::time::Instant)>,
    pub(crate) search_options: SearchOptions,
    pub(crate) search_in_progress: bool,
    pub(crate) pending_initial_query: RefCell<Option<String>>,
    pub(crate) panels_with_errors: HashMap<InputPanel, String>,
    pub(crate) split_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    pub(crate) history_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    pub(crate) search_history_cursor: SearchHistoryCursor,
    pub(crate) file_count: usize,
    pub(crate) unique_files: HashSet<ProjectPath>,
}

impl TextPickerDelegate {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        initial_query: Option<String>,
        cx: &mut ui::Context<TextPicker>,
    ) -> Self {
        Self {
            workspace,
            project,
            matches: Vec::new(),
            selected_index: 0,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            last_selection_change_time: None,
            last_click: None,
            search_options: SearchOptions::from_settings(&EditorSettings::get_global(cx).search),
            search_in_progress: false,
            pending_initial_query: RefCell::new(initial_query),
            panels_with_errors: HashMap::default(),
            split_popover_menu_handle: PopoverMenuHandle::default(),
            history_popover_menu_handle: PopoverMenuHandle::default(),
            search_history_cursor: SearchHistoryCursor::default(),
            file_count: 0,
            unique_files: HashSet::default(),
            focus_handle: cx.focus_handle(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum InputPanel {
    Query,
    Include,
    Exclude,
}
