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
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use crate::endpoint_store::default_client_factory;
use crate::endpoint_store::{DockerEndpointStore, EndpointStatus, EndpointStoreEvent};

actions!(
    docker_panel,
    [
        /// Toggles the docker panel.
        Toggle,
        /// Toggles focus on the docker panel.
        ToggleFocus,
        /// Reconnects the selected endpoint and reloads its tree.
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
    },
    Category {
        label: &'static str,
        node: TreeNodeId,
        expanded: bool,
        count: Option<usize>,
    },
    Container {
        name: String,
        state: ContainerState,
        status: String,
    },
    Image {
        repository: String,
        tag: String,
    },
    Compose {
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

pub struct DockerPanel {
    store: Entity<DockerEndpointStore>,
    focus_handle: FocusHandle,
    expanded: HashSet<TreeNodeId>,
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
        cx.new(|cx| {
            let client_factory = default_client_factory();
            let store = cx.new(|cx| DockerEndpointStore::new(client_factory, cx));
            Self::from_parts(store, cx)
        })
    }

    /// Builds the panel entity around an already-constructed store, shared by
    /// [`Self::new`] and tests that supply a fake-backed store.
    fn from_parts(store: Entity<DockerEndpointStore>, cx: &mut Context<Self>) -> Self {
        let mut subscriptions = vec![cx.observe(&store, |_this: &mut Self, _, cx| {
            cx.notify();
        })];
        subscriptions.push(cx.subscribe(&store, Self::on_store_event));
        Self {
            store,
            focus_handle: cx.focus_handle(),
            expanded: HashSet::new(),
            scroll_handle: UniformListScrollHandle::new(),
            _subscriptions: subscriptions,
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test(store: Entity<DockerEndpointStore>, cx: &mut Context<Self>) -> Self {
        Self::from_parts(store, cx)
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

    fn refresh_endpoint(
        &mut self,
        _: &RefreshEndpoint,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let endpoint_name = self
            .store
            .read(cx)
            .endpoints()
            .first()
            .map(|state| state.endpoint.name.clone());
        if let Some(endpoint_name) = endpoint_name {
            self.store
                .update(cx, |store, cx| store.refresh(&endpoint_name, cx));
        }
    }

    fn render_row(&self, row: &TreeRow, index: usize, cx: &Context<Self>) -> AnyElement {
        match row {
            TreeRow::Endpoint {
                name,
                status,
                expanded,
            } => self.render_endpoint_row(index, name, status, *expanded, cx),
            TreeRow::Category {
                label,
                node,
                expanded,
                count,
                ..
            } => self.render_category_row(index, label, node.clone(), *expanded, *count, cx),
            TreeRow::Container {
                name,
                state,
                status,
            } => Self::render_container_row(index, name, *state, status),
            TreeRow::Image { repository, tag } => Self::render_image_row(index, repository, tag),
            TreeRow::Compose { name, status } => Self::render_compose_row(index, name, status),
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
                    .child(Label::new(endpoint_name)),
            )
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_node(node.clone(), cx)));

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
        index: usize,
        name: &str,
        state: ContainerState,
        status: &str,
    ) -> AnyElement {
        let indicator_color = match state {
            ContainerState::Running => Color::Success,
            ContainerState::Exited | ContainerState::Dead => Color::Error,
            ContainerState::Paused | ContainerState::Restarting => Color::Warning,
            ContainerState::Created | ContainerState::Unknown => Color::Muted,
        };
        ListItem::new(index)
            .indent_level(2)
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
            .into_any_element()
    }

    fn render_image_row(index: usize, repository: &str, tag: &str) -> AnyElement {
        let label = format!("{repository}:{tag}");
        ListItem::new(index)
            .indent_level(2)
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
            .into_any_element()
    }

    fn render_compose_row(index: usize, name: &str, status: &str) -> AnyElement {
        ListItem::new(index)
            .indent_level(2)
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
            .into_any_element()
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
}
