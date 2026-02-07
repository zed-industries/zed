use cpal::{
    DeviceDescription, default_host,
    traits::{DeviceTrait, HostTrait},
};
use gpui::{AnyElement, App, ElementId, ReadGlobal, SharedString, Window};
use settings::{AudioInputDeviceName, AudioOutputDeviceName, SettingsStore};
use ui::{ContextMenu, DropdownMenu, DropdownStyle, IconPosition, IntoElement};
use util::ResultExt;

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile, update_settings_file};

/// Trait for audio device name types to enable shared rendering logic
pub(crate) trait AudioDeviceName: Clone + Send + 'static {
    fn device_id(&self) -> &str;
    fn from_device_id(id: String) -> Self;
}

impl AudioDeviceName for AudioInputDeviceName {
    fn device_id(&self) -> &str {
        &self.0
    }
    fn from_device_id(id: String) -> Self {
        Self(id)
    }
}

impl AudioDeviceName for AudioOutputDeviceName {
    fn device_id(&self) -> &str {
        &self.0
    }
    fn from_device_id(id: String) -> Self {
        Self(id)
    }
}

pub(crate) const SYSTEM_DEFAULT: &str = "System Default";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum AudioDeviceKind {
    Input,
    Output,
}

pub(crate) struct AudioDeviceInfo {
    pub id: String,
    pub label: String,
}

pub(crate) fn should_show_device(
    device_id: &str,
    description: &DeviceDescription,
    kind: AudioDeviceKind,
) -> bool {
    let is_valid_kind = match kind {
        AudioDeviceKind::Input => description.supports_input(),
        AudioDeviceKind::Output => description.supports_output(),
    };

    if !is_valid_kind {
        return false;
    }

    // On non-Linux platforms, show all devices as they're typically already user-friendly
    #[cfg(not(target_os = "linux"))]
    {
        let _ = device_id;
        let _ = description;
        let _ = kind;
        return true;
    }

    // On Linux/ALSA, filter to show only user-friendly devices
    #[cfg(target_os = "linux")]
    {
        // Always show pipewire and default
        if device_id == "alsa:default" || device_id == "alsa:pipewire" {
            return true;
        }

        match kind {
            // For input devices, sysdefault works well
            AudioDeviceKind::Input => {
                if device_id.starts_with("alsa:sysdefault:") {
                    return true;
                }
            }
            // For output devices, plughw handles format conversion reliably
            AudioDeviceKind::Output => {
                if device_id.starts_with("alsa:plughw:") {
                    return true;
                }
            }
        }

        // Filter out everything else:
        // - null (discard samples)
        // - hw: (raw hardware access without format conversion)
        // - front:, surround*: (speaker configuration variants)
        // - hdmi: (usually duplicated)
        // - iec958: (S/PDIF digital)
        false
    }
}

pub(crate) fn get_audio_devices(kind: AudioDeviceKind) -> Vec<AudioDeviceInfo> {
    let Some(devices) = default_host().devices().ok() else {
        return Vec::new();
    };

    devices
        .filter_map(|device| {
            let id = device.id().ok()?;
            let id_string = id.to_string();
            let desc = device.description().ok()?;

            if !should_show_device(&id_string, &desc, kind) {
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

pub(crate) fn get_current_device_label(current_id: &str, devices: &[AudioDeviceInfo]) -> String {
    if current_id == SYSTEM_DEFAULT {
        return SYSTEM_DEFAULT.to_string();
    }

    devices
        .iter()
        .find(|d| d.id == current_id)
        .map(|d| format!("{} ({})", d.label, d.id))
        // .map(|d| d.label.clone())
        .unwrap_or_else(|| SYSTEM_DEFAULT.to_string())
}

/// Renders an audio device dropdown with a callback for handling device selection.
///
/// The `on_select` callback receives the selected device ID (or None for the system default option).
pub(crate) fn render_audio_device_dropdown<F>(
    dropdown_id: impl Into<ElementId>,
    kind: AudioDeviceKind,
    current_device_id: String,
    on_select: F,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    F: Fn(Option<String>, &mut Window, &mut App) + Clone + 'static,
{
    let devices = get_audio_devices(kind);
    let current_label = get_current_device_label(&current_device_id, &devices);

    let menu = ContextMenu::build(window, cx, {
        let current_device_id = current_device_id.clone();
        move |mut menu, _, _cx| {
            let is_system_default = current_device_id == SYSTEM_DEFAULT;
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
                let is_current = device.id == current_device_id;
                let device_id = device.id.clone();

                menu = menu.toggleable_entry(
                    format!("{} ({})", device.label, device.id),
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

    DropdownMenu::new(dropdown_id, current_label, menu)
        .style(DropdownStyle::Outlined)
        .full_width(true)
        .into_any_element()
}

fn render_settings_audio_device_dropdown<T: AudioDeviceName>(
    field: SettingField<Option<T>>,
    file: SettingsUiFile,
    kind: AudioDeviceKind,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value): (_, Option<&Option<T>>) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_device_id: String = current_value
        .and_then(|opt| opt.as_ref().map(|x| x.device_id().to_string()))
        .unwrap_or_else(|| SYSTEM_DEFAULT.into());

    let dropdown_id: SharedString = match kind {
        AudioDeviceKind::Input => "input-audio-device-dropdown".into(),
        AudioDeviceKind::Output => "output-audio-device-dropdown".into(),
    };

    render_audio_device_dropdown(
        dropdown_id,
        kind,
        current_device_id,
        move |device_id, window, cx| {
            let value: Option<Option<T>> = device_id.map(|id| Some(T::from_device_id(id)));
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
        },
        window,
        cx,
    )
}

pub fn render_input_audio_device_dropdown(
    field: SettingField<Option<AudioInputDeviceName>>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    render_settings_audio_device_dropdown(field, file, AudioDeviceKind::Input, window, cx)
}

pub fn render_output_audio_device_dropdown(
    field: SettingField<Option<AudioOutputDeviceName>>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    render_settings_audio_device_dropdown(field, file, AudioDeviceKind::Output, window, cx)
}
