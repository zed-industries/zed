use editor::Editor;
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, TaskExt, actions,
    prelude::*,
};
use language::{Buffer, BufferEvent};
use project::{Project, ProjectItem as _};
use ui::{ButtonLike, ElevationIndex, KeyBinding, prelude::*};
use util::ResultExt as _;
use workspace::item::ItemEvent;
use workspace::{Workspace, item::Item};

use crate::jupyter_settings::JupyterSettings;
use crate::repl_store::ReplStore;

fn refresh_python_kernelspecs_for_buffer(
    buffer: &Entity<Buffer>,
    project: &Entity<Project>,
    cx: &mut App,
) {
    let buffer = buffer.read(cx);

    if buffer
        .language()
        .is_none_or(|language| language.name() != "Python")
    {
        return;
    };

    let Some(project_path) = buffer.project_path(cx) else {
        return;
    };
    let store = ReplStore::global(cx);
    store.update(cx, |store, cx| {
        store
            .refresh_python_kernelspecs(project_path.worktree_id, project, cx)
            .detach_and_log_err(cx);
    });
}

actions!(
    repl,
    [
        /// Runs the current cell and advances to the next one.
        Run,
        /// Runs the current cell without advancing.
        RunInPlace,
        /// Clears all outputs in the REPL.
        ClearOutputs,
        /// Clears the output of the cell at the current cursor position.
        ClearCurrentOutput,
        /// Opens the REPL sessions panel.
        Sessions,
        /// Interrupts the currently running kernel.
        Interrupt,
        /// Shuts down the current kernel.
        Shutdown,
        /// Restarts the current kernel.
        Restart,
        /// Refreshes the list of available kernelspecs.
        RefreshKernelspecs
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &Sessions, window, cx| {
                let existing = workspace
                    .active_pane()
                    .read(cx)
                    .items()
                    .find_map(|item| item.downcast::<ReplSessionsPage>());

                if let Some(existing) = existing {
                    workspace.activate_item(&existing, true, true, window, cx);
                } else {
                    let repl_sessions_page = ReplSessionsPage::new(window, cx);
                    workspace.add_item_to_active_pane(
                        Box::new(repl_sessions_page),
                        None,
                        true,
                        window,
                        cx,
                    )
                }
            });

            workspace.register_action(|_workspace, _: &RefreshKernelspecs, _, cx| {
                let store = ReplStore::global(cx);
                store.update(cx, |store, cx| {
                    store.refresh_kernelspecs(cx).detach();
                });
            });
        },
    )
    .detach();

    cx.observe_new(
        move |editor: &mut Editor, window, cx: &mut Context<Editor>| {
            let Some(window) = window else {
                return;
            };

            if !editor.use_modal_editing() || !editor.buffer().read(cx).is_singleton() {
                return;
            }

            cx.defer_in(window, |editor, _window, cx| {
                let project = editor.project().cloned();

                let is_valid_project = project
                    .as_ref()
                    .map(|project| {
                        let p = project.read(cx);
                        !p.is_via_collab()
                    })
                    .unwrap_or(false);

                if !is_valid_project {
                    return;
                }

                let buffer = editor.buffer().read(cx).as_singleton();

                let editor_handle = cx.entity().downgrade();

                // Subscribe to the buffer's `LanguageChanged` events so remote projects,
                // where language detection can complete after the editor is observed,
                // still trigger a kernelspec refresh. Without this the REPL UI stays
                // hidden until something else populates the global kernel list.
                if let Some((buffer, project)) = buffer.zip(project) {
                    refresh_python_kernelspecs_for_buffer(&buffer, &project, cx);

                    cx.subscribe(&buffer, move |_editor, buffer, event, cx| {
                        if let BufferEvent::LanguageChanged(_) = event {
                            refresh_python_kernelspecs_for_buffer(&buffer, &project, cx);
                        }
                    })
                    .detach();
                }

                editor
                    .register_action({
                        let editor_handle = editor_handle.clone();
                        move |_: &Run, window, cx| {
                            if !JupyterSettings::enabled(cx) {
                                return;
                            }

                            crate::run(editor_handle.clone(), true, window, cx).log_err();
                        }
                    })
                    .detach();

                editor
                    .register_action({
                        move |_: &RunInPlace, window, cx| {
                            if !JupyterSettings::enabled(cx) {
                                return;
                            }

                            crate::run(editor_handle.clone(), false, window, cx).log_err();
                        }
                    })
                    .detach();
            });
        },
    )
    .detach();
}

pub struct ReplSessionsPage {
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ReplSessionsPage {
    pub fn new(window: &mut Window, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            let subscriptions = vec![
                cx.on_focus_in(&focus_handle, window, |_this, _window, cx| cx.notify()),
                cx.on_focus_out(&focus_handle, window, |_this, _event, _window, cx| {
                    cx.notify()
                }),
            ];

            Self {
                focus_handle,
                _subscriptions: subscriptions,
            }
        })
    }
}

impl EventEmitter<ItemEvent> for ReplSessionsPage {}

impl Focusable for ReplSessionsPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ReplSessionsPage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "REPL Sessions".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("REPL Session Started")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl Render for ReplSessionsPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let store = ReplStore::global(cx);

        let (kernel_specifications, sessions) = store.update(cx, |store, cx| {
            store.ensure_kernelspecs(cx);
            (
                store
                    .pure_jupyter_kernel_specifications()
                    .cloned()
                    .collect::<Vec<_>>(),
                store.sessions().cloned().collect::<Vec<_>>(),
            )
        });

        // When there are no kernel specifications, show a link to the Zed docs explaining how to
        // install kernels. It can be assumed they don't have a running kernel if we have no
        // specifications.
        if kernel_specifications.is_empty() {
            let instructions = "To start interactively running code in your editor, you need to install and configure Jupyter kernels.";

            return ReplSessionsContainer::new("No Jupyter Kernels Available")
                .child(Label::new(instructions))
                .child(
                    h_flex().w_full().p_4().justify_center().gap_2().child(
                        ButtonLike::new("install-kernels")
                            .style(ButtonStyle::Filled)
                            .size(ButtonSize::Large)
                            .layer(ElevationIndex::ModalSurface)
                            .child(Label::new("Install Kernels"))
                            .on_click(move |_, _, cx| {
                                cx.open_url(
                                    "https://zed.dev/docs/repl#language-specific-instructions",
                                )
                            }),
                    ),
                );
        }

        // When there are no sessions, show the command to run code in an editor
        if sessions.is_empty() {
            let instructions = "To run code in a Jupyter kernel, select some code and use the 'repl::Run' command.";

            return ReplSessionsContainer::new("No Jupyter Kernel Sessions").child(
                v_flex()
                    .child(Label::new(instructions))
                    .child(KeyBinding::for_action(&Run, cx)),
            );
        }

        ReplSessionsContainer::new("Jupyter Kernel Sessions").children(sessions)
    }
}

#[derive(IntoElement)]
struct ReplSessionsContainer {
    title: SharedString,
    children: Vec<AnyElement>,
}

impl ReplSessionsContainer {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for ReplSessionsContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ReplSessionsContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .p_4()
            .gap_2()
            .size_full()
            .child(Label::new(self.title).size(LabelSize::Large))
            .children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{path::PathBuf, sync::Arc};

    use async_trait::async_trait;
    use collections::HashMap;
    use editor::EditorMode;
    use gpui::TestAppContext;
    use language::{
        Language, LanguageConfig, LanguageMatcher, LanguageName, ManifestName, Toolchain,
        ToolchainList, ToolchainLister, ToolchainMetadata,
    };
    use multi_buffer::MultiBuffer;
    use task::ShellKind;
    use util::path;

    struct TestPythonToolchainLister;

    #[async_trait]
    impl ToolchainLister for TestPythonToolchainLister {
        async fn list(
            &self,
            _worktree_root: PathBuf,
            _subroot_relative_path: Arc<util::rel_path::RelPath>,
            _project_env: Option<HashMap<String, String>>,
        ) -> ToolchainList {
            ToolchainList {
                toolchains: vec![Toolchain {
                    name: SharedString::new_static("Test Python"),
                    path: SharedString::new_static("/test/python"),
                    language_name: LanguageName::new_static("Python"),
                    as_json: serde_json::Value::Null,
                }],
                ..Default::default()
            }
        }

        async fn resolve(
            &self,
            _path: PathBuf,
            _project_env: Option<HashMap<String, String>>,
        ) -> anyhow::Result<Toolchain> {
            anyhow::bail!("not implemented")
        }

        fn activation_script(
            &self,
            _toolchain: &Toolchain,
            _shell: ShellKind,
            _cx: &App,
        ) -> futures::future::BoxFuture<'static, Vec<String>> {
            Box::pin(async { Vec::new() })
        }

        fn meta(&self) -> ToolchainMetadata {
            ToolchainMetadata {
                term: SharedString::new_static("Python"),
                new_toolchain_placeholder: SharedString::default(),
                manifest_name: ManifestName::from(SharedString::new_static("pyproject.toml")),
            }
        }
    }

    #[gpui::test]
    async fn test_refreshes_python_kernelspecs_when_buffer_language_changes(
        cx: &mut TestAppContext,
    ) {
        cx.update(|cx| {
            settings::init(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = project::FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            serde_json::json!({
                "main.txt": "print('hi')",
            }),
        )
        .await;

        let project = project::Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let python = Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Python".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["py".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                None,
            )
            .with_manifest(Some(ManifestName::from(SharedString::new_static(
                "pyproject.toml",
            ))))
            .with_toolchain_lister(Some(Arc::new(TestPythonToolchainLister))),
        );
        project.read_with(cx, |project, _cx| {
            project.languages().add(python.clone());
        });

        cx.update(|cx| crate::init(fs, cx));

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/project/main.txt"), cx)
            })
            .await
            .expect("failed to open buffer");

        let worktree_id = buffer
            .read_with(cx, |buffer, cx| {
                buffer.project_path(cx).map(|path| path.worktree_id)
            })
            .expect("buffer should have a project path");

        cx.add_window(|window, cx| {
            let multi_buffer = MultiBuffer::build_from_buffer(buffer.clone(), cx);
            Editor::new(
                EditorMode::full(),
                multi_buffer,
                Some(project.clone()),
                window,
                cx,
            )
        });
        cx.run_until_parked();

        let store = cx.update(|cx| ReplStore::global(cx));
        assert!(!cx.update(|cx| store.read(cx).has_python_kernelspecs(worktree_id)));

        buffer.update(cx, |buffer, cx| {
            buffer.set_language(Some(python), cx);
        });
        cx.run_until_parked();

        assert!(cx.update(|cx| store.read(cx).has_python_kernelspecs(worktree_id)));
    }
}
