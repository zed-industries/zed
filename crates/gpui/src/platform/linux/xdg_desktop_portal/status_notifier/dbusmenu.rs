use std::collections::HashMap;

use serde::Serialize;
use zbus::{
    interface,
    object_server::SignalContext,
    zvariant::{Structure, StructureBuilder, Type, Value},
};

use super::menu::MenuItem;

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
    pub(crate) menu: MenuItem,
    pub(crate) revision: u32,
}

#[interface(name = "com.canonical.dbusmenu")]
impl DBusMenuInterface {
    // TODO: This is not done.
    #[zbus(out_args("revision", "layout"))]
    pub async fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        _property_names: Vec<String>,
    ) -> (u32, DBusMenuLayoutItem) {
        let mut main_menu = DBusMenuLayoutItem::default();
        let menu = self.menu.find_by_id(parent_id).unwrap();
        if !menu.children.is_empty() {
            main_menu
                .properties
                .insert("children-display".into(), Value::from("submenu"));
            for child in &menu.children {
                let submenu = child.clone().to_dbus(recursion_depth);
                main_menu.children.push(Value::from(submenu));
            }
        }
        (self.revision, main_menu)
    }

    pub async fn event(&self, id: i32, event_id: String, event_data: Value<'_>, _timestamp: u32) {
        let menu = self.menu.find_by_id(id).unwrap();
        menu.action
            .as_ref()
            .map(|action| action(event_id, event_data));
    }

    // TODO: Not sure what is the purpose of this.
    pub async fn about_to_show(&self, _id: i32) -> bool {
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
