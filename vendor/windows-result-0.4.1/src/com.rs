use super::*;

#[doc(hidden)]
#[macro_export]
macro_rules! com_call {
    ($vtbl:ty, $this:ident.$method:ident($($args:tt)*)) => {
        ((&**($this.as_raw() as *mut *mut $vtbl)).$method)($this.as_raw(), $($args)*)
    }
}

#[repr(transparent)]
pub struct ComPtr(core::ptr::NonNull<core::ffi::c_void>);

impl ComPtr {
    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        unsafe { core::mem::transmute_copy(self) }
    }

    pub fn cast(&self, iid: &GUID) -> Option<Self> {
        let mut result = None;
        unsafe {
            com_call!(
                IUnknown_Vtbl,
                self.QueryInterface(iid, &mut result as *mut _ as _)
            );
        }
        result
    }
}

impl PartialEq for ComPtr {
    fn eq(&self, other: &Self) -> bool {
        self.cast(&IID_IUnknown).unwrap().0 == other.cast(&IID_IUnknown).unwrap().0
    }
}

impl Eq for ComPtr {}

impl Clone for ComPtr {
    fn clone(&self) -> Self {
        unsafe {
            com_call!(IUnknown_Vtbl, self.AddRef());
        }
        Self(self.0)
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        unsafe {
            com_call!(IUnknown_Vtbl, self.Release());
        }
    }
}
