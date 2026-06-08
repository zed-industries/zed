use context_server::ContextServerId;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle, Window, prelude::*,
};
use project::context_server_store::ContextServerStore;
use ui::{
    CommonAnimationExt, Divider, DividerColor, KeyBinding, Modal, ModalFooter, ModalHeader,
    Section, WithScrollbar, prelude::*,
};
use workspace::ModalView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Idle,
    Waiting,
    Error(SharedString),
}

pub struct ScopeUpgradeModal {
    server_id: ContextServerId,
    context_server_store: Entity<ContextServerStore>,
    existing_scopes: Vec<String>,
    required_scopes: Vec<String>,
    state: State,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
}

impl ScopeUpgradeModal {
    pub fn new(
        server_id: ContextServerId,
        context_server_store: Entity<ContextServerStore>,
        existing_scopes: Vec<String>,
        required_scopes: Vec<String>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            server_id,
            context_server_store,
            existing_scopes,
            required_scopes,
            state: State::Idle,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }

    fn render_modal_header(&self) -> ModalHeader {
        ModalHeader::new()
            .headline(format!("Upgrade Permissions: {}", self.server_id.0))
            .show_dismiss_button(true)
    }

    fn render_modal_description(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        Label::new("This MCP Server requires new scopes to complete the action.")
            .color(Color::Muted)
            .into_any_element()
    }

    fn render_modal_content(&self, _cx: &App) -> AnyElement {
        v_flex()
            .gap_4()
            .py_2()
            .child(
                v_flex()
                    .gap_1p5()
                    .child(
                        Label::new("Previously granted:")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .children(if self.existing_scopes.is_empty() {
                        vec![
                            Label::new("None")
                                .color(Color::Muted)
                                .size(LabelSize::Small)
                                .into_any_element(),
                        ]
                    } else {
                        self.existing_scopes
                            .iter()
                            .map(|scope| {
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(IconName::Check).size(IconSize::XSmall))
                                    .child(Label::new(scope.clone()).size(LabelSize::Small))
                                    .into_any_element()
                            })
                            .collect::<Vec<_>>()
                    }),
            )
            .child(Divider::horizontal().color(DividerColor::BorderVariant))
            .child(
                v_flex()
                    .gap_1p5()
                    .child(Label::new("Newly requested:").size(LabelSize::Small))
                    .children(self.required_scopes.iter().map(|scope| {
                        h_flex()
                            .gap_2()
                            .child(Icon::new(IconName::Plus).size(IconSize::XSmall))
                            .child(Label::new(scope.clone()).size(LabelSize::Small))
                            .into_any_element()
                    })),
            )
            .children(match &self.state {
                State::Waiting => Some(
                    h_flex()
                        .w_full()
                        .justify_center()
                        .gap_2()
                        .py_2()
                        .child(
                            Icon::new(IconName::LoadCircle)
                                .size(IconSize::Small)
                                .color(Color::Muted)
                                .with_rotate_animation(3),
                        )
                        .child(
                            Label::new("Please complete authentication in your browser...")
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        ),
                ),
                State::Error(err) => Some(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .py_2()
                        .child(
                            Icon::new(IconName::Warning)
                                .size(IconSize::Small)
                                .color(Color::Error),
                        )
                        .child(
                            Label::new(err.clone())
                                .color(Color::Error)
                                .size(LabelSize::Small),
                        ),
                ),
                State::Idle => None,
            })
            .into_any_element()
    }

    fn render_modal_footer(&self, cx: &mut Context<Self>) -> ModalFooter {
        let focus_handle = self.focus_handle(cx);
        let is_busy = matches!(self.state, State::Waiting);

        ModalFooter::new().end_slot(
            h_flex()
                .gap_2()
                .child(
                    Button::new("cancel", "Cancel")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Cancel, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(cx.listener(|this, _event, window, cx| {
                            this.cancel(&menu::Cancel, window, cx);
                        })),
                )
                .child(
                    Button::new("accept-scopes", "Accept New Scopes")
                        .style(ButtonStyle::Filled)
                        .disabled(is_busy)
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(cx.listener(|this, _event, _window, cx| {
                            this.state = State::Waiting;
                            cx.notify();

                            let store = this.context_server_store.clone();
                            let server_id = this.server_id.clone();
                            let result = store
                                .update(cx, |store, cx| store.authenticate_server(&server_id, cx));
                            if let Err(err) = result {
                                this.state = State::Error(
                                    format!("Failed to start authentication: {err}").into(),
                                );
                                cx.notify();
                            }
                            let wait_for_context_server_task =
                                wait_for_context_server(&store, server_id.clone(), cx);

                            cx.spawn({
                                async move |this, cx| {
                                    let result = wait_for_context_server_task.await;

                                    this.update(cx, |this, cx| match result {
                                        Ok(project::context_server_store::ContextServerStatus::Running) => {
                                            cx.emit(DismissEvent);
                                        }
                                        Err(err) => {
                                            this.state = State::Error(err.into());
                                            cx.notify();
                                        }
                                        _ => {}
                                    }).ok();
                                }
                            })
                            .detach();
                        })),
                ),
        )
    }

    pub fn show_modal(
        server_id: context_server::ContextServerId,
        context_server_store: gpui::Entity<project::context_server_store::ContextServerStore>,
        existing_scopes: Vec<String>,
        required_scopes: Vec<String>,
        workspace: gpui::WeakEntity<workspace::Workspace>,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> gpui::Task<anyhow::Result<()>> {
        window.spawn(cx, async move |cx| {
            workspace.update_in(cx, |workspace, window, cx| {
                workspace.toggle_modal(window, cx, |window, cx| {
                    Self::new(
                        server_id,
                        context_server_store,
                        existing_scopes,
                        required_scopes,
                        window,
                        cx,
                    )
                })
            })
        })
    }
}

impl ModalView for ScopeUpgradeModal {}

impl Focusable for ScopeUpgradeModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ScopeUpgradeModal {}

impl Render for ScopeUpgradeModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .elevation_3(cx)
            .w(rems(36.))
            .key_context("ScopeUpgradeModal")
            .on_action(cx.listener(|this, action: &menu::Cancel, window, cx| {
                this.cancel(action, window, cx)
            }))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window, cx);
            }))
            .child(
                Modal::new("upgrade-context-server-scopes", None)
                    .header(self.render_modal_header())
                    .section(
                        Section::new().child(
                            div()
                                .id("modal-content")
                                .max_h(vh(0.6, window))
                                .overflow_y_scroll()
                                .track_scroll(&self.scroll_handle)
                                .child(self.render_modal_description(window, cx))
                                .child(self.render_modal_content(cx))
                                .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                        ),
                    )
                    .footer(self.render_modal_footer(cx)),
            )
    }
}

fn wait_for_context_server(
    context_server_store: &gpui::Entity<ContextServerStore>,
    context_server_id: ContextServerId,
    cx: &mut gpui::App,
) -> gpui::Task<Result<project::context_server_store::ContextServerStatus, std::sync::Arc<str>>> {
    use parking_lot::Mutex;
    use project::context_server_store::{ContextServerStatus, ServerStatusChangedEvent};
    use std::sync::Arc;
    use std::time::Duration;

    const WAIT_TIMEOUT: Duration = Duration::from_secs(120);

    let (tx, rx) = futures::channel::oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let context_server_id_for_timeout = context_server_id.clone();
    let subscription = cx.subscribe(context_server_store, move |_, event, _cx| {
        let ServerStatusChangedEvent { server_id, status } = event;

        if server_id != &context_server_id {
            return;
        }

        match status {
            ContextServerStatus::Running | ContextServerStatus::AuthRequired => {
                if let Some(tx) = tx.lock().take() {
                    let _ = tx.send(Ok(status.clone()));
                }
            }
            ContextServerStatus::Stopped => {
                log::debug!("Context server is restarting.");
            }
            ContextServerStatus::Error(error) => {
                if let Some(tx) = tx.lock().take() {
                    let _ = tx.send(Err(error.clone()));
                }
            }
            ContextServerStatus::Starting
            | ContextServerStatus::Authenticating
            | ContextServerStatus::InsufficientScope {
                existing_scopes: _,
                required_scopes: _,
            }
            | ContextServerStatus::ClientSecretRequired { .. } => {}
        }
    });

    cx.spawn(async move |cx| {
        let timeout = cx.background_executor().timer(WAIT_TIMEOUT);
        let result = futures::future::select(rx, timeout).await;
        drop(subscription);
        match result {
            futures::future::Either::Left((Ok(inner), _)) => inner,
            futures::future::Either::Left((Err(_), _)) => {
                Err(Arc::from("Context server store was dropped"))
            }
            futures::future::Either::Right(_) => Err(Arc::from(format!(
                "Timed out waiting for context server `{}` to start. Check the Zed log for details.",
                context_server_id_for_timeout
            ))),
        }
    })
}
