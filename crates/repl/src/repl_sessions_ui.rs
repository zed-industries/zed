use editor::Editor;
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, actions,
    prelude::*,
};
use project::ProjectItem as _;
use ui::{ButtonLike, ElevationIndex, KeyBinding, prelude::*};
use util::ResultExt as _;
use workspace::item::ItemEvent;
use workspace::{Workspace, item::Item};

use crate::jupyter_settings::JupyterSettings;
use crate::repl_store::ReplStore;

actions!(
    repl,
    [
        /// Runs the current cell and advances to the next one.
        Run,
        /// Runs the current cell without advancing.
        RunInPlace,
        /// Clears all outputs in the REPL.
        ClearOutputs,
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

            cx.defer_in(window, |editor, window, cx| {
                let workspace = Workspace::for_window(window, cx);
                let project = workspace.map(|workspace| workspace.read(cx).project().clone());

                let is_local_project = project
                    .as_ref()
                    .map(|project| project.read(cx).is_local())
                    .unwrap_or(false);

                if !is_local_project {
                    return;
                }

                let buffer = editor.buffer().read(cx).as_singleton();

                let language = buffer
                    .as_ref()
                    .and_then(|buffer| buffer.read(cx).language());

                let project_path = buffer.and_then(|buffer| buffer.read(cx).project_path(cx));

                let editor_handle = cx.entity().downgrade();

                if let Some(language) = language
                    && language.name() == "Python".into()
                    && let (Some(project_path), Some(project)) = (project_path, project)
                {
                    let store = ReplStore::global(cx);
                    store.update(cx, |store, cx| {
                        store
                            .refresh_python_kernelspecs(project_path.worktree_id, &project, cx)
                            .detach_and_log_err(cx);
                    });
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

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl Render for ReplSessionsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let store = ReplStore::global(cx);

        let (kernel_specifications, sessions) = store.update(cx, |store, _cx| {
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
                    .children(KeyBinding::for_action(&Run, window, cx)),
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
