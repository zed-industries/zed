use audio::{AudioDeviceInfo, AvailableAudioDevices};
use cpal::DeviceId;
use gpui::{AnyElement, App, ElementId, ReadGlobal, SharedString, Window};
use settings::{AudioInputDeviceName, AudioOutputDeviceName, SettingsStore};
use std::str::FromStr;
use ui::{ContextMenu, DropdownMenu, DropdownStyle, IconPosition, IntoElement};
use util::ResultExt;

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile, update_settings_file};

pub(crate) const SYSTEM_DEFAULT: &str = "System Default";

pub(crate) fn get_current_device(
    current_id: Option<&DeviceId>,
    is_input: bool,
    devices: &[AudioDeviceInfo],
) -> Option<AudioDeviceInfo> {
    let Some(current_id) = current_id else {
        return None;
    };
    devices
        .iter()
        .find(|d| d.matches(current_id, is_input))
        .cloned()
}

pub(crate) fn render_audio_device_dropdown<F>(
    dropdown_id: impl Into<ElementId>,
    current_device_id: Option<DeviceId>,
    is_input: bool,
    on_select: F,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    F: Fn(Option<DeviceId>, &mut Window, &mut App) + Clone + 'static,
{
    audio::ensure_devices_initialized(cx);
    let devices = cx.global::<AvailableAudioDevices>().0.clone();
    let current_device = get_current_device(current_device_id.as_ref(), is_input, &devices);

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

            for device in devices.iter().filter(|d| d.matches_input(is_input)) {
                let is_current = current_device
                    .as_ref()
                    .map(|info| info.matches(&device.id, is_input))
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
            .map(|info| info.desc.name().to_string())
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
    is_input: bool,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value): (_, Option<&T>) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_device_id =
        current_value.and_then(|x| x.as_ref().clone().and_then(|x| DeviceId::from_str(&x).ok()));

    let dropdown_id: SharedString = if is_input {
        "input-audio-device-dropdown".into()
    } else {
        "output-audio-device-dropdown".into()
    };

    render_audio_device_dropdown(
        dropdown_id,
        current_device_id,
        is_input,
        move |device_id, window, cx| {
            let value: Option<T> = device_id.map(|id| T::from(Some(id.to_string())));
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
    render_settings_audio_device_dropdown(field, file, true, window, cx)
}

pub fn render_output_audio_device_dropdown(
    field: SettingField<AudioOutputDeviceName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    render_settings_audio_device_dropdown(field, file, false, window, cx)
}
