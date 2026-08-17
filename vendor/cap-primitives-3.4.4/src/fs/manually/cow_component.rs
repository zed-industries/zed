use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Component;

/// Like `std::path::Component` except we combine `Prefix` and `RootDir` since
/// we don't support absolute paths, and `Normal` has a `Cow` instead of a
/// plain `OsStr` reference, so it can optionally own its own string.
pub(super) enum CowComponent<'borrow> {
    PrefixOrRootDir,
    CurDir,
    ParentDir,
    Normal(Cow<'borrow, OsStr>),
}

impl<'borrow> CowComponent<'borrow> {
    /// Convert a `Component` into a `CowComponent` which borrows strings.
    pub(super) fn borrowed(component: Component<'borrow>) -> Self {
        match component {
            Component::Prefix(_) | Component::RootDir => Self::PrefixOrRootDir,
            Component::CurDir => Self::CurDir,
            Component::ParentDir => Self::ParentDir,
            Component::Normal(os_str) => Self::Normal(os_str.into()),
        }
    }

    /// Convert a `Component` into a `CowComponent` which owns strings.
    pub(super) fn owned(component: Component) -> Self {
        match component {
            Component::Prefix(_) | Component::RootDir => Self::PrefixOrRootDir,
            Component::CurDir => Self::CurDir,
            Component::ParentDir => Self::ParentDir,
            Component::Normal(os_str) => Self::Normal(os_str.to_os_string().into()),
        }
    }

    /// Test whether `self` is `Component::Normal`.
    #[cfg(windows)]
    pub(super) fn is_normal(&self) -> bool {
        match self {
            CowComponent::Normal(_) => true,
            _ => false,
        }
    }
}
