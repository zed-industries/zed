#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCS_CALLBACK(pub isize);
impl HCS_CALLBACK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
