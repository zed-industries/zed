use std::any::Any;

use gpui::{AnyElement, App, Window};

pub trait SettingsUI {
    fn settings_ui_render() -> SettingsUIRender {
        SettingsUIRender::None
    }
    fn settings_ui_item() -> SettingsUIItem;
}

pub struct SettingsUIItem {
    // TODO: move this back here once there isn't a None variant
    // pub path: &'static str,
    // pub title: &'static str,
    pub item: SettingsUIItemVariant,
}

pub enum SettingsUIItemVariant {
    Group {
        path: &'static str,
        title: &'static str,
        group: SettingsUIItemGroup,
    },
    Item {
        path: &'static str,
        item: SettingsUIItemSingle,
    },
    // TODO: remove
    None,
}

pub struct SettingsUIItemGroup {
    pub items: Vec<SettingsUIItem>,
}

pub enum SettingsUIItemSingle {
    // TODO: default/builtin variants
    SwitchField,
    Custom(Box<dyn Fn(&dyn Any, &mut Window, &mut App) -> AnyElement>),
}

pub enum SettingsUIRender {
    Group {
        title: &'static str,
        items: Vec<SettingsUIItem>,
    },
    Item(SettingsUIItemSingle),
    None,
}

impl SettingsUI for bool {
    fn settings_ui_render() -> SettingsUIRender {
        SettingsUIRender::Item(SettingsUIItemSingle::SwitchField)
    }

    fn settings_ui_item() -> SettingsUIItem {
        SettingsUIItem {
            item: SettingsUIItemVariant::None,
        }
    }
}

/*
#[derive(SettingsUI)]
#[settings_ui(group = "Foo")]
struct Foo {
    // #[settings_ui(render = "my_render_function")]
    pub toggle: bool,
    pub font_size: u32,

   Group(vec![Item {path: "toggle", item: SwitchField}])
}

macro code:
settings_ui_item() {
 group.items = struct.fields.map((field_name, field_type) => quote! { SettingsUIItem::Item {path: #field_type::settings_ui_path().unwrap_or_else(|| #field_name), item:  if field.attrs.render { #render } else field::settings_ui_render()}})
 }
 */

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
