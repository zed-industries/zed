//! Device metadata and description types.
//!
//! This module provides structured information about audio devices including manufacturer,
//! device type, interface type, and connection details. Not all backends provide complete
//! information - availability depends on platform capabilities.

use std::fmt;

use crate::ChannelCount;

/// Describes an audio device with structured metadata.
///
/// This type provides structured information about an audio device beyond just its name.
/// Availability depends on the host implementation and platform capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceDescription {
    /// Human-readable device name
    name: String,

    /// Device manufacturer or vendor name
    manufacturer: Option<String>,

    /// Driver name
    driver: Option<String>,

    /// Categorization of device type
    device_type: DeviceType,

    /// Connection/interface type
    interface_type: InterfaceType,

    /// Direction: input, output, or duplex
    direction: DeviceDirection,

    /// Physical address or connection identifier
    address: Option<String>,

    /// Additional description lines with non-structured, detailed information.
    extended: Vec<String>,
}

/// Categorization of audio device types.
///
/// This describes the kind of audio device (speaker, microphone, headset, etc.)
/// regardless of how it connects to the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DeviceType {
    /// Speaker (built-in or external)
    Speaker,

    /// Microphone (built-in or external)
    Microphone,

    /// Headphones (audio output only)
    Headphones,

    /// Headset (combined headphones + microphone)
    Headset,

    /// Earpiece (phone-style speaker, typically for voice calls)
    Earpiece,

    /// Handset (telephone-style handset with speaker and microphone)
    Handset,

    /// Hearing aid device
    HearingAid,

    /// Docking station audio
    Dock,

    /// Radio/TV tuner
    Tuner,

    /// Virtual/loopback device (software audio routing)
    Virtual,

    /// Unknown or unclassified device type
    #[default]
    Unknown,
}

/// How the device connects to the system (interface/connection type).
///
/// This describes the physical or logical connection between the audio device
/// and the computer system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum InterfaceType {
    /// Built-in to the system (integrated audio chipset)
    BuiltIn,

    /// USB connection
    Usb,

    /// Bluetooth wireless connection
    Bluetooth,

    /// PCI or PCIe card (internal sound card)
    Pci,

    /// FireWire connection (IEEE 1394)
    FireWire,

    /// Thunderbolt connection
    Thunderbolt,

    /// HDMI connection
    Hdmi,

    /// Line-level analog connection (line in/out, aux)
    Line,

    /// S/PDIF digital audio interface
    Spdif,

    /// Network connection (Dante, AVB, AirPlay, IP audio, etc.)
    Network,

    /// Virtual/loopback connection (software audio routing, not physical hardware)
    Virtual,

    /// DisplayPort audio
    DisplayPort,

    /// Aggregate device (combines multiple devices)
    Aggregate,

    /// Unknown connection type
    #[default]
    Unknown,
}

/// The direction(s) that a device supports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DeviceDirection {
    /// Input only (capture/recording)
    Input,

    /// Output only (playback/rendering)
    Output,

    /// Both input and output
    Duplex,

    /// Direction unknown or not yet determined
    #[default]
    Unknown,
}

impl DeviceDescription {
    /// Returns the human-readable device name.
    ///
    /// This is always available and is the primary user-facing identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the manufacturer/vendor name if available.
    pub fn manufacturer(&self) -> Option<&str> {
        self.manufacturer.as_deref()
    }

    /// Returns the driver name if available.
    pub fn driver(&self) -> Option<&str> {
        self.driver.as_deref()
    }

    /// Returns the device type categorization.
    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }

    /// Returns the interface/connection type.
    pub fn interface_type(&self) -> InterfaceType {
        self.interface_type
    }

    /// Returns the device direction.
    pub fn direction(&self) -> DeviceDirection {
        self.direction
    }

    /// Returns whether this device supports audio input (capture).
    ///
    /// This is a convenience method that checks if direction is `Input` or `Duplex`.
    pub fn supports_input(&self) -> bool {
        matches!(
            self.direction,
            DeviceDirection::Input | DeviceDirection::Duplex
        )
    }

    /// Returns whether this device supports audio output (playback).
    ///
    /// This is a convenience method that checks if direction is `Output` or `Duplex`.
    pub fn supports_output(&self) -> bool {
        matches!(
            self.direction,
            DeviceDirection::Output | DeviceDirection::Duplex
        )
    }

    /// Returns the physical address or connection identifier if available.
    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    /// Returns additional description lines with detailed information.
    pub fn extended(&self) -> &[String] {
        &self.extended
    }
}

impl fmt::Display for DeviceDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(mfr) = &self.manufacturer {
            write!(f, " ({})", mfr)?;
        }

        if self.device_type != DeviceType::Unknown {
            write!(f, " [{}]", self.device_type)?;
        }

        if self.interface_type != InterfaceType::Unknown {
            write!(f, " via {}", self.interface_type)?;
        }

        Ok(())
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Speaker => write!(f, "Speaker"),
            DeviceType::Microphone => write!(f, "Microphone"),
            DeviceType::Headphones => write!(f, "Headphones"),
            DeviceType::Headset => write!(f, "Headset"),
            DeviceType::Earpiece => write!(f, "Earpiece"),
            DeviceType::Handset => write!(f, "Handset"),
            DeviceType::HearingAid => write!(f, "Hearing Aid"),
            DeviceType::Dock => write!(f, "Dock"),
            DeviceType::Tuner => write!(f, "Tuner"),
            DeviceType::Virtual => write!(f, "Virtual"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceType::BuiltIn => write!(f, "Built-in"),
            InterfaceType::Usb => write!(f, "USB"),
            InterfaceType::Bluetooth => write!(f, "Bluetooth"),
            InterfaceType::Pci => write!(f, "PCI"),
            InterfaceType::FireWire => write!(f, "FireWire"),
            InterfaceType::Thunderbolt => write!(f, "Thunderbolt"),
            InterfaceType::Hdmi => write!(f, "HDMI"),
            InterfaceType::Line => write!(f, "Line"),
            InterfaceType::Spdif => write!(f, "S/PDIF"),
            InterfaceType::Network => write!(f, "Network"),
            InterfaceType::Virtual => write!(f, "Virtual"),
            InterfaceType::DisplayPort => write!(f, "DisplayPort"),
            InterfaceType::Aggregate => write!(f, "Aggregate"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for DeviceDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceDirection::Input => write!(f, "Input"),
            DeviceDirection::Output => write!(f, "Output"),
            DeviceDirection::Duplex => write!(f, "Duplex"),
            _ => write!(f, "Unknown"),
        }
    }
}

/// Builder for constructing a `DeviceDescription`.
///
/// This is primarily used by host implementations and custom hosts
/// to gradually build up device descriptions with available metadata.
#[derive(Debug, Clone)]
pub struct DeviceDescriptionBuilder {
    name: String,
    manufacturer: Option<String>,
    driver: Option<String>,
    device_type: DeviceType,
    interface_type: InterfaceType,
    direction: DeviceDirection,
    address: Option<String>,
    extended: Vec<String>,
}

impl DeviceDescriptionBuilder {
    /// Creates a new builder with the device name (required).
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            manufacturer: None,
            driver: None,
            device_type: DeviceType::default(),
            interface_type: InterfaceType::default(),
            direction: DeviceDirection::default(),
            address: None,
            extended: Vec::new(),
        }
    }

    /// Sets the manufacturer name.
    pub fn manufacturer(mut self, manufacturer: impl Into<String>) -> Self {
        self.manufacturer = Some(manufacturer.into());
        self
    }

    /// Sets the driver name.
    pub fn driver(mut self, driver: impl Into<String>) -> Self {
        self.driver = Some(driver.into());
        self
    }

    /// Sets the device type.
    pub fn device_type(mut self, device_type: DeviceType) -> Self {
        self.device_type = device_type;
        self
    }

    /// Sets the interface type.
    pub fn interface_type(mut self, interface_type: InterfaceType) -> Self {
        self.interface_type = interface_type;
        self
    }

    /// Sets the device direction.
    pub fn direction(mut self, direction: DeviceDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the physical address.
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Sets the description lines.
    pub fn extended(mut self, lines: Vec<String>) -> Self {
        self.extended = lines;
        self
    }

    /// Adds a single description line.
    pub fn add_extended_line(mut self, line: impl Into<String>) -> Self {
        self.extended.push(line.into());
        self
    }

    /// Builds the [`DeviceDescription`].
    pub fn build(self) -> DeviceDescription {
        DeviceDescription {
            name: self.name,
            manufacturer: self.manufacturer,
            driver: self.driver,
            device_type: self.device_type,
            interface_type: self.interface_type,
            direction: self.direction,
            address: self.address,
            extended: self.extended,
        }
    }
}

/// Determines device direction from input/output capabilities.
pub(crate) fn direction_from_caps(has_input: bool, has_output: bool) -> DeviceDirection {
    match (has_input, has_output) {
        (true, true) => DeviceDirection::Duplex,
        (true, false) => DeviceDirection::Input,
        (false, true) => DeviceDirection::Output,
        (false, false) => DeviceDirection::Unknown,
    }
}

/// Determines device direction from input/output channel counts.
#[allow(dead_code)]
pub(crate) fn direction_from_counts(
    input_channels: Option<ChannelCount>,
    output_channels: Option<ChannelCount>,
) -> DeviceDirection {
    let has_input = input_channels.map(|n| n > 0).unwrap_or(false);
    let has_output = output_channels.map(|n| n > 0).unwrap_or(false);
    direction_from_caps(has_input, has_output)
}
