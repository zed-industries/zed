use std::collections::HashSet;
use std::ops::Range;
use std::sync::Arc;

use docker_client::ContainerState;
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, ListSizingBehavior, ParentElement, Pixels, Render, Styled, Subscription,
    UniformListScrollHandle, WeakEntity, Window, actions, px, uniform_list,
};
use ui::{Indicator, ListItem, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use crate::confirm_modal::ConfirmModal;
use crate::detail_view::{DetailView, SelectedItem};
use crate::endpoint_store::default_client_factory;
use crate::endpoint_store::{
    DockerAction, DockerEndpointStore, EndpointStatus, EndpointStoreEvent,
};
use crate::inspect_tab::open_inspect_tab;
use crate::logs_tab::open_logs_tab;

actions!(
    docker_panel,
    [
        /// Toggles the docker panel.
        Toggle,
        /// Toggles focus on the docker panel.
        ToggleFocus,
        /// Reconnects the selected endpoint and reloads its tree, or every
        /// endpoint if nothing is selected.
        RefreshEndpoint,
    ]
);

/// Identifies an expandable node in the endpoints tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TreeNodeId {
    Endpoint(String),
    Containers(String),
    Images(String),
    Compose(String),
}

/// A single visible row in the flattened tree, fed to `uniform_list`.
enum TreeRow {
    Endpoint {
        name: String,
        status: EndpointStatus,
        expanded: bool,
        read_only: bool,
    },
    Category {
        label: &'static str,
        node: TreeNodeId,
        expanded: bool,
        count: Option<usize>,
    },
    Container {
        endpoint_name: String,
        id: String,
        name: String,
        state: ContainerState,
        status: String,
    },
    Image {
        endpoint_name: String,
        id: String,
        repository: String,
        tag: String,
    },
    Compose {
        endpoint_name: String,
        name: String,
        status: String,
    },
    Loading {
        depth: usize,
    },
    Error {
        depth: usize,
        message: String,
    },
}

/// A destructive [`DockerAction`] awaiting user confirmation via
/// [`ConfirmModal`]. Tracked as plain data (rather than only living inside
/// the modal entity) so tests can assert a confirmation was requested
/// without needing a real `Workspace`/window to host the modal.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PendingConfirmation {
    endpoint_name: String,
    action: DockerAction,
}

pub struct DockerPanel {
    store: Entity<DockerEndpointStore>,
    workspace: Option<WeakEntity<Workspace>>,
    focus_handle: FocusHandle,
    expanded: HashSet<TreeNodeId>,
    selected: Option<SelectedItem>,
    /// Set while a `ConfirmModal` is open for a destructive action; cleared
    /// on Confirm or Cancel. See [`Self::request_action`].
    pending_confirmation: Option<PendingConfirmation>,
    scroll_handle: UniformListScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl DockerPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            Self::new(workspace, window, cx)
        })
    }

    fn new(
        _workspace: &mut Workspace,
        _window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let workspace_handle = cx.entity().downgrade();
        cx.new(|cx| {
            let client_factory = default_client_factory();
            let store = cx.new(|cx| DockerEndpointStore::new(client_factory, cx));
            Self::from_parts(store, Some(workspace_handle), cx)
        })
    }

    /// Builds the panel entity around an already-constructed store, shared by
    /// [`Self::new`] and tests that supply a fake-backed store.
    fn from_parts(
        store: Entity<DockerEndpointStore>,
        workspace: Option<WeakEntity<Workspace>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut subscriptions = vec![cx.observe(&store, |_this: &mut Self, _, cx| {
            cx.notify();
        })];
        subscriptions.push(cx.subscribe(&store, Self::on_store_event));
        Self {
            store,
            workspace,
            focus_handle: cx.focus_handle(),
            expanded: HashSet::new(),
            selected: None,
            pending_confirmation: None,
            scroll_handle: UniformListScrollHandle::new(),
            _subscriptions: subscriptions,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test(store: Entity<DockerEndpointStore>, cx: &mut Context<Self>) -> Self {
        Self::from_parts(store, None, cx)
    }

    fn on_store_event(
        &mut self,
        _store: Entity<DockerEndpointStore>,
        event: &EndpointStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            EndpointStoreEvent::Changed => cx.notify(),
        }
    }

    fn is_expanded(&self, node: &TreeNodeId) -> bool {
        self.expanded.contains(node)
    }

    #[cfg(test)]
    pub(crate) fn expand_for_test(&mut self, node: TreeNodeId) {
        self.expanded.insert(node);
    }

    /// Walks the store's endpoints, emitting only the rows that are currently
    /// visible given the set of expanded nodes.
    fn build_rows(&self, cx: &App) -> Vec<TreeRow> {
        let store = self.store.read(cx);
        let mut rows = Vec::new();

        for state in store.endpoints() {
            let endpoint_name = state.endpoint.name.clone();
            let endpoint_id = TreeNodeId::Endpoint(endpoint_name.clone());
            let endpoint_expanded = self.is_expanded(&endpoint_id);
            rows.push(TreeRow::Endpoint {
                name: endpoint_name.clone(),
                status: state.status.clone(),
                expanded: endpoint_expanded,
                read_only: state.endpoint.read_only,
            });

            if !endpoint_expanded {
                continue;
            }

            if state.status == EndpointStatus::Connecting {
                rows.push(TreeRow::Loading { depth: 1 });
                continue;
            }
            if let EndpointStatus::Error(message) = &state.status {
                rows.push(TreeRow::Error {
                    depth: 1,
                    message: message.clone(),
                });
                continue;
            }

            let containers_node = TreeNodeId::Containers(endpoint_name.clone());
            let containers_expanded = self.is_expanded(&containers_node);
            rows.push(TreeRow::Category {
                label: "Containers",
                node: containers_node,
                expanded: containers_expanded,
                count: state.containers.as_ref().map(Vec::len),
            });
            if containers_expanded {
                match &state.containers {
                    None => rows.push(TreeRow::Loading { depth: 2 }),
                    Some(containers) => {
                        for container in containers {
                            rows.push(TreeRow::Container {
                                endpoint_name: endpoint_name.clone(),
                                id: container.id.clone(),
                                name: container.names.clone(),
                                state: container.state,
                                status: container.status.clone(),
                            });
                        }
                    }
                }
            }

            let images_node = TreeNodeId::Images(endpoint_name.clone());
            let images_expanded = self.is_expanded(&images_node);
            rows.push(TreeRow::Category {
                label: "Images",
                node: images_node,
                expanded: images_expanded,
                count: state.images.as_ref().map(Vec::len),
            });
            if images_expanded {
                match &state.images {
                    None => rows.push(TreeRow::Loading { depth: 2 }),
                    Some(images) => {
                        for image in images {
                            rows.push(TreeRow::Image {
                                endpoint_name: endpoint_name.clone(),
                                id: image.id.clone(),
                                repository: image.repository.clone(),
                                tag: image.tag.clone(),
                            });
                        }
                    }
                }
            }

            let compose_node = TreeNodeId::Compose(endpoint_name.clone());
            let compose_expanded = self.is_expanded(&compose_node);
            rows.push(TreeRow::Category {
                label: "Compose",
                node: compose_node,
                expanded: compose_expanded,
                count: state.compose.as_ref().map(Vec::len),
            });
            if compose_expanded {
                match &state.compose {
                    None => rows.push(TreeRow::Loading { depth: 2 }),
                    Some(compose) => {
                        for project in compose {
                            rows.push(TreeRow::Compose {
                                endpoint_name: endpoint_name.clone(),
                                name: project.name.clone(),
                                status: project.status.clone(),
                            });
                        }
                    }
                }
            }
        }

        rows
    }

    fn toggle_node(&mut self, node: TreeNodeId, cx: &mut Context<Self>) {
        if self.expanded.remove(&node) {
            cx.notify();
            return;
        }
        self.expanded.insert(node.clone());

        // Trigger lazy loading for the newly-expanded node.
        if let TreeNodeId::Endpoint(endpoint) = &node {
            let needs_refresh = self.store.read(cx).endpoints().iter().any(|state| {
                &state.endpoint.name == endpoint
                    && state.containers.is_none()
                    && state.status != EndpointStatus::Connecting
            });
            if needs_refresh {
                self.store
                    .update(cx, |store, cx| store.refresh(endpoint, cx));
            }
        }
        cx.notify();
    }

    /// Refreshes the currently selected endpoint if there is a selection,
    /// otherwise refreshes every endpoint.
    fn refresh_endpoint(
        &mut self,
        _: &RefreshEndpoint,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.selected.as_ref() {
            Some(selected) => {
                let endpoint_name = selected.endpoint_name().to_string();
                self.store
                    .update(cx, |store, cx| store.refresh(&endpoint_name, cx));
            }
            None => {
                self.store.update(cx, |store, cx| store.refresh_all(cx));
            }
        }
    }

    fn render_row(&self, row: &TreeRow, index: usize, cx: &Context<Self>) -> AnyElement {
        match row {
            TreeRow::Endpoint {
                name,
                status,
                expanded,
                read_only,
            } => self.render_endpoint_row(index, name, status, *expanded, *read_only, cx),
            TreeRow::Category {
                label,
                node,
                expanded,
                count,
                ..
            } => self.render_category_row(index, label, node.clone(), *expanded, *count, cx),
            TreeRow::Container {
                endpoint_name,
                id,
                name,
                state,
                status,
            } => self.render_container_row(index, endpoint_name, id, name, *state, status, cx),
            TreeRow::Image {
                endpoint_name,
                id,
                repository,
                tag,
            } => self.render_image_row(index, endpoint_name, id, repository, tag, cx),
            TreeRow::Compose {
                endpoint_name,
                name,
                status,
            } => self.render_compose_row(index, endpoint_name, name, status, cx),
            TreeRow::Loading { depth } => ListItem::new(index)
                .indent_level(*depth)
                .child(
                    Label::new("Loading…")
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
                .into_any_element(),
            TreeRow::Error { depth, message } => ListItem::new(index)
                .indent_level(*depth)
                .child(
                    Label::new(message.clone())
                        .color(Color::Error)
                        .size(LabelSize::Small),
                )
                .tooltip({
                    let message = message.clone();
                    move |_, cx| Tooltip::simple(message.clone(), cx)
                })
                .into_any_element(),
        }
    }

    fn render_endpoint_row(
        &self,
        index: usize,
        name: &str,
        status: &EndpointStatus,
        expanded: bool,
        read_only: bool,
        cx: &Context<Self>,
    ) -> AnyElement {
        let node = TreeNodeId::Endpoint(name.to_string());
        let (indicator_color, is_error) = match status {
            EndpointStatus::Idle => (Color::Muted, false),
            EndpointStatus::Connecting => (Color::Warning, false),
            EndpointStatus::Connected => (Color::Success, false),
            EndpointStatus::Error(_) => (Color::Error, true),
        };
        let endpoint_name = name.to_string();

        let mut item = ListItem::new(index)
            .indent_level(0)
            .toggle(Some(expanded))
            .on_toggle(cx.listener({
                let node = node.clone();
                move |this, _, _, cx| this.toggle_node(node.clone(), cx)
            }))
            .child(
                h_flex()
                    .gap_1p5()
                    .child(Indicator::dot().color(indicator_color))
                    .child(
                        Icon::new(IconName::Server)
                            .color(if is_error {
                                Color::Error
                            } else {
                                Color::Default
                            })
                            .size(IconSize::Small),
                    )
                    .child(Label::new(endpoint_name))
                    .when(read_only, |this| {
                        this.child(
                            Icon::new(IconName::Lock)
                                .color(Color::Muted)
                                .size(IconSize::Small),
                        )
                    }),
            )
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_node(node.clone(), cx)));

        if read_only {
            item = item.tooltip(Tooltip::text("read-only endpoint"));
        }

        if let EndpointStatus::Error(message) = status {
            let message = message.clone();
            item = item.tooltip(move |_, cx| Tooltip::simple(message.clone(), cx));
        }

        item.into_any_element()
    }

    fn render_category_row(
        &self,
        index: usize,
        label: &str,
        node: TreeNodeId,
        expanded: bool,
        count: Option<usize>,
        cx: &Context<Self>,
    ) -> AnyElement {
        let icon = if expanded {
            IconName::FolderOpen
        } else {
            IconName::Folder
        };
        let label = match count {
            Some(count) => format!("{label} ({count})"),
            None => label.to_string(),
        };
        ListItem::new(index)
            .indent_level(1)
            .toggle(Some(expanded))
            .on_toggle(cx.listener({
                let node = node.clone();
                move |this, _, _, cx| this.toggle_node(node.clone(), cx)
            }))
            .child(
                h_flex()
                    .gap_1p5()
                    .child(Icon::new(icon).color(Color::Muted).size(IconSize::Small))
                    .child(Label::new(label)),
            )
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_node(node.clone(), cx)))
            .into_any_element()
    }

    fn render_container_row(
        &self,
        index: usize,
        endpoint_name: &str,
        id: &str,
        name: &str,
        state: ContainerState,
        status: &str,
        cx: &Context<Self>,
    ) -> AnyElement {
        let indicator_color = match state {
            ContainerState::Running => Color::Success,
            ContainerState::Exited | ContainerState::Dead => Color::Error,
            ContainerState::Paused | ContainerState::Restarting => Color::Warning,
            ContainerState::Created | ContainerState::Unknown => Color::Muted,
        };
        let selected =
            self.selected.as_ref() == Some(&self.container_item(endpoint_name, id, name));
        ListItem::new(index)
            .indent_level(2)
            .toggle_state(selected)
            .child(
                h_flex()
                    .gap_1p5()
                    .child(Indicator::dot().color(indicator_color))
                    .child(Label::new(name.to_string()))
                    .child(
                        Label::new(status.to_string())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
            )
            .on_click(cx.listener({
                let endpoint_name = endpoint_name.to_string();
                let id = id.to_string();
                let name = name.to_string();
                move |this, _, _, cx| {
                    this.select_container(endpoint_name.clone(), id.clone(), name.clone(), cx)
                }
            }))
            .into_any_element()
    }

    fn container_item(&self, endpoint_name: &str, id: &str, name: &str) -> SelectedItem {
        SelectedItem::Container {
            endpoint_name: endpoint_name.to_string(),
            id: id.to_string(),
            name: name.to_string(),
        }
    }

    fn render_image_row(
        &self,
        index: usize,
        endpoint_name: &str,
        id: &str,
        repository: &str,
        tag: &str,
        cx: &Context<Self>,
    ) -> AnyElement {
        let label = format!("{repository}:{tag}");
        let selected = self.selected
            == Some(SelectedItem::Image {
                endpoint_name: endpoint_name.to_string(),
                id: id.to_string(),
                repository: repository.to_string(),
                tag: tag.to_string(),
            });
        ListItem::new(index)
            .indent_level(2)
            .toggle_state(selected)
            .child(
                h_flex()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::Box)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                    .child(Label::new(label)),
            )
            .on_click(cx.listener({
                let endpoint_name = endpoint_name.to_string();
                let id = id.to_string();
                let repository = repository.to_string();
                let tag = tag.to_string();
                move |this, _, _, cx| {
                    this.select_image(
                        endpoint_name.clone(),
                        id.clone(),
                        repository.clone(),
                        tag.clone(),
                        cx,
                    );
                }
            }))
            .into_any_element()
    }

    fn render_compose_row(
        &self,
        index: usize,
        endpoint_name: &str,
        name: &str,
        status: &str,
        cx: &Context<Self>,
    ) -> AnyElement {
        let selected = self.selected
            == Some(SelectedItem::Compose {
                endpoint_name: endpoint_name.to_string(),
                project: name.to_string(),
            });
        ListItem::new(index)
            .indent_level(2)
            .toggle_state(selected)
            .child(
                h_flex()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::FileTree)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                    .child(Label::new(name.to_string()))
                    .child(
                        Label::new(status.to_string())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
            )
            .on_click(cx.listener({
                let endpoint_name = endpoint_name.to_string();
                let name = name.to_string();
                move |this, _, _, cx| {
                    this.select_compose(endpoint_name.clone(), name.clone(), cx);
                }
            }))
            .into_any_element()
    }

    fn select_container(
        &mut self,
        endpoint_name: String,
        id: String,
        name: String,
        cx: &mut Context<Self>,
    ) {
        self.selected = Some(SelectedItem::Container {
            endpoint_name,
            id,
            name,
        });
        cx.notify();
    }

    fn select_image(
        &mut self,
        endpoint_name: String,
        id: String,
        repository: String,
        tag: String,
        cx: &mut Context<Self>,
    ) {
        self.selected = Some(SelectedItem::Image {
            endpoint_name,
            id,
            repository,
            tag,
        });
        cx.notify();
    }

    fn select_compose(&mut self, endpoint_name: String, project: String, cx: &mut Context<Self>) {
        self.selected = Some(SelectedItem::Compose {
            endpoint_name,
            project,
        });
        cx.notify();
    }

    /// Opens a full-size center-pane tab streaming `docker logs -f` for the
    /// container with `id` on the currently selected endpoint. Always
    /// allowed: Logs is a read action, never gated by `read_only`. A no-op if
    /// there is no `Workspace` (e.g. a bare panel built via `new_for_test`).
    pub(crate) fn open_logs_tab(
        &mut self,
        id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((endpoint, name)) = self.selected_container_context(&id, cx) else {
            return;
        };
        let Some(workspace) = self.workspace.clone() else {
            return;
        };
        let client = self.store.read(cx).make_client();
        workspace
            .update(cx, |workspace, cx| {
                open_logs_tab(workspace, endpoint, id, name, client, window, cx);
            })
            .log_err();
    }

    /// Opens a full-size center-pane tab showing `docker inspect` output for
    /// the container with `id` on the currently selected endpoint. Always
    /// allowed: Inspect is a read action, never gated by `read_only`. A no-op
    /// if there is no `Workspace`.
    pub(crate) fn open_inspect_tab(
        &mut self,
        id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((endpoint, name)) = self.selected_container_context(&id, cx) else {
            return;
        };
        let Some(workspace) = self.workspace.clone() else {
            return;
        };
        let client = self.store.read(cx).make_client();
        workspace
            .update(cx, |workspace, cx| {
                open_inspect_tab(workspace, endpoint, id, name, client, window, cx);
            })
            .log_err();
    }

    /// Resolves the endpoint and display name for the container `id`, used by
    /// both `open_logs_tab` and `open_inspect_tab`. Only returns a value when
    /// `id` matches the currently selected container: the Logs/Inspect
    /// buttons are only rendered for the selected container's detail pane, so
    /// a mismatch means the selection changed out from under a stale button
    /// closure and the click should be a no-op rather than opening a tab for
    /// the wrong container.
    fn selected_container_context(
        &self,
        id: &str,
        cx: &Context<Self>,
    ) -> Option<(docker_client::DockerEndpoint, String)> {
        let selected = self.selected.as_ref()?;
        let name = match selected {
            SelectedItem::Container {
                id: selected_id,
                name,
                ..
            } if selected_id == id => name.clone(),
            _ => return None,
        };
        let endpoint_name = selected.endpoint_name().to_string();
        let endpoint = self.store.read(cx).endpoint(&endpoint_name)?.clone();
        Some((endpoint, name))
    }

    fn render_detail_pane(&self, cx: &Context<Self>) -> AnyElement {
        let store = self.store.read(cx);
        let selected = self.selected.as_ref();
        let endpoint = selected.and_then(|selected| {
            store
                .endpoints()
                .iter()
                .find(|state| state.endpoint.name == selected.endpoint_name())
                .map(|state| &state.endpoint)
        });
        let state = selected.and_then(|selected| {
            store
                .endpoints()
                .iter()
                .find(|state| state.endpoint.name == selected.endpoint_name())
        });

        let container = match selected {
            Some(SelectedItem::Container { id, .. }) => state
                .and_then(|state| state.containers.as_ref())
                .and_then(|containers| containers.iter().find(|c| &c.id == id)),
            _ => None,
        };
        let image = match selected {
            Some(SelectedItem::Image { id, .. }) => state
                .and_then(|state| state.images.as_ref())
                .and_then(|images| images.iter().find(|i| &i.id == id)),
            _ => None,
        };
        let compose = match selected {
            Some(SelectedItem::Compose { project, .. }) => state
                .and_then(|state| state.compose.as_ref())
                .and_then(|projects| projects.iter().find(|p| &p.name == project)),
            _ => None,
        };

        DetailView {
            selected,
            endpoint,
            container,
            image,
            compose,
        }
        .render(cx)
    }

    /// Entry point for every action button in the detail view.
    ///
    /// This is the handler-side half of the read-only gate: even though
    /// destructive buttons are already rendered `disabled` when the
    /// endpoint is read-only (see `detail_view::action_button`), this method
    /// re-checks `endpoint.read_only` itself and bails before doing anything
    /// else. That way a caller bug in the disabled-button logic (or a test
    /// invoking this handler directly, bypassing the UI) can never reach the
    /// `DockerClient`.
    ///
    /// Non-destructive actions run immediately. Destructive ones open a
    /// `ConfirmModal` (when a `Workspace` is available) and are only sent to
    /// the store once the user confirms — see `ConfirmModal::confirm`, which
    /// is the only place a destructive action actually reaches the store.
    pub(crate) fn request_action(
        &mut self,
        action: DockerAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(pending) = self.gate_and_dispatch_or_pend(action, cx) else {
            return;
        };

        // Tests that construct a bare panel (see `new_for_test`) have no
        // `Workspace`/window to host a modal in; they instead observe
        // `pending_confirmation` directly and drive the outcome via
        // `confirm_pending_for_test`/`cancel_pending_for_test`, which run
        // the exact same store-dispatch path a real Confirm click would.
        let Some(workspace) = self.workspace.clone() else {
            return;
        };
        let store = self.store.clone();
        workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, |_window, cx| {
                    ConfirmModal::new(pending.endpoint_name, pending.action, store, cx)
                });
            })
            .log_err();
        // The real `ConfirmModal` now owns the decision; a bare-panel
        // `pending_confirmation` (used only when there is no `Workspace`) is
        // not applicable once a modal has actually been shown.
        self.pending_confirmation = None;
    }

    /// The windowless core of [`Self::request_action`]: applies the
    /// read-only gate, runs non-destructive actions immediately, and records
    /// a [`PendingConfirmation`] for destructive ones. Returns the pending
    /// confirmation when the caller still needs to decide how to surface it
    /// (open a real modal, or leave it for a windowless test to observe).
    fn gate_and_dispatch_or_pend(
        &mut self,
        action: DockerAction,
        cx: &mut Context<Self>,
    ) -> Option<PendingConfirmation> {
        let endpoint_name = self
            .selected
            .as_ref()
            .map(|s| s.endpoint_name().to_string())?;
        let read_only = self
            .store
            .read(cx)
            .endpoints()
            .iter()
            .find(|state| state.endpoint.name == endpoint_name)
            .map(|state| state.endpoint.read_only)
            .unwrap_or(false);

        if action.is_destructive() && read_only {
            // Belt-and-suspenders: the button that dispatched here should
            // already be disabled, but bail regardless of how we got called.
            return None;
        }

        if !action.is_destructive() {
            self.store.update(cx, |store, cx| {
                store.dispatch_action(&endpoint_name, action, cx)
            });
            return None;
        }

        let pending = PendingConfirmation {
            endpoint_name,
            action,
        };
        self.pending_confirmation = Some(pending.clone());
        Some(pending)
    }

    /// Test-only equivalent of clicking an action button, for panels built
    /// via `new_for_test` (no `Workspace`/window available to host a real
    /// `ConfirmModal`). Runs the exact same read-only gate and dispatch path
    /// as `request_action`; destructive actions land in
    /// `pending_confirmation` instead of opening a modal.
    #[cfg(test)]
    pub(crate) fn request_action_for_test(&mut self, action: DockerAction, cx: &mut Context<Self>) {
        self.gate_and_dispatch_or_pend(action, cx);
    }

    #[cfg(test)]
    pub(crate) fn pending_confirmation_for_test(&self) -> Option<(String, DockerAction)> {
        self.pending_confirmation
            .as_ref()
            .map(|pending| (pending.endpoint_name.clone(), pending.action.clone()))
    }

    /// Test-only equivalent of clicking Confirm on the open `ConfirmModal`:
    /// runs the exact same store-dispatch path (including the store's own
    /// read-only re-check) without requiring a real `Workspace`/window.
    #[cfg(test)]
    pub(crate) fn confirm_pending_for_test(&mut self, cx: &mut Context<Self>) {
        let Some(pending) = self.pending_confirmation.take() else {
            return;
        };
        self.store.update(cx, |store, cx| {
            store.dispatch_action(&pending.endpoint_name, pending.action, cx);
        });
    }

    #[cfg(test)]
    pub(crate) fn cancel_pending_for_test(&mut self) {
        self.pending_confirmation = None;
    }
}

impl Render for DockerPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self.build_rows(cx);

        let content = if rows.is_empty() {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .gap_2()
                .child(Label::new("No endpoints").color(Color::Muted))
                .into_any_element()
        } else {
            let rows = Arc::new(rows);
            uniform_list(
                "docker-tree",
                rows.len(),
                cx.processor(move |this, range: Range<usize>, _window, cx| {
                    range
                        .filter_map(|index| {
                            rows.get(index).map(|row| this.render_row(row, index, cx))
                        })
                        .collect::<Vec<_>>()
                }),
            )
            .size_full()
            .with_sizing_behavior(ListSizingBehavior::Infer)
            .track_scroll(&self.scroll_handle)
            .into_any_element()
        };

        v_flex()
            .key_context("DockerPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::refresh_endpoint))
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new("Docker"))
                    .child(
                        IconButton::new("refresh-endpoint", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Refresh"))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(RefreshEndpoint.boxed_clone(), cx);
                            }),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .size_full()
                    .overflow_hidden()
                    .child(content),
            )
            .when(self.selected.is_some(), |this| {
                this.child(
                    v_flex()
                        .h(rems(16.))
                        .border_t_1()
                        .border_color(cx.theme().colors().border)
                        .child(self.render_detail_pane(cx)),
                )
            })
    }
}

impl Focusable for DockerPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for DockerPanel {}

impl Panel for DockerPanel {
    fn persistent_name() -> &'static str {
        "DockerPanel"
    }

    fn panel_key() -> &'static str {
        "DockerPanel"
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        // `position()` is hard-coded to Left and `set_position` does not persist
        // a move, so only advertise Left as valid; advertising Right would offer
        // a "Dock Right" menu entry that silently does nothing.
        matches!(position, DockPosition::Left)
    }

    fn set_position(&mut self, _: DockPosition, _: &mut Window, _: &mut Context<Self>) {}

    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(240.)
    }

    fn icon(&self, _: &Window, _: &App) -> Option<ui::IconName> {
        Some(ui::IconName::Server)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Docker Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        // Must be unique across panels that can share a dock (Dock::add_panel
        // panics in debug builds on a collision). 0-7 are taken by other
        // panels (ProjectPanel=1, TerminalPanel=2, GitPanel=3, DatabasePanel=4,
        // CollabPanel=5, OutlinePanel=6, DebugPanel=7); 8 is free.
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use docker_client::DockerClient;
    use docker_client::fake::FakeDockerClient;
    use gpui::TestAppContext;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            gpui_tokio::init(cx);
            crate::init(cx);
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
    async fn expanding_endpoint_exposes_container_name(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));

        let panel = cx.new(|cx| DockerPanel::new_for_test(store.clone(), cx));

        let endpoint_name =
            store.read_with(cx, |store, _| store.endpoints()[0].endpoint.name.clone());

        panel.update(cx, |panel, _cx| {
            panel.expand_for_test(TreeNodeId::Endpoint(endpoint_name.clone()));
        });
        store.update(cx, |store, cx| store.refresh(&endpoint_name, cx));

        wait_until(cx, |cx| {
            store.read_with(cx, |store, _| {
                store
                    .endpoints()
                    .iter()
                    .find(|state| state.endpoint.name == endpoint_name)
                    .and_then(|state| state.containers.as_ref())
                    .is_some_and(|containers| containers.len() == 1)
            })
        })
        .await;

        panel.update(cx, |panel, _cx| {
            panel.expand_for_test(TreeNodeId::Containers(endpoint_name.clone()));
        });

        let rows = panel.read_with(cx, |panel, cx| panel.build_rows(cx));
        let found = rows.iter().any(|row| match row {
            TreeRow::Container { name, state, .. } => {
                name == "api" && *state == ContainerState::Running
            }
            _ => false,
        });
        assert!(found, "expected an expanded container row named `api`");
    }

    /// Fix D: the endpoint row must carry `read_only` straight from
    /// `EndpointState.endpoint.read_only`, so the tree can render the
    /// read-only lock marker without re-deriving the flag.
    #[gpui::test]
    fn build_rows_reflects_endpoint_read_only_flag(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fake = Arc::new(FakeDockerClient::new());
        let factory = Arc::new(move || fake.clone() as Arc<dyn DockerClient>);
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let endpoint_name =
            store.read_with(cx, |store, _| store.endpoints()[0].endpoint.name.clone());

        let panel = cx.new(|cx| DockerPanel::new_for_test(store.clone(), cx));

        let read_only_before = panel.read_with(cx, |panel, cx| {
            panel
                .build_rows(cx)
                .iter()
                .find_map(|row| match row {
                    TreeRow::Endpoint {
                        name, read_only, ..
                    } if name == &endpoint_name => Some(*read_only),
                    _ => None,
                })
                .expect("endpoint row should exist")
        });
        assert!(
            !read_only_before,
            "the synthetic local endpoint defaults to writable"
        );

        store.update(cx, |store, cx| {
            if let Some(state) = store.endpoint_mut_for_test(&endpoint_name) {
                state.endpoint.read_only = true;
            }
            cx.notify();
        });

        let read_only_after = panel.read_with(cx, |panel, cx| {
            panel
                .build_rows(cx)
                .iter()
                .find_map(|row| match row {
                    TreeRow::Endpoint {
                        name, read_only, ..
                    } if name == &endpoint_name => Some(*read_only),
                    _ => None,
                })
                .expect("endpoint row should exist")
        });
        assert!(
            read_only_after,
            "build_rows should reflect the endpoint becoming read-only"
        );
    }

    /// SAFETY-CRITICAL: invoking a destructive action against a read-only
    /// endpoint must never reach the `DockerClient`, even when called
    /// directly on the handler (bypassing a disabled button in the UI).
    #[gpui::test]
    async fn destructive_action_blocked_on_read_only_endpoint(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let endpoint_name =
            store.read_with(cx, |store, _| store.endpoints()[0].endpoint.name.clone());

        // Force the endpoint read-only directly on the store's state (the
        // "local" synthetic endpoint defaults to `read_only: false`).
        store.update(cx, |store, cx| {
            if let Some(state) = store.endpoint_mut_for_test(&endpoint_name) {
                state.endpoint.read_only = true;
            }
            cx.notify();
        });

        let panel = cx.new(|cx| DockerPanel::new_for_test(store.clone(), cx));
        panel.update(cx, |panel, cx| {
            panel.select_container(endpoint_name.clone(), "api-id".into(), "api".into(), cx);
        });

        panel.update(cx, |panel, cx| {
            panel.request_action_for_test(
                DockerAction::StopContainer {
                    id: "api-id".into(),
                },
                cx,
            );
        });
        cx.run_until_parked();

        assert!(
            !fake.calls().iter().any(|c| c.starts_with("stop_container")),
            "a destructive action on a read-only endpoint must never reach the client; calls: {:?}",
            fake.calls()
        );
        // No confirmation should be pending either: the gate bails before
        // getting anywhere near a confirm step.
        panel.read_with(cx, |panel, _| {
            assert!(panel.pending_confirmation_for_test().is_none());
        });
    }

    /// SAFETY-CRITICAL: invoking a destructive action on a non-read-only
    /// endpoint must not call the client until the user confirms; after
    /// confirming, the client must be called exactly once.
    #[gpui::test]
    async fn destructive_action_requires_confirmation(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let endpoint_name =
            store.read_with(cx, |store, _| store.endpoints()[0].endpoint.name.clone());

        let panel = cx.new(|cx| DockerPanel::new_for_test(store.clone(), cx));
        panel.update(cx, |panel, cx| {
            panel.select_container(endpoint_name.clone(), "api-id".into(), "api".into(), cx);
        });

        panel.update(cx, |panel, cx| {
            panel.request_action_for_test(
                DockerAction::RestartContainer {
                    id: "api-id".into(),
                },
                cx,
            );
        });
        cx.run_until_parked();

        // A confirmation must be pending (standing in for an opened
        // `ConfirmModal`) and the client must not have been called yet.
        panel.read_with(cx, |panel, _| {
            let pending = panel
                .pending_confirmation_for_test()
                .expect("restart on a writable endpoint should require confirmation");
            assert_eq!(pending.0, endpoint_name);
            assert_eq!(
                pending.1,
                DockerAction::RestartContainer {
                    id: "api-id".into()
                }
            );
        });
        assert!(
            !fake
                .calls()
                .iter()
                .any(|c| c.starts_with("restart_container")),
            "the client must not be called before confirmation; calls: {:?}",
            fake.calls()
        );

        panel.update(cx, |panel, cx| panel.confirm_pending_for_test(cx));
        wait_until(cx, |_| {
            fake.calls()
                .iter()
                .any(|c| c.starts_with("restart_container"))
        })
        .await;

        assert!(
            fake.calls()
                .iter()
                .any(|c| c.starts_with("restart_container")),
            "confirming should dispatch restart_container; calls: {:?}",
            fake.calls()
        );
    }

    /// Cancelling a pending confirmation must never dispatch the action.
    #[gpui::test]
    async fn cancelling_confirmation_does_not_dispatch(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let endpoint_name =
            store.read_with(cx, |store, _| store.endpoints()[0].endpoint.name.clone());

        let panel = cx.new(|cx| DockerPanel::new_for_test(store.clone(), cx));
        panel.update(cx, |panel, cx| {
            panel.select_container(endpoint_name.clone(), "api-id".into(), "api".into(), cx);
        });
        panel.update(cx, |panel, cx| {
            panel.request_action_for_test(
                DockerAction::StopContainer {
                    id: "api-id".into(),
                },
                cx,
            );
        });
        panel.read_with(cx, |panel, _| {
            assert!(panel.pending_confirmation_for_test().is_some());
        });

        panel.update(cx, |panel, _cx| panel.cancel_pending_for_test());
        cx.run_until_parked();

        panel.read_with(cx, |panel, _| {
            assert!(panel.pending_confirmation_for_test().is_none());
        });
        assert!(
            !fake.calls().iter().any(|c| c.starts_with("stop_container")),
            "cancelling must not dispatch the action; calls: {:?}",
            fake.calls()
        );
    }

    /// Logs and Inspect now open as full-size center tabs owned by
    /// `DockerLogsView`/`DockerInspectView` (see `logs_tab.rs`/
    /// `inspect_tab.rs` for the streaming/follow/inspect-fetch tests); the
    /// panel itself no longer holds any logs/inspect state. `new_for_test`
    /// panels have no `Workspace` to add a tab to, so `open_logs_tab`/
    /// `open_inspect_tab` must no-op cleanly (not panic) in that case, which
    /// is what a caller bug or an unexpected `None` workspace should do.
    #[gpui::test]
    async fn open_logs_and_inspect_tab_noop_without_workspace(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();

        let fake = Arc::new(FakeDockerClient::new_with_container("api"));
        let factory = {
            let fake = fake.clone();
            Arc::new(move || fake.clone() as Arc<dyn DockerClient>)
        };
        let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
        let endpoint_name =
            store.read_with(cx, |store, _| store.endpoints()[0].endpoint.name.clone());

        let cx = cx.add_empty_window();
        let panel =
            cx.update(|_window, cx| cx.new(|cx| DockerPanel::new_for_test(store.clone(), cx)));
        panel.update(cx, |panel, cx| {
            panel.select_container(endpoint_name.clone(), "api-id".into(), "api".into(), cx);
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.open_logs_tab("api-id".into(), window, cx);
            panel.open_inspect_tab("api-id".into(), window, cx);
        });
        cx.run_until_parked();

        // Neither a stream nor an inspect fetch should have started: with no
        // `Workspace` to host a tab, both calls must bail out before ever
        // reaching the client.
        assert!(
            !fake.calls().iter().any(|c| c.starts_with("container_logs")),
            "open_logs_tab must no-op without a workspace; calls: {:?}",
            fake.calls()
        );
        assert!(
            !fake
                .calls()
                .iter()
                .any(|c| c.starts_with("inspect_container")),
            "open_inspect_tab must no-op without a workspace; calls: {:?}",
            fake.calls()
        );
    }
}
