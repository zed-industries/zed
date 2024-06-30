use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use serde::Serialize;
use zbus::{
    interface,
    object_server::SignalContext,
    zvariant::{Structure, StructureBuilder, Type, Value},
};

pub const DBUS_MENU_PATH: &str = "/MenuBar";

const MENU_PROPERTY_TYPE: &str = "type";
const MENU_PROPERTY_LABEL: &str = "label";
const MENU_PROPERTY_ENABLED: &str = "enabled";
const MENU_PROPERTY_VISIBLE: &str = "visible";
const MENU_PROPERTY_ICON_NAME: &str = "icon-name";
const MENU_PROPERTY_ICON_DATA: &str = "icon-data";
const MENU_PROPERTY_TOGGLE_TYPE: &str = "toggle-type";
const MENU_PROPERTY_TOGGLE_STATE: &str = "toggle-state";

#[derive(Default, Debug, Clone, Type)]
pub struct Pixmap {
    width: i32,
    height: i32,
    bytes: Vec<u8>,
}

impl From<Pixmap> for Structure<'_> {
    fn from(value: Pixmap) -> Self {
        StructureBuilder::new()
            .add_field(value.width)
            .add_field(value.height)
            .add_field(value.bytes)
            .build()
    }
}

#[derive(Debug, Clone, Type)]
#[zvariant(signature = "(sv)")]
pub enum Icon {
    Name(String),
    Bytes(Vec<u8>),
    Pixmaps(Vec<Pixmap>),
}

///
#[derive(Clone)]
pub enum MenuType {
    /// "standard"
    Standard,
    /// "separator"
    Separator,
}

///
#[derive(Clone)]
pub enum MenuToggleType {
    /// "checkmark"
    Checkmark,
    /// "radio"
    Radio,
}

#[derive(Clone)]
pub enum MenuProperty {
    ///
    Type(MenuType),
    ///
    Label(String),
    ///
    Enabled(bool),
    ///
    Visible(bool),
    ///
    Icon(Icon),
    ///
    ToggleType(MenuToggleType),
    ///
    ToggleState(i32),
}

impl<'a> Into<Value<'a>> for MenuProperty {
    fn into(self) -> Value<'a> {
        match self {
            Self::Type(menu) => match menu {
                MenuType::Standard => Value::from("standard"),
                MenuType::Separator => Value::from("separator"),
            },
            Self::Label(label) => Value::from(label),
            Self::Enabled(enabled) => Value::from(enabled),
            Self::Visible(visible) => Value::from(visible),
            Self::Icon(icon) => match icon {
                Icon::Name(name) => Value::from(name),
                Icon::Bytes(bytes) => Value::from(bytes),
                _ => panic!("Wrong icon variant"),
            },
            Self::ToggleType(toggle_type) => match toggle_type {
                MenuToggleType::Checkmark => Value::from("checkmark"),
                MenuToggleType::Radio => Value::from("radio"),
            },
            Self::ToggleState(state) => Value::from(state),
        }
    }
}

#[derive(Default, Serialize, Type)]
struct DBusMenuLayoutItem {
    id: i32,
    properties: HashMap<String, Value<'static>>,
    children: Vec<Value<'static>>,
}

impl<'a> From<DBusMenuLayoutItem> for Structure<'a> {
    fn from(value: DBusMenuLayoutItem) -> Self {
        StructureBuilder::new()
            .add_field(value.id)
            .add_field(value.properties)
            .add_field(value.children)
            .build()
    }
}

#[derive(Debug, Serialize, Type)]
pub(crate) struct DBusMenuUpdatedProperties {
    id: i32,
    properties: HashMap<String, Value<'static>>,
}

#[derive(Debug, Serialize, Type)]
pub(crate) struct DBusMenuRemovedProperties {
    id: i32,
    property_names: Vec<String>,
}

#[derive(Default)]
pub(crate) struct DBusMenuItemPrivate {
    id: i32,
    parent_id: i32,
    pub(crate) user_id: String,
    properties: HashMap<&'static str, MenuProperty>,
    children: Vec<i32>,
}

impl DBusMenuItemPrivate {
    fn filter_properties(&self, props: &Vec<String>) -> HashMap<String, Value<'static>> {
        if props.is_empty() {
            return self
                .properties
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone().into()))
                .collect();
        }
        let mut filtered_props = HashMap::default();
        for prop_name in props {
            let prop_name = prop_name.as_str();
            let Some(prop) = self.properties.get(prop_name) else {
                continue;
            };
            filtered_props.insert(prop_name.to_string(), prop.clone().into());
        }
        filtered_props
    }

    pub(crate) fn update_properties(
        &mut self,
        properties: impl IntoIterator<Item = MenuProperty>,
    ) -> Option<DBusMenuUpdatedProperties> {
        for prop in properties {
            let key = match &prop {
                MenuProperty::Type(_) => MENU_PROPERTY_TYPE,
                MenuProperty::Label(_) => MENU_PROPERTY_LABEL,
                MenuProperty::Enabled(_) => MENU_PROPERTY_ENABLED,
                MenuProperty::Visible(_) => MENU_PROPERTY_VISIBLE,
                MenuProperty::Icon(icon) => match icon {
                    Icon::Name(_) => MENU_PROPERTY_ICON_NAME,
                    Icon::Bytes(_) => MENU_PROPERTY_ICON_DATA,
                    _ => panic!("Wrong Icon Type"),
                },
                MenuProperty::ToggleType(_) => MENU_PROPERTY_TOGGLE_TYPE,
                MenuProperty::ToggleState(_) => MENU_PROPERTY_TOGGLE_STATE,
            };
            self.properties.insert(key, prop);
        }
        let mut result = None;
        if !self.properties.is_empty() {
            result = Some(DBusMenuUpdatedProperties {
                id: self.id,
                properties: self
                    .properties
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone().into()))
                    .collect(),
            });
        }
        result
    }

    pub(crate) fn remove_properties<'a>(
        &mut self,
        properties: impl IntoIterator<Item = &'a str>,
    ) -> Option<DBusMenuRemovedProperties> {
        let mut properties_removed: Vec<String> = Vec::default();
        for prop_name in properties {
            let Some(_) = self.properties.remove(prop_name) else {
                continue;
            };
            properties_removed.push(prop_name.into());
        }
        let mut result = None;
        if !properties_removed.is_empty() {
            result = Some(DBusMenuRemovedProperties {
                id: self.id,
                property_names: properties_removed,
            });
        }
        result
    }
}

///
#[derive(Default)]
pub struct DBusMenuItem {
    user_id: String,
    properties: HashMap<&'static str, MenuProperty>,
    children: Vec<DBusMenuItem>,
}

impl DBusMenuItem {
    ///
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            ..Default::default()
        }
    }

    ///
    pub fn set_type(&mut self, menu_type: MenuType) -> &mut Self {
        self.properties
            .insert(MENU_PROPERTY_TYPE, MenuProperty::Type(menu_type));
        self
    }

    ///
    pub fn set_label(&mut self, label: impl Into<String>) -> &mut Self {
        self.properties
            .insert(MENU_PROPERTY_LABEL, MenuProperty::Label(label.into()));
        self
    }

    ///
    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        match enabled {
            true => self.properties.remove(MENU_PROPERTY_ENABLED),
            false => self
                .properties
                .insert(MENU_PROPERTY_ENABLED, MenuProperty::Enabled(false)),
        };
        self
    }

    ///
    pub fn set_visible(&mut self, visible: bool) -> &mut Self {
        match visible {
            true => self.properties.remove(MENU_PROPERTY_VISIBLE),
            false => self
                .properties
                .insert(MENU_PROPERTY_VISIBLE, MenuProperty::Visible(false)),
        };
        self
    }

    ///
    pub fn set_icon(&mut self, icon: Icon) -> &mut Self {
        match icon {
            Icon::Name(_) | Icon::Bytes(_) => self
                .properties
                .insert(MENU_PROPERTY_ICON_NAME, MenuProperty::Icon(icon)),
            _ => panic!("Invalid Icon Type"),
        };
        self
    }

    ///
    pub fn set_toggle_type(&mut self, toggle_type: MenuToggleType) -> &mut Self {
        self.properties.insert(
            MENU_PROPERTY_TOGGLE_TYPE,
            MenuProperty::ToggleType(toggle_type),
        );
        self
    }

    ///
    pub fn set_toggle_state(&mut self, state: i32) -> &mut Self {
        self.properties
            .insert(MENU_PROPERTY_TOGGLE_TYPE, MenuProperty::ToggleState(state));
        self
    }

    ///
    pub fn set_children(&mut self, mut children: Vec<DBusMenuItem>) {
        self.children = children;
    }
}

///
pub struct DBusMenu {
    next_id: i32,
    pub(crate) items: HashMap<i32, DBusMenuItemPrivate>,
    user_id_to_id_map: HashMap<String, i32>,
}

impl DBusMenu {
    ///
    pub fn new() -> Self {
        Self {
            next_id: 1,
            items: HashMap::from_iter([(0, DBusMenuItemPrivate::default())]),
            user_id_to_id_map: HashMap::default(),
        }
    }

    ///
    pub fn submenu(mut self, submenu: DBusMenuItem) -> Self {
        self.add_to_root(submenu, 0);
        self
    }

    /// Add a submenu to the root.
    pub(crate) fn add_submenu(&mut self, submenu: DBusMenuItem) -> Vec<DBusMenuUpdatedProperties> {
        let mut result = Vec::default();
        let new_id = self.add_to_root(submenu, 0);
        let mut queue = VecDeque::default();
        queue.push_back(self.items.get(&new_id).unwrap());
        while !queue.is_empty() {
            let submenu = queue.pop_front().unwrap();
            result.push(DBusMenuUpdatedProperties {
                id: new_id,
                properties: submenu
                    .properties
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone().into()))
                    .collect(),
            });
            for id in &submenu.children {
                queue.push_back(self.items.get(&id).unwrap())
            }
        }
        result
    }

    /// Add a submenu to the specified id, if the id is not found,
    /// the submenu is not gonna be added
    pub(crate) fn add_submenu_to(
        &mut self,
        user_id: &str,
        submenu: DBusMenuItem,
    ) -> Option<(i32, Vec<DBusMenuUpdatedProperties>)> {
        let Some(parent) = self.user_id_to_id_map.get(user_id) else {
            return None;
        };
        let parent_id = *parent;
        drop(parent);
        let new_id = self.add_to_root(submenu, parent_id);
        let mut result = Vec::default();
        let mut queue = VecDeque::default();
        queue.push_back(self.items.get(&new_id).unwrap());
        while !queue.is_empty() {
            let submenu = queue.pop_front().unwrap();
            result.push(DBusMenuUpdatedProperties {
                id: submenu.id,
                properties: submenu
                    .properties
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone().into()))
                    .collect(),
            });
            for id in &submenu.children {
                queue.push_back(self.items.get(id).unwrap());
            }
        }
        Some((parent_id, result))
    }

    /// Returns the parent id of the submenu and a vector of removed properties.
    pub(crate) fn remove_submenu(
        &mut self,
        user_id: impl Into<String>,
    ) -> Option<(i32, Vec<DBusMenuRemovedProperties>)> {
        let Some(id) = self.user_id_to_id_map.get(&user_id.into()) else {
            return None;
        };
        let menu = self.items.remove(id).unwrap();
        let parent_id = menu.parent_id;
        let mut result = Vec::default();
        let mut queue = VecDeque::default();
        queue.push_back(menu);
        while !queue.is_empty() {
            let submenu = queue.pop_front().unwrap();
            let user_id = submenu.user_id;
            let parent = self.items.get_mut(&submenu.parent_id).unwrap();
            parent.children.retain(|child| *child != submenu.id);
            self.user_id_to_id_map.remove(&user_id);
            for id in submenu.children {
                queue.push_back(self.items.remove(&id).unwrap());
            }
            if !submenu.properties.is_empty() {
                result.push(DBusMenuRemovedProperties {
                    id: submenu.id,
                    property_names: submenu
                        .properties
                        .iter()
                        .map(|(k, _)| k.to_string())
                        .collect(),
                });
            }
        }
        Some((parent_id, result))
    }

    pub(crate) fn update_submenu_properties<'a>(
        &mut self,
        user_id: &str,
        new_properties: Option<impl IntoIterator<Item = MenuProperty>>,
        remove_properties: Option<impl IntoIterator<Item = &'a str>>,
    ) -> (
        i32,
        Option<DBusMenuUpdatedProperties>,
        Option<DBusMenuRemovedProperties>,
    ) {
        let mut updated = None;
        let mut removed = None;
        let mut parent_id = 0;
        if let Some(id) = self.user_id_to_id_map.get(user_id) {
            let submenu = self.items.get_mut(id).unwrap();
            parent_id = submenu.parent_id;
            if let Some(props) = remove_properties {
                removed = submenu.remove_properties(props);
            }
            if let Some(props) = new_properties {
                updated = submenu.update_properties(props);
            }
        }
        (parent_id, updated, removed)
    }

    fn add_to_root(&mut self, submenu: DBusMenuItem, parent_id: i32) -> i32 {
        let id = self.next_id;
        let new_submenu = DBusMenuItemPrivate {
            id,
            parent_id,
            user_id: submenu.user_id,
            properties: submenu
                .properties
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            ..Default::default()
        };
        self.next_id += 1;
        for child in submenu.children {
            self.add_to_root(child, id);
        }
        self.user_id_to_id_map
            .insert(new_submenu.user_id.clone(), id);
        self.items.insert(id, new_submenu);
        if let Some(parent) = self.items.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    fn to_dbus(&self, parent_id: i32, depth: i32, properties: &Vec<String>) -> DBusMenuLayoutItem {
        let parent = self.items.get(&parent_id).unwrap();
        let mut menu = DBusMenuLayoutItem {
            id: parent.id,
            ..Default::default()
        };
        menu.properties = parent.filter_properties(properties);
        if !parent.children.is_empty() && depth != 0 {
            menu.properties
                .insert("children-display".into(), Value::from("submenu"));
            for child_id in &parent.children {
                menu.children
                    .push(Value::from(self.to_dbus(*child_id, depth - 1, properties)));
            }
        }
        menu
    }
}

pub(crate) struct DBusMenuInterface {
    pub(crate) menu: DBusMenu,
    pub(crate) revision: u32,
}

#[interface(name = "com.canonical.dbusmenu")]
impl DBusMenuInterface {
    #[zbus(out_args("revision", "layout"))]
    async fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        properties: Vec<String>,
    ) -> (u32, DBusMenuLayoutItem) {
        let menu = self.menu.to_dbus(parent_id, recursion_depth, &properties);
        (self.revision, menu)
    }

    async fn get_group_properties(
        &self,
        ids: Vec<i32>,
        property_names: Vec<String>,
    ) -> Vec<DBusMenuUpdatedProperties> {
        let mut properties = Vec::default();
        for id in ids {
            let menu = self.menu.items.get(&id).unwrap();
            let new_properties = menu.filter_properties(&property_names);
            properties.push(DBusMenuUpdatedProperties {
                id: menu.id,
                properties: new_properties,
            });
        }
        properties
    }

    async fn about_to_show(&self, _id: i32) -> bool {
        false
    }

    #[zbus(signal, name = "ItemsPropertiesUpdated")]
    pub(crate) async fn items_properties_updated(
        &self,
        cx: &SignalContext<'_>,
        updated: Vec<DBusMenuUpdatedProperties>,
        removed: Vec<DBusMenuRemovedProperties>,
    ) -> zbus::Result<()>;

    #[zbus(signal, name = "LayoutUpdated")]
    pub(crate) async fn layout_updated(
        &self,
        cx: &SignalContext<'_>,
        revision: u32,
        parent: i32,
    ) -> zbus::Result<()>;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_dbusmenu_signature() {
        let signature = DBusMenuLayoutItem::signature();
        assert_eq!(signature.as_str(), "(ia{sv}av)");
    }

    #[test]
    fn test_dbusmenu_unique_ids() {
        // let menu = DBusMenu::from_items(Vec::from_iter([
        //     DBusMenuItemPrivate::new().children(Vec::from_iter([
        //         DBusMenuItemPrivate::new(),
        //         DBusMenuItemPrivate::new(),
        //     ])),
        //     DBusMenuItemPrivate::new(),
        // ]));
        // let mut queue = VecDeque::new();
        // let mut set: HashSet<i32> = HashSet::new();
        // queue.push_back(menu.root);
        // while !queue.is_empty() {
        //     let item = queue.pop_front().unwrap();
        //     assert!(set.insert(item.id));
        //     for child in item.children {
        //         queue.push_back(child);
        //     }
        // }
    }

    #[test]
    fn test_dbus_user_id() {
        // let mut menu = DBusMenu::from_items(Vec::from_iter([
        //     DBusMenuItemPrivate::new()
        //         .id("id1")
        //         .label("Test1")
        //         .children(Vec::from_iter([
        //             DBusMenuItemPrivate::new().id("id10").label("Test-1"),
        //             DBusMenuItemPrivate::new().id("id20").label("Test-2"),
        //         ])),
        //     DBusMenuItemPrivate::new().id("id2").label("Test2"),
        // ]));
        // let mut found = menu.find_by_user_id_mut("id1");
        // assert!(found.is_some());

        // found = menu.find_by_user_id_mut("id2");
        // assert!(found.is_some());

        // found = menu.find_by_user_id_mut("id10");
        // assert!(found.is_some());

        // found = menu.find_by_user_id_mut("id20");
        // assert!(found.is_some());

        // found = menu.find_by_user_id_mut("id21");
        // assert!(found.is_none());
    }
}
