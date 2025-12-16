mod persistence;

use std::{
    cmp::{self, Reverse},
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use client::parse_zed_link;
use command_palette_hooks::{
    CommandInterceptItem, CommandInterceptResult, CommandPaletteFilter,
    GlobalCommandPaletteInterceptor,
};

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    ParentElement, Render, Styled, Task, WeakEntity, Window,
};
use persistence::COMMAND_PALETTE_HISTORY;
use picker::Direction;
use picker::{Picker, PickerDelegate};
use postage::{sink::Sink, stream::Stream};
use settings::Settings;
use ui::{HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace, WorkspaceSettings};
use zed_actions::{OpenZedUrl, command_palette::Toggle};

pub fn init(cx: &mut App) {
    command_palette_hooks::init(cx);
    cx.observe_new(CommandPalette::register).detach();
}

impl ModalView for CommandPalette {}

pub struct CommandPalette {
    picker: Entity<Picker<CommandPaletteDelegate>>,
}

/// Removes subsequent whitespace characters and double colons from the query.
///
/// This improves the likelihood of a match by either humanized name or keymap-style name.
pub fn normalize_action_query(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut last_char = None;

    for char in input.trim().chars() {
        match (last_char, char) {
            (Some(':'), ':') => continue,
            (Some(last_char), char) if last_char.is_whitespace() && char.is_whitespace() => {
                continue;
            }
            _ => {
                last_char = Some(char);
            }
        }
        result.push(char);
    }

    result
}

impl CommandPalette {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            Self::toggle(workspace, "", window, cx)
        });
    }

    pub fn toggle(
        workspace: &mut Workspace,
        query: &str,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(previous_focus_handle) = window.focused(cx) else {
            return;
        };

        let entity = cx.weak_entity();
        workspace.toggle_modal(window, cx, move |window, cx| {
            CommandPalette::new(previous_focus_handle, query, entity, window, cx)
        });
    }

    fn new(
        previous_focus_handle: FocusHandle,
        query: &str,
        entity: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let filter = CommandPaletteFilter::try_global(cx);

        let commands = window
            .available_actions(cx)
            .into_iter()
            .filter_map(|action| {
                if filter.is_some_and(|filter| filter.is_hidden(&*action)) {
                    return None;
                }

                Some(Command {
                    name: humanize_action_name(action.name()),
                    action,
                })
            })
            .collect();

        let delegate = CommandPaletteDelegate::new(
            cx.entity().downgrade(),
            entity,
            commands,
            previous_focus_handle,
        );

        let picker = cx.new(|cx| {
            let picker = Picker::uniform_list(delegate, window, cx);
            picker.set_query(query, window, cx);
            picker
        });
        Self { picker }
    }

    pub fn set_query(&mut self, query: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.picker
            .update(cx, |picker, cx| picker.set_query(query, window, cx))
    }
}

impl EventEmitter<DismissEvent> for CommandPalette {}

impl Focusable for CommandPalette {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("CommandPalette")
            .w(rems(34.))
            .child(self.picker.clone())
    }
}

pub struct CommandPaletteDelegate {
    latest_query: String,
    command_palette: WeakEntity<CommandPalette>,
    workspace: WeakEntity<Workspace>,
    all_commands: Vec<Command>,
    commands: Vec<Command>,
    matches: Vec<StringMatch>,
    selected_ix: usize,
    previous_focus_handle: FocusHandle,
    updating_matches: Option<(
        Task<()>,
        postage::dispatch::Receiver<(Vec<Command>, Vec<StringMatch>, CommandInterceptResult)>,
    )>,
    query_history: QueryHistory,
}

struct Command {
    name: String,
    action: Box<dyn Action>,
}

#[derive(Default)]
struct QueryHistory {
    history: Option<VecDeque<String>>,
    cursor: Option<usize>,
    prefix: Option<String>,
}

impl QueryHistory {
    fn history(&mut self) -> &mut VecDeque<String> {
        self.history.get_or_insert_with(|| {
            COMMAND_PALETTE_HISTORY
                .list_recent_queries()
                .unwrap_or_default()
                .into_iter()
                .collect()
        })
    }

    fn add(&mut self, query: String) {
        if let Some(pos) = self.history().iter().position(|h| h == &query) {
            self.history().remove(pos);
        }
        self.history().push_back(query);
        self.cursor = None;
        self.prefix = None;
    }

    fn validate_cursor(&mut self, current_query: &str) -> Option<usize> {
        if let Some(pos) = self.cursor {
            if self.history().get(pos).map(|s| s.as_str()) != Some(current_query) {
                self.cursor = None;
                self.prefix = None;
            }
        }
        self.cursor
    }

    fn previous(&mut self, current_query: &str) -> Option<&str> {
        if self.validate_cursor(current_query).is_none() {
            self.prefix = Some(current_query.to_string());
        }

        let prefix = self.prefix.clone().unwrap_or_default();
        let start_index = self.cursor.unwrap_or(self.history().len());

        for i in (0..start_index).rev() {
            if self
                .history()
                .get(i)
                .is_some_and(|e| e.starts_with(&prefix))
            {
                self.cursor = Some(i);
                return self.history().get(i).map(|s| s.as_str());
            }
        }
        None
    }

    fn next(&mut self, current_query: &str) -> Option<&str> {
        let selected = self.validate_cursor(current_query)?;
        let prefix = self.prefix.clone().unwrap_or_default();

        for i in (selected + 1)..self.history().len() {
            if self
                .history()
                .get(i)
                .is_some_and(|e| e.starts_with(&prefix))
            {
                self.cursor = Some(i);
                return self.history().get(i).map(|s| s.as_str());
            }
        }
        None
    }

    fn reset_cursor(&mut self) {
        self.cursor = None;
        self.prefix = None;
    }

    fn is_navigating(&self) -> bool {
        self.cursor.is_some()
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            action: self.action.boxed_clone(),
        }
    }
}

impl CommandPaletteDelegate {
    fn new(
        command_palette: WeakEntity<CommandPalette>,
        workspace: WeakEntity<Workspace>,
        commands: Vec<Command>,
        previous_focus_handle: FocusHandle,
    ) -> Self {
        Self {
            command_palette,
            workspace,
            all_commands: commands.clone(),
            matches: vec![],
            commands,
            selected_ix: 0,
            previous_focus_handle,
            latest_query: String::new(),
            updating_matches: None,
            query_history: Default::default(),
        }
    }

    fn matches_updated(
        &mut self,
        query: String,
        mut commands: Vec<Command>,
        mut matches: Vec<StringMatch>,
        intercept_result: CommandInterceptResult,
        _: &mut Context<Picker<Self>>,
    ) {
        self.updating_matches.take();
        self.latest_query = query;

        let mut new_matches = Vec::new();

        for CommandInterceptItem {
            action,
            string,
            positions,
        } in intercept_result.results
        {
            if let Some(idx) = matches
                .iter()
                .position(|m| commands[m.candidate_id].action.partial_eq(&*action))
            {
                matches.remove(idx);
            }
            commands.push(Command {
                name: string.clone(),
                action,
            });
            new_matches.push(StringMatch {
                candidate_id: commands.len() - 1,
                string,
                positions,
                score: 0.0,
            })
        }
        if !intercept_result.exclusive {
            new_matches.append(&mut matches);
        }
        self.commands = commands;
        self.matches = new_matches;
        if self.matches.is_empty() {
            self.selected_ix = 0;
        } else {
            self.selected_ix = cmp::min(self.selected_ix, self.matches.len() - 1);
        }
    }

    /// Hit count for each command in the palette.
    /// We only account for commands triggered directly via command palette and not by e.g. keystrokes because
    /// if a user already knows a keystroke for a command, they are unlikely to use a command palette to look for it.
    fn hit_counts(&self) -> HashMap<String, u16> {
        if let Ok(commands) = COMMAND_PALETTE_HISTORY.list_commands_used() {
            commands
                .into_iter()
                .map(|command| (command.command_name, command.invocations))
                .collect()
        } else {
            HashMap::new()
        }
    }

    fn selected_command(&self) -> Option<&Command> {
        let action_ix = self
            .matches
            .get(self.selected_ix)
            .map(|m| m.candidate_id)
            .unwrap_or(self.selected_ix);
        // this gets called in headless tests where there are no commands loaded
        // so we need to return an Option here
        self.commands.get(action_ix)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn seed_history(&mut self, queries: &[&str]) {
        self.query_history.history = Some(queries.iter().map(|s| s.to_string()).collect());
    }
}

impl PickerDelegate for CommandPaletteDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Execute a command...".into()
    }

    fn select_history(
        &mut self,
        direction: Direction,
        query: &str,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<String> {
        match direction {
            Direction::Up => {
                let should_use_history =
                    self.selected_ix == 0 || self.query_history.is_navigating();
                if should_use_history {
                    if let Some(query) = self.query_history.previous(query).map(|s| s.to_string()) {
                        return Some(query);
                    }
                }
            }
            Direction::Down => {
                if self.query_history.is_navigating() {
                    if let Some(query) = self.query_history.next(query).map(|s| s.to_string()) {
                        return Some(query);
                    } else {
                        let prefix = self.query_history.prefix.take().unwrap_or_default();
                        self.query_history.reset_cursor();
                        return Some(prefix);
                    }
                }
            }
        }
        None
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_ix = ix;
    }

    fn update_matches(
        &mut self,
        mut query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let settings = WorkspaceSettings::get_global(cx);
        if let Some(alias) = settings.command_aliases.get(&query) {
            query = alias.to_string();
        }

        let workspace = self.workspace.clone();

        let intercept_task = GlobalCommandPaletteInterceptor::intercept(&query, workspace, cx);

        let (mut tx, mut rx) = postage::dispatch::channel(1);

        let query_str = query.as_str();
        let is_zed_link = parse_zed_link(query_str, cx).is_some();

        let task = cx.background_spawn({
            let mut commands = self.all_commands.clone();
            let hit_counts = self.hit_counts();
            let executor = cx.background_executor().clone();
            let query = normalize_action_query(query_str);
            let query_for_link = query_str.to_string();
            async move {
                commands.sort_by_key(|action| {
                    (
                        Reverse(hit_counts.get(&action.name).cloned()),
                        action.name.clone(),
                    )
                });

                let candidates = commands
                    .iter()
                    .enumerate()
                    .map(|(ix, command)| StringMatchCandidate::new(ix, &command.name))
                    .collect::<Vec<_>>();

                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    executor,
                )
                .await;

                let intercept_result = if is_zed_link {
                    CommandInterceptResult {
                        results: vec![CommandInterceptItem {
                            action: OpenZedUrl {
                                url: query_for_link.clone(),
                            }
                            .boxed_clone(),
                            string: query_for_link,
                            positions: vec![],
                        }],
                        exclusive: false,
                    }
                } else if let Some(task) = intercept_task {
                    task.await
                } else {
                    CommandInterceptResult::default()
                };

                tx.send((commands, matches, intercept_result))
                    .await
                    .log_err();
            }
        });

        self.updating_matches = Some((task, rx.clone()));

        cx.spawn_in(window, async move |picker, cx| {
            let Some((commands, matches, intercept_result)) = rx.recv().await else {
                return;
            };

            picker
                .update(cx, |picker, cx| {
                    picker
                        .delegate
                        .matches_updated(query, commands, matches, intercept_result, cx)
                })
                .log_err();
        })
    }

    fn finalize_update_matches(
        &mut self,
        query: String,
        duration: Duration,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> bool {
        let Some((task, rx)) = self.updating_matches.take() else {
            return true;
        };

        match cx
            .background_executor()
            .block_with_timeout(duration, rx.clone().recv())
        {
            Ok(Some((commands, matches, interceptor_result))) => {
                self.matches_updated(query, commands, matches, interceptor_result, cx);
                true
            }
            _ => {
                self.updating_matches = Some((task, rx));
                false
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.command_palette
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if secondary {
            let Some(selected_command) = self.selected_command() else {
                return;
            };
            let action_name = selected_command.action.name();
            let open_keymap = Box::new(zed_actions::ChangeKeybinding {
                action: action_name.to_string(),
            });
            window.dispatch_action(open_keymap, cx);
            self.dismissed(window, cx);
            return;
        }

        if self.matches.is_empty() {
            self.dismissed(window, cx);
            return;
        }

        if !self.latest_query.is_empty() {
            self.query_history.add(self.latest_query.clone());
            self.query_history.reset_cursor();
        }

        let action_ix = self.matches[self.selected_ix].candidate_id;
        let command = self.commands.swap_remove(action_ix);
        telemetry::event!(
            "Action Invoked",
            source = "command palette",
            action = command.name
        );
        self.matches.clear();
        self.commands.clear();
        let command_name = command.name.clone();
        let latest_query = self.latest_query.clone();
        cx.background_spawn(async move {
            COMMAND_PALETTE_HISTORY
                .write_command_invocation(command_name, latest_query)
                .await
        })
        .detach_and_log_err(cx);
        let action = command.action;
        window.focus(&self.previous_focus_handle);
        self.dismissed(window, cx);
        window.dispatch_action(action, cx);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let matching_command = self.matches.get(ix)?;
        let command = self.commands.get(matching_command.candidate_id)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .w_full()
                        .py_px()
                        .justify_between()
                        .child(HighlightedLabel::new(
                            command.name.clone(),
                            matching_command.positions.clone(),
                        ))
                        .child(KeyBinding::for_action_in(
                            &*command.action,
                            &self.previous_focus_handle,
                            cx,
                        )),
                ),
        )
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let selected_command = self.selected_command()?;
        let keybind =
            KeyBinding::for_action_in(&*selected_command.action, &self.previous_focus_handle, cx);

        let focus_handle = &self.previous_focus_handle;
        let keybinding_buttons = if keybind.has_binding(window) {
            Button::new("change", "Change Keybinding…")
                .key_binding(
                    KeyBinding::for_action_in(&menu::SecondaryConfirm, focus_handle, cx)
                        .map(|kb| kb.size(rems_from_px(12.))),
                )
                .on_click(move |_, window, cx| {
                    window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx);
                })
        } else {
            Button::new("add", "Add Keybinding…")
                .key_binding(
                    KeyBinding::for_action_in(&menu::SecondaryConfirm, focus_handle, cx)
                        .map(|kb| kb.size(rems_from_px(12.))),
                )
                .on_click(move |_, window, cx| {
                    window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx);
                })
        };

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(keybinding_buttons)
                .child(
                    Button::new("run-action", "Run")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .into_any(),
        )
    }
}

pub fn humanize_action_name(name: &str) -> String {
    let capacity = name.len() + name.chars().filter(|c| c.is_uppercase()).count();
    let mut result = String::with_capacity(capacity);
    for char in name.chars() {
        if char == ':' {
            if result.ends_with(':') {
                result.push(' ');
            } else {
                result.push(':');
            }
        } else if char == '_' {
            result.push(' ');
        } else if char.is_uppercase() {
            if !result.ends_with(' ') {
                result.push(' ');
            }
            result.extend(char.to_lowercase());
        } else {
            result.push(char);
        }
    }
    result
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use editor::Editor;
    use go_to_line::GoToLine;
    use gpui::{TestAppContext, VisualTestContext};
    use language::Point;
    use project::Project;
    use settings::KeymapFile;
    use workspace::{AppState, Workspace};

    #[test]
    fn test_humanize_action_name() {
        assert_eq!(
            humanize_action_name("editor::GoToDefinition"),
            "editor: go to definition"
        );
        assert_eq!(
            humanize_action_name("editor::Backspace"),
            "editor: backspace"
        );
        assert_eq!(
            humanize_action_name("go_to_line::Deploy"),
            "go to line: deploy"
        );
    }

    #[test]
    fn test_normalize_query() {
        assert_eq!(
            normalize_action_query("editor: backspace"),
            "editor: backspace"
        );
        assert_eq!(
            normalize_action_query("editor:  backspace"),
            "editor: backspace"
        );
        assert_eq!(
            normalize_action_query("editor:    backspace"),
            "editor: backspace"
        );
        assert_eq!(
            normalize_action_query("editor::GoToDefinition"),
            "editor:GoToDefinition"
        );
        assert_eq!(
            normalize_action_query("editor::::GoToDefinition"),
            "editor:GoToDefinition"
        );
        assert_eq!(
            normalize_action_query("editor: :GoToDefinition"),
            "editor: :GoToDefinition"
        );
    }

    #[gpui::test]
    async fn test_command_palette(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text("abc", window, cx);
            editor
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
            editor.update(cx, |editor, cx| window.focus(&editor.focus_handle(cx)))
        });

        cx.simulate_keystrokes("cmd-shift-p");

        let palette = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<CommandPalette>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        });

        palette.read_with(cx, |palette, _| {
            assert!(palette.delegate.commands.len() > 5);
            let is_sorted =
                |actions: &[Command]| actions.windows(2).all(|pair| pair[0].name <= pair[1].name);
            assert!(is_sorted(&palette.delegate.commands));
        });

        cx.simulate_input("bcksp");

        palette.read_with(cx, |palette, _| {
            assert_eq!(palette.delegate.matches[0].string, "editor: backspace");
        });

        cx.simulate_keystrokes("enter");

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<CommandPalette>(cx).is_none());
            assert_eq!(editor.read(cx).text(cx), "ab")
        });

        // Add namespace filter, and redeploy the palette
        cx.update(|_window, cx| {
            CommandPaletteFilter::update_global(cx, |filter, _| {
                filter.hide_namespace("editor");
            });
        });

        cx.simulate_keystrokes("cmd-shift-p");
        cx.simulate_input("bcksp");

        let palette = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<CommandPalette>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        });
        palette.read_with(cx, |palette, _| {
            assert!(palette.delegate.matches.is_empty())
        });
    }
    #[gpui::test]
    async fn test_normalized_matches(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let editor = cx.new_window_entity(|window, cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text("abc", window, cx);
            editor
        });

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
            editor.update(cx, |editor, cx| window.focus(&editor.focus_handle(cx)))
        });

        // Test normalize (trimming whitespace and double colons)
        cx.simulate_keystrokes("cmd-shift-p");

        let palette = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<CommandPalette>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        });

        cx.simulate_input("Editor::    Backspace");
        palette.read_with(cx, |palette, _| {
            assert_eq!(palette.delegate.matches[0].string, "editor: backspace");
        });
    }

    #[gpui::test]
    async fn test_go_to_line(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        cx.simulate_keystrokes("cmd-n");

        let editor = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<Editor>(cx).unwrap()
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("1\n2\n3\n4\n5\n6\n", window, cx)
        });

        cx.simulate_keystrokes("cmd-shift-p");
        cx.simulate_input("go to line: Toggle");
        cx.simulate_keystrokes("enter");

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<GoToLine>(cx).is_some())
        });

        cx.simulate_keystrokes("3 enter");

        editor.update_in(cx, |editor, window, cx| {
            assert!(editor.focus_handle(cx).is_focused(window));
            assert_eq!(
                editor
                    .selections
                    .last::<Point>(&editor.display_snapshot(cx))
                    .range()
                    .start,
                Point::new(2, 0)
            );
        });
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            menu::init();
            go_to_line::init(cx);
            workspace::init(app_state.clone(), cx);
            init(cx);
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[
                    {
                        "bindings": {
                            "cmd-n": "workspace::NewFile",
                            "enter": "menu::Confirm",
                            "cmd-shift-p": "command_palette::Toggle",
                            "up": "menu::SelectPrevious",
                            "down": "menu::SelectNext"
                        }
                    }
                ]"#,
                cx,
            ));
            app_state
        })
    }

    fn open_palette_with_history(
        workspace: &Entity<Workspace>,
        history: &[&str],
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<CommandPaletteDelegate>> {
        cx.simulate_keystrokes("cmd-shift-p");
        cx.run_until_parked();

        let palette = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<CommandPalette>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        });

        palette.update(cx, |palette, _cx| {
            palette.delegate.seed_history(history);
        });

        palette
    }

    #[gpui::test]
    async fn test_history_navigation_basic(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let palette = open_palette_with_history(&workspace, &["backspace", "select all"], cx);

        // Query should be empty initially
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "");
        });

        // Press up - should load most recent query "select all"
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "select all");
        });

        // Press up again - should load "backspace"
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "backspace");
        });

        // Press down - should go back to "select all"
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "select all");
        });

        // Press down again - should clear query (exit history mode)
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "");
        });
    }

    #[gpui::test]
    async fn test_history_mode_exit_on_typing(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let palette = open_palette_with_history(&workspace, &["backspace"], cx);

        // Press up to enter history mode
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "backspace");
        });

        // Type something - should append to the history query
        cx.simulate_input("x");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "backspacex");
        });
    }

    #[gpui::test]
    async fn test_history_navigation_with_suggestions(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let palette = open_palette_with_history(&workspace, &["editor: close", "editor: open"], cx);

        // Open palette with a query that has multiple matches
        cx.simulate_input("editor");
        cx.background_executor.run_until_parked();

        // Should have multiple matches, selected_ix should be 0
        palette.read_with(cx, |palette, _| {
            assert!(palette.delegate.matches.len() > 1);
            assert_eq!(palette.delegate.selected_ix, 0);
        });

        // Press down - should navigate to next suggestion (not history)
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, _| {
            assert_eq!(palette.delegate.selected_ix, 1);
        });

        // Press up - should go back to first suggestion
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, _| {
            assert_eq!(palette.delegate.selected_ix, 0);
        });

        // Press up again at top - should enter history mode and show previous query
        // that matches the "editor" prefix
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "editor: open");
        });
    }

    #[gpui::test]
    async fn test_history_prefix_search(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let palette = open_palette_with_history(
            &workspace,
            &["open file", "select all", "select line", "backspace"],
            cx,
        );

        // Type "sel" as a prefix
        cx.simulate_input("sel");
        cx.background_executor.run_until_parked();

        // Press up - should get "select line" (most recent matching "sel")
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "select line");
        });

        // Press up again - should get "select all" (next matching "sel")
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "select all");
        });

        // Press up again - should stay at "select all" (no more matches for "sel")
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "select all");
        });

        // Press down - should go back to "select line"
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "select line");
        });

        // Press down again - should return to original prefix "sel"
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "sel");
        });
    }

    #[gpui::test]
    async fn test_history_prefix_search_no_matches(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let palette =
            open_palette_with_history(&workspace, &["open file", "backspace", "select all"], cx);

        // Type "xyz" as a prefix that doesn't match anything
        cx.simulate_input("xyz");
        cx.background_executor.run_until_parked();

        // Press up - should stay at "xyz" (no matches)
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "xyz");
        });
    }

    #[gpui::test]
    async fn test_history_empty_prefix_searches_all(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let palette = open_palette_with_history(&workspace, &["alpha", "beta", "gamma"], cx);

        // With empty query, press up - should get "gamma" (most recent)
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "gamma");
        });

        // Press up - should get "beta"
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "beta");
        });

        // Press up - should get "alpha"
        cx.simulate_keystrokes("up");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "alpha");
        });

        // Press down - should get "beta"
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "beta");
        });

        // Press down - should get "gamma"
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "gamma");
        });

        // Press down - should return to empty string (exit history mode)
        cx.simulate_keystrokes("down");
        cx.background_executor.run_until_parked();
        palette.read_with(cx, |palette, cx| {
            assert_eq!(palette.query(cx), "");
        });
    }
}
