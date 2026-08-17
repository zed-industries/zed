use std::os::raw::c_int;

/// Perform lazy binding.
///
/// Relocations shall be performed at an implementation-defined time, ranging from the time
/// of the [`Library::open`] call until the first reference to a given symbol occurs.
/// Specifying `RTLD_LAZY` should improve performance on implementations supporting dynamic
/// symbol binding since a process might not reference all of the symbols in an executable
/// object file. And, for systems supporting dynamic symbol resolution for normal process
/// execution, this behaviour mimics the normal handling of process execution.
///
/// Conflicts with [`RTLD_NOW`].
///
/// [`Library::open`]: crate::os::unix::Library::open
pub const RTLD_LAZY: c_int = posix::RTLD_LAZY;

/// Perform eager binding.
///
/// All necessary relocations shall be performed when the executable object file is first
/// loaded. This may waste some processing if relocations are performed for symbols
/// that are never referenced. This behaviour may be useful for applications that need to
/// know that all symbols referenced during execution will be available before
/// [`Library::open`] returns.
///
/// Conflicts with [`RTLD_LAZY`].
///
/// [`Library::open`]: crate::os::unix::Library::open
pub const RTLD_NOW: c_int = posix::RTLD_NOW;

/// Make loaded symbols available for resolution globally.
///
/// The executable object file's symbols shall be made available for relocation processing of any
/// other executable object file. In addition, calls to [`Library::get`] on `Library` obtained from
/// [`Library::this`] allows executable object files loaded with this mode to be searched.
///
/// [`Library::this`]: crate::os::unix::Library::this
/// [`Library::get`]: crate::os::unix::Library::get
pub const RTLD_GLOBAL: c_int = posix::RTLD_GLOBAL;

/// Load symbols into an isolated namespace.
///
/// The executable object file's symbols shall not be made available for relocation processing of
/// any other executable object file. This mode of operation is most appropriate for e.g. plugins.
pub const RTLD_LOCAL: c_int = posix::RTLD_LOCAL;

#[cfg(all(libloading_docs, not(unix)))]
mod posix {
    use super::c_int;
    pub(super) const RTLD_LAZY: c_int = !0;
    pub(super) const RTLD_NOW: c_int = !0;
    pub(super) const RTLD_GLOBAL: c_int = !0;
    pub(super) const RTLD_LOCAL: c_int = !0;
}

#[cfg(any(not(libloading_docs), unix))]
mod posix {
    extern crate cfg_if;
    use self::cfg_if::cfg_if;
    use super::c_int;
    cfg_if! {
        if #[cfg(target_os = "haiku")] {
            pub(super) const RTLD_LAZY: c_int = 0;
        } else if #[cfg(target_os = "aix")] {
            pub(super) const RTLD_LAZY: c_int = 4;
        } else if #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "emscripten",

            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos",
            target_os = "watchos",

            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd",

            target_os = "solaris",
            target_os = "illumos",

            target_env = "uclibc",
            target_env = "newlib",

            target_os = "fuchsia",
            target_os = "redox",
            target_os = "nto",
            target_os = "hurd",
            target_os = "cygwin",
        ))] {
            pub(super) const RTLD_LAZY: c_int = 1;
        } else {
            compile_error!(
                "Target has no known `RTLD_LAZY` value. Please submit an issue or PR adding it."
            );
        }
    }

    cfg_if! {
        if #[cfg(target_os = "haiku")] {
            pub(super) const RTLD_NOW: c_int = 1;
        } else if #[cfg(any(
            target_os = "linux",
            all(target_os = "android", target_pointer_width = "64"),
            target_os = "emscripten",

            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos",
            target_os = "watchos",

            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd",

            target_os = "aix",
            target_os = "solaris",
            target_os = "illumos",

            target_env = "uclibc",
            target_env = "newlib",

            target_os = "fuchsia",
            target_os = "redox",
            target_os = "nto",
            target_os = "hurd",
            target_os = "cygwin",
        ))] {
            pub(super) const RTLD_NOW: c_int = 2;
        } else if #[cfg(all(target_os = "android",target_pointer_width = "32"))] {
            pub(super) const RTLD_NOW: c_int = 0;
        } else {
            compile_error!(
                "Target has no known `RTLD_NOW` value. Please submit an issue or PR adding it."
            );
        }
    }

    cfg_if! {
        if #[cfg(any(
            target_os = "haiku",
            all(target_os = "android",target_pointer_width = "32"),
        ))] {
            pub(super) const RTLD_GLOBAL: c_int = 2;
        } else if #[cfg(target_os = "aix")] {
            pub(super) const RTLD_GLOBAL: c_int = 0x10000;
        } else if #[cfg(any(
            target_env = "uclibc",
            all(target_os = "linux", target_arch = "mips"),
            all(target_os = "linux", target_arch = "mips64"),
            target_os = "cygwin",
        ))] {
            pub(super) const RTLD_GLOBAL: c_int = 4;
        } else if #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos",
            target_os = "watchos",
        ))] {
            pub(super) const RTLD_GLOBAL: c_int = 8;
        } else if #[cfg(any(
            target_os = "linux",
            all(target_os = "android", target_pointer_width = "64"),
            target_os = "emscripten",

            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd",

            target_os = "solaris",
            target_os = "illumos",

            target_env = "newlib",

            target_os = "fuchsia",
            target_os = "redox",
            target_os = "nto",
            target_os = "hurd",
        ))] {
            pub(super) const RTLD_GLOBAL: c_int = 0x100;
        } else {
            compile_error!(
                "Target has no known `RTLD_GLOBAL` value. Please submit an issue or PR adding it."
            );
        }
    }

    cfg_if! {
        if #[cfg(any(
           target_os = "netbsd",
           target_os = "nto",
        ))] {
            pub(super) const RTLD_LOCAL: c_int = 0x200;
        } else if #[cfg(target_os = "aix")] {
            pub(super) const RTLD_LOCAL: c_int = 0x80000;
        } else if #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos",
            target_os = "watchos",
        ))] {
            pub(super) const RTLD_LOCAL: c_int = 4;
        } else if #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "emscripten",

            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",

            target_os = "haiku",

            target_os = "solaris",
            target_os = "illumos",

            target_env = "uclibc",
            target_env = "newlib",

            target_os = "fuchsia",
            target_os = "redox",
            target_os = "hurd",
            target_os = "cygwin",
        ))] {
            pub(super) const RTLD_LOCAL: c_int = 0;
        } else {
            compile_error!(
                "Target has no known `RTLD_LOCAL` value. Please submit an issue or PR adding it."
            );
        }
    }
}

// Other constants that exist but are not bound because they are platform-specific (non-posix)
// extensions. Some of these constants are only relevant to `dlsym` or `dlmopen` calls.
//
// RTLD_CONFGEN
// RTLD_DEFAULT
// RTLD_DI_CONFIGADDR
// RTLD_DI_LINKMAP
// RTLD_DI_LMID
// RTLD_DI_ORIGIN
// RTLD_DI_PROFILENAME
// RTLD_DI_PROFILEOUT
// RTLD_DI_SERINFO
// RTLD_DI_SERINFOSIZE
// RTLD_DI_TLS_DATA
// RTLD_DI_TLS_MODID
// RTLD_FIRST
// RTLD_GROUP
// RTLD_NEXT
// RTLD_PARENT
// RTLD_PROBE
// RTLD_SELF
// RTLD_WORLD
// RTLD_NODELETE
// RTLD_NOLOAD
// RTLD_DEEPBIND
