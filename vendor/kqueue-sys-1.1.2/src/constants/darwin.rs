use bitflags::bitflags;
use libc::{c_uint, c_ushort};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(i16)]
pub enum EventFilter {
    EVFILT_READ = -1,
    EVFILT_WRITE = -2,
    EVFILT_AIO = -3,      /* attached to aio requests */
    EVFILT_VNODE = -4,    /* attached to vnodes */
    EVFILT_PROC = -5,     /* attached to struct proc */
    EVFILT_SIGNAL = -6,   /* attached to struct proc */
    EVFILT_TIMER = -7,    /* timers */
    EVFILT_MACHPORT = -8, /* Mach portsets */
    EVFILT_FS = -9,       /* Filesystem events */
    EVFILT_USER = -10,    /* User events */
    EVFILT_VM = -12,      /* Virtual memory events */
    EVFILT_SYSCOUNT = 14,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub struct EventFlag: c_ushort {
        const EV_ADD            = 0x0001;   /* add event to kq (implies enable) */
        const EV_DELETE         = 0x0002;   /* delete event from kq */
        const EV_ENABLE         = 0x0004;   /* enable event */
        const EV_DISABLE        = 0x0008;   /* disable event (not reported) */
        const EV_UDATA_SPECIFIC = 0x0100;   /* unique kevent per udata value */
                                            /* ... in combination with EV_DELETE */
                                            /* will defer delete until udata-specific */
                                            /* event enabled. EINPROGRESS will be */
                                            /* returned to indicate the deferral */

        const EV_ONESHOT        = 0x0010;   /* only report one occurrence */
        const EV_CLEAR          = 0x0020;   /* clear event state after reporting */
        const EV_RECEIPT        = 0x0040;   /* force EV_ERROR on success, data == 0 */
        const EV_DISPATCH       = 0x0080;   /* disable event after reporting */

        const EV_SYSFLAGS       = 0xF000;   /* reserved by system */
        const EV_FLAG0          = 0x1000;   /* filter-specific flag */
        const EV_FLAG1          = 0x2000;   /* filter-specific flag */
        const EV_EOF            = 0x8000;   /* EOF detected */
        const EV_ERROR          = 0x4000;   /* error, data contains errno */
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
        const NOTE_FFNOP                        = 0x00000000;   /* ignore input fflags */
        const NOTE_FFAND                        = 0x40000000;   /* and fflags */
        const NOTE_FFOR                         = 0x80000000;   /* or fflags */
        const NOTE_FFCOPY                       = 0xc0000000;   /* copy fflags */
        const NOTE_FFCTRLMASK                   = 0xc0000000;   /* mask for operations */
        const NOTE_FFLAGSMASK                   = 0x00ffffff;
        const NOTE_LOWAT                        = 0x00000001;   /* low water mark */
        const NOTE_DELETE                       = 0x00000001;   /* vnode was removed */
        const NOTE_WRITE                        = 0x00000002;   /* data contents changed */
        const NOTE_EXTEND                       = 0x00000004;   /* size increased */
        const NOTE_ATTRIB                       = 0x00000008;   /* attributes changed */
        const NOTE_LINK                         = 0x00000010;   /* link count changed */
        const NOTE_RENAME                       = 0x00000020;   /* vnode was renamed */
        const NOTE_REVOKE                       = 0x00000040;   /* vnode access was revoked */
        const NOTE_NONE                         = 0x00000080;   /* No specific vnode event: to test for EVFILT_READ activation*/
        const NOTE_EXIT                         = 0x80000000;   /* process exited */
        const NOTE_FORK                         = 0x40000000;   /* process forked */
        const NOTE_EXEC                         = 0x20000000;   /* process exec'd */
        const NOTE_SIGNAL                       = 0x08000000;   /* shared with EVFILT_SIGNAL */
        const NOTE_EXITSTATUS                   = 0x04000000;   /* exit status to be returned, valid for child process only */
        const NOTE_EXIT_DETAIL                  = 0x02000000;   /* provide details on reasons for exit */
        const NOTE_PDATAMASK                    = 0x000fffff;   /* mask for signal & exit status */
        const NOTE_PCTRLMASK                    = 0xf0000000;
        const NOTE_SECONDS                      = 0x00000001;   /* data is seconds         */
        const NOTE_USECONDS                     = 0x00000002;   /* data is microseconds    */
        const NOTE_NSECONDS                     = 0x00000004;   /* data is nanoseconds     */
        const NOTE_ABSOLUTE                     = 0x00000008;   /* absolute timeout        */
                                                                /* ... implicit EV_ONESHOT */
        const NOTE_LEEWAY                       = 0x00000010;   /* ext[1] holds leeway for power aware timers */
        const NOTE_CRITICAL                     = 0x00000020;   /* system does minimal timer coalescing */
        const NOTE_BACKGROUND                   = 0x00000040;   /* system does maximum timer coalescing */
        const NOTE_VM_PRESSURE                  = 0x80000000;   /* will react on memory pressure */
        const NOTE_VM_PRESSURE_TERMINATE        = 0x40000000;   /* will quit on memory pressure, possibly after cleaning up dirty state */
        const NOTE_VM_PRESSURE_SUDDEN_TERMINATE = 0x20000000;   /* will quit immediately on memory pressure */
        const NOTE_VM_ERROR                     = 0x10000000;   /* there was an error */
        const NOTE_TRACK                        = 0x00000001;   /* follow across forks */
        const NOTE_TRACKERR                     = 0x00000002;   /* could not track child */
        const NOTE_CHILD                        = 0x00000004;   /* am a child process */
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
