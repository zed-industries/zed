mod unified_palette_tests;

use gpui::{App, Context, Window};
use picker::Picker;
use workspace::Workspace;
use zed_actions::{
    outline::ToggleOutline,
    text_finder::Toggle as ToggleTextFinder,
    unified_palette::{SwitchModeFromFileFinder, ToggleCommandMode, ToggleProjectSymbolsMode},
};

pub fn init(cx: &mut App) {
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
        })
        .register_action(|workspace, _: &ToggleCommandMode, window, cx| {
            handle_mode_toggle(workspace, Mode::CommandPalette, window, cx);
        })
        .register_action(|workspace, _: &ToggleProjectSymbolsMode, window, cx| {
            handle_mode_toggle(workspace, Mode::ProjectSymbols, window, cx);
        })
        .register_action(|workspace, _: &ToggleOutline, window, cx| {
            handle_mode_toggle(workspace, Mode::Outline, window, cx);
        })
        .register_action(
            |workspace, _: &editor::actions::ToggleGoToLine, window, cx| {
                handle_mode_toggle(workspace, Mode::GoToLine, window, cx);
            },
        )
        .register_action(|workspace, _: &ToggleTextFinder, window, cx| {
            handle_mode_toggle(workspace, Mode::TextFinder, window, cx);
        });
}

fn handle_file_finder_mode_switch(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let Some(file_finder) = workspace.active_modal::<file_finder::FileFinder>(cx) else {
        return;
    };
    let query = file_finder.read(cx).query(cx);
    let Some((mode, query)) = mode_from_query(&query) else {
        return;
    };

    cx.stop_propagation();
    open_mode(workspace, mode, query.to_owned(), true, window, cx);
}

fn handle_file_finder_toggle(
    workspace: &mut Workspace,
    action: &workspace::ToggleFileFinder,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
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

fn handle_mode_toggle(
    workspace: &mut Workspace,
    mode: Mode,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if let Some(file_finder) = workspace.active_modal::<file_finder::FileFinder>(cx) {
        let query = file_finder.read(cx).query(cx);
        cx.stop_propagation();
        open_mode(
            workspace,
            mode,
            strip_mode_prefix(&query).to_owned(),
            true,
            window,
            cx,
        );
        return;
    }

    if let Some(command_palette) = workspace.active_modal::<command_palette::CommandPalette>(cx) {
        if mode == Mode::CommandPalette {
            cx.stop_propagation();
            command_palette::CommandPalette::toggle(workspace, "", window, cx);
            return;
        }
        let query = command_palette.read(cx).query(cx);
        let unified_mode = command_palette.read(cx).is_unified_mode(cx);
        cx.stop_propagation();
        open_mode(workspace, mode, query, unified_mode, window, cx);
        return;
    }

    if let Some(project_symbols) =
        workspace.active_modal::<Picker<project_symbols::ProjectSymbolsDelegate>>(cx)
    {
        if mode == Mode::ProjectSymbols {
            cx.stop_propagation();
            workspace.hide_modal(window, cx);
            return;
        }
        let picker = project_symbols.read(cx);
        let query = picker.query_without_mode_prefix(cx);
        let unified_mode = picker.has_query_mode_prefix();
        cx.stop_propagation();
        open_mode(workspace, mode, query, unified_mode, window, cx);
        return;
    }

    if let Some(text_finder) = workspace.active_modal::<search::text_finder::TextFinder>(cx) {
        if mode == Mode::TextFinder {
            cx.stop_propagation();
            text_finder.update(cx, |text_finder, cx| {
                text_finder.cycle_selection(window, cx);
            });
            return;
        }
        let text_finder = text_finder.read(cx);
        let query = text_finder.query(cx);
        let unified_mode = text_finder.is_unified_mode(cx);
        cx.stop_propagation();
        open_mode(workspace, mode, query, unified_mode, window, cx);
        return;
    }

    if let Some(outline) = workspace.active_modal::<outline::OutlineView>(cx) {
        if mode == Mode::Outline {
            return;
        }
        let outline = outline.read(cx);
        let query = outline.query(cx);
        let unified_mode = outline.is_unified_mode(cx);
        cx.stop_propagation();
        open_mode(workspace, mode, query, unified_mode, window, cx);
        return;
    }

    if let Some(go_to_line) = workspace.active_modal::<go_to_line::GoToLine>(cx) {
        if mode == Mode::GoToLine {
            return;
        }
        let go_to_line = go_to_line.read(cx);
        let query = go_to_line.query(cx);
        let unified_mode = go_to_line.is_unified_mode();
        cx.stop_propagation();
        open_mode(workspace, mode, query, unified_mode, window, cx);
        return;
    }

    match mode {
        Mode::CommandPalette => {
            cx.stop_propagation();
            command_palette::CommandPalette::toggle(workspace, "", window, cx);
        }
        Mode::ProjectSymbols => {
            cx.stop_propagation();
            project_symbols::open_with_query(workspace, String::new(), window, cx);
        }
        Mode::TextFinder => {
            cx.stop_propagation();
            search::text_finder::TextFinder::open_from_workspace(workspace, window, cx).detach();
        }
        Mode::Outline | Mode::GoToLine => {}
    }
}

fn open_mode(
    workspace: &mut Workspace,
    mode: Mode,
    query: String,
    unified_mode: bool,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    match (mode, unified_mode) {
        (Mode::CommandPalette, true) => {
            command_palette::CommandPalette::toggle_in_unified_mode(
                workspace, '>', &query, window, cx,
            );
        }
        (Mode::CommandPalette, false) => {
            command_palette::CommandPalette::toggle(workspace, &query, window, cx);
        }
        (Mode::ProjectSymbols, true) => {
            project_symbols::open_in_unified_mode(workspace, '#', query, window, cx);
        }
        (Mode::ProjectSymbols, false) => {
            project_symbols::open_with_query(workspace, query, window, cx);
        }
        (Mode::Outline, unified_mode) => {
            if let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.downcast::<editor::Editor>())
            {
                if unified_mode {
                    outline::open_in_unified_mode(editor, '@', query, window, cx);
                } else {
                    outline::open_with_query(editor, query, window, cx);
                }
            }
        }
        (Mode::GoToLine, true) => {
            go_to_line::GoToLine::open_in_unified_mode(workspace, ':', query, window, cx);
        }
        (Mode::GoToLine, false) => {
            go_to_line::GoToLine::open_with_query(workspace, query, window, cx);
        }
        (Mode::TextFinder, true) => {
            search::text_finder::TextFinder::open_in_unified_mode(query, window, cx).detach();
        }
        (Mode::TextFinder, false) => {
            search::text_finder::TextFinder::open_with_query(query, window, cx).detach();
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

fn strip_mode_prefix(query: &str) -> &str {
    mode_from_query(query).map_or(query, |(_, query)| query)
}
