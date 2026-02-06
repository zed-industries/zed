use gpui::{AnyElement, App, ReadGlobal, SharedString, Window, prelude::*};
use rodio::cpal::traits::{DeviceTrait, HostTrait};
use settings::{AudioInputDeviceName, AudioOutputDeviceName, SettingsStore};
use ui::{ContextMenu, DropdownMenu, DropdownStyle, IconPosition};
use util::ResultExt;

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile, update_settings_file};

const SYSTEM_DEFAULT: &str = "System Default";

fn get_available_input_devices() -> anyhow::Result<Vec<SharedString>> {
    let host = rodio::cpal::default_host();
    let input_devices = host.input_devices()?;
    input_devices
        .into_iter()
        .map(|device| {
            device
                .name()
                .map(SharedString::from)
                .map_err(anyhow::Error::from)
        })
        .collect()
}

fn get_available_output_devices() -> anyhow::Result<Vec<SharedString>> {
    let host = rodio::cpal::default_host();
    let output_devices = host.output_devices()?;
    output_devices
        .into_iter()
        .map(|device| {
            device
                .name()
                .map(SharedString::from)
                .map_err(anyhow::Error::from)
        })
        .collect()
}

pub fn render_input_audio_device_dropdown(
    field: SettingField<Option<AudioInputDeviceName>>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value): (_, Option<&Option<AudioInputDeviceName>>) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_device: SharedString = current_value
        .and_then(|opt| opt.as_ref())
        .map(|d| SharedString::from(d.0.clone()))
        .unwrap_or_else(|| SYSTEM_DEFAULT.into());

    let devices = get_available_input_devices().unwrap_or_default();

    let menu = ContextMenu::build(window, cx, {
        let current_device = current_device.clone();
        let file = file.clone();
        move |mut menu, _, _cx| {
            for device in &devices {
                let is_current = *device == current_device;
                let device_name = device.clone();
                let file = file.clone();

                menu =
                    menu.toggleable_entry(device.clone(), is_current, IconPosition::Start, None, {
                        move |window, cx| {
                            let value: Option<Option<AudioInputDeviceName>> =
                                if device_name.as_ref() == SYSTEM_DEFAULT {
                                    Some(None)
                                } else {
                                    Some(Some(AudioInputDeviceName(device_name.to_string())))
                                };
                            update_settings_file(
                                file.clone(),
                                field.json_path,
                                window,
                                cx,
                                move |settings, _cx| {
                                    (field.write)(settings, value.clone());
                                },
                            )
                            .log_err();
                        }
                    });
            }
            menu
        }
    });

    DropdownMenu::new("input-audio-device-dropdown", current_device, menu)
        .tab_index(0)
        .style(DropdownStyle::Outlined)
        .into_any_element()
}

pub fn render_output_audio_device_dropdown(
    field: SettingField<Option<AudioOutputDeviceName>>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value): (_, Option<&Option<AudioOutputDeviceName>>) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_device: SharedString = current_value
        .and_then(|opt| opt.as_ref())
        .map(|d| SharedString::from(d.0.clone()))
        .unwrap_or_else(|| SYSTEM_DEFAULT.into());

    let devices = get_available_output_devices().unwrap_or_default();

    let menu = ContextMenu::build(window, cx, {
        let current_device = current_device.clone();
        let file = file.clone();
        move |mut menu, _, _cx| {
            for device in &devices {
                let is_current = *device == current_device;
                let device_name = device.clone();
                let file = file.clone();

                menu =
                    menu.toggleable_entry(device.clone(), is_current, IconPosition::Start, None, {
                        move |window, cx| {
                            let value: Option<Option<AudioOutputDeviceName>> =
                                if device_name.as_ref() == SYSTEM_DEFAULT {
                                    Some(None)
                                } else {
                                    Some(Some(AudioOutputDeviceName(device_name.to_string())))
                                };
                            update_settings_file(
                                file.clone(),
                                field.json_path,
                                window,
                                cx,
                                move |settings, _cx| {
                                    (field.write)(settings, value.clone());
                                },
                            )
                            .log_err();
                        }
                    });
            }
            menu
        }
    });

    DropdownMenu::new("output-audio-device-dropdown", current_device, menu)
        .tab_index(0)
        .style(DropdownStyle::Outlined)
        .into_any_element()
}
