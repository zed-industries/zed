use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::OnceLock;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use calloop::channel::Sender as CalloopSender;
use gpui::{Action, KeyContext, Keymap, KeybindingKeystroke, OwnedMenu, OwnedMenuItem};
use zbus::zvariant::{OwnedValue, Value};

pub const DBUSMENU_OBJECT_PATH: &str = "/MenuBar";

type ActionCallback = dyn Fn(Box<dyn Action>) + Send + Sync;
type WillOpenCallback = dyn Fn() + Send + Sync;

#[derive(Clone)]
pub enum DBusMenuCommand {
    EnsureExported {
        object_path: String,
        responded: std::sync::mpsc::Sender<bool>,
    },
    LayoutUpdated {
        revision: u32,
        parent: i32,
        object_paths: Vec<String>,
    },
    ItemsPropertiesUpdated {
        updated_props: Vec<(i32, HashMap<String, OwnedValue>)>,
        removed_props: Vec<(i32, Vec<String>)>,
        object_paths: Vec<String>,
    },
    Unexport {
        object_path: String,
    },
}

pub struct ValidateRequest {
    pub action: Box<dyn Action>,
    pub responded: std::sync::mpsc::Sender<bool>,
}

struct MenuItemEntry {
    action: Option<Box<dyn Action>>,
    enabled: Option<bool>,
    properties: HashMap<String, OwnedValue>,
    children: Vec<i32>,
}

struct MenuState {
    items: HashMap<i32, MenuItemEntry>,
    revision: u32,
}

#[derive(Clone)]
pub struct DBusMenuServer {
    state: Arc<Mutex<MenuState>>,
    action_callback: Arc<Mutex<Option<Box<ActionCallback>>>>,
    will_open_callback: Arc<Mutex<Option<Arc<WillOpenCallback>>>>,
    command_sender: Arc<Mutex<Option<async_channel::Sender<DBusMenuCommand>>>>,
    validate_sender: Arc<Mutex<Option<CalloopSender<ValidateRequest>>>>,
    connected: Arc<AtomicBool>,
    object_paths: Arc<Mutex<HashSet<String>>>,
    exported_paths: Arc<Mutex<HashSet<String>>>,
}

impl DBusMenuServer {
    pub fn new() -> Self {
        let mut items = HashMap::new();
        items.insert(
            0,
            MenuItemEntry {
                action: None,
                enabled: None,
                properties: root_properties(),
                children: Vec::new(),
            },
        );

        let mut object_paths = HashSet::new();
        object_paths.insert(DBUSMENU_OBJECT_PATH.to_string());

        let mut exported_paths = HashSet::new();
        exported_paths.insert(DBUSMENU_OBJECT_PATH.to_string());

        Self {
            state: Arc::new(Mutex::new(MenuState { items, revision: 1 })),
            action_callback: Arc::new(Mutex::new(None)),
            will_open_callback: Arc::new(Mutex::new(None)),
            command_sender: Arc::new(Mutex::new(None)),
            validate_sender: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
            object_paths: Arc::new(Mutex::new(object_paths)),
            exported_paths: Arc::new(Mutex::new(exported_paths)),
        }
    }

    pub fn set_command_sender(&self, sender: async_channel::Sender<DBusMenuCommand>) {
        match self.command_sender.lock() {
            Ok(mut slot) => *slot = Some(sender),
            Err(error) => {
                log::error!("Failed to store DBusMenu command sender: {error}");
            }
        }
    }

    pub fn set_validate_sender(&self, sender: CalloopSender<ValidateRequest>) {
        match self.validate_sender.lock() {
            Ok(mut slot) => *slot = Some(sender),
            Err(error) => {
                log::error!("Failed to store DBusMenu validate sender: {error}");
            }
        }
    }

    pub fn mark_connected(&self) {
        self.connected.store(true, Ordering::SeqCst);

        let revision = match self.state.lock() {
            Ok(state) => state.revision,
            Err(error) => {
                log::error!("Failed to read DBusMenu revision on connect: {error}");
                return;
            }
        };
        self.request_layout_updated(revision);
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    pub fn note_exported(&self, object_path: String) {
        if let Ok(mut exported_paths) = self.exported_paths.lock() {
            exported_paths.insert(object_path);
        }
    }

    pub fn set_action_callback(&self, callback: Box<ActionCallback>) {
        if let Ok(mut slot) = self.action_callback.lock() {
            *slot = Some(callback);
        } else {
            log::error!("Failed to store DBusMenu action callback due to lock poisoning");
        }
    }

    pub fn set_will_open_callback(&self, callback: Arc<WillOpenCallback>) {
        if let Ok(mut slot) = self.will_open_callback.lock() {
            *slot = Some(callback);
        } else {
            log::error!("Failed to store DBusMenu will-open callback due to lock poisoning");
        }
    }

    pub fn set_menus(&self, menus: Vec<OwnedMenu>, keymap: &Keymap) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(error) => {
                log::error!("Failed to update DBusMenu state: {error}");
                return;
            }
        };
        state.items.clear();

        let mut next_id: i32 = 1;
        let mut root_children = Vec::new();

        for menu in &menus {
            let submenu_id = next_id;
            next_id = next_id.wrapping_add(1);
            build_menu_tree(&mut state.items, &mut next_id, submenu_id, menu, keymap);
            root_children.push(submenu_id);
        }

        state.items.insert(
            0,
            MenuItemEntry {
                action: None,
                enabled: None,
                properties: root_properties(),
                children: root_children,
            },
        );

        state.revision = state.revision.wrapping_add(1);
        let revision = state.revision;
        drop(state);
        self.request_layout_updated(revision);
    }

    pub fn ensure_exported_blocking(&self, object_path: String, timeout: Duration) -> bool {
        let should_export = match self.object_paths.lock() {
            Ok(mut paths) => paths.insert(object_path.clone()),
            Err(error) => {
                log::error!("Failed to update DBusMenu object paths: {error}");
                return false;
            }
        };
        if !should_export {
            return true;
        }

        if match self.exported_paths.lock() {
            Ok(exported_paths) => exported_paths.contains(&object_path),
            Err(error) => {
                log::error!("Failed to read exported DBusMenu paths: {error}");
                false
            }
        } {
            return true;
        }

        if !self.is_connected() {
            return false;
        }

        let sender = match self.command_sender.lock() {
            Ok(sender) => sender.clone(),
            Err(error) => {
                log::error!("Failed to read DBusMenu command sender: {error}");
                return false;
            }
        };
        let Some(sender) = sender else {
            return false;
        };

        let (responded_tx, responded_rx) = std::sync::mpsc::channel();
        if let Err(error) = sender.try_send(DBusMenuCommand::EnsureExported {
            object_path: object_path.clone(),
            responded: responded_tx,
        }) {
            log::error!("Failed to send DBusMenu export request: {error}");
            return false;
        }

        let exported = match responded_rx.recv_timeout(timeout) {
            Ok(ok) => ok,
            Err(error) => {
                log::error!("Timed out exporting DBusMenu object at {object_path}: {error}");
                false
            }
        };

        if exported {
            let revision = match self.state.lock() {
                Ok(state) => state.revision,
                Err(error) => {
                    log::error!("Failed to read DBusMenu revision after export: {error}");
                    return true;
                }
            };
            self.request_layout_updated_for_paths(revision, vec![object_path]);
        }

        exported
    }

    pub fn unexport_object_path(&self, object_path: String) {
        if object_path == DBUSMENU_OBJECT_PATH {
            return;
        }

        if let Ok(mut object_paths) = self.object_paths.lock() {
            object_paths.remove(&object_path);
        }
        if let Ok(mut exported_paths) = self.exported_paths.lock() {
            exported_paths.remove(&object_path);
        }

        let sender = match self.command_sender.lock() {
            Ok(sender) => sender.clone(),
            Err(error) => {
                log::error!("Failed to read DBusMenu command sender: {error}");
                return;
            }
        };
        let Some(sender) = sender else {
            return;
        };

        if let Err(error) = sender.try_send(DBusMenuCommand::Unexport { object_path }) {
            log::error!("Failed to queue DBusMenu unexport request: {error}");
        }
    }

    pub fn refresh_enabled_states(&self, validate: &mut dyn FnMut(&dyn Action) -> bool) {
        let mut updated_props: Vec<(i32, HashMap<String, OwnedValue>)> = Vec::new();

        {
            let mut state = match self.state.lock() {
                Ok(state) => state,
                Err(error) => {
                    log::error!("Failed to access DBusMenu state for enable refresh: {error}");
                    return;
                }
            };

            for (id, entry) in state.items.iter_mut() {
                let Some(previous_enabled) = entry.enabled else {
                    continue;
                };
                let Some(action) = entry.action.as_ref() else {
                    continue;
                };

                let enabled = validate(action.as_ref());
                if enabled == previous_enabled {
                    continue;
                }

                entry.enabled = Some(enabled);
                if let Some(value) = owned_bool(enabled) {
                    entry.properties.insert("enabled".to_string(), value.clone());
                    let mut props = HashMap::new();
                    props.insert("enabled".to_string(), value);
                    updated_props.push((*id, props));
                }
            }
        }

        if updated_props.is_empty() {
            return;
        }

        self.request_items_properties_updated(updated_props, Vec::new());
    }

    fn validate_enabled(&self, action: &dyn Action) -> Option<bool> {
        let sender = match self.validate_sender.lock() {
            Ok(sender) => sender,
            Err(error) => {
                log::error!("Failed to read DBusMenu validate sender: {error}");
                return None;
            }
        };
        let Some(sender) = sender.as_ref() else {
            return None;
        };

        let (responded_tx, responded_rx) = std::sync::mpsc::channel();
        let request = ValidateRequest {
            action: action.boxed_clone(),
            responded: responded_tx,
        };

        if let Err(error) = sender.send(request) {
            log::error!("Failed to send DBusMenu validate request: {error}");
            return None;
        }

        responded_rx.recv_timeout(Duration::from_millis(20)).ok()
    }

    fn get_layout_node(
        &self,
        id: i32,
        remaining_depth: i32,
    ) -> Option<(i32, HashMap<String, OwnedValue>, Vec<OwnedValue>)> {
        let state = match self.state.lock() {
            Ok(state) => state,
            Err(error) => {
                log::error!("Failed to read DBusMenu state: {error}");
                return None;
            }
        };
        let entry = state.items.get(&id)?;

        let properties = entry.properties.clone();
        let children = if remaining_depth == 0 {
            Vec::new()
        } else {
            let child_ids = entry.children.clone();
            drop(state);
            let mut result = Vec::new();
            for child_id in child_ids {
                if let Some(child_node) = self.get_layout_node(child_id, remaining_depth - 1) {
                    let variant =
                        Value::from(zbus::zvariant::Structure::from(child_node)).try_into();
                    if let Ok(value) = variant {
                        result.push(value);
                    }
                }
            }
            result
        };

        Some((id, properties, children))
    }

    fn request_layout_updated(&self, revision: u32) {
        let object_paths = self.object_paths();
        self.request_layout_updated_for_paths(revision, object_paths);
    }

    fn request_layout_updated_for_paths(&self, revision: u32, object_paths: Vec<String>) {
        if !self.is_connected() {
            return;
        }

        let sender = match self.command_sender.lock() {
            Ok(sender) => sender.clone(),
            Err(error) => {
                log::error!("Failed to read DBusMenu command sender: {error}");
                return;
            }
        };
        let Some(sender) = sender else {
            return;
        };

        if let Err(error) = sender.try_send(DBusMenuCommand::LayoutUpdated {
            revision,
            parent: 0,
            object_paths,
        }) {
            log::error!("Failed to queue DBusMenu LayoutUpdated signal: {error}");
        }
    }

    fn request_items_properties_updated(
        &self,
        updated_props: Vec<(i32, HashMap<String, OwnedValue>)>,
        removed_props: Vec<(i32, Vec<String>)>,
    ) {
        let object_paths = self.object_paths();
        self.request_items_properties_updated_for_paths(updated_props, removed_props, object_paths);
    }

    fn request_items_properties_updated_for_paths(
        &self,
        updated_props: Vec<(i32, HashMap<String, OwnedValue>)>,
        removed_props: Vec<(i32, Vec<String>)>,
        object_paths: Vec<String>,
    ) {
        if !self.is_connected() {
            return;
        }

        let sender = match self.command_sender.lock() {
            Ok(sender) => sender.clone(),
            Err(error) => {
                log::error!("Failed to read DBusMenu command sender: {error}");
                return;
            }
        };
        let Some(sender) = sender else {
            return;
        };

        if let Err(error) = sender.try_send(DBusMenuCommand::ItemsPropertiesUpdated {
            updated_props,
            removed_props,
            object_paths,
        }) {
            log::error!("Failed to queue DBusMenu ItemsPropertiesUpdated signal: {error}");
        }
    }

    fn object_paths(&self) -> Vec<String> {
        match self.object_paths.lock() {
            Ok(paths) => paths.iter().cloned().collect(),
            Err(error) => {
                log::error!("Failed to read DBusMenu object paths: {error}");
                vec![DBUSMENU_OBJECT_PATH.to_string()]
            }
        }
    }
}

#[zbus::interface(name = "com.canonical.dbusmenu")]
impl DBusMenuServer {
    async fn about_to_show(&self, _id: i32) -> zbus::fdo::Result<bool> {
        let callback = match self.will_open_callback.lock() {
            Ok(callback) => callback.clone(),
            Err(_) => {
                return Err(zbus::fdo::Error::Failed(
                    "Failed to access will-open callback".to_string(),
                ));
            }
        };
        if let Some(callback) = callback {
            callback();
        }
        Ok(false)
    }

    async fn about_to_show_group(&self, _ids: Vec<i32>) -> zbus::fdo::Result<(Vec<i32>, Vec<i32>)> {
        let callback = match self.will_open_callback.lock() {
            Ok(callback) => callback.clone(),
            Err(_) => {
                return Err(zbus::fdo::Error::Failed(
                    "Failed to access will-open callback".to_string(),
                ));
            }
        };
        if let Some(callback) = callback {
            callback();
        }
        Ok((Vec::new(), Vec::new()))
    }

    async fn event(
        &self,
        id: i32,
        event_id: &str,
        _data: Value<'_>,
        _timestamp: u32,
    ) -> zbus::fdo::Result<()> {
        if event_id != "clicked" {
            return Ok(());
        }

        let action = {
            let state = self.state.lock().map_err(|_| {
                zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string())
            })?;
            state
                .items
                .get(&id)
                .and_then(|entry| entry.action.as_ref().map(|a| a.boxed_clone()))
        };

        if let Some(action) = action {
            let callback = self.action_callback.lock().map_err(|_| {
                zbus::fdo::Error::Failed("Failed to access DBusMenu action callback".to_string())
            })?;
            if let Some(callback) = callback.as_ref() {
                callback(action);
            }
        }

        Ok(())
    }

    async fn get_group_properties(
        &self,
        ids: Vec<i32>,
        _property_names: Vec<String>,
    ) -> zbus::fdo::Result<Vec<(i32, HashMap<String, OwnedValue>)>> {
        let entries = {
            let state = self.state.lock().map_err(|_| {
                zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string())
            })?;
            ids.into_iter()
                .filter_map(|id| {
                    state.items.get(&id).map(|entry| {
                        (
                            id,
                            entry.properties.clone(),
                            entry.action.as_ref().map(|action| action.boxed_clone()),
                        )
                    })
                })
                .collect::<Vec<_>>()
        };

        let mut updated: Vec<(i32, bool, OwnedValue)> = Vec::new();
        let mut result = Vec::with_capacity(entries.len());

        for (id, mut properties, action) in entries {
            if let Some(action) = action {
                if let Some(enabled) = self.validate_enabled(action.as_ref()) {
                    if let Some(value) = owned_bool(enabled) {
                        properties.insert("enabled".to_string(), value.clone());
                        updated.push((id, enabled, value));
                    }
                }
            }
            result.push((id, properties));
        }

        if !updated.is_empty() {
            if let Ok(mut state) = self.state.lock() {
                for (id, enabled, value) in updated {
                    if let Some(entry) = state.items.get_mut(&id) {
                        entry.enabled = Some(enabled);
                        entry.properties.insert("enabled".to_string(), value);
                    }
                }
            }
        }

        Ok(result)
    }

    async fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        _property_names: Vec<String>,
    ) -> zbus::fdo::Result<(u32, (i32, HashMap<String, OwnedValue>, Vec<OwnedValue>))> {
        let revision = self
            .state
            .lock()
            .map_err(|_| {
                zbus::fdo::Error::Failed("Failed to access DBusMenu revision".to_string())
            })?
            .revision;
        let depth = if recursion_depth < 0 {
            i32::MAX
        } else {
            recursion_depth
        };

        let layout = self.get_layout_node(parent_id, depth).ok_or_else(|| {
            zbus::fdo::Error::InvalidArgs(format!("Unknown menu item id: {parent_id}"))
        })?;

        Ok((revision, layout))
    }

    async fn get_property(&self, id: i32, name: &str) -> zbus::fdo::Result<OwnedValue> {
        if name == "enabled" {
            let action = {
                let state = self.state.lock().map_err(|_| {
                    zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string())
                })?;
                state
                    .items
                    .get(&id)
                    .and_then(|entry| entry.action.as_ref().map(|action| action.boxed_clone()))
            };

            if let Some(action) = action {
                if let Some(enabled) = self.validate_enabled(action.as_ref())
                    && let Some(value) = owned_bool(enabled)
                {
                    if let Ok(mut state) = self.state.lock() {
                        if let Some(entry) = state.items.get_mut(&id) {
                            entry.enabled = Some(enabled);
                            entry.properties.insert("enabled".to_string(), value.clone());
                        }
                    }
                    return Ok(value);
                }
            }
        }

        let state = self.state.lock().map_err(|_| {
            zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string())
        })?;
        state
            .items
            .get(&id)
            .and_then(|entry| entry.properties.get(name).cloned())
            .ok_or_else(|| zbus::fdo::Error::UnknownProperty(name.to_string()))
    }

    #[zbus(property)]
    fn status(&self) -> &str {
        "normal"
    }

    #[zbus(property)]
    fn version(&self) -> u32 {
        3
    }

    #[zbus(property)]
    fn text_direction(&self) -> &str {
        if is_rtl_locale() { "rtl" } else { "ltr" }
    }

    #[zbus(property)]
    fn icon_theme_path(&self) -> Vec<String> {
        Vec::new()
    }

    #[zbus(signal)]
    pub async fn layout_updated(
        ctxt: &zbus::object_server::SignalEmitter<'_>,
        revision: u32,
        parent: i32,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn items_properties_updated(
        ctxt: &zbus::object_server::SignalEmitter<'_>,
        updated_props: Vec<(i32, HashMap<String, OwnedValue>)>,
        removed_props: Vec<(i32, Vec<String>)>,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn item_activation_requested(
        ctxt: &zbus::object_server::SignalEmitter<'_>,
        id: i32,
        timestamp: u32,
    ) -> zbus::Result<()>;
}

fn root_properties() -> HashMap<String, OwnedValue> {
    let mut props = HashMap::new();
    let value = Value::Str("submenu".into());
    if let Ok(value) = value.try_into() {
        props.insert("children-display".to_string(), value);
    } else {
        log::error!("Failed to build DBusMenu root properties");
    }
    props
}

fn build_menu_tree(
    items: &mut HashMap<i32, MenuItemEntry>,
    next_id: &mut i32,
    this_id: i32,
    menu: &OwnedMenu,
    keymap: &Keymap,
) {
    let mut properties = HashMap::new();
    let label_value = Value::Str(menu.name.to_string().into());
    if let Ok(value) = label_value.try_into() {
        properties.insert("label".to_string(), value);
    } else {
        log::error!("Failed to encode DBusMenu label for menu {}", menu.name);
    }
    let children_value = Value::Str("submenu".into());
    if let Ok(value) = children_value.try_into() {
        properties.insert("children-display".to_string(), value);
    } else {
        log::error!("Failed to encode DBusMenu children-display property");
    }

    let mut child_ids = Vec::new();

    for item in &menu.items {
        let child_id = *next_id;
        *next_id = next_id.wrapping_add(1);

        match item {
            OwnedMenuItem::Separator => {
                let mut props = HashMap::new();
                let value = Value::Str("separator".into());
                if let Ok(value) = value.try_into() {
                    props.insert("type".to_string(), value);
                } else {
                    log::error!("Failed to encode DBusMenu separator type");
                }
                items.insert(
                    child_id,
                    MenuItemEntry {
                        action: None,
                        enabled: None,
                        properties: props,
                        children: Vec::new(),
                    },
                );
                child_ids.push(child_id);
            }
            OwnedMenuItem::Action {
                name,
                action,
                checked,
                ..
            } => {
                let mut props = HashMap::new();
                let label_value = Value::Str(name.clone().into());
                if let Ok(value) = label_value.try_into() {
                    props.insert("label".to_string(), value);
                } else {
                    log::error!("Failed to encode DBusMenu label for menu item {}", name);
                }

                if let Some(value) = owned_bool(true) {
                    props.insert("enabled".to_string(), value);
                } else {
                    log::error!("Failed to encode DBusMenu enabled state for menu item {}", name);
                }

                if let Some(shortcut) = dbus_shortcut_for_action(action.as_ref(), keymap) {
                    props.insert("shortcut".to_string(), shortcut);
                }

                if *checked {
                    let toggle_type = Value::Str("checkmark".into());
                    if let Ok(value) = toggle_type.try_into() {
                        props.insert("toggle-type".to_string(), value);
                    } else {
                        log::error!("Failed to encode DBusMenu toggle-type");
                    }
                    let toggle_state = Value::I32(1);
                    if let Ok(value) = toggle_state.try_into() {
                        props.insert("toggle-state".to_string(), value);
                    } else {
                        log::error!("Failed to encode DBusMenu toggle-state");
                    }
                }
                items.insert(
                    child_id,
                    MenuItemEntry {
                        action: Some(action.boxed_clone()),
                        enabled: Some(true),
                        properties: props,
                        children: Vec::new(),
                    },
                );
                child_ids.push(child_id);
            }
            OwnedMenuItem::Submenu(submenu) => {
                build_menu_tree(items, next_id, child_id, submenu, keymap);
                child_ids.push(child_id);
            }
            OwnedMenuItem::SystemMenu(_) => {}
        }
    }

    items.insert(
        this_id,
        MenuItemEntry {
            action: None,
            enabled: None,
            properties,
            children: child_ids,
        },
    );
}

pub fn object_path_for_window(window_id: u32) -> String {
    format!("{DBUSMENU_OBJECT_PATH}/window_{window_id}")
}

fn owned_bool(value: bool) -> Option<OwnedValue> {
    Value::Bool(value).try_into().ok()
}

fn dbus_shortcut_for_action(action: &dyn Action, keymap: &Keymap) -> Option<OwnedValue> {
    static DEFAULT_CONTEXT: OnceLock<Vec<KeyContext>> = OnceLock::new();

    let contexts = DEFAULT_CONTEXT.get_or_init(|| {
        let mut workspace_context = KeyContext::new_with_defaults();
        workspace_context.add("Workspace");
        let mut pane_context = KeyContext::new_with_defaults();
        pane_context.add("Pane");
        let mut editor_context = KeyContext::new_with_defaults();
        editor_context.add("Editor");

        pane_context.extend(&editor_context);
        workspace_context.extend(&pane_context);
        vec![workspace_context]
    });

    let binding = keymap
        .bindings_for_action(action)
        .find(|binding| binding.predicate().is_none_or(|predicate| predicate.eval(contexts)))
        .or_else(|| keymap.bindings_for_action(action).next());

    let keystrokes = binding?.keystrokes();
    if keystrokes.len() != 1 {
        return None;
    }

    dbus_shortcut_for_keystroke(&keystrokes[0])
}

fn dbus_shortcut_for_keystroke(keystroke: &KeybindingKeystroke) -> Option<OwnedValue> {
    let mut keys: Vec<String> = Vec::new();

    let modifiers = keystroke.modifiers();
    if modifiers.control {
        keys.push("Control".to_string());
    }
    if modifiers.alt {
        keys.push("Alt".to_string());
    }
    if modifiers.shift {
        keys.push("Shift".to_string());
    }
    if modifiers.platform {
        keys.push("Super".to_string());
    }
    if modifiers.function {
        keys.push("Fn".to_string());
    }

    keys.push(keystroke.key().to_string());

    let shortcut: Vec<Vec<String>> = vec![keys];
    Value::from(shortcut).try_into().ok()
}

fn is_rtl_locale() -> bool {
    let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|key| env::var(key).ok())
        .unwrap_or_default();

    let language = locale
        .split('.')
        .next()
        .and_then(|value| value.split('@').next())
        .and_then(|value| value.split('_').next())
        .and_then(|value| value.split('-').next())
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();

    matches!(
        language.as_str(),
        "ar" | "he" | "iw" | "fa" | "ur" | "ps" | "dv" | "ku" | "sd" | "ug" | "yi"
    )
}
