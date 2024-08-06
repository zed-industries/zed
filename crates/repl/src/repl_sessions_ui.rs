use editor::Editor;
use gpui::{
    actions, prelude::*, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView,
    Subscription, View,
};
use ui::{prelude::*, ButtonLike, ElevationIndex, KeyBinding};
use util::ResultExt as _;
use workspace::item::ItemEvent;
use workspace::WorkspaceId;
use workspace::{item::Item, Workspace};

use crate::components::KernelListItem;
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

        let editor_handle = cx.view().downgrade();

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

        editor
            .register_action({
                let editor_handle = editor_handle.clone();
                move |_: &ClearOutputs, cx| {
                    if !JupyterSettings::enabled(cx) {
                        return;
                    }

                    crate::clear_outputs(editor_handle.clone(), cx);
                }
            })
            .detach();

        editor
            .register_action({
                let editor_handle = editor_handle.clone();
                move |_: &Interrupt, cx| {
                    if !JupyterSettings::enabled(cx) {
                        return;
                    }

                    crate::interrupt(editor_handle.clone(), cx);
                }
            })
            .detach();

        editor
            .register_action({
                let editor_handle = editor_handle.clone();
                move |_: &Shutdown, cx| {
                    if !JupyterSettings::enabled(cx) {
                        return;
                    }

                    crate::shutdown(editor_handle.clone(), cx);
                }
            })
            .detach();
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
                store.kernel_specifications().cloned().collect::<Vec<_>>(),
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
                                    "https://docs.jupyter.org/en/latest/install/kernels.html",
                                )
                            }),
                    ),
                );
        }

        // When there are no sessions, show the command to run code in an editor
        if sessions.is_empty() {
            let instructions = "To run code in a Jupyter kernel, select some code and use the 'repl::Run' command.";

            return ReplSessionsContainer::new("No Jupyter Kernel Sessions")
                .child(
                    v_flex()
                        .child(Label::new(instructions))
                        .children(KeyBinding::for_action(&Run, cx)),
                )
                .child(Label::new("Kernels available").size(LabelSize::Large))
                .children(kernel_specifications.into_iter().map(|spec| {
                    KernelListItem::new(spec.clone()).child(
                        h_flex()
                            .gap_2()
                            .child(Label::new(spec.name))
                            .child(Label::new(spec.kernelspec.language).color(Color::Muted)),
                    )
                }));
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
