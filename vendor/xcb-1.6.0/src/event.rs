use crate::base::{BaseEvent, Raw, ResolveWireEvent, ResolveWireGeEvent};
use crate::ext::{Extension, ExtensionData};
use crate::ffi::*;
use crate::x;

#[cfg(feature = "damage")]
use crate::damage;

#[cfg(feature = "dri2")]
use crate::dri2;

#[cfg(feature = "glx")]
use crate::glx;

#[cfg(feature = "present")]
use crate::present;

#[cfg(feature = "randr")]
use crate::randr;

#[cfg(feature = "screensaver")]
use crate::screensaver;

#[cfg(feature = "shape")]
use crate::shape;

#[cfg(feature = "shm")]
use crate::shm;

#[cfg(feature = "sync")]
use crate::sync;

#[cfg(feature = "xfixes")]
use crate::xfixes;

#[cfg(feature = "xinput")]
use crate::xinput;

#[cfg(feature = "xkb")]
use crate::xkb;

#[cfg(feature = "xprint")]
use crate::xprint;

#[cfg(feature = "xv")]
use crate::xv;

/// Unified Event type from the X server.
#[derive(Debug)]
pub enum Event {
    /// The event is issued from the X core protocol.
    X(x::Event),

    #[cfg(feature = "damage")]
    /// The event is issued from the `DAMAGE` extension.
    Damage(damage::Event),

    #[cfg(feature = "dri2")]
    /// The event is issued from the `DRI2` extension.
    Dri2(dri2::Event),

    #[cfg(feature = "glx")]
    /// The event is issued from the `GLX` extension.
    Glx(glx::Event),

    #[cfg(feature = "present")]
    /// The event is issued from the `Present` extension.
    Present(present::Event),

    #[cfg(feature = "randr")]
    /// The event is issued from the `RANDR` extension.
    RandR(randr::Event),

    #[cfg(feature = "screensaver")]
    /// The event is issued from the `MIT-SCREEN-SAVER` extension.
    ScreenSaver(screensaver::Event),

    #[cfg(feature = "shape")]
    /// The event is issued from the `SHAPE` extension.
    Shape(shape::Event),

    #[cfg(feature = "shm")]
    /// The event is issued from the `MIT-SHM` extension.
    Shm(shm::Event),

    #[cfg(feature = "sync")]
    /// The event is issued from the `SYNC` extension.
    Sync(sync::Event),

    #[cfg(feature = "xfixes")]
    /// The event is issued from the `XFIXES` extension.
    XFixes(xfixes::Event),

    #[cfg(feature = "xinput")]
    /// The event is issued from the `XInputExtension` extension.
    Input(xinput::Event),

    #[cfg(feature = "xkb")]
    /// The event is issued from the `XKEYBOARD` extension.
    Xkb(xkb::Event),

    #[cfg(feature = "xprint")]
    /// The event is issued from the `XpExtension` extension.
    XPrint(xprint::Event),

    #[cfg(feature = "xv")]
    /// The event is issued from the `XVideo` extension.
    Xv(xv::Event),

    /// The event was not recognized, it was likely issued from a disabled extension.
    Unknown(UnknownEvent),
}

impl Event {
    pub fn as_raw(&self) -> *mut xcb_generic_event_t {
        match self {
            Self::X(e) => e.as_raw(),
            #[cfg(feature = "damage")]
            Self::Damage(e) => e.as_raw(),
            #[cfg(feature = "dri2")]
            Self::Dri2(e) => e.as_raw(),
            #[cfg(feature = "glx")]
            Self::Glx(e) => e.as_raw(),
            #[cfg(feature = "present")]
            Self::Present(e) => e.as_raw(),
            #[cfg(feature = "randr")]
            Self::RandR(e) => e.as_raw(),
            #[cfg(feature = "screensaver")]
            Self::ScreenSaver(e) => e.as_raw(),
            #[cfg(feature = "shape")]
            Self::Shape(e) => e.as_raw(),
            #[cfg(feature = "shm")]
            Self::Shm(e) => e.as_raw(),
            #[cfg(feature = "sync")]
            Self::Sync(e) => e.as_raw(),
            #[cfg(feature = "xfixes")]
            Self::XFixes(e) => e.as_raw(),
            #[cfg(feature = "xinput")]
            Self::Input(e) => e.as_raw(),
            #[cfg(feature = "xkb")]
            Self::Xkb(e) => e.as_raw(),
            #[cfg(feature = "xprint")]
            Self::XPrint(e) => e.as_raw(),
            #[cfg(feature = "xv")]
            Self::Xv(e) => e.as_raw(),
            Self::Unknown(e) => e.as_raw(),
        }
    }
}

/// an event was not recognized as part of the core protocol or any enabled extension
pub struct UnknownEvent {
    raw: *mut xcb_generic_event_t,
}

impl Raw<xcb_generic_event_t> for UnknownEvent {
    unsafe fn from_raw(raw: *mut xcb_generic_event_t) -> Self {
        UnknownEvent { raw }
    }

    fn as_raw(&self) -> *mut xcb_generic_event_t {
        self.raw
    }
}

impl BaseEvent for UnknownEvent {
    const EXTENSION: Option<Extension> = None;
    const NUMBER: u32 = u32::MAX;
}

impl UnknownEvent {
    pub fn response_type(&self) -> u8 {
        unsafe { (*self.raw).response_type }
    }
    pub fn sequence(&self) -> u16 {
        unsafe { (*self.raw).sequence }
    }
    pub fn full_sequence(&self) -> u32 {
        unsafe { (*self.raw).full_sequence }
    }
}

unsafe impl Send for UnknownEvent {}
unsafe impl Sync for UnknownEvent {}

impl std::fmt::Debug for UnknownEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UnknownEvent").finish()
    }
}

impl Drop for UnknownEvent {
    fn drop(&mut self) {
        unsafe { libc::free(self.raw as *mut _) }
    }
}

/// Resolve an event from the X server
///
/// If the event originates from an extension, the `extension_data` parameter must contain the
/// data for the extension of origin.
/// If the resolution fails, an `Event::Unknown` is returned.
///
/// # Panics
/// The function will panic if the matches with an `XCB_GE_GENERIC` event, but can't
/// be resolved as such.
///
/// # Safety
/// The caller must ensure that `event` is a valid pointer to an `xcb_generic_event_t`.
pub unsafe fn resolve_event(
    event: *mut xcb_generic_event_t,
    extension_data: &[ExtensionData],
) -> Event {
    let response_type = (*event).response_type & 0x7F;

    if response_type == XCB_GE_GENERIC {
        let event = event as *mut xcb_ge_generic_event_t;
        let extension = (*event).extension;
        for ext in extension_data {
            if ext.major_opcode == extension {
                match ext.ext {
                    #[cfg(feature = "present")]
                    Extension::Present => {
                        return Event::Present(present::Event::resolve_wire_ge_event(event));
                    }
                    #[cfg(feature = "xinput")]
                    Extension::Input => {
                        return Event::Input(xinput::Event::resolve_wire_ge_event(event));
                    }
                    _ => panic!("could not resolve Generic Event extension"),
                }
            }
        }
        return Event::Unknown(UnknownEvent::from_raw(event as *mut _));
    }

    for data in extension_data {
        if response_type >= data.first_event && data.first_event != 0 {
            match data.ext {
                #[cfg(feature = "damage")]
                Extension::Damage => {
                    return Event::Damage(
                        damage::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "dri2")]
                Extension::Dri2 => {
                    return Event::Dri2(
                        dri2::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "glx")]
                Extension::Glx => {
                    return Event::Glx(
                        glx::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "present")]
                Extension::Present => {
                    return Event::Present(
                        present::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "randr")]
                Extension::RandR => {
                    return Event::RandR(
                        randr::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "screensaver")]
                Extension::ScreenSaver => {
                    return Event::ScreenSaver(
                        screensaver::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "shape")]
                Extension::Shape => {
                    return Event::Shape(
                        shape::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "shm")]
                Extension::Shm => {
                    return Event::Shm(
                        shm::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "sync")]
                Extension::Sync => {
                    return Event::Sync(
                        sync::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "xfixes")]
                Extension::XFixes => {
                    return Event::XFixes(
                        xfixes::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "xinput")]
                Extension::Input => {
                    return Event::Input(
                        xinput::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "xkb")]
                Extension::Xkb => {
                    return Event::Xkb(
                        xkb::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "xprint")]
                Extension::XPrint => {
                    return Event::XPrint(
                        xprint::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                #[cfg(feature = "xv")]
                Extension::Xv => {
                    return Event::Xv(
                        xv::Event::resolve_wire_event(data.first_event, event).unwrap(),
                    );
                }

                _ => {}
            }
        }
    }

    x::Event::resolve_wire_event(0, event)
        .map(Event::X)
        .unwrap_or_else(|| {
            // SAFETY the event type is checked above and the function panicked if it was
            // not a basic event (XCB_GE_GENERIC)
            let unknown = UnknownEvent::from_raw(event);
            Event::Unknown(unknown)
        })
}

#[test]
fn test_event_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<Event>();
    assert_sync::<Event>();
}
