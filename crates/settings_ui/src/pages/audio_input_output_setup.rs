use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait},
};
use gpui::{AnyElement, App, ElementId, ReadGlobal, SharedString, Window};
use settings::{AudioInputDeviceName, AudioOutputDeviceName, SettingsStore};
use ui::{ContextMenu, DropdownMenu, DropdownStyle, IconPosition, IntoElement};
use util::ResultExt;

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile, update_settings_file};

pub(crate) const SYSTEM_DEFAULT: &str = "System Default";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum AudioDeviceKind {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub(crate) struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
}

impl std::fmt::Display for AudioDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

pub(crate) fn get_audio_devices(kind: AudioDeviceKind) -> Vec<AudioDeviceInfo> {
    let Some(devices) = default_host().devices().ok() else {
        return Vec::new();
    };

    devices
        .filter_map(|device| {
            let id = device.id().ok()?.to_string();
            let desc = device.description().ok()?;

            let is_valid_kind = match kind {
                AudioDeviceKind::Input => desc.supports_input(),
                AudioDeviceKind::Output => desc.supports_output(),
            };

            if !is_valid_kind {
                return None;
            }

            let name = device
                .description()
                .map(|desc| desc.name().to_string())
                .unwrap_or_else(|_| "Unknown Device".to_string());

            Some(AudioDeviceInfo { id, name })
        })
        .collect()
}

pub(crate) fn get_current_device(
    current_id: Option<&String>,
    devices: &[AudioDeviceInfo],
) -> Option<AudioDeviceInfo> {
    let Some(current_id) = current_id else {
        return None;
    };
    devices.iter().find(|d| &d.id == current_id).cloned()
}

pub(crate) fn render_audio_device_dropdown<F>(
    dropdown_id: impl Into<ElementId>,
    kind: AudioDeviceKind,
    current_device_id: Option<String>,
    on_select: F,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    F: Fn(Option<String>, &mut Window, &mut App) + Clone + 'static,
{
    let devices = get_audio_devices(kind);
    let current_device = get_current_device(current_device_id.as_ref(), &devices);

    let menu = ContextMenu::build(window, cx, {
        let current_device = current_device.clone();
        move |mut menu, _, _cx| {
            let is_system_default = current_device.is_none();
            menu = menu.toggleable_entry(
                SYSTEM_DEFAULT,
                is_system_default,
                IconPosition::Start,
                None,
                {
                    let on_select = on_select.clone();
                    move |window, cx| {
                        on_select(None, window, cx);
                    }
                },
            );

            for device in &devices {
                let is_current = current_device
                    .as_ref()
                    .map(|info| info.id == device.id)
                    .unwrap_or(false);
                let device_id = device.id.clone();

                menu = menu.toggleable_entry(
                    device.to_string(),
                    is_current,
                    IconPosition::Start,
                    None,
                    {
                        let on_select = on_select.clone();
                        move |window, cx| {
                            on_select(Some(device_id.clone()), window, cx);
                        }
                    },
                );
            }
            menu
        }
    });

    DropdownMenu::new(
        dropdown_id,
        current_device
            .map(|info| info.name)
            .unwrap_or(SYSTEM_DEFAULT.to_string()),
        menu,
    )
    .style(DropdownStyle::Outlined)
    .full_width(true)
    .into_any_element()
}

fn render_settings_audio_device_dropdown<T: AsRef<Option<String>> + From<Option<String>> + Send>(
    field: SettingField<T>,
    file: SettingsUiFile,
    kind: AudioDeviceKind,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value): (_, Option<&T>) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_device_id = current_value.map(|x| x.as_ref().clone()).flatten();

    let dropdown_id: SharedString = match kind {
        AudioDeviceKind::Input => "input-audio-device-dropdown".into(),
        AudioDeviceKind::Output => "output-audio-device-dropdown".into(),
    };

    render_audio_device_dropdown(
        dropdown_id,
        kind,
        current_device_id,
        move |device_id, window, cx| {
            let value: Option<T> = device_id.map(|id| T::from(Some(id)));
            update_settings_file(
                file.clone(),
                field.json_path,
                window,
                cx,
                move |settings, _cx| {
                    (field.write)(settings, value);
                },
            )
            .log_err();
        },
        window,
        cx,
    )
}

pub fn render_input_audio_device_dropdown(
    field: SettingField<AudioInputDeviceName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    render_settings_audio_device_dropdown(field, file, AudioDeviceKind::Input, window, cx)
}

pub fn render_output_audio_device_dropdown(
    field: SettingField<AudioOutputDeviceName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    render_settings_audio_device_dropdown(field, file, AudioDeviceKind::Output, window, cx)
}
