// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, num::NonZeroU32};

use crate::{ffi, translate::*, GStr};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[repr(transparent)]
#[doc(alias = "GQuark")]
pub struct Quark(NonZeroU32);

impl Quark {
    #[doc(alias = "g_quark_from_string")]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: impl IntoGStr) -> Self {
        unsafe { s.run_with_gstr(|s| from_glib(ffi::g_quark_from_string(s.as_ptr()))) }
    }

    #[doc(alias = "g_quark_from_static_string")]
    #[allow(clippy::should_implement_trait)]
    pub fn from_static_str(s: &'static GStr) -> Self {
        unsafe { from_glib(ffi::g_quark_from_static_string(s.as_ptr())) }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[doc(alias = "g_quark_to_string")]
    pub fn as_str<'a>(&self) -> &'a GStr {
        unsafe { GStr::from_ptr(ffi::g_quark_to_string(self.into_glib())) }
    }

    #[doc(alias = "g_quark_try_string")]
    pub fn try_from_str(s: &str) -> Option<Self> {
        unsafe { Self::try_from_glib(ffi::g_quark_try_string(s.to_glib_none().0)).ok() }
    }
}

impl fmt::Debug for Quark {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(Quark::as_str(self))
    }
}

impl<T: IntoGStr> From<T> for Quark {
    fn from(s: T) -> Self {
        Self::from_str(s)
    }
}

impl std::str::FromStr for Quark {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str(s))
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GQuark> for Quark {
    #[inline]
    unsafe fn from_glib(value: ffi::GQuark) -> Self {
        debug_assert_ne!(value, 0);
        Self(NonZeroU32::new_unchecked(value))
    }
}

#[doc(hidden)]
impl TryFromGlib<ffi::GQuark> for Quark {
    type Error = GlibNoneError;
    unsafe fn try_from_glib(value: ffi::GQuark) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(GlibNoneError)
        } else {
            Ok(Self(NonZeroU32::new_unchecked(value)))
        }
    }
}

#[doc(hidden)]
impl IntoGlib for Quark {
    type GlibType = ffi::GQuark;

    #[inline]
    fn into_glib(self) -> ffi::GQuark {
        self.0.get()
    }
}

#[doc(hidden)]
impl IntoGlib for Option<Quark> {
    type GlibType = ffi::GQuark;

    #[inline]
    fn into_glib(self) -> ffi::GQuark {
        self.map(|s| s.0.get()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let q1 = Quark::from_str("some-quark");
        let q2 = Quark::try_from_str("some-quark");
        assert_eq!(Some(q1), q2);
        assert_eq!(q1.as_str(), "some-quark");
    }
}
