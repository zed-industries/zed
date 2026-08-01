//! Exports Zed's application menu (see [`gpui::Menu`]) over the `com.canonical.dbusmenu`
//! D-Bus interface, and attaches it to windows so desktop environments with a "global menu"
//! panel widget (KDE Plasma's Application Menu Bar, and Unity-derived shells) can render it
//! outside of Zed's own window content.
//!
//! Two independent association mechanisms are used, since there's no single standard:
//!
//! - X11 (and XWayland): registering with the `com.canonical.AppMenu.Registrar` service, keyed
//!   by the window's XID. See [`DbusMenuService::register_x11_window`].
//! - Native Wayland under KWin: attaching the D-Bus service name + object path directly to the
//!   `wl_surface` via KDE's `org_kde_kwin_appmenu` protocol. See [`DbusMenuService::address`],
//!   which the Wayland backend polls when (re-)configuring a window.
//!
//! Zed's menu tree is process-global (not per-window), so a single `com.canonical.dbusmenu`
//! object is exported for the whole process and every window points at the same object path.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use calloop::channel::Sender;
use gpui::{Action, BackgroundExecutor, KeybindingKeystroke, Keymap, OwnedMenu, OwnedMenuItem};
use gpui_util::ResultExt as _;
use serde::Serialize;
use zbus::{
    self, interface,
    object_server::SignalEmitter,
    zvariant::{OwnedObjectPath, OwnedValue, Type, Value},
};

use crate::linux::LinuxPlatformEvent;

const MENU_OBJECT_PATH: &str = "/org/zed_industries/Zed/MenuBar";
const REGISTRAR_BUS_NAME: &str = "com.canonical.AppMenu.Registrar";
const REGISTRAR_OBJECT_PATH: &str = "/com/canonical/AppMenu/Registrar";
const REGISTRAR_INTERFACE: &str = "com.canonical.AppMenu.Registrar";

/// The wire representation of a single DBusMenu layout node: `(ia{sv}av)`. Each entry in
/// `children` is a variant wrapping another `RawLayoutItem`, matching the protocol's recursive
/// layout encoding.
#[derive(Debug, Clone, Serialize, Type, Value, OwnedValue)]
struct RawLayoutItem {
    id: i32,
    properties: HashMap<String, OwnedValue>,
    children: Vec<OwnedValue>,
}

enum NodeKind {
    Root,
    Standard,
    Separator,
}

struct MenuNode {
    children: Vec<i32>,
    kind: NodeKind,
    label: String,
    enabled: bool,
    is_submenu: bool,
    toggled: Option<bool>,
    shortcut: Vec<Vec<String>>,
    action: Option<Box<dyn Action>>,
}

impl MenuNode {
    fn root() -> Self {
        Self {
            children: Vec::new(),
            kind: NodeKind::Root,
            label: String::new(),
            enabled: true,
            is_submenu: true,
            toggled: None,
            shortcut: Vec::new(),
            action: None,
        }
    }

    fn properties(&self, requested: &[String]) -> HashMap<String, OwnedValue> {
        let mut all = HashMap::new();
        let mut insert = |key: &str, value: Value<'_>| match OwnedValue::try_from(value) {
            Ok(value) => {
                all.insert(key.to_string(), value);
            }
            Err(error) => {
                log::warn!("dbusmenu: failed to encode {key:?} property: {error:?}");
            }
        };

        match self.kind {
            NodeKind::Separator => insert("type", Value::from("separator")),
            NodeKind::Root | NodeKind::Standard => {
                insert("label", Value::from(escape_mnemonic(&self.label)));
                if !self.enabled {
                    insert("enabled", Value::from(false));
                }
                if self.is_submenu {
                    insert("children-display", Value::from("submenu"));
                }
                if let Some(toggled) = self.toggled {
                    insert("toggle-type", Value::from("checkmark"));
                    insert(
                        "toggle-state",
                        Value::from(if toggled { 1i32 } else { 0i32 }),
                    );
                }
                if !self.shortcut.is_empty() {
                    insert("shortcut", Value::from(self.shortcut.clone()));
                }
            }
        }

        if requested.is_empty() {
            all
        } else {
            all.retain(|key, _| requested.iter().any(|name| name == key));
            all
        }
    }
}

/// Escapes literal underscores, since DBusMenu (like GTK) treats a single `_` as marking the
/// following character as a mnemonic accelerator.
fn escape_mnemonic(label: &str) -> String {
    label.replace('_', "__")
}

fn shortcut_for_action(action: &dyn Action, keymap: &Keymap) -> Vec<Vec<String>> {
    keymap
        .bindings_for_action(action)
        .rev()
        .find_map(|binding| match binding.keystrokes() {
            [keystroke] => Some(shortcut_tokens(keystroke)),
            _ => None,
        })
        .map(|tokens| vec![tokens])
        .unwrap_or_default()
}

fn shortcut_tokens(keystroke: &KeybindingKeystroke) -> Vec<String> {
    let modifiers = keystroke.modifiers();
    let mut tokens = Vec::with_capacity(5);
    if modifiers.control {
        tokens.push("Control".to_string());
    }
    if modifiers.alt {
        tokens.push("Alt".to_string());
    }
    if modifiers.shift {
        tokens.push("Shift".to_string());
    }
    if modifiers.platform {
        tokens.push("Super".to_string());
    }
    tokens.push(display_key_name(keystroke.key()));
    tokens
}

fn display_key_name(key: &str) -> String {
    match key {
        "escape" => "Escape".to_string(),
        "enter" => "Return".to_string(),
        "tab" => "Tab".to_string(),
        "space" => "Space".to_string(),
        "backspace" => "BackSpace".to_string(),
        "delete" => "Delete".to_string(),
        "up" => "Up".to_string(),
        "down" => "Down".to_string(),
        "left" => "Left".to_string(),
        "right" => "Right".to_string(),
        "pageup" => "Prior".to_string(),
        "pagedown" => "Next".to_string(),
        "home" => "Home".to_string(),
        "end" => "End".to_string(),
        "insert" => "Insert".to_string(),
        other if other.chars().count() == 1 => other.to_uppercase(),
        other => {
            let mut chars = other.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        }
    }
}

fn sanitize_items(items: Vec<OwnedMenuItem>) -> Vec<OwnedMenuItem> {
    let mut cleaned = Vec::with_capacity(items.len());
    let mut last_was_separator = false;

    for item in items {
        match item {
            OwnedMenuItem::Separator => {
                if !last_was_separator {
                    cleaned.push(item);
                    last_was_separator = true;
                }
            }
            OwnedMenuItem::Submenu(submenu) => {
                if !submenu.items.is_empty() {
                    cleaned.push(OwnedMenuItem::Submenu(submenu));
                    last_was_separator = false;
                }
            }
            OwnedMenuItem::SystemMenu(_) => {}
            item => {
                cleaned.push(item);
                last_was_separator = false;
            }
        }
    }

    if matches!(cleaned.last(), Some(OwnedMenuItem::Separator)) {
        cleaned.pop();
    }

    cleaned
}

/// The current menu tree, flattened into an id-indexed map (root = 0). Rebuilt wholesale on
/// every [`DbusMenuService::update`] call rather than diffed, since Zed's menu content is
/// already fully materialized by the time it reaches the platform layer.
pub(crate) struct MenuTree {
    nodes: HashMap<i32, MenuNode>,
    next_id: i32,
    revision: u32,
}

impl MenuTree {
    fn empty() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(0, MenuNode::root());
        Self {
            nodes,
            next_id: 1,
            revision: 0,
        }
    }

    fn alloc_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn push_child(&mut self, parent: i32, id: i32, node: MenuNode) {
        self.nodes.insert(id, node);
        if let Some(parent) = self.nodes.get_mut(&parent) {
            parent.children.push(id);
        }
    }

    fn rebuild(&mut self, menus: &[OwnedMenu], keymap: &Keymap) {
        self.nodes.clear();
        self.nodes.insert(0, MenuNode::root());
        self.next_id = 1;
        self.revision = self.revision.wrapping_add(1);

        for menu in menus {
            if menu.items.is_empty() {
                continue;
            }
            let id = self.alloc_id();
            self.push_child(
                0,
                id,
                MenuNode {
                    children: Vec::new(),
                    kind: NodeKind::Standard,
                    label: menu.name.to_string(),
                    enabled: !menu.disabled,
                    is_submenu: true,
                    toggled: None,
                    shortcut: Vec::new(),
                    action: None,
                },
            );
            self.add_items(id, menu.items.clone(), keymap);
        }
    }

    fn add_items(&mut self, parent: i32, items: Vec<OwnedMenuItem>, keymap: &Keymap) {
        for item in sanitize_items(items) {
            match item {
                OwnedMenuItem::Separator => {
                    let id = self.alloc_id();
                    self.push_child(
                        parent,
                        id,
                        MenuNode {
                            children: Vec::new(),
                            kind: NodeKind::Separator,
                            label: String::new(),
                            enabled: true,
                            is_submenu: false,
                            toggled: None,
                            shortcut: Vec::new(),
                            action: None,
                        },
                    );
                }
                OwnedMenuItem::Submenu(submenu) => {
                    let id = self.alloc_id();
                    self.push_child(
                        parent,
                        id,
                        MenuNode {
                            children: Vec::new(),
                            kind: NodeKind::Standard,
                            label: submenu.name.to_string(),
                            enabled: !submenu.disabled,
                            is_submenu: true,
                            toggled: None,
                            shortcut: Vec::new(),
                            action: None,
                        },
                    );
                    self.add_items(id, submenu.items, keymap);
                }
                OwnedMenuItem::Action {
                    name,
                    action,
                    checked,
                    disabled,
                    ..
                } => {
                    let id = self.alloc_id();
                    let shortcut = shortcut_for_action(action.as_ref(), keymap);
                    self.push_child(
                        parent,
                        id,
                        MenuNode {
                            children: Vec::new(),
                            kind: NodeKind::Standard,
                            label: name,
                            enabled: !disabled,
                            is_submenu: false,
                            toggled: checked.then_some(true),
                            shortcut,
                            action: Some(action),
                        },
                    );
                }
                OwnedMenuItem::SystemMenu(_) => {}
            }
        }
    }

    fn layout_for(&self, id: i32, depth: i32, names: &[String]) -> Option<RawLayoutItem> {
        let node = self.nodes.get(&id)?;
        let properties = node.properties(names);
        let children = if depth == 0 {
            Vec::new()
        } else {
            let next_depth = if depth < 0 { -1 } else { depth - 1 };
            node.children
                .iter()
                .filter_map(|child_id| self.layout_for(*child_id, next_depth, names))
                .filter_map(|child| match OwnedValue::try_from(child) {
                    Ok(value) => Some(value),
                    Err(error) => {
                        log::warn!("dbusmenu: failed to encode child layout: {error:?}");
                        None
                    }
                })
                .collect()
        };
        Some(RawLayoutItem {
            id,
            properties,
            children,
        })
    }

    fn group_properties(
        &self,
        ids: &[i32],
        names: &[String],
    ) -> Vec<(i32, HashMap<String, OwnedValue>)> {
        ids.iter()
            .filter_map(|id| self.nodes.get(id).map(|node| (*id, node.properties(names))))
            .collect()
    }

    fn clone_action(&self, id: i32) -> Option<Box<dyn Action>> {
        self.nodes
            .get(&id)?
            .action
            .as_ref()
            .map(|action| action.boxed_clone())
    }
}

pub(crate) struct DbusMenuInterface {
    state: Arc<Mutex<MenuTree>>,
    events: Sender<LinuxPlatformEvent>,
}

#[interface(name = "com.canonical.dbusmenu")]
impl DbusMenuInterface {
    #[zbus(property(emits_changed_signal = "const"))]
    fn version(&self) -> u32 {
        4
    }

    #[zbus(property(emits_changed_signal = "const"))]
    fn text_direction(&self) -> String {
        "ltr".to_string()
    }

    #[zbus(property(emits_changed_signal = "const"))]
    fn status(&self) -> String {
        "normal".to_string()
    }

    #[zbus(property(emits_changed_signal = "const"))]
    fn icon_theme_path(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: Vec<String>,
    ) -> zbus::fdo::Result<(u32, RawLayoutItem)> {
        let tree = self.state.lock().unwrap();
        let layout = tree
            .layout_for(parent_id, recursion_depth, &property_names)
            .ok_or_else(|| zbus::fdo::Error::Failed(format!("no menu item with id {parent_id}")))?;
        Ok((tree.revision, layout))
    }

    fn get_group_properties(
        &self,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> Vec<(i32, HashMap<String, OwnedValue>)> {
        self.state
            .lock()
            .unwrap()
            .group_properties(&ids, &property_names)
    }

    fn get_property(&self, id: i32, name: String) -> zbus::fdo::Result<OwnedValue> {
        let tree = self.state.lock().unwrap();
        tree.nodes
            .get(&id)
            .ok_or_else(|| zbus::fdo::Error::Failed(format!("no menu item with id {id}")))?
            .properties(std::slice::from_ref(&name))
            .remove(&name)
            .ok_or_else(|| zbus::fdo::Error::UnknownProperty(name))
    }

    fn event(&self, id: i32, event_id: String, _data: OwnedValue, _timestamp: u32) {
        if event_id != "clicked" {
            return;
        }
        if let Some(action) = self.state.lock().unwrap().clone_action(id) {
            self.events
                .send(LinuxPlatformEvent::MenuAction(action))
                .ok();
        }
    }

    fn event_group(&self, events: Vec<(i32, String, OwnedValue, u32)>) -> Vec<i32> {
        let tree = self.state.lock().unwrap();
        let mut id_errors = Vec::new();
        for (id, event_id, _data, _timestamp) in events {
            if !tree.nodes.contains_key(&id) {
                id_errors.push(id);
                continue;
            }
            if event_id == "clicked"
                && let Some(action) = tree.clone_action(id)
            {
                self.events
                    .send(LinuxPlatformEvent::MenuAction(action))
                    .ok();
            }
        }
        id_errors
    }

    fn about_to_show(&self, _id: i32) -> bool {
        self.events.send(LinuxPlatformEvent::MenuWillOpen).ok();
        false
    }

    fn about_to_show_group(&self, ids: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        self.events.send(LinuxPlatformEvent::MenuWillOpen).ok();
        let tree = self.state.lock().unwrap();
        let id_errors = ids
            .into_iter()
            .filter(|id| !tree.nodes.contains_key(id))
            .collect();
        (Vec::new(), id_errors)
    }

    #[zbus(signal)]
    async fn layout_updated(
        emitter: &SignalEmitter<'_>,
        revision: u32,
        parent: i32,
    ) -> zbus::Result<()>;
}

struct ConnectionInfo {
    connection: zbus::Connection,
    unique_name: String,
    path: OwnedObjectPath,
}

/// Owns the lazily-started D-Bus connection that hosts [`DbusMenuInterface`], plus the
/// bookkeeping needed to associate it with windows on both the X11 and Wayland backends.
///
/// Lives in `LinuxCommon`, so it's confined to the main thread; the actual D-Bus connect/serve
/// work happens on the background executor and reports back via the `LinuxPlatformEvent`
/// calloop channel, since `zbus::Connection` needs to be driven from *some* executor and
/// `LinuxCommon`'s `Rc`-based state can't cross threads.
pub(crate) struct DbusMenuService {
    state: Arc<Mutex<MenuTree>>,
    background_executor: BackgroundExecutor,
    events: Sender<LinuxPlatformEvent>,
    connection: Rc<RefCell<Option<ConnectionInfo>>>,
    pending_x11_windows: Rc<RefCell<Vec<u32>>>,
    started: std::cell::Cell<bool>,
}

impl DbusMenuService {
    pub(crate) fn new(
        background_executor: BackgroundExecutor,
        events: Sender<LinuxPlatformEvent>,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(MenuTree::empty())),
            background_executor,
            events,
            connection: Rc::new(RefCell::new(None)),
            pending_x11_windows: Rc::new(RefCell::new(Vec::new())),
            started: std::cell::Cell::new(false),
        }
    }

    /// Rebuilds the exported menu tree from the latest `set_menus` call and, if the D-Bus
    /// service is already up, notifies any listening global-menu consumers.
    pub(crate) fn update(&self, menus: &[OwnedMenu], keymap: &Keymap) {
        self.ensure_started();

        let revision = {
            let mut tree = self.state.lock().unwrap();
            tree.rebuild(menus, keymap);
            tree.revision
        };

        if let Some(info) = self.connection.borrow().as_ref() {
            let connection = info.connection.clone();
            let path = info.path.clone();
            self.background_executor
                .spawn(async move {
                    if let Ok(iface_ref) = connection
                        .object_server()
                        .interface::<_, DbusMenuInterface>(&path)
                        .await
                    {
                        DbusMenuInterface::layout_updated(iface_ref.signal_emitter(), revision, 0)
                            .await
                            .log_err();
                    }
                })
                .detach();
        }
    }

    fn ensure_started(&self) {
        if self.started.replace(true) {
            return;
        }

        let state = self.state.clone();
        let events = self.events.clone();
        self.background_executor
            .spawn(async move {
                if let Err(error) = start_service(state, events).await {
                    log::warn!("failed to start global menu (DBusMenu) service: {error:?}");
                }
            })
            .detach();
    }

    /// Called once the background connect task finishes, from the main-thread calloop handler
    /// for `LinuxPlatformEvent::MenuServiceReady`.
    pub(crate) fn set_connection(&self, connection: zbus::Connection, path: OwnedObjectPath) {
        let unique_name = connection
            .unique_name()
            .map(|name| name.to_string())
            .unwrap_or_default();
        *self.connection.borrow_mut() = Some(ConnectionInfo {
            connection: connection.clone(),
            unique_name,
            path: path.clone(),
        });

        for xid in self.pending_x11_windows.borrow_mut().split_off(0) {
            self.spawn_register_x11(connection.clone(), path.clone(), xid);
        }
    }

    /// The `(unique bus name, object path)` pair to hand to KDE's `org_kde_kwin_appmenu::set_address`
    /// on Wayland, once the D-Bus connection is up.
    pub(crate) fn address(&self) -> Option<(String, OwnedObjectPath)> {
        self.connection
            .borrow()
            .as_ref()
            .map(|info| (info.unique_name.clone(), info.path.clone()))
    }

    pub(crate) fn register_x11_window(&self, xid: u32) {
        match self.connection.borrow().as_ref() {
            Some(info) => self.spawn_register_x11(info.connection.clone(), info.path.clone(), xid),
            None => self.pending_x11_windows.borrow_mut().push(xid),
        }
    }

    pub(crate) fn unregister_x11_window(&self, xid: u32) {
        self.pending_x11_windows
            .borrow_mut()
            .retain(|pending| *pending != xid);
        let Some(connection) = self
            .connection
            .borrow()
            .as_ref()
            .map(|info| info.connection.clone())
        else {
            return;
        };
        self.background_executor
            .spawn(async move {
                if let Ok(proxy) = zbus::Proxy::new(
                    &connection,
                    REGISTRAR_BUS_NAME,
                    REGISTRAR_OBJECT_PATH,
                    REGISTRAR_INTERFACE,
                )
                .await
                {
                    // Best-effort: the registrar also drops entries automatically when our
                    // connection closes, so a failure here isn't user-visible.
                    proxy
                        .call_method("UnregisterWindow", &(xid,))
                        .await
                        .log_err();
                }
            })
            .detach();
    }

    fn spawn_register_x11(&self, connection: zbus::Connection, path: OwnedObjectPath, xid: u32) {
        self.background_executor
            .spawn(async move {
                let proxy = match zbus::Proxy::new(
                    &connection,
                    REGISTRAR_BUS_NAME,
                    REGISTRAR_OBJECT_PATH,
                    REGISTRAR_INTERFACE,
                )
                .await
                {
                    Ok(proxy) => proxy,
                    Err(_) => return,
                };
                match proxy.call_method("RegisterWindow", &(xid, path)).await {
                    Ok(_) => {}
                    Err(error) => {
                        log::debug!("com.canonical.AppMenu.Registrar unavailable: {error:?}");
                    }
                }
            })
            .detach();
    }
}

async fn start_service(
    state: Arc<Mutex<MenuTree>>,
    events: Sender<LinuxPlatformEvent>,
) -> anyhow::Result<()> {
    let path = OwnedObjectPath::try_from(MENU_OBJECT_PATH)?;
    let interface = DbusMenuInterface {
        state,
        events: events.clone(),
    };
    let connection = zbus::connection::Builder::session()?
        .serve_at(MENU_OBJECT_PATH, interface)?
        .build()
        .await?;

    events
        .send(LinuxPlatformEvent::MenuServiceReady(connection, path))
        .map_err(|_| anyhow::anyhow!("linux event loop channel closed"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Keymap, Menu, MenuItem, NoAction, OwnedMenu, OwnedMenuItem, SharedString};

    fn test_keymap() -> Keymap {
        Keymap::default()
    }

    fn action_item(name: &str) -> OwnedMenuItem {
        OwnedMenuItem::Action {
            name: name.to_string(),
            action: Box::new(NoAction),
            os_action: None,
            checked: false,
            disabled: false,
        }
    }

    fn submenu(name: &str, items: Vec<OwnedMenuItem>) -> OwnedMenuItem {
        OwnedMenuItem::Submenu(OwnedMenu {
            name: SharedString::from(name),
            items,
            disabled: false,
        })
    }

    #[test]
    fn test_escape_mnemonic() {
        assert_eq!(escape_mnemonic("File"), "File");
        assert_eq!(escape_mnemonic("_File"), "__File");
        assert_eq!(escape_mnemonic("Save _As"), "Save __As");
        // Each literal underscore is doubled: "__" becomes "____"
        assert_eq!(escape_mnemonic("__"), "____");
    }

    #[test]
    fn test_sanitize_removes_duplicate_separators() {
        let items = vec![
            action_item("A"),
            OwnedMenuItem::Separator,
            OwnedMenuItem::Separator,
            action_item("B"),
        ];
        let cleaned = sanitize_items(items);
        assert_eq!(cleaned.len(), 3); // A, separator, B
    }

    #[test]
    fn test_sanitize_keeps_first_separator_preserves_leading() {
        let items = vec![
            OwnedMenuItem::Separator,
            action_item("A"),
        ];
        let cleaned = sanitize_items(items);
        // Leading separators are preserved; only duplicates and trailing are removed
        assert_eq!(cleaned.len(), 2);
    }

    #[test]
    fn test_sanitize_removes_trailing_separator() {
        let items = vec![
            action_item("A"),
            OwnedMenuItem::Separator,
        ];
        let cleaned = sanitize_items(items);
        assert_eq!(cleaned.len(), 1); // just A
    }

    #[test]
    fn test_sanitize_removes_empty_submenus() {
        let items = vec![
            action_item("A"),
            submenu("Empty", vec![]),
            action_item("B"),
        ];
        let cleaned = sanitize_items(items);
        assert_eq!(cleaned.len(), 2); // A, B
    }

    #[test]
    fn test_menu_tree_rebuild_and_layout() {
        let mut tree = MenuTree::empty();
        let keymap = test_keymap();

        let menus = vec![OwnedMenu {
            name: SharedString::from("File"),
            disabled: false,
            items: vec![
                action_item("New"),
                OwnedMenuItem::Separator,
                action_item("Quit"),
            ],
        }];

        tree.rebuild(&menus, &keymap);

        // Root node should have one child (the File menu)
        let root_layout = tree.layout_for(0, 1, &[]).expect("root layout exists");
        assert_eq!(root_layout.id, 0);
        assert_eq!(root_layout.children.len(), 1);

        // The File menu child should contain our items
        let file_child = &root_layout.children[0];
        // Each child is a variant wrapping another RawLayoutItem
        // The File menu itself has 3 children: New, separator, Quit
        // But at depth 1 we only get the File menu's direct children
    }

    #[test]
    fn test_menu_tree_skips_empty_top_level_menus() {
        let mut tree = MenuTree::empty();
        let keymap = test_keymap();

        let menus = vec![OwnedMenu {
            name: SharedString::from("Empty"),
            disabled: false,
            items: vec![],
        }];

        tree.rebuild(&menus, &keymap);

        let root_layout = tree.layout_for(0, 1, &[]).expect("root layout exists");
        assert_eq!(root_layout.children.len(), 0);
    }

    #[test]
    fn test_display_key_name() {
        assert_eq!(display_key_name("escape"), "Escape");
        assert_eq!(display_key_name("enter"), "Return");
        assert_eq!(display_key_name("space"), "Space");
        assert_eq!(display_key_name("up"), "Up");
        assert_eq!(display_key_name("pageup"), "Prior");
        assert_eq!(display_key_name("a"), "A");
        assert_eq!(display_key_name("z"), "Z");
        assert_eq!(display_key_name("f1"), "F1");
    }
}
