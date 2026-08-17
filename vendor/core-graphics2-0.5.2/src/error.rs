#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGError {
    #[doc(alias = "kCGErrorSuccess")]
    Success           = 0,
    #[doc(alias = "kCGErrorFailure")]
    Failure           = 1000,
    #[doc(alias = "kCGErrorIllegalArgument")]
    IllegalArgument   = 1001,
    #[doc(alias = "kCGErrorInvalidConnection")]
    InvalidConnection = 1002,
    #[doc(alias = "kCGErrorInvalidContext")]
    InvalidContext    = 1003,
    #[doc(alias = "kCGErrorCannotComplete")]
    CannotComplete    = 1004,
    #[doc(alias = "kCGErrorNotImplemented")]
    NotImplemented    = 1006,
    #[doc(alias = "kCGErrorRangeCheck")]
    RangeCheck        = 1007,
    #[doc(alias = "kCGErrorTypeCheck")]
    TypeCheck         = 1008,
    #[doc(alias = "kCGErrorInvalidOperation")]
    InvalidOperation  = 1010,
    #[doc(alias = "kCGErrorNoneAvailable")]
    NoneAvailable     = 1011,
}

pub type CGErrorCallback = extern "C" fn();

extern "C" {
    pub fn CGErrorSetCallback(callback: CGErrorCallback);
}
