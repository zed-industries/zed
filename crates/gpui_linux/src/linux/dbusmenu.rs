use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::OnceLock;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use calloop::{LoopHandle, channel::Sender as CalloopSender};
use gpui::{Action, KeyContext, KeybindingKeystroke, Keymap, OsAction, OwnedMenu, OwnedMenuItem};
use util::ResultExt as _;
use zbus::zvariant::{OwnedValue, Value};

use std::rc::Weak;
use std::cell::RefCell;

pub trait GlobalMenuState {
    fn linux_common(&mut self) -> &mut crate::linux::LinuxCommon;
}

pub fn setup_global_menu_sources<D: 'static, T: GlobalMenuState + 'static>(
    dbus_menu_server: &DBusMenuServer,
    loop_handle: &LoopHandle<D>,
    client: Weak<RefCell<T>>,
    mut on_connected: impl FnMut(&mut T) + 'static,
) {
    dbus_menu_server.install_global_menu_sources(
        loop_handle,
        {
            let client = client.clone();
            move |action| {
                if let Some(client) = client.upgrade() {
                    let mut state = client.borrow_mut();
                    if let Some(callback) = state.linux_common().callbacks.app_menu_action.as_mut() {
                        callback(action.as_ref());
                    }
                }
            }
        },
        {
            let client = client.clone();
            move |request| {
                if let Some(client) = client.upgrade() {
                    let (dbus_menu_server, mut validate_app_menu_command, mut will_open_app_menu) = {
                        let mut state = client.borrow_mut();
                        let common = state.linux_common();
                        (
                            common.dbus_menu_server.clone(),
                            common.callbacks.validate_app_menu_command.take(),
                            common.callbacks.will_open_app_menu.take(),
                        )
                    };
                    
                    let request_ids = request.ids;
                    if let Some(callback) = will_open_app_menu.as_mut() {
                        callback();
                    }

                    let (valid_ids, id_errors) = dbus_menu_server
                        .as_ref()
                        .map(|dbus_menu_server| {
                            dbus_menu_server.classify_ids(&request_ids)
                        })
                        .unwrap_or_else(|| (Vec::new(), request_ids.clone()));

                    let refreshed_ids = match (
                        dbus_menu_server.as_ref(),
                        validate_app_menu_command.as_mut(),
                    ) {
                        (Some(dbus_menu_server), Some(validate_callback)) => {
                            dbus_menu_server.refresh_enabled_states_inner(validate_callback)
                        }
                        _ => Vec::new(),
                    };

                    let updated_ids = if refreshed_ids.is_empty() {
                        Vec::new()
                    } else {
                        valid_ids
                    };

                    let mut state = client.borrow_mut();
                    let common = state.linux_common();
                    if common.callbacks.validate_app_menu_command.is_none() {
                        common.callbacks.validate_app_menu_command = validate_app_menu_command;
                    }
                    if common.callbacks.will_open_app_menu.is_none() {
                        common.callbacks.will_open_app_menu = will_open_app_menu;
                    }

                    let _ = request.responded.send(
                        AboutToShowResponse {
                            updated_ids,
                            id_errors,
                        },
                    );
                }
            }
        },
        {
            let client = client.clone();
            move || {
                if let Some(client) = client.upgrade() {
                    let mut state = client.borrow_mut();
                    on_connected(&mut state);
                }
            }
        },
    );
}

pub const DBUSMENU_OBJECT_PATH: &str = "/MenuBar";

type ActionCallback = dyn Fn(Box<dyn Action>) + Send + Sync;
type ConnectedCallback = dyn Fn() + Send + Sync;

#[derive(Clone)]
pub enum DBusMenuCommand {
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
    Shutdown,
}

pub struct AboutToShowRequest {
    pub ids: Vec<i32>,
    pub responded: std::sync::mpsc::Sender<AboutToShowResponse>,
}

pub struct AboutToShowResponse {
    pub updated_ids: Vec<i32>,
    pub id_errors: Vec<i32>,
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
    connected_callback: Arc<Mutex<Option<Box<ConnectedCallback>>>>,
    command_sender: Arc<Mutex<Option<async_channel::Sender<DBusMenuCommand>>>>,
    about_to_show_sender: Arc<Mutex<Option<CalloopSender<AboutToShowRequest>>>>,
    connected: Arc<AtomicBool>,
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

        Self {
            state: Arc::new(Mutex::new(MenuState { items, revision: 1 })),
            action_callback: Arc::new(Mutex::new(None)),
            connected_callback: Arc::new(Mutex::new(None)),
            command_sender: Arc::new(Mutex::new(None)),
            about_to_show_sender: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
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

    pub fn spawn_dbus_menu_thread(
        &self,
        service_name: String,
        object_path: String,
        unique_name_sender: Option<CalloopSender<String>>,
    ) -> Option<std::thread::JoinHandle<()>> {
        let (dbus_command_tx, dbus_command_rx) = async_channel::unbounded();
        self.set_command_sender(dbus_command_tx);

        let dbus_menu_server = self.clone();
        std::thread::Builder::new()
            .name("dbus-menu".into())
            .spawn(move || {
                smol::block_on(async move {
                    let builder = match zbus::connection::Builder::session()
                        .and_then(|builder| builder.name(service_name.as_str()))
                        .and_then(|builder| {
                            builder.serve_at(object_path.as_str(), dbus_menu_server.clone())
                        }) {
                        Ok(builder) => builder,
                        Err(error) => {
                            log::error!("Failed to configure DBus connection: {error}");
                            return;
                        }
                    };

                    let connection = match builder.build().await {
                        Ok(connection) => connection,
                        Err(error) => {
                            log::error!("Failed to build DBus connection: {error}");
                            return;
                        }
                    };

                    dbus_menu_server.mark_connected();
                    log::info!("DBusMenu server started on {service_name}");

                    if let Some(unique_name_sender) = unique_name_sender {
                        if let Some(unique_name) = connection.unique_name() {
                            if let Err(error) = unique_name_sender.send(unique_name.to_string()) {
                                log::error!("Failed to send DBusMenu unique name: {error}");
                            }
                        }
                    }

                    while let Ok(command) = dbus_command_rx.recv().await {
                        match command {
                            DBusMenuCommand::LayoutUpdated {
                                revision,
                                parent,
                                object_paths,
                            } => {
                                for object_path in object_paths {
                                    let emitter =
                                        match zbus::object_server::SignalEmitter::new(
                                            &connection,
                                            object_path.as_str(),
                                        ) {
                                            Ok(emitter) => emitter,
                                            Err(error) => {
                                                log::error!(
                                                    "Failed to build DBusMenu signal emitter for {object_path}: {error}"
                                                );
                                                continue;
                                            }
                                        };
                                    if let Err(error) = DBusMenuServer::layout_updated(
                                        &emitter,
                                        revision,
                                        parent,
                                    )
                                    .await
                                    {
                                        log::error!(
                                            "Failed to emit DBusMenu LayoutUpdated signal for {object_path}: {error}"
                                        );
                                    }
                                }
                            }
                            DBusMenuCommand::ItemsPropertiesUpdated {
                                updated_props,
                                removed_props,
                                object_paths,
                            } => {
                                for object_path in object_paths {
                                    let emitter =
                                        match zbus::object_server::SignalEmitter::new(
                                            &connection,
                                            object_path.as_str(),
                                        ) {
                                            Ok(emitter) => emitter,
                                            Err(error) => {
                                                log::error!(
                                                    "Failed to build DBusMenu signal emitter for {object_path}: {error}"
                                                );
                                                continue;
                                            }
                                        };
                                    if let Err(error) = DBusMenuServer::items_properties_updated(
                                        &emitter,
                                        updated_props.clone(),
                                        removed_props.clone(),
                                    )
                                    .await
                                    {
                                        log::error!(
                                            "Failed to emit DBusMenu ItemsPropertiesUpdated for {object_path}: {error}"
                                        );
                                    }
                                }
                            }
                            DBusMenuCommand::Shutdown => {
                                break;
                            }
                        }
                    }
                });
            })
            .log_err()
    }

    pub fn install_global_menu_sources<D: 'static>(
        &self,
        loop_handle: &LoopHandle<D>,
        mut action_handler: impl FnMut(Box<dyn Action>) + 'static,
        mut about_to_show_handler: impl FnMut(AboutToShowRequest) + 'static,
        mut connected_handler: impl FnMut() + 'static,
    ) {
        let (action_tx, action_rx) = calloop::channel::channel::<Box<dyn Action>>();
        let (about_to_show_tx, about_to_show_rx) =
            calloop::channel::channel::<AboutToShowRequest>();
        let (connected_tx, connected_rx) = calloop::channel::channel::<()>();

        self.set_action_callback(Box::new(move |action| {
            if let Err(error) = action_tx.send(action) {
                log::error!("Failed to send DBus menu action: {error}");
            }
        }));
        self.set_about_to_show_sender(about_to_show_tx);
        self.set_connected_callback(Box::new(move || {
            if let Err(error) = connected_tx.send(()) {
                log::error!("Failed to send DBusMenu connected event: {error}");
            }
        }));

        loop_handle
            .insert_source(action_rx, move |event, _, _| {
                if let calloop::channel::Event::Msg(action) = event {
                    action_handler(action);
                }
            })
            .log_err();

        loop_handle
            .insert_source(about_to_show_rx, move |event, _, _| {
                if let calloop::channel::Event::Msg(request) = event {
                    about_to_show_handler(request);
                }
            })
            .log_err();

        loop_handle
            .insert_source(connected_rx, move |event, _, _| {
                if let calloop::channel::Event::Msg(()) = event {
                    connected_handler();
                }
            })
            .log_err();
    }

    pub fn set_about_to_show_sender(&self, sender: CalloopSender<AboutToShowRequest>) {
        match self.about_to_show_sender.lock() {
            Ok(mut slot) => *slot = Some(sender),
            Err(error) => {
                log::error!("Failed to store DBusMenu about-to-show sender: {error}");
            }
        }
    }

    pub fn mark_connected(&self) {
        self.connected.store(true, Ordering::SeqCst);

        if let Ok(callback) = self.connected_callback.lock() {
            if let Some(callback) = callback.as_ref() {
                callback();
            }
        }

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

    pub fn set_action_callback(&self, callback: Box<ActionCallback>) {
        if let Ok(mut slot) = self.action_callback.lock() {
            *slot = Some(callback);
        } else {
            log::error!("Failed to store DBusMenu action callback due to lock poisoning");
        }
    }

    pub fn set_connected_callback(&self, callback: Box<ConnectedCallback>) {
        if let Ok(mut slot) = self.connected_callback.lock() {
            *slot = Some(callback);
        } else {
            log::error!("Failed to store DBusMenu connected callback due to lock poisoning");
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

    pub fn shutdown(&self) {
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

        if let Err(error) = sender.try_send(DBusMenuCommand::Shutdown) {
            log::error!("Failed to queue DBusMenu shutdown request: {error}");
        }
    }

    pub fn refresh_enabled_states(&self, validate: &mut dyn FnMut(&dyn Action) -> bool) -> bool {
        let updated_ids = self.refresh_enabled_states_inner(validate);
        !updated_ids.is_empty()
    }

    pub fn refresh_enabled_states_inner(
        &self,
        validate: &mut dyn FnMut(&dyn Action) -> bool,
    ) -> Vec<i32> {
        let items_to_check: Vec<(i32, Box<dyn Action>, bool)> = {
            let state = match self.state.lock() {
                Ok(state) => state,
                Err(error) => {
                    log::error!("Failed to access DBusMenu state for enable refresh: {error}");
                    return Vec::new();
                }
            };
            state
                .items
                .iter()
                .filter_map(|(id, entry)| {
                    let previous_enabled = entry.enabled?;
                    let action = entry.action.as_ref()?.boxed_clone();
                    Some((*id, action, previous_enabled))
                })
                .collect()
        };

        let mut updated_props: Vec<(i32, HashMap<String, OwnedValue>)> = Vec::new();
        let mut updates: Vec<(i32, bool, OwnedValue)> = Vec::new();

        for (id, action, previous_enabled) in items_to_check {
            let enabled = validate(action.as_ref());
            if enabled == previous_enabled {
                continue;
            }
            if let Some(value) = owned_bool(enabled) {
                let mut props = HashMap::new();
                props.insert("enabled".to_string(), value.clone());
                updated_props.push((id, props));
                updates.push((id, enabled, value));
            }
        }

        if updates.is_empty() {
            return Vec::new();
        }

        {
            let mut state = match self.state.lock() {
                Ok(state) => state,
                Err(error) => {
                    log::error!("Failed to update DBusMenu state after refresh: {error}");
                    return Vec::new();
                }
            };
            for (id, enabled, value) in &updates {
                if let Some(entry) = state.items.get_mut(id) {
                    entry.enabled = Some(*enabled);
                    entry.properties.insert("enabled".to_string(), value.clone());
                }
            }
        }

        self.request_items_properties_updated(updated_props, Vec::new());
        updates.iter().map(|(id, _, _)| *id).collect()
    }

    fn request_layout_updated(&self, revision: u32) {
        self.request_layout_updated_for_paths(revision, vec![DBUSMENU_OBJECT_PATH.to_string()]);
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
        self.request_items_properties_updated_for_paths(
            updated_props, 
            removed_props, 
            vec![DBUSMENU_OBJECT_PATH.to_string()]
        );
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

    pub fn classify_ids(&self, ids: &[i32]) -> (Vec<i32>, Vec<i32>) {
        let state = match self.state.lock() {
            Ok(state) => state,
            Err(error) => {
                log::error!("Failed to read DBusMenu state for id lookup: {error}");
                return (Vec::new(), ids.to_vec());
            }
        };

        let mut valid = Vec::new();
        let mut errors = Vec::new();

        for id in ids {
            if state.items.contains_key(id) {
                valid.push(*id);
            } else {
                errors.push(*id);
            }
        }

        (valid, errors)
    }
}

#[zbus::interface(name = "com.canonical.dbusmenu")]
impl DBusMenuServer {
    async fn about_to_show(&self, id: i32) -> zbus::fdo::Result<bool> {
        let sender = match self.about_to_show_sender.lock() {
            Ok(sender) => sender.clone(),
            Err(_) => return Ok(false),
        };
        let Some(sender) = sender else {
            return Ok(false);
        };

        let (responded_tx, responded_rx) = std::sync::mpsc::channel();
        if sender
            .send(AboutToShowRequest {
                ids: vec![id],
                responded: responded_tx,
            })
            .is_err()
        {
            return Ok(false);
        }

        match responded_rx.recv_timeout(Duration::from_millis(50)) {
            Ok(response) => Ok(!response.updated_ids.is_empty()),
            Err(_) => Ok(false),
        }
    }

    async fn about_to_show_group(
        &self,
        ids: Vec<i32>,
    ) -> zbus::fdo::Result<(Vec<i32>, Vec<i32>)> {
        let sender = match self.about_to_show_sender.lock() {
            Ok(sender) => sender.clone(),
            Err(_) => return Ok((Vec::new(), Vec::new())),
        };
        let Some(sender) = sender else {
            return Ok((Vec::new(), Vec::new()));
        };

        let (responded_tx, responded_rx) = std::sync::mpsc::channel();
        if sender
            .send(AboutToShowRequest {
                ids,
                responded: responded_tx,
            })
            .is_err()
        {
            return Ok((Vec::new(), Vec::new()));
        }

        let response = responded_rx
            .recv_timeout(Duration::from_millis(50))
            .unwrap_or(AboutToShowResponse {
                updated_ids: Vec::new(),
                id_errors: Vec::new(),
            });

        Ok((response.updated_ids, response.id_errors))
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
            let entry = state.items.get(&id);
            let enabled = entry.and_then(|e| e.enabled).unwrap_or(true);
            if !enabled {
                return Ok(());
            }
            entry.and_then(|entry| entry.action.as_ref().map(|a| a.boxed_clone()))
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
        property_names: Vec<String>,
    ) -> zbus::fdo::Result<Vec<(i32, HashMap<String, OwnedValue>)>> {
        let property_filter = build_property_filter(&property_names);
        let entries = {
            let state = self.state.lock().map_err(|_| {
                zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string())
            })?;
            ids.into_iter()
                .filter_map(|id| {
                    state.items.get(&id).map(|entry| {
                        (id, filter_properties(entry.properties.clone(), &property_filter))
                    })
                })
                .collect::<Vec<_>>()
        };

        Ok(entries)
    }

    async fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: Vec<String>,
    ) -> zbus::fdo::Result<(u32, (i32, HashMap<String, OwnedValue>, Vec<OwnedValue>))> {
        let property_filter = build_property_filter(&property_names);
        let state = self.state.lock().map_err(|_| {
            zbus::fdo::Error::Failed("Failed to access DBusMenu state".to_string())
        })?;
        let revision = state.revision;
        let depth = if recursion_depth < 0 {
            i32::MAX
        } else {
            recursion_depth
        };
        let layout = collect_layout_node_filtered(&state, parent_id, depth, &property_filter)
            .ok_or_else(|| {
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
    insert_visible_property(&mut props);
    props
}

fn build_property_filter(property_names: &[String]) -> Option<HashSet<String>> {
    if property_names.is_empty() {
        None
    } else {
        Some(property_names.iter().cloned().collect())
    }
}

fn filter_properties(
    properties: HashMap<String, OwnedValue>,
    filter: &Option<HashSet<String>>,
) -> HashMap<String, OwnedValue> {
    match filter {
        Some(filter) => properties
            .into_iter()
            .filter(|(name, _)| filter.contains(name))
            .collect(),
        None => properties,
    }
}

fn collect_layout_node_filtered(
    state: &MenuState,
    id: i32,
    remaining_depth: i32,
    filter: &Option<HashSet<String>>,
) -> Option<(i32, HashMap<String, OwnedValue>, Vec<OwnedValue>)> {
    let entry = state.items.get(&id)?;
    let properties = filter_properties(entry.properties.clone(), filter);

    let children = if remaining_depth == 0 {
        Vec::new()
    } else {
        let mut result = Vec::new();
        for &child_id in &entry.children {
            if let Some(child_node) =
                collect_layout_node_filtered(state, child_id, remaining_depth - 1, filter)
            {
                let variant = Value::from(zbus::zvariant::Structure::from(child_node)).try_into();
                if let Ok(value) = variant {
                    result.push(value);
                }
            }
        }
        result
    };

    Some((id, properties, children))
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
    insert_visible_property(&mut properties);

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
                insert_visible_property(&mut props);
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
                os_action,
                checkable,
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
                    log::error!(
                        "Failed to encode DBusMenu enabled state for menu item {}",
                        name
                    );
                }

                if let Some(shortcut) = dbus_shortcut_for_action(action.as_ref(), keymap) {
                    props.insert("shortcut".to_string(), shortcut);
                }

                if let Some(os_action) = os_action {
                    if let Some(icon_name) = icon_name_for_os_action(*os_action) {
                        match Value::Str(icon_name.into()).try_into() {
                            Ok(value) => {
                                props.insert("icon-name".to_string(), value);
                            }
                            Err(error) => {
                                log::error!(
                                    "Failed to encode DBusMenu icon-name for {icon_name}: {error}"
                                );
                            }
                        }
                    }
                }

                if *checkable {
                    let toggle_type = Value::Str("checkmark".into());
                    if let Ok(value) = toggle_type.try_into() {
                        props.insert("toggle-type".to_string(), value);
                    } else {
                        log::error!("Failed to encode DBusMenu toggle-type");
                    }
                    let toggle_state = if *checked { Value::I32(1) } else { Value::I32(0) };
                    if let Ok(value) = toggle_state.try_into() {
                        props.insert("toggle-state".to_string(), value);
                    } else {
                        log::error!("Failed to encode DBusMenu toggle-state");
                    }
                }
                insert_visible_property(&mut props);
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


pub fn global_menu_env_override() -> Option<bool> {
    match std::env::var("ZED_GLOBAL_MENU").ok().as_deref() {
        None => None,
        Some("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON") => Some(true),
        Some("0" | "false" | "FALSE" | "no" | "NO" | "off" | "OFF") => Some(false),
        Some(value) => {
            log::warn!(
                "Ignoring invalid ZED_GLOBAL_MENU value {value:?}. Expected 0/1, true/false, yes/no, or on/off."
            );
            None
        }
    }
}

fn owned_bool(value: bool) -> Option<OwnedValue> {
    Value::Bool(value).try_into().ok()
}

fn insert_visible_property(props: &mut HashMap<String, OwnedValue>) {
    if let Some(value) = owned_bool(true) {
        props.insert("visible".to_string(), value);
    } else {
        log::error!("Failed to encode DBusMenu visible property");
    }
}

fn icon_name_for_os_action(action: OsAction) -> Option<&'static str> {
    match action {
        OsAction::Cut => Some("edit-cut"),
        OsAction::Copy => Some("edit-copy"),
        OsAction::Paste => Some("edit-paste"),
        OsAction::SelectAll => Some("edit-select-all"),
        OsAction::Undo => Some("edit-undo"),
        OsAction::Redo => Some("edit-redo"),
    }
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
        .find(|binding| {
            binding
                .predicate()
                .is_none_or(|predicate| predicate.eval(contexts))
        })
        .or_else(|| keymap.bindings_for_action(action).next());

    let keystrokes = binding?.keystrokes();
    dbus_shortcut_for_keystrokes(keystrokes)
}

fn dbus_shortcut_for_keystrokes(keystrokes: &[KeybindingKeystroke]) -> Option<OwnedValue> {
    if keystrokes.is_empty() {
        return None;
    }
    let mut shortcut: Vec<Vec<String>> = Vec::with_capacity(keystrokes.len());
    for keystroke in keystrokes {
        shortcut.push(dbus_keys_for_keystroke(keystroke)?);
    }
    Value::from(shortcut).try_into().ok()
}

fn dbus_keys_for_keystroke(keystroke: &KeybindingKeystroke) -> Option<Vec<String>> {
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

    keys.push(normalize_dbus_key(keystroke.key())?);

    Some(keys)
}

fn normalize_dbus_key(key: &str) -> Option<String> {
    let normalized = match key {
        "enter" | "return" => "Return".to_string(),
        "escape" => "Escape".to_string(),
        "tab" => "Tab".to_string(),
        "backspace" => "BackSpace".to_string(),
        "delete" => "Delete".to_string(),
        "insert" => "Insert".to_string(),
        "home" => "Home".to_string(),
        "end" => "End".to_string(),
        "pageup" => "PageUp".to_string(),
        "pagedown" => "PageDown".to_string(),
        "left" => "Left".to_string(),
        "right" => "Right".to_string(),
        "up" => "Up".to_string(),
        "down" => "Down".to_string(),
        "space" => "space".to_string(),
        _ => {
            if let Some(suffix) = key.strip_prefix('f') {
                if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
                    format!("F{suffix}")
                } else {
                    key.to_string()
                }
            } else if key.len() == 1 {
                let mut chars = key.chars();
                let ch = chars.next()?;
                if chars.next().is_some() {
                    key.to_string()
                } else if ch.is_ascii_alphabetic() {
                    ch.to_ascii_uppercase().to_string()
                } else {
                    key.to_string()
                }
            } else {
                key.to_string()
            }
        }
    };

    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
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
