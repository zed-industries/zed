//! # Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::{
//!     PersistMode,
//!     remote_desktop::{DeviceType, KeyState, RemoteDesktop, SelectDevicesOptions},
//! };
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = RemoteDesktop::new().await?;
//!     let session = proxy.create_session(Default::default()).await?;
//!     proxy
//!         .select_devices(
//!             &session,
//!             SelectDevicesOptions::default()
//!                 .set_devices(DeviceType::Keyboard | DeviceType::Pointer),
//!         )
//!         .await?;
//!
//!     let response = proxy
//!         .start(&session, None, Default::default())
//!         .await?
//!         .response()?;
//!     println!("{:#?}", response.devices());
//!
//!     // 13 for Enter key code
//!     proxy
//!         .notify_keyboard_keycode(&session, 13, KeyState::Pressed, Default::default())
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! You can also use the Remote Desktop portal with the ScreenCast one. In order
//! to do so, you need to call
//! [`Screencast::select_sources()`][select_sources]
//! on the session created with
//! [`RemoteDesktop::create_session()`][create_session]
//!
//! ```rust,no_run
//! use ashpd::desktop::{
//!     PersistMode,
//!     remote_desktop::{DeviceType, KeyState, RemoteDesktop, SelectDevicesOptions},
//!     screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType},
//! };
//!
//! async fn run() -> ashpd::Result<()> {
//!     let remote_desktop = RemoteDesktop::new().await?;
//!     let screencast = Screencast::new().await?;
//!     let session = remote_desktop.create_session(Default::default()).await?;
//!
//!     remote_desktop
//!         .select_devices(
//!             &session,
//!             SelectDevicesOptions::default()
//!                 .set_devices(DeviceType::Keyboard | DeviceType::Pointer),
//!         )
//!         .await?;
//!     screencast
//!         .select_sources(
//!             &session,
//!             SelectSourcesOptions::default()
//!                 .set_cursor_mode(CursorMode::Metadata)
//!                 .set_sources(SourceType::Monitor | SourceType::Window)
//!                 .set_multiple(true)
//!                 .set_persist_mode(PersistMode::DoNot),
//!         )
//!         .await?;
//!
//!     let response = remote_desktop
//!         .start(&session, None, Default::default())
//!         .await?
//!         .response()?;
//!     println!("{:#?}", response.devices());
//!     println!("{:#?}", response.streams());
//!
//!     // 13 for Enter key code
//!     remote_desktop
//!         .notify_keyboard_keycode(&session, 13, KeyState::Pressed, Default::default())
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//! [select_sources]: crate::desktop::screencast::Screencast::select_sources
//! [create_session]: crate::desktop::remote_desktop::RemoteDesktop::create_session

use std::os::fd::OwnedFd;

use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{
    self, Optional, Type,
    as_value::{self, optional},
};

use super::{
    HandleToken, PersistMode, Request, Session, screencast::Stream, session::SessionPortal,
};
use crate::{
    Error, WindowIdentifier,
    desktop::session::{CreateSessionOptions, CreateSessionResponse},
    proxy::Proxy,
};

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdKeyState"))]
#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, PartialEq, Eq, Debug, Type)]
#[doc(alias = "XdpKeyState")]
/// The keyboard key state.
#[repr(u32)]
pub enum KeyState {
    #[doc(alias = "XDP_KEY_PRESSED")]
    /// The key is pressed.
    Pressed = 1,
    #[doc(alias = "XDP_KEY_RELEASED")]
    /// The key is released..
    Released = 0,
}

#[bitflags]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Clone, Copy, Type)]
#[repr(u32)]
#[doc(alias = "XdpDeviceType")]
/// A bit flag for the available devices.
pub enum DeviceType {
    #[doc(alias = "XDP_DEVICE_KEYBOARD")]
    /// A keyboard.
    Keyboard,
    #[doc(alias = "XDP_DEVICE_POINTER")]
    /// A mouse pointer.
    Pointer,
    #[doc(alias = "XDP_DEVICE_TOUCHSCREEN")]
    /// A touchscreen
    Touchscreen,
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdAxis"))]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Clone, Copy, Type)]
#[doc(alias = "XdpDiscreteAxis")]
#[repr(u32)]
/// The available axis.
pub enum Axis {
    #[doc(alias = "XDP_AXIS_VERTICAL_SCROLL")]
    /// Vertical axis.
    Vertical = 0,
    #[doc(alias = "XDP_AXIS_HORIZONTAL_SCROLL")]
    /// Horizontal axis.
    Horizontal = 1,
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_keyboard_keycode`] request.
pub struct NotifyKeyboardKeycodeOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_keyboard_keysym`] request.
pub struct NotifyKeyboardKeysymOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_pointer_axis_discrete`]
/// request.
pub struct NotifyPointerAxisDiscreteOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_touch_up`] request.
pub struct NotifyTouchUpOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_touch_down`] request.
pub struct NotifyTouchDownOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_touch_motion`] request.
pub struct NotifyTouchMotionOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_pointer_button`] request.
pub struct NotifyPointerButtonOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_pointer_motion`] request.
pub struct NotifyPointerMotionOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_pointer_motion_absolute`]
/// request.
pub struct NotifyPointerMotionAbsoluteOptions {}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::notify_pointer_axis`] request.
pub struct NotifyPointerAxisOptions {
    #[serde(with = "as_value")]
    finish: bool,
}

impl NotifyPointerAxisOptions {
    /// Sets whether the axis event is the last one in a sequence.
    pub fn set_finish(mut self, finish: bool) -> Self {
        self.finish = finish;
        self
    }
}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`RemoteDesktop::select_devices`] request.
#[zvariant(signature = "dict")]
pub struct SelectDevicesOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    types: Option<BitFlags<DeviceType>>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    restore_token: Option<String>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    persist_mode: Option<PersistMode>,
}

impl SelectDevicesOptions {
    /// Sets the device types to request remote controlling of.
    pub fn set_devices(mut self, types: impl Into<Option<BitFlags<DeviceType>>>) -> Self {
        self.types = types.into();
        self
    }

    /// Sets the persist mode.
    pub fn set_persist_mode(mut self, persist_mode: impl Into<Option<PersistMode>>) -> Self {
        self.persist_mode = persist_mode.into();
        self
    }

    /// Sets the restore token.
    pub fn set_restore_token<'a>(mut self, token: impl Into<Option<&'a str>>) -> Self {
        self.restore_token = token.into().map(ToOwned::to_owned);
        self
    }
}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`RemoteDesktop::start`] request.
#[zvariant(signature = "dict")]
pub struct StartOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
}

#[derive(Deserialize, Type, Debug, Default)]
/// A response to a [`RemoteDesktop::select_devices`] request.
#[zvariant(signature = "dict")]
pub struct SelectedDevices {
    #[serde(default, with = "as_value")]
    devices: BitFlags<DeviceType>,
    #[serde(default, with = "as_value")]
    streams: Vec<Stream>,
    #[serde(default, with = "optional")]
    restore_token: Option<String>,
    #[serde(default, with = "optional")]
    clipboard_enabled: Option<bool>,
}

impl SelectedDevices {
    /// The selected devices.
    pub fn devices(&self) -> BitFlags<DeviceType> {
        self.devices
    }

    /// The selected streams if a ScreenCast portal is used on the same session
    pub fn streams(&self) -> &[Stream] {
        &self.streams
    }

    /// The session restore token.
    pub fn restore_token(&self) -> Option<&str> {
        self.restore_token.as_deref()
    }

    /// Whether the clipboard was enabled.
    pub fn is_clipboard_enabled(&self) -> bool {
        self.clipboard_enabled.unwrap_or(false)
    }
}

#[derive(Default, Debug, Serialize, Type)]
#[zvariant(signature = "dict")]
/// Specified options for a [`RemoteDesktop::connect_to_eis`] request.
pub struct ConnectToEISOptions {}

/// The interface lets sandboxed applications create remote desktop sessions.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.RemoteDesktop`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.RemoteDesktop")]
pub struct RemoteDesktop(Proxy<'static>);

impl RemoteDesktop {
    /// Create a new instance of [`RemoteDesktop`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.RemoteDesktop").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`RemoteDesktop`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.RemoteDesktop")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Create a remote desktop session.
    /// A remote desktop session is used to allow remote controlling a desktop
    /// session. It can also be used together with a screen cast session.
    ///
    /// # Specifications
    ///
    /// See also [`CreateSession`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-createsession).
    #[doc(alias = "CreateSession")]
    #[doc(alias = "xdp_portal_create_remote_desktop_session")]
    pub async fn create_session(
        &self,
        options: CreateSessionOptions,
    ) -> Result<Session<Self>, Error> {
        let (request, proxy) = futures_util::try_join!(
            self.0.request::<CreateSessionResponse>(
                &options.handle_token,
                "CreateSession",
                &options
            ),
            Session::from_unique_name(self.0.connection().clone(), &options.session_handle_token)
        )?;
        assert_eq!(proxy.path(), &request.response()?.session_handle.as_ref());
        Ok(proxy)
    }

    /// Select input devices to remote control.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `types` - The device types to request remote controlling of.
    ///
    /// # Specifications
    ///
    /// See also [`SelectDevices`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-selectdevices).
    #[doc(alias = "SelectDevices")]
    pub async fn select_devices(
        &self,
        session: &Session<Self>,
        options: SelectDevicesOptions,
    ) -> Result<Request<()>, Error> {
        self.0
            .empty_request(&options.handle_token, "SelectDevices", &(session, &options))
            .await
    }

    ///  Start the remote desktop session.
    ///
    /// This will typically result in the portal presenting a dialog letting
    /// the user select what to share, including devices and optionally screen
    /// content if screen cast sources was selected.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `identifier` - The application window identifier.
    ///
    /// # Specifications
    ///
    /// See also [`Start`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-start).
    #[doc(alias = "Start")]
    pub async fn start(
        &self,
        session: &Session<Self>,
        identifier: Option<&WindowIdentifier>,
        options: StartOptions,
    ) -> Result<Request<SelectedDevices>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "Start",
                &(session, identifier, &options),
            )
            .await
    }

    /// Notify keyboard code.
    ///
    /// **Note** only works if [`DeviceType::Keyboard`] access was provided
    /// after starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `keycode` - Keyboard code that was pressed or released.
    /// * `state` - The new state of the keyboard code.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyKeyboardKeycode`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifykeyboardkeycode).
    #[doc(alias = "NotifyKeyboardKeycode")]
    pub async fn notify_keyboard_keycode(
        &self,
        session: &Session<Self>,
        keycode: i32,
        state: KeyState,
        options: NotifyKeyboardKeycodeOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyKeyboardKeycode", &(session, options, keycode, state))
            .await
    }

    /// Notify keyboard symbol.
    ///
    /// **Note** only works if [`DeviceType::Keyboard`] access was provided
    /// after starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `keysym` - Keyboard symbol that was pressed or released.
    /// * `state` - The new state of the keyboard code.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyKeyboardKeysym`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifykeyboardkeysym).
    #[doc(alias = "NotifyKeyboardKeysym")]
    pub async fn notify_keyboard_keysym(
        &self,
        session: &Session<Self>,
        keysym: i32,
        state: KeyState,
        options: NotifyKeyboardKeysymOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyKeyboardKeysym", &(session, options, keysym, state))
            .await
    }

    /// Notify about a new touch up event.
    ///
    /// **Note** only works if [`DeviceType::Touchscreen`] access was provided
    /// after starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `slot` - Touch slot where touch point appeared.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyTouchUp`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifytouchup).
    #[doc(alias = "NotifyTouchUp")]
    pub async fn notify_touch_up(
        &self,
        session: &Session<Self>,
        slot: u32,
        options: NotifyTouchUpOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyTouchUp", &(session, options, slot))
            .await
    }

    /// Notify about a new touch down event.
    /// The (x, y) position represents the new touch point position in the
    /// streams logical coordinate space.
    ///
    /// **Note** only works if [`DeviceType::Touchscreen`] access was provided
    /// after starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `stream` - The PipeWire stream node the coordinate is relative to.
    /// * `slot` - Touch slot where touch point appeared.
    /// * `x` - Touch down x coordinate.
    /// * `y` - Touch down y coordinate.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyTouchDown`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifytouchdown).
    #[doc(alias = "NotifyTouchDown")]
    pub async fn notify_touch_down(
        &self,
        session: &Session<Self>,
        stream: u32,
        slot: u32,
        x: f64,
        y: f64,
        options: NotifyTouchDownOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyTouchDown", &(session, options, stream, slot, x, y))
            .await
    }

    /// Notify about a new touch motion event.
    /// The (x, y) position represents where the touch point position in the
    /// streams logical coordinate space moved.
    ///
    /// **Note** only works if [`DeviceType::Touchscreen`] access was provided
    /// after starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `stream` - The PipeWire stream node the coordinate is relative to.
    /// * `slot` - Touch slot where touch point appeared.
    /// * `x` - Touch motion x coordinate.
    /// * `y` - Touch motion y coordinate.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyTouchMotion`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifytouchmotion).
    #[doc(alias = "NotifyTouchMotion")]
    pub async fn notify_touch_motion(
        &self,
        session: &Session<Self>,
        stream: u32,
        slot: u32,
        x: f64,
        y: f64,
        options: NotifyTouchMotionOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyTouchMotion", &(session, options, stream, slot, x, y))
            .await
    }

    /// Notify about a new absolute pointer motion event.
    /// The (x, y) position represents the new pointer position in the streams
    /// logical coordinate space.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `stream` - The PipeWire stream node the coordinate is relative to.
    /// * `x` - Pointer motion x coordinate.
    /// * `y` - Pointer motion y coordinate.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyPointerMotionAbsolute`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifypointermotionabsolute).
    #[doc(alias = "NotifyPointerMotionAbsolute")]
    pub async fn notify_pointer_motion_absolute(
        &self,
        session: &Session<Self>,
        stream: u32,
        x: f64,
        y: f64,
        options: NotifyPointerMotionAbsoluteOptions,
    ) -> Result<(), Error> {
        self.0
            .call(
                "NotifyPointerMotionAbsolute",
                &(session, options, stream, x, y),
            )
            .await
    }

    /// Notify about a new relative pointer motion event.
    /// The (dx, dy) vector represents the new pointer position in the streams
    /// logical coordinate space.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `dx` - Relative movement on the x axis.
    /// * `dy` - Relative movement on the y axis.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyPointerMotion`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifypointermotion).
    #[doc(alias = "NotifyPointerMotion")]
    pub async fn notify_pointer_motion(
        &self,
        session: &Session<Self>,
        dx: f64,
        dy: f64,
        options: NotifyPointerMotionOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyPointerMotion", &(session, options, dx, dy))
            .await
    }

    /// Notify pointer button.
    /// The pointer button is encoded according to Linux Evdev button codes.
    ///
    ///
    /// **Note** only works if [`DeviceType::Pointer`] access was provided after
    /// starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `button` - The pointer button was pressed or released.
    /// * `state` - The new state of the keyboard code.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyPointerButton`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifypointerbutton).
    #[doc(alias = "NotifyPointerButton")]
    pub async fn notify_pointer_button(
        &self,
        session: &Session<Self>,
        button: i32,
        state: KeyState,
        options: NotifyPointerButtonOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyPointerButton", &(session, options, button, state))
            .await
    }

    /// Notify pointer axis discrete.
    ///
    /// **Note** only works if [`DeviceType::Pointer`] access was provided after
    /// starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `axis` - The axis that was scrolled.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyPointerAxisDiscrete`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifypointeraxisdiscrete).
    #[doc(alias = "NotifyPointerAxisDiscrete")]
    pub async fn notify_pointer_axis_discrete(
        &self,
        session: &Session<Self>,
        axis: Axis,
        steps: i32,
        options: NotifyPointerAxisDiscreteOptions,
    ) -> Result<(), Error> {
        self.0
            .call(
                "NotifyPointerAxisDiscrete",
                &(session, options, axis, steps),
            )
            .await
    }

    /// Notify pointer axis.
    /// The axis movement from a "smooth scroll" device, such as a touchpad.
    /// When applicable, the size of the motion delta should be equivalent to
    /// the motion vector of a pointer motion done using the same advice.
    ///
    ///
    /// **Note** only works if [`DeviceType::Pointer`] access was provided after
    /// starting the session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    /// * `dx` - Relative axis movement on the x axis.
    /// * `dy` - Relative axis movement on the y axis.
    /// * `finish` - Whether it is the last axis event.
    ///
    /// # Specifications
    ///
    /// See also [`NotifyPointerAxis`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-notifypointeraxis).
    #[doc(alias = "NotifyPointerAxis")]
    pub async fn notify_pointer_axis(
        &self,
        session: &Session<Self>,
        dx: f64,
        dy: f64,
        options: NotifyPointerAxisOptions,
    ) -> Result<(), Error> {
        self.0
            .call("NotifyPointerAxis", &(session, options, dx, dy))
            .await
    }

    /// Connect to EIS.
    ///
    /// **Note** only succeeds if called after [`RemoteDesktop::start`].
    ///
    /// Requires RemoteDesktop version 2.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`RemoteDesktop::create_session`].
    ///
    /// # Required version
    ///
    /// The method requires the 2nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`ConnectToEIS`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-connecttoeis).
    #[doc(alias = "ConnectToEIS")]
    pub async fn connect_to_eis(
        &self,
        session: &Session<Self>,
        options: ConnectToEISOptions,
    ) -> Result<OwnedFd, Error> {
        let fd = self
            .0
            .call_versioned::<zvariant::OwnedFd>("ConnectToEIS", &(session, options), 2)
            .await?;
        Ok(fd.into())
    }

    /// Available source types.
    ///
    /// # Specifications
    ///
    /// See also [`AvailableDeviceTypes`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.RemoteDesktop.html#org-freedesktop-portal-remotedesktop-availabledevicetypes).
    #[doc(alias = "AvailableDeviceTypes")]
    pub async fn available_device_types(&self) -> Result<BitFlags<DeviceType>, Error> {
        self.0.property("AvailableDeviceTypes").await
    }
}

impl std::ops::Deref for RemoteDesktop {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl crate::Sealed for RemoteDesktop {}
impl SessionPortal for RemoteDesktop {}
