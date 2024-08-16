use std::{
    cmp::{self, Reverse},
    sync::Arc,
    time::Duration,
};

use client::{parse_zed_link, telemetry::Telemetry};
use collections::HashMap;
use command_palette_hooks::{
    CommandInterceptResult, CommandPaletteFilter, CommandPaletteInterceptor,
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, Action, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Global,
    ParentElement, Render, Styled, Task, UpdateGlobal, View, ViewContext, VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};

use postage::{sink::Sink, stream::Stream};
use settings::Settings;
use ui::{h_flex, prelude::*, v_flex, HighlightedLabel, KeyBinding, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ModalView, Workspace, WorkspaceSettings};
use zed_actions::OpenZedUrl;

actions!(command_palette, [Toggle]);

pub fn init(cx: &mut AppContext) {
    client::init_settings(cx);
    cx.set_global(HitCounts::default());
    command_palette_hooks::init(cx);
    cx.observe_new_views(CommandPalette::register).detach();
}

impl ModalView for CommandPalette {}

pub struct CommandPalette {
    picker: View<Picker<CommandPaletteDelegate>>,
}

fn trim_consecutive_whitespaces(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut last_char_was_whitespace = false;

    for char in input.trim().chars() {
        if char.is_whitespace() {
            if !last_char_was_whitespace {
                result.push(char);
            }
            last_char_was_whitespace = true;
        } else {
            result.push(char);
            last_char_was_whitespace = false;
        }
    }
    result
}

impl CommandPalette {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &Toggle, cx| Self::toggle(workspace, "", cx));
    }

    pub fn toggle(workspace: &mut Workspace, query: &str, cx: &mut ViewContext<Workspace>) {
        let Some(previous_focus_handle) = cx.focused() else {
            return;
        };
        let telemetry = workspace.client().telemetry().clone();
        workspace.toggle_modal(cx, move |cx| {
            CommandPalette::new(previous_focus_handle, telemetry, query, cx)
        });
    }

    fn new(
        previous_focus_handle: FocusHandle,
        telemetry: Arc<Telemetry>,
        query: &str,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let filter = CommandPaletteFilter::try_global(cx);

        let commands = cx
            .available_actions()
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
            cx.view().downgrade(),
            commands,
            telemetry,
            previous_focus_handle,
        );

        let picker = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx);
            picker.set_query(query, cx);
            picker
        });
        Self { picker }
    }

    pub fn set_query(&mut self, query: &str, cx: &mut ViewContext<Self>) {
        self.picker
            .update(cx, |picker, cx| picker.set_query(query, cx))
    }
}

impl EventEmitter<DismissEvent> for CommandPalette {}

impl FocusableView for CommandPalette {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

pub struct CommandPaletteDelegate {
    command_palette: WeakView<CommandPalette>,
    all_commands: Vec<Command>,
    commands: Vec<Command>,
    matches: Vec<StringMatch>,
    selected_ix: usize,
    telemetry: Arc<Telemetry>,
    previous_focus_handle: FocusHandle,
    updating_matches: Option<(
        Task<()>,
        postage::dispatch::Receiver<(Vec<Command>, Vec<StringMatch>)>,
    )>,
}

struct Command {
    name: String,
    action: Box<dyn Action>,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            action: self.action.boxed_clone(),
        }
    }
}

/// Hit count for each command in the palette.
/// We only account for commands triggered directly via command palette and not by e.g. keystrokes because
/// if a user already knows a keystroke for a command, they are unlikely to use a command palette to look for it.
#[derive(Default, Clone)]
struct HitCounts(HashMap<String, usize>);

impl Global for HitCounts {}

impl CommandPaletteDelegate {
    fn new(
        command_palette: WeakView<CommandPalette>,
        commands: Vec<Command>,
        telemetry: Arc<Telemetry>,
        previous_focus_handle: FocusHandle,
    ) -> Self {
        Self {
            command_palette,
            all_commands: commands.clone(),
            matches: vec![],
            commands,
            selected_ix: 0,
            telemetry,
            previous_focus_handle,
            updating_matches: None,
        }
    }

    fn matches_updated(
        &mut self,
        query: String,
        mut commands: Vec<Command>,
        mut matches: Vec<StringMatch>,
        cx: &mut ViewContext<Picker<Self>>,
    ) {
        self.updating_matches.take();

        let mut intercept_result = CommandPaletteInterceptor::try_global(cx)
            .and_then(|interceptor| interceptor.intercept(&query, cx));

        if parse_zed_link(&query, cx).is_some() {
            intercept_result = Some(CommandInterceptResult {
                action: OpenZedUrl { url: query.clone() }.boxed_clone(),
                string: query.clone(),
                positions: vec![],
            })
        }

        if let Some(CommandInterceptResult {
            action,
            string,
            positions,
        }) = intercept_result
        {
            if let Some(idx) = matches
                .iter()
                .position(|m| commands[m.candidate_id].action.type_id() == action.type_id())
            {
                matches.remove(idx);
            }
            commands.push(Command {
                name: string.clone(),
                action,
            });
            matches.insert(
                0,
                StringMatch {
                    candidate_id: commands.len() - 1,
                    string,
                    positions,
                    score: 0.0,
                },
            )
        }
        self.commands = commands;
        self.matches = matches;
        if self.matches.is_empty() {
            self.selected_ix = 0;
        } else {
            self.selected_ix = cmp::min(self.selected_ix, self.matches.len() - 1);
        }
    }
}

impl PickerDelegate for CommandPaletteDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Execute a command...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_ix = ix;
    }

    fn update_matches(
        &mut self,
        mut query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let settings = WorkspaceSettings::get_global(cx);
        if let Some(alias) = settings.command_aliases.get(&query) {
            query = alias.to_string();
        }
        let (mut tx, mut rx) = postage::dispatch::channel(1);
        let task = cx.background_executor().spawn({
            let mut commands = self.all_commands.clone();
            let hit_counts = cx.global::<HitCounts>().clone();
            let executor = cx.background_executor().clone();
            let query = trim_consecutive_whitespaces(&query.as_str());
            async move {
                commands.sort_by_key(|action| {
                    (
                        Reverse(hit_counts.0.get(&action.name).cloned()),
                        action.name.clone(),
                    )
                });

                let candidates = commands
                    .iter()
                    .enumerate()
                    .map(|(ix, command)| StringMatchCandidate {
                        id: ix,
                        string: command.name.to_string(),
                        char_bag: command.name.chars().collect(),
                    })
                    .collect::<Vec<_>>();
                let matches = if query.is_empty() {
                    candidates
                        .into_iter()
                        .enumerate()
                        .map(|(index, candidate)| StringMatch {
                            candidate_id: index,
                            string: candidate.string,
                            positions: Vec::new(),
                            score: 0.0,
                        })
                        .collect()
                } else {
                    let ret = fuzzy::match_strings(
                        &candidates,
                        &query,
                        true,
                        10000,
                        &Default::default(),
                        executor,
                    )
                    .await;
                    ret
                };

                tx.send((commands, matches)).await.log_err();
            }
        });
        self.updating_matches = Some((task, rx.clone()));

        cx.spawn(move |picker, mut cx| async move {
            let Some((commands, matches)) = rx.recv().await else {
                return;
            };

            picker
                .update(&mut cx, |picker, cx| {
                    picker
                        .delegate
                        .matches_updated(query, commands, matches, cx)
                })
                .log_err();
        })
    }

    fn finalize_update_matches(
        &mut self,
        query: String,
        duration: Duration,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> bool {
        let Some((task, rx)) = self.updating_matches.take() else {
            return true;
        };

        match cx
            .background_executor()
            .block_with_timeout(duration, rx.clone().recv())
        {
            Ok(Some((commands, matches))) => {
                self.matches_updated(query, commands, matches, cx);
                true
            }
            _ => {
                self.updating_matches = Some((task, rx));
                false
            }
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.command_palette
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if self.matches.is_empty() {
            self.dismissed(cx);
            return;
        }
        let action_ix = self.matches[self.selected_ix].candidate_id;
        let command = self.commands.swap_remove(action_ix);

        self.telemetry
            .report_action_event("command palette", command.name.clone());

        self.matches.clear();
        self.commands.clear();
        HitCounts::update_global(cx, |hit_counts, _cx| {
            *hit_counts.0.entry(command.name).or_default() += 1;
        });
        let action = command.action;
        cx.focus(&self.previous_focus_handle);
        self.dismissed(cx);
        cx.dispatch_action(action);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let r#match = self.matches.get(ix)?;
        let command = self.commands.get(r#match.candidate_id)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(
                    h_flex()
                        .w_full()
                        .py_px()
                        .justify_between()
                        .child(HighlightedLabel::new(
                            command.name.clone(),
                            r#match.positions.clone(),
                        ))
                        .children(KeyBinding::for_action_in(
                            &*command.action,
                            &self.previous_focus_handle,
                            cx,
                        )),
                ),
        )
    }
}

fn humanize_action_name(name: &str) -> String {
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
    use gpui::TestAppContext;
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

    #[gpui::test]
    async fn test_command_palette(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::single_line(cx);
            editor.set_text("abc", cx);
            editor
        });

        workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, cx);
            editor.update(cx, |editor, cx| editor.focus(cx))
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

        palette.update(cx, |palette, _| {
            assert!(palette.delegate.commands.len() > 5);
            let is_sorted =
                |actions: &[Command]| actions.windows(2).all(|pair| pair[0].name <= pair[1].name);
            assert!(is_sorted(&palette.delegate.commands));
        });

        cx.simulate_input("bcksp");

        palette.update(cx, |palette, _| {
            assert_eq!(palette.delegate.matches[0].string, "editor: backspace");
        });

        cx.simulate_keystrokes("enter");

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<CommandPalette>(cx).is_none());
            assert_eq!(editor.read(cx).text(cx), "ab")
        });

        // Add namespace filter, and redeploy the palette
        cx.update(|cx| {
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
        palette.update(cx, |palette, _| {
            assert!(palette.delegate.matches.is_empty())
        });
    }

    #[gpui::test]
    async fn test_go_to_line(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

        cx.simulate_keystrokes("cmd-n");

        let editor = workspace.update(cx, |workspace, cx| {
            workspace.active_item_as::<Editor>(cx).unwrap()
        });
        editor.update(cx, |editor, cx| editor.set_text("1\n2\n3\n4\n5\n6\n", cx));

        cx.simulate_keystrokes("cmd-shift-p");
        cx.simulate_input("go to line: Toggle");
        cx.simulate_keystrokes("enter");

        workspace.update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<GoToLine>(cx).is_some())
        });

        cx.simulate_keystrokes("3 enter");

        editor.update(cx, |editor, cx| {
            assert!(editor.focus_handle(cx).is_focused(cx));
            assert_eq!(
                editor.selections.last::<Point>(cx).range().start,
                Point::new(2, 0)
            );
        });
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            editor::init(cx);
            menu::init();
            go_to_line::init(cx);
            workspace::init(app_state.clone(), cx);
            init(cx);
            Project::init_settings(cx);
            KeymapFile::parse(
                r#"[
                    {
                        "bindings": {
                            "cmd-n": "workspace::NewFile",
                            "enter": "menu::Confirm",
                            "cmd-shift-p": "command_palette::Toggle"
                        }
                    }
                ]"#,
            )
            .unwrap()
            .add_to_cx(cx)
            .unwrap();
            app_state
        })
    }
}
