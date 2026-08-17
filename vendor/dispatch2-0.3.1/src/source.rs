#![allow(non_camel_case_types)] // TODO
#![allow(missing_docs)] // TODO
use core::ffi::c_ulong;

dispatch_object!(
    /// Dispatch source.
    #[doc(alias = "dispatch_source_t")]
    #[doc(alias = "dispatch_source_s")]
    pub struct DispatchSource;
);

dispatch_object_not_data!(unsafe DispatchSource);

#[repr(C)]
#[derive(Debug)]
pub struct dispatch_source_type_s {
    /// opaque value
    _inner: [u8; 0],
    _p: crate::OpaqueData,
}

#[cfg(feature = "objc2")]
// SAFETY: Dispatch types are internally objects.
unsafe impl objc2::encode::RefEncode for dispatch_source_type_s {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}

#[allow(missing_docs)]
pub type dispatch_source_type_t = *mut dispatch_source_type_s;

enum_with_val! {
    /// Mach send-right flags.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_mach_send_flags_t(pub c_ulong) {
        DISPATCH_MACH_SEND_DEAD = 0x1
    }
}

enum_with_val! {
    /// Mach receive-right flags.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_mach_recv_flags_t(pub c_ulong) {
        // no definition
    }
}

enum_with_val! {
    // Memory pressure events.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_memorypressure_flags_t(pub c_ulong) {
        DISPATCH_MEMORYPRESSURE_NORMAL = 0x1,
        DISPATCH_MEMORYPRESSURE_WARN = 0x2,
        DISPATCH_MEMORYPRESSURE_CRITICAL = 0x4,
    }
}

enum_with_val! {
    /// Events related to a process.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_proc_flags_t(pub c_ulong) {
        DISPATCH_PROC_EXIT = 0x80000000,
        DISPATCH_PROC_FORK = 0x40000000,
        DISPATCH_PROC_EXEC = 0x20000000,
        DISPATCH_PROC_SIGNAL = 0x08000000,
    }
}

enum_with_val! {
    /// Events involving a change to a file system object.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_vnode_flags_t(pub c_ulong) {
        DISPATCH_VNODE_DELETE = 0x1,
        DISPATCH_VNODE_WRITE = 0x2,
        DISPATCH_VNODE_EXTEND = 0x4,
        DISPATCH_VNODE_ATTRIB = 0x8,
        DISPATCH_VNODE_LINK = 0x10,
        DISPATCH_VNODE_RENAME = 0x20,
        DISPATCH_VNODE_REVOKE = 0x40,
        DISPATCH_VNODE_FUNLOCK = 0x100,
    }
}

enum_with_val! {
    /// Flags to use when configuring a timer dispatch source.
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct dispatch_source_timer_flags_t(pub c_ulong) {
        DISPATCH_TIMER_STRICT = 0x1,
    }
}
