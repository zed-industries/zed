use std::fmt::{Debug, Display};

use serde::Deserialize;
use zbus::{
    export::futures_util::{Stream, StreamExt},
    interface,
    object_server::{InterfaceRef, SignalContext},
    zvariant::{OwnedObjectPath, Structure, StructureBuilder, Type},
};

use super::dbusmenu::{
    DBusMenu, DBusMenuInterface, DBusMenuItem, DBusMenuRemovedProperties,
    DBusMenuUpdatedProperties, Icon, MenuProperty, Pixmap, DBUS_MENU_PATH,
};

pub(crate) struct StatusNotifierWatcher<'a>(zbus::Proxy<'a>);

const STATUS_NOTIFIER_WATCHER_INTERFACE: &str = "org.kde.StatusNotifierWatcher";
const STATUS_NOTIFIER_WATCHER_PATH: &str = "/StatusNotifierWatcher";
const STATUS_NOTIFIER_WATCHER_DESTINATION: &str = "org.kde.StatusNotifierWatcher";

const STATUS_NOTIFIER_ITEM_PATH: &str = "/StatusNotifierItem";

impl<'a> StatusNotifierWatcher<'a> {
    async fn new() -> zbus::Result<Self> {
        let conn = zbus::Connection::session().await?;
        let proxy: zbus::Proxy = zbus::ProxyBuilder::new(&conn)
            .interface(STATUS_NOTIFIER_WATCHER_INTERFACE)?
            .path(STATUS_NOTIFIER_WATCHER_PATH)?
            .destination(STATUS_NOTIFIER_WATCHER_DESTINATION)?
            .build()
            .await?;
        Ok(Self(proxy))
    }

    async fn register_status_notifier_item(&self, service: impl Into<String>) -> zbus::Result<()> {
        self.0
            .connection()
            .call_method(
                Some(STATUS_NOTIFIER_WATCHER_DESTINATION),
                STATUS_NOTIFIER_WATCHER_PATH,
                Some(STATUS_NOTIFIER_WATCHER_INTERFACE),
                "RegisterStatusNotifierItem",
                &(service.into()),
            )
            .await?;
        Ok(())
    }

    async fn register_status_notifier_host(&self, service: impl Into<String>) -> zbus::Result<()> {
        self.0
            .connection()
            .call_method(
                Some(STATUS_NOTIFIER_WATCHER_DESTINATION),
                STATUS_NOTIFIER_WATCHER_PATH,
                Some(STATUS_NOTIFIER_WATCHER_INTERFACE),
                "RegisterStatusNotifierHost",
                &(service.into()),
            )
            .await?;
        Ok(())
    }

    async fn registered_status_notifier_items(&self) -> zbus::Result<Vec<String>> {
        Ok(self.0.get_property("RegisteredStatusNotifierItems").await?)
    }

    async fn is_status_notifier_host_registered(&self) -> zbus::Result<bool> {
        Ok(self
            .0
            .get_property("IsStatusNotifierHostRegistered")
            .await?)
    }

    async fn receive_status_notifier_item_registered(
        &self,
    ) -> zbus::Result<impl Stream<Item = bool>> {
        self.receive_signal("StatusNotifierItemRegistered").await
    }

    async fn receive_status_notifier_item_unregistered(
        &self,
    ) -> zbus::Result<impl Stream<Item = bool>> {
        self.receive_signal("StatusNotifierItemUnregistered").await
    }

    async fn receive_status_notifier_host_registered(
        &self,
    ) -> zbus::Result<impl Stream<Item = bool>> {
        self.receive_signal("StatusNotifierHostRegistered").await
    }

    async fn receive_status_notifier_host_unregistered(
        &self,
    ) -> zbus::Result<impl Stream<Item = bool>> {
        self.receive_signal("StatusNotifierHostUnregistered").await
    }

    async fn receive_signal<R>(&self, name: &'static str) -> zbus::Result<impl Stream<Item = R>>
    where
        R: for<'de> Deserialize<'de> + Type + Debug,
    {
        let stream = self.0.receive_signal(name).await?;
        Ok(stream.filter_map(move |msg| core::future::ready(msg.body().deserialize().ok())))
    }
}

impl<'a> std::ops::Deref for StatusNotifierWatcher<'a> {
    type Target = zbus::Proxy<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Debug, Clone, Type)]
pub enum Category {
    /// The item describes the status of a generic application,
    /// for instance the current state of a media player.
    /// In the case where the category of the item can not be known,
    /// such as when the item is being proxied from another incompatible or emulated system,
    /// ApplicationStatus can be used a sensible default fallback.
    #[default]
    ApplicationStatus,
    /// The item describes the status of communication oriented applications,
    /// like an instant messenger or an email client.
    Communications,
    /// The item describes services of the system not seen as a stand alone application by the user,
    /// such as an indicator for the activity of a disk indexing service.
    SystemServices,
    /// The item describes the state and control of a particular hardware,
    /// such as an indicator of the battery charge or sound card volume control.
    Hardware,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Type, Default, Clone, Debug)]
pub enum Status {
    /// The item is active, is more important that the item will be shown in some way to the user.
    #[default]
    Active,
    /// The item doesn't convey important information to the user,
    /// it can be considered an "idle" status and is likely that visualizations will chose to hide it.
    Passive,
    /// The item carries really important information for the user,
    /// such as battery charge running out and is wants to incentive the direct user intervention.
    /// Visualizations should emphasize in some way the items with NeedsAttention status.
    NeedsAttention,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Type)]
pub struct ToolTip {
    icon: Icon,
    title: String,
    description: String,
}

impl ToolTip {
    pub fn new() -> Self {
        Self {
            icon: Icon::Name(String::default()),
            title: String::default(),
            description: String::default(),
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

impl From<ToolTip> for Structure<'_> {
    fn from(value: ToolTip) -> Self {
        let (name, pixmaps) = match value.icon {
            Icon::Name(name) => (name, Vec::default()),
            Icon::Pixmaps(pixmaps) => (String::default(), pixmaps),
        };
        StructureBuilder::new()
            .add_field(name)
            .add_field(pixmaps)
            .add_field(value.title)
            .add_field(value.description)
            .build()
    }
}

#[derive(Default)]
struct Callbacks {
    on_activate: Option<Box<dyn Fn(i32, i32) + Sync + Send>>,
    on_secondary_activate: Option<Box<dyn Fn(i32, i32) + Sync + Send>>,
    on_scroll: Option<Box<dyn Fn(i32, String) + Sync + Send>>,
    on_provide_xdg_activation_token: Option<Box<dyn Fn(String) + Sync + Send>>,
}

#[derive(Debug, Clone, Type)]
pub struct Attention {
    icon: Icon,
    movie_name: String,
}

impl Attention {
    pub fn new() -> Self {
        Self {
            icon: Icon::Name(String::default()),
            movie_name: String::default(),
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    pub fn movie_name(mut self, name: impl Into<String>) -> Self {
        self.movie_name = name.into();
        self
    }
}

#[derive(Debug, Clone, Type)]
pub struct StatusNotifierItemOptions {
    category: Category,
    title: String,
    status: Status,
    icon: Icon,
    overlay: Icon,
    attention: Attention,
    is_menu: bool,
    tooltip: ToolTip,
}

impl StatusNotifierItemOptions {
    pub fn new() -> Self {
        Self {
            category: Category::default(),
            title: String::default(),
            status: Status::default(),
            icon: Icon::Name(String::default()),
            overlay: Icon::Name(String::default()),
            attention: Attention {
                icon: Icon::Name(String::default()),
                movie_name: String::default(),
            },
            is_menu: false,
            tooltip: ToolTip {
                icon: Icon::Name(String::default()),
                title: String::default(),
                description: String::default(),
            },
        }
    }

    /// This property is used for sorting purposes
    pub fn category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    /// Text displayed when a user hover the icon
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// This tells the compositor if the item should be on the system tray
    /// or hidden
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Icon displayed, could be a freedesktop compliant icon name or an array of icons
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = icon;
        self
    }

    /// Icon displayed on top of icon
    pub fn overlay(mut self, overlay: Icon) -> Self {
        self.overlay = overlay;
        self
    }

    /// Icon dislayed when `Status` is `Status::NeedsAttention`
    pub fn attention(mut self, attention: Attention) -> Self {
        self.attention = attention;
        self
    }

    pub fn is_menu(mut self, is_menu: bool) -> Self {
        self.is_menu = is_menu;
        self
    }

    /// Text and description displayed when hovering the tray icon
    /// the compositor can override the title with the tooltip title
    pub fn tooltip(mut self, tooltip: ToolTip) -> Self {
        self.tooltip = tooltip;
        self
    }
}

struct StatusNotifierItemInterface {
    id: String,
    callbacks: Callbacks,
    options: StatusNotifierItemOptions,
}

#[interface(name = "org.kde.StatusNotifierItem")]
impl StatusNotifierItemInterface {
    #[zbus(property, name = "Category")]
    async fn category(&self) -> String {
        self.options.category.to_string()
    }

    #[zbus(property, name = "Id")]
    async fn id(&self) -> String {
        self.id.clone()
    }

    #[zbus(property, name = "Title")]
    async fn title(&self) -> String {
        self.options.title.clone()
    }

    #[zbus(property, name = "Status")]
    async fn status(&self) -> String {
        self.options.status.to_string()
    }

    #[zbus(property, name = "IconName")]
    async fn icon_name(&self) -> String {
        match &self.options.icon {
            Icon::Name(name) => name.clone(),
            _ => String::default(),
        }
    }

    #[zbus(property, name = "IconPixmap")]
    async fn icon_pixmap(&self) -> Vec<Pixmap> {
        match &self.options.icon {
            Icon::Pixmaps(pixmaps) => pixmaps.clone(),
            _ => Vec::default(),
        }
    }

    #[zbus(property, name = "OverlayIconName")]
    async fn overlay_icon_name(&self) -> String {
        match &self.options.overlay {
            Icon::Name(name) => name.clone(),
            _ => String::default(),
        }
    }

    #[zbus(property, name = "OverlayIconPixmap")]
    async fn overlay_icon_pixmap(&self) -> Vec<Pixmap> {
        match &self.options.overlay {
            Icon::Pixmaps(pixmaps) => pixmaps.clone(),
            _ => Vec::default(),
        }
    }

    #[zbus(property, name = "AttentionIconName")]
    async fn attention_icon_name(&self) -> String {
        match &self.options.attention.icon {
            Icon::Name(name) => name.clone(),
            _ => String::default(),
        }
    }

    #[zbus(property, name = "AttentionIconPixmap")]
    async fn attention_icon_pixmap(&self) -> Vec<Pixmap> {
        match &self.options.attention.icon {
            Icon::Pixmaps(pixmaps) => pixmaps.clone(),
            _ => Vec::default(),
        }
    }

    #[zbus(property, name = "AttentionMovieName")]
    async fn attention_movie_name(&self) -> String {
        self.options.attention.movie_name.clone()
    }

    #[zbus(property, name = "ToolTip")]
    async fn tooltip(&self) -> ToolTip {
        self.options.tooltip.clone()
    }

    #[zbus(property, name = "ItemIsMenu")]
    async fn item_is_menu(&self) -> bool {
        self.options.is_menu
    }

    #[zbus(property, name = "Menu")]
    async fn menu(&self) -> OwnedObjectPath {
        OwnedObjectPath::try_from(DBUS_MENU_PATH).unwrap()
    }

    async fn provide_xdg_activation_token(&self, token: String) {
        self.callbacks
            .on_provide_xdg_activation_token
            .as_ref()
            .map(move |xdg_activation_token| xdg_activation_token(token));
    }

    async fn activate(&self, x: i32, y: i32) {
        self.callbacks
            .on_activate
            .as_ref()
            .map(move |activate| activate(x, y));
    }

    async fn secondary_activate(&self, x: i32, y: i32) {
        self.callbacks
            .on_secondary_activate
            .as_ref()
            .map(move |activate| activate(x, y));
    }

    async fn scroll(&self, delta: i32, orientation: String) {
        self.callbacks
            .on_scroll
            .as_ref()
            .map(move |scroll| scroll(delta, orientation));
    }

    #[zbus(signal, name = "NewTitle")]
    async fn new_title(&self, cx: &SignalContext<'_>) -> zbus::Result<()>;

    #[zbus(signal, name = "NewIcon")]
    async fn new_icon(&self, cx: &SignalContext<'_>) -> zbus::Result<()>;

    #[zbus(signal, name = "NewAttentionIcon")]
    async fn new_attention_icon(&self, cx: &SignalContext<'_>) -> zbus::Result<()>;

    #[zbus(signal, name = "NewOverlayIcon")]
    async fn new_overlay_icon(&self, cx: &SignalContext<'_>) -> zbus::Result<()>;

    #[zbus(signal, name = "NewMenu")]
    async fn new_menu(&self, cx: &SignalContext<'_>) -> zbus::Result<()>;

    #[zbus(signal, name = "NewToolTip")]
    async fn new_tooltip(&self, cx: &SignalContext<'_>) -> zbus::Result<()>;

    #[zbus(signal, name = "NewStatus")]
    async fn new_status(&self, cx: &SignalContext<'_>, status: String) -> zbus::Result<()>;
}

pub struct StatusNotifierItem {
    connection: zbus::Connection,
    item_ref: InterfaceRef<StatusNotifierItemInterface>,
    menu_ref: InterfaceRef<DBusMenuInterface>,
}

impl StatusNotifierItem {
    pub async fn new(
        id: i32,
        options: StatusNotifierItemOptions,
        menu: Option<DBusMenu>,
    ) -> zbus::Result<Self> {
        let watcher = StatusNotifierWatcher::new().await?;
        let item_iface = StatusNotifierItemInterface {
            id: id.to_string(),
            options,
            callbacks: Default::default(),
        };
        let menu_iface = DBusMenuInterface {
            menu: menu.unwrap_or(DBusMenu::default()),
            revision: 1,
        };
        let name = format!(
            "org.freedesktop.StatusNotifierItem-{}-{}",
            std::process::id(),
            id
        );

        let conn = zbus::connection::Builder::session()?
            .name(name.clone())?
            .serve_at(STATUS_NOTIFIER_ITEM_PATH, item_iface)?
            .serve_at(DBUS_MENU_PATH, menu_iface)?
            .build()
            .await?;
        watcher.register_status_notifier_item(name).await?;

        let item_ref = conn
            .object_server()
            .interface::<_, StatusNotifierItemInterface>(STATUS_NOTIFIER_ITEM_PATH)
            .await?;
        let menu_ref = conn
            .object_server()
            .interface::<_, DBusMenuInterface>(DBUS_MENU_PATH)
            .await?;
        Ok(Self {
            connection: conn,
            item_ref,
            menu_ref,
        })
    }

    /// Usually called when the user left clicks the icon
    pub async fn on_activate(&self, fun: Box<dyn Fn(i32, i32) + Sync + Send>) {
        let mut iface = self.item_ref.get_mut().await;
        iface.callbacks.on_activate = Some(fun);
    }

    /// Usually called when the user middle clicks the icon
    pub async fn on_secondary_activate(&self, fun: Box<dyn Fn(i32, i32) + Sync + Send>) {
        let mut iface = self.item_ref.get_mut().await;
        iface.callbacks.on_secondary_activate = Some(fun);
    }

    /// Called when the user scrolls in the icon
    pub async fn on_scroll(&self, fun: Box<dyn Fn(i32, String) + Sync + Send>) {
        let mut iface = self.item_ref.get_mut().await;
        iface.callbacks.on_scroll = Some(fun);
    }

    /// Usually called when `Activate` and `OnSecondaryActivate` is called
    pub async fn on_provide_xdg_activation_token(&self, fun: Box<dyn Fn(String) + Sync + Send>) {
        let mut iface = self.item_ref.get_mut().await;
        iface.callbacks.on_provide_xdg_activation_token = Some(fun);
    }

    /// Changes the current title. This may not work if the tooltip title is set
    pub async fn set_title(&self, name: impl Into<String>) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        iface.options.title = name.into();
        iface.new_title(cx).await?;
        Ok(())
    }

    /// Changes the current icon.
    pub async fn set_icon(&self, icon: Icon) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        iface.options.icon = icon;
        iface.new_icon(cx).await?;
        Ok(())
    }

    /// Changes the current overlay icon.
    pub async fn set_overlay(&self, overlay: Icon) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        iface.options.overlay = overlay;
        iface.new_overlay_icon(cx).await?;
        Ok(())
    }

    /// Changes the current attention.
    pub async fn set_attention(&self, attention: Attention) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        iface.options.attention = attention;
        iface.new_attention_icon(cx).await?;
        Ok(())
    }

    /// Changes the current tooltip.
    pub async fn set_tooltip(&self, tooltip: ToolTip) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        iface.options.tooltip = tooltip;
        iface.new_tooltip(cx).await?;
        Ok(())
    }

    /// Changes only the tooltip title.
    pub async fn set_tooltip_title(&self, title: String) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        iface.options.tooltip.title = title;
        iface.new_tooltip(cx).await?;
        Ok(())
    }

    /// Changes only the tooltip description.
    pub async fn set_tooltip_description(&self, description: String) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        iface.options.tooltip.description = description;
        iface.new_tooltip(cx).await?;
        Ok(())
    }

    /// Changes the current status
    pub async fn set_status(&self, status: Status) -> zbus::Result<()> {
        let cx = self.item_ref.signal_context();
        let mut iface = self.item_ref.get_mut().await;
        let status_str = status.to_string();
        iface.options.status = status;
        iface.new_status(cx, status_str).await?;
        Ok(())
    }

    /// Adds a submenu to the menu
    pub async fn add_submenu(&self, submenu: DBusMenuItem) -> zbus::Result<()> {
        let mut iface = self.menu_ref.get_mut().await;
        let updated = iface.menu.add_submenu(submenu);
        drop(iface);
        self.update_layout(0, updated, Vec::default()).await?;
        Ok(())
    }

    pub async fn add_submenu_to(&self, user_id: &str, submenu: DBusMenuItem) -> zbus::Result<()> {
        let mut iface = self.menu_ref.get_mut().await;
        let updated = iface.menu.add_submenu_to(user_id, submenu);
        drop(iface);
        if let Some((parent_id, updated)) = updated {
            self.update_layout(parent_id, updated, Vec::default())
                .await?;
        }
        Ok(())
    }

    /// Removes a submenu from the menu tree
    pub async fn remove_submenu(&self, id: impl Into<String>) -> zbus::Result<()> {
        let mut iface = self.menu_ref.get_mut().await;
        if let Some((parent_id, removed)) = iface.menu.remove_submenu(id) {
            drop(iface);
            self.update_layout(parent_id, Vec::default(), removed)
                .await?;
        }
        Ok(())
    }

    /// Updates the submenu properties.
    pub async fn update_submenu<'a>(
        &self,
        id: &str,
        new_properties: Option<Vec<MenuProperty>>,
        remove_properties: Option<impl IntoIterator<Item = &'a str>>,
    ) -> zbus::Result<()> {
        let mut iface = self.menu_ref.get_mut().await;
        let (parent_id, updated, removed) =
            iface
                .menu
                .update_submenu_properties(id, new_properties, remove_properties);
        drop(iface);
        self.update_layout(
            parent_id,
            updated.map_or(Vec::default(), |prop| vec![prop]),
            removed.map_or(Vec::default(), |prop| vec![prop]),
        )
        .await?;
        Ok(())
    }

    async fn update_layout(
        &self,
        parent_id: i32,
        updated: Vec<DBusMenuUpdatedProperties>,
        removed: Vec<DBusMenuRemovedProperties>,
    ) -> zbus::Result<()> {
        let cx = self.menu_ref.signal_context();
        let mut iface = self.menu_ref.get_mut().await;
        if !updated.is_empty() || !removed.is_empty() {
            iface.revision += 1;
            iface.layout_updated(cx, iface.revision, parent_id).await?;
            iface.items_properties_updated(cx, updated, removed).await?;
        }
        Ok(())
    }
}
