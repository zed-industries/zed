#[cfg(feature = "raw_handle")]
use std::ptr::NonNull;
#[cfg(feature = "gtk4_wayland")]
use std::sync::Arc;

#[cfg(feature = "gtk4_wayland")]
use futures_util::lock::Mutex;
use gdk::Backend;
#[cfg(feature = "raw_handle")]
use glib::translate::ToGlibPtr;
use gtk4::{gdk, glib, prelude::*};
#[cfg(feature = "raw_handle")]
use raw_window_handle::{
    DisplayHandle, RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
    WindowHandle, XlibDisplayHandle, XlibWindowHandle,
};

use super::WindowIdentifierType;

#[cfg(feature = "gtk4_wayland")]
const WINDOW_HANDLE_KEY: &str = "ashpd-wayland-gtk4-window-handle";

pub struct Gtk4WindowIdentifier {
    #[allow(dead_code)]
    native: gtk4::Native,
    type_: WindowIdentifierType,
    exported: bool,
}

impl Gtk4WindowIdentifier {
    pub async fn new(native: &impl glib::prelude::IsA<gtk4::Native>) -> Option<Self> {
        let surface = native.surface()?;
        match surface.display().backend() {
            #[cfg(feature = "gtk4_wayland")]
            Backend::Wayland => {
                let top_level = surface
                    .downcast_ref::<gdk4wayland::WaylandToplevel>()
                    .unwrap();
                let handle = unsafe {
                    if let Some(mut handle) = top_level.data(WINDOW_HANDLE_KEY) {
                        let (handle, ref_count): &mut (Option<String>, u8) = handle.as_mut();
                        *ref_count += 1;
                        handle.clone()
                    } else {
                        let (sender, receiver) =
                            futures_channel::oneshot::channel::<Option<String>>();
                        let sender = Arc::new(Mutex::new(Some(sender)));

                        let result = top_level.export_handle(glib::clone!(#[strong] sender, move |_, handle| {
                            let handle = handle.map(ToOwned::to_owned);
                            glib::spawn_future_local(glib::clone!(#[strong] sender, #[strong] handle, async move {
                                if let Some(m) = sender.lock().await.take() {
                                    match handle {
                                        Ok(h) => {
                                            let _ = m.send(Some(h.to_string()));
                                        },
                                        Err(_err) => {
                                            let _ = m.send(None);
                                            #[cfg(feature = "tracing")]
                                            tracing::warn!("Failed to export window identifier. The compositor doesn't support xdg-foreign protocol. {_err}");
                                        }
                                    }
                                }
                            }));
                        }));

                        if !result {
                            return None;
                        }
                        let handle = receiver.await.ok().flatten();
                        top_level.set_data(WINDOW_HANDLE_KEY, (handle.clone(), 1));
                        handle
                    }
                };
                Some(Gtk4WindowIdentifier {
                    native: native.clone().upcast(),
                    exported: handle.is_some(),
                    type_: WindowIdentifierType::Wayland(handle.unwrap_or_default()),
                })
            }
            #[cfg(feature = "gtk4_x11")]
            Backend::X11 => {
                let xid = surface
                    .downcast_ref::<gdk4x11::X11Surface>()
                    .map(|w| w.xid())?;
                Some(Gtk4WindowIdentifier {
                    native: native.clone().upcast(),
                    exported: false,
                    type_: WindowIdentifierType::X11(xid),
                })
            }
            _ => None,
        }
    }

    #[cfg(feature = "raw_handle")]
    pub fn as_raw_window_handle(&self) -> WindowHandle<'_> {
        unsafe {
            let raw_handle = match self.type_ {
                #[cfg(feature = "gtk4_wayland")]
                WindowIdentifierType::Wayland(_) => {
                    let surface = self.native.surface().unwrap();
                    RawWindowHandle::Wayland(WaylandWindowHandle::new(
                        NonNull::new(gdk4wayland::ffi::gdk_wayland_surface_get_wl_surface(
                            surface
                                .downcast_ref::<gdk4wayland::WaylandSurface>()
                                .unwrap()
                                .to_glib_none()
                                .0,
                        ))
                        .expect("Identifier must be attached to a wl_surface"),
                    ))
                }
                #[cfg(feature = "gtk4_x11")]
                WindowIdentifierType::X11(xid) => RawWindowHandle::Xlib(XlibWindowHandle::new(xid)),
            };
            WindowHandle::borrow_raw(raw_handle)
        }
    }

    #[cfg(feature = "raw_handle")]
    pub fn as_raw_display_handle(&self) -> DisplayHandle<'_> {
        let surface = self.native.surface().unwrap();
        let display = surface.display();
        unsafe {
            let raw_handle = match self.type_ {
                #[cfg(feature = "gtk4_wayland")]
                WindowIdentifierType::Wayland(_) => {
                    RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
                        NonNull::new(gdk4wayland::ffi::gdk_wayland_display_get_wl_display(
                            display
                                .downcast_ref::<gdk4wayland::WaylandDisplay>()
                                .unwrap()
                                .to_glib_none()
                                .0,
                        ))
                        .expect("Identifier must be attached to a wl_display"),
                    ))
                }
                #[cfg(feature = "gtk4_x11")]
                WindowIdentifierType::X11(_xid) => RawDisplayHandle::Xlib(XlibDisplayHandle::new(
                    NonNull::new(gdk4x11::ffi::gdk_x11_display_get_xdisplay(
                        display
                            .downcast_ref::<gdk4x11::X11Display>()
                            .unwrap()
                            .to_glib_none()
                            .0,
                    )),
                    display
                        .downcast_ref::<gdk4x11::X11Display>()
                        .unwrap()
                        .screen()
                        .screen_number(),
                )),
            };
            DisplayHandle::borrow_raw(raw_handle)
        }
    }
}

impl std::fmt::Display for Gtk4WindowIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.type_.fmt(f)
    }
}

impl Drop for Gtk4WindowIdentifier {
    fn drop(&mut self) {
        if !self.exported {
            return;
        }
        match self.type_ {
            #[cfg(feature = "gtk4_wayland")]
            WindowIdentifierType::Wayland(_) => {
                let surface = self.native.surface().unwrap();
                let top_level = surface
                    .downcast_ref::<gdk4wayland::WaylandToplevel>()
                    .unwrap();
                unsafe {
                    let (_handle, ref_count): &mut (Option<String>, u8) =
                        top_level.data(WINDOW_HANDLE_KEY).unwrap().as_mut();
                    if ref_count > &mut 1 {
                        *ref_count -= 1;
                        return;
                    }
                    top_level.unexport_handle();
                    #[cfg(feature = "tracing")]
                    tracing::debug!("Unexporting handle: {_handle:?}");
                    let _ = top_level.steal_data::<(Option<String>, u8)>(WINDOW_HANDLE_KEY);
                }
            }
            _ => (),
        }
    }
}
