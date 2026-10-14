mod unified_palette_tests;

use gpui::{App, Context, Window};
use picker::{Picker, QueryMode};
use settings::{Settings, UnifiedPaletteSettings};
use workspace::Workspace;
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
}
