use anyhow::Context as _;
use fs::Fs;
use gpui::{AnyElement, App, AppContext as _, ReadGlobal as _, Window};
use smallvec::SmallVec;

use crate::SettingsStore;

pub trait SettingsUi {
    fn settings_ui_item() -> SettingsUiItem {
        // todo(settings_ui): remove this default impl, only entry should have a default impl
        // because it's expected that the macro or custom impl use the item and the known paths to create the entry
        SettingsUiItem::None
    }

    fn settings_ui_entry() -> SettingsUiEntry {
        SettingsUiEntry {
            path: None,
            title: "None entry",
            item: SettingsUiItem::None,
        }
    }
}

pub struct SettingsUiEntry {
    // todo(settings_ui): move this back here once there isn't a None variant
    /// The path in the settings JSON file for this setting. Relative to parent
    /// None implies `#[serde(flatten)]` or `Settings::KEY.is_none()` for top level settings
    pub path: Option<&'static str>,
    pub title: &'static str,
    pub item: SettingsUiItem,
}

pub enum SettingsUiItemSingle {
    SwitchField,
    NumericStepper,
    ToggleGroup(&'static [&'static str]),
    /// This should be used when toggle group size > 6
    DropDown(&'static [&'static str]),
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
}

impl SettingsValue<serde_json::Value> {
    pub fn write_value(path: &SmallVec<[&'static str; 1]>, value: serde_json::Value, cx: &mut App) {
        let settings_store = SettingsStore::global(cx);
        let fs = <dyn Fs>::global(cx);

        let rx = settings_store.update_settings_file_at_path(fs.clone(), path.as_slice(), value);
        let path = path.clone();
        cx.background_spawn(async move {
            rx.await?
                .with_context(|| format!("Failed to update setting at path `{:?}`", path.join(".")))
        })
        .detach_and_log_err(cx);
    }
}

impl<T: serde::Serialize> SettingsValue<T> {
    pub fn write(
        path: &SmallVec<[&'static str; 1]>,
        value: T,
        cx: &mut App,
    ) -> Result<(), serde_json::Error> {
        SettingsValue::write_value(path, serde_json::to_value(value)?, cx);
        Ok(())
    }
}

pub struct SettingsUiItemDynamic {
    pub options: Vec<SettingsUiEntry>,
    pub determine_option: fn(&serde_json::Value, &mut App) -> usize,
}

pub struct SettingsUiItemGroup {
    pub items: Vec<SettingsUiEntry>,
}

pub enum SettingsUiItem {
    Group(SettingsUiItemGroup),
    Single(SettingsUiItemSingle),
    Dynamic(SettingsUiItemDynamic),
    None,
}

impl SettingsUi for bool {
    fn settings_ui_item() -> SettingsUiItem {
        SettingsUiItem::Single(SettingsUiItemSingle::SwitchField)
    }
}

impl SettingsUi for Option<bool> {
    fn settings_ui_item() -> SettingsUiItem {
        SettingsUiItem::Single(SettingsUiItemSingle::SwitchField)
    }
}

impl SettingsUi for u64 {
    fn settings_ui_item() -> SettingsUiItem {
        SettingsUiItem::Single(SettingsUiItemSingle::NumericStepper)
    }
}
