use std::{
    collections::BTreeSet,
    sync::{Arc, atomic::AtomicBool},
};

use editor::Editor;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ParentElement,
    Render, SharedString, WeakEntity, Window, actions,
};
use lsp::{LanguageServerId, LanguageServerName};
use notifications::status_toast::StatusToast;
use picker::{Picker, PickerDelegate};
use project::Project;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
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
            let commands_available = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
                .and_then(|editor| editor.read(cx).active_buffer(cx))
                .is_some_and(|buffer| has_available_commands(&buffer, workspace.project(), cx));
            if !commands_available {
                return div;
            }
            div.on_action(cx.listener(|workspace, _: &Toggle, window, cx| {
                Self::toggle(workspace, window, cx);
            }))
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
        let buffer = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_buffer(cx)?;
        let project = workspace.project().clone();
        let commands = available_commands(&buffer, &project, cx);
        let workspace_handle = cx.weak_entity();
        workspace.toggle_modal(window, cx, move |window, cx| {
            LspCommandSelector::new(workspace_handle, project, commands, window, cx)
        });
        Some(())
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        commands: Vec<AvailableCommand>,
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
            workspace,
            project,
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

pub struct LspCommandSelectorDelegate {
    selector: WeakEntity<LspCommandSelector>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    arguments_editor: Entity<Editor>,
    commands: Arc<[AvailableCommand]>,
    candidates: Arc<[StringMatchCandidate]>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl LspCommandSelectorDelegate {
    fn new(
        selector: WeakEntity<LspCommandSelector>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        arguments_editor: Entity<Editor>,
        commands: Vec<AvailableCommand>,
    ) -> Self {
        let candidates = commands
            .iter()
            .enumerate()
            .map(|(index, command)| StringMatchCandidate::new(index, &command.command))
            .collect::<Arc<[_]>>();
        Self {
            selector,
            workspace,
            project,
            arguments_editor,
            commands: commands.into(),
            candidates,
            matches: Vec::new(),
            selected_index: 0,
        }
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

    fn has_another_open_menu(&self, window: &Window, cx: &App) -> bool {
        self.arguments_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(mat) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(command) = self.commands.get(mat.candidate_id).cloned() else {
            return;
        };
        let arguments = self.parse_arguments(cx);
        let task = self.project.update(cx, |project, cx| {
            project.lsp_store().update(cx, |lsp_store, cx| {
                lsp_store.execute_lsp_command(
                    command.server_id,
                    command.command.to_string(),
                    arguments,
                    cx,
                )
            })
        });
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            match task.await {
                Ok(Some(result)) => {
                    log::info!("LSP command {} returned {result}", command.command)
                }
                Ok(None) => {}
                Err(error) => {
                    log::error!(
                        "Failed to execute LSP command {}: {error:?}",
                        command.command
                    );
                    workspace
                        .update(cx, |workspace, cx| {
                            let status_toast = StatusToast::new(
                                format!(
                                    "Failed to execute LSP command {}: {error:#}",
                                    command.command
                                ),
                                cx,
                                |this, _| {
                                    this.icon(Icon::new(IconName::Warning).color(Color::Error))
                                        .dismiss_button(true)
                                },
                            );
                            workspace.toggle_status_toast(status_toast, cx);
                        })
                        .ok();
                }
            }
            anyhow::Ok(())
        })
        .detach();
        self.dismissed(window, cx);
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
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn_in(window, async move |this, cx| {
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

            this.update_in(cx, |this, window, cx| {
                this.delegate.matches = matches;
                this.set_selected_index(0, None, false, window, cx);
                cx.notify();
            })
            .log_err();
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
    ) -> Option<gpui::AnyElement> {
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
                            Label::new("Tab to switch focus")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                )
                .child(self.arguments_editor.clone())
                .into_any_element(),
        )
    }
}

fn available_commands(
    buffer: &Entity<language::Buffer>,
    project: &Entity<Project>,
    cx: &App,
) -> Vec<AvailableCommand> {
    let Some(language) = buffer.read(cx).language() else {
        return Vec::new();
    };
    let project = project.read(cx);
    let adapters = project.languages().lsp_adapters(&language.name());
    let lsp_store = project.lsp_store().read(cx);
    project
        .language_server_statuses(cx)
        .filter(|(_, status)| adapters.iter().any(|adapter| adapter.name() == status.name))
        .flat_map(|(server_id, status)| {
            lsp_store
                .lsp_server_capabilities
                .get(&server_id)
                .and_then(|capabilities| capabilities.execute_command_provider.as_ref())
                .into_iter()
                .flat_map(move |provider| {
                    provider
                        .commands
                        .iter()
                        .map(move |command| AvailableCommand {
                            server_name: status.name.clone(),
                            server_id,
                            command: SharedString::from(command.clone()),
                        })
                })
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn has_available_commands(
    buffer: &Entity<language::Buffer>,
    project: &Entity<Project>,
    cx: &App,
) -> bool {
    let Some(language) = buffer.read(cx).language() else {
        return false;
    };
    let project = project.read(cx);
    let adapters = project.languages().lsp_adapters(&language.name());
    let lsp_store = project.lsp_store().read(cx);
    project
        .language_server_statuses(cx)
        .filter(|(_, status)| adapters.iter().any(|adapter| adapter.name() == status.name))
        .any(|(server_id, _)| {
            lsp_store
                .lsp_server_capabilities
                .get(&server_id)
                .and_then(|capabilities| capabilities.execute_command_provider.as_ref())
                .is_some_and(|provider| !provider.commands.is_empty())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt as _;
    use gpui::{TestAppContext, VisualTestContext};
    use language::{FakeLspAdapter, Language, LanguageConfig, LanguageMatcher};
    use project::{Project, ProjectPath};
    use serde_json::json;
    use std::{
        ops::Deref,
        sync::{Arc, Mutex},
    };
    use util::{path, rel_path::rel_path};
    use workspace::{AppState, MultiWorkspace};

    #[gpui::test]
    async fn test_executing_commands_from_the_picker(cx: &mut TestAppContext) {
        let (workspace, fake_server, executed_commands, mut cx) = init_selector_test(cx).await;

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
        cx.run_until_parked();

        workspace.read_with(&cx, |workspace, cx| {
            assert_eq!(
                workspace
                    .active_modal::<LspCommandSelector>(cx)
                    .map(|_| "a modal"),
                None,
                "the selector should be dismissed after a successful command execution"
            );
        });
        assert_eq!(
            executed_commands
                .lock()
                .unwrap()
                .drain(..)
                .collect::<Vec<_>>(),
            vec![("mdo.today".to_string(), vec![json!("foo"), json!(42)])],
        );
        drop(fake_server);
    }

    #[gpui::test]
    async fn test_action_is_not_registered_without_commands(cx: &mut TestAppContext) {
        let (workspace, fake_server, _executed_commands, mut cx) =
            init_selector_test_with_capabilities(
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
        workspace.read_with(&cx, |workspace, cx| {
            assert_eq!(
                workspace
                    .active_modal::<LspCommandSelector>(cx)
                    .map(|_| "a modal"),
                None,
                "the selector should not open when no server advertises commands"
            );
        });
        drop(fake_server);
    }

    #[gpui::test]
    async fn test_toggling_arguments_focus(cx: &mut TestAppContext) {
        let (workspace, fake_server, _executed_commands, mut cx) = init_selector_test(cx).await;

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
        drop(fake_server);
    }

    #[gpui::test]
    async fn test_plain_text_arguments_are_sent_as_a_single_string(cx: &mut TestAppContext) {
        let (workspace, fake_server, executed_commands, mut cx) = init_selector_test(cx).await;

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

        workspace.read_with(&cx, |workspace, cx| {
            assert_eq!(
                workspace
                    .active_modal::<LspCommandSelector>(cx)
                    .map(|_| "a modal"),
                None,
                "the selector should be dismissed after execution"
            );
        });
        assert_eq!(
            executed_commands
                .lock()
                .unwrap()
                .drain(..)
                .collect::<Vec<_>>(),
            vec![("mdo.today".to_string(), vec![json!("next friday")])],
            "plain text that is not a JSON stream should be sent as a single string argument"
        );
        drop(fake_server);
    }

    async fn init_selector_test(
        cx: &mut TestAppContext,
    ) -> (
        Entity<Workspace>,
        lsp::FakeLanguageServer,
        Arc<Mutex<Vec<(String, Vec<serde_json::Value>)>>>,
        VisualTestContext,
    ) {
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
    ) -> (
        Entity<Workspace>,
        lsp::FakeLanguageServer,
        Arc<Mutex<Vec<(String, Vec<serde_json::Value>)>>>,
        VisualTestContext,
    ) {
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

        (workspace, fake_server, executed_commands, cx.clone())
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
}
