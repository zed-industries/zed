/*
 * Copyright (C) 2013 James Miller <james@aatch.net>
 * Copyright (c) 2016
 *         Remi Thebault <remi.thebault@gmail.com>
 *         Thomas Bracht Laumann Jespersen <laumann.thomas@gmail.com>
 * Copyright (c) 2017-2021 Remi Thebault <remi.thebault@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any
 * person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the
 * Software without restriction, including without
 * limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software
 * is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice
 * shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 * SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::diverging_sub_expression)]

//! Rust bindings to the XCB library.
//!
//! The X protocol C-language Binding (XCB - <https://xcb.freedesktop.org/>) is
//! a replacement for Xlib featuring a small footprint, latency hiding, direct
//! access to the protocol, improved threading support, and extensibility.
//!
//! The communication is established with the X server by the creation of a
//! [Connection] object.
//!
//! A client communicates with the server by sending requests. There are 2 types
//! of requests:
//!
//!   - void requests: requests that do not expect an answer
//!     (e.g. [x::ChangeProperty])
//!   - non-void requests: requests that need a `Reply` (e.g. [x::GetProperty])
//!
//! Requests are passed to the server by filling a request structure e.g.
//! [x::CreateWindow] and passing it to [Connection::send_request].
//!
//! The server can also communicate with clients by sending `Event`s.
//! The client listens to events with calls such as [Connection::wait_for_event]
//! (blocking) or [Connection::poll_for_event] (non-blocking).
//!
//! The [x] module contains definitions of the core X protocol.
//! Each extension is defined in its own module such as [xkb] or [render], and
//! is activated by a cargo feature of the same name.
//!
//! # Example
//!
//! Here is a walk-through of a simple `xcb` client.
//! ```no_run
//! // we import the necessary modules (only the core X module in this application).
//! use xcb::{x};
//! // we need to import the `Xid` trait for the `resource_id` call down there.
//! use xcb::{Xid};
//!
//! // Many xcb functions return a `xcb::Result` or compatible result.
//! fn main() -> xcb::Result<()> {
//!     // Connect to the X server.
//!     let (conn, screen_num) = xcb::Connection::connect(None)?;
//!
//!     // Fetch the `x::Setup` and get the main `x::Screen` object.
//!     let setup = conn.get_setup();
//!     let screen = setup.roots().nth(screen_num as usize).unwrap();
//!
//!     // Generate an `Xid` for the client window.
//!     // The type inference is needed here.
//!     let window: x::Window = conn.generate_id();
//!
//!     // We can now create a window. For this we pass a `Request`
//!     // object to the `send_request_checked` method. The method
//!     // returns a cookie that will be used to check for success.
//!     let cookie = conn.send_request_checked(&x::CreateWindow {
//!         depth: x::COPY_FROM_PARENT as u8,
//!         wid: window,
//!         parent: screen.root(),
//!         x: 0,
//!         y: 0,
//!         width: 150,
//!         height: 150,
//!         border_width: 0,
//!         class: x::WindowClass::InputOutput,
//!         visual: screen.root_visual(),
//!         // this list must be in same order than `Cw` enum order
//!         value_list: &[
//!             x::Cw::BackPixel(screen.white_pixel()),
//!             x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS)
//!         ],
//!     });
//!     // We now check if the window creation worked.
//!     // A cookie can't be cloned; it is moved to the function.
//!     conn.check_request(cookie)?;
//!
//!     // Let's change the window title
//!     let cookie = conn.send_request_checked(&x::ChangeProperty {
//!         mode: x::PropMode::Replace,
//!         window,
//!         property: x::ATOM_WM_NAME,
//!         r#type: x::ATOM_STRING,
//!         data: b"My XCB Window",
//!     });
//!     // And check for success again
//!     conn.check_request(cookie)?;
//!
//!     // We now show ("map" in X terminology) the window.
//!     // This time we do not check for success, so we discard the cookie.
//!     conn.send_request(&x::MapWindow {
//!         window,
//!     });
//!
//!     // We need a few atoms for our application.
//!     // We send a few requests in a row and wait for the replies after.
//!     let (wm_protocols, wm_del_window, wm_state, wm_state_maxv, wm_state_maxh) = {
//!         let cookies = (
//!             conn.send_request(&x::InternAtom {
//!                 only_if_exists: true,
//!                 name: b"WM_PROTOCOLS",
//!             }),
//!             conn.send_request(&x::InternAtom {
//!                 only_if_exists: true,
//!                 name: b"WM_DELETE_WINDOW",
//!             }),
//!             conn.send_request(&x::InternAtom {
//!                 only_if_exists: true,
//!                 name: b"_NET_WM_STATE",
//!             }),
//!             conn.send_request(&x::InternAtom {
//!                 only_if_exists: true,
//!                 name: b"_NET_WM_STATE_MAXIMIZED_VERT",
//!             }),
//!             conn.send_request(&x::InternAtom {
//!                 only_if_exists: true,
//!                 name: b"_NET_WM_STATE_MAXIMIZED_HORZ",
//!             }),
//!         );
//!         (
//!             conn.wait_for_reply(cookies.0)?.atom(),
//!             conn.wait_for_reply(cookies.1)?.atom(),
//!             conn.wait_for_reply(cookies.2)?.atom(),
//!             conn.wait_for_reply(cookies.3)?.atom(),
//!             conn.wait_for_reply(cookies.4)?.atom(),
//!         )
//!     };
//!
//!     // We now activate the window close event by sending the following request.
//!     // If we don't do this we can still close the window by clicking on the "x" button,
//!     // but the event loop is notified through a connection shutdown error.
//!     conn.check_request(conn.send_request_checked(&x::ChangeProperty {
//!         mode: x::PropMode::Replace,
//!         window,
//!         property: wm_protocols,
//!         r#type: x::ATOM_ATOM,
//!         data: &[wm_del_window],
//!     }))?;
//!
//!     // Previous request was checked, so a flush is not necessary in this case.
//!     // Otherwise, here is how to perform a connection flush.
//!     conn.flush()?;
//!
//!     let mut maximized = false;
//!
//!     // We enter the main event loop
//!     loop {
//!         match conn.wait_for_event()? {
//!             xcb::Event::X(x::Event::KeyPress(ev)) => {
//!                 if ev.detail() == 0x3a {
//!                     // The M key was pressed
//!                     // (M only on qwerty keyboards. Keymap support is done
//!                     // with the `xkb` extension and the `xkbcommon-rs` crate)
//!
//!                     // We toggle maximized state, for this we send a message
//!                     // by building a `x::ClientMessageEvent` with the proper
//!                     // atoms and send it to the server.
//!
//!                     let data = x::ClientMessageData::Data32([
//!                         if maximized { 0 } else { 1 },
//!                         wm_state_maxv.resource_id(),
//!                         wm_state_maxh.resource_id(),
//!                         0,
//!                         0,
//!                     ]);
//!                     let event = x::ClientMessageEvent::new(window, wm_state, data);
//!                     let cookie = conn.send_request_checked(&x::SendEvent {
//!                         propagate: false,
//!                         destination: x::SendEventDest::Window(screen.root()),
//!                         event_mask: x::EventMask::STRUCTURE_NOTIFY,
//!                         event: &event,
//!                     });
//!                     conn.check_request(cookie)?;
//!
//!                     // Same as before, if we don't check for error, we have to flush
//!                     // the connection.
//!                     // conn.flush()?;
//!
//!                     maximized = !maximized;
//!                 } else if ev.detail() == 0x18 {
//!                     // Q (on qwerty)
//!
//!                     // We exit the event loop (and the program)
//!                     break Ok(());
//!                 }
//!             }
//!             xcb::Event::X(x::Event::ClientMessage(ev)) => {
//!                 // We have received a message from the server
//!                 if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
//!                     if atom == wm_del_window.resource_id() {
//!                         // The received atom is "WM_DELETE_WINDOW".
//!                         // We can check here if the user needs to save before
//!                         // exit, or in our case, exit right away.
//!                         break Ok(());
//!                     }
//!                 }
//!             }
//!             _ => {}
//!         }
//!     }
//! }
//! ```
//!
//! # Cargo features
//!
//! The following Cargo features are available
//!
//! ## `xlib_xcb`
//!
//! This feature activates the use of `xlib::Display` to connect to XCB, therefore making
//! available both Xlib and XCB functions to the same connection.
//! While XCB is sufficient to handle all communication with the X server, some things can
//! still only be done by Xlib. E.g. hardware initialization for OpenGL is done by Xlib only.
//!
//! ## `debug_atom_names`
//!
//! When this feature is activated, the `fmt::Debug` implementation for `x::Atom` will print
//! out the name of the atom in addition to its value.
//!
//! E.g. the feature would turn `Atom { res_id: 303 }` into `Atom("Abs Pressure" ; 303)`.
//!
//! This can be useful in situations where you are not sure which atom you have to intern
//! in order to look up some data in a reply.
//!
//! It should be noted that the feature sets global variable to have access to
//! the connection in the `fmt::Debug` implementation, and that the `Debug` print
//! have side effects (`x::GetAtomName` requests) which can sometimes not be desirable.
//! The feature should therefore only be activated when needed.
//!
//! ## `libxcb_v1_14`
//!
//! This feature is enabled by default and activates the libxcb API version 1.14.
//! To use a version of the libxcb API prior to 1.14, you must disable it.
//!
//! ## Extension features
//!
//! The following X extensions are activated by a cargo feature:
//!
//! | Extension name                | Cargo feature |
//! |-------------------------------|---------------|
//! | `Composite`                   | `composite`   |
//! | `DAMAGE`                      | `damage`      |
//! | `DPMS`                        | `dpms`        |
//! | `DRI2`                        | `dri2`        |
//! | `DRI3`                        | `dri3`        |
//! | `Generic Event Extension`     | `ge`          |
//! | `GLX`                         | `glx`         |
//! | `Present`                     | `present`     |
//! | `RANDR`                       | `randr`       |
//! | `RECORD`                      | `record`      |
//! | `RENDER`                      | `render`      |
//! | `X-Resource`                  | `res`         |
//! | `MIT-SCREEN-SAVER`            | `screensaver` |
//! | `SHAPE`                       | `shape`       |
//! | `MIT-SHM`                     | `shm`         |
//! | `SYNC`                        | `sync`        |
//! | `XEVIE`                       | `xevie`       |
//! | `XFree86-DRI`                 | `xf86dri`     |
//! | `XFree86-VidModeExtension`    | `xf86vidmode` |
//! | `XFIXES`                      | `xfixes`      |
//! | `XINERAMA`                    | `xinerama`    |
//! | `XInputExtension`             | `xinput`      |
//! | `XKEYBOARD`                   | `xkb`         |
//! | `XpExtension`                 | `xprint`      |
//! | `SELinux`                     | `xselinux`    |
//! | `TEST`                        | `xtest`       |
//! | `XVideo`                      | `xv`          |
//! | `XVideo-MotionCompensation`   | `xvmc`        |

mod base;
mod error;
mod event;
mod ext;
mod lat1_str;

pub use base::*;
pub use error::*;
pub use event::*;
pub use ext::*;
pub use lat1_str::*;

pub mod x {
    //! The core X protocol definitions

    pub use super::xproto::*;
}

pub mod ffi {
    //! Module for Foreign Function Interface bindings.

    #![allow(non_camel_case_types)]
    #![allow(improper_ctypes)]

    pub(crate) mod base;
    pub(crate) mod ext;

    #[cfg(feature = "xlib_xcb")]
    pub(crate) mod xlib_xcb;

    pub use base::*;
    pub use ext::*;

    #[cfg(feature = "xlib_xcb")]
    pub use xlib_xcb::*;
}

#[cfg(test)]
mod test;

mod xproto {
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::new_ret_no_self)]
    #![allow(clippy::too_many_arguments)]

    /// `COPY_FROM_PARENT` can be used for many `CreateWindow` fields
    pub const COPY_FROM_PARENT: u32 = 0;

    /// `CURRENT_TIME` can be used in most requests that take a `Timestamp`
    pub const CURRENT_TIME: Timestamp = 0;

    /// `NO_SYMBOL` fills in unused entries in `Keysym` tables
    pub const NO_SYMBOL: Keysym = 0;

    /// `GRAB_ANY` can be used in various requests such as `GrabKey`, `UngrabKey`, `xinput::GrabDeviceKey`...
    pub const GRAB_ANY: Keycode = 0;

    pub const ATOM_ANY: Atom = Atom { res_id: 0 };

    /// Trait for element in a property list
    ///
    /// In events (e.g. `GetProperty::value`), it allows to assert that the format
    /// correspond to the type cast and therefore to do the cast safely at runtime.
    ///
    /// In request (e.g. `ChangeProperty::data`), it allows to infer the format value
    /// from the type of passed data.
    pub trait PropEl {
        const FORMAT: u8;
    }

    impl PropEl for u8 {
        const FORMAT: u8 = 8;
    }

    impl PropEl for u16 {
        const FORMAT: u8 = 16;
    }

    impl PropEl for u32 {
        const FORMAT: u8 = 32;
    }

    impl PropEl for Atom {
        const FORMAT: u8 = 32;
    }

    impl PropEl for Window {
        // _NET_CLIENT_LIST returns a list of windows
        const FORMAT: u8 = 32;
    }

    include!(concat!(env!("OUT_DIR"), "/xproto.rs"));
}

/// An helper macro that generate a struct of atoms.
///
/// The struct provide a constructor `intern_all` that takes a `Connection` as parameter,
/// interns all the atoms and return `xcb::Result<[struct name]>`.
/// `intern_all` takes advantage of XCB asynchronous design by sending all the
/// [`x::InternAtom`] requests before starting to wait for the first reply.
/// Fields that refer to atoms not existing in the server are set to `x::ATOM_NONE`
/// (i.e. `only_if_exists` is always set to `true`).
///
/// Both the struct and each field can receive visibility attributes.
///
/// # Example
/// ```no_run
/// # use xcb::x;
/// xcb::atoms_struct! {
///     #[derive(Copy, Clone, Debug)]
///     pub(crate) struct Atoms {
///         pub wm_protocols    => b"WM_PROTOCOLS",
///         pub wm_del_window   => b"WM_DELETE_WINDOW",
///         /// Supported EWMH hints
///         pub net_supported  => b"_NET_SUPPORTED",
///
///         // You can also explicitly set the `only_if_exists` argument when interning
///         // each atom with the following syntax (the default is `true`):
///         pub custom_atom    => b"MY_CUSTOM_ATOM" only_if_exists = false,
///     }
/// }
///
/// fn main() -> xcb::Result<()> {
/// #   let (conn, screen_num) = xcb::Connection::connect(None)?;
/// #   let window = conn.generate_id();
///     // ...
///     let atoms = Atoms::intern_all(&conn)?;
///
///     conn.check_request(conn.send_request_checked(&x::ChangeProperty {
///         mode: x::PropMode::Replace,
///         window,
///         property: atoms.wm_protocols,
///         r#type: x::ATOM_ATOM,
///         data: &[atoms.wm_del_window],
///     }))?;
///     // ...
/// #   Ok(())
/// }
/// ```
#[macro_export]
macro_rules! atoms_struct {
    (
        $(#[$outer:meta])*
        $vis:vis struct $Atoms:ident {
            $(
                $(#[$fmeta:meta])* $fvis:vis $field:ident => $name:tt $( only_if_exists = $only_if_exists:expr)?,
            )*
        }
    ) => {
        $(#[$outer])*
        $vis struct $Atoms {
            $($(#[$fmeta])* $fvis $field: xcb::x::Atom,)*
        }
        impl $Atoms {
            #[allow(dead_code)]
            pub fn intern_all(conn: &xcb::Connection) -> xcb::Result<$Atoms> {
                $(
                    #[allow(unused_assignments)]
                    let mut only_if_exists = true;
                    $( only_if_exists = $only_if_exists; )?
                    let $field = conn.send_request(&xcb::x::InternAtom {
                        only_if_exists,
                        name: $name,
                    });
                )*
                $(
                    let $field = conn.wait_for_reply($field)?.atom();
                )*
                Ok($Atoms {
                    $($field,)*
                })
            }
        }
    };
}

pub mod bigreq {
    //! The `BIG-REQUESTS` extension.
    include!(concat!(env!("OUT_DIR"), "/bigreq.rs"));
}

pub mod xc_misc {
    //! The `XC-MISC` extension.
    include!(concat!(env!("OUT_DIR"), "/xc_misc.rs"));
}

#[cfg(feature = "composite")]
pub mod composite {
    //! The `Composite` X extension.
    //!
    //! Accessible with the `composite` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/composite.rs"));
}

#[cfg(feature = "damage")]
pub mod damage {
    //! The `DAMAGE` X extension.
    //!
    //! Accessible with the `damage` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/damage.rs"));

    impl NotifyEvent {
        /// Returns the raw level/continuation byte (2nd byte of the on-wire structure).
        fn raw_level(&self) -> u8 {
            // level / continuation byte is the 2nd byte of the on-wire structure
            unsafe { std::ptr::read_unaligned(self.wire_ptr().add(1) as *const u8) }
        }

        /// Returns `true` when the server indicates that more
        /// `NotifyEvent`s belonging to the same logical notification
        /// follow this one (bit 7 of the level byte is set).
        pub fn more(&self) -> bool {
            self.raw_level() & 0x80 != 0
        }

        /// Return the `ReportLevel` of the event.
        /// Only the lower 7 bits are considered, the 8th bit is a continuation flag.
        pub fn level(&self) -> ReportLevel {
            match (self.raw_level() & 0x7F) as u32 {
                0 => ReportLevel::RawRectangles,
                1 => ReportLevel::DeltaRectangles,
                2 => ReportLevel::BoundingBox,
                3 => ReportLevel::NonEmpty,
                _ => unimplemented!(
                    "unknown damage::ReportLevel value {}",
                    self.raw_level() & 0x7F
                ),
            }
        }
    }
}

#[cfg(feature = "dpms")]
pub mod dpms {
    //! The `DPMS` X extension.
    //!
    //! Accessible with the `dpms` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/dpms.rs"));
}

#[cfg(feature = "dri2")]
pub mod dri2 {
    //! The `DRI2` X extension.
    //!
    //! Accessible with the `dri2` cargo feature.
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/dri2.rs"));
}

#[cfg(feature = "dri3")]
pub mod dri3 {
    //! The `DRI3` X extension.
    //!
    //! Accessible with the `dri3` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/dri3.rs"));
}

#[cfg(feature = "ge")]
pub mod ge {
    //! The `Generic Event Extension` X extension.
    //!
    //! Accessible with the `ge` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/ge.rs"));
}

#[cfg(feature = "glx")]
pub mod glx {
    //! The `GLX` X extension.
    //!
    //! Accessible with the `glx` cargo feature.
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/glx.rs"));
}

#[cfg(feature = "xinput")]
pub mod xinput {
    //! The `XInputExtension` X extension.
    //!
    //! Accessible with the `xinput` cargo feature.
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]

    #[derive(Copy, Clone, Debug)]
    pub enum Device {
        All,
        AllMaster,
        Id(u16),
    }

    impl Device {
        pub fn from_id(id: u16) -> Self {
            match id {
                0 => Device::All,
                1 => Device::AllMaster,
                id => Device::Id(id),
            }
        }

        pub fn id(&self) -> u16 {
            match self {
                Device::All => 0,
                Device::AllMaster => 1,
                Device::Id(id) => *id,
            }
        }
    }

    impl WiredOut for Device {
        fn wire_len(&self) -> usize {
            2
        }
        fn serialize(&self, wire_buf: &mut [u8]) -> usize {
            assert!(wire_buf.len() >= 2);
            unsafe {
                *(wire_buf.as_mut_ptr() as *mut u16) = self.id();
            }
            2
        }
    }

    impl WiredIn for Device {
        type Params = ();
        unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {
            2
        }
        unsafe fn unserialize(ptr: *const u8, params: Self::Params, offset: &mut usize) -> Self {
            *offset = 2;
            let id = *(ptr as *const u16);
            Device::from_id(id)
        }
    }

    include!(concat!(env!("OUT_DIR"), "/xinput.rs"));
}

#[cfg(feature = "present")]
pub mod present {
    //! The `Present` X extension.
    //!
    //! Accessible with the `present` cargo feature.
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/present.rs"));
}

#[cfg(feature = "randr")]
pub mod randr {
    //! The `RANDR` X extension.
    //!
    //! Accessible with the `randr` cargo feature.
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/randr.rs"));

    /// For information on what the various properties mean, see the [RandR specification][randr-spec]
    ///
    /// [randr-spec]: https://cgit.freedesktop.org/xorg/proto/randrproto/tree/randrproto.txt#n1860
    pub mod property {
        #[doc(alias = "RR_PROPERTY_BACKLIGHT")]
        pub const BACKLIGHT: &str = "Backlight";

        #[doc(alias = "RR_PROPERTY_CLONE_LIST")]
        pub const CLONE_LIST: &str = "CloneList";

        #[doc(alias = "RR_PROPERTY_COMPATIBILITY_LIST")]
        pub const COMPATIBILITY_LIST: &str = "CompatibilityList";

        #[doc(alias = "RR_PROPERTY_CONNECTOR_NUMBER")]
        pub const CONNECTOR_NUMBER: &str = "ConnectorNumber";

        #[doc(alias = "RR_PROPERTY_CONNECTOR_TYPE")]
        pub const CONNECTOR_TYPE: &str = "ConnectorType";

        #[doc(alias = "RR_PROPERTY_RANDR_EDID")]
        pub const EDID: &str = "EDID";

        #[doc(alias = "RR_PROPERTY_SIGNAL_FORMAT")]
        pub const SIGNAL_FORMAT: &str = "SignalFormat";

        #[doc(alias = "RR_PROPERTY_SIGNAL_PROPERTIES")]
        pub const SIGNAL_PROPERTIES: &str = "SignalProperties";

        #[doc(alias = "RR_PROPERTY_BORDER")]
        pub const BORDER: &str = "Border";

        #[doc(alias = "RR_PROPERTY_BORDER_DIMENSIONS")]
        pub const BORDER_DIMENSIONS: &str = "BorderDimensions";

        #[doc(alias = "RR_PROPERTY_GUID")]
        pub const GUID: &str = "GUID";

        #[doc(alias = "RR_PROPERTY_RANDR_TILE")]
        pub const TILE: &str = "TILE";

        #[doc(alias = "RR_PROPERTY_NON_DESKTOP")]
        pub const NON_DESKTOP: &str = "non-desktop";
    }
}

#[cfg(feature = "record")]
pub mod record {
    //! The `RECORD` X extension.
    //!
    //! Accessible with the `record` cargo feature.
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/record.rs"));
}

#[cfg(feature = "render")]
pub mod render {
    //! The `RENDER` X extension.
    //!
    //! Accessible with the `render` cargo feature.
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/render.rs"));
}

#[cfg(feature = "res")]
pub mod res {
    //! The `X-Resource` X extension.
    //!
    //! Accessible with the `res` cargo feature.
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/res.rs"));
}

#[cfg(feature = "screensaver")]
pub mod screensaver {
    //! The `MIT-SCREEN-SAVER` X extension.
    //!
    //! Accessible with the `screensaver` cargo feature.
    #![allow(unused_variables)]
    include!(concat!(env!("OUT_DIR"), "/screensaver.rs"));
}

#[cfg(feature = "xselinux")]
pub mod xselinux {
    //! The `SELinux` X extension.
    //!
    //! Accessible with the `xselinux` cargo feature.
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/xselinux.rs"));
}

#[cfg(feature = "shape")]
pub mod shape {
    //! The `SHAPE` X extension.
    //!
    //! Accessible with the `shape` cargo feature.
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/shape.rs"));
}

#[cfg(feature = "shm")]
pub mod shm {
    //! The `MIT-SHM` X extension.
    //!
    //! Accessible with the `shm` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/shm.rs"));
}

#[cfg(feature = "sync")]
pub mod sync {
    //! The `SYNC` X extension.
    //!
    //! Accessible with the `sync` cargo feature.
    #![allow(unused_variables)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/sync.rs"));
}

#[cfg(feature = "xtest")]
pub mod xtest {
    //! The `XTEST` X extension.
    //!
    //! Accessible with the `xtest` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/xtest.rs"));
}

#[cfg(feature = "xprint")]
pub mod xprint {
    //! The `XpExtension` X extension.
    //!
    //! Accessible with the `xprint` cargo feature.
    #![allow(clippy::unit_arg)]
    include!(concat!(env!("OUT_DIR"), "/xprint.rs"));
}

#[cfg(feature = "xevie")]
pub mod xevie {
    //! The `XEVIE` X extension.
    //!
    //! Accessible with the `xevie` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/xevie.rs"));
}

#[cfg(feature = "xf86dri")]
pub mod xf86dri {
    //! The `XFree86-DRI` X extension.
    //!
    //! Accessible with the `xf86dri` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/xf86dri.rs"));
}

#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode {
    //! The `XFree86-VidModeExtension` X extension.
    //!
    //! Accessible with the `xf86vidmode` cargo feature.
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/xf86vidmode.rs"));
}

#[cfg(feature = "xfixes")]
pub mod xfixes {
    //! The `XFIXES` X extension.
    //!
    //! Accessible with the `xfixes` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/xfixes.rs"));
}

#[cfg(feature = "xinerama")]
pub mod xinerama {
    //! The `XINERAMA` X extension.
    //!
    //! Accessible with the `xinerama` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/xinerama.rs"));
}

#[cfg(feature = "xkb")]
pub mod xkb {
    //! The `XKEYBOARD` X extension.
    //!
    //! Accessible with the `xkb` cargo feature.
    #![allow(unused_variables)]
    #![allow(clippy::let_unit_value)]
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/xkb.rs"));
}

#[cfg(feature = "xv")]
pub mod xv {
    //! The `XVideo` X extension.
    //!
    //! Accessible with the `xv` cargo feature.
    #![allow(clippy::unit_arg)]
    #![allow(clippy::too_many_arguments)]
    include!(concat!(env!("OUT_DIR"), "/xv.rs"));
}

#[cfg(feature = "xvmc")]
pub mod xvmc {
    //! The `XVideo-MotionCompensation` X extension.
    //!
    //! Accessible with the `xvmc` cargo feature.
    include!(concat!(env!("OUT_DIR"), "/xvmc.rs"));
}
