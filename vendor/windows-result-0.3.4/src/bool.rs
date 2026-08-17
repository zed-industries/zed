use super::*;

/// A 32-bit value representing boolean values and returned by some functions to indicate success or failure.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BOOL(pub i32);

impl BOOL {
    /// Converts the [`BOOL`] to a [`prim@bool`] value.
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }

    /// Converts the [`BOOL`] to [`Result<()>`][Result<_>].
    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(Error::from_win32())
        }
    }

    /// Asserts that `self` is a success code.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }

    /// Asserts that `self` is a success code using the given panic message.
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}

impl From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.as_bool()
    }
}

impl From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}

impl From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}

impl From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}

impl PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

impl PartialEq<BOOL> for bool {
    fn eq(&self, other: &BOOL) -> bool {
        *self == other.as_bool()
    }
}

impl core::ops::Not for BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            Self(0)
        } else {
            Self(1)
        }
    }
}
