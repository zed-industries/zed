use docker_client::{ComposeProject, Container, DockerEndpoint, Image};
use gpui::{AnyElement, Context, InteractiveElement, ParentElement, ScrollHandle, Styled};
use ui::{Tooltip, prelude::*};

use crate::docker_panel::DockerPanel;
use crate::endpoint_store::DockerAction;

/// The tree item currently selected in [`DockerPanel`], driving what
/// [`DetailView`] shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectedItem {
    Container {
        endpoint_name: String,
        id: String,
        name: String,
    },
    Image {
        endpoint_name: String,
        id: String,
        repository: String,
        tag: String,
    },
    Compose {
        endpoint_name: String,
        project: String,
    },
}

impl SelectedItem {
    pub fn endpoint_name(&self) -> &str {
        match self {
            SelectedItem::Container { endpoint_name, .. }
            | SelectedItem::Image { endpoint_name, .. }
            | SelectedItem::Compose { endpoint_name, .. } => endpoint_name,
        }
    }
}

/// The raw-JSON inspect output for the selected container, if it has been
/// fetched. `None` before the user opens Inspect or after switching selection.
#[derive(Clone, Debug, Default)]
pub struct InspectState {
    pub id: String,
    pub json: String,
}

/// The tail of streamed log lines for the selected container, if Logs has
/// been opened.
///
/// `follow` gates whether newly-appended lines pull the view down to the
/// tail: the background stream in `DockerPanel::load_logs` keeps pushing
/// into `lines` regardless of `follow`. The displayed buffer is capped at
/// `MAX_DISPLAYED_LOG_LINES` (500) either way, evicting the oldest lines past
/// that cap; pausing only freezes the scroll position, it does not exempt
/// the buffer from that cap.
#[derive(Clone, Debug)]
pub struct LogsState {
    pub id: String,
    pub lines: Vec<String>,
    pub follow: bool,
    pub scroll_handle: ScrollHandle,
}

impl LogsState {
    pub fn new(id: String) -> Self {
        Self {
            id,
            lines: Vec::new(),
            follow: true,
            scroll_handle: ScrollHandle::new(),
        }
    }
}

/// Builds an action button that is disabled with an explanatory tooltip when
/// `action` is destructive and `endpoint.read_only` is true.
///
/// This only controls the button's rendered/clickable state; the handler
/// bound to `on_click` (`DockerPanel::request_action`) re-checks
/// `endpoint.read_only` itself before calling into the store, so a bug in
/// this gating can never be the sole thing standing between a click and a
/// mutating call to the `DockerClient`.
fn action_button(
    id: &'static str,
    label: &'static str,
    action: DockerAction,
    endpoint: &DockerEndpoint,
    cx: &Context<DockerPanel>,
) -> AnyElement {
    let blocked = action.is_destructive() && endpoint.read_only;
    let mut button = Button::new(id, label).disabled(blocked);
    if blocked {
        button = button.tooltip(Tooltip::text("endpoint is read-only"));
    } else {
        button = button.on_click(cx.listener(move |this, _, window, cx| {
            this.request_action(action.clone(), window, cx);
        }));
    }
    button.into_any_element()
}

/// Renders the detail pane for the currently selected item, or a placeholder
/// when nothing is selected. Built fresh on every `DockerPanel::render` call
/// from borrowed references into the store/panel state — it holds no state
/// of its own.
pub struct DetailView<'a> {
    pub selected: Option<&'a SelectedItem>,
    pub endpoint: Option<&'a DockerEndpoint>,
    pub container: Option<&'a Container>,
    pub image: Option<&'a Image>,
    pub compose: Option<&'a ComposeProject>,
    pub inspect: Option<&'a InspectState>,
    pub logs: Option<&'a LogsState>,
}

impl<'a> DetailView<'a> {
    pub fn render(self, cx: &Context<DockerPanel>) -> AnyElement {
        let Some(selected) = self.selected else {
            return v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("Select an item").color(Color::Muted))
                .into_any_element();
        };
        let Some(endpoint) = self.endpoint else {
            return v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("Endpoint not found").color(Color::Muted))
                .into_any_element();
        };

        match selected {
            SelectedItem::Container { id, name, .. } => render_container_detail(
                id,
                name,
                self.container,
                endpoint,
                self.inspect,
                self.logs,
                cx,
            ),
            SelectedItem::Image {
                id,
                repository,
                tag,
                ..
            } => render_image_detail(id, repository, tag, self.image, endpoint, cx),
            SelectedItem::Compose { project, .. } => {
                render_compose_detail(project, self.compose, endpoint, cx)
            }
        }
    }
}

fn render_container_detail(
    id: &str,
    name: &str,
    container: Option<&Container>,
    endpoint: &DockerEndpoint,
    inspect: Option<&InspectState>,
    logs: Option<&LogsState>,
    cx: &Context<DockerPanel>,
) -> AnyElement {
    let status = container.map(|c| c.status.clone()).unwrap_or_default();

    v_flex()
        .size_full()
        .p_2()
        .gap_2()
        .child(Label::new(format!("Container: {name}")).size(LabelSize::Large))
        .child(
            Label::new(status)
                .color(Color::Muted)
                .size(LabelSize::Small),
        )
        .child(
            h_flex()
                .gap_2()
                .child(action_button(
                    "start-container",
                    "Start",
                    DockerAction::StartContainer { id: id.to_string() },
                    endpoint,
                    cx,
                ))
                .child(action_button(
                    "stop-container",
                    "Stop",
                    DockerAction::StopContainer { id: id.to_string() },
                    endpoint,
                    cx,
                ))
                .child(action_button(
                    "restart-container",
                    "Restart",
                    DockerAction::RestartContainer { id: id.to_string() },
                    endpoint,
                    cx,
                ))
                .child(Button::new("logs-container", "Logs").on_click(cx.listener({
                    let id = id.to_string();
                    move |this, _, _window, cx| this.load_logs(id.clone(), cx)
                })))
                .child(
                    Button::new("inspect-container", "Inspect").on_click(cx.listener({
                        let id = id.to_string();
                        move |this, _, _window, cx| this.load_inspect(id.clone(), cx)
                    })),
                ),
        )
        .when_some(
            inspect.filter(|inspect| inspect.id == id),
            |this, inspect| {
                this.child(
                    v_flex()
                        .id("container-inspect-scroll")
                        .flex_1()
                        .overflow_y_scroll()
                        .p_2()
                        .bg(cx.theme().colors().editor_background)
                        .child(
                            Label::new(inspect.json.clone())
                                .buffer_font(cx)
                                .size(LabelSize::Small),
                        ),
                )
            },
        )
        .when_some(logs.filter(|logs| logs.id == id), |this, logs| {
            let follow_label = if logs.follow { "Following" } else { "Paused" };
            this.child(
                h_flex().gap_2().child(
                    Button::new("toggle-follow-logs", follow_label)
                        .toggle_state(logs.follow)
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.toggle_logs_follow(cx);
                        })),
                ),
            )
            .child(
                v_flex()
                    .id("container-logs-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .track_scroll(&logs.scroll_handle)
                    .p_2()
                    .bg(cx.theme().colors().editor_background)
                    .children(logs.lines.iter().map(|line| {
                        Label::new(line.clone())
                            .buffer_font(cx)
                            .size(LabelSize::Small)
                            .into_any_element()
                    })),
            )
        })
        .into_any_element()
}

fn render_image_detail(
    id: &str,
    repository: &str,
    tag: &str,
    image: Option<&Image>,
    endpoint: &DockerEndpoint,
    cx: &Context<DockerPanel>,
) -> AnyElement {
    let size = image.map(|i| i.size.clone()).unwrap_or_default();
    let reference = format!("{repository}:{tag}");

    v_flex()
        .size_full()
        .p_2()
        .gap_2()
        .child(Label::new(format!("Image: {reference}")).size(LabelSize::Large))
        .child(Label::new(size).color(Color::Muted).size(LabelSize::Small))
        .child(
            h_flex()
                .gap_2()
                .child(action_button(
                    "pull-image",
                    "Pull",
                    DockerAction::PullImage { reference },
                    endpoint,
                    cx,
                ))
                .child(action_button(
                    "remove-image",
                    "Remove",
                    DockerAction::RemoveImage { id: id.to_string() },
                    endpoint,
                    cx,
                )),
        )
        .into_any_element()
}

fn render_compose_detail(
    project: &str,
    compose: Option<&ComposeProject>,
    endpoint: &DockerEndpoint,
    cx: &Context<DockerPanel>,
) -> AnyElement {
    let status = compose.map(|p| p.status.clone()).unwrap_or_default();

    v_flex()
        .size_full()
        .p_2()
        .gap_2()
        .child(Label::new(format!("Compose project: {project}")).size(LabelSize::Large))
        .child(
            Label::new(status)
                .color(Color::Muted)
                .size(LabelSize::Small),
        )
        .child(
            h_flex()
                .gap_2()
                .child(action_button(
                    "compose-up",
                    "Up",
                    DockerAction::ComposeUp {
                        project: project.to_string(),
                        service: None,
                    },
                    endpoint,
                    cx,
                ))
                .child(action_button(
                    "compose-down",
                    "Down",
                    DockerAction::ComposeDown {
                        project: project.to_string(),
                    },
                    endpoint,
                    cx,
                ))
                .child(action_button(
                    "compose-restart",
                    "Restart",
                    DockerAction::ComposeRestart {
                        project: project.to_string(),
                        service: None,
                    },
                    endpoint,
                    cx,
                )),
        )
        .into_any_element()
}
