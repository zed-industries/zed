//! Additional bindings not (or incorrectly) exposed by the [`mach2`] crate.
//! These are lifted from <https://github.com/apple-oss-distributions/xnu>

pub use mach2::{
    exception_types as et,
    kern_return::{KERN_SUCCESS, kern_return_t},
    mach_init::mach_thread_self,
    mach_port as mp, mach_types as mt, message as msg,
    port::{self, MACH_PORT_NULL, mach_port_t},
    task, thread_status as ts,
    traps::mach_task_self,
};

/// Number of top level exception types
///
/// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/arm/exception.h#L34>
/// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/i386/exception.h#L68>
pub const EXC_TYPES_COUNT: usize = 14;
/// For `EXC_SOFTWARE` exceptions, this indicates the exception was due to a Unix signal
///
/// The actual Unix signal is stored in the subcode of the exception
///
/// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/exception_types.h#L176-L182>
pub const EXC_SOFT_SIGNAL: u32 = 0x10003;

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        /// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/i386/thread_status.h#L118>
        pub const THREAD_STATE_NONE: ts::thread_state_flavor_t = 13;
    } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
        /// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/arm/thread_status.h#L57>
        pub const THREAD_STATE_NONE: ts::thread_state_flavor_t = 5;
    }
}

/// Network Data Representation Record
///
/// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/ndr.h#L40-L49>
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NdrRecord {
    pub mig_vers: u8,
    pub if_vers: u8,
    pub reserved1: u8,
    pub mig_encoding: u8,
    pub int_rep: u8,
    pub char_rep: u8,
    pub float_rep: u8,
    pub reserved2: u8,
}

/// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/message.h#L379-L391>
#[repr(C, packed(4))]
pub struct MachMsgPortDescriptor {
    pub name: u32,
    __pad1: u32,
    __pad2: u16,
    __disposition: u8,
    __type: u8,
}

#[repr(C, packed(4))]
pub struct MachMsgBody {
    pub descriptor_count: u32,
}

/// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/message.h#L545-L552>
#[repr(C, packed(4))]
pub struct MachMsgHeader {
    pub bits: u32,
    pub size: u32,
    pub remote_port: u32,
    pub local_port: u32,
    pub voucher_port: u32,
    pub id: u32,
}

/// <https://github.com/apple-oss-distributions/xnu/blob/e6231be02a03711ca404e5121a151b24afbff733/osfmk/mach/message.h#L585-L588>
#[repr(C, packed(4))]
pub struct MachMsgTrailer {
    pub kind: u32,
    pub size: u32,
}

/// This structure can be obtained by running `mig <path to OSX SDK>/usr/include/mach_exc.defs`
#[repr(C, packed(4))]
pub struct ExceptionMessage {
    pub header: MachMsgHeader,
    /* start of the kernel processed data */
    pub body: MachMsgBody,
    pub thread: MachMsgPortDescriptor,
    pub task: MachMsgPortDescriptor,
    /* end of the kernel processed data */
    _ndr: NdrRecord,
    pub exception: u32,
    pub code_count: u32,
    pub code: [u64; 2],
    _trailer: MachMsgTrailer,
}

/// Whenever MIG detects an error, it sends back a generic `mig_reply_error_t`
/// format message.  Clients must accept these in addition to the expected reply
/// message format.
///
/// This structure can be obtained by running `mig <path to OSX SDK>/usr/include/mach_exc.defs`
#[repr(C, packed(4))]
pub struct ExceptionRaiseReply {
    pub header: MachMsgHeader,
    pub ndr: NdrRecord,
    pub ret_code: kern_return_t,
}

unsafe extern "C" {
    /// Set an exception handler for a thread on one or more exception types.
    /// At the same time, return the previously defined exception handlers for
    /// those types.
    ///
    /// Atomically (I assume) swaps the currently registered exception ports
    /// with a new one, returning the previously registered ports so that
    /// they can be restored later.
    ///
    /// Given the order of arguments I'm assuming this function has evolved
    /// over time, but basically (at least, according to how it is used in
    /// Breakpad), the output of this function will be 4 distinct arrays,
    /// which are basically a structure of arrays
    ///
    /// <https://github.com/apple-oss-distributions/xnu/blob/e7776783b89a353188416a9a346c6cdb4928faad/osfmk/mach/task.defs#L281-L295>
    pub fn task_swap_exception_ports(
        task: mt::task_t,                      // The task we want to swap the ports for
        exception_mask: et::exception_mask_t, // The mask of exceptions, will only swaps ports that match an exception in the mask
        new_port: mach_port_t,                // The new exception port we want to use
        behavior: et::exception_behavior_t,   // The exception behavior when sending to the port
        new_flavor: ts::thread_state_flavor_t, // What CPU context info to retrieve
        masks: *mut et::exception_mask_t, // Output array of each exception mask that has a registered port
        masks_count: *mut u32, // The length of the masks array, as well as the following arrays
        old_handlers: *mut mach_port_t, // Output array of ports that are registered
        old_behaviors: *mut et::exception_behavior_t, // Output array of behaviors
        old_flavors: *mut ts::thread_state_flavor_t, // Output array of thread flavors
    ) -> kern_return_t;

    /// Set an exception handler for a task on one or more exception types.
    /// These handlers are invoked for all threads in the task if there are
    /// no thread-specific exception handlers or those handlers returned an
    /// error.
    ///
    /// <https://github.com/apple-oss-distributions/xnu/blob/e7776783b89a353188416a9a346c6cdb4928faad/osfmk/mach/task.defs#L249-L260>
    pub fn task_set_exception_ports(
        task: mt::task_t,                      // The task we want to set the port for
        exception_mask: et::exception_mask_t,  // The exception we want to set the port for
        new_port: mach_port_t,                 // The new port to receive exceptions on
        behavior: et::exception_behavior_t,    // The exception behavior when send to the port
        new_flavor: ts::thread_state_flavor_t, // What CPU context info to send with the exception
    ) -> kern_return_t;

    /// The host? NDR
    ///
    /// <https://github.com/apple-oss-distributions/xnu/blob/e7776783b89a353188416a9a346c6cdb4928faad/osfmk/mach/arm/ndr_def.h#L36-L45>
    /// <https://github.com/apple-oss-distributions/xnu/blob/e7776783b89a353188416a9a346c6cdb4928faad/osfmk/mach/i386/ndr_def.h#L36-L45>
    pub static NDR_record: NdrRecord;
}
