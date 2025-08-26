use std::any::Any;

use gpui::{AnyElement, App, Window};

pub trait SettingsUI {
    fn ui_item() -> SettingsUIItem {
        SettingsUIItem {
            item: SettingsUIItemVariant::None,
        }
    }
}

pub struct SettingsUIItem {
    // TODO:
    // path: SmallVec<[&'static str; 8]>,
    pub item: SettingsUIItemVariant,
}

pub enum SettingsUIItemVariant {
    Group(SettingsUIItemGroup),
    Item(SettingsUIItemSingle),
    // TODO: remove
    None,
}

pub struct SettingsUIItemGroup {
    pub items: Vec<SettingsUIItem>,
}

pub enum SettingsUIItemSingle {
    // TODO: default/builtin variants
    Custom(Box<dyn Fn(&dyn Any, &mut Window, &mut App) -> AnyElement>),
}

/* NOTES:

# Root Group
some_setting: {
    # First Item
    # this shouldn't be a group
    # it should just be item with path "some_bool.enabled" and title "Some Bool"
    "some_bool": {
        # this should
        enabled: true | false
    }
    # Second Item
    "some_other_thing": "foo" | "bar" | "baz"
}

Structure:
Group {
    path: "some_item",
    items: [
        Item(
            path: ["some_bool", "enabled"],
        ),
        Item(
            path: ["some_other_thing"],
        )
    ]
}

is the following better than "foo.enabled"?
- for objects with single key "enabled", should just be a bool, with no "enabled"
for objects with enabled and other settings, enabled should be implicit,
so
"vim": false, # disabled
"vim": true, # enabled with default settings
"vim": {
    "default_mode": "HelixNormal"
} # enabled with custom settings
*/
