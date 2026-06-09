use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use collections::HashSet;
use editor::EditorSettings;
use gpui::{Entity, FocusHandle, WeakEntity};
use project::{Project, ProjectPath};
use settings::Settings;
use workspace::Workspace;

use super::SearchMatch;
use crate::SearchOptions;
use super::TextFinder;

mod impl_delegate;

pub struct Delegate {
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
    pub(crate) file_count: usize,
    pub(crate) unique_files: HashSet<ProjectPath>,
    /// Whether the preview is currently shown to the side. Kept in sync by the
    /// picker via [`PickerDelegate::set_horizontal_preview`], because the
    /// delegate cannot read the picker entity while rendering.
    pub(crate) preview_layout_is_horizontal: bool,
}

impl Delegate {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        cx: &mut ui::Context<TextFinder>,
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
            file_count: 0,
            unique_files: HashSet::default(),
            preview_layout_is_horizontal: false,
            focus_handle: cx.focus_handle(),
        }
    }
}
