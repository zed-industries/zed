mod unified_palette_tests;

use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task,
    Window,
};
use gpui_util::ResultExt;
use picker::{Picker, PickerDelegate, QueryMode};
use settings::{Settings, UnifiedPaletteSettings};
use std::sync::Arc;
use ui::{ListItem, prelude::*};
use workspace::{ModalView, Workspace};
use zed_actions::unified_palette::{SwitchModeFromFileFinder, ToggleFileFinderMode};

pub fn init(cx: &mut App) {
    UnifiedPaletteSettings::register(cx);
    cx.observe_new(register).detach();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    CommandPalette,
    ProjectSymbols,
    Outline,
    GoToLine,
    TextFinder,
    Help,
}

fn register(workspace: &mut Workspace, _window: Option<&mut Window>, _cx: &mut Context<Workspace>) {
    workspace
        .register_action(handle_file_finder_toggle)
        .register_action(|workspace, _: &SwitchModeFromFileFinder, window, cx| {
            handle_file_finder_mode_switch(workspace, window, cx);
        });
}

fn handle_file_finder_toggle(
    workspace: &mut Workspace,
    action: &ToggleFileFinderMode,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if !UnifiedPaletteSettings::get(None, cx).enabled {
        return;
    }

    if let Some(file_finder) = workspace.active_modal::<file_finder::FileFinder>(cx) {
        cx.stop_propagation();
        file_finder.update(cx, |file_finder, cx| {
            file_finder.cycle_selection(window, cx);
        });
        return;
    }

    let query = workspace
        .active_modal::<command_palette::CommandPalette>(cx)
        .map(|command_palette| command_palette.read(cx).query(cx))
        .or_else(|| {
            workspace
                .active_modal::<outline::OutlineView>(cx)
                .map(|outline| outline.read(cx).query(cx))
        })
        .or_else(|| {
            workspace
                .active_modal::<go_to_line::GoToLine>(cx)
                .map(|go_to_line| go_to_line.read(cx).query(cx))
        })
        .or_else(|| {
            workspace
                .active_modal::<search::text_finder::TextFinder>(cx)
                .map(|text_finder| text_finder.read(cx).query(cx))
        })
        .or_else(|| {
            workspace
                .active_modal::<Picker<project_symbols::ProjectSymbolsDelegate>>(cx)
                .map(|picker| picker.read(cx).query_without_mode_prefix(cx))
        });
    cx.stop_propagation();
    file_finder::FileFinder::open_with_options(
        workspace,
        action.separate_history,
        action.include_ignored,
        query,
        window,
        cx,
    )
    .detach();
}

fn handle_file_finder_mode_switch(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if !UnifiedPaletteSettings::get(None, cx).enabled {
        return;
    }

    let Some(file_finder) = workspace.active_modal::<file_finder::FileFinder>(cx) else {
        return;
    };
    let query = file_finder.read(cx).query(cx);
    let Some((mode, query)) = mode_from_query(&query) else {
        return;
    };

    if mode == Mode::Help {
        let workspace_handle = cx.entity().downgrade();
        workspace.toggle_modal(window, cx, |window, cx| {
            ModeHelp::new(workspace_handle, window, cx)
        });
        cx.stop_propagation();
        return;
    }

    // Let the file finder handle the prefix when this mode cannot open.
    if open_mode(workspace, mode, query.to_owned(), window, cx) {
        cx.stop_propagation();
    }
}

fn open_mode(
    workspace: &mut Workspace,
    mode: Mode,
    query: String,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> bool {
    match mode {
        Mode::CommandPalette => {
            command_palette::CommandPalette::open_in_unified_mode(workspace, &query, window, cx);
            true
        }
        Mode::ProjectSymbols => {
            project_symbols::open_in_unified_mode(workspace, query, window, cx);
            true
        }
        Mode::Outline => {
            let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<editor::Editor>())
            else {
                return false;
            };
            outline::open_in_unified_mode(editor, query, window, cx)
        }
        Mode::GoToLine => go_to_line::GoToLine::open_in_unified_mode(workspace, query, window, cx),
        Mode::TextFinder => {
            search::text_finder::TextFinder::open_in_unified_mode(query, window, cx).detach();
            true
        }
        Mode::Help => false,
    }
}

fn mode_from_query(query: &str) -> Option<(Mode, &str)> {
    [
        (Mode::CommandPalette, QueryMode::COMMAND_PALETTE),
        (Mode::ProjectSymbols, QueryMode::PROJECT_SYMBOLS),
        (Mode::Outline, QueryMode::OUTLINE),
        (Mode::GoToLine, QueryMode::GO_TO_LINE),
        (Mode::TextFinder, QueryMode::TEXT_FINDER),
    ]
    .into_iter()
    .find_map(|(mode, query_mode)| query_mode.strip_prefix(query).map(|query| (mode, query)))
    .or_else(|| query.strip_prefix('?').map(|query| (Mode::Help, query)))
}

struct ModeHelp {
    picker: Entity<Picker<ModeHelpDelegate>>,
    _subscription: Subscription,
}

impl ModeHelp {
    fn new(
        workspace: gpui::WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| {
            let mut picker = Picker::uniform_list(
                ModeHelpDelegate {
                    workspace,
                    selected_index: 0,
                },
                window,
                cx,
            );
            picker.set_query_mode(QueryMode::Prefixed('?'), "", window, cx);
            picker
        });
        let subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });
        Self {
            picker,
            _subscription: subscription,
        }
    }
}

impl Render for ModeHelp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

impl Focusable for ModeHelp {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for ModeHelp {}
impl ModalView for ModeHelp {}

struct ModeHelpDelegate {
    workspace: gpui::WeakEntity<Workspace>,
    selected_index: usize,
}

const HELP_MODES: [(char, &str, Mode); 6] = [
    (' ', "Find files", Mode::Help),
    ('>', "Run commands", Mode::CommandPalette),
    ('#', "Search project symbols", Mode::ProjectSymbols),
    ('@', "Browse symbols in the current file", Mode::Outline),
    (':', "Go to a line", Mode::GoToLine),
    ('%', "Search text in files", Mode::TextFinder),
];

impl PickerDelegate for ModeHelpDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "unified palette help"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search modes".into()
    }

    fn match_count(&self) -> usize {
        HELP_MODES.len()
    }
    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = index;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        // Keep the help prefix fixed without preventing the user from deleting it.
        if !query.is_empty() {
            cx.defer_in(window, |picker, window, cx| {
                picker.set_query("?", window, cx);
            });
        }
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some((prefix, _, mode)) = HELP_MODES.get(self.selected_index).copied() else {
            return;
        };
        let query = if prefix == ' ' {
            String::new()
        } else {
            prefix.to_string()
        };
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            workspace
                .update_in(cx, |workspace, window, cx| {
                    if mode == Mode::Help {
                        file_finder::FileFinder::open_with_options(
                            workspace,
                            false,
                            None,
                            Some(query),
                            window,
                            cx,
                        )
                        .detach();
                    } else {
                        open_mode(workspace, mode, query, window, cx);
                    }
                })
                .log_err();
        })
        .detach();
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let (prefix, description, _) = HELP_MODES.get(index)?;
        let prefix = if *prefix == ' ' {
            String::new()
        } else {
            format!("{prefix}  ")
        };
        Some(
            ListItem::new(index)
                .inset(true)
                .toggle_state(selected)
                .child(Label::new(format!("{prefix}{description}"))),
        )
    }
}
