#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use num_derive::FromPrimitive;

/// Values for
/// [`MINIDUMP_EXCEPTION::exception_code`](crate::format::MINIDUMP_EXCEPTION::exception_code)
/// for crashes on macOS.
///
/// Based on Darwin/macOS' [osfmk/mach/exception_types.h][header]. This is what macOS calls an "exception",
/// not a "code".
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/exception_types.h#L64-L105
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMac {
    /// code can be a kern_return_t
    EXC_BAD_ACCESS = 1,
    /// code is CPU-specific
    EXC_BAD_INSTRUCTION = 2,
    /// code is CPU-specific
    EXC_ARITHMETIC = 3,
    /// code is CPU-specific
    EXC_EMULATION = 4,
    EXC_SOFTWARE = 5,
    /// code is CPU-specific
    EXC_BREAKPOINT = 6,
    EXC_SYSCALL = 7,
    EXC_MACH_SYSCALL = 8,
    EXC_RPC_ALERT = 9,
    EXC_RESOURCE = 11,
    EXC_GUARD = 12,
    /// Fake exception code used by Crashpad's SimulateCrash ('CPsx')
    SIMULATED = 0x43507378,
}

/// Mac/iOS Kernel Bad Access Exceptions
///
/// These are the relevant kern_return_t values from [osfmk/mach/kern_return.h][header]
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/kern_return.h#L70-L340
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBadAccessKernType {
    KERN_INVALID_ADDRESS = 1,
    KERN_PROTECTION_FAILURE = 2,
    KERN_FAILURE = 5,
    KERN_NO_ACCESS = 8,
    KERN_MEMORY_FAILURE = 9,
    KERN_MEMORY_ERROR = 10,
    KERN_CODESIGN_ERROR = 50,
}

/// Mac/iOS ARM Userland Bad Accesses Exceptions
///
/// See the [osfmk/mach/arm/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/arm/exception.h#L66-L75
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBadAccessArmType {
    EXC_ARM_DA_ALIGN = 0x0101,
    EXC_ARM_DA_DEBUG = 0x0102,
    EXC_ARM_SP_ALIGN = 0x0103,
    EXC_ARM_SWP = 0x0104,
    EXC_ARM_PAC_FAIL = 0x0105,
}

/// Mac/iOS PowerPC Userland Bad Access Exceptions
///
/// See the [osfmk/mach/ppc/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/b472f0612b8556cd1c6eb1c285ec1953de759e35/osfmk/mach/ppc/exception.h#L71-L78
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBadAccessPpcType {
    EXC_PPC_VM_PROT_READ = 0x0101,
    EXC_PPC_BADSPACE = 0x0102,
    EXC_PPC_UNALIGNED = 0x0103,
}

/// Mac/iOS x86 Userland Bad Access Exceptions
///
/// See the [osfmk/mach/i386/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/i386/exception.h#L122
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBadAccessX86Type {
    EXC_I386_GPFLT = 13,
}

/// Mac/iOS ARM Bad Instruction Exceptions
///
/// See the [osfmk/mach/arm/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/arm/exception.h#L48-L52
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBadInstructionArmType {
    EXC_ARM_UNDEFINED = 1,
}

/// Mac/iOS PowerPC Bad Instruction Exceptions
///
/// See the [osfmk/mach/ppc/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/b472f0612b8556cd1c6eb1c285ec1953de759e35/osfmk/mach/ppc/exception.h#L60-L69
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBadInstructionPpcType {
    EXC_PPC_INVALID_SYSCALL = 1,
    EXC_PPC_UNIPL_INST = 2,
    EXC_PPC_PRIVINST = 3,
    EXC_PPC_PRIVREG = 4,
    EXC_PPC_TRACE = 5,
    EXC_PPC_PERFMON = 6,
}

/// Mac/iOS x86 Bad Instruction Exceptions
///
/// See the [osfmk/mach/i386/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/i386/exception.h#L74-L78
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBadInstructionX86Type {
    /// Invalid Operation
    EXC_I386_INVOP = 1,

    // The rest of these are raw x86 interrupt codes.
    /// Invalid Task State Segment
    EXC_I386_INVTSSFLT = 10,
    /// Segment Not Present
    EXC_I386_SEGNPFLT = 11,
    /// Stack Fault
    EXC_I386_STKFLT = 12,
    /// General Protection Fault
    EXC_I386_GPFLT = 13,
    /// Alignment Fault
    EXC_I386_ALIGNFLT = 17,
    // For sake of completeness, here's the interrupt codes that won't show up here (and why):

    // EXC_I386_DIVERR    =  0: mapped to EXC_ARITHMETIC/EXC_I386_DIV
    // EXC_I386_SGLSTP    =  1: mapped to EXC_BREAKPOINT/EXC_I386_SGL
    // EXC_I386_NMIFLT    =  2: should not occur in user space
    // EXC_I386_BPTFLT    =  3: mapped to EXC_BREAKPOINT/EXC_I386_BPT
    // EXC_I386_INTOFLT   =  4: mapped to EXC_ARITHMETIC/EXC_I386_INTO
    // EXC_I386_BOUNDFLT  =  5: mapped to EXC_ARITHMETIC/EXC_I386_BOUND
    // EXC_I386_INVOPFLT  =  6: mapped to EXC_BAD_INSTRUCTION/EXC_I386_INVOP
    // EXC_I386_NOEXTFLT  =  7: should be handled by the kernel
    // EXC_I386_DBLFLT    =  8: should be handled (if possible) by the kernel
    // EXC_I386_EXTOVRFLT =  9: mapped to EXC_BAD_ACCESS/(PROT_READ|PROT_EXEC)
    // EXC_I386_PGFLT     = 14: should not occur in user space
    // EXC_I386_EXTERRFLT = 16: mapped to EXC_ARITHMETIC/EXC_I386_EXTERR
    // EXC_I386_ENOEXTFLT = 32: should be handled by the kernel
    // EXC_I386_ENDPERR   = 33: should not occur
}

/// Mac/iOS ARM Arithmetic Exceptions
///
/// See the [osfmk/mach/arm/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/arm/exception.h#L54-L64
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacArithmeticArmType {
    EXC_ARM_FP_IO = 1,
    EXC_ARM_FP_DZ = 2,
    EXC_ARM_FP_OF = 3,
    EXC_ARM_FP_UF = 4,
    EXC_ARM_FP_IX = 5,
    EXC_ARM_FP_ID = 6,
}

/// Mac/iOS PowerPC Arithmetic Exceptions
///
/// See the [osfmk/mach/ppc/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/b472f0612b8556cd1c6eb1c285ec1953de759e35/osfmk/mach/ppc/exception.h#L80-L90
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacArithmeticPpcType {
    /// Integer ovrflow
    EXC_PPC_OVERFLOW = 1,
    /// Integer Divide-By-Zero
    EXC_PPC_ZERO_DIVIDE = 2,
    /// Float Inexact
    EXC_FLT_INEXACT = 3,
    /// Float Divide-By-Zero
    EXC_PPC_FLT_ZERO_DIVIDE = 4,
    /// Float Underflow
    EXC_PPC_FLT_UNDERFLOW = 5,
    /// Float Overflow
    EXC_PPC_FLT_OVERFLOW = 6,
    /// Float Not A Number
    EXC_PPC_FLT_NOT_A_NUMBER = 7,

    // NOTE: comments in breakpad suggest these two are actually supposed to be
    // for ExceptionCodeMac::EXC_EMULATION, but for now let's duplicate breakpad.
    EXC_PPC_NOEMULATION = 8,
    EXC_PPC_ALTIVECASSIST = 9,
}

/// Mac/iOS x86 Arithmetic Exceptions
///
/// See the [osfmk/mach/i386/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/i386/exception.h#L80-L91
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacArithmeticX86Type {
    EXC_I386_DIV = 1,
    EXC_I386_INTO = 2,
    EXC_I386_NOEXT = 3,
    EXC_I386_EXTOVR = 4,
    EXC_I386_EXTERR = 5,
    EXC_I386_EMERR = 6,
    EXC_I386_BOUND = 7,
    EXC_I386_SSEEXTERR = 8,
}

/// Mac/iOS "Software" Exceptions
///
/// See the [bsd/sys/ux_exception.h][header1] and [osfmk/mach/ppc/exception.h][header2]
/// headers in Apple's kernel sources
///
/// [header1]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/bsd/sys/ux_exception.h#L48-L52
/// [header2]: https://github.com/apple/darwin-xnu/blob/b472f0612b8556cd1c6eb1c285ec1953de759e35/osfmk/mach/ppc/exception.h#L100-L105
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacSoftwareType {
    SIGABRT = 0x00010002u32,
    UNCAUGHT_NS_EXCEPTION = 0xDEADC0DE,
    EXC_PPC_TRAP = 0x00000001,
    EXC_PPC_MIGRATE = 0x00010100,
    // Breakpad also defines these doesn't use them for Software crashes
    // SIGSYS  = 0x00010000,
    // SIGPIPE = 0x00010001,
}

/// Mac/iOS ARM Breakpoint Exceptions
///
/// See the [osfmk/mach/arm/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/arm/exception.h#L77-L81
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBreakpointArmType {
    EXC_ARM_BREAKPOINT = 1,
}

/// Mac/iOS PowerPC Breakpoint Exceptions
///
/// See the [osfmk/mach/ppc/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/b472f0612b8556cd1c6eb1c285ec1953de759e35/osfmk/mach/ppc/exception.h#L108-L112
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBreakpointPpcType {
    EXC_PPC_BREAKPOINT = 1,
}

/// Mac/iOS x86 Breakpoint Exceptions
///
/// See the [osfmk/mach/i386/exception.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/i386/exception.h#L102-L107
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacBreakpointX86Type {
    EXC_I386_SGL = 1,
    EXC_I386_BPT = 2,
}

/// Mac/iOS Resource exception types
///
/// See the [osfmk/kern/exc_resource.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/kern/exc_resource.h#L60-L65
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacResourceType {
    RESOURCE_TYPE_CPU = 1,
    RESOURCE_TYPE_WAKEUPS = 2,
    RESOURCE_TYPE_MEMORY = 3,
    RESOURCE_TYPE_IO = 4,
    RESOURCE_TYPE_THREADS = 5,
}

/// Mac/iOS CPU resource exception flavors
///
/// See the [osfmk/kern/exc_resource.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/kern/exc_resource.h#L67-L69
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacResourceCpuFlavor {
    FLAVOR_CPU_MONITOR = 1,
    FLAVOR_CPU_MONITOR_FATAL = 2,
}

/// Mac/iOS wakeups resource exception flavors
///
/// See the [osfmk/kern/exc_resource.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/kern/exc_resource.h#L67-L69
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacResourceWakeupsFlavor {
    FLAVOR_WAKEUPS_MONITOR = 1,
}

/// Mac/iOS memory resource exception flavors
///
/// See the [osfmk/kern/exc_resource.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/kern/exc_resource.h#L102-L103
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacResourceMemoryFlavor {
    FLAVOR_HIGH_WATERMARK = 1,
}

/// Mac/iOS I/O resource exception flavors
///
/// See the [osfmk/kern/exc_resource.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/kern/exc_resource.h#L164-L166
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacResourceIOFlavor {
    FLAVOR_IO_PHYSICAL_WRITES = 1,
    FLAVOR_IO_LOGICAL_WRITES = 2,
}

/// Mac/iOS threads resource exception flavors
///
/// See the [osfmk/kern/exc_resource.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/kern/exc_resource.h#L136-L137
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacResourceThreadsFlavor {
    FLAVOR_THREADS_HIGH_WATERMARK = 1,
}

/// Mac/iOS Guard exception types
///
/// See the [osfmk/kern/exc_guard.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple-oss-distributions/xnu/blob/main/osfmk/kern/exc_guard.h
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacGuardType {
    GUARD_TYPE_NONE = 0,
    GUARD_TYPE_MACH_PORT = 1,
    GUARD_TYPE_FD = 2,
    GUARD_TYPE_USER = 3,
    GUARD_TYPE_VN = 4,
    GUARD_TYPE_VIRT_MEMORY = 5,
    GUARD_TYPE_REJECTED_SC = 6,
}

/// Mac/iOS Mach port guard exception flavors
///
/// See the [osfmk/mach/port.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple-oss-distributions/xnu/blob/main/osfmk/mach/port.h
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacGuardMachPortFlavor {
    GUARD_EXC_DESTROY = 1,
    GUARD_EXC_MOD_REFS = 2,
    GUARD_EXC_INVALID_OPTIONS = 3,
    GUARD_EXC_SET_CONTEXT = 4,
    GUARD_EXC_THREAD_SET_STATE = 5,
    GUARD_EXC_UNGUARDED = 0x00000008,
    GUARD_EXC_INCORRECT_GUARD = 0x00000010,
    GUARD_EXC_IMMOVABLE = 0x00000020,
    GUARD_EXC_STRICT_REPLY = 0x00000040,
    GUARD_EXC_MSG_FILTERED = 0x00000080,
    GUARD_EXC_INVALID_RIGHT = 0x00000100,
    GUARD_EXC_INVALID_NAME = 0x00000200,
    GUARD_EXC_INVALID_VALUE = 0x00000400,
    GUARD_EXC_INVALID_ARGUMENT = 0x00000800,
    GUARD_EXC_RIGHT_EXISTS = 0x00001000,
    GUARD_EXC_KERN_NO_SPACE = 0x00002000,
    GUARD_EXC_KERN_FAILURE = 0x00004000,
    GUARD_EXC_KERN_RESOURCE = 0x00008000,
    GUARD_EXC_SEND_INVALID_REPLY = 0x00010000,
    GUARD_EXC_SEND_INVALID_VOUCHER = 0x00020000,
    GUARD_EXC_SEND_INVALID_RIGHT = 0x00040000,
    GUARD_EXC_RCV_INVALID_NAME = 0x00080000,
    GUARD_EXC_RCV_GUARDED_DESC = 0x00100000,
    GUARD_EXC_MOD_REFS_NON_FATAL = 0x00200000,
    GUARD_EXC_IMMOVABLE_NON_FATAL = 0x00400000,
    GUARD_EXC_REQUIRE_REPLY_PORT_SEMANTICS = 0x00800000,
    GUARD_EXC_EXCEPTION_BEHAVIOR_ENFORCE = 0x01000000,
}

/// Mac/iOS fd guard exception flavors
///
/// See the [bsd/sys/guarded.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/main/bsd/sys/guarded.h
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacGuardFDFlavor {
    GUARD_EXC_CLOSE = 0x00000001,
    GUARD_EXC_DUP = 0x00000002,
    GUARD_EXC_NOCLOEXEC = 0x00000004,
    GUARD_EXC_SOCKET_IPC = 0x00000008,
    GUARD_EXC_FILEPORT = 0x00000010,
    GUARD_EXC_MISMATCH = 0x00000020,
    GUARD_EXC_WRITE = 0x00000040,
}

/// Mac/iOS vnode guard exception flavors
///
/// See the [bsd/sys/guarded.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/main/bsd/sys/guarded.h
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacGuardVNFlavor {
    GUARD_EXC_RENAME_TO = 0x00000001,
    GUARD_EXC_RENAME_FROM = 0x00000002,
    GUARD_EXC_UNLINK = 0x00000004,
    GUARD_EXC_WRITE_OTHER = 0x00000008,
    GUARD_EXC_TRUNC_OTHER = 0x00000010,
    GUARD_EXC_LINK = 0x00000020,
    GUARD_EXC_EXCHDATA = 0x00000040,
}

/// Mac/iOS virtual memory guard exception flavors
///
/// See the [osfmk/mach/vm_statistics.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple/darwin-xnu/blob/main/osfmk/mach/vm_statistics.h
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacGuardVirtMemoryFlavor {
    GUARD_EXC_DEALLOC_GAP = 0x00000001,
}

/// Mac/iOS rejected syscall guard exception flavors
///
/// See the [osfmk/kern/exc_guard.h][header] header in Apple's kernel sources
///
/// [header]: https://github.com/apple-oss-distributions/xnu/blob/1031c584a5e37aff177559b9f69dbd3c8c3fd30a/osfmk/kern/exc_guard.h#L149-L163
#[derive(Copy, Clone, PartialEq, Eq, Debug, FromPrimitive)]
pub enum ExceptionCodeMacGuardRejecteSysCallFlavor {
    GUARD_EXC_MACH_TRAP = 0x00000000,
}
