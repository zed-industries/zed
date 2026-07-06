use docker_client::{ComposeProject, Container, ContainerState, DockerEndpoint, Image};
use gpui::{AnyElement, Context};
use ui::{Tooltip, prelude::*};

use crate::docker_panel::DockerPanel;
use crate::endpoint_store::DockerAction;

/// The container-state-changing action a button in the container detail
/// pane can represent. Distinct from [`DockerAction`] (which carries the
/// container id needed to actually dispatch) so [`action_enabled`] can be a
/// pure function of just the three things that determine enablement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerAction {
    Start,
    Stop,
    Restart,
}

/// Why a container detail action button is disabled, driving the tooltip
/// text. `None` means the button is enabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionDisabledReason {
    ReadOnly,
    AlreadyRunning,
    NotRunning,
}

impl ActionDisabledReason {
    pub fn tooltip_text(self) -> &'static str {
        match self {
            ActionDisabledReason::ReadOnly => "endpoint is read-only",
            ActionDisabledReason::AlreadyRunning => "container is already running",
            ActionDisabledReason::NotRunning => "container is not running",
        }
    }
}

/// Whether `container_action` is enabled for a container currently in
/// `state`, on an endpoint whose `read_only` flag is given. Pure and
/// independent of any `Window`/`Context`, so the full state/read-only matrix
/// can be unit-tested directly (see the `tests` module below) without
/// needing to render anything.
///
/// - **Start**: enabled only when the container is NOT running and the
///   endpoint is writable.
/// - **Stop** / **Restart**: enabled only when the container IS running
///   (including `Restarting`/`Paused`, both of which are meaningfully "up")
///   and the endpoint is writable. Restarting a stopped container is an
///   unusual `docker restart` use case, so Restart is kept running-only for
///   a clear mental model that mirrors Stop.
///
/// Returns `Ok(())` when enabled, or the `Err(reason)` to show in the
/// button's tooltip when disabled. Read-only takes precedence over a state
/// mismatch: when the endpoint is read-only, Start/Stop/Restart are all
/// disabled with the "endpoint is read-only" tooltip regardless of the
/// container's state, since read-only is the more fundamental reason the
/// action can't run.
pub fn action_enabled(
    container_action: ContainerAction,
    state: ContainerState,
    read_only: bool,
) -> Result<(), ActionDisabledReason> {
    if read_only {
        return Err(ActionDisabledReason::ReadOnly);
    }
    let is_running = matches!(
        state,
        ContainerState::Running | ContainerState::Restarting | ContainerState::Paused
    );
    match container_action {
        ContainerAction::Start if is_running => Err(ActionDisabledReason::AlreadyRunning),
        ContainerAction::Stop | ContainerAction::Restart if !is_running => {
            Err(ActionDisabledReason::NotRunning)
        }
        _ => Ok(()),
    }
}

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

/// Builds a Start/Stop/Restart container action button whose enabled state
/// reflects the container's running state as well as `endpoint.read_only`
/// (see [`action_enabled`]), rather than only the read-only flag that
/// [`action_button`] checks for image/compose actions.
///
/// This only controls the button's rendered/clickable state; the handler
/// bound to `on_click` (`DockerPanel::request_action`) re-checks
/// `endpoint.read_only` itself (and `DockerEndpointStore::dispatch_action`
/// re-checks it again) before calling into the store, so a bug in this
/// gating can never be the sole thing standing between a click and a
/// mutating call to the `DockerClient`.
fn container_action_button(
    id: &'static str,
    label: &'static str,
    container_action: ContainerAction,
    action: DockerAction,
    state: ContainerState,
    endpoint: &DockerEndpoint,
    cx: &Context<DockerPanel>,
) -> AnyElement {
    let enabled = action_enabled(container_action, state, endpoint.read_only);
    let mut button = Button::new(id, label).disabled(enabled.is_err());
    match enabled {
        Ok(()) => {
            button = button.on_click(cx.listener(move |this, _, window, cx| {
                this.request_action(action.clone(), window, cx);
            }));
        }
        Err(reason) => {
            button = button.tooltip(Tooltip::text(reason.tooltip_text()));
        }
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
            SelectedItem::Container { id, name, .. } => {
                render_container_detail(id, name, self.container, endpoint, cx)
            }
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
    cx: &Context<DockerPanel>,
) -> AnyElement {
    let status = container.map(|c| c.status.clone()).unwrap_or_default();
    // Absence of a loaded `Container` (e.g. the list hasn't refreshed since
    // selection) is treated as `Unknown`, which `action_enabled` treats as
    // not-running: Stop/Restart stay disabled rather than defaulting to
    // enabled for a container we know nothing about.
    let state = container
        .map(|c| c.state)
        .unwrap_or(ContainerState::Unknown);

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
                .child(container_action_button(
                    "start-container",
                    "Start",
                    ContainerAction::Start,
                    DockerAction::StartContainer { id: id.to_string() },
                    state,
                    endpoint,
                    cx,
                ))
                .child(container_action_button(
                    "stop-container",
                    "Stop",
                    ContainerAction::Stop,
                    DockerAction::StopContainer { id: id.to_string() },
                    state,
                    endpoint,
                    cx,
                ))
                .child(container_action_button(
                    "restart-container",
                    "Restart",
                    ContainerAction::Restart,
                    DockerAction::RestartContainer { id: id.to_string() },
                    state,
                    endpoint,
                    cx,
                ))
                .child(Button::new("logs-container", "Logs").on_click(cx.listener({
                    let id = id.to_string();
                    move |this, _, window, cx| this.open_logs_tab(id.clone(), window, cx)
                })))
                .child(
                    Button::new("inspect-container", "Inspect").on_click(cx.listener({
                        let id = id.to_string();
                        move |this, _, window, cx| this.open_inspect_tab(id.clone(), window, cx)
                    })),
                ),
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Every non-running state (everything except `Running`/`Restarting`/
    /// `Paused`) must behave identically for enablement purposes: grouping
    /// them here keeps the matrix below from silently missing one if a new
    /// `ContainerState` variant is added.
    const NOT_RUNNING_STATES: [ContainerState; 3] = [
        ContainerState::Exited,
        ContainerState::Created,
        ContainerState::Dead,
    ];
    const RUNNING_STATES: [ContainerState; 3] = [
        ContainerState::Running,
        ContainerState::Restarting,
        ContainerState::Paused,
    ];

    #[test]
    fn start_enabled_only_when_not_running_and_writable() {
        for state in NOT_RUNNING_STATES {
            assert_eq!(
                action_enabled(ContainerAction::Start, state, false),
                Ok(()),
                "Start should be enabled for {state:?} on a writable endpoint"
            );
            assert_eq!(
                action_enabled(ContainerAction::Start, state, true),
                Err(ActionDisabledReason::ReadOnly),
                "Start should be blocked by read-only even when not running ({state:?})"
            );
        }
        for state in RUNNING_STATES {
            assert_eq!(
                action_enabled(ContainerAction::Start, state, false),
                Err(ActionDisabledReason::AlreadyRunning),
                "Start should be disabled while {state:?} (already running)"
            );
            assert_eq!(
                action_enabled(ContainerAction::Start, state, true),
                Err(ActionDisabledReason::ReadOnly),
                "read-only should take precedence over already-running for {state:?}"
            );
        }
        assert_eq!(
            action_enabled(ContainerAction::Start, ContainerState::Unknown, false),
            Ok(())
        );
    }

    #[test]
    fn stop_enabled_only_when_running_and_writable() {
        for state in RUNNING_STATES {
            assert_eq!(
                action_enabled(ContainerAction::Stop, state, false),
                Ok(()),
                "Stop should be enabled while {state:?} on a writable endpoint"
            );
            assert_eq!(
                action_enabled(ContainerAction::Stop, state, true),
                Err(ActionDisabledReason::ReadOnly),
                "Stop should be blocked by read-only even while running ({state:?})"
            );
        }
        for state in NOT_RUNNING_STATES {
            assert_eq!(
                action_enabled(ContainerAction::Stop, state, false),
                Err(ActionDisabledReason::NotRunning),
                "Stop should be disabled while {state:?} (not running)"
            );
            assert_eq!(
                action_enabled(ContainerAction::Stop, state, true),
                Err(ActionDisabledReason::ReadOnly),
                "read-only should take precedence over not-running for {state:?}"
            );
        }
        assert_eq!(
            action_enabled(ContainerAction::Stop, ContainerState::Unknown, false),
            Err(ActionDisabledReason::NotRunning)
        );
    }

    #[test]
    fn restart_enabled_only_when_running_and_writable() {
        for state in RUNNING_STATES {
            assert_eq!(
                action_enabled(ContainerAction::Restart, state, false),
                Ok(()),
                "Restart should be enabled while {state:?} on a writable endpoint"
            );
            assert_eq!(
                action_enabled(ContainerAction::Restart, state, true),
                Err(ActionDisabledReason::ReadOnly),
                "Restart should be blocked by read-only even while running ({state:?})"
            );
        }
        for state in NOT_RUNNING_STATES {
            assert_eq!(
                action_enabled(ContainerAction::Restart, state, false),
                Err(ActionDisabledReason::NotRunning),
                "Restart should be disabled while {state:?} (not running)"
            );
        }
    }

    /// The exact scenario from the bug report: a running container under a
    /// (now-writable-if-local) endpoint must show Start disabled/Stop+Restart
    /// enabled, never the reverse.
    #[test]
    fn running_container_on_writable_endpoint_enables_stop_restart_not_start() {
        let state = ContainerState::Running;
        assert_eq!(
            action_enabled(ContainerAction::Start, state, false),
            Err(ActionDisabledReason::AlreadyRunning)
        );
        assert_eq!(action_enabled(ContainerAction::Stop, state, false), Ok(()));
        assert_eq!(
            action_enabled(ContainerAction::Restart, state, false),
            Ok(())
        );
    }

    #[test]
    fn tooltip_text_matches_reason() {
        assert_eq!(
            ActionDisabledReason::ReadOnly.tooltip_text(),
            "endpoint is read-only"
        );
        assert_eq!(
            ActionDisabledReason::AlreadyRunning.tooltip_text(),
            "container is already running"
        );
        assert_eq!(
            ActionDisabledReason::NotRunning.tooltip_text(),
            "container is not running"
        );
    }
}
