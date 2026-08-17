extern crate libc;
extern crate libloading;
extern crate static_assertions;

#[cfg(all(test, unix))]
mod unix {
    use super::static_assertions::const_assert_eq;

    const_assert_eq!(libloading::os::unix::RTLD_LOCAL, libc::RTLD_LOCAL);
    const_assert_eq!(libloading::os::unix::RTLD_GLOBAL, libc::RTLD_GLOBAL);
    const_assert_eq!(libloading::os::unix::RTLD_NOW, libc::RTLD_NOW);
    const_assert_eq!(libloading::os::unix::RTLD_LAZY, libc::RTLD_LAZY);
}
