//! Start a screencast session and get the PipeWire remote of it.
//!
//! # Examples
//!
//! How to create a screen cast session & start it.
//! The portal is currently useless without PipeWire & Rust support.
//!
//! ```rust,no_run
//! use ashpd::desktop::{
//!     PersistMode,
//!     screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType},
//! };
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = Screencast::new().await?;
//!     let session = proxy.create_session(Default::default()).await?;
//!     proxy
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
//!     let response = proxy
//!         .start(&session, None, Default::default())
//!         .await?
//!         .response()?;
//!     response.streams().iter().for_each(|stream| {
//!         println!("node id: {}", stream.pipe_wire_node_id());
//!         println!("size: {:?}", stream.size());
//!         println!("position: {:?}", stream.position());
//!     });
//!     Ok(())
//! }
//! ```
//! An example on how to connect with Pipewire can be found [here](https://github.com/bilelmoussaoui/ashpd/blob/main/examples/screen_cast_pw.rs), and with GStreamer [here](https://github.com/bilelmoussaoui/ashpd/blob/main/examples/screen_cast_gstreamer.rs).

use std::{fmt::Debug, os::fd::OwnedFd};

use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{
    self, Optional, OwnedValue, Type,
    as_value::{self, optional},
};

use super::{HandleToken, PersistMode, Request, Session, session::SessionPortal};
use crate::{
    Error, WindowIdentifier,
    desktop::session::{CreateSessionOptions, CreateSessionResponse},
    proxy::Proxy,
};

#[bitflags]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Copy, Clone, Debug, Type)]
#[repr(u32)]
#[doc(alias = "XdpOutputType")]
/// A bit flag for the available sources to record.
pub enum SourceType {
    #[doc(alias = "XDP_OUTPUT_MONITOR")]
    /// A monitor.
    Monitor,
    #[doc(alias = "XDP_OUTPUT_WINDOW")]
    /// A specific window
    Window,
    #[doc(alias = "XDP_OUTPUT_VIRTUAL")]
    /// Virtual
    Virtual,
}

#[bitflags]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone, Type)]
#[repr(u32)]
#[doc(alias = "XdpCursorMode")]
/// A bit flag for the possible cursor modes.
pub enum CursorMode {
    #[doc(alias = "XDP_CURSOR_MODE_HIDDEN")]
    /// The cursor is not part of the screen cast stream.
    Hidden,
    #[doc(alias = "XDP_CURSOR_MODE_EMBEDDED")]
    /// The cursor is embedded as part of the stream buffers.
    Embedded,
    #[doc(alias = "XDP_CURSOR_MODE_METADATA")]
    /// The cursor is not part of the screen cast stream, but sent as PipeWire
    /// stream metadata.
    Metadata,
}

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`Screencast::open_pipe_wire_remote`] request.
#[zvariant(signature = "dict")]
pub struct OpenPipeWireRemoteOptions {}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
/// Specified options for a [`Screencast::select_sources`] request.
#[zvariant(signature = "dict")]
pub struct SelectSourcesOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    types: Option<BitFlags<SourceType>>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    multiple: Option<bool>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    cursor_mode: Option<CursorMode>,
    #[serde(
        with = "optional",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    restore_token: Option<String>,
    #[serde(default, with = "optional", skip_serializing)]
    #[cfg_attr(not(feature = "backend"), allow(dead_code))]
    restore_data: Option<(String, u32, OwnedValue)>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    persist_mode: Option<PersistMode>,
}

impl SelectSourcesOptions {
    /// Sets whether to allow selecting multiple sources.
    #[must_use]
    pub fn set_multiple(mut self, multiple: impl Into<Option<bool>>) -> Self {
        self.multiple = multiple.into();
        self
    }

    /// Gets whether to allow selecting multiple sources.
    #[cfg(feature = "backend")]
    pub fn is_multiple(&self) -> Option<bool> {
        self.multiple
    }

    /// Sets how the cursor will be drawn on the screen cast stream.
    #[must_use]
    pub fn set_cursor_mode(mut self, cursor_mode: impl Into<Option<CursorMode>>) -> Self {
        self.cursor_mode = cursor_mode.into();
        self
    }

    /// Gets how the cursor will be drawn on the screen cast stream.
    #[cfg(feature = "backend")]
    pub fn cursor_mode(&self) -> Option<CursorMode> {
        self.cursor_mode
    }

    /// Sets the types of content to record.
    #[must_use]
    pub fn set_sources(mut self, types: impl Into<Option<BitFlags<SourceType>>>) -> Self {
        self.types = types.into();
        self
    }

    /// Gets the types of content to record.
    #[cfg(feature = "backend")]
    pub fn sources(&self) -> Option<BitFlags<SourceType>> {
        self.types
    }

    /// Sets the persist mode.
    #[must_use]
    pub fn set_persist_mode(mut self, persist_mode: impl Into<Option<PersistMode>>) -> Self {
        self.persist_mode = persist_mode.into();
        self
    }

    /// Gets the persist mode.
    #[cfg(feature = "backend")]
    pub fn persist_mode(&self) -> Option<PersistMode> {
        self.persist_mode
    }

    /// Sets the restore token.
    #[must_use]
    pub fn set_restore_token<'a>(mut self, token: impl Into<Option<&'a str>>) -> Self {
        self.restore_token = token.into().map(ToOwned::to_owned);
        self
    }

    /// Gets the restore data.
    #[cfg(feature = "backend")]
    pub fn restore_data<'a>(&'a self) -> Option<(&'a str, u32, &'a zbus::zvariant::Value<'a>)> {
        use std::borrow::Borrow;
        match &self.restore_data {
            Some((key, version, data)) => Some((key.as_str(), *version, data.borrow())),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
/// Specified options for a [`Screencast::start`] request.
#[zvariant(signature = "dict")]
pub struct StartCastOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
}

#[derive(Default, Serialize, Deserialize, Type)]
/// A response to a [`Screencast::start`] request.
#[zvariant(signature = "dict")]
pub struct Streams {
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    streams: Vec<Stream>,
    #[serde(
        default,
        with = "optional",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    restore_token: Option<String>,
    #[serde(default, with = "optional", skip_serializing)]
    #[cfg_attr(not(feature = "backend"), allow(dead_code))]
    restore_data: Option<(String, u32, OwnedValue)>,
}

impl Streams {
    /// The session restore token.
    pub fn restore_token(&self) -> Option<&str> {
        self.restore_token.as_deref()
    }

    /// The list of streams.
    pub fn streams(&self) -> &[Stream] {
        &self.streams
    }

    /// The session restore data.
    #[cfg(feature = "backend")]
    pub fn restore_data(&self) -> Option<&(String, u32, OwnedValue)> {
        self.restore_data.as_ref()
    }
}

impl Debug for Streams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Streams")
            .field(&self.restore_token)
            .field(&self.streams)
            .finish()
    }
}

/// A [builder-pattern] type to construct a response to a [`Screencast::start`]
/// request.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[cfg(feature = "backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
pub struct StreamsBuilder {
    streams: Streams,
}

#[cfg(feature = "backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
impl StreamsBuilder {
    /// Create a new instance of a streams builder.
    pub fn new(streams: Vec<Stream>) -> Self {
        Self {
            streams: Streams {
                streams,
                restore_token: None,
                restore_data: None,
            },
        }
    }

    /// Set the streams' optional restore data.
    pub fn restore_data(mut self, data: Option<(String, u32, impl Into<OwnedValue>)>) -> Self {
        self.streams.restore_data = data.map(|(s, u, d)| (s, u, d.into()));
        self
    }

    /// Build the [`Streams`].
    pub fn build(self) -> Streams {
        self.streams
    }
}

#[derive(Clone, Serialize, Deserialize, Type)]
/// A PipeWire stream.
pub struct Stream(u32, StreamProperties);

impl Stream {
    /// The PipeWire stream Node ID
    pub fn pipe_wire_node_id(&self) -> u32 {
        self.0
    }

    /// A tuple consisting of the position (x, y) in the compositor coordinate
    /// space.
    ///
    /// **Note** the position may not be equivalent to a position in a pixel
    /// coordinate space. Only available for monitor streams.
    pub fn position(&self) -> Option<(i32, i32)> {
        self.1.position
    }

    /// A tuple consisting of (width, height).
    /// The size represents the size of the stream as it is displayed in the
    /// compositor coordinate space.
    ///
    /// **Note** the size may not be equivalent to a size in a pixel coordinate
    /// space. The size may differ from the size of the stream.
    pub fn size(&self) -> Option<(i32, i32)> {
        self.1.size
    }

    /// The source type of the stream.
    pub fn source_type(&self) -> Option<SourceType> {
        self.1.source_type
    }

    /// The stream identifier.
    pub fn id(&self) -> Option<&str> {
        self.1.id.as_deref()
    }

    // TODO Added in version 5 of the interface.
    /// The stream mapping id.
    pub fn mapping_id(&self) -> Option<&str> {
        self.1.mapping_id.as_deref()
    }
}

impl Debug for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stream")
            .field("pipewire_node_id", &self.pipe_wire_node_id())
            .field("position", &self.position())
            .field("size", &self.size())
            .field("source_type", &self.source_type())
            .field("id", &self.id())
            .finish()
    }
}
#[derive(Clone, Serialize, Deserialize, Type, Debug)]
/// The stream properties.
#[zvariant(signature = "dict")]
struct StreamProperties {
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    position: Option<(i32, i32)>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    size: Option<(i32, i32)>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    source_type: Option<SourceType>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    mapping_id: Option<String>,
}

/// A [builder-pattern] type to construct a PipeWire stream [`Stream`].
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[cfg(feature = "backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
pub struct StreamBuilder {
    stream: Stream,
}

#[cfg(feature = "backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
impl StreamBuilder {
    /// Create a new instance of a stream builder.
    pub fn new(pipe_wire_node_id: u32) -> Self {
        Self {
            stream: Stream(
                pipe_wire_node_id,
                StreamProperties {
                    id: None,
                    position: None,
                    size: None,
                    source_type: None,
                    mapping_id: None,
                },
            ),
        }
    }

    /// Set the stream's optional id (opaque identifier, local to a given
    /// session, persisted across restored sessions).
    pub fn id(mut self, id: impl Into<Option<String>>) -> Self {
        self.stream.1.id = id.into();
        self
    }

    /// Set the stream's optional position (in the compositor coordinate space).
    pub fn position(mut self, position: impl Into<Option<(i32, i32)>>) -> Self {
        self.stream.1.position = position.into();
        self
    }

    /// Set the stream's optional size (in the compositor coordinate space).
    pub fn size(mut self, size: impl Into<Option<(i32, i32)>>) -> Self {
        self.stream.1.size = size.into();
        self
    }

    /// Set the stream's optional source type.
    pub fn source_type(mut self, source_type: impl Into<Option<SourceType>>) -> Self {
        self.stream.1.source_type = source_type.into();
        self
    }

    /// Set the stream's optional mapping id (identifier used to map different
    /// aspects of the resource this stream corresponds to).
    pub fn mapping_id(mut self, mapping_id: impl Into<Option<String>>) -> Self {
        self.stream.1.mapping_id = mapping_id.into();
        self
    }

    /// Build the [`Stream`].
    pub fn build(self) -> Stream {
        self.stream
    }
}

/// The interface lets sandboxed applications create screen cast sessions.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.ScreenCast`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.ScreenCast")]
pub struct Screencast(Proxy<'static>);

impl Screencast {
    /// Create a new instance of [`Screencast`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.ScreenCast").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`Screencast`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.ScreenCast")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Create a screen cast session.
    ///
    /// # Specifications
    ///
    /// See also [`CreateSession`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html#org-freedesktop-portal-screencast-createsession).
    #[doc(alias = "CreateSession")]
    #[doc(alias = "xdp_portal_create_screencast_session")]
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
            Session::from_unique_name(self.0.connection().clone(), &options.session_handle_token),
        )?;
        assert_eq!(proxy.path(), &request.response()?.session_handle.as_ref());
        Ok(proxy)
    }

    /// Open a file descriptor to the PipeWire remote where the screen cast
    /// streams are available.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`Screencast::create_session`].
    ///
    /// # Returns
    ///
    /// File descriptor of an open PipeWire remote.
    ///
    /// # Specifications
    ///
    /// See also [`OpenPipeWireRemote`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html#org-freedesktop-portal-screencast-openpipewireremote).
    #[doc(alias = "OpenPipeWireRemote")]
    pub async fn open_pipe_wire_remote(
        &self,
        session: &Session<impl HasScreencastSession>,
        options: OpenPipeWireRemoteOptions,
    ) -> Result<OwnedFd, Error> {
        let fd = self
            .0
            .call::<zvariant::OwnedFd>("OpenPipeWireRemote", &(session, options))
            .await?;
        Ok(fd.into())
    }

    /// Configure what the screen cast session should record.
    /// This method must be called before starting the session.
    ///
    /// Passing invalid input to this method will cause the session to be
    /// closed. An application may only attempt to select sources once per
    /// session.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`Screencast::create_session`].
    /// * `cursor_mode` - Sets how the cursor will be drawn on the screen cast
    ///   stream.
    /// * `types` - Sets the types of content to record.
    /// * `multiple`- Sets whether to allow selecting multiple sources.
    ///
    /// # Specifications
    ///
    /// See also [`SelectSources`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html#org-freedesktop-portal-screencast-selectsources).
    #[doc(alias = "SelectSources")]
    pub async fn select_sources(
        &self,
        session: &Session<impl HasScreencastSession>,
        options: SelectSourcesOptions,
    ) -> Result<Request<()>, Error> {
        self.0
            .empty_request(&options.handle_token, "SelectSources", &(session, &options))
            .await
    }

    /// Start the screen cast session.
    ///
    /// This will typically result the portal presenting a dialog letting the
    /// user do the selection set up by `select_sources`.
    ///
    /// An application can only attempt start a session once.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_session()`][`Screencast::create_session`].
    /// * `identifier` - Identifier for the application window.
    ///
    /// # Return
    ///
    /// A list of [`Stream`] and an optional restore token.
    ///
    /// # Specifications
    ///
    /// See also [`Start`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html#org-freedesktop-portal-screencast-start).
    #[doc(alias = "Start")]
    pub async fn start(
        &self,
        session: &Session<impl HasScreencastSession>,
        identifier: Option<&WindowIdentifier>,
        options: StartCastOptions,
    ) -> Result<Request<Streams>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "Start",
                &(session, identifier, &options),
            )
            .await
    }

    /// Available cursor mode.
    ///
    /// # Specifications
    ///
    /// See also [`AvailableCursorModes`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html#org-freedesktop-portal-screencast-availablecursormodes).
    #[doc(alias = "AvailableCursorModes")]
    pub async fn available_cursor_modes(&self) -> Result<BitFlags<CursorMode>, Error> {
        self.0.property_versioned("AvailableCursorModes", 2).await
    }

    /// Available source types.
    ///
    /// # Specifications
    ///
    /// See also [`AvailableSourceTypes`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.ScreenCast.html#org-freedesktop-portal-screencast-availablesourcetypes).
    #[doc(alias = "AvailableSourceTypes")]
    pub async fn available_source_types(&self) -> Result<BitFlags<SourceType>, Error> {
        self.0.property("AvailableSourceTypes").await
    }
}

impl std::ops::Deref for Screencast {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl crate::Sealed for Screencast {}
impl SessionPortal for Screencast {}

/// Defines which portals session can be used in a screen-cast.
pub trait HasScreencastSession: SessionPortal {}
impl HasScreencastSession for Screencast {}
#[cfg(feature = "remote_desktop")]
impl HasScreencastSession for super::remote_desktop::RemoteDesktop {}
