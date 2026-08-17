use crate::Win32::System::Rpc::RPC_STATUS;

impl RPC_STATUS {
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == 0
    }
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }
    #[inline]
    pub const fn to_hresult(self) -> windows_core::HRESULT {
        windows_core::HRESULT::from_win32(self.0 as u32)
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
impl From<RPC_STATUS> for windows_core::HRESULT {
    fn from(value: RPC_STATUS) -> Self {
        value.to_hresult()
    }
}
impl From<RPC_STATUS> for windows_core::Error {
    fn from(value: RPC_STATUS) -> Self {
        value.to_hresult().into()
    }
}
