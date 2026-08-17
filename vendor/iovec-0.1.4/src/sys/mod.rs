#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::{
    IoVec,
    MAX_LENGTH,
};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::{
    IoVec,
    MAX_LENGTH,
};

#[cfg(not(any(windows, unix)))]
mod unknown;

#[cfg(not(any(windows, unix)))]
pub use self::unknown::{
    IoVec,
    MAX_LENGTH,
};
