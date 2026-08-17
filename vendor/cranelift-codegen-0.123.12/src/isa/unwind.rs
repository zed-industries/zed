//! Represents information relating to function unwinding.

use crate::machinst::RealReg;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "unwind")]
pub mod systemv;

#[cfg(feature = "unwind")]
pub mod winx64;

#[cfg(feature = "unwind")]
pub mod winarm64;

/// CFA-based unwind information used on SystemV.
pub type CfaUnwindInfo = systemv::UnwindInfo;

/// Expected unwind info type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnwindInfoKind {
    /// No unwind info.
    None,
    /// SystemV CIE/FDE unwind info.
    #[cfg(feature = "unwind")]
    SystemV,
    /// Windows X64 Unwind info
    #[cfg(feature = "unwind")]
    Windows,
}

/// Represents unwind information for a single function.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum UnwindInfo {
    /// Windows x64 ABI unwind information.
    #[cfg(feature = "unwind")]
    WindowsX64(winx64::UnwindInfo),
    /// System V ABI unwind information.
    #[cfg(feature = "unwind")]
    SystemV(CfaUnwindInfo),
    /// Windows Arm64 ABI unwind information.
    #[cfg(feature = "unwind")]
    WindowsArm64(winarm64::UnwindInfo),
}

/// Unwind pseudoinstruction used in VCode backends: represents that
/// at the present location, an action has just been taken.
///
/// VCode backends always emit unwind info that is relative to a frame
/// pointer, because we are planning to allow for dynamic frame allocation,
/// and because it makes the design quite a lot simpler in general: we don't
/// have to be precise about SP adjustments throughout the body of the function.
///
/// We include only unwind info for prologues at this time. Note that unwind
/// info for epilogues is only necessary if one expects to unwind while within
/// the last few instructions of the function (after FP has been restored) or
/// if one wishes to instruction-step through the epilogue and see a backtrace
/// at every point. This is not necessary for correct operation otherwise and so
/// we simplify the world a bit by omitting epilogue information. (Note that
/// some platforms also don't require or have a way to describe unwind
/// information for epilogues at all: for example, on Windows, the `UNWIND_INFO`
/// format only stores information for the function prologue.)
///
/// Because we are defining an abstraction over multiple unwind formats (at
/// least Windows/fastcall and System V) and multiple architectures (at least
/// x86-64 and aarch64), we have to be a little bit flexible in how we describe
/// the frame. However, it turns out that a least-common-denominator prologue
/// works for all of the cases we have to worry about today!
///
/// We assume the stack looks something like this:
///
///
/// ```plain
///                  +----------------------------------------------+
///                  | stack arg area, etc (according to ABI)       |
///                  | ...                                          |
///   SP at call --> +----------------------------------------------+
///                  | return address (pushed by HW or SW)          |
///                  +----------------------------------------------+
///                  | old frame pointer (FP)                       |
///   FP in this --> +----------------------------------------------+
///   function       | clobbered callee-save registers              |
///                  | ...                                          |
///   start of   --> +----------------------------------------------+
///   clobbers       | (rest of function's frame, irrelevant here)  |
///                  | ...                                          |
///   SP in this --> +----------------------------------------------+
///   function
/// ```
///
/// We assume that the prologue consists of:
///
/// * `PushFrameRegs`: A push operation that adds the old FP to the stack (and
///    maybe the link register, on architectures that do not push return addresses
///    in hardware)
/// * `DefineFrame`: An update that sets FP to SP to establish a new frame
/// * `SaveReg`: A number of stores or pushes to the stack to save clobbered registers
///
/// Each of these steps has a corresponding pseudo-instruction. At each step,
/// we need some information to determine where the current stack frame is
/// relative to SP or FP. When the `PushFrameRegs` occurs, we need to know how
/// much SP was decremented by, so we can allow the unwinder to continue to find
/// the caller's frame. When we define the new frame, we need to know where FP
/// is in relation to "SP at call" and also "start of clobbers", because
/// different unwind formats define one or the other of those as the anchor by
/// which we define the frame. Finally, when registers are saved, we need to
/// know which ones, and where.
///
/// Different unwind formats work differently; here is a whirlwind tour of how
/// they define frames to help understanding:
///
/// - Windows unwind information defines a frame that must start below the
///   clobber area, because all clobber-save offsets are non-negative. We set it
///   at the "start of clobbers" in the figure above. The `UNWIND_INFO` contains
///   a "frame pointer offset" field; when we define the new frame, the frame is
///   understood to be the value of FP (`RBP`) *minus* this offset. In other
///   words, the FP is *at the frame pointer offset* relative to the
///   start-of-clobber-frame. We use the "FP offset down to clobber area" offset
///   to generate this info.
///
/// - System V unwind information defines a frame in terms of the CFA
///   (call-frame address), which is equal to the "SP at call" above. SysV
///   allows negative offsets, so there is no issue defining clobber-save
///   locations in terms of CFA. The format allows us to define CFA flexibly in
///   terms of any register plus an offset; we define it in terms of FP plus
///   the clobber-to-caller-SP offset once FP is established.
///
/// Note that certain architectures impose limits on offsets: for example, on
/// Windows, the base of the clobber area must not be more than 240 bytes below
/// FP.
///
/// Unwind pseudoinstructions are emitted inline by ABI code as it generates
/// a prologue. Thus, for the usual case, a prologue might look like (using x64
/// as an example):
///
/// ```plain
/// push rbp
/// unwind UnwindInst::PushFrameRegs { offset_upward_to_caller_sp: 16 }
/// mov rbp, rsp
/// unwind UnwindInst::DefineNewFrame { offset_upward_to_caller_sp: 16,
///                                     offset_downward_to_clobbers: 16 }
/// sub rsp, 32
/// mov [rsp+16], r12
/// unwind UnwindInst::SaveReg { reg: R12, clobber_offset: 0 }
/// mov [rsp+24], r13
/// unwind UnwindInst::SaveReg { reg: R13, clobber_offset: 8 }
/// ...
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum UnwindInst {
    /// The frame-pointer register for this architecture has just been pushed to
    /// the stack (and on architectures where return-addresses are not pushed by
    /// hardware, the link register as well). The FP has not been set to this
    /// frame yet. The current location of SP is such that
    /// `offset_upward_to_caller_sp` is the distance to SP-at-callsite (our
    /// caller's frame).
    PushFrameRegs {
        /// The offset from the current SP (after push) to the SP at
        /// caller's callsite.
        offset_upward_to_caller_sp: u32,
    },
    /// The frame-pointer register for this architecture has just been
    /// set to the current stack location. We wish to define a new
    /// frame that is anchored on this new FP value. Offsets are provided
    /// upward to the caller's stack frame and downward toward the clobber
    /// area. We expect this pseudo-op to come after `PushFrameRegs`.
    DefineNewFrame {
        /// The offset from the current SP and FP value upward to the value of
        /// SP at the callsite that invoked us.
        offset_upward_to_caller_sp: u32,
        /// The offset from the current SP and FP value downward to the start of
        /// the clobber area.
        offset_downward_to_clobbers: u32,
    },
    /// The stack pointer was adjusted to allocate the stack.
    StackAlloc {
        /// Size to allocate.
        size: u32,
    },
    /// The stack slot at the given offset from the clobber-area base has been
    /// used to save the given register.
    ///
    /// Given that `CreateFrame` has occurred first with some
    /// `offset_downward_to_clobbers`, `SaveReg` with `clobber_offset` indicates
    /// that the value of `reg` is saved on the stack at address `FP -
    /// offset_downward_to_clobbers + clobber_offset`.
    SaveReg {
        /// The offset from the start of the clobber area to this register's
        /// stack location.
        clobber_offset: u32,
        /// The saved register.
        reg: RealReg,
    },
    /// Computes the value of the given register in the caller as stack offset.
    /// Typically used to unwind the stack pointer if the default rule does not apply.
    /// The `clobber_offset` is computed the same way as for the `SaveReg` rule.
    RegStackOffset {
        /// The offset from the start of the clobber area to this register's value.
        clobber_offset: u32,
        /// The register whose value is to be set.
        reg: RealReg,
    },
    /// Defines if the aarch64-specific pointer authentication available for ARM v8.3+ devices
    /// is enabled for certain pointers or not.
    Aarch64SetPointerAuth {
        /// Whether return addresses (hold in LR) contain a pointer-authentication code.
        return_addresses: bool,
    },
}

struct Writer<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Writer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }

    fn write_u8(&mut self, v: u8) {
        self.buf[self.offset] = v;
        self.offset += 1;
    }

    fn write_u16_le(&mut self, v: u16) {
        self.buf[self.offset..(self.offset + 2)].copy_from_slice(&v.to_le_bytes());
        self.offset += 2;
    }

    fn write_u16_be(&mut self, v: u16) {
        self.buf[self.offset..(self.offset + 2)].copy_from_slice(&v.to_be_bytes());
        self.offset += 2;
    }

    fn write_u32_le(&mut self, v: u32) {
        self.buf[self.offset..(self.offset + 4)].copy_from_slice(&v.to_le_bytes());
        self.offset += 4;
    }

    fn write_u32_be(&mut self, v: u32) {
        self.buf[self.offset..(self.offset + 4)].copy_from_slice(&v.to_be_bytes());
        self.offset += 4;
    }
}
