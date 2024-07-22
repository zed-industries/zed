use crate::repl_store::ReplStore;
use crate::{
    jupyter_settings::{JupyterDockPosition, JupyterSettings},
    kernels::{kernel_specifications, KernelSpecification},
    session::{Session, SessionEvent},
};
use anyhow::{Context as _, Result};
use collections::HashMap;
use editor::{Anchor, Editor, RangeToAnchorExt};
use gpui::{
    actions, prelude::*, AppContext, AsyncWindowContext, EntityId, EventEmitter, FocusHandle,
    FocusOutEvent, FocusableView, Subscription, Task, View, WeakView, WindowContext,
};
use language::{Language, Point};
use multi_buffer::MultiBufferRow;
use project::Fs;
use settings::{Settings as _, SettingsStore};
use std::{ops::Range, sync::Arc};
use ui::{prelude::*, ButtonLike, ElevationIndex, KeyBinding};
use util::ResultExt as _;
use workspace::{
    dock::{Panel, PanelEvent},
    Workspace,
};

actions!(
    repl,
    [Run, ClearOutputs, Interrupt, Shutdown, RefreshKernelspecs]
);
actions!(repl_panel, [ToggleFocus]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<RuntimePanel>(cx);
            });

            workspace.register_action(|workspace, _: &RefreshKernelspecs, cx| {
                let store = ReplStore::global(cx);
                store.update(cx, |store, cx| {
                    store.refresh_kernelspecs(cx).detach();
                });
            });
        },
    )
    .detach();

    cx.observe_new_views(move |editor: &mut Editor, cx: &mut ViewContext<Editor>| {
        // Only allow editors that support vim mode and are singleton buffers
        if !editor.use_modal_editing() || !editor.buffer().read(cx).is_singleton() {
            return;
        }

        editor
            .register_action(cx.listener(
                move |editor: &mut Editor, _: &Run, cx: &mut ViewContext<Editor>| {
                    if !JupyterSettings::enabled(cx) {
                        return;
                    }
                    let Some(workspace) = editor.workspace() else {
                        return;
                    };
                    let Some(panel) = workspace.read(cx).panel::<RuntimePanel>(cx) else {
                        return;
                    };
                    let weak_editor = cx.view().downgrade();
                    panel.update(cx, |_, cx| {
                        cx.defer(|panel, cx| {
                            let store = ReplStore::global(cx);
                            store.update(cx, |store, cx| {
                                store.run(weak_editor, cx).log_err();
                            });
                        });
                    })
                },
            ))
            .detach();

        editor
            .register_action(cx.listener(
                move |editor: &mut Editor, _: &ClearOutputs, cx: &mut ViewContext<Editor>| {
                    if !JupyterSettings::enabled(cx) {
                        return;
                    }
                    let Some(workspace) = editor.workspace() else {
                        return;
                    };
                    let Some(panel) = workspace.read(cx).panel::<RuntimePanel>(cx) else {
                        return;
                    };
                    let weak_editor = cx.view().downgrade();
                    panel.update(cx, |_, cx| {
                        cx.defer(|panel, cx| {
                            panel.clear_outputs(weak_editor, cx);
                        });
                    })
                },
            ))
            .detach();

        editor
            .register_action(cx.listener(
                move |editor: &mut Editor, _: &Interrupt, cx: &mut ViewContext<Editor>| {
                    if !JupyterSettings::enabled(cx) {
                        return;
                    }
                    let Some(workspace) = editor.workspace() else {
                        return;
                    };
                    let Some(panel) = workspace.read(cx).panel::<RuntimePanel>(cx) else {
                        return;
                    };
                    let weak_editor = cx.view().downgrade();
                    panel.update(cx, |_, cx| {
                        cx.defer(|panel, cx| {
                            panel.interrupt(weak_editor, cx);
                        });
                    })
                },
            ))
            .detach();

        editor
            .register_action(cx.listener(
                move |editor: &mut Editor, _: &Shutdown, cx: &mut ViewContext<Editor>| {
                    if !JupyterSettings::enabled(cx) {
                        return;
                    }
                    let Some(workspace) = editor.workspace() else {
                        return;
                    };
                    let Some(panel) = workspace.read(cx).panel::<RuntimePanel>(cx) else {
                        return;
                    };
                    let weak_editor = cx.view().downgrade();
                    panel.update(cx, |_, cx| {
                        cx.defer(|panel, cx| {
                            panel.shutdown(weak_editor, cx);
                        });
                    })
                },
            ))
            .detach();
    })
    .detach();
}

pub struct RuntimePanel {
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl RuntimePanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let view = workspace.update(&mut cx, |workspace, cx| {
                cx.new_view::<Self>(|cx| {
                    let focus_handle = cx.focus_handle();

                    let fs = workspace.app_state().fs.clone();

                    let subscriptions = vec![
                        cx.on_focus_in(&focus_handle, Self::focus_in),
                        cx.on_focus_out(&focus_handle, Self::focus_out),
                    ];

                    let runtime_panel = Self {
                        fs,
                        width: None,
                        focus_handle,
                        _subscriptions: subscriptions,
                    };

                    runtime_panel
                })
            })?;

            view.update(&mut cx, |this, cx| this.refresh_kernelspecs(cx))?
                .await?;

            Ok(view)
        })
    }

    fn focus_in(&mut self, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn focus_out(&mut self, _event: FocusOutEvent, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl Panel for RuntimePanel {
    fn persistent_name() -> &'static str {
        "RuntimePanel"
    }

    fn position(&self, cx: &ui::WindowContext) -> workspace::dock::DockPosition {
        match JupyterSettings::get_global(cx).dock {
            JupyterDockPosition::Left => workspace::dock::DockPosition::Left,
            JupyterDockPosition::Right => workspace::dock::DockPosition::Right,
            JupyterDockPosition::Bottom => workspace::dock::DockPosition::Bottom,
        }
    }

    fn position_is_valid(&self, _position: workspace::dock::DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        cx: &mut ViewContext<Self>,
    ) {
        settings::update_settings_file::<JupyterSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                workspace::dock::DockPosition::Left => JupyterDockPosition::Left,
                workspace::dock::DockPosition::Right => JupyterDockPosition::Right,
                workspace::dock::DockPosition::Bottom => JupyterDockPosition::Bottom,
            };
            settings.set_dock(dock);
        })
    }

    fn size(&self, cx: &ui::WindowContext) -> Pixels {
        let settings = JupyterSettings::get_global(cx);

        self.width.unwrap_or(settings.default_width)
    }

    fn set_size(&mut self, size: Option<ui::Pixels>, _cx: &mut ViewContext<Self>) {
        self.width = size;
    }

    fn icon(&self, cx: &ui::WindowContext) -> Option<ui::IconName> {
        let store = ReplStore::global(cx);

        if !store.read(cx).is_enabled() {
            return None;
        }

        Some(IconName::Code)
    }

    fn icon_tooltip(&self, _cx: &ui::WindowContext) -> Option<&'static str> {
        Some("Runtime Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for RuntimePanel {}

impl FocusableView for RuntimePanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RuntimePanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let store = ReplStore::global(cx);

        let (kernel_specifications, sessions) = store.update(cx, |store, cx| {
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
