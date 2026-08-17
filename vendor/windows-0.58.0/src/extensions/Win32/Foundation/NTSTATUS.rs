use crate::Win32::Foundation::NTSTATUS;

impl NTSTATUS {
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 >= 0
    }
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }
    #[inline]
    pub const fn to_hresult(self) -> windows_core::HRESULT {
        windows_core::HRESULT::from_nt(self.0)
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.to_hresult().into())
        }
    }
}
impl From<NTSTATUS> for windows_core::HRESULT {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult()
    }
}
impl From<NTSTATUS> for windows_core::Error {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult().into()
    }
}
