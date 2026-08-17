use std::io;

#[cfg(any(target_os = "android", target_os = "linux"))]
#[cold]
pub(crate) fn invalid_flags() -> io::Error {
    rustix::io::Errno::INVAL.into()
}

#[cold]
pub(crate) fn no_such_file_or_directory() -> io::Error {
    rustix::io::Errno::NOENT.into()
}

#[cold]
pub(crate) fn is_directory() -> io::Error {
    rustix::io::Errno::ISDIR.into()
}

#[cold]
pub(crate) fn is_not_directory() -> io::Error {
    rustix::io::Errno::NOTDIR.into()
}

#[cold]
pub(crate) fn too_many_symlinks() -> io::Error {
    rustix::io::Errno::LOOP.into()
}
