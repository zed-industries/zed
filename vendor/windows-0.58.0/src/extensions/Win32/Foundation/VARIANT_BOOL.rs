use crate::Win32::Foundation::{VARIANT_BOOL, VARIANT_FALSE, VARIANT_TRUE};

impl VARIANT_BOOL {
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
impl From<VARIANT_BOOL> for bool {
    fn from(value: VARIANT_BOOL) -> Self {
        value.as_bool()
    }
}
impl From<&VARIANT_BOOL> for bool {
    fn from(value: &VARIANT_BOOL) -> Self {
        value.as_bool()
    }
}
impl From<bool> for VARIANT_BOOL {
    fn from(value: bool) -> Self {
        if value {
            VARIANT_TRUE
        } else {
            VARIANT_FALSE
        }
    }
}
impl From<&bool> for VARIANT_BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl PartialEq<bool> for VARIANT_BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl PartialEq<VARIANT_BOOL> for bool {
    fn eq(&self, other: &VARIANT_BOOL) -> bool {
        *self == other.as_bool()
    }
}
impl core::ops::Not for VARIANT_BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            VARIANT_FALSE
        } else {
            VARIANT_TRUE
        }
    }
}
