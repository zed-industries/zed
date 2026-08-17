use crate::time::SystemTime;

/// A value for specifying a time.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum SystemTimeSpec {
    /// A value which always represents the current time, in symbolic form, so
    /// that even as time elapses, it continues to represent the current time.
    SymbolicNow,

    /// An absolute time value.
    Absolute(SystemTime),
}

impl SystemTimeSpec {
    /// Constructs a new instance of `Self` from the given
    /// [`fs_set_times::SystemTimeSpec`].
    // TODO: Make this a `const fn` once `SystemTime::from_std` is a `const fn`.
    #[inline]
    pub fn from_std(std: fs_set_times::SystemTimeSpec) -> Self {
        match std {
            fs_set_times::SystemTimeSpec::SymbolicNow => Self::SymbolicNow,
            fs_set_times::SystemTimeSpec::Absolute(time) => {
                Self::Absolute(SystemTime::from_std(time))
            }
        }
    }

    /// Constructs a new instance of [`fs_set_times::SystemTimeSpec`] from the
    /// given `Self`.
    #[inline]
    pub const fn into_std(self) -> fs_set_times::SystemTimeSpec {
        match self {
            Self::SymbolicNow => fs_set_times::SystemTimeSpec::SymbolicNow,
            Self::Absolute(time) => fs_set_times::SystemTimeSpec::Absolute(time.into_std()),
        }
    }
}

impl From<SystemTime> for SystemTimeSpec {
    #[inline]
    fn from(time: SystemTime) -> Self {
        Self::Absolute(time)
    }
}
