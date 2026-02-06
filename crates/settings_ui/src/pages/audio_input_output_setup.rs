use cpal::{
    Device, default_host,
    traits::{DeviceTrait, HostTrait},
};
use gpui::{AnyElement, App, ReadGlobal, SharedString, Window, prelude::*};
use settings::{AudioDeviceName, SettingsStore};
use ui::{ContextMenu, DropdownMenu, DropdownStyle, IconPosition};
use util::ResultExt;

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile, update_settings_file};

const SYSTEM_DEFAULT: &str = "System Default";

#[derive(Clone, Copy, PartialEq, Eq)]
enum AudioDeviceKind {
    Input,
    Output,
}

struct AudioDeviceInfo {
    id: String,
    label: String,
}

fn should_show_device(device_id: &str) -> bool {
    // On non-Linux platforms, show all devices as they're typically already user-friendly
    #[cfg(not(target_os = "linux"))]
    {
        let _ = device_id;
        return true;
    }

    // On Linux/ALSA, filter to show only user-friendly devices
    #[cfg(target_os = "linux")]
    {
        // Always show pipewire and default
        if device_id == "alsa:default" || device_id == "alsa:pipewire" {
            return true;
        }

        // Show sysdefault entries (one per card) - these are the main user-facing devices
        if device_id.starts_with("alsa:sysdefault:") {
            return true;
        }

        // Filter out everything else:
        // - null (discard samples)
        // - hw:, plughw: (raw hardware access)
        // - front:, surround*: (speaker configuration variants)
        // - hdmi: (usually duplicated by sysdefault)
        // - iec958: (S/PDIF digital)
        false
    }
}

fn get_audio_devices(kind: AudioDeviceKind) -> Vec<AudioDeviceInfo> {
    let host = default_host();

    let devices: Vec<Device> = match kind {
        AudioDeviceKind::Input => host
            .input_devices()
            .map(|d| d.collect())
            .unwrap_or_default(),
        AudioDeviceKind::Output => host
            .output_devices()
            .map(|d| d.collect())
            .unwrap_or_default(),
    };

    devices
        .into_iter()
        .filter_map(|device| {
            let id = device.id().ok()?;
            let id_string = id.to_string();

            if !should_show_device(&id_string) {
                return None;
            }

            let label = device
                .description()
                .map(|desc| desc.name().to_string())
                .unwrap_or_else(|_| "Unknown Device".to_string());

            Some(AudioDeviceInfo {
                id: id_string,
                label,
            })
        })
        .collect()
}

fn get_current_device_label(current_id: &str, devices: &[AudioDeviceInfo]) -> String {
    if current_id == SYSTEM_DEFAULT {
        return SYSTEM_DEFAULT.to_string();
    }

    devices
        .iter()
        .find(|d| d.id == current_id)
        .map(|d| d.label.clone())
        .unwrap_or_else(|| SYSTEM_DEFAULT.to_string())
}

fn render_audio_device_dropdown(
    field: SettingField<Option<AudioDeviceName>>,
    file: SettingsUiFile,
    kind: AudioDeviceKind,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value): (_, Option<&Option<AudioDeviceName>>) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_device_id: String = current_value
        .and_then(|opt| opt.as_ref().map(|x| x.0.clone()))
        .unwrap_or_else(|| SYSTEM_DEFAULT.into());

    let devices = get_audio_devices(kind);
    let current_label = get_current_device_label(&current_device_id, &devices);

    let dropdown_id: SharedString = match kind {
        AudioDeviceKind::Input => "input-audio-device-dropdown".into(),
        AudioDeviceKind::Output => "output-audio-device-dropdown".into(),
    };

    let menu = ContextMenu::build(window, cx, {
        let current_device_id = current_device_id.clone();
        let file = file.clone();
        move |mut menu, _, _cx| {
            // Add "System Default" option first
            let is_system_default = current_device_id == SYSTEM_DEFAULT;
            menu = menu.toggleable_entry(
                SYSTEM_DEFAULT,
                is_system_default,
                IconPosition::Start,
                None,
                {
                    let file = file.clone();
                    move |window, cx| {
                        update_settings_file(
                            file.clone(),
                            field.json_path,
                            window,
                            cx,
                            move |settings, _cx| {
                                (field.write)(settings, Some(None));
                            },
                        )
                        .log_err();
                    }
                },
            );

            // Add all detected devices
            for device in &devices {
                let is_current = device.id == current_device_id;
                let device_id = device.id.clone();
                let file = file.clone();

                menu = menu.toggleable_entry(
                    device.label.clone(),
                    is_current,
                    IconPosition::Start,
                    None,
                    {
                        move |window, cx| {
                            let value: Option<Option<AudioDeviceName>> =
                                Some(Some(AudioDeviceName(device_id.clone())));
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
                    },
                );
            }
            menu
        }
    });

    DropdownMenu::new(dropdown_id, current_label, menu)
        .tab_index(0)
        .style(DropdownStyle::Outlined)
        .into_any_element()
}

pub fn render_input_audio_device_dropdown(
    field: SettingField<Option<AudioDeviceName>>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    render_audio_device_dropdown(field, file, AudioDeviceKind::Input, window, cx)
}

pub fn render_output_audio_device_dropdown(
    field: SettingField<Option<AudioDeviceName>>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    render_audio_device_dropdown(field, file, AudioDeviceKind::Output, window, cx)
}
