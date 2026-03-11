use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use gpui::{Action, OwnedMenu, OwnedMenuItem};
use zbus::Connection;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{OwnedValue, Value};

pub const DBUSMENU_OBJECT_PATH: &str = "/MenuBar";

type ActionCallback = dyn Fn(Box<dyn Action>) + Send + Sync;
type WillOpenCallback = dyn Fn() + Send + Sync;

struct MenuItemEntry {
    action: Option<Box<dyn Action>>,
    properties: HashMap<String, OwnedValue>,
    children: Vec<i32>,
}

struct MenuState {
    items: HashMap<i32, MenuItemEntry>,
    revision: u32,
}

/// The DBusMenu server that exposes the application's menus over DBus.
///
/// Implements the `com.canonical.dbusmenu` interface so that desktop environments
/// (KDE Plasma, etc.) can render the app's menus in their global menu bar.
#[derive(Clone)]
pub struct DBusMenuServer {
    state: Arc<Mutex<MenuState>>,
    action_callback: Arc<Mutex<Option<Box<ActionCallback>>>>,
    will_open_callback: Arc<Mutex<Option<Arc<WillOpenCallback>>>>,
    connection: Arc<Mutex<Option<Connection>>>,
    runtime_handle: Arc<Mutex<Option<tokio::runtime::Handle>>>,
}

impl DBusMenuServer {
    pub fn new() -> Self {
        let mut items = HashMap::new();
        items.insert(
            0,
            MenuItemEntry {
                action: None,
                properties: root_properties(),
                children: Vec::new(),
            },
        );
        Self {
            state: Arc::new(Mutex::new(MenuState { items, revision: 1 })),
            action_callback: Arc::new(Mutex::new(None)),
            will_open_callback: Arc::new(Mutex::new(None)),
            connection: Arc::new(Mutex::new(None)),
            runtime_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_connection(&self, connection: Connection) {
        match self.connection.lock() {
            Ok(mut slot) => {
                *slot = Some(connection);
            }
            Err(error) => {
                log::error!("Failed to store DBus connection for DBusMenu: {error}");
                return;
            }
        }

        let revision = match self.state.lock() {
            Ok(state) => state.revision,
            Err(error) => {
                log::error!("Failed to read DBusMenu revision for layout update: {error}");
                return;
            }
        };
        self.emit_layout_updated(revision);
    }

    pub fn set_runtime_handle(&self, runtime_handle: tokio::runtime::Handle) {
        if let Ok(mut slot) = self.runtime_handle.lock() {
            *slot = Some(runtime_handle);
        } else {
            log::error!("Failed to store DBusMenu runtime handle due to lock poisoning");
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

    pub fn set_menus(&self, menus: Vec<OwnedMenu>) {
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
            build_menu_tree(&mut state.items, &mut next_id, submenu_id, menu);
            root_children.push(submenu_id);
        }

        state.items.insert(
            0,
            MenuItemEntry {
                action: None,
                properties: root_properties(),
                children: root_children,
            },
        );

        state.revision = state.revision.wrapping_add(1);
        let revision = state.revision;
        drop(state);
        self.emit_layout_updated(revision);
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
                    if let Ok(v) = variant {
                        result.push(v);
                    }
                }
            }
            result
        };

        Some((id, properties, children))
    }

    fn emit_layout_updated(&self, revision: u32) {
        let runtime_handle = match self.runtime_handle.lock() {
            Ok(handle) => handle.clone(),
            Err(error) => {
                log::error!("Failed to read DBusMenu runtime handle: {error}");
                return;
            }
        };
        let connection = match self.connection.lock() {
            Ok(connection) => connection.clone(),
            Err(error) => {
                log::error!("Failed to read DBusMenu connection: {error}");
                return;
            }
        };

        let (Some(runtime_handle), Some(connection)) = (runtime_handle, connection) else {
            return;
        };

        runtime_handle.spawn(async move {
            let emitter = match SignalEmitter::new(&connection, DBUSMENU_OBJECT_PATH) {
                Ok(emitter) => emitter,
                Err(error) => {
                    log::error!("Failed to build DBusMenu signal emitter: {error}");
                    return;
                }
            };
            if let Err(error) = DBusMenuServer::layout_updated(&emitter, revision, 0).await {
                log::error!("Failed to emit DBusMenu LayoutUpdated signal: {error}");
            }
        });
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
        let state = self
            .state
            .lock()
            .map_err(|_| zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string()))?;
        let result = ids
            .into_iter()
            .filter_map(|id| {
                state
                    .items
                    .get(&id)
                    .map(|entry| (id, entry.properties.clone()))
            })
            .collect();
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
        let state = self
            .state
            .lock()
            .map_err(|_| zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string()))?;
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
    async fn layout_updated(
        ctxt: &SignalEmitter<'_>,
        revision: u32,
        parent: i32,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn items_properties_updated(
        ctxt: &SignalEmitter<'_>,
        updated_props: Vec<(i32, HashMap<String, OwnedValue>)>,
        removed_props: Vec<(i32, Vec<String>)>,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn item_activation_requested(
        ctxt: &SignalEmitter<'_>,
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
                        properties: props,
                        children: Vec::new(),
                    },
                );
                child_ids.push(child_id);
            }
            OwnedMenuItem::Submenu(submenu) => {
                build_menu_tree(items, next_id, child_id, submenu);
                child_ids.push(child_id);
            }
            OwnedMenuItem::SystemMenu(_) => {
                // System menus (e.g., macOS Services) are not meaningful on Linux
            }
        }
    }

    items.insert(
        this_id,
        MenuItemEntry {
            action: None,
            properties,
            children: child_ids,
        },
    );
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
