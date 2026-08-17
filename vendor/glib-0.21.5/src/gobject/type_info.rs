// Take a look at the license at the top of the repository in the LICENSE file.

use crate::gobject_ffi;

#[derive(Debug, Copy, Clone)]
#[doc(alias = "GTypeInfo")]
#[repr(transparent)]
pub struct TypeInfo(pub(crate) gobject_ffi::GTypeInfo);

impl TypeInfo {
    // rustdoc-stripper-ignore-next
    /// Returns a `GTypeInfo` pointer.
    #[doc(hidden)]
    #[inline]
    pub fn as_ptr(&self) -> *mut gobject_ffi::GTypeInfo {
        &self.0 as *const gobject_ffi::GTypeInfo as *mut _
    }

    // rustdoc-stripper-ignore-next
    /// Borrows the underlying C value mutably.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn from_glib_ptr_borrow_mut<'a>(ptr: *mut gobject_ffi::GTypeInfo) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}

impl Default for TypeInfo {
    // rustdoc-stripper-ignore-next
    /// Creates a new TypeInfo with default value.
    fn default() -> Self {
        Self(gobject_ffi::GTypeInfo {
            class_size: 0u16,
            base_init: None,
            base_finalize: None,
            class_init: None,
            class_finalize: None,
            class_data: ::std::ptr::null(),
            instance_size: 0,
            n_preallocs: 0,
            instance_init: None,
            value_table: ::std::ptr::null(),
        })
    }
}
