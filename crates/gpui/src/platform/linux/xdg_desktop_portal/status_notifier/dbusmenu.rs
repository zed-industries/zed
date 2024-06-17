use std::collections::HashMap;

use serde::Serialize;
use zbus::{
    interface,
    object_server::SignalContext,
    zvariant::{Structure, StructureBuilder, Type, Value},
};

pub const DBUS_MENU_PATH: &str = "/MenuBar";

#[derive(Default, Serialize, Type)]
pub struct DBusMenuLayoutItem<'a> {
    pub id: i32,
    #[zvariant(signature = "dict")]
    pub properties: HashMap<String, Value<'a>>,
    pub children: Vec<Value<'a>>,
}

impl<'a> Clone for DBusMenuLayoutItem<'a> {
    fn clone(&self) -> Self {
        let properties: HashMap<String, Value<'a>> = self
            .properties
            .iter()
            .map(|(k, v)| (k.clone(), v.try_clone().unwrap()))
            .collect();
        let children: Vec<_> = self
            .children
            .iter()
            .map(|it| it.try_clone().unwrap())
            .collect();
        Self {
            id: self.id.clone(),
            properties,
            children,
        }
    }
}

impl<'a> From<DBusMenuLayoutItem<'a>> for Structure<'a> {
    fn from(value: DBusMenuLayoutItem<'a>) -> Self {
        StructureBuilder::new()
            .add_field(value.id)
            .add_field(value.properties)
            .add_field(value.children)
            .build()
    }
}

#[derive(Clone)]
pub enum DBusMenuProperties {
    // "standard" | "separator"
    Type(String),
    Label(String),
    Enabled(bool),
    Visible(bool),
    IconName(String),
    // PNG data of the icon
    IconData(Vec<u8>),
    Shortcut(Vec<Vec<String>>),
    // "checkmark" | "radio"
    ToggleType(String),
    // 0 = off | 1 = on | x = indeterminate
    ToggleState(i32),
}

#[derive(Clone)]
pub struct Submenu {
    pub id: i32,
    pub properties: Vec<DBusMenuProperties>,
    pub children: Vec<Submenu>,
}

impl<'a> From<Submenu> for DBusMenuLayoutItem<'a> {
    fn from(value: Submenu) -> Self {
        let mut menu = DBusMenuLayoutItem {
            id: value.id,
            ..Default::default()
        };
        for property in value.properties {
            match property {
                DBusMenuProperties::Type(menu_type) => {
                    menu.properties
                        .insert("type".into(), Value::from(menu_type));
                }
                DBusMenuProperties::Label(label) => {
                    menu.properties.insert("label".into(), Value::from(label));
                }
                DBusMenuProperties::Enabled(enabled) => {
                    menu.properties
                        .insert("enabled".into(), Value::from(enabled));
                }
                DBusMenuProperties::Visible(visible) => {
                    menu.properties
                        .insert("visible".into(), Value::from(visible));
                }
                DBusMenuProperties::IconName(name) => {
                    menu.properties
                        .insert("icon-name".into(), Value::from(name));
                }
                DBusMenuProperties::IconData(data) => {
                    menu.properties
                        .insert("icon-data".into(), Value::from(data));
                }
                DBusMenuProperties::Shortcut(shortcut) => {
                    menu.properties
                        .insert("shortcut".into(), Value::from(shortcut));
                }
                DBusMenuProperties::ToggleType(toggle) => {
                    menu.properties
                        .insert("toggle-type".into(), Value::from(toggle));
                }
                DBusMenuProperties::ToggleState(state) => {
                    menu.properties
                        .insert("toggle-state".into(), Value::from(state));
                }
                _ => {}
            }
        }
        if !value.children.is_empty() {
            menu.properties
                .insert("children-display".into(), Value::from("submenu"));
            for child in value.children {
                menu.children.push(Value::from(Self::from(child)));
            }
        }
        menu
    }
}

#[derive(Default)]
pub struct Menu {
    pub children: Vec<Submenu>,
}

#[derive(Default)]
pub struct DBusMenuInterface {
    pub menu: Menu,
}

#[interface(name = "com.canonical.dbusmenu")]
impl DBusMenuInterface {
    // TODO: This is not done.
    #[zbus(out_args("revision", "layout"))]
    pub async fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: Vec<String>,
    ) -> (u32, DBusMenuLayoutItem) {
        let mut main_menu = DBusMenuLayoutItem::default();
        if !self.menu.children.is_empty() {
            main_menu
                .properties
                .insert("children-display".into(), Value::from("submenu"));
            for child in &self.menu.children {
                let submenu = DBusMenuLayoutItem::from(child.clone());
                main_menu.children.push(Value::from(submenu));
            }
        }
        (0, main_menu)
    }

    // TODO: This is not done.
    pub async fn event(&self, id: i32, event_id: String, event_data: Value<'_>, timestamp: u32) {}

    // TODO: This is not done.
    pub async fn about_to_show(&self, id: i32) -> bool {
        false
    }

    #[zbus(signal, name = "LayoutUpdated")]
    pub async fn layout_updated(
        &self,
        cx: &SignalContext<'_>,
        revision: u32,
        parent: i32,
    ) -> zbus::Result<()>;
}
