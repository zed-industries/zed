use editor::Editor;
use gpui::{
    actions, prelude::*, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView,
    Subscription, View,
};
use project::ProjectItem as _;
use ui::{prelude::*, ButtonLike, ElevationIndex, KeyBinding};
use util::ResultExt as _;
use workspace::item::ItemEvent;
use workspace::WorkspaceId;
use workspace::{item::Item, Workspace};

use crate::jupyter_settings::JupyterSettings;
use crate::repl_store::ReplStore;

actions!(
    repl,
    [
        Run,
        RunInPlace,
        ClearOutputs,
        Sessions,
        Interrupt,
        Shutdown,
        Restart,
        RefreshKernelspecs
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &Sessions, cx| {
                let existing = workspace
                    .active_pane()
                    .read(cx)
                    .items()
                    .find_map(|item| item.downcast::<ReplSessionsPage>());

                if let Some(existing) = existing {
                    workspace.activate_item(&existing, true, true, cx);
                } else {
                    let repl_sessions_page = ReplSessionsPage::new(cx);
                    workspace.add_item_to_active_pane(Box::new(repl_sessions_page), None, true, cx)
                }
            });

            workspace.register_action(|_workspace, _: &RefreshKernelspecs, cx| {
                let store = ReplStore::global(cx);
                store.update(cx, |store, cx| {
                    store.refresh_kernelspecs(cx).detach();
                });
            });
        },
    )
    .detach();

    cx.observe_new_views(move |editor: &mut Editor, cx: &mut ViewContext<Editor>| {
        if !editor.use_modal_editing() || !editor.buffer().read(cx).is_singleton() {
            return;
        }

        cx.defer(|editor, cx| {
            let workspace = Workspace::for_window(cx);
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

            let editor_handle = cx.view().downgrade();

            if let Some(language) = language {
                if language.name() == "Python".into() {
                    if let (Some(project_path), Some(project)) = (project_path, project) {
                        let store = ReplStore::global(cx);
                        store.update(cx, |store, cx| {
                            store
                                .refresh_python_kernelspecs(project_path.worktree_id, &project, cx)
                                .detach_and_log_err(cx);
                        });
                    }
                }
            }

            editor
                .register_action({
                    let editor_handle = editor_handle.clone();
                    move |_: &Run, cx| {
                        if !JupyterSettings::enabled(cx) {
                            return;
                        }

                        crate::run(editor_handle.clone(), true, cx).log_err();
                    }
                })
                .detach();

            editor
                .register_action({
                    let editor_handle = editor_handle.clone();
                    move |_: &RunInPlace, cx| {
                        if !JupyterSettings::enabled(cx) {
                            return;
                        }

                        crate::run(editor_handle.clone(), false, cx).log_err();
                    }
                })
                .detach();
        });
    })
    .detach();
}

pub struct ReplSessionsPage {
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ReplSessionsPage {
    pub fn new(cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();

            let subscriptions = vec![
                cx.on_focus_in(&focus_handle, |_this, cx| cx.notify()),
                cx.on_focus_out(&focus_handle, |_this, _event, cx| cx.notify()),
            ];

            Self {
                focus_handle,
                _subscriptions: subscriptions,
            }
        })
    }
}

impl EventEmitter<ItemEvent> for ReplSessionsPage {}

impl FocusableView for ReplSessionsPage {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ReplSessionsPage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("REPL Sessions".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("repl sessions")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        None
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl Render for ReplSessionsPage {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                            .on_click(move |_, cx| {
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
                    .children(KeyBinding::for_action(&Run, cx)),
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
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .p_4()
            .gap_2()
            .size_full()
            .child(Label::new(self.title).size(LabelSize::Large))
            .children(self.children)
    }
}
