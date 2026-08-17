use crate::Win32::Foundation::WIN32_ERROR;

impl WIN32_ERROR {
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
        windows_core::HRESULT::from_win32(self.0)
    }
    #[inline]
    pub fn from_error(error: &windows_core::Error) -> Option<Self> {
        let hresult = error.code().0 as u32;
        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(Self(hresult & 0xFFFF))
        } else {
            None
        }
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
impl From<WIN32_ERROR> for windows_core::HRESULT {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult()
    }
}
impl From<WIN32_ERROR> for windows_core::Error {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult().into()
    }
}
