use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use super::{alsa, Device};
use crate::{BackendSpecificError, DeviceDirection, DevicesError};

const HW_PREFIX: &str = "hw";
const PLUGHW_PREFIX: &str = "plughw";

/// Information about a physical device
struct PhysicalDevice {
    card_index: u32,
    card_name: Option<String>,
    device_index: u32,
    device_name: Option<String>,
    direction: DeviceDirection,
}

/// Iterator over available ALSA PCM devices (physical hardware and virtual/plugin devices).
pub type Devices = std::vec::IntoIter<Device>;

/// Enumerates all available ALSA PCM devices (physical hardware and virtual/plugin devices).
///
/// We enumerate both ALSA hints and physical devices because:
/// - Hints provide virtual devices, user configurations, and card-specific devices with metadata
/// - Physical probing provides traditional numeric naming (hw:CARD=0,DEV=0) for compatibility
pub fn devices() -> Result<Devices, DevicesError> {
    let mut devices = Vec::new();
    let mut seen_pcm_ids = HashSet::new();

    let physical_devices = physical_devices();

    // Add all hint devices, including virtual devices
    if let Ok(hints) = alsa::device_name::HintIter::new_str(None, "pcm") {
        for hint in hints {
            if let Ok(device) = Device::try_from(hint) {
                seen_pcm_ids.insert(device.pcm_id.clone());
                devices.push(device);
            }
        }
    }

    // Add hw:/plughw: for all physical devices with numeric index (traditional naming)
    for phys_dev in physical_devices {
        for prefix in [HW_PREFIX, PLUGHW_PREFIX] {
            let pcm_id = format!(
                "{}:CARD={},DEV={}",
                prefix, phys_dev.card_index, phys_dev.device_index
            );

            if seen_pcm_ids.insert(pcm_id.clone()) {
                devices.push(Device {
                    pcm_id,
                    desc: Some(format_device_description(&phys_dev, prefix)),
                    direction: phys_dev.direction,
                    handles: Arc::new(Mutex::new(Default::default())),
                });
            }
        }
    }

    Ok(devices.into_iter())
}

/// Formats device description in ALSA style: "Card Name, Device Name\nPurpose"
fn format_device_description(phys_dev: &PhysicalDevice, prefix: &str) -> String {
    // "Card Name, Device Name" or variations
    let first_line = match (&phys_dev.card_name, &phys_dev.device_name) {
        (Some(card), Some(device)) => format!("{}, {}", card, device),
        (Some(card), None) => card.clone(),
        (None, Some(device)) => device.clone(),
        (None, None) => format!("Card {}", phys_dev.card_index),
    };

    // ALSA standard description
    let second_line = match prefix {
        HW_PREFIX => "Direct hardware device without any conversions",
        PLUGHW_PREFIX => "Hardware device with all software conversions",
        _ => "",
    };

    format!("{}\n{}", first_line, second_line)
}

fn physical_devices() -> Vec<PhysicalDevice> {
    let mut devices = Vec::new();
    for card in alsa::card::Iter::new().filter_map(Result::ok) {
        let card_index = card.get_index() as u32;
        let ctl = match alsa::Ctl::new(&format!("{}:{}", HW_PREFIX, card_index), false) {
            Ok(ctl) => ctl,
            Err(_) => continue,
        };
        let card_name = ctl
            .card_info()
            .ok()
            .and_then(|info| info.get_name().ok().map(|s| s.to_string()));

        for device_index in alsa::ctl::DeviceIter::new(&ctl) {
            let device_index = device_index as u32;
            let playback_info = ctl
                .pcm_info(device_index, 0, alsa::Direction::Playback)
                .ok();
            let capture_info = ctl.pcm_info(device_index, 0, alsa::Direction::Capture).ok();

            let (direction, device_name) = match (&playback_info, &capture_info) {
                (Some(p_info), Some(_c_info)) => (
                    DeviceDirection::Duplex,
                    p_info.get_name().ok().map(|s| s.to_string()),
                ),
                (Some(p_info), None) => (
                    DeviceDirection::Output,
                    p_info.get_name().ok().map(|s| s.to_string()),
                ),
                (None, Some(c_info)) => (
                    DeviceDirection::Input,
                    c_info.get_name().ok().map(|s| s.to_string()),
                ),
                (None, None) => {
                    // Device doesn't exist - skip
                    continue;
                }
            };

            let device_name = device_name.unwrap_or_else(|| format!("Device {}", device_index));
            devices.push(PhysicalDevice {
                card_index,
                card_name: card_name.clone(),
                device_index,
                device_name: Some(device_name),
                direction,
            });
        }
    }

    devices
}

impl From<alsa::Error> for DevicesError {
    fn from(err: alsa::Error) -> Self {
        let err: BackendSpecificError = err.into();
        err.into()
    }
}

impl TryFrom<alsa::device_name::Hint> for Device {
    type Error = BackendSpecificError;

    fn try_from(hint: alsa::device_name::Hint) -> Result<Self, Self::Error> {
        let pcm_id = hint.name.ok_or_else(|| Self::Error {
            description: "ALSA hint missing PCM ID".to_string(),
        })?;

        // Per ALSA docs (https://alsa-project.org/alsa-doc/alsa-lib/group___hint.html),
        // NULL IOID means both Input/Output. Whether a stream can actually open in a given
        // direction can only be determined by attempting to open it.
        let direction = hint.direction.map_or(DeviceDirection::Duplex, Into::into);

        Ok(Self {
            pcm_id: pcm_id.to_owned(),
            desc: hint.desc,
            direction,
            handles: Arc::new(Mutex::new(Default::default())),
        })
    }
}

impl From<alsa::Direction> for DeviceDirection {
    fn from(direction: alsa::Direction) -> Self {
        match direction {
            alsa::Direction::Playback => DeviceDirection::Output,
            alsa::Direction::Capture => DeviceDirection::Input,
        }
    }
}
