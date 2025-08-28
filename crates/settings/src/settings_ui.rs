use std::any::Any;

use gpui::{AnyElement, App, Window};
use smallvec::SmallVec;

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
    NumericStepper,
    ToggleGroup,
    Custom(Box<dyn Fn(SettingsValue<serde_json::Value>, &mut Window, &mut App) -> AnyElement>),
}

pub struct SettingsValue<T> {
    pub title: &'static str,
    pub path: SmallVec<[&'static str; 1]>,
    pub value: Option<T>,
    pub default_value: T,
}

impl<T> SettingsValue<T> {
    pub fn read(&self) -> &T {
        match &self.value {
            Some(value) => value,
            None => &self.default_value,
        }
    }

    pub fn write(&self, _value: T) {
        todo!()
    }
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

impl SettingsUI for u64 {
    fn settings_ui_render() -> SettingsUIRender {
        SettingsUIRender::Item(SettingsUIItemSingle::NumericStepper)
    }

    fn settings_ui_item() -> SettingsUIItem {
        SettingsUIItem {
            item: SettingsUIItemVariant::None,
        }
    }
}

/*
FOR DOC COMMENTS ON "Contents" TYPES:
define trait: SettingsUIDocProvider with derive
derive creates:
impl SettingsUIDocProvider for Foo {
    fn settings_ui_doc() -> Hashmap<&'static str, &'static str> {
        Hashmap::from(Foo.fields.map(|field| (field.name, field.doc_comment)))
    }
}

on derive settings_ui, have attr
#[settings_ui(doc_from = "Foo")]

and have derive(SettingsUI) do

if doc_from {
quote! {
        doc_comments = doc_from.type::settings_ui_doc();
        for fields {
            field.doc_comment = doc_comments.get(field.name).unwrap()
        }
    }
} else {
 doc_comments = <Self as Settings>FileContent::settings_ui
}

FOR PATH:
if derive attr also contains "Settings", then we can use <T as Settings>::KEY,
otherwise we need a #[settings_ui(path = ...)].

FOR BOTH OF ABOVE, we can check if derive() attr contains Settings, otherwise assert that both doc_from and path are present
like so: #[settings_ui(doc_from = "Foo", path = "foo")]
 */

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
