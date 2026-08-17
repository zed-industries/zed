cfg_if! {
    if #[cfg(miri)] {
        mod fallback;
        pub use fallback::guess_os_stack_limit;
    } else if #[cfg(windows)] {
        pub(crate) mod windows;
        pub use windows::guess_os_stack_limit;
    } else if #[cfg(any(
        target_os = "linux",
        target_os = "solaris",
        target_os = "netbsd",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "haiku",
        target_os = "illumos"
    ))] {
        mod unix;
        pub use unix::guess_os_stack_limit;
    } else if #[cfg(target_os = "openbsd")] {
        mod openbsd;
        pub use openbsd::guess_os_stack_limit;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub use macos::guess_os_stack_limit;
    } else {
        mod fallback;
        pub use fallback::guess_os_stack_limit;
    }
}
