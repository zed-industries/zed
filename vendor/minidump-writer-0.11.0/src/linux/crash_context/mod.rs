//! Minidump defines register structures which are different from the raw
//! structures which we get from the kernel. These are platform specific
//! functions to juggle the `ucontext_t` and user structures into minidump format.

pub struct CrashContext {
    pub inner: crash_context::CrashContext,
}

impl std::fmt::Debug for CrashContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrashContext")
            .field("siginfo", &self.inner.siginfo)
            .field("pid", &self.inner.pid)
            .field("tid", &self.inner.tid)
            .finish_non_exhaustive()
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
    } else if #[cfg(target_arch = "x86")] {
        mod x86;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
    } else if #[cfg(target_arch = "arm")] {
        mod arm;
    }
}
