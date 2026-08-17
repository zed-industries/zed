// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, prelude::*, translate::*};

// rustdoc-stripper-ignore-next
/// Continue calling the closure in the future iterations or drop it.
///
/// This is the return type of `idle_add` and `timeout_add` closures.
///
/// `ControlFlow::Continue` keeps the closure assigned, to be rerun when appropriate.
///
/// `ControlFlow::Break` disconnects and drops it.
///
/// `Continue` and `Break` map to `G_SOURCE_CONTINUE` (`true`) and
/// `G_SOURCE_REMOVE` (`false`), respectively.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ControlFlow {
    #[doc(alias = "G_SOURCE_CONTINUE")]
    Continue,
    #[doc(alias = "G_SOURCE_REMOVE")]
    Break,
}

impl ControlFlow {
    // rustdoc-stripper-ignore-next
    /// Returns `true` if this is a `Continue` variant.
    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }

    // rustdoc-stripper-ignore-next
    /// Returns `true` if this is a `Break` variant.
    pub fn is_break(&self) -> bool {
        matches!(self, Self::Break)
    }
}

impl From<std::ops::ControlFlow<()>> for ControlFlow {
    fn from(c: std::ops::ControlFlow<()>) -> Self {
        match c {
            std::ops::ControlFlow::Break(_) => Self::Break,
            std::ops::ControlFlow::Continue(_) => Self::Continue,
        }
    }
}

impl From<ControlFlow> for std::ops::ControlFlow<()> {
    fn from(c: ControlFlow) -> Self {
        match c {
            ControlFlow::Break => Self::Break(()),
            ControlFlow::Continue => Self::Continue(()),
        }
    }
}

impl From<bool> for ControlFlow {
    fn from(c: bool) -> Self {
        if c {
            Self::Continue
        } else {
            Self::Break
        }
    }
}

impl From<ControlFlow> for bool {
    fn from(c: ControlFlow) -> Self {
        match c {
            ControlFlow::Break => false,
            ControlFlow::Continue => true,
        }
    }
}

#[doc(hidden)]
impl IntoGlib for ControlFlow {
    type GlibType = ffi::gboolean;

    #[inline]
    fn into_glib(self) -> ffi::gboolean {
        bool::from(self).into_glib()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::gboolean> for ControlFlow {
    #[inline]
    unsafe fn from_glib(value: ffi::gboolean) -> Self {
        bool::from_glib(value).into()
    }
}

impl crate::value::ToValue for ControlFlow {
    fn to_value(&self) -> crate::Value {
        bool::from(*self).to_value()
    }

    fn value_type(&self) -> crate::Type {
        <bool as StaticType>::static_type()
    }
}

impl From<ControlFlow> for crate::Value {
    #[inline]
    fn from(v: ControlFlow) -> Self {
        bool::from(v).into()
    }
}
