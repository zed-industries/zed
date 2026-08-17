#![allow(missing_docs)] // TODO
use core::ffi::{c_long, c_ulong};

dispatch_object!(
    /// Dispatch IO.
    #[doc(alias = "dispatch_io_t")]
    #[doc(alias = "dispatch_io_s")]
    pub struct DispatchIO;
);

dispatch_object_not_data!(unsafe DispatchIO);

enum_with_val! {
    #[doc(alias = "dispatch_io_type_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchIOStreamType(pub c_ulong) {
        DISPATCH_IO_STREAM = 0,
        DISPATCH_IO_RANDOM = 1,
    }
}

enum_with_val! {
    #[doc(alias = "dispatch_io_close_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchIOCloseFlags(pub c_ulong) {
        DISPATCH_IO_STOP = 0x1,
    }
}

enum_with_val! {
    #[doc(alias = "dispatch_io_interval_flags_t")]
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct DispatchIOIntervalFlags(pub c_long) {
        DISPATCH_IO_STRICT_INTERVAL = 0x1,
    }
}
