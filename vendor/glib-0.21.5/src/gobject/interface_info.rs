// Take a look at the license at the top of the repository in the LICENSE file.

use crate::gobject_ffi;

#[derive(Debug, Copy, Clone)]
#[doc(alias = "GInterfaceInfo")]
#[repr(transparent)]
pub struct InterfaceInfo(pub(crate) gobject_ffi::GInterfaceInfo);

impl InterfaceInfo {
    // rustdoc-stripper-ignore-next
    /// Returns a `GInterfaceInfo` pointer.
    #[doc(hidden)]
    #[inline]
    pub fn as_ptr(&self) -> *mut gobject_ffi::GInterfaceInfo {
        &self.0 as *const gobject_ffi::GInterfaceInfo as *mut _
    }

    // rustdoc-stripper-ignore-next
    /// Borrows the underlying C value mutably.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn from_glib_ptr_borrow_mut<'a>(
        ptr: *mut gobject_ffi::GInterfaceInfo,
    ) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}

impl Default for InterfaceInfo {
    // rustdoc-stripper-ignore-next
    /// Creates a new InterfaceInfo with default value.
    fn default() -> Self {
        Self(gobject_ffi::GInterfaceInfo {
            interface_init: None,
            interface_finalize: None,
            interface_data: ::std::ptr::null_mut(),
        })
    }
}
