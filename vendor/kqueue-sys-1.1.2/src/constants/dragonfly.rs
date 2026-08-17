use bitflags::bitflags;
use libc::{c_uint, c_ushort};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(i16)]
pub enum EventFilter {
    EVFILT_READ = -1,
    EVFILT_WRITE = -2,
    EVFILT_AIO = -3,
    EVFILT_VNODE = -4,
    EVFILT_PROC = -5,
    EVFILT_SIGNAL = -6,
    EVFILT_TIMER = -7,
    EVFILT_EXCEPT = -8,
    EVFILT_USER = -9,
    EVFILT_FS = -10,
    EVFILT_SYSCOUNT = 10,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub struct EventFlag: c_ushort {
        const EV_ADD      = 0x0001;
        const EV_DELETE   = 0x0002;
        const EV_ENABLE   = 0x0004;
        const EV_DISABLE  = 0x0008;
        const EV_ONESHOT  = 0x0010;
        const EV_CLEAR    = 0x0020;
        const EV_RECEIPT  = 0x0040;
        const EV_DISPATCH = 0x0080;
        const EV_SYSFLAGS = 0xF000;
        const EV_FLAG1    = 0x2000;
        const EV_EOF      = 0x8000;
        const EV_ERROR    = 0x4000;
        const EV_NODATA   = 0x1000;
    }
}

impl EventFlag {
    /// Convert from underlying bit representation, preserving all
    /// bits (even those not corresponding to a defined flag).
    ///
    /// # Safety
    ///
    /// The caller of the `bitflags!` macro can chose to allow or
    /// disallow extra bits for their bitflags type.
    ///
    /// The caller of `from_bits_unchecked()` has to ensure that
    /// all bits correspond to a defined flag or that extra bits
    /// are valid for this bitflags type.
    #[deprecated = "use the safe `from_bits_retain` method instead"]
    pub const unsafe fn from_bits_unchecked(bits: c_ushort) -> Self {
        Self::from_bits_retain(bits)
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub struct FilterFlag: c_uint {
        const NOTE_FFNOP      = 0x00000000;
        const NOTE_FFAND      = 0x40000000;
        const NOTE_FFOR       = 0x80000000;
        const NOTE_FFCOPY     = 0xc0000000;
        const NOTE_FFCTRLMASK = 0xc0000000;
        const NOTE_FFLAGSMASK = 0x00ffffff;
        const NOTE_TRIGGER    = 0x01000000;
        const NOTE_LOWAT      = 0x00000001;
        const NOTE_OOB        = 0x00000002;
        const NOTE_DELETE     = 0x00000001;
        const NOTE_WRITE      = 0x00000002;
        const NOTE_EXTEND     = 0x00000004;
        const NOTE_ATTRIB     = 0x00000008;
        const NOTE_LINK       = 0x00000010;
        const NOTE_RENAME     = 0x00000020;
        const NOTE_REVOKE     = 0x00000040;
        const NOTE_EXIT       = 0x80000000;
        const NOTE_FORK       = 0x40000000;
        const NOTE_EXEC       = 0x20000000;
        const NOTE_SIGNAL     = 0x08000000;
        const NOTE_PCTRLMASK  = 0xf0000000;
        const NOTE_PDATAMASK  = 0x000fffff;
        const NOTE_TRACK      = 0x00000001;
        const NOTE_TRACKERR   = 0x00000002;
        const NOTE_CHILD      = 0x00000004;
    }
}

impl FilterFlag {
    /// Convert from underlying bit representation, preserving all
    /// bits (even those not corresponding to a defined flag).
    ///
    /// # Safety
    ///
    /// The caller of the `bitflags!` macro can chose to allow or
    /// disallow extra bits for their bitflags type.
    ///
    /// The caller of `from_bits_unchecked()` has to ensure that
    /// all bits correspond to a defined flag or that extra bits
    /// are valid for this bitflags type.
    #[deprecated = "use the safe `from_bits_retain` method instead"]
    pub const unsafe fn from_bits_unchecked(bits: c_uint) -> Self {
        Self::from_bits_retain(bits)
    }
}
