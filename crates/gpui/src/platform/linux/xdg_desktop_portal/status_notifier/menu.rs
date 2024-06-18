use std::collections::VecDeque;

use zbus::zvariant::Value;

use super::dbusmenu::DBusMenuLayoutItem;

#[derive(Clone)]
pub enum MenuProperties {
    // "standard" | "separator"
    Type(String),
    Label(String),
    Enabled(bool),
    Visible(bool),
    IconName(String),
    // PNG data of the icon
    IconData(Vec<u8>),
    Shortcut(Vec<Vec<String>>),
    /// "checkmark" | "radio"
    ToggleType(String),
    // 0 = off | 1 = on | x = indeterminate
    ToggleState(i32),
}

#[derive(Default, Clone)]
pub struct MenuItem {
    pub(crate) id: i32,
    pub(crate) properties: Vec<MenuProperties>,
    pub(crate) children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn find_by_id(&self, id: i32) -> Option<&Self> {
        let mut queue: VecDeque<&Self> = VecDeque::new();
        for child in &self.children {
            queue.push_back(child);
        }
        while !queue.is_empty() {
            let submenu = queue.pop_front().unwrap();
            if submenu.id == id {
                return Some(submenu);
            }
            for child in &submenu.children {
                queue.push_back(child);
            }
        }
        None
    }

    pub fn to_dbus<'a>(self, depth: i32) -> DBusMenuLayoutItem<'a> {
        let mut menu = DBusMenuLayoutItem {
            id: self.id,
            ..Default::default()
        };
        for property in self.properties {
            match property {
                MenuProperties::Type(menu_type) => {
                    menu.properties
                        .insert("type".into(), Value::from(menu_type));
                }
                MenuProperties::Label(label) => {
                    menu.properties.insert("label".into(), Value::from(label));
                }
                MenuProperties::Enabled(enabled) => {
                    menu.properties
                        .insert("enabled".into(), Value::from(enabled));
                }
                MenuProperties::Visible(visible) => {
                    menu.properties
                        .insert("visible".into(), Value::from(visible));
                }
                MenuProperties::IconName(name) => {
                    menu.properties
                        .insert("icon-name".into(), Value::from(name));
                }
                MenuProperties::IconData(data) => {
                    menu.properties
                        .insert("icon-data".into(), Value::from(data));
                }
                MenuProperties::Shortcut(shortcut) => {
                    menu.properties
                        .insert("shortcut".into(), Value::from(shortcut));
                }
                MenuProperties::ToggleType(toggle) => {
                    menu.properties
                        .insert("toggle-type".into(), Value::from(toggle));
                }
                MenuProperties::ToggleState(state) => {
                    menu.properties
                        .insert("toggle-state".into(), Value::from(state));
                }
                _ => {}
            }
        }
        if !self.children.is_empty() && depth != 0 {
            menu.properties
                .insert("children-display".into(), Value::from("submenu"));
            for child in self.children {
                menu.children.push(Value::from(child.to_dbus(depth - 1)));
            }
        }
        menu
    }
}

pub struct SubmenuGenerator {
    next_id: i32,
}

impl SubmenuGenerator {
    pub fn new() -> Self {
        SubmenuGenerator { next_id: 1 }
    }

    pub fn create_submenu(&mut self, properties: Vec<MenuProperties>) -> MenuItem {
        let id = self.next_id;
        self.next_id += 1;
        MenuItem {
            id,
            properties,
            children: Vec::default(),
        }
    }
}
