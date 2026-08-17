//! Check if a camera is available, request access to it and open a PipeWire
//! remote stream.
//!
//! ### Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::camera::Camera;
//!
//! pub async fn run() -> ashpd::Result<()> {
//!     let camera = Camera::new().await?;
//!     if camera.is_present().await? {
//!         camera.request_access(Default::default()).await?;
//!         let remote_fd = camera.open_pipe_wire_remote(Default::default()).await?;
//!         // pass the remote fd to GStreamer for example
//!     }
//!     Ok(())
//! }
//! ```
//! An example on how to connect with Pipewire can be found [here](https://github.com/bilelmoussaoui/ashpd/blob/main/examples/screen_cast_pw.rs), and with GStreamer [here](https://github.com/bilelmoussaoui/ashpd/blob/main/examples/screen_cast_gstreamer.rs).
//! Although the examples are primary focus is screen casting, stream connection
//! logic remains the same -- with one accessibility change:
//! ```rust,ignore
//! let stream = pw::stream::Stream::new(
//!    &core,
//!    "video-test",
//!    properties! {
//!        *pw::keys::MEDIA_TYPE => "Video",
//!        *pw::keys::MEDIA_CATEGORY => "Capture",
//!        *pw::keys::MEDIA_ROLE => "Screen", // <-- make this 'Camera'
//!    },
//! )?;
//! ```

use std::{collections::HashMap, os::fd::OwnedFd};

use serde::Serialize;
use zbus::zvariant::{self, Type, as_value};

use super::{HandleToken, Request};
use crate::{Error, proxy::Proxy};

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`Camera::request_access`] request.
pub struct CameraAccessOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`Camera::open_pipe_wire_remote`] request.
pub struct OpenPipeWireRemoteOptions {}

/// The interface lets sandboxed applications access camera devices, such as web
/// cams.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Camera`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Camera.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Camera")]
pub struct Camera(Proxy<'static>);

impl Camera {
    /// Create a new instance of [`Camera`].
    pub async fn new() -> Result<Camera, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Camera").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`Camera`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Camera, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Camera").await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Requests an access to the camera.
    ///
    /// # Specifications
    ///
    /// See also [`AccessCamera`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Camera.html#org-freedesktop-portal-camera-accesscamera).
    #[doc(alias = "AccessCamera")]
    #[doc(alias = "xdp_portal_access_camera")]
    pub async fn request_access(&self, options: CameraAccessOptions) -> Result<Request<()>, Error> {
        self.0
            .empty_request(&options.handle_token, "AccessCamera", &options)
            .await
    }

    /// Open a file descriptor to the PipeWire remote where the camera nodes are
    /// available.
    ///
    /// # Returns
    ///
    /// File descriptor of an open PipeWire remote.
    ///
    /// # Specifications
    ///
    /// See also [`OpenPipeWireRemote`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Camera.html#org-freedesktop-portal-camera-openpipewireremote).
    #[doc(alias = "OpenPipeWireRemote")]
    #[doc(alias = "xdp_portal_open_pipewire_remote_for_camera")]
    pub async fn open_pipe_wire_remote(
        &self,
        options: OpenPipeWireRemoteOptions,
    ) -> Result<OwnedFd, Error> {
        let fd = self
            .0
            .call::<zvariant::OwnedFd>("OpenPipeWireRemote", &options)
            .await?;
        Ok(fd.into())
    }

    /// A boolean stating whether there is any cameras available.
    ///
    /// # Specifications
    ///
    /// See also [`IsCameraPresent`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Camera.html#org-freedesktop-portal-camera-iscamerapresent).
    #[doc(alias = "IsCameraPresent")]
    #[doc(alias = "xdp_portal_is_camera_present")]
    pub async fn is_present(&self) -> Result<bool, Error> {
        self.0.property("IsCameraPresent").await
    }
}

impl std::ops::Deref for Camera {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "pipewire")]
/// A PipeWire camera stream returned by [`pipewire_streams`].
#[derive(Debug)]
pub struct Stream {
    node_id: u32,
    properties: HashMap<String, String>,
}

#[cfg(feature = "pipewire")]
impl Stream {
    /// The id of the PipeWire node.
    pub fn node_id(&self) -> u32 {
        self.node_id
    }

    /// The node properties.
    pub fn properties(&self) -> HashMap<String, String> {
        self.properties.clone()
    }
}

#[cfg(feature = "pipewire")]
fn pipewire_streams_inner<F: Fn(Stream) + Clone + 'static, G: FnOnce() + Clone + 'static>(
    fd: OwnedFd,
    callback: F,
    done_callback: G,
) -> Result<(), pipewire::Error> {
    let mainloop = pipewire::main_loop::MainLoopRc::new(None)?;
    let context = pipewire::context::ContextRc::new(&mainloop, None)?;
    let core = context.connect_fd(fd, None)?;
    let registry = core.get_registry()?;

    let pending = core.sync(0).expect("sync failed");

    let loop_clone = mainloop.clone();
    let _listener_reg = registry
        .add_listener_local()
        .global(move |global| {
            if let Some(props) = &global.props
                && props.get("media.role") == Some("Camera")
            {
                #[cfg(feature = "tracing")]
                tracing::info!("found camera: {:#?}", props);

                let mut properties = HashMap::new();
                for (key, value) in props.iter() {
                    properties.insert(key.to_string(), value.to_string());
                }
                let node_id = global.id;

                let stream = Stream {
                    node_id,
                    properties,
                };
                callback.clone()(stream);
            }
        })
        .register();
    let _listener_core = core
        .add_listener_local()
        .done(move |id, seq| {
            if id == pipewire::core::PW_ID_CORE && seq == pending {
                loop_clone.quit();
                done_callback.clone()();
            }
        })
        .register();

    mainloop.run();

    Ok(())
}

/// A helper to get a list of PipeWire streams to use with the camera file
/// descriptor returned by [`Camera::open_pipe_wire_remote`].
///
/// Currently, the camera portal only gives us a file descriptor. Not passing a
/// node id may cause the media session controller to auto-connect the client to
/// an incorrect node.
///
/// The method looks for the available output streams of a `media.role` type of
/// `Camera` and return a list of `Stream`s.
///
/// *Note* The socket referenced by `fd` must not be used while this function is
/// running.
#[cfg(feature = "pipewire")]
#[cfg_attr(docsrs, doc(cfg(feature = "pipewire")))]
pub async fn pipewire_streams(fd: OwnedFd) -> Result<Vec<Stream>, pipewire::Error> {
    let (sender, receiver) = futures_channel::oneshot::channel();
    let (streams_sender, mut streams_receiver) = futures_channel::mpsc::unbounded();

    let sender = std::sync::Arc::new(std::sync::Mutex::new(Some(sender)));
    let streams_sender = std::sync::Arc::new(std::sync::Mutex::new(streams_sender));

    std::thread::spawn(move || {
        let inner_sender = sender.clone();

        if let Err(err) = pipewire_streams_inner(
            fd,
            move |stream| {
                let inner_streams_sender = streams_sender.clone();
                if let Ok(mut sender) = inner_streams_sender.lock() {
                    let _result = sender.start_send(stream);
                };
            },
            move || {
                if let Ok(mut guard) = inner_sender.lock()
                    && let Some(inner_sender) = guard.take()
                {
                    let _result = inner_sender.send(Ok(()));
                }
            },
        ) {
            #[cfg(feature = "tracing")]
            tracing::error!("Failed to get pipewire streams {:#?}", err);
            let mut guard = sender.lock().unwrap();
            if let Some(sender) = guard.take() {
                let _ = sender.send(Err(err));
            }
        }
    });

    receiver.await.unwrap()?;

    let mut streams = vec![];
    let mut seen_node_ids = std::collections::HashSet::new();
    while let Ok(stream) = streams_receiver.try_recv() {
        if seen_node_ids.insert(stream.node_id) {
            streams.push(stream);
        }
    }

    Ok(streams)
}

#[cfg(not(feature = "pipewire"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "pipewire"))))]
/// Request access to the camera and return a file descriptor if one is
/// available.
pub async fn request() -> Result<Option<OwnedFd>, Error> {
    let proxy = Camera::new().await?;
    proxy.request_access(Default::default()).await?;
    if proxy.is_present().await? {
        Ok(Some(proxy.open_pipe_wire_remote(Default::default()).await?))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "pipewire")]
#[cfg_attr(docsrs, doc(cfg(feature = "pipewire")))]
/// Request access to the camera and return a file descriptor and a list of the
/// available streams, one per camera.
pub async fn request() -> Result<Option<(OwnedFd, Vec<Stream>)>, Error> {
    let proxy = Camera::new().await?;
    proxy.request_access(Default::default()).await?;
    if proxy.is_present().await? {
        let fd = proxy.open_pipe_wire_remote(Default::default()).await?;
        let streams = pipewire_streams(fd.try_clone()?).await?;
        Ok(Some((fd, streams)))
    } else {
        Ok(None)
    }
}
