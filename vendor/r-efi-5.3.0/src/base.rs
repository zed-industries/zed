//! UEFI Base Environment
//!
//! This module defines the base environment for UEFI development. It provides types and macros as
//! declared in the UEFI specification, as well as de-facto standard additions provided by the
//! reference implementation by Intel.
//!
//! # Target Configuration
//!
//! Wherever possible, native rust types are used to represent their UEFI counter-parts. However,
//! this means the ABI depends on the implementation of said rust types. Hence, native rust types
//! are only used where rust supports a stable ABI of said types, and their ABI matches the ABI
//! defined by the UEFI specification.
//!
//! Nevertheless, even if the ABI of a specific type is marked stable, this does not imply that it
//! is the same across architectures. For instance, rust's `u64` type has the same binary
//! representation as the `UINT64` type in UEFI. But this does not imply that it has the same
//! binary representation on `x86_64` and on `ppc64be`. As a result of this, the compilation of
//! this module is tied to the target-configuration you passed to the rust compiler. Wherever
//! possible and reasonable, any architecture differences are abstracted, though. This means that
//! in most cases you can use this module even though your target-configuration might not match
//! the native UEFI target-configuration.
//!
//! The recommend way to compile your code, is to use the native target-configuration for UEFI.
//! These configurations are not necessarily included in the upstream rust compiler. Hence, you
//! might have to craft one yourself. For all systems that we can test on, we make sure to push
//! the target configuration into upstream rust-lang.
//!
//! However, there are situations where you want to access UEFI data from a non-native host. For
//! instance, a UEFI boot loader might store data in boot variables, formatted according to types
//! declared in the UEFI specification. An OS booted thereafter might want to access these
//! variables, but it might be compiled with a different target-configuration than the UEFI
//! environment that it was booted from. A similar situation occurs when you call UEFI runtime
//! functions from your OS. In all those cases, you should very likely be able to use this module
//! to interact with UEFI as well. This is, because most bits of the target-configuration of UEFI
//! and your OS very likely match. In fact, to figure out whether this is safe, you need to make
//! sure that the rust ABI would match in both target-configurations. If it is, all other details
//! are handled within this module just fine.
//!
//! In case of doubt, contact us!
//!
//! # Core Primitives
//!
//! Several of the UEFI primitives are represented by native Rust. These have no type aliases or
//! other definitions here, but you are recommended to use native rust directly. These include:
//!
//!  * `NULL`, `void *`: Void pointers have a native rust implementation in
//!                      [`c_void`](core::ffi::c_void). `NULL` is represented through
//!                      [`null`](core::ptr::null) and [`is_null()`](core::ptr) for
//!                      all pointer types.
//!  * `uint8_t`..`uint64_t`,
//!    `int8_t`..`int64_t`: Fixed-size integers are represented by their native rust equivalents
//!                         (`u8`..`u64`, `i8`..`i64`).
//!
//!  * `UINTN`, `INTN`: Native-sized (or instruction-width sized) integers are represented by
//!                     their native rust equivalents (`usize`, `isize`).
//!
//! # UEFI Details
//!
//! The UEFI Specification describes its target environments in detail. Each supported
//! architecture has a separate section with details on calling conventions, CPU setup, and more.
//! You are highly recommended to conduct the UEFI Specification for details on the programming
//! environment. Following a summary of key parts relevant to rust developers:
//!
//!  * Similar to rust, integers are either fixed-size, or native size. This maps nicely to the
//!    native rust types. The common `long`, `int`, `short` types known from ISO-C are not used.
//!    Whenever you refer to memory (either pointing to it, or remember the size of a memory
//!    block), the native size integers should be your tool of choice.
//!
//!  * Even though the CPU might run in any endianness, all stored data is little-endian. That
//!    means, if you encounter integers split into byte-arrays (e.g.,
//!    `CEfiDevicePathProtocol.length`), you must assume it is little-endian encoded. But if you
//!    encounter native integers, you must assume they are encoded in native endianness.
//!    For now the UEFI specification only defines little-endian architectures, hence this did not
//!    pop up as actual issue. Future extensions might change this, though.
//!
//!  * The Microsoft calling-convention is used. That is, all external calls to UEFI functions
//!    follow a calling convention that is very similar to that used on Microsoft Windows. All
//!    such ABI functions must be marked with the right calling-convention. The UEFI Specification
//!    defines some additional common rules for all its APIs, though. You will most likely not see
//!    any of these mentioned in the individual API documentions. So here is a short reminder:
//!
//!     * Pointers must reference physical-memory locations (no I/O mappings, no
//!       virtual addresses, etc.). Once ExitBootServices() was called, and the
//!       virtual address mapping was set, you must provide virtual-memory
//!       locations instead.
//!     * Pointers must be correctly aligned.
//!     * NULL is disallowed, unless explicitly mentioned otherwise.
//!     * Data referenced by pointers is undefined on error-return from a
//!       function.
//!     * You must not pass data larger than native-size (sizeof(CEfiUSize)) on
//!       the stack. You must pass them by reference.
//!
//!  * Stack size is at least 128KiB and 16-byte aligned. All stack space might be marked
//!    non-executable! Once ExitBootServices() was called, you must guarantee at least 4KiB of
//!    stack space, 16-byte aligned for all runtime services you call.
//!    Details might differ depending on architectures. But the numbers here should serve as
//!    ball-park figures.

// Target Architecture
//
// The UEFI Specification explicitly lists all supported target architectures. While external
// implementors are free to port UEFI to other targets, we need information on the target
// architecture to successfully compile for it. This includes calling-conventions, register
// layouts, endianness, and more. Most of these details are hidden in the rust-target-declaration.
// However, some details are still left to the actual rust code.
//
// This initial check just makes sure the compilation is halted with a suitable error message if
// the target architecture is not supported.
//
// We try to minimize conditional compilations as much as possible. A simple search for
// `target_arch` should reveal all uses throughout the code-base. If you add your target to this
// error-check, you must adjust all other uses as well.
//
// Similarly, UEFI only defines configurations for little-endian architectures so far. Several
// bits of the specification are thus unclear how they would be applied on big-endian systems. We
// therefore mark it as unsupported. If you override this, you are on your own.
#[cfg(not(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "x86",
    target_arch = "x86_64"
)))]
compile_error!("The target architecture is not supported.");
#[cfg(not(target_endian = "little"))]
compile_error!("The target endianness is not supported.");

// eficall_abi!()
//
// This macro is the architecture-dependent implementation of eficall!(). See the documentation of
// the eficall!() macro for a description. Nowadays, this simply maps to `extern "efiapi"`, since
// this has been stabilized with rust-1.68.

#[macro_export]
#[doc(hidden)]
macro_rules! eficall_abi {
    (($($prefix:tt)*),($($suffix:tt)*)) => { $($prefix)* extern "efiapi" $($suffix)* };
}

/// Annotate function with UEFI calling convention
///
/// Since rust-1.68 you can use `extern "efiapi"` as calling-convention to achieve the same
/// behavior as this macro. This macro is kept for backwards-compatibility only, but will nowadays
/// map to `extern "efiapi"`.
///
/// This macro takes a function-declaration as argument and produces the same function-declaration
/// but annotated with the correct calling convention. Since the default `extern "C"` annotation
/// depends on your compiler defaults, we cannot use it. Instead, this macro selects the default
/// for your target platform.
///
/// Ideally, the macro would expand to `extern "<abi>"` so you would be able to write:
///
/// ```ignore
/// // THIS DOES NOT WORK!
/// pub fn eficall!{} foobar() {
///     // ...
/// }
/// ```
///
/// However, macros are evaluated too late for this to work. Instead, the entire construct must be
/// wrapped in a macro, which then expands to the same construct but with `extern "<abi>"`
/// inserted at the correct place:
///
/// ```
/// use r_efi::{eficall, eficall_abi};
///
/// eficall!{pub fn foobar() {
///     // ...
/// }}
///
/// type FooBar = eficall!{fn(u8) -> (u8)};
/// ```
///
/// The `eficall!{}` macro takes either a function-type or function-definition as argument. It
/// inserts `extern "<abi>"` after the function qualifiers, but before the `fn` keyword.
///
/// # Internals
///
/// The `eficall!{}` macro tries to parse the function header so it can insert `extern "<abi>"` at
/// the right place. If, for whatever reason, this does not work with a particular syntax, you can
/// use the internal `eficall_abi!{}` macro. This macro takes two token-streams as input and
/// evaluates to the concatenation of both token-streams, but separated by the selected ABI.
///
/// For instance, the following 3 type definitions are equivalent, assuming the selected ABI
/// is "C":
///
/// ```
/// use r_efi::{eficall, eficall_abi};
///
/// type FooBar1 = unsafe extern "C" fn(u8) -> (u8);
/// type FooBar2 = eficall!{unsafe fn(u8) -> (u8)};
/// type FooBar3 = eficall_abi!{(unsafe), (fn(u8) -> (u8))};
/// ```
///
/// # Calling Conventions
///
/// The UEFI specification defines the calling convention for each platform individually. It
/// usually refers to other standards for details, but adds some restrictions on top. As of this
/// writing, it mentions:
///
///  * aarch32 / arm: The `aapcs` calling-convention is used. It is native to aarch32 and described
///                   in a document called
///                   "Procedure Call Standard for the ARM Architecture". It is openly distributed
///                   by ARM and widely known under the keyword `aapcs`.
///  * aarch64: The `aapcs64` calling-convention is used. It is native to aarch64 and described in
///             a document called
///             "Procedure Call Standard for the ARM 64-bit Architecture (AArch64)". It is openly
///             distributed by ARM and widely known under the keyword `aapcs64`.
///  * ia-64: The "P64 C Calling Convention" as described in the
///           "Itanium Software Conventions and Runtime Architecture Guide". It is also
///           standardized in the "Intel Itanium SAL Specification".
///  * RISC-V: The "Standard RISC-V C Calling Convention" is used. The UEFI specification
///            describes it in detail, but also refers to the official RISC-V resources for
///            detailed information.
///  * x86 / ia-32: The `cdecl` C calling convention is used. Originated in the C Language and
///                 originally tightly coupled to C specifics. Unclear whether a formal
///                 specification exists (does anyone know?). Most compilers support it under the
///                 `cdecl` keyword, and in nearly all situations it is the default on x86.
///  * x86_64 / amd64 / x64: The `win64` calling-convention is used. It is similar to the `sysv64`
///                          convention that is used on most non-windows x86_64 systems, but not
///                          exactly the same. Microsoft provides open documentation on it. See
///                          MSDN "x64 Software Conventions -> Calling Conventions".
///                          The UEFI Specification does not directly refer to `win64`, but
///                          contains a full specification of the calling convention itself.
///
/// Note that in most cases the UEFI Specification adds several more restrictions on top of the
/// common calling-conventions. These restrictions usually do not affect how the compiler will lay
/// out the function calls. Instead, it usually only restricts the set of APIs that are allowed in
/// UEFI. Therefore, most compilers already support the calling conventions used on UEFI.
///
/// # Variadics
///
/// For some reason, the rust compiler allows variadics only in combination with the `"C"` calling
/// convention, even if the selected calling-convention matches what `"C"` would select on the
/// target platform. Hence, you will very likely be unable to use variadics with this macro.
/// Luckily, all of the UEFI functions that use variadics are wrappers around more low-level
/// accessors, so they are not necessarily required.
#[macro_export]
macro_rules! eficall {
    // Muncher
    //
    // The `@munch()` rules are internal and should not be invoked directly. We walk through the
    // input, moving one token after the other from the suffix into the prefix until we find the
    // position where to insert `extern "<abi>"`. This muncher never drops any tokens, hence we
    // can safely match invalid statements just fine, as the compiler will later print proper
    // diagnostics when parsing the macro output.
    // Once done, we invoke the `eficall_abi!{}` macro, which simply inserts the correct ABI.
    (@munch(($($prefix:tt)*),(pub $($suffix:tt)*))) => { eficall!{@munch(($($prefix)* pub),($($suffix)*))} };
    (@munch(($($prefix:tt)*),(unsafe $($suffix:tt)*))) => { eficall!{@munch(($($prefix)* unsafe),($($suffix)*))} };
    (@munch(($($prefix:tt)*),($($suffix:tt)*))) => { eficall_abi!{($($prefix)*),($($suffix)*)} };

    // Entry Point
    //
    // This captures the entire argument and invokes its own TT-muncher, but splits the input into
    // prefix and suffix, so the TT-muncher can walk through it. Note that initially everything is
    // in the suffix and the prefix is empty.
    ($($arg:tt)*) => { eficall!{@munch((),($($arg)*))} };
}

/// Boolean Type
///
/// This boolean type works very similar to the rust primitive type of [`bool`]. However, the rust
/// primitive type has no stable ABI, hence we provide this type to represent booleans on the FFI
/// interface.
///
/// UEFI defines booleans to be 1-byte integers, which can only have the values of `0` or `1`.
/// However, in practice anything non-zero is considered `true` by nearly all UEFI systems. Hence,
/// this type implements a boolean over `u8` and maps `0` to `false`, everything else to `true`.
///
/// The binary representation of this type is ABI. That is, you are allowed to transmute from and
/// to `u8`. Furthermore, this type never modifies its binary representation. If it was
/// initialized as, or transmuted from, a specific integer value, this value will be retained.
/// However, on the rust side you will never see the integer value. It instead behaves truly as a
/// boolean. If you need access to the integer value, you have to transmute it back to `u8`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
// Manual impls for: Default, Eq, Hash, Ord, PartialEq, PartialOrd
pub struct Boolean(u8);

/// Single-byte Character Type
///
/// The `Char8` type represents single-byte characters. UEFI defines them to be ASCII compatible,
/// using the ISO-Latin-1 character set.
pub type Char8 = u8;

/// Dual-byte Character Type
///
/// The `Char16` type represents dual-byte characters. UEFI defines them to be UCS-2 encoded.
pub type Char16 = u16;

/// Status Codes
///
/// UEFI uses the `Status` type to represent all kinds of status codes. This includes return codes
/// from functions, but also complex state of different devices and drivers. It is a simple
/// `usize`, but wrapped in a rust-type to allow us to implement helpers on this type. Depending
/// on the context, different state is stored in it. Note that it is always binary compatible to a
/// usize!
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Status(usize);

/// Object Handles
///
/// Handles represent access to an opaque object. Handles are untyped by default, but get a
/// meaning when you combine them with an interface. Internally, they are simple void pointers. It
/// is the UEFI driver model that applies meaning to them.
pub type Handle = *mut core::ffi::c_void;

/// Event Objects
///
/// Event objects represent hooks into the main-loop of a UEFI environment. They allow to register
/// callbacks, to be invoked when a specific event happens. In most cases you use events to
/// register timer-based callbacks, as well as chaining events together. Internally, they are
/// simple void pointers. It is the UEFI task management that applies meaning to them.
pub type Event = *mut core::ffi::c_void;

/// Logical Block Addresses
///
/// The LBA type is used to denote logical block addresses of block devices. It is a simple 64-bit
/// integer, that is used to denote addresses when working with block devices.
pub type Lba = u64;

/// Thread Priority Levels
///
/// The process model of UEFI systems is highly simplified. Priority levels are used to order
/// execution of pending tasks. The TPL type denotes a priority level of a specific task. The
/// higher the number, the higher the priority. It is a simple integer type, but its range is
/// usually highly restricted. The UEFI task management provides constants and accessors for TPLs.
pub type Tpl = usize;

/// Physical Memory Address
///
/// A simple 64bit integer containing a physical memory address.
pub type PhysicalAddress = u64;

/// Virtual Memory Address
///
/// A simple 64bit integer containing a virtual memory address.
pub type VirtualAddress = u64;

/// Application Entry Point
///
/// This type defines the entry-point of UEFI applications. It is ABI and cannot be changed.
/// Whenever you load UEFI images, the entry-point is called with this signature.
///
/// In most cases the UEFI image (or application) is unloaded when control returns from the entry
/// point. In case of UEFI drivers, they can request to stay loaded until an explicit unload.
///
/// The system table is provided as mutable pointer. This is, because there is no guarantee that
/// timer interrupts do not modify the table. Furthermore, exiting boot services causes several
/// modifications on that table. And lastly, the system table lives longer than the function
/// invocation, if invoked as an UEFI driver.
/// In most cases it is perfectly fine to cast the pointer to a real rust reference. However, this
/// should be an explicit decision by the caller.
pub type ImageEntryPoint = eficall! {fn(Handle, *mut crate::system::SystemTable) -> Status};

/// Globally Unique Identifiers
///
/// The `Guid` type represents globally unique identifiers as defined by RFC-4122 (i.e., only the
/// `10x` variant is used), with the caveat that LE is used instead of BE.
///
/// Note that only the binary representation of Guids is stable. You are highly recommended to
/// interpret Guids as 128bit integers.
///
/// The UEFI specification requires the type to be 64-bit aligned, yet EDK2 uses a mere 32-bit
/// alignment. Hence, for compatibility, a 32-bit alignment is used.
///
/// UEFI uses the Microsoft-style Guid format. Hence, a lot of documentation and code refers to
/// these Guids. If you thusly cannot treat Guids as 128-bit integers, this Guid type allows you
/// to access the individual fields of the Microsoft-style Guid. A reminder of the Guid encoding:
///
/// ```text
///    0                   1                   2                   3
///    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                          time_low                             |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |       time_mid                |         time_hi_and_version   |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |clk_seq_hi_res |  clk_seq_low  |         node (0-1)            |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |                         node (2-5)                            |
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
///
/// The individual fields are encoded as little-endian. Accessors are provided for the Guid
/// structure allowing access to these fields in native endian byte order.
#[repr(C, align(4))]
#[derive(Clone, Copy, Debug)]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Guid {
    time_low: [u8; 4],
    time_mid: [u8; 2],
    time_hi_and_version: [u8; 2],
    clk_seq_hi_res: u8,
    clk_seq_low: u8,
    node: [u8; 6],
}

/// Network MAC Address
///
/// This type encapsulates a single networking media access control address
/// (MAC). It is a simple 32 bytes buffer with no special alignment. Note that
/// no comparison function are defined by default, since trailing bytes of the
/// address might be random.
///
/// The interpretation of the content differs depending on the protocol it is
/// used with. See each documentation for details. In most cases this contains
/// an Ethernet address.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MacAddress {
    pub addr: [u8; 32],
}

/// IPv4 Address
///
/// Binary representation of an IPv4 address. It is encoded in network byte
/// order (i.e., big endian). Note that no special alignment restrictions are
/// defined by the standard specification.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}

/// IPv6 Address
///
/// Binary representation of an IPv6 address, encoded in network byte order
/// (i.e., big endian). Similar to the IPv4 address, no special alignment
/// restrictions are defined by the standard specification.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}

/// IP Address
///
/// A union type over the different IP addresses available. Alignment is always
/// fixed to 4-bytes. Note that trailing bytes might be random, so no
/// comparison functions are derived.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub union IpAddress {
    pub addr: [u32; 4],
    pub v4: Ipv4Address,
    pub v6: Ipv6Address,
}

impl Boolean {
    /// Literal False
    ///
    /// This constant represents the `false` value of the `Boolean` type.
    pub const FALSE: Boolean = Boolean(0u8);

    /// Literal True
    ///
    /// This constant represents the `true` value of the `Boolean` type.
    pub const TRUE: Boolean = Boolean(1u8);
}

impl From<u8> for Boolean {
    fn from(v: u8) -> Self {
        Boolean(v)
    }
}

impl From<bool> for Boolean {
    fn from(v: bool) -> Self {
        match v {
            false => Boolean::FALSE,
            true => Boolean::TRUE,
        }
    }
}

impl Default for Boolean {
    fn default() -> Self {
        Self::FALSE
    }
}

impl From<Boolean> for u8 {
    fn from(v: Boolean) -> Self {
        match v.0 {
            0 => 0,
            _ => 1,
        }
    }
}

impl From<Boolean> for bool {
    fn from(v: Boolean) -> Self {
        match v.0 {
            0 => false,
            _ => true,
        }
    }
}

impl Eq for Boolean {}

impl core::hash::Hash for Boolean {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        bool::from(*self).hash(state)
    }
}

impl Ord for Boolean {
    fn cmp(&self, other: &Boolean) -> core::cmp::Ordering {
        bool::from(*self).cmp(&(*other).into())
    }
}

impl PartialEq for Boolean {
    fn eq(&self, other: &Boolean) -> bool {
        bool::from(*self).eq(&(*other).into())
    }
}

impl PartialEq<bool> for Boolean {
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self).eq(other)
    }
}

impl PartialOrd for Boolean {
    fn partial_cmp(&self, other: &Boolean) -> Option<core::cmp::Ordering> {
        bool::from(*self).partial_cmp(&(*other).into())
    }
}

impl PartialOrd<bool> for Boolean {
    fn partial_cmp(&self, other: &bool) -> Option<core::cmp::Ordering> {
        bool::from(*self).partial_cmp(other)
    }
}

impl Status {
    const WIDTH: usize = 8usize * core::mem::size_of::<Status>();
    const MASK: usize = 0xc0 << (Status::WIDTH - 8);
    const ERROR_MASK: usize = 0x80 << (Status::WIDTH - 8);
    const WARNING_MASK: usize = 0x00 << (Status::WIDTH - 8);

    /// Success Code
    ///
    /// This code represents a successfull function invocation. Its value is guaranteed to be 0.
    /// However, note that warnings are considered success as well, so this is not the only code
    /// that can be returned by UEFI functions on success. However, in nearly all situations
    /// warnings are not allowed, so the effective result will be SUCCESS.
    pub const SUCCESS: Status = Status::from_usize(0);

    // List of predefined error codes
    pub const LOAD_ERROR: Status = Status::from_usize(1 | Status::ERROR_MASK);
    pub const INVALID_PARAMETER: Status = Status::from_usize(2 | Status::ERROR_MASK);
    pub const UNSUPPORTED: Status = Status::from_usize(3 | Status::ERROR_MASK);
    pub const BAD_BUFFER_SIZE: Status = Status::from_usize(4 | Status::ERROR_MASK);
    pub const BUFFER_TOO_SMALL: Status = Status::from_usize(5 | Status::ERROR_MASK);
    pub const NOT_READY: Status = Status::from_usize(6 | Status::ERROR_MASK);
    pub const DEVICE_ERROR: Status = Status::from_usize(7 | Status::ERROR_MASK);
    pub const WRITE_PROTECTED: Status = Status::from_usize(8 | Status::ERROR_MASK);
    pub const OUT_OF_RESOURCES: Status = Status::from_usize(9 | Status::ERROR_MASK);
    pub const VOLUME_CORRUPTED: Status = Status::from_usize(10 | Status::ERROR_MASK);
    pub const VOLUME_FULL: Status = Status::from_usize(11 | Status::ERROR_MASK);
    pub const NO_MEDIA: Status = Status::from_usize(12 | Status::ERROR_MASK);
    pub const MEDIA_CHANGED: Status = Status::from_usize(13 | Status::ERROR_MASK);
    pub const NOT_FOUND: Status = Status::from_usize(14 | Status::ERROR_MASK);
    pub const ACCESS_DENIED: Status = Status::from_usize(15 | Status::ERROR_MASK);
    pub const NO_RESPONSE: Status = Status::from_usize(16 | Status::ERROR_MASK);
    pub const NO_MAPPING: Status = Status::from_usize(17 | Status::ERROR_MASK);
    pub const TIMEOUT: Status = Status::from_usize(18 | Status::ERROR_MASK);
    pub const NOT_STARTED: Status = Status::from_usize(19 | Status::ERROR_MASK);
    pub const ALREADY_STARTED: Status = Status::from_usize(20 | Status::ERROR_MASK);
    pub const ABORTED: Status = Status::from_usize(21 | Status::ERROR_MASK);
    pub const ICMP_ERROR: Status = Status::from_usize(22 | Status::ERROR_MASK);
    pub const TFTP_ERROR: Status = Status::from_usize(23 | Status::ERROR_MASK);
    pub const PROTOCOL_ERROR: Status = Status::from_usize(24 | Status::ERROR_MASK);
    pub const INCOMPATIBLE_VERSION: Status = Status::from_usize(25 | Status::ERROR_MASK);
    pub const SECURITY_VIOLATION: Status = Status::from_usize(26 | Status::ERROR_MASK);
    pub const CRC_ERROR: Status = Status::from_usize(27 | Status::ERROR_MASK);
    pub const END_OF_MEDIA: Status = Status::from_usize(28 | Status::ERROR_MASK);
    pub const END_OF_FILE: Status = Status::from_usize(31 | Status::ERROR_MASK);
    pub const INVALID_LANGUAGE: Status = Status::from_usize(32 | Status::ERROR_MASK);
    pub const COMPROMISED_DATA: Status = Status::from_usize(33 | Status::ERROR_MASK);
    pub const IP_ADDRESS_CONFLICT: Status = Status::from_usize(34 | Status::ERROR_MASK);
    pub const HTTP_ERROR: Status = Status::from_usize(35 | Status::ERROR_MASK);

    // List of error codes from protocols
    // UDP4
    pub const NETWORK_UNREACHABLE: Status = Status::from_usize(100 | Status::ERROR_MASK);
    pub const HOST_UNREACHABLE: Status = Status::from_usize(101 | Status::ERROR_MASK);
    pub const PROTOCOL_UNREACHABLE: Status = Status::from_usize(102 | Status::ERROR_MASK);
    pub const PORT_UNREACHABLE: Status = Status::from_usize(103 | Status::ERROR_MASK);
    // TCP4
    pub const CONNECTION_FIN: Status = Status::from_usize(104 | Status::ERROR_MASK);
    pub const CONNECTION_RESET: Status = Status::from_usize(105 | Status::ERROR_MASK);
    pub const CONNECTION_REFUSED: Status = Status::from_usize(106 | Status::ERROR_MASK);

    // List of predefined warning codes
    pub const WARN_UNKNOWN_GLYPH: Status = Status::from_usize(1 | Status::WARNING_MASK);
    pub const WARN_DELETE_FAILURE: Status = Status::from_usize(2 | Status::WARNING_MASK);
    pub const WARN_WRITE_FAILURE: Status = Status::from_usize(3 | Status::WARNING_MASK);
    pub const WARN_BUFFER_TOO_SMALL: Status = Status::from_usize(4 | Status::WARNING_MASK);
    pub const WARN_STALE_DATA: Status = Status::from_usize(5 | Status::WARNING_MASK);
    pub const WARN_FILE_SYSTEM: Status = Status::from_usize(6 | Status::WARNING_MASK);
    pub const WARN_RESET_REQUIRED: Status = Status::from_usize(7 | Status::WARNING_MASK);

    /// Create Status Code from Integer
    ///
    /// This takes the literal value of a status code and turns it into a `Status` object. Note
    /// that we want it as `const fn` so we cannot use `core::convert::From`.
    pub const fn from_usize(v: usize) -> Status {
        Status(v)
    }

    /// Return Underlying Integer Representation
    ///
    /// This takes the `Status` object and returns the underlying integer representation as
    /// defined by the UEFI specification.
    pub const fn as_usize(&self) -> usize {
        self.0
    }

    fn value(&self) -> usize {
        self.0
    }

    fn mask(&self) -> usize {
        self.value() & Status::MASK
    }

    /// Check whether this is an error
    ///
    /// This returns true if the given status code is considered an error. Errors mean the
    /// operation did not succeed, nor produce any valuable output. Output parameters must be
    /// considered invalid if an error was returned. That is, its content is not well defined.
    pub fn is_error(&self) -> bool {
        self.mask() == Status::ERROR_MASK
    }

    /// Check whether this is a warning
    ///
    /// This returns true if the given status code is considered a warning. Warnings are to be
    /// treated as success, but might indicate data loss or other device errors. However, if an
    /// operation returns with a warning code, it must be considered successfull, and the output
    /// parameters are valid.
    pub fn is_warning(&self) -> bool {
        self.value() != 0 && self.mask() == Status::WARNING_MASK
    }
}

impl From<Status> for Result<Status, Status> {
    fn from(status: Status) -> Self {
        if status.is_error() {
            Err(status)
        } else {
            Ok(status)
        }
    }
}

impl Guid {
    const fn u32_to_bytes_le(num: u32) -> [u8; 4] {
        [
            num as u8,
            (num >> 8) as u8,
            (num >> 16) as u8,
            (num >> 24) as u8,
        ]
    }

    const fn u32_from_bytes_le(bytes: &[u8; 4]) -> u32 {
        (bytes[0] as u32)
            | ((bytes[1] as u32) << 8)
            | ((bytes[2] as u32) << 16)
            | ((bytes[3] as u32) << 24)
    }

    const fn u16_to_bytes_le(num: u16) -> [u8; 2] {
        [num as u8, (num >> 8) as u8]
    }

    const fn u16_from_bytes_le(bytes: &[u8; 2]) -> u16 {
        (bytes[0] as u16) | ((bytes[1] as u16) << 8)
    }

    /// Initialize a Guid from its individual fields
    ///
    /// This function initializes a Guid object given the individual fields as specified in the
    /// UEFI specification. That is, if you simply copy the literals from the specification into
    /// your code, this function will correctly initialize the Guid object.
    ///
    /// In other words, this takes the individual fields in native endian and converts them to the
    /// correct endianness for a UEFI Guid.
    ///
    /// Due to the fact that UEFI Guids use variant 2 of the UUID specification in a little-endian
    /// (or even mixed-endian) format, the following transformation is likely applied from text
    /// representation to binary representation:
    ///
    ///   00112233-4455-6677-8899-aabbccddeeff
    ///   =>
    ///   33 22 11 00 55 44 77 66 88 99 aa bb cc dd ee ff
    ///
    /// (Note that UEFI protocols often use `88-99` instead of `8899`)
    /// The first 3 parts use little-endian notation, the last 2 use big-endian.
    pub const fn from_fields(
        time_low: u32,
        time_mid: u16,
        time_hi_and_version: u16,
        clk_seq_hi_res: u8,
        clk_seq_low: u8,
        node: &[u8; 6],
    ) -> Guid {
        Guid {
            time_low: Self::u32_to_bytes_le(time_low),
            time_mid: Self::u16_to_bytes_le(time_mid),
            time_hi_and_version: Self::u16_to_bytes_le(time_hi_and_version),
            clk_seq_hi_res: clk_seq_hi_res,
            clk_seq_low: clk_seq_low,
            node: *node,
        }
    }

    /// Access a Guid as individual fields
    ///
    /// This decomposes a Guid back into the individual fields as given in the specification. The
    /// individual fields are returned in native-endianness.
    pub const fn as_fields(&self) -> (u32, u16, u16, u8, u8, &[u8; 6]) {
        (
            Self::u32_from_bytes_le(&self.time_low),
            Self::u16_from_bytes_le(&self.time_mid),
            Self::u16_from_bytes_le(&self.time_hi_and_version),
            self.clk_seq_hi_res,
            self.clk_seq_low,
            &self.node,
        )
    }

    /// Initialize a Guid from its byte representation
    ///
    /// Create a new Guid object from its byte representation. This
    /// reinterprets the bytes as a Guid and copies them into a new Guid
    /// instance. Note that you can safely transmute instead.
    ///
    /// See `as_bytes()` for the inverse operation.
    pub const fn from_bytes(bytes: &[u8; 16]) -> Self {
        unsafe { core::mem::transmute::<[u8; 16], Guid>(*bytes) }
    }

    /// Access a Guid as raw byte array
    ///
    /// This provides access to a Guid through a byte array. It is a simple re-interpretation of
    /// the Guid value as a 128-bit byte array. No conversion is performed. This is a simple cast.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        unsafe { core::mem::transmute::<&Guid, &[u8; 16]>(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    // Helper to compute a hash of an object.
    fn hash<T: core::hash::Hash>(v: &T) -> u64 {
        let mut h = std::hash::DefaultHasher::new();
        v.hash(&mut h);
        core::hash::Hasher::finish(&h)
    }

    // Verify Type Size and Alignemnt
    //
    // Since UEFI defines explicitly the ABI of their types, we can verify that our implementation
    // is correct by checking the size and alignment of the ABI types matches what the spec
    // mandates.
    #[test]
    fn type_size_and_alignment() {
        //
        // Booleans
        //

        assert_eq!(size_of::<Boolean>(), 1);
        assert_eq!(align_of::<Boolean>(), 1);

        //
        // Char8 / Char16
        //

        assert_eq!(size_of::<Char8>(), 1);
        assert_eq!(align_of::<Char8>(), 1);
        assert_eq!(size_of::<Char16>(), 2);
        assert_eq!(align_of::<Char16>(), 2);

        assert_eq!(size_of::<Char8>(), size_of::<u8>());
        assert_eq!(align_of::<Char8>(), align_of::<u8>());
        assert_eq!(size_of::<Char16>(), size_of::<u16>());
        assert_eq!(align_of::<Char16>(), align_of::<u16>());

        //
        // Status
        //

        assert_eq!(size_of::<Status>(), size_of::<usize>());
        assert_eq!(align_of::<Status>(), align_of::<usize>());

        //
        // Handles / Events
        //

        assert_eq!(size_of::<Handle>(), size_of::<usize>());
        assert_eq!(align_of::<Handle>(), align_of::<usize>());
        assert_eq!(size_of::<Event>(), size_of::<usize>());
        assert_eq!(align_of::<Event>(), align_of::<usize>());

        assert_eq!(size_of::<Handle>(), size_of::<*mut ()>());
        assert_eq!(align_of::<Handle>(), align_of::<*mut ()>());
        assert_eq!(size_of::<Event>(), size_of::<*mut ()>());
        assert_eq!(align_of::<Event>(), align_of::<*mut ()>());

        //
        // Lba / Tpl
        //

        assert_eq!(size_of::<Lba>(), size_of::<u64>());
        assert_eq!(align_of::<Lba>(), align_of::<u64>());
        assert_eq!(size_of::<Tpl>(), size_of::<usize>());
        assert_eq!(align_of::<Tpl>(), align_of::<usize>());

        //
        // PhysicalAddress / VirtualAddress
        //

        assert_eq!(size_of::<PhysicalAddress>(), size_of::<u64>());
        assert_eq!(align_of::<PhysicalAddress>(), align_of::<u64>());
        assert_eq!(size_of::<VirtualAddress>(), size_of::<u64>());
        assert_eq!(align_of::<VirtualAddress>(), align_of::<u64>());

        //
        // ImageEntryPoint
        //

        assert_eq!(size_of::<ImageEntryPoint>(), size_of::<fn()>());
        assert_eq!(align_of::<ImageEntryPoint>(), align_of::<fn()>());

        //
        // Guid
        //

        assert_eq!(size_of::<Guid>(), 16);
        assert_eq!(align_of::<Guid>(), 4);

        //
        // Networking Types
        //

        assert_eq!(size_of::<MacAddress>(), 32);
        assert_eq!(align_of::<MacAddress>(), 1);
        assert_eq!(size_of::<Ipv4Address>(), 4);
        assert_eq!(align_of::<Ipv4Address>(), 1);
        assert_eq!(size_of::<Ipv6Address>(), 16);
        assert_eq!(align_of::<Ipv6Address>(), 1);
        assert_eq!(size_of::<IpAddress>(), 16);
        assert_eq!(align_of::<IpAddress>(), 4);
    }

    #[test]
    fn eficall() {
        //
        // Make sure the eficall!{} macro can deal with all kinds of function callbacks.
        //

        let _: eficall! {fn()};
        let _: eficall! {unsafe fn()};
        let _: eficall! {fn(i32)};
        let _: eficall! {fn(i32) -> i32};
        let _: eficall! {fn(i32, i32) -> (i32, i32)};

        eficall! {fn _unused00() {}}
        eficall! {unsafe fn _unused01() {}}
        eficall! {pub unsafe fn _unused02() {}}
    }

    // Verify Boolean ABI
    //
    // Even though booleans are strictly 1-bit, and thus 0 or 1, in practice all UEFI systems
    // treat it more like C does, and a boolean formatted as `u8` now allows any value other than
    // 0 to represent `true`. Make sure we support the same.
    #[test]
    fn booleans() {
        // Verify PartialEq works.
        assert_ne!(Boolean::FALSE, Boolean::TRUE);

        // Verify Boolean<->bool conversion and comparison works.
        assert_eq!(Boolean::FALSE, false);
        assert_eq!(Boolean::TRUE, true);

        // Iterate all possible values for `u8` and verify 0 behaves as `false`, and everything
        // else behaves as `true`. We verify both, the natural constructor through `From`, as well
        // as a transmute.
        for i in 0u8..=255u8 {
            let v1: Boolean = i.into();
            let v2: Boolean = unsafe { std::mem::transmute::<u8, Boolean>(i) };

            assert_eq!(v1, v2);
            assert_eq!(v1, v1);
            assert_eq!(v2, v2);

            match i {
                0 => {
                    assert_eq!(v1, Boolean::FALSE);
                    assert_eq!(v1, false);
                    assert_eq!(v2, Boolean::FALSE);
                    assert_eq!(v2, false);

                    assert_ne!(v1, Boolean::TRUE);
                    assert_ne!(v1, true);
                    assert_ne!(v2, Boolean::TRUE);
                    assert_ne!(v2, true);

                    assert!(v1 < Boolean::TRUE);
                    assert!(v1 < true);
                    assert!(v1 >= Boolean::FALSE);
                    assert!(v1 >= false);
                    assert!(v1 <= Boolean::FALSE);
                    assert!(v1 <= false);
                    assert_eq!(v1.cmp(&true.into()), core::cmp::Ordering::Less);
                    assert_eq!(v1.cmp(&false.into()), core::cmp::Ordering::Equal);

                    assert_eq!(hash(&v1), hash(&false));
                }
                _ => {
                    assert_eq!(v1, Boolean::TRUE);
                    assert_eq!(v1, true);
                    assert_eq!(v2, Boolean::TRUE);
                    assert_eq!(v2, true);

                    assert_ne!(v1, Boolean::FALSE);
                    assert_ne!(v1, false);
                    assert_ne!(v2, Boolean::FALSE);
                    assert_ne!(v2, false);

                    assert!(v1 <= Boolean::TRUE);
                    assert!(v1 <= true);
                    assert!(v1 >= Boolean::TRUE);
                    assert!(v1 >= true);
                    assert!(v1 > Boolean::FALSE);
                    assert!(v1 > false);
                    assert_eq!(v1.cmp(&true.into()), core::cmp::Ordering::Equal);
                    assert_eq!(v1.cmp(&false.into()), core::cmp::Ordering::Greater);

                    assert_eq!(hash(&v1), hash(&true));
                }
            }
        }
    }

    // Verify Guid Manipulations
    //
    // Test that creation of Guids from fields and bytes yields the expected
    // values, and conversions work as expected.
    #[test]
    fn guid() {
        let fields = (
            0x550e8400,
            0xe29b,
            0x41d4,
            0xa7,
            0x16,
            &[0x44, 0x66, 0x55, 0x44, 0x00, 0x00],
        );
        #[rustfmt::skip]
        let bytes = [
            0x00, 0x84, 0x0e, 0x55,
            0x9b, 0xe2,
            0xd4, 0x41,
            0xa7,
            0x16,
            0x44, 0x66, 0x55, 0x44, 0x00, 0x00,
        ];
        let (f0, f1, f2, f3, f4, f5) = fields;
        let g_fields = Guid::from_fields(f0, f1, f2, f3, f4, f5);
        let g_bytes = Guid::from_bytes(&bytes);

        assert_eq!(g_fields, g_bytes);
        assert_eq!(g_fields.as_bytes(), &bytes);
        assert_eq!(g_bytes.as_fields(), fields);
    }
}
