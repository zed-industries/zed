use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ParentElement,
    Render, SharedString, Styled, Window, rems,
};
use ui::{Headline, HeadlineSize, prelude::*};
use workspace::ModalView;

use crate::endpoint_store::{DockerAction, DockerEndpointStore};

/// Emitted when the user picks Confirm or Cancel, for observers (e.g. tests)
/// that want to know the outcome without inspecting the store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmModalEvent {
    Confirmed,
    Cancelled,
}

/// Confirmation modal shown before running a destructive Docker action.
///
/// Displays the exact command that will run and the endpoint it targets, so
/// the user can see precisely what is about to execute (mirrors
/// `database_ui::ConnectionModal`'s `ManagedView` structure, which similarly
/// performs its side effect directly from a button handler rather than
/// emitting an event for some other entity to act on).
///
/// On Confirm, the modal itself calls into [`DockerEndpointStore::dispatch_action`]
/// — never before. [`DockerEndpointStore::dispatch_action`] re-checks
/// `endpoint.read_only` on its own, so even if this modal were somehow shown
/// for a read-only endpoint, the client would still not be called.
pub struct ConfirmModal {
    command: SharedString,
    endpoint_name: SharedString,
    action: DockerAction,
    store: Entity<DockerEndpointStore>,
    focus_handle: FocusHandle,
}

impl ConfirmModal {
    pub fn new(
        endpoint_name: impl Into<SharedString>,
        action: DockerAction,
        store: Entity<DockerEndpointStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let endpoint_name = endpoint_name.into();
        Self {
            command: action.command_string().into(),
            endpoint_name,
            action,
            store,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn confirm(&mut self, cx: &mut Context<Self>) {
        self.store.update(cx, |store, cx| {
            store.dispatch_action(&self.endpoint_name, self.action.clone(), cx);
        });
        cx.emit(ConfirmModalEvent::Confirmed);
        cx.emit(DismissEvent);
    }

    pub fn cancel(&mut self, cx: &mut Context<Self>) {
        cx.emit(ConfirmModalEvent::Cancelled);
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ConfirmModal {}
impl EventEmitter<ConfirmModalEvent> for ConfirmModal {}
impl ModalView for ConfirmModal {}

impl Focusable for ConfirmModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ConfirmModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("ConfirmModal")
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .w(rems(28.))
            .child(
                h_flex()
                    .p_3()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(Headline::new("Confirm Action").size(HeadlineSize::XSmall)),
            )
            .child(
                v_flex()
                    .p_3()
                    .gap_2()
                    .child(
                        Label::new(format!("{}  on  {}", self.command, self.endpoint_name))
                            .buffer_font(cx),
                    )
                    .child(
                        Label::new("This action cannot be undone.")
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
            )
            .child(
                h_flex()
                    .p_3()
                    .gap_2()
                    .justify_end()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Button::new("confirm-cancel", "Cancel").on_click(cx.listener(
                            |this, _, _window, cx| {
                                this.cancel(cx);
                            },
                        )),
                    )
                    .child(
                        Button::new("confirm-confirm", "Confirm")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.confirm(cx);
                            })),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use docker_client::DockerClient;
    use docker_client::fake::FakeDockerClient;
    use gpui::TestAppContext;
    use std::sync::Arc;

    use crate::endpoint_store::ClientFactory;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            gpui_tokio::init(cx);
        });
    }

    async fn wait_until(cx: &mut TestAppContext, condition: impl Fn(&mut TestAppContext) -> bool) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.executor()
                .timer(std::time::Duration::from_millis(5))
                .await;
        }
        cx.run_until_parked();
        assert!(
            condition(cx),
            "condition did not become true within the time bound"
        );
    }

    #[gpui::test]
    async fn confirm_dispatches_the_action_to_the_store(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory: ClientFactory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let modal = cx.new(|cx| {
            ConfirmModal::new(
                "local",
                DockerAction::RestartContainer { id: "api".into() },
                store.clone(),
                cx,
            )
        });

        modal.update(cx, |modal, cx| modal.confirm(cx));
        wait_until(cx, |_| {
            fake.calls()
                .iter()
                .any(|c| c.starts_with("restart_container"))
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|c| c.starts_with("restart_container local api")),
            "confirm should dispatch restart_container with the right endpoint/id; calls: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    fn cancel_emits_cancelled_without_dispatching(cx: &mut TestAppContext) {
        init_test(cx);
        let fake = Arc::new(FakeDockerClient::new());
        let factory: ClientFactory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let modal = cx.new(|cx| {
            ConfirmModal::new(
                "local",
                DockerAction::RestartContainer { id: "api".into() },
                store.clone(),
                cx,
            )
        });
        let events = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        cx.update(|cx| {
            let events = events.clone();
            cx.subscribe(&modal, move |_modal, event: &ConfirmModalEvent, _cx| {
                events.borrow_mut().push(*event);
            })
            .detach();
        });
        modal.update(cx, |modal, cx| modal.cancel(cx));
        assert_eq!(events.borrow().as_slice(), [ConfirmModalEvent::Cancelled]);
        assert!(
            !fake
                .calls()
                .iter()
                .any(|c| c.starts_with("restart_container")),
            "cancel must not dispatch the action"
        );
    }
}
