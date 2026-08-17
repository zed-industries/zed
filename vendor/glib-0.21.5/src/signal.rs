// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `IMPL` Low level signal support.

use std::{mem, num::NonZeroU64};

use crate::{ffi, gobject_ffi};
use libc::{c_char, c_ulong, c_void};

use crate::{prelude::*, translate::*};

// rustdoc-stripper-ignore-next
/// The id of a signal that is returned by `connect`.
///
/// This type does not implement `Clone` to prevent disconnecting
/// the same signal handler multiple times.
///
/// ```ignore
/// use glib::SignalHandlerId;
/// use gtk::prelude::*;
/// use std::cell::RefCell;
///
/// struct Button {
///     widget: gtk::Button,
///     clicked_handler_id: RefCell<Option<SignalHandlerId>>,
/// }
///
/// impl Button {
///     fn new() -> Self {
///         let widget = gtk::Button::new();
///         let clicked_handler_id = RefCell::new(Some(widget.connect_clicked(|_button| {
///             // Do something.
///         })));
///         Self {
///             widget,
///             clicked_handler_id,
///         }
///     }
///
///     fn disconnect(&self) {
///         if let Some(id) = self.clicked_handler_id.take() {
///             self.widget.disconnect(id)
///         }
///     }
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct SignalHandlerId(NonZeroU64);

impl SignalHandlerId {
    // rustdoc-stripper-ignore-next
    /// Returns the internal signal handler ID.
    pub unsafe fn as_raw(&self) -> libc::c_ulong {
        self.0.get() as libc::c_ulong
    }
}

impl FromGlib<c_ulong> for SignalHandlerId {
    #[inline]
    unsafe fn from_glib(val: c_ulong) -> Self {
        debug_assert_ne!(val, 0);
        Self(NonZeroU64::new_unchecked(val as _))
    }
}

pub unsafe fn connect_raw<F>(
    receiver: *mut gobject_ffi::GObject,
    signal_name: *const c_char,
    trampoline: gobject_ffi::GCallback,
    closure: *mut F,
) -> SignalHandlerId {
    unsafe extern "C" fn destroy_closure<F>(ptr: *mut c_void, _: *mut gobject_ffi::GClosure) {
        // destroy
        let _ = Box::<F>::from_raw(ptr as *mut _);
    }
    debug_assert_eq!(mem::size_of::<*mut F>(), mem::size_of::<ffi::gpointer>());
    debug_assert!(trampoline.is_some());
    let handle = gobject_ffi::g_signal_connect_data(
        receiver,
        signal_name,
        trampoline,
        closure as *mut _,
        Some(destroy_closure::<F>),
        0,
    );
    debug_assert!(handle > 0);
    from_glib(handle)
}

#[doc(alias = "g_signal_handler_block")]
pub fn signal_handler_block<T: ObjectType>(instance: &T, handler_id: &SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_block(
            instance.as_object_ref().to_glib_none().0,
            handler_id.as_raw(),
        );
    }
}

#[doc(alias = "g_signal_handler_unblock")]
pub fn signal_handler_unblock<T: ObjectType>(instance: &T, handler_id: &SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_unblock(
            instance.as_object_ref().to_glib_none().0,
            handler_id.as_raw(),
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
#[doc(alias = "g_signal_handler_disconnect")]
pub fn signal_handler_disconnect<T: ObjectType>(instance: &T, handler_id: SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_disconnect(
            instance.as_object_ref().to_glib_none().0,
            handler_id.as_raw(),
        );
    }
}

#[doc(alias = "g_signal_stop_emission_by_name")]
pub fn signal_stop_emission_by_name<T: ObjectType>(instance: &T, signal_name: &str) {
    unsafe {
        gobject_ffi::g_signal_stop_emission_by_name(
            instance.as_object_ref().to_glib_none().0,
            signal_name.to_glib_none().0,
        );
    }
}

#[doc(alias = "g_signal_has_handler_pending")]
pub fn signal_has_handler_pending<T: ObjectType>(
    instance: &T,
    signal_id: crate::subclass::SignalId,
    detail: Option<crate::Quark>,
    may_be_blocked: bool,
) -> bool {
    unsafe {
        from_glib(gobject_ffi::g_signal_has_handler_pending(
            instance.as_object_ref().to_glib_none().0,
            signal_id.into_glib(),
            detail.map_or(0, |d| d.into_glib()),
            may_be_blocked.into_glib(),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Whether to invoke the other event handlers.
///
/// `Stop` and `Proceed` map to `GDK_EVENT_STOP` (`true`) and
/// `GDK_EVENT_PROPAGATE` (`false`), respectively.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Propagation {
    // Stop other handlers from being invoked for the event.
    #[doc(alias = "GDK_EVENT_STOP")]
    Stop,
    // Propagate the event further.
    #[doc(alias = "GDK_EVENT_PROPAGATE")]
    Proceed,
}

impl Propagation {
    // rustdoc-stripper-ignore-next
    /// Returns `true` if this is a `Stop` variant.
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if this is a `Proceed` variant.
    pub fn is_proceed(&self) -> bool {
        matches!(self, Self::Proceed)
    }
}

impl From<bool> for Propagation {
    fn from(value: bool) -> Self {
        if value {
            Self::Stop
        } else {
            Self::Proceed
        }
    }
}

impl From<Propagation> for bool {
    fn from(c: Propagation) -> Self {
        match c {
            Propagation::Stop => true,
            Propagation::Proceed => false,
        }
    }
}

#[doc(hidden)]
impl IntoGlib for Propagation {
    type GlibType = ffi::gboolean;

    #[inline]
    fn into_glib(self) -> ffi::gboolean {
        bool::from(self).into_glib()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::gboolean> for Propagation {
    #[inline]
    unsafe fn from_glib(value: ffi::gboolean) -> Self {
        bool::from_glib(value).into()
    }
}

impl crate::value::ToValue for Propagation {
    fn to_value(&self) -> crate::Value {
        bool::from(*self).to_value()
    }

    fn value_type(&self) -> crate::Type {
        <bool as StaticType>::static_type()
    }
}

impl From<Propagation> for crate::Value {
    #[inline]
    fn from(v: Propagation) -> Self {
        bool::from(v).into()
    }
}
