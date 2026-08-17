use bitflags::bitflags;

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum EventFilter {
    EVFILT_READ = 0,
    EVFILT_WRITE = 1,
    EVFILT_AIO = 2,
    EVFILT_VNODE = 3,
    EVFILT_PROC = 4,
    EVFILT_SIGNAL = 5,
    EVFILT_TIMER = 6,
    EVFILT_SYSCOUNT = 7,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub struct EventFlag: u32 {
        const EV_ADD       = 0x0001;
        const EV_DELETE    = 0x0002;
        const EV_ENABLE    = 0x0004;
        const EV_DISABLE   = 0x0008;
        const EV_ONESHOT   = 0x0010;
        const EV_CLEAR     = 0x0020;
        const EV_RECEIPT   = 0x0040;
        const EV_DISPATCH  = 0x0080;
        const EV_SYSFLAGS  = 0xF000;
        const EV_NODATA    = 0x1000;
        const EV_FLAG1     = 0x2000;
        const EV_EOF       = 0x8000;
        const EV_ERROR     = 0x4000;
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
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self::from_bits_retain(bits)
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub struct FilterFlag: u32 {
        const NOTE_LOWAT                           = 0x00000001;
        const NOTE_DELETE                          = 0x00000001;
        const NOTE_WRITE                           = 0x00000002;
        const NOTE_EXTEND                          = 0x00000004;
        const NOTE_ATTRIB                          = 0x00000008;
        const NOTE_LINK                            = 0x00000010;
        const NOTE_RENAME                          = 0x00000020;
        const NOTE_REVOKE                          = 0x00000040;
        const NOTE_EXIT                            = 0x80000000;
        const NOTE_FORK                            = 0x40000000;
        const NOTE_EXEC                            = 0x20000000;
        const NOTE_SIGNAL                          = 0x08000000;
        const NOTE_PDATAMASK                       = 0x000fffff;
        const NOTE_PCTRLMASK                       = 0xf0000000;
        const NOTE_TRACK                           = 0x00000001;
        const NOTE_TRACKERR                        = 0x00000002;
        const NOTE_CHILD                           = 0x00000004;
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
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self::from_bits_retain(bits)
    }
}
