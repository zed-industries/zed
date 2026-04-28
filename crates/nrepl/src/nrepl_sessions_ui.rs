//! UI surface for the nREPL sessions panel and the workspace-level
//! `nrepl::Connect` / `nrepl::Disconnect` / `nrepl::Sessions` actions.
//!
//! Modeled on `crates/repl/src/repl_sessions_ui.rs`. The store side
//! (which owns the actual connections and their lifetime) lives in
//! `nrepl_store.rs`; this module is purely the workspace-facing UI.

use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, Subscription, actions,
    prelude::*,
};
use ui::{ButtonLike, KeyBinding, prelude::*};
use workspace::item::ItemEvent;
use workspace::{Toast, Workspace, item::Item, notifications::NotificationId};

use crate::nrepl_settings::NreplSettings;
use crate::nrepl_store::{ConnectTarget, ConnectionState, NreplConnection, NreplStore};

actions!(
    nrepl,
    [
        /// Connects to an nREPL server. Auto-discovers a port via
        /// `.nrepl-port` in any visible local worktree first; reports a
        /// failure state in the sessions panel if nothing is found.
        Connect,
        /// Disconnects from the current workspace's nREPL server.
        Disconnect,
        /// Opens the nREPL sessions panel.
        Sessions,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &Sessions, window, cx| {
                show_sessions_page(workspace, window, cx);
            });

            workspace.register_action(|workspace, _: &Connect, _window, cx| {
                if !NreplSettings::enabled(cx) {
                    return;
                }
                let weak = workspace.weak_handle();
                let store = NreplStore::global(cx);
                store.update(cx, |store, cx| {
                    store.connect(weak, ConnectTarget::Auto, cx);
                });
            });

            workspace.register_action(|workspace, _: &Disconnect, _window, cx| {
                if !NreplSettings::enabled(cx) {
                    return;
                }
                let workspace_id = cx.entity_id();
                let store = NreplStore::global(cx);
                let removed = store.update(cx, |store, cx| store.disconnect(workspace_id, cx));
                if !removed {
                    let id = NotificationId::unique::<NreplDisconnectMissing>();
                    workspace.show_toast(
                        Toast::new(id, "No active nREPL connection to disconnect.").autohide(),
                        cx,
                    );
                }
            });
        },
    )
    .detach();
}

/// Marker type for the "nothing to disconnect" toast id.
struct NreplDisconnectMissing;

fn show_sessions_page(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let existing = workspace
        .active_pane()
        .read(cx)
        .items()
        .find_map(|item| item.downcast::<NreplSessionsPage>());

    if let Some(existing) = existing {
        workspace.activate_item(&existing, true, true, window, cx);
    } else {
        let page = NreplSessionsPage::new(window, cx);
        workspace.add_item_to_active_pane(Box::new(page), None, true, window, cx);
    }
}

pub struct NreplSessionsPage {
    focus_handle: FocusHandle,
    // Re-renders the page when the set of connections changes (connect /
    // disconnect / settings flip).
    _store_subscription: Subscription,
    // One observer per live connection so per-connection state
    // transitions (Resolving -> Connecting -> Connected / Failed) repaint
    // the page. Replaced wholesale whenever the store fires.
    connection_subscriptions: Vec<Subscription>,
    _focus_subscriptions: Vec<Subscription>,
}

impl NreplSessionsPage {
    pub fn new(window: &mut Window, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();
            let store = NreplStore::global(cx);
            let connection_subscriptions = subscribe_to_connections(&store, cx);

            let store_subscription = cx.observe(&store, |this, store, cx| {
                this.connection_subscriptions = subscribe_to_connections(&store, cx);
                cx.notify();
            });

            let focus_subscriptions = vec![
                cx.on_focus_in(&focus_handle, window, |_, _, cx| cx.notify()),
                cx.on_focus_out(&focus_handle, window, |_, _, _, cx| cx.notify()),
            ];

            Self {
                focus_handle,
                _store_subscription: store_subscription,
                connection_subscriptions,
                _focus_subscriptions: focus_subscriptions,
            }
        })
    }
}

fn subscribe_to_connections(
    store: &Entity<NreplStore>,
    cx: &mut Context<NreplSessionsPage>,
) -> Vec<Subscription> {
    // Collect first so we can release the immutable borrow on cx before
    // calling cx.observe.
    let connections: Vec<Entity<NreplConnection>> = store.read(cx).connections().cloned().collect();
    connections
        .iter()
        .map(|conn| cx.observe(conn, |_, _, cx| cx.notify()))
        .collect()
}

impl EventEmitter<ItemEvent> for NreplSessionsPage {}

impl Focusable for NreplSessionsPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for NreplSessionsPage {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "nREPL Sessions".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("nREPL Session Started")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl Render for NreplSessionsPage {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !NreplSettings::enabled(cx) {
            return NreplSessionsContainer::new("nREPL is disabled").child(Label::new(
                "Set `\"nrepl\": { \"enabled\": true }` in your settings.json to enable nREPL.",
            ));
        }

        let store = NreplStore::global(cx);
        let connections: Vec<Entity<NreplConnection>> =
            store.read(cx).connections().cloned().collect();

        if connections.is_empty() {
            let instructions = "Start an nREPL server (for example `clj -M:nrepl`, \
                 `lein repl`, or `bb nrepl-server`), then run the `nrepl::Connect` \
                 command from the command palette to connect.";
            return NreplSessionsContainer::new("No nREPL Connections").child(
                v_flex()
                    .gap_2()
                    .child(Label::new(instructions))
                    .child(KeyBinding::for_action(&Connect, cx)),
            );
        }

        NreplSessionsContainer::new("nREPL Connections").children(connections)
    }
}

impl Render for NreplConnection {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let workspace = self.workspace().clone();
        let entity_id = cx.entity_id();

        let (status, detail, action_button): (
            SharedString,
            Option<SharedString>,
            Option<AnyElement>,
        ) = match self.state() {
            ConnectionState::Resolving => ("Resolving…".into(), None, None),
            ConnectionState::Connecting { addr } => {
                ("Connecting".into(), Some(addr.to_string().into()), None)
            }
            ConnectionState::Connected { addr, session, .. } => {
                let detail: SharedString = format!("{addr} · session {session}").into();
                let button = ButtonLike::new(("nrepl-disconnect", entity_id))
                    .style(ButtonStyle::Subtle)
                    .child(Label::new("Disconnect"))
                    .on_click(move |_, _window, cx| {
                        let workspace_id = workspace.entity_id();
                        let store = NreplStore::global(cx);
                        store.update(cx, |store, cx| {
                            store.disconnect(workspace_id, cx);
                        });
                    })
                    .into_any_element();
                ("Connected".into(), Some(detail), Some(button))
            }
            ConnectionState::Failed { error } => {
                let button = ButtonLike::new(("nrepl-retry", entity_id))
                    .style(ButtonStyle::Subtle)
                    .child(Label::new("Retry"))
                    .on_click(move |_, _window, cx| {
                        let store = NreplStore::global(cx);
                        store.update(cx, |store, cx| {
                            store.connect(workspace.clone(), ConnectTarget::Auto, cx);
                        });
                    })
                    .into_any_element();
                ("Failed".into(), Some(error.clone()), Some(button))
            }
        };

        v_flex()
            .gap_1()
            .p_3()
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new(status))
                    .children(action_button),
            )
            .when_some(detail, |this, detail| {
                this.child(
                    Label::new(detail)
                        .color(Color::Muted)
                        .size(LabelSize::Small),
                )
            })
    }
}

#[derive(IntoElement)]
struct NreplSessionsContainer {
    title: SharedString,
    children: Vec<AnyElement>,
}

impl NreplSessionsContainer {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for NreplSessionsContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for NreplSessionsContainer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .p_4()
            .gap_2()
            .size_full()
            .child(Label::new(self.title).size(LabelSize::Large))
            .children(self.children)
    }
}
