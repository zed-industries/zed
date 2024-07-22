use editor::Editor;
use gpui::{
    actions, prelude::*, AppContext, EventEmitter, FocusHandle, FocusableView, Subscription, View,
};
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
                    let extensions_page = ReplSessionsPage::new(cx);
                    workspace.add_item_to_active_pane(Box::new(extensions_page), None, true, cx)
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

                    crate::run(editor_handle.clone(), cx).log_err();
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
            return v_flex()
                .p_4()
                .size_full()
                .gap_2()
                .child(Label::new("No Jupyter Kernels Available").size(LabelSize::Large))
                .child(
                    Label::new("To start interactively running code in your editor, you need to install and configure Jupyter kernels.")
                        .size(LabelSize::Default),
                )
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
                )
                .into_any_element();
        }

        // When there are no sessions, show the command to run code in an editor
        if sessions.is_empty() {
            return v_flex()
                .p_4()
                .size_full()
                .gap_2()
                .child(Label::new("No Jupyter Kernel Sessions").size(LabelSize::Large))
                .child(
                    v_flex().child(
                        Label::new("To run code in a Jupyter kernel, select some code and use the 'repl::Run' command.")
                            .size(LabelSize::Default)
                    )
                    .children(
                            KeyBinding::for_action(&Run, cx)
                            .map(|binding|
                                binding.into_any_element()
                            )
                    )
                )
                .child(Label::new("Kernels available").size(LabelSize::Large))
                .children(
                    kernel_specifications.into_iter().map(|spec| {
                        h_flex().gap_2().child(Label::new(spec.name.clone()))
                            .child(Label::new(spec.kernelspec.language.clone()).color(Color::Muted))
                    })
                )

                .into_any_element();
        }

        v_flex()
            .p_4()
            .child(Label::new("Jupyter Kernel Sessions").size(LabelSize::Large))
            .children(
                sessions
                    .into_iter()
                    .map(|session| session.clone().into_any_element()),
            )
            .into_any_element()
    }
}
