mod unified_palette_tests;

use gpui::{App, Context, Window};
use picker::Picker;
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

    cx.stop_propagation();
    open_mode(workspace, mode, query.to_owned(), window, cx);
}

fn open_mode(
    workspace: &mut Workspace,
    mode: Mode,
    query: String,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    match mode {
        Mode::CommandPalette => {
            command_palette::CommandPalette::toggle_in_unified_mode(
                workspace, '>', &query, window, cx,
            );
        }
        Mode::ProjectSymbols => {
            project_symbols::open_in_unified_mode(workspace, '#', query, window, cx);
        }
        Mode::Outline => {
            if let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<editor::Editor>())
            {
                outline::open_in_unified_mode(editor, '@', query, window, cx);
            }
        }
        Mode::GoToLine => {
            go_to_line::GoToLine::open_in_unified_mode(workspace, ':', query, window, cx);
        }
        Mode::TextFinder => {
            search::text_finder::TextFinder::open_in_unified_mode(query, window, cx).detach();
        }
    }
}

fn mode_from_query(query: &str) -> Option<(Mode, &str)> {
    if let Some(query) = query.strip_prefix('>') {
        Some((Mode::CommandPalette, query))
    } else if let Some(query) = query.strip_prefix('#') {
        Some((Mode::ProjectSymbols, query))
    } else if let Some(query) = query.strip_prefix('@') {
        Some((Mode::Outline, query))
    } else if let Some(query) = query.strip_prefix(':') {
        Some((Mode::GoToLine, query))
    } else if let Some(query) = query.strip_prefix('%') {
        Some((Mode::TextFinder, query))
    } else {
        None
    }
}
