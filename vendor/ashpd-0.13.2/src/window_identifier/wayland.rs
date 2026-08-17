use std::fmt;

use wayland_backend::sys::client::Backend;
use wayland_client::{
    Proxy, QueueHandle,
    protocol::{wl_registry, wl_surface::WlSurface},
};
use wayland_protocols::xdg::foreign::{
    zv1::client::{
        zxdg_exported_v1::{self, ZxdgExportedV1},
        zxdg_exporter_v1::ZxdgExporterV1,
    },
    zv2::client::{
        zxdg_exported_v2::{self, ZxdgExportedV2},
        zxdg_exporter_v2::ZxdgExporterV2,
    },
};

use super::WindowIdentifierType;

// Supported versions.
const ZXDG_EXPORTER_V1: u32 = 1;
const ZXDG_EXPORTER_V2: u32 = 1;

#[derive(Debug)]
pub struct WaylandWindowIdentifier {
    exported: Exported,
    type_: WindowIdentifierType,
}

#[derive(Debug)]
enum Exported {
    V1(ZxdgExportedV1),
    V2(ZxdgExportedV2),
}

impl Exported {
    fn destroy(&self) {
        match self {
            Self::V1(exported) => exported.destroy(),
            Self::V2(exported) => exported.destroy(),
        }
    }
}

#[derive(Debug)]
enum Exporter {
    V1(ZxdgExporterV1),
    V2(ZxdgExporterV2),
}

impl WaylandWindowIdentifier {
    pub async fn new(surface: &WlSurface) -> Option<Self> {
        let backend = surface.backend().upgrade()?;
        let conn = wayland_client::Connection::from_backend(backend);

        Self::new_inner(conn, surface).await
    }

    pub async unsafe fn from_raw(
        surface_ptr: *mut std::ffi::c_void,
        display_ptr: *mut std::ffi::c_void,
    ) -> Option<Self> {
        if surface_ptr.is_null() || display_ptr.is_null() {
            return None;
        }

        let backend = unsafe { Backend::from_foreign_display(display_ptr as *mut _) };
        let conn = wayland_client::Connection::from_backend(backend);
        let obj_id = unsafe {
            wayland_backend::sys::client::ObjectId::from_ptr(
                WlSurface::interface(),
                surface_ptr as *mut _,
            )
        }
        .ok()?;

        let surface = WlSurface::from_id(&conn, obj_id).ok()?;

        Self::new_inner(conn, &surface).await
    }

    async fn new_inner(conn: wayland_client::Connection, surface: &WlSurface) -> Option<Self> {
        let (sender, receiver) =
            futures_channel::oneshot::channel::<Option<WaylandWindowIdentifier>>();

        // Cheap clone, protocol objects are essentially smart pointers
        let surface = surface.clone();
        std::thread::spawn(move || match wayland_export_handle(conn, &surface) {
            Ok(window_handle) => sender.send(Some(window_handle)).unwrap(),
            Err(_err) => {
                #[cfg(feature = "tracing")]
                tracing::info!("Could not get wayland window identifier: {_err}");
                sender.send(None).unwrap();
            }
        });

        receiver.await.unwrap()
    }
}

impl fmt::Display for WaylandWindowIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.type_.fmt(f)
    }
}

impl Drop for WaylandWindowIdentifier {
    fn drop(&mut self) {
        self.exported.destroy();
        #[cfg(feature = "tracing")]
        if let WindowIdentifierType::Wayland(ref handle) = self.type_ {
            tracing::debug!("Unexporting handle: {handle}");
        }
    }
}

#[derive(Default, Debug)]
struct State {
    handle: String,
    exporter: Option<Exporter>,
}

impl wayland_client::Dispatch<ZxdgExportedV1, ()> for State {
    fn event(
        state: &mut Self,
        _proxy: &ZxdgExportedV1,
        event: <ZxdgExportedV1 as Proxy>::Event,
        _data: &(),
        _connhandle: &wayland_client::Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let zxdg_exported_v1::Event::Handle { handle } = event {
            state.handle = handle;
        }
    }
}

impl wayland_client::Dispatch<ZxdgExportedV2, ()> for State {
    fn event(
        state: &mut Self,
        _proxy: &ZxdgExportedV2,
        event: <ZxdgExportedV2 as Proxy>::Event,
        _data: &(),
        _connhandle: &wayland_client::Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let zxdg_exported_v2::Event::Handle { handle } = event {
            state.handle = handle;
        }
    }
}

impl wayland_client::Dispatch<ZxdgExporterV1, ()> for State {
    fn event(
        _state: &mut Self,
        _proxy: &ZxdgExporterV1,
        _event: <ZxdgExporterV1 as Proxy>::Event,
        _data: &(),
        _connhandle: &wayland_client::Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl wayland_client::Dispatch<ZxdgExporterV2, ()> for State {
    fn event(
        _state: &mut Self,
        _proxy: &ZxdgExporterV2,
        _event: <ZxdgExporterV2 as Proxy>::Event,
        _data: &(),
        _connhandle: &wayland_client::Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl wayland_client::Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &wayland_client::Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "zxdg_exporter_v1" => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("Found wayland interface {interface} v{version}");
                    let exporter = registry.bind::<ZxdgExporterV1, (), State>(
                        name,
                        version.min(ZXDG_EXPORTER_V1),
                        qhandle,
                        (),
                    );
                    match state.exporter {
                        Some(Exporter::V2(_)) => (),
                        _ => state.exporter = Some(Exporter::V1(exporter)),
                    }
                }
                "zxdg_exporter_v2" => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("Found wayland interface {interface} v{version}");
                    let exporter = registry.bind::<ZxdgExporterV2, (), State>(
                        name,
                        version.min(ZXDG_EXPORTER_V2),
                        qhandle,
                        (),
                    );
                    state.exporter = Some(Exporter::V2(exporter));
                }
                _ => (),
            }
        }
    }
}

/// A helper to export a wayland handle from a surface and a connection
///
/// Needed for converting a RawWindowHandle to a WindowIdentifier.
fn wayland_export_handle(
    conn: wayland_client::Connection,
    surface: &WlSurface,
) -> Result<WaylandWindowIdentifier, Box<dyn std::error::Error>> {
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();
    let mut state = State::default();
    display.get_registry(&qhandle, ());
    event_queue.roundtrip(&mut state)?;

    let exported = match state.exporter.take() {
        Some(Exporter::V2(exporter)) => {
            let exp = exporter.export_toplevel(surface, &qhandle, ());
            event_queue.roundtrip(&mut state)?;
            exporter.destroy();

            Some(Exported::V2(exp))
        }
        Some(Exporter::V1(exporter)) => {
            let exp = exporter.export(surface, &qhandle, ());
            event_queue.roundtrip(&mut state)?;
            exporter.destroy();

            Some(Exported::V1(exp))
        }
        None => {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "The compositor does not support the zxdg_exporter_v1 nor zxdg_exporter_v2 protocols"
            );
            None
        }
    };

    if let Some(exported) = exported {
        Ok(WaylandWindowIdentifier {
            exported,
            type_: WindowIdentifierType::Wayland(state.handle),
        })
    } else {
        Err(Box::new(crate::Error::NoResponse))
    }
}
