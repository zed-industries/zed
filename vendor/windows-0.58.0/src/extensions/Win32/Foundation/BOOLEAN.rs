use crate::Win32::Foundation::BOOLEAN;

impl BOOLEAN {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    #[inline]
    pub fn ok(self) -> windows_core::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(windows_core::Error::from_win32())
        }
    }
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl From<BOOLEAN> for bool {
    fn from(value: BOOLEAN) -> Self {
        value.as_bool()
    }
}
impl From<&BOOLEAN> for bool {
    fn from(value: &BOOLEAN) -> Self {
        value.as_bool()
    }
}
impl From<bool> for BOOLEAN {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}
impl From<&bool> for BOOLEAN {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl PartialEq<bool> for BOOLEAN {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl PartialEq<BOOLEAN> for bool {
    fn eq(&self, other: &BOOLEAN) -> bool {
        *self == other.as_bool()
    }
}
impl core::ops::Not for BOOLEAN {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            Self(0)
        } else {
            Self(1)
        }
    }
}
impl windows_core::Param<BOOLEAN> for bool {
    unsafe fn param(self) -> windows_core::ParamValue<BOOLEAN> {
        windows_core::ParamValue::Owned(self.into())
    }
}
