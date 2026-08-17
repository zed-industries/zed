#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCS_CALLBACK(pub *mut core::ffi::c_void);
impl HCS_CALLBACK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HCS_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HCS_CALLBACK {
    type TypeKind = windows_core::CopyType;
}
