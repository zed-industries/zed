// Take a look at the license at the top of the repository in the LICENSE file.

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct ExitCode(u8);

impl ExitCode {
    const MIN: i32 = u8::MIN as i32;
    const MAX: i32 = u8::MAX as i32;

    // rustdoc-stripper-ignore-next
    /// The canonical ExitCode for successful termination on this platform.
    pub const SUCCESS: Self = Self(libc::EXIT_SUCCESS as u8);

    // rustdoc-stripper-ignore-next
    /// The canonical ExitCode for unsuccessful termination on this platform.
    pub const FAILURE: Self = Self(libc::EXIT_FAILURE as u8);

    pub const fn new(code: u8) -> Self {
        Self(code)
    }

    pub const fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct InvalidExitCode(i32);

impl InvalidExitCode {
    pub const fn new(code: i32) -> Option<Self> {
        match code {
            ExitCode::MIN..=ExitCode::MAX => None,
            _ => Some(Self(code)),
        }
    }

    pub const fn get(&self) -> i32 {
        self.0
    }
}

impl std::fmt::Display for InvalidExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(code) = self;
        write!(f, "{code} is not a valid glib exit code")
    }
}

impl std::error::Error for InvalidExitCode {}

impl From<u8> for ExitCode {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl TryFrom<i32> for ExitCode {
    type Error = InvalidExitCode;

    fn try_from(code: i32) -> Result<Self, Self::Error> {
        match code {
            ExitCode::MIN..=ExitCode::MAX => Ok(Self(code as _)),
            _ => Err(InvalidExitCode(code)),
        }
    }
}

macro_rules! impl_from_exit_code_0 {
    ($($ty:ty)*) => {
        $(
            impl From<ExitCode> for $ty {
                fn from(code: ExitCode) -> Self {
                    <$ty>::from(code.0)
                }
            }
        )*
    };
}

impl_from_exit_code_0! { u8 u16 u32 u64 i16 i32 i64 std::process::ExitCode }

impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        self.into()
    }
}
