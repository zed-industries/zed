use std::sync::Arc;

use docker_client::{DockerClient, DockerEndpoint};
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, ParentElement, ScrollHandle,
    SharedString, Styled, Task, Window,
};
use ui::{Tooltip, prelude::*};
use workspace::{Workspace, item::Item};

/// The tail of streamed log lines is capped at this many lines, evicting the
/// oldest lines past the cap regardless of the follow/pause toggle.
const MAX_DISPLAYED_LOG_LINES: usize = 500;
const TAIL: usize = 200;

/// A full-size center-pane tab streaming `docker logs -f` for one container.
///
/// Owns the stream for as long as the tab is open: dropping this view (e.g.
/// the user closes the tab) drops `_logs_task`, which cancels the background
/// `cx.spawn` future and, with it, the receiver from
/// `CliDockerClient::container_logs`. That receiver's drop is what lets the
/// `docker logs -f` child exit via `kill_on_drop`.
pub struct DockerLogsView {
    focus_handle: FocusHandle,
    container_name: String,
    lines: Vec<String>,
    follow: bool,
    scroll_handle: ScrollHandle,
    _logs_task: Task<()>,
}

impl DockerLogsView {
    pub fn new(
        endpoint: DockerEndpoint,
        container_id: String,
        container_name: String,
        client: Arc<dyn DockerClient>,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let task = gpui_tokio::Tokio::spawn_result(cx, async move {
                client.container_logs(&endpoint, &container_id, TAIL).await
            });
            let logs_task = cx.spawn(async move |this, cx| {
                use futures::StreamExt as _;

                let mut receiver = match task.await {
                    Ok(receiver) => receiver,
                    Err(error) => {
                        log::warn!("docker logs failed: {error:#}");
                        return;
                    }
                };
                while let Some(chunk) = receiver.next().await {
                    let updated = this.update(cx, |this: &mut Self, cx| {
                        this.lines.push(chunk.line);
                        if this.lines.len() > MAX_DISPLAYED_LOG_LINES {
                            let overflow = this.lines.len() - MAX_DISPLAYED_LOG_LINES;
                            this.lines.drain(0..overflow);
                        }
                        // Only pull the view down to the tail while
                        // following; paused, the scroll position is left
                        // alone so the user can read back through history
                        // without it jumping. The buffer stays capped at
                        // MAX_DISPLAYED_LOG_LINES regardless of follow state.
                        if this.follow {
                            this.scroll_handle.scroll_to_bottom();
                        }
                        cx.notify();
                    });
                    if updated.is_err() {
                        return;
                    }
                }
            });

            Self {
                focus_handle: cx.focus_handle(),
                container_name,
                lines: Vec::new(),
                follow: true,
                scroll_handle: ScrollHandle::new(),
                _logs_task: logs_task,
            }
        })
    }

    /// Exposed for tests, mirroring `SqlQueryView::result`.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn follow(&self) -> bool {
        self.follow
    }

    /// Flips the follow/pause toggle. Switching back to follow immediately
    /// snaps to the tail; pausing leaves the scroll position where the user
    /// left it.
    pub fn toggle_follow(&mut self, cx: &mut Context<Self>) {
        self.follow = !self.follow;
        if self.follow {
            self.scroll_handle.scroll_to_bottom();
        }
        cx.notify();
    }

    fn render_toolbar(&self, cx: &Context<Self>) -> gpui::AnyElement {
        let follow_label = if self.follow { "Following" } else { "Paused" };
        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(Label::new(format!("Logs: {}", self.container_name)))
            .child(
                Button::new("toggle-follow-logs", follow_label)
                    .toggle_state(self.follow)
                    .tooltip(Tooltip::text("Toggle follow/pause"))
                    .on_click(cx.listener(|this, _, _window, cx| this.toggle_follow(cx))),
            )
            .into_any_element()
    }
}

impl Render for DockerLogsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_toolbar(cx))
            .child(
                v_flex()
                    .id("docker-logs-scroll")
                    .flex_1()
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .p_2()
                    .children(self.lines.iter().map(|line| {
                        Label::new(line.clone())
                            .buffer_font(cx)
                            .size(LabelSize::Small)
                            .into_any_element()
                    })),
            )
    }
}

impl Focusable for DockerLogsView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for DockerLogsView {}

impl Item for DockerLogsView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Terminal))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("Logs: {}", self.container_name).into()
    }

    fn is_dirty(&self, _cx: &App) -> bool {
        false
    }
}

/// Opens a new logs tab in the workspace's active pane. Always opens a new
/// tab rather than focusing an existing one for the same container: simpler,
/// and acceptable per the brief (focusing an existing tab is a nicer-to-have,
/// not required).
pub fn open_logs_tab(
    workspace: &mut Workspace,
    endpoint: DockerEndpoint,
    container_id: String,
    container_name: String,
    client: Arc<dyn DockerClient>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let view = DockerLogsView::new(endpoint, container_id, container_name, client, cx);
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

    use super::DockerLogsView;

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

    /// Drives the deterministic scheduler while giving the real tokio runtime
    /// a chance to complete cross-thread work, until `condition` holds or a
    /// bound is reached. Requires `cx.executor().allow_parking()`.
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

    /// The logs buffer must fill from the streamed `LogChunk`s the client
    /// yields, in order, and follow defaults to true.
    #[gpui::test]
    async fn logs_stream_fills_buffer(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.log_lines = vec!["a".into(), "b".into()];
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|_window, cx| {
            DockerLogsView::new(test_endpoint(), "api-id".into(), "api".into(), client, cx)
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.lines() == ["a".to_string(), "b".to_string()]
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.lines(), ["a".to_string(), "b".to_string()]);
            assert!(view.follow(), "follow should default to true");
        });
    }

    /// Toggling follow/pause must flip the observable `follow` flag; this is
    /// the behavior the toggle button in the view's toolbar drives.
    #[gpui::test]
    async fn toggle_logs_follow_flips_flag(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.log_lines = vec!["a".into()];
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|_window, cx| {
            DockerLogsView::new(test_endpoint(), "api-id".into(), "api".into(), client, cx)
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| !view.lines().is_empty())
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.follow(), "follow should default to true");
        });

        view.update(cx, |view, cx| view.toggle_follow(cx));
        view.read_with(cx, |view, _| {
            assert!(!view.follow(), "toggling once should pause following");
        });

        view.update(cx, |view, cx| view.toggle_follow(cx));
        view.read_with(cx, |view, _| {
            assert!(view.follow(), "toggling twice should resume following");
        });
    }

    /// Guards against leaking the `docker logs -f` child process: the stream
    /// task is held in a field (`_logs_task`) so dropping the view cancels
    /// the stream. This test asserts the view can be dropped cleanly while a
    /// stream is in flight, without panicking or hanging.
    #[gpui::test]
    async fn dropping_view_cancels_stream(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let mut fake = FakeDockerClient::new_with_container("api");
        fake.log_lines = vec!["a".into(), "b".into()];
        let client: Arc<dyn DockerClient> = Arc::new(fake);

        let cx = cx.add_empty_window();
        let view = cx.update(|_window, cx| {
            DockerLogsView::new(test_endpoint(), "api-id".into(), "api".into(), client, cx)
        });

        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| !view.lines().is_empty())
        })
        .await;

        drop(view);
        cx.run_until_parked();
    }
}
