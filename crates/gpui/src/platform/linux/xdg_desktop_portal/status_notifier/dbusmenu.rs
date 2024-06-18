use std::collections::HashMap;

use super::menu::MenuItem;
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

impl<'a> From<DBusMenuLayoutItem<'a>> for Structure<'a> {
    fn from(value: DBusMenuLayoutItem<'a>) -> Self {
        StructureBuilder::new()
            .add_field(value.id)
            .add_field(value.properties)
            .add_field(value.children)
            .build()
    }
}

#[derive(Default)]
pub struct DBusMenuInterface {
    pub menu: MenuItem,
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
        let menu = if parent_id == 0 {
            &self.menu
        } else {
            // This is not supposed to panic if we do it correctly
            self.menu.find_by_id(parent_id).unwrap()
        };
        if !menu.children.is_empty() {
            main_menu
                .properties
                .insert("children-display".into(), Value::from("submenu"));
            for child in &menu.children {
                let submenu = child.clone().to_dbus(recursion_depth);
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
