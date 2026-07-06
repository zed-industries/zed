use std::sync::Arc;

use docker_client::{DockerClient, DockerEndpoint};
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, ParentElement, SharedString,
    Styled, Task, Window,
};
use ui::prelude::*;
use workspace::{Workspace, item::Item};

enum InspectState {
    Loading,
    Loaded(String),
    Error(String),
}

/// A full-size center-pane tab showing raw `docker inspect` JSON for one
/// container: read-only, scrollable, text-selectable. A plain scroll
/// container of monospaced text is enough here, so this deliberately avoids
/// pulling in the `editor` crate.
pub struct DockerInspectView {
    focus_handle: FocusHandle,
    container_name: String,
    state: InspectState,
    _inspect_task: Task<()>,
}

impl DockerInspectView {
    pub fn new(
        endpoint: DockerEndpoint,
        container_id: String,
        container_name: String,
        client: Arc<dyn DockerClient>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let task = gpui_tokio::Tokio::spawn_result(cx, async move {
                client.inspect_container(&endpoint, &container_id).await
            });
            let inspect_task = cx.spawn(async move |this, cx| {
                let result = task.await;
                this.update(cx, |this: &mut Self, cx| {
                    this.state = match result {
                        Ok(json) => InspectState::Loaded(json),
                        Err(error) => InspectState::Error(format!("{error:#}")),
                    };
                    cx.notify();
                })
                .ok();
            });

            Self {
                focus_handle: cx.focus_handle(),
                container_name,
                state: InspectState::Loading,
                _inspect_task: inspect_task,
            }
        })
    }

    /// Exposed for tests.
    pub fn json(&self) -> Option<&str> {
        match &self.state {
            InspectState::Loaded(json) => Some(json),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&str> {
        match &self.state {
            InspectState::Error(message) => Some(message),
            _ => None,
        }
    }
}

impl Render for DockerInspectView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = match &self.state {
            InspectState::Loading => v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("Loading…").color(Color::Muted))
                .into_any_element(),
            InspectState::Error(message) => v_flex()
                .size_full()
                .p_4()
                .child(Label::new(message.clone()).color(Color::Error))
                .into_any_element(),
            InspectState::Loaded(json) => v_flex()
                .id("docker-inspect-scroll")
                .size_full()
                .overflow_y_scroll()
                .p_2()
                .child(
                    Label::new(json.clone())
                        .buffer_font(cx)
                        .size(LabelSize::Small),
                )
                .into_any_element(),
        };

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new(format!("Inspect: {}", self.container_name))),
            )
            .child(v_flex().flex_1().size_full().overflow_hidden().child(body))
    }
}

impl Focusable for DockerInspectView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for DockerInspectView {}

impl Item for DockerInspectView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileTextOutlined))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("Inspect: {}", self.container_name).into()
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        false
    }
}

/// Opens a new inspect tab in the workspace's active pane, mirroring
/// `open_logs_tab`.
pub fn open_inspect_tab(
    workspace: &mut Workspace,
    endpoint: DockerEndpoint,
    container_id: String,
    container_name: String,
    client: Arc<dyn DockerClient>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let view = DockerInspectView::new(endpoint, container_id, container_name, client, cx);
    workspace.active_pane().update(cx, |pane, cx| {
        pane.add_item(Box::new(view), true, true, None, window, cx);
    });
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use docker_client::fake::FakeDockerClient;
    use docker_client::{DockerClient, EndpointKind};
    use gpui::{TestAppContext, VisualTestContext};

    use super::DockerInspectView;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            gpui_tokio::init(cx);
            crate::init(cx);
        });
    }

    fn test_endpoint() -> docker_client::DockerEndpoint {
        docker_client::DockerEndpoint {
            name: "local".into(),
            kind: EndpointKind::Local,
            read_only: false,
        }
    }

    async fn wait_until(
        cx: &mut VisualTestContext,
        condition: impl Fn(&mut VisualTestContext) -> bool,
    ) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.background_executor
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
    async fn inspect_fetches_and_stores_json(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.inspect = "{\"Id\": \"api-id\"}".into();
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|_window, cx| {
            DockerInspectView::new(test_endpoint(), "api-id".into(), "api".into(), client, cx)
        });

        wait_until(cx, |cx| view.read_with(cx, |view, _| view.json().is_some())).await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.json(), Some("{\"Id\": \"api-id\"}"));
            assert_eq!(view.error(), None);
        });
    }

    #[gpui::test]
    async fn inspect_error_surfaces_message(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fake = FakeDockerClient::with_error("boom");
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|_window, cx| {
            DockerInspectView::new(test_endpoint(), "api-id".into(), "api".into(), client, cx)
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.error().is_some())
        })
        .await;

        view.read_with(cx, |view, _| {
            let message = view.error().expect("error should be surfaced");
            assert!(message.contains("boom"), "unexpected error: {message}");
            assert!(view.json().is_none());
        });
    }
}
