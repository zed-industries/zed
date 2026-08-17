#[repr(transparent)]
#[derive(PartialEq, Eq)]
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
impl Clone for HCS_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for HCS_CALLBACK {}
impl core::fmt::Debug for HCS_CALLBACK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HCS_CALLBACK").field(&self.0).finish()
    }
}
impl windows_core::TypeKind for HCS_CALLBACK {
    type TypeKind = windows_core::CopyType;
}
