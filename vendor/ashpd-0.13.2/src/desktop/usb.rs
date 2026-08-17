// Copyright (C) 2024-2025 GNOME Foundation
//
// Authors:
//     Hubert Figuière <hub@figuiere.net>
//

//! Provide an interface to USB device. Allow enumerating devices
//! and requiring access to.
//! ```rust,no_run
//! use ashpd::desktop::usb::UsbProxy;
//! use futures_util::StreamExt;
//!
//! async fn watch_devices() -> ashpd::Result<()> {
//!     let usb = UsbProxy::new().await?;
//!     let session = usb.create_session(Default::default()).await?;
//!     if let Some(response) = usb.receive_device_events().await?.next().await {
//!         let events = response.events();
//!         for ev in events {
//!             println!(
//!                 "Received event: {:#?} for device {}",
//!                 ev.action(),
//!                 ev.device_id()
//!             );
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;

use futures_util::Stream;
use serde::{Deserialize, Serialize};
use zbus::zvariant::{
    ObjectPath, OwnedFd, OwnedObjectPath, OwnedValue, Type,
    as_value::{self, optional},
};

use crate::{
    Error, WindowIdentifier,
    desktop::{CreateSessionOptions, HandleToken, Session, SessionPortal},
    proxy::Proxy,
};

#[derive(Debug, Clone, Hash, Serialize, Deserialize, Type, PartialEq, Eq)]
#[zvariant(signature = "s")]
/// A device identifier.
pub struct DeviceID(String);

impl DeviceID {
    /// Gets the ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DeviceID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for DeviceID {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for DeviceID {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl std::fmt::Display for DeviceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`UsbProxy::enumerate_devices`] request.
pub struct EnumerateDevicesOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`UsbProxy::release_devices`] request.
pub struct ReleaseDevicesOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`UsbProxy::finish_acquire_devices`] request.
struct FinishAcquireDevicesOptions {}

/// USB device description
#[derive(Serialize, Deserialize, Clone, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct UsbDevice {
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    parent: Option<DeviceID>,
    /// Device can be opened for reading. Default is false.
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    readable: Option<bool>,
    /// Device can be opened for writing. Default is false.
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    writable: Option<bool>,
    /// The device node for the USB.
    #[serde(
        default,
        rename = "device-file",
        with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    device_file: Option<String>,
    /// Device properties
    #[serde(default, with = "as_value", skip_serializing_if = "HashMap::is_empty")]
    properties: HashMap<String, OwnedValue>,
}

impl UsbDevice {
    /// Device ID of the parent device
    pub fn parent(&self) -> Option<&DeviceID> {
        self.parent.as_ref()
    }

    /// Return if the device is readable.
    pub fn is_readable(&self) -> bool {
        self.readable.unwrap_or(false)
    }

    /// Return if the device is writable.
    pub fn is_writable(&self) -> bool {
        self.writable.unwrap_or(false)
    }

    /// Return the optional device file.
    pub fn device_file(&self) -> Option<&str> {
        self.device_file.as_deref()
    }

    /// Return the vendor string property for display.
    pub fn vendor(&self) -> Option<String> {
        self.properties
            .get("ID_VENDOR_FROM_DATABASE")
            .or_else(|| self.properties.get("ID_VENDOR_ENC"))
            .and_then(|v| v.downcast_ref::<String>().ok())
    }

    /// Return the model string property for display.
    pub fn model(&self) -> Option<String> {
        self.properties
            .get("ID_MODEL_FROM_DATABASE")
            .or_else(|| self.properties.get("ID_MODEL_ENC"))
            .and_then(|v| v.downcast_ref::<String>().ok())
            .map(|model| {
                model
                    .replace("\\x20", " ") // Unescape the literal \x20 to space
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
    }

    /// Return the device properties.
    pub fn properties(&self) -> &HashMap<String, OwnedValue> {
        &self.properties
    }
}

/// USB error for acquiring device.
#[derive(Debug)]
pub struct UsbError(pub Option<String>);

impl std::fmt::Display for UsbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_deref().unwrap_or(""))
    }
}

impl std::error::Error for UsbError {}

impl From<AcquiredDevice> for Result<OwnedFd, UsbError> {
    fn from(v: AcquiredDevice) -> Result<OwnedFd, UsbError> {
        if let Some(fd) = v.fd {
            if v.success {
                Ok(fd)
            } else {
                Err(UsbError(v.error))
            }
        } else {
            Err(UsbError(None))
        }
    }
}

/// Device to acquire.
#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
struct AcquireDevice {
    #[serde(with = "as_value")]
    writable: bool,
}

/// Device to acquire. Tuple with the device ID and whether the
/// requested permission should be to write.
pub struct Device(DeviceID, bool /* writable */);

impl Device {
    /// Create a new `Device`.
    pub fn new(id: DeviceID, writable: bool) -> Self {
        Self(id, writable)
    }

    /// Get the device ID.
    pub fn id(&self) -> &DeviceID {
        &self.0
    }

    /// Return whether the device is writable.
    pub fn is_writable(&self) -> bool {
        self.1
    }
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`UsbProxy::acquire_devices`] request.
pub struct AcquireDevicesOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
}

/// Finished device acquired.
#[derive(Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
struct AcquiredDevice {
    #[serde(with = "as_value")]
    success: bool,
    #[serde(default, with = "optional")]
    fd: Option<OwnedFd>,
    #[serde(default, with = "optional")]
    error: Option<String>,
}

#[derive(Debug, Deserialize, Type)]
/// A USB event received part of the `device_event` signal response.
/// An event is composed of an `action`, a device `id` and the
/// [`UsbDevice`] description.
pub struct UsbEvent(UsbEventAction, DeviceID, UsbDevice);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// A USB event action.
pub enum UsbEventAction {
    /// Add.
    Add,
    /// Change.
    Change,
    /// Remove.
    Remove,
}

impl UsbEvent {
    /// The action.
    pub fn action(&self) -> UsbEventAction {
        self.0
    }

    /// The device ID string.
    pub fn device_id(&self) -> &DeviceID {
        &self.1
    }

    /// The [`UsbDevice`] properties.
    pub fn device(&self) -> &UsbDevice {
        &self.2
    }
}

#[derive(Debug, Deserialize, Type)]
/// A response received when the `device_event` signal is received.
pub struct UsbDeviceEvent(OwnedObjectPath, Vec<UsbEvent>);

impl UsbDeviceEvent {
    /// The session that triggered the state change
    pub fn session_handle(&self) -> ObjectPath<'_> {
        self.0.as_ref()
    }

    /// Events received
    pub fn events(&self) -> &[UsbEvent] {
        &self.1
    }
}

/// This interface provides access to USB devices.
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Usb")]
pub struct UsbProxy(Proxy<'static>);

impl UsbProxy {
    /// Create a new instance of [`UsbProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Usb").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`UsbProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Usb").await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Create a USB session.
    ///
    /// While this session is active, the caller will receive
    /// `DeviceEvents` signals with device addition and removal.
    ///
    /// # Specifications
    ///
    /// See also [`CreateSession`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Usb.html#org-freedesktop-portal-usb-createsession).
    #[doc(alias = "CreateSession")]
    pub async fn create_session(
        &self,
        options: CreateSessionOptions,
    ) -> Result<Session<Self>, Error> {
        let session: OwnedObjectPath = self.0.call("CreateSession", &(&options)).await?;
        Session::with_connection(self.0.connection().clone(), session).await
    }

    /// Enumerate USB devices.
    ///
    /// Return a vector of tuples with the device ID and the [`UsbDevice`]
    ///
    /// # Specifications
    ///
    /// See also [`EnumerateDevices`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Usb.html#org-freedesktop-portal-usb-enumeratedevices).
    #[doc(alias = "EnumerateDevices")]
    pub async fn enumerate_devices(
        &self,
        options: EnumerateDevicesOptions,
    ) -> Result<Vec<(DeviceID, UsbDevice)>, Error> {
        self.0.call("EnumerateDevices", &(&options)).await
    }

    /// Acquire devices
    ///
    /// The portal will perform the permission request. In case of
    /// success, ie permission is granted, it returns a vector of
    /// tuples containing the device ID and the file descriptor or
    /// error.
    ///
    /// # Specifications
    ///
    /// See also [`AcquireDevices`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Usb.html#org-freedesktop-portal-usb-acquiredevices).
    #[doc(alias = "AcquireDevices")]
    pub async fn acquire_devices(
        &self,
        parent_window: Option<&WindowIdentifier>,
        devices: &[Device],
        options: AcquireDevicesOptions,
    ) -> Result<Vec<(DeviceID, Result<OwnedFd, UsbError>)>, Error> {
        let parent_window = parent_window.map(|i| i.to_string()).unwrap_or_default();
        let acquire_devices: Vec<(DeviceID, AcquireDevice)> = devices
            .iter()
            .map(|dev| {
                let device = AcquireDevice { writable: dev.1 };
                (dev.id().clone(), device)
            })
            .collect();
        let request = self
            .0
            .empty_request(
                &options.handle_token,
                "AcquireDevices",
                &(&parent_window, &acquire_devices, &options),
            )
            .await?;
        let mut devices: Vec<(DeviceID, Result<OwnedFd, UsbError>)> = vec![];
        if request.response().is_ok() {
            let path = request.path();
            loop {
                let (mut new_devices, finished) = self
                    .finish_acquire_devices(path, Default::default())
                    .await?;
                devices.append(&mut new_devices);
                if finished {
                    break;
                }
            }
        }
        Ok(devices)
    }

    /// Call on success of acquire_devices. This never need to be called
    /// by client applications.
    ///
    /// # Specifications
    ///
    /// See also [`FinishAcquireDevices`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Usb.html#org-freedesktop-portal-usb-finishacquiredevices).
    #[doc(alias = "FinishAcquireDevices")]
    async fn finish_acquire_devices(
        &self,
        request_path: &ObjectPath<'_>,
        options: FinishAcquireDevicesOptions,
    ) -> Result<(Vec<(DeviceID, Result<OwnedFd, UsbError>)>, bool), Error> {
        self.0
            .call("FinishAcquireDevices", &(request_path, &options))
            .await
            .map(|result: (Vec<(DeviceID, AcquiredDevice)>, bool)| {
                let finished = result.1;
                (
                    result
                        .0
                        .into_iter()
                        .map(|item| (item.0, item.1.into()))
                        .collect::<Vec<_>>(),
                    finished,
                )
            })
    }

    /// Release devices
    ///
    /// Release all the devices whose ID is specified in `devices`.
    ///
    /// # Specifications
    ///
    /// See also [`ReleaseDevices`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Usb.html#org-freedesktop-portal-usb-releasedevices).
    #[doc(alias = "ReleaseDevices")]
    pub async fn release_devices(
        &self,
        devices: &[&DeviceID],
        options: ReleaseDevicesOptions,
    ) -> Result<(), Error> {
        self.0.call("ReleaseDevices", &(devices, &options)).await
    }

    /// Signal emitted on a device event
    ///
    /// Will emit [`UsbDeviceEvent`].
    ///
    /// # Specifications
    ///
    /// See also [`DeviceEvents`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Usb.html#org-freedesktop-portal-usb-deviceevents).
    #[doc(alias = "DeviceEvents")]
    pub async fn receive_device_events(
        &self,
    ) -> Result<impl Stream<Item = UsbDeviceEvent> + use<'_>, Error> {
        self.0.signal("DeviceEvents").await
    }
}

impl crate::Sealed for UsbProxy {}
impl SessionPortal for UsbProxy {}
