// Take a look at the license at the top of the repository in the LICENSE file.

use crate::gobject_ffi;

#[derive(Debug, Copy, Clone)]
#[doc(alias = "GTypeValueTable")]
#[repr(transparent)]
pub struct TypeValueTable(pub(crate) gobject_ffi::GTypeValueTable);

impl TypeValueTable {
    // rustdoc-stripper-ignore-next
    /// Returns a `GTypeValueTable` pointer.
    #[doc(hidden)]
    #[inline]
    pub fn as_ptr(&self) -> *mut gobject_ffi::GTypeValueTable {
        &self.0 as *const gobject_ffi::GTypeValueTable as *mut _
    }

    // rustdoc-stripper-ignore-next
    /// Borrows the underlying C value mutably.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn from_glib_ptr_borrow_mut<'a>(
        ptr: *mut gobject_ffi::GTypeValueTable,
    ) -> &'a mut Self {
        &mut *(ptr as *mut Self)
    }
}

impl Default for TypeValueTable {
    // rustdoc-stripper-ignore-next
    /// Creates a new TypeValueTable with default value.
    fn default() -> Self {
        Self(gobject_ffi::GTypeValueTable {
            value_init: None,
            value_free: None,
            value_copy: None,
            value_peek_pointer: None,
            collect_format: ::std::ptr::null(),
            collect_value: None,
            lcopy_format: ::std::ptr::null(),
            lcopy_value: None,
        })
    }
}
