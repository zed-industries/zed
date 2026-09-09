use std::{
    collections::BTreeSet,
    sync::{Arc, atomic::AtomicBool},
};

use editor::Editor;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    ParentElement, Render, SharedString, Task, WeakEntity, Window, actions,
};
use language::Buffer;
use lsp::{LanguageServerId, LanguageServerName};
use picker::{Picker, PickerDelegate};
use project::{File, LspStore, Project};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use workspace::{ModalView, Workspace};

actions!(lsp_command_selector, [Toggle, ToggleArgumentsFocus]);

pub fn init(cx: &mut App) {
    cx.observe_new(LspCommandSelector::register).detach();
}

pub struct LspCommandSelector {
    picker: Entity<Picker<LspCommandSelectorDelegate>>,
}

impl LspCommandSelector {
    fn register(workspace: &mut Workspace, _: Option<&mut Window>, _: &mut Context<Workspace>) {
        workspace.register_action_renderer(|div, workspace, _, cx| {
            let commands_available = active_buffer(workspace, cx).is_some_and(|buffer| {
                available_commands(&buffer, workspace.project(), cx)
                    .next()
                    .is_some()
            });
            if !commands_available {
                return div;
            }
            div.on_action(cx.listener(|workspace, _: &Toggle, window, cx| {
                Self::toggle(workspace, window, cx);
            }))
        });
    }

    fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        let Some(buffer) = active_buffer(workspace, cx) else {
            return;
        };
        let project = workspace.project();
        let commands = available_commands(&buffer, project, cx).collect::<BTreeSet<_>>();
        let lsp_store = project.read(cx).lsp_store();
        workspace.toggle_modal(window, cx, move |window, cx| {
            LspCommandSelector::new(lsp_store, commands, window, cx)
        });
    }

    fn new(
        lsp_store: Entity<LspStore>,
        commands: BTreeSet<AvailableCommand>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let arguments_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(
                "JSON arguments (42, {\"key\": \"value\"}, …) or plain text for one string",
                window,
                cx,
            );
            editor
        });
        let delegate = LspCommandSelectorDelegate::new(
            cx.entity().downgrade(),
            lsp_store,
            arguments_editor,
            commands,
        );
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        Self { picker }
    }

    fn toggle_arguments_focus(
        &mut self,
        _: &ToggleArgumentsFocus,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let arguments_focus_handle = self
            .picker
            .read(cx)
            .delegate
            .arguments_editor
            .focus_handle(cx);
        if arguments_focus_handle.contains_focused(window, cx) {
            let picker_focus_handle = self.picker.focus_handle(cx);
            window.focus(&picker_focus_handle, cx);
        } else {
            window.focus(&arguments_focus_handle, cx);
        }
    }
}

impl Render for LspCommandSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("LspCommandSelector")
            .on_action(cx.listener(Self::toggle_arguments_focus))
            .child(self.picker.clone())
    }
}

impl Focusable for LspCommandSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for LspCommandSelector {}
impl ModalView for LspCommandSelector {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AvailableCommand {
    server_name: LanguageServerName,
    server_id: LanguageServerId,
    command: SharedString,
}

enum Execution {
    Idle,
    Running { _task: Task<()> },
    Failed(SharedString),
}

pub struct LspCommandSelectorDelegate {
    selector: WeakEntity<LspCommandSelector>,
    lsp_store: Entity<LspStore>,
    arguments_editor: Entity<Editor>,
    commands: Arc<[AvailableCommand]>,
    execution: Execution,
    candidates: Arc<[StringMatchCandidate]>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl LspCommandSelectorDelegate {
    fn new(
        selector: WeakEntity<LspCommandSelector>,
        lsp_store: Entity<LspStore>,
        arguments_editor: Entity<Editor>,
        commands: BTreeSet<AvailableCommand>,
    ) -> Self {
        let commands = commands.into_iter().collect::<Arc<[_]>>();
        let candidates = commands
            .iter()
            .enumerate()
            .map(|(index, command)| StringMatchCandidate::new(index, &command.command))
            .collect::<Arc<[_]>>();
        Self {
            selector,
            lsp_store,
            arguments_editor,
            commands,
            execution: Execution::Idle,
            candidates,
            matches: Vec::new(),
            selected_index: 0,
        }
    }

    fn is_executing(&self) -> bool {
        matches!(self.execution, Execution::Running { .. })
    }

    fn parse_arguments(&self, cx: &App) -> Vec<serde_json::Value> {
        let arguments_text = self.arguments_editor.read(cx).text(cx);
        let trimmed_arguments = arguments_text.trim();
        if trimmed_arguments.is_empty() {
            return Vec::new();
        }
        serde_json::Deserializer::from_str(trimmed_arguments)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|_| vec![serde_json::Value::String(trimmed_arguments.to_owned())])
    }
}

impl PickerDelegate for LspCommandSelectorDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "lsp command selector"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a language server command…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        !self.is_executing() && self.matches.get(ix).is_some()
    }

    fn has_another_open_menu(&self, window: &Window, cx: &App) -> bool {
        self.arguments_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.is_executing() {
            return;
        }
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(command) = self.commands.get(mat.candidate_id).cloned() else {
            return;
        };
        let arguments = self.parse_arguments(cx);
        let task = self.lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.execute_lsp_command(
                command.server_id,
                command.command.to_string(),
                arguments,
                cx,
            )
        });
        self.arguments_editor
            .update(cx, |editor, _| editor.set_read_only(true));
        let task = cx.spawn_in(window, async move |picker, cx| {
            let execution = match task.await {
                Ok(result) => {
                    if let Some(result) = result {
                        log::info!("LSP command {} returned {result}", command.command);
                    }
                    Execution::Idle
                }
                Err(error) => {
                    let error = format!(
                        "Failed to execute LSP command {}: {error:#}",
                        command.command
                    );
                    log::error!("{error}");
                    Execution::Failed(SharedString::from(error))
                }
            };
            picker
                .update_in(cx, |picker, window, cx| {
                    let succeeded = matches!(execution, Execution::Idle);
                    picker.delegate.execution = execution;
                    picker
                        .delegate
                        .arguments_editor
                        .update(cx, |editor, _| editor.set_read_only(false));
                    cx.notify();
                    if succeeded {
                        picker.delegate.dismissed(window, cx);
                    }
                })
                .ok();
        });
        self.execution = Execution::Running { _task: task };
        cx.notify();
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selector.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn_in(window, async move |picker, cx| {
            let matches = if query.is_empty() {
                candidates
                    .iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string.clone(),
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &AtomicBool::new(false),
                    background,
                )
                .await
            };

            picker
                .update_in(cx, |picker, window, cx| {
                    picker.delegate.matches = matches;
                    picker.set_selected_index(0, None, false, window, cx);
                    cx.notify();
                })
                .ok();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = self.matches.get(ix)?;
        let command = self.commands.get(mat.candidate_id)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .disabled(self.is_executing())
                .child(HighlightedLabel::new(
                    mat.string.clone(),
                    mat.positions.clone(),
                ))
                .end_slot(
                    Label::new(command.server_name.0.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            v_flex()
                .p_2()
                .gap_1()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .child(
                            Label::new("Arguments")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(ui::KeyBinding::for_action_in(
                                    &ToggleArgumentsFocus,
                                    &self.arguments_editor.focus_handle(cx),
                                    cx,
                                ))
                                .child(
                                    Label::new("to switch focus")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                ),
                        ),
                )
                .child(self.arguments_editor.clone())
                .map(|footer| match &self.execution {
                    Execution::Idle => footer,
                    Execution::Running { .. } => footer.child(
                        Label::new("Executing command…")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                    Execution::Failed(error) => footer.child(
                        Label::new(error.clone())
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    ),
                })
                .into_any_element(),
        )
    }
}

fn active_buffer(workspace: &Workspace, cx: &App) -> Option<Entity<Buffer>> {
    let editor = workspace.active_item(cx)?.act_as::<Editor>(cx)?;
    let editor = editor.read(cx);
    editor
        .buffer()
        .read(cx)
        .as_singleton()
        .or_else(|| editor.active_buffer(cx))
}

fn available_commands<'a>(
    buffer: &Entity<Buffer>,
    project: &Entity<Project>,
    cx: &'a App,
) -> impl Iterator<Item = AvailableCommand> + 'a {
    let project = project.read(cx);
    let lsp_store = project.lsp_store().read(cx);
    let buffer = buffer.read(cx);
    let adapters = buffer
        .language()
        .map(|language| project.languages().lsp_adapters(&language.name()))
        .unwrap_or_default();
    let worktree_owning_servers = File::from_dyn(buffer.file())
        .and_then(|file| project.worktree_for_id(file.worktree_id(cx), cx))
        .map(|worktree| worktree.read(cx))
        .filter(|worktree| worktree.is_visible())
        .map(|worktree| worktree.id());
    let opened_in_servers = lsp_store.language_server_ids_for_opened_buffer(buffer.remote_id());
    project
        .language_server_statuses(cx)
        .filter(move |(server_id, status)| {
            adapters.iter().any(|adapter| adapter.name() == status.name)
                && match (opened_in_servers, worktree_owning_servers, status.worktree) {
                    (Some(server_ids), _, _) => server_ids.contains(server_id),
                    (None, Some(buffer_worktree), Some(server_worktree)) => {
                        buffer_worktree == server_worktree
                    }
                    (None, _, _) => true,
                }
        })
        .filter_map(move |(server_id, status)| {
            let provider = lsp_store
                .lsp_server_capabilities
                .get(&server_id)?
                .execute_command_provider
                .as_ref()?;
            Some((server_id, status.name.clone(), &provider.commands))
        })
        .flat_map(|(server_id, server_name, commands)| {
            commands.iter().map(move |command| AvailableCommand {
                server_name: server_name.clone(),
                server_id,
                command: SharedString::from(command.clone()),
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{
        StreamExt as _,
        channel::{mpsc::UnboundedReceiver, oneshot},
    };
    use gpui::{KeyBinding, TestAppContext, VisualTestContext};
    use language::{FakeLspAdapter, Language, LanguageConfig, LanguageMatcher};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use std::{
        ops::Deref,
        path::PathBuf,
        sync::{Arc, Mutex},
    };
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, MultiWorkspace, OpenOptions, OpenVisible};

    #[gpui::test]
    async fn test_executing_commands_from_the_picker(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let SelectorTest {
            workspace,
            fake_server,
            executed_commands,
            mut cx,
            ..
        } = init_selector_test(cx).await;
        let (response_sender, response_receiver) = oneshot::channel();
        let response_receiver = Arc::new(Mutex::new(Some(response_receiver)));
        fake_server.set_request_handler::<lsp::request::ExecuteCommand, _, _>({
            let executed_commands = executed_commands.clone();
            move |params, _| {
                executed_commands
                    .lock()
                    .expect("executed commands lock should not be poisoned")
                    .push((params.command, params.arguments));
                let response_receiver = response_receiver
                    .lock()
                    .expect("response receiver lock should not be poisoned")
                    .take()
                    .expect("execute command should only be requested once");
                async move {
                    response_receiver
                        .await
                        .expect("response sender should remain alive");
                    Ok(None)
                }
            }
        });

        let picker = open_selector(&workspace, &mut cx);
        picker.read_with(&cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .matches
                    .iter()
                    .map(|mat| mat.string.as_str())
                    .collect::<Vec<_>>(),
                vec!["mdo.index", "mdo.today"],
            );
        });

        picker.update_in(&mut cx, |picker, window, cx| {
            picker.set_query("today", window, cx);
        });
        cx.run_until_parked();
        cx.dispatch_action(ToggleArgumentsFocus);
        cx.run_until_parked();
        picker.update_in(&mut cx, |picker, window, cx| {
            picker.delegate.arguments_editor.update(cx, |editor, cx| {
                editor.set_text("\"foo\" 42", window, cx);
            });
        });
        cx.dispatch_action(menu::Confirm);
        assert!(
            selector_is_open(&workspace, &cx),
            "the selector should remain open until the server responds"
        );
        assert_eq!(
            picker_state(&picker, &mut cx),
            PickerState {
                executing: true,
                error: None,
                arguments_read_only: true,
                command_selectable: false,
            },
        );

        cx.dispatch_action(menu::Confirm);
        assert_eq!(
            executed_commands
                .lock()
                .expect("executed commands lock should not be poisoned")
                .as_slice(),
            &[("mdo.today".to_string(), vec![json!("foo"), json!(42)])],
            "confirmation while executing should not send another command"
        );

        response_sender
            .send(())
            .expect("execute command response receiver should remain alive");
        cx.run_until_parked();

        assert!(
            !selector_is_open(&workspace, &cx),
            "the selector should be dismissed after a successful command execution"
        );
        assert_eq!(
            picker_state(&picker, &mut cx),
            PickerState::idle(),
            "the dismissed selector should be ready for reopening"
        );
    }

    #[gpui::test]
    async fn test_command_error_is_shown_in_the_picker_footer(cx: &mut TestAppContext) {
        let SelectorTest {
            workspace,
            fake_server,
            mut cx,
            ..
        } = init_selector_test(cx).await;
        fake_server.set_request_handler::<lsp::request::ExecuteCommand, _, _>(
            move |_, _| async move { Err(anyhow::anyhow!("server rejected command")) },
        );

        let picker = open_selector(&workspace, &mut cx);
        picker.update_in(&mut cx, |picker, window, cx| {
            picker.set_query("today", window, cx);
        });
        cx.run_until_parked();
        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();

        assert!(
            selector_is_open(&workspace, &cx),
            "the selector should remain open after a failed command execution"
        );
        assert_eq!(
            picker_state(&picker, &mut cx),
            PickerState {
                executing: false,
                error: Some(
                    "Failed to execute LSP command mdo.today: server rejected command".to_string()
                ),
                arguments_read_only: false,
                command_selectable: true,
            },
        );

        fake_server.set_request_handler::<lsp::request::ExecuteCommand, _, _>(
            move |_, _| async move { Ok(None) },
        );
        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();
        assert!(
            !selector_is_open(&workspace, &cx),
            "the selector should be dismissed after a successful retry"
        );
        assert_eq!(
            picker_state(&picker, &mut cx),
            PickerState::idle(),
            "the error should be cleared and the selector should be ready for reopening"
        );
    }

    #[gpui::test]
    async fn test_action_is_not_registered_without_commands(cx: &mut TestAppContext) {
        let SelectorTest {
            workspace, mut cx, ..
        } = init_selector_test_with_capabilities(
            cx,
            lsp::ServerCapabilities {
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: Vec::new(),
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
        )
        .await;

        cx.dispatch_action(Toggle);
        cx.run_until_parked();
        assert!(
            !selector_is_open(&workspace, &cx),
            "the selector should not open when no server advertises commands"
        );
    }

    #[gpui::test]
    async fn test_action_is_not_registered_for_unrelated_buffers(cx: &mut TestAppContext) {
        let SelectorTest {
            workspace, mut cx, ..
        } = init_selector_test(cx).await;

        let worktree_id = workspace.read_with(&cx, |workspace, cx| {
            workspace
                .project()
                .read(cx)
                .worktrees(cx)
                .next()
                .expect("project should have a worktree")
                .read(cx)
                .id()
        });
        workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path(
                    ProjectPath {
                        worktree_id,
                        path: rel_path("notes.txt").into(),
                    },
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .expect("file should open");
        cx.run_until_parked();

        cx.dispatch_action(Toggle);
        cx.run_until_parked();
        assert!(
            !selector_is_open(&workspace, &cx),
            "the selector should not open for a buffer whose language has no matching server, \
             even while another server in the project advertises commands"
        );
    }

    #[gpui::test]
    async fn test_commands_are_scoped_to_the_servers_of_the_buffer_worktree(
        cx: &mut TestAppContext,
    ) {
        let SelectorTest {
            workspace,
            fake_server: first_server,
            mut fake_servers,
            executed_commands: first_server_commands,
            mut cx,
        } = init_selector_test(cx).await;
        let fs = workspace.read_with(&cx, |workspace, cx| {
            workspace.project().read(cx).fs().clone()
        });
        fs.as_fake()
            .insert_tree(path!("/other"), json!({ "other.md": "# other\n" }))
            .await;
        let (other_worktree, _) = workspace
            .update(&mut cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    project.find_or_create_worktree(path!("/other"), true, cx)
                })
            })
            .await
            .expect("other worktree should be created");
        let other_worktree_id = other_worktree.read_with(&cx, |worktree, _| worktree.id());
        workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path(
                    ProjectPath {
                        worktree_id: other_worktree_id,
                        path: rel_path("other.md").into(),
                    },
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .expect("file should open");
        let second_server = fake_servers
            .next()
            .await
            .expect("a second server should start for the other worktree");
        let second_server_commands = Arc::new(Mutex::new(Vec::new()));
        second_server.set_request_handler::<lsp::request::ExecuteCommand, _, _>({
            let second_server_commands = second_server_commands.clone();
            move |params, _| {
                second_server_commands
                    .lock()
                    .unwrap()
                    .push((params.command, params.arguments));
                async move { Ok(None) }
            }
        });
        cx.run_until_parked();
        assert_ne!(
            first_server.server.server_id(),
            second_server.server.server_id(),
            "each worktree should get its own server instance"
        );

        let picker = open_selector(&workspace, &mut cx);
        picker.read_with(&cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .commands
                    .iter()
                    .map(|command| (command.command.as_ref(), command.server_id))
                    .collect::<Vec<_>>(),
                vec![
                    ("mdo.index", second_server.server.server_id()),
                    ("mdo.today", second_server.server.server_id()),
                ],
                "only the commands of the server for the active buffer's worktree should be listed"
            );
        });

        picker.update_in(&mut cx, |picker, window, cx| {
            picker.set_query("today", window, cx);
        });
        cx.run_until_parked();
        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();
        assert_eq!(
            (
                first_server_commands.lock().unwrap().as_slice(),
                second_server_commands.lock().unwrap().as_slice(),
            ),
            (
                [].as_slice(),
                [("mdo.today".to_string(), Vec::new())].as_slice(),
            ),
            "the command should be executed by the server of the active buffer's worktree only"
        );
    }

    #[gpui::test]
    async fn test_commands_of_reused_servers_for_buffers_outside_the_project(
        cx: &mut TestAppContext,
    ) {
        let SelectorTest {
            workspace,
            fake_server,
            mut fake_servers,
            executed_commands,
            mut cx,
        } = init_selector_test(cx).await;
        let fs = workspace.read_with(&cx, |workspace, cx| {
            workspace.project().read(cx).fs().clone()
        });
        fs.as_fake()
            .insert_tree(path!("/outside"), json!({ "readme.md": "# outside\n" }))
            .await;
        workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/outside/readme.md")),
                    OpenOptions {
                        visible: Some(OpenVisible::None),
                        ..OpenOptions::default()
                    },
                    window,
                    cx,
                )
            })
            .await
            .expect("file outside the project should open");
        cx.run_until_parked();
        assert_eq!(
            fake_servers.try_recv().ok().map(|_| "a new server"),
            None,
            "a buffer outside the project should reuse the existing server"
        );

        let picker = open_selector(&workspace, &mut cx);
        picker.read_with(&cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .commands
                    .iter()
                    .map(|command| (command.command.as_ref(), command.server_id))
                    .collect::<Vec<_>>(),
                vec![
                    ("mdo.index", fake_server.server.server_id()),
                    ("mdo.today", fake_server.server.server_id()),
                ],
                "the reused server's commands should be listed for a buffer outside the project"
            );
        });
        picker.update_in(&mut cx, |picker, window, cx| {
            picker.set_query("index", window, cx);
        });
        cx.run_until_parked();
        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();
        assert_eq!(
            executed_commands.lock().unwrap().as_slice(),
            &[("mdo.index".to_string(), Vec::new())],
        );
    }

    #[gpui::test]
    async fn test_toggling_arguments_focus(cx: &mut TestAppContext) {
        let SelectorTest {
            workspace, mut cx, ..
        } = init_selector_test(cx).await;

        let picker = open_selector(&workspace, &mut cx);
        let picker_focus_handle = picker.read_with(&cx, |picker, cx| picker.focus_handle(cx));
        let arguments_focus_handle = picker.read_with(&cx, |picker, cx| {
            picker.delegate.arguments_editor.focus_handle(cx)
        });
        cx.update(|window, _| {
            assert_eq!(
                (
                    picker_focus_handle.is_focused(window),
                    arguments_focus_handle.is_focused(window),
                ),
                (true, false),
                "the query editor should be focused when the selector opens"
            );
        });

        cx.dispatch_action(ToggleArgumentsFocus);
        cx.run_until_parked();
        cx.update(|window, _| {
            assert_eq!(
                (
                    picker_focus_handle.is_focused(window),
                    arguments_focus_handle.is_focused(window),
                ),
                (false, true),
                "the arguments editor should be focused after toggling"
            );
        });

        cx.dispatch_action(ToggleArgumentsFocus);
        cx.run_until_parked();
        cx.update(|window, _| {
            assert_eq!(
                (
                    picker_focus_handle.is_focused(window),
                    arguments_focus_handle.is_focused(window),
                ),
                (true, false),
                "the query editor should be focused after toggling again"
            );
        });
    }

    #[gpui::test]
    async fn test_tab_keystroke_toggles_arguments_focus(cx: &mut TestAppContext) {
        let SelectorTest {
            workspace, mut cx, ..
        } = init_selector_test(cx).await;
        cx.update(|_, cx| {
            cx.bind_keys([
                KeyBinding::new("tab", menu::SelectNext, None),
                KeyBinding::new("tab", editor::actions::Tab, Some("Editor")),
                KeyBinding::new("tab", picker::ConfirmCompletion, Some("Picker > Editor")),
                KeyBinding::new(
                    "tab",
                    ToggleArgumentsFocus,
                    Some("LspCommandSelector > Picker > Editor"),
                ),
            ]);
        });

        let picker = open_selector(&workspace, &mut cx);
        let picker_focus_handle = picker.read_with(&cx, |picker, cx| picker.focus_handle(cx));
        let arguments_focus_handle = picker.read_with(&cx, |picker, cx| {
            picker.delegate.arguments_editor.focus_handle(cx)
        });

        cx.simulate_keystrokes("tab");
        cx.update(|window, _| {
            assert_eq!(
                (
                    picker_focus_handle.is_focused(window),
                    arguments_focus_handle.is_focused(window),
                ),
                (false, true),
                "tab should win over the less specific tab bindings and focus the arguments editor"
            );
        });

        cx.simulate_keystrokes("tab");
        cx.update(|window, _| {
            assert_eq!(
                (
                    picker_focus_handle.is_focused(window),
                    arguments_focus_handle.is_focused(window),
                ),
                (true, false),
                "tab should focus the query editor back"
            );
        });
    }

    #[gpui::test]
    async fn test_plain_text_arguments_are_sent_as_a_single_string(cx: &mut TestAppContext) {
        let SelectorTest {
            workspace,
            executed_commands,
            mut cx,
            ..
        } = init_selector_test(cx).await;

        let picker = open_selector(&workspace, &mut cx);
        picker.update_in(&mut cx, |picker, window, cx| {
            picker.set_query("today", window, cx);
        });
        cx.run_until_parked();
        picker.update_in(&mut cx, |picker, window, cx| {
            picker.delegate.arguments_editor.update(cx, |editor, cx| {
                editor.set_text("next friday", window, cx);
            });
        });
        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();

        assert!(
            !selector_is_open(&workspace, &cx),
            "the selector should be dismissed after execution"
        );
        assert_eq!(
            executed_commands
                .lock()
                .unwrap()
                .drain(..)
                .collect::<Vec<_>>(),
            vec![("mdo.today".to_string(), vec![json!("next friday")])],
            "plain text that is not a JSON stream should be sent as a single string argument"
        );
    }

    struct SelectorTest {
        workspace: Entity<Workspace>,
        fake_server: lsp::FakeLanguageServer,
        fake_servers: UnboundedReceiver<lsp::FakeLanguageServer>,
        executed_commands: Arc<Mutex<Vec<(String, Vec<serde_json::Value>)>>>,
        cx: VisualTestContext,
    }

    async fn init_selector_test(cx: &mut TestAppContext) -> SelectorTest {
        init_selector_test_with_capabilities(
            cx,
            lsp::ServerCapabilities {
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["mdo.today".to_string(), "mdo.index".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
        )
        .await
    }

    async fn init_selector_test_with_capabilities(
        cx: &mut TestAppContext,
        capabilities: lsp::ServerCapabilities,
    ) -> SelectorTest {
        zlog::init_test();
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            settings::init(cx);
            super::init(cx);
            editor::init(cx);
            app_state
        });
        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/test"),
                json!({
                    "main.md": "# hi\n",
                    "notes.txt": "plain text\n",
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/test").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Markdown".into(),
                matcher: (LanguageMatcher {
                    path_suffixes: vec!["md".to_string()],
                    ..LanguageMatcher::default()
                })
                .into(),
                ..LanguageConfig::default()
            },
            None,
        )));
        let mut fake_servers = language_registry.register_fake_lsp(
            "Markdown",
            FakeLspAdapter {
                name: "test-server",
                capabilities,
                ..FakeLspAdapter::default()
            },
        );

        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let multi_workspace = window.root(cx).expect("window should have a root");
        let mut cx = VisualTestContext::from_window(*window.deref(), cx);
        let cx = &mut cx;
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());

        let worktree_id = project.update(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .expect("project should have a worktree")
                .read(cx)
                .id()
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    ProjectPath {
                        worktree_id,
                        path: rel_path("main.md").into(),
                    },
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .expect("file should open");

        let fake_server = fake_servers.next().await.expect("server should start");
        let executed_commands = Arc::new(Mutex::new(Vec::new()));
        fake_server.set_request_handler::<lsp::request::ExecuteCommand, _, _>({
            let executed_commands = executed_commands.clone();
            move |params, _| {
                executed_commands
                    .lock()
                    .unwrap()
                    .push((params.command, params.arguments));
                async move { Ok(None) }
            }
        });
        cx.run_until_parked();

        SelectorTest {
            workspace,
            fake_server,
            fake_servers,
            executed_commands,
            cx: cx.clone(),
        }
    }

    fn open_selector(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<Picker<LspCommandSelectorDelegate>> {
        cx.dispatch_action(Toggle);
        cx.run_until_parked();
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<LspCommandSelector>(cx)
                .expect("lsp command selector should be open")
                .read(cx)
                .picker
                .clone()
        })
    }

    fn selector_is_open(workspace: &Entity<Workspace>, cx: &VisualTestContext) -> bool {
        workspace.read_with(cx, |workspace, cx| {
            workspace.active_modal::<LspCommandSelector>(cx).is_some()
        })
    }

    #[derive(Debug, PartialEq)]
    struct PickerState {
        executing: bool,
        error: Option<String>,
        arguments_read_only: bool,
        command_selectable: bool,
    }

    impl PickerState {
        fn idle() -> Self {
            Self {
                executing: false,
                error: None,
                arguments_read_only: false,
                command_selectable: true,
            }
        }
    }

    fn picker_state(
        picker: &Entity<Picker<LspCommandSelectorDelegate>>,
        cx: &mut VisualTestContext,
    ) -> PickerState {
        picker.update_in(cx, |picker, window, cx| PickerState {
            executing: picker.delegate.is_executing(),
            error: match &picker.delegate.execution {
                Execution::Failed(error) => Some(error.to_string()),
                Execution::Idle | Execution::Running { .. } => None,
            },
            arguments_read_only: picker.delegate.arguments_editor.read(cx).read_only(cx),
            command_selectable: picker.delegate.can_select(0, window, cx),
        })
    }
}
