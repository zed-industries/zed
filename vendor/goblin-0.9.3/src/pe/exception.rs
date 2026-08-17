//! Exception handling and stack unwinding for x64.
//!
//! Exception information is exposed via the [`ExceptionData`] structure. If present in a PE file,
//! it contains a list of [`RuntimeFunction`] entries that can be used to get [`UnwindInfo`] for a
//! particular code location.
//!
//! Unwind information contains a list of unwind codes which specify the operations that are
//! necessary to restore registers (including the stack pointer RSP) when unwinding out of a
//! function.
//!
//! Depending on where the instruction pointer lies, there are three strategies to unwind:
//!
//!  1. If the RIP is within an epilog, then control is leaving the function, there can be no
//!     exception handler associated with this exception for this function, and the effects of the
//!     epilog must be continued to compute the context of the caller function. To determine if the
//!     RIP is within an epilog, the code stream from RIP on is examined. If that code stream can be
//!     matched to the trailing portion of a legitimate epilog, then it's in an epilog, and the
//!     remaining portion of the epilog is simulated, with the context record updated as each
//!     instruction is processed. After this, step 1 is repeated.
//!
//!  2. Case b) If the RIP lies within the prologue, then control has not entered the function,
//!     there can be no exception handler associated with this exception for this function, and the
//!     effects of the prolog must be undone to compute the context of the caller function. The RIP
//!     is within the prolog if the distance from the function start to the RIP is less than or
//!     equal to the prolog size encoded in the unwind info. The effects of the prolog are unwound
//!     by scanning forward through the unwind codes array for the first entry with an offset less
//!     than or equal to the offset of the RIP from the function start, then undoing the effect of
//!     all remaining items in the unwind code array. Step 1 is then repeated.
//!
//!  3. If the RIP is not within a prolog or epilog and the function has an exception handler, then
//!     the language-specific handler is called. The handler scans its data and calls filter
//!     functions as appropriate. The language-specific handler can return that the exception was
//!     handled or that the search is to be continued. It can also initiate an unwind directly.
//!
//! For more information, see [x64 exception handling].
//!
//! [`ExceptionData`]: struct.ExceptionData.html
//! [`RuntimeFunction`]: struct.RuntimeFunction.html
//! [`UnwindInfo`]: struct.UnwindInfo.html
//! [x64 exception handling]: https://docs.microsoft.com/en-us/cpp/build/exception-handling-x64?view=vs-2017

use core::cmp::Ordering;
use core::fmt;
use core::iter::FusedIterator;

use scroll::ctx::TryFromCtx;
use scroll::{self, Pread, Pwrite};

use crate::error;

use crate::pe::data_directories;
use crate::pe::options;
use crate::pe::section_table;
use crate::pe::utils;

/// The function has an exception handler that should be called when looking for functions that need
/// to examine exceptions.
const UNW_FLAG_EHANDLER: u8 = 0x01;
/// The function has a termination handler that should be called when unwinding an exception.
const UNW_FLAG_UHANDLER: u8 = 0x02;
/// This unwind info structure is not the primary one for the procedure. Instead, the chained unwind
/// info entry is the contents of a previous `RUNTIME_FUNCTION` entry. If this flag is set, then the
/// `UNW_FLAG_EHANDLER` and `UNW_FLAG_UHANDLER` flags must be cleared. Also, the frame register and
/// fixed-stack allocation fields must have the same values as in the primary unwind info.
const UNW_FLAG_CHAININFO: u8 = 0x04;

/// info == register number
const UWOP_PUSH_NONVOL: u8 = 0;
/// no info, alloc size in next 2 slots
const UWOP_ALLOC_LARGE: u8 = 1;
/// info == size of allocation / 8 - 1
const UWOP_ALLOC_SMALL: u8 = 2;
/// no info, FP = RSP + UNWIND_INFO.FPRegOffset*16
const UWOP_SET_FPREG: u8 = 3;
/// info == register number, offset in next slot
const UWOP_SAVE_NONVOL: u8 = 4;
/// info == register number, offset in next 2 slots
const UWOP_SAVE_NONVOL_FAR: u8 = 5;
/// changes the structure of unwind codes to `struct Epilogue`.
/// (was UWOP_SAVE_XMM in version 1, but deprecated and removed)
const UWOP_EPILOG: u8 = 6;
/// reserved
/// (was UWOP_SAVE_XMM_FAR in version 1, but deprecated and removed)
const UWOP_SPARE_CODE: u8 = 7;
/// info == XMM reg number, offset in next slot
const UWOP_SAVE_XMM128: u8 = 8;
/// info == XMM reg number, offset in next 2 slots
const UWOP_SAVE_XMM128_FAR: u8 = 9;
/// info == 0: no error-code, 1: error-code
const UWOP_PUSH_MACHFRAME: u8 = 10;

/// Size of `RuntimeFunction` entries.
const RUNTIME_FUNCTION_SIZE: usize = 12;
/// Size of unwind code slots. Codes take 1 - 3 slots.
const UNWIND_CODE_SIZE: usize = 2;

/// An unwind entry for a range of a function.
///
/// Unwind information for this function can be loaded with [`ExceptionData::get_unwind_info`].
///
/// [`ExceptionData::get_unwind_info`]: struct.ExceptionData.html#method.get_unwind_info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default, Pread, Pwrite)]
pub struct RuntimeFunction {
    /// Function start address.
    pub begin_address: u32,
    /// Function end address.
    pub end_address: u32,
    /// Unwind info address.
    pub unwind_info_address: u32,
}

impl fmt::Debug for RuntimeFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RuntimeFunction")
            .field("begin_address", &format_args!("{:#x}", self.begin_address))
            .field("end_address", &format_args!("{:#x}", self.end_address))
            .field(
                "unwind_info_address",
                &format_args!("{:#x}", self.unwind_info_address),
            )
            .finish()
    }
}

/// Iterator over runtime function entries in [`ExceptionData`](struct.ExceptionData.html).
#[derive(Debug)]
pub struct RuntimeFunctionIterator<'a> {
    data: &'a [u8],
}

impl Iterator for RuntimeFunctionIterator<'_> {
    type Item = error::Result<RuntimeFunction>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        Some(match self.data.pread_with(0, scroll::LE) {
            Ok(func) => {
                self.data = &self.data[RUNTIME_FUNCTION_SIZE..];
                Ok(func)
            }
            Err(error) => {
                self.data = &[];
                Err(error.into())
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.data.len() / RUNTIME_FUNCTION_SIZE;
        (len, Some(len))
    }
}

impl FusedIterator for RuntimeFunctionIterator<'_> {}
impl ExactSizeIterator for RuntimeFunctionIterator<'_> {}

/// An x64 register used during unwinding.
///
///  - `0` - `15`: General purpose registers
///  - `17` - `32`: XMM registers
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Register(pub u8);

impl Register {
    fn xmm(number: u8) -> Self {
        Register(number + 17)
    }

    /// Returns the x64 register name.
    pub fn name(self) -> &'static str {
        match self.0 {
            0 => "$rax",
            1 => "$rcx",
            2 => "$rdx",
            3 => "$rbx",
            4 => "$rsp",
            5 => "$rbp",
            6 => "$rsi",
            7 => "$rdi",
            8 => "$r8",
            9 => "$r9",
            10 => "$r10",
            11 => "$r11",
            12 => "$r12",
            13 => "$r13",
            14 => "$r14",
            15 => "$r15",
            16 => "$rip",
            17 => "$xmm0",
            18 => "$xmm1",
            19 => "$xmm2",
            20 => "$xmm3",
            21 => "$xmm4",
            22 => "$xmm5",
            23 => "$xmm6",
            24 => "$xmm7",
            25 => "$xmm8",
            26 => "$xmm9",
            27 => "$xmm10",
            28 => "$xmm11",
            29 => "$xmm12",
            30 => "$xmm13",
            31 => "$xmm14",
            32 => "$xmm15",
            _ => "",
        }
    }
}

/// An unsigned offset to a value in the local stack frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StackFrameOffset {
    /// Offset from the current RSP, that is, the lowest address of the fixed stack allocation.
    ///
    /// To restore this register, read the value at the given offset from the RSP.
    RSP(u32),

    /// Offset from the value of the frame pointer register.
    ///
    /// To restore this register, read the value at the given offset from the FP register, reduced
    /// by the `frame_register_offset` value specified in the `UnwindInfo` structure. By definition,
    /// the frame pointer register is any register other than RAX (`0`).
    FP(u32),
}

impl StackFrameOffset {
    fn with_ctx(offset: u32, ctx: UnwindOpContext) -> Self {
        match ctx.frame_register {
            Register(0) => StackFrameOffset::RSP(offset),
            Register(_) => StackFrameOffset::FP(offset),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// An unwind operation corresponding to code in the function prolog.
///
/// Unwind operations can be used to reverse the effects of the function prolog and restore register
/// values of parent stack frames that have been saved to the stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnwindOperation {
    /// Push a nonvolatile integer register, decrementing `RSP` by 8.
    PushNonVolatile(Register),

    /// Allocate a fixed-size area on the stack.
    Alloc(u32),

    /// Establish the frame pointer register by setting the register to some offset of the current
    /// RSP. The use of an offset permits establishing a frame pointer that points to the middle of
    /// the fixed stack allocation, helping code density by allowing more accesses to use short
    /// instruction forms.
    SetFPRegister,

    /// Save a nonvolatile integer register on the stack using a MOV instead of a PUSH. This code is
    /// primarily used for shrink-wrapping, where a nonvolatile register is saved to the stack in a
    /// position that was previously allocated.
    SaveNonVolatile(Register, StackFrameOffset),

    /// Save the lower 64 bits of a nonvolatile XMM register on the stack.
    SaveXMM(Register, StackFrameOffset),

    /// Describes the function epilog.
    ///
    /// This operation has been introduced with unwind info version 2 and is not implemented yet.
    Epilog,

    /// Save all 128 bits of a nonvolatile XMM register on the stack.
    SaveXMM128(Register, StackFrameOffset),

    /// Push a machine frame. This is used to record the effect of a hardware interrupt or
    /// exception. Depending on the error flag, this frame has two different layouts.
    ///
    /// This unwind code always appears in a dummy prolog, which is never actually executed but
    /// instead appears before the real entry point of an interrupt routine, and exists only to
    /// provide a place to simulate the push of a machine frame. This operation records that
    /// simulation, which indicates the machine has conceptually done this:
    ///
    ///  1. Pop RIP return address from top of stack into `temp`
    ///  2. `$ss`, Push old `$rsp`, `$rflags`, `$cs`, `temp`
    ///  3. If error flag is `true`, push the error code
    ///
    /// Without an error code, RSP was incremented by `40` and the following was frame pushed:
    ///
    /// Offset   | Value
    /// ---------|--------
    /// RSP + 32 | `$ss`
    /// RSP + 24 | old `$rsp`
    /// RSP + 16 | `$rflags`
    /// RSP +  8 | `$cs`
    /// RSP +  0 | `$rip`
    ///
    /// With an error code, RSP was incremented by `48` and the following was frame pushed:
    ///
    /// Offset   | Value
    /// ---------|--------
    /// RSP + 40 | `$ss`
    /// RSP + 32 | old `$rsp`
    /// RSP + 24 | `$rflags`
    /// RSP + 16 | `$cs`
    /// RSP +  8 | `$rip`
    /// RSP +  0 | error code
    PushMachineFrame(bool),

    /// A reserved operation without effect.
    Noop,
}

/// Context used to parse unwind operation.
#[derive(Clone, Copy, Debug, PartialEq)]
struct UnwindOpContext {
    /// Version of the unwind info.
    version: u8,

    /// The nonvolatile register used as the frame pointer of this function.
    ///
    /// If this register is non-zero, all stack frame offsets used in unwind operations are of type
    /// `StackFrameOffset::FP`. When loading these offsets, they have to be based off the value of
    /// this frame register instead of the conventional RSP. This allows the RSP to be modified.
    frame_register: Register,
}

/// An unwind operation that is executed at a particular place in the function prolog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnwindCode {
    /// Offset of the corresponding instruction in the function prolog.
    ///
    /// To be precise, this is the offset from the beginning of the prolog of the end of the
    /// instruction that performs this operation, plus 1 (that is, the offset of the start of the
    /// next instruction).
    ///
    /// Unwind codes are ordered by this offset in reverse order, suitable for unwinding.
    pub code_offset: u8,

    /// The operation that was performed by the code in the prolog.
    pub operation: UnwindOperation,
}

impl<'a> TryFromCtx<'a, UnwindOpContext> for UnwindCode {
    type Error = error::Error;
    #[inline]
    fn try_from_ctx(bytes: &'a [u8], ctx: UnwindOpContext) -> Result<(Self, usize), Self::Error> {
        let mut read = 0;
        let code_offset = bytes.gread_with::<u8>(&mut read, scroll::LE)?;
        let operation = bytes.gread_with::<u8>(&mut read, scroll::LE)?;

        let operation_code = operation & 0xf;
        let operation_info = operation >> 4;

        let operation = match operation_code {
            self::UWOP_PUSH_NONVOL => {
                let register = Register(operation_info);
                UnwindOperation::PushNonVolatile(register)
            }
            self::UWOP_ALLOC_LARGE => {
                let offset = match operation_info {
                    0 => u32::from(bytes.gread_with::<u16>(&mut read, scroll::LE)?) * 8,
                    1 => bytes.gread_with::<u32>(&mut read, scroll::LE)?,
                    i => {
                        let msg = format!("invalid op info ({}) for UWOP_ALLOC_LARGE", i);
                        return Err(error::Error::Malformed(msg));
                    }
                };
                UnwindOperation::Alloc(offset)
            }
            self::UWOP_ALLOC_SMALL => {
                let offset = u32::from(operation_info) * 8 + 8;
                UnwindOperation::Alloc(offset)
            }
            self::UWOP_SET_FPREG => UnwindOperation::SetFPRegister,
            self::UWOP_SAVE_NONVOL => {
                let register = Register(operation_info);
                let offset = u32::from(bytes.gread_with::<u16>(&mut read, scroll::LE)?) * 8;
                UnwindOperation::SaveNonVolatile(register, StackFrameOffset::with_ctx(offset, ctx))
            }
            self::UWOP_SAVE_NONVOL_FAR => {
                let register = Register(operation_info);
                let offset = bytes.gread_with::<u32>(&mut read, scroll::LE)?;
                UnwindOperation::SaveNonVolatile(register, StackFrameOffset::with_ctx(offset, ctx))
            }
            self::UWOP_EPILOG => {
                let data = u32::from(bytes.gread_with::<u16>(&mut read, scroll::LE)?) * 16;
                if ctx.version == 1 {
                    let register = Register::xmm(operation_info);
                    UnwindOperation::SaveXMM(register, StackFrameOffset::with_ctx(data, ctx))
                } else {
                    // TODO: See https://weekly-geekly.github.io/articles/322956/index.html
                    UnwindOperation::Epilog
                }
            }
            self::UWOP_SPARE_CODE => {
                let data = bytes.gread_with::<u32>(&mut read, scroll::LE)?;
                if ctx.version == 1 {
                    let register = Register::xmm(operation_info);
                    UnwindOperation::SaveXMM128(register, StackFrameOffset::with_ctx(data, ctx))
                } else {
                    UnwindOperation::Noop
                }
            }
            self::UWOP_SAVE_XMM128 => {
                let register = Register::xmm(operation_info);
                let offset = u32::from(bytes.gread_with::<u16>(&mut read, scroll::LE)?) * 16;
                UnwindOperation::SaveXMM128(register, StackFrameOffset::with_ctx(offset, ctx))
            }
            self::UWOP_SAVE_XMM128_FAR => {
                let register = Register::xmm(operation_info);
                let offset = bytes.gread_with::<u32>(&mut read, scroll::LE)?;
                UnwindOperation::SaveXMM128(register, StackFrameOffset::with_ctx(offset, ctx))
            }
            self::UWOP_PUSH_MACHFRAME => {
                let is_error = match operation_info {
                    0 => false,
                    1 => true,
                    i => {
                        let msg = format!("invalid op info ({}) for UWOP_PUSH_MACHFRAME", i);
                        return Err(error::Error::Malformed(msg));
                    }
                };
                UnwindOperation::PushMachineFrame(is_error)
            }
            op => {
                let msg = format!("unknown unwind op code ({})", op);
                return Err(error::Error::Malformed(msg));
            }
        };

        let code = UnwindCode {
            code_offset,
            operation,
        };

        Ok((code, read))
    }
}

/// An iterator over unwind codes for a function or part of a function, returned from
/// [`UnwindInfo`].
///
/// [`UnwindInfo`]: struct.UnwindInfo.html
#[derive(Clone, Debug)]
pub struct UnwindCodeIterator<'a> {
    bytes: &'a [u8],
    offset: usize,
    context: UnwindOpContext,
}

impl Iterator for UnwindCodeIterator<'_> {
    type Item = error::Result<UnwindCode>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.bytes.len() {
            return None;
        }

        Some(self.bytes.gread_with(&mut self.offset, self.context))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper = (self.bytes.len() - self.offset) / UNWIND_CODE_SIZE;
        // the largest codes take up three slots
        let lower = (upper + 3 - (upper % 3)) / 3;
        (lower, Some(upper))
    }
}

impl FusedIterator for UnwindCodeIterator<'_> {}

/// A language-specific handler that is called as part of the search for an exception handler or as
/// part of an unwind.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnwindHandler<'a> {
    /// The image-relative address of an exception handler and its implementation-defined data.
    ExceptionHandler(u32, &'a [u8]),
    /// The image-relative address of a termination handler and its implementation-defined data.
    TerminationHandler(u32, &'a [u8]),
}

/// Unwind information for a function or portion of a function.
///
/// The unwind info structure is used to record the effects a function has on the stack pointer and
/// where the nonvolatile registers are saved on the stack. The unwind codes can be enumerated with
/// [`unwind_codes`].
///
/// This unwind info might only be secondary information, and link to a [chained unwind handler].
/// For unwinding, this link shall be followed until the root unwind info record has been resolved.
///
/// [`unwind_codes`]: struct.UnwindInfo.html#method.unwind_codes
/// [chained unwind handler]: struct.UnwindInfo.html#structfield.chained_info
#[derive(Clone)]
pub struct UnwindInfo<'a> {
    /// Version of this unwind info.
    pub version: u8,

    /// Length of the function prolog in bytes.
    pub size_of_prolog: u8,

    /// The nonvolatile register used as the frame pointer of this function.
    ///
    /// If this register is non-zero, all stack frame offsets used in unwind operations are of type
    /// `StackFrameOffset::FP`. When loading these offsets, they have to be based off the value of
    /// this frame register instead of the conventional RSP. This allows the RSP to be modified.
    pub frame_register: Register,

    /// Offset from RSP that is applied to the FP register when it is established.
    ///
    /// When loading offsets of type `StackFrameOffset::FP` from the stack, this offset has to be
    /// subtracted before loading the value since the actual RSP was lower by that amount in the
    /// prolog.
    pub frame_register_offset: u32,

    /// A record pointing to chained unwind information.
    ///
    /// If chained unwind info is present, then this unwind info is a secondary one and the linked
    /// unwind info contains primary information. Chained info is useful in two situations. First,
    /// it is used for noncontiguous code segments. Second, this mechanism is sometimes used to
    /// group volatile register saves.
    ///
    /// The referenced unwind info can itself specify chained unwind information, until it arrives
    /// at the root unwind info. Generally, the entire chain should be considered when unwinding.
    pub chained_info: Option<RuntimeFunction>,

    /// An exception or termination handler called as part of the unwind.
    pub handler: Option<UnwindHandler<'a>>,

    /// A list of unwind codes, sorted descending by code offset.
    code_bytes: &'a [u8],
}

impl<'a> UnwindInfo<'a> {
    /// Parses unwind information from the image at the given offset.
    pub fn parse(bytes: &'a [u8], mut offset: usize) -> error::Result<Self> {
        // Read the version and flags fields, which are combined into a single byte.
        let version_flags: u8 = bytes.gread_with(&mut offset, scroll::LE)?;
        let version = version_flags & 0b111;
        let flags = version_flags >> 3;

        if version < 1 || version > 2 {
            let msg = format!("unsupported unwind code version ({})", version);
            return Err(error::Error::Malformed(msg));
        }

        let size_of_prolog = bytes.gread_with::<u8>(&mut offset, scroll::LE)?;
        let count_of_codes = bytes.gread_with::<u8>(&mut offset, scroll::LE)?;

        // Parse the frame register and frame register offset values, that are combined into a
        // single byte.
        let frame_info = bytes.gread_with::<u8>(&mut offset, scroll::LE)?;
        // If nonzero, then the function uses a frame pointer (FP), and this field is the number
        // of the nonvolatile register used as the frame pointer. The zero register value does
        // not need special casing since it will not be referenced by the unwind operations.
        let frame_register = Register(frame_info & 0xf);
        // The the scaled offset from RSP that is applied to the FP register when it's
        // established. The actual FP register is set to RSP + 16 * this number, allowing
        // offsets from 0 to 240.
        let frame_register_offset = u32::from((frame_info >> 4) * 16);

        // An array of items that explains the effect of the prolog on the nonvolatile registers and
        // RSP. Some unwind codes require more than one slot in the array.
        let codes_size = count_of_codes as usize * UNWIND_CODE_SIZE;
        let code_bytes = bytes.gread_with(&mut offset, codes_size)?;

        // For alignment purposes, the codes array always has an even number of entries, and the
        // final entry is potentially unused. In that case, the array is one longer than indicated
        // by the count of unwind codes field.
        if count_of_codes % 2 != 0 {
            offset += 2;
        }
        debug_assert!(offset % 4 == 0);

        let mut chained_info = None;
        let mut handler = None;

        // If flag UNW_FLAG_CHAININFO is set then the UNWIND_INFO structure ends with three UWORDs.
        // These UWORDs represent the RUNTIME_FUNCTION information for the function of the chained
        // unwind.
        if flags & UNW_FLAG_CHAININFO != 0 {
            chained_info = Some(bytes.gread_with(&mut offset, scroll::LE)?);

        // The relative address of the language-specific handler is present in the UNWIND_INFO
        // whenever flags UNW_FLAG_EHANDLER or UNW_FLAG_UHANDLER are set. The language-specific
        // handler is called as part of the search for an exception handler or as part of an unwind.
        } else if flags & (UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER) != 0 {
            let address = bytes.gread_with::<u32>(&mut offset, scroll::LE)?;
            let data = &bytes[offset..];

            handler = Some(if flags & UNW_FLAG_EHANDLER != 0 {
                UnwindHandler::ExceptionHandler(address, data)
            } else {
                UnwindHandler::TerminationHandler(address, data)
            });
        }

        Ok(UnwindInfo {
            version,
            size_of_prolog,
            frame_register,
            frame_register_offset,
            chained_info,
            handler,
            code_bytes,
        })
    }

    /// Returns an iterator over unwind codes in this unwind info.
    ///
    /// Unwind codes are iterated in descending `code_offset` order suitable for unwinding. If the
    /// optional [`chained_info`](Self::chained_info) is present, codes of that unwind info should be interpreted
    /// immediately afterwards.
    pub fn unwind_codes(&self) -> UnwindCodeIterator<'a> {
        UnwindCodeIterator {
            bytes: self.code_bytes,
            offset: 0,
            context: UnwindOpContext {
                version: self.version,
                frame_register: self.frame_register,
            },
        }
    }
}

impl fmt::Debug for UnwindInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count_of_codes = self.code_bytes.len() / UNWIND_CODE_SIZE;

        f.debug_struct("UnwindInfo")
            .field("version", &self.version)
            .field("size_of_prolog", &self.size_of_prolog)
            .field("frame_register", &self.frame_register)
            .field("frame_register_offset", &self.frame_register_offset)
            .field("count_of_codes", &count_of_codes)
            .field("chained_info", &self.chained_info)
            .field("handler", &self.handler)
            .finish()
    }
}

impl<'a> IntoIterator for &'_ UnwindInfo<'a> {
    type Item = error::Result<UnwindCode>;
    type IntoIter = UnwindCodeIterator<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.unwind_codes()
    }
}

/// Exception handling and stack unwind information for functions in the image.
pub struct ExceptionData<'a> {
    bytes: &'a [u8],
    offset: usize,
    size: usize,
    file_alignment: u32,
}

impl<'a> ExceptionData<'a> {
    /// Parses exception data from the image at the given offset.
    pub fn parse(
        bytes: &'a [u8],
        directory: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<Self> {
        Self::parse_with_opts(
            bytes,
            directory,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    /// Parses exception data from the image at the given offset.
    pub fn parse_with_opts(
        bytes: &'a [u8],
        directory: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<Self> {
        let size = directory.size as usize;

        if size % RUNTIME_FUNCTION_SIZE != 0 {
            return Err(error::Error::from(scroll::Error::BadInput {
                size,
                msg: "invalid exception directory table size",
            }));
        }

        let rva = directory.virtual_address as usize;
        let offset = utils::find_offset(rva, sections, file_alignment, opts).ok_or_else(|| {
            error::Error::Malformed(format!("cannot map exception_rva ({:#x}) into offset", rva))
        })?;

        if offset % 4 != 0 {
            return Err(error::Error::from(scroll::Error::BadOffset(offset)));
        }

        Ok(ExceptionData {
            bytes,
            offset,
            size,
            file_alignment,
        })
    }

    /// The number of function entries described by this exception data.
    pub fn len(&self) -> usize {
        self.size / RUNTIME_FUNCTION_SIZE
    }

    /// Indicating whether there are functions in this entry.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates all function entries in order of their code offset.
    ///
    /// To search for a function by relative instruction address, use [`find_function`]. To resolve
    /// unwind information, use [`get_unwind_info`].
    ///
    /// [`find_function`]: struct.ExceptionData.html#method.find_function
    /// [`get_unwind_info`]: struct.ExceptionData.html#method.get_unwind_info
    pub fn functions(&self) -> RuntimeFunctionIterator<'a> {
        RuntimeFunctionIterator {
            data: &self.bytes[self.offset..self.offset + self.size],
        }
    }

    /// Returns the function at the given index.
    pub fn get_function(&self, index: usize) -> error::Result<RuntimeFunction> {
        self.get_function_by_offset(self.offset + index * RUNTIME_FUNCTION_SIZE)
    }

    /// Performs a binary search to find a function entry covering the given RVA relative to the
    /// image.
    pub fn find_function(&self, rva: u32) -> error::Result<Option<RuntimeFunction>> {
        // NB: Binary search implementation copied from std::slice::binary_search_by and adapted.
        // Theoretically, there should be nothing that causes parsing runtime functions to fail and
        // all access to the bytes buffer is guaranteed to be in range. However, since all other
        // functions also return Results, this is much more ergonomic here.

        let mut size = self.len();
        if size == 0 {
            return Ok(None);
        }

        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let offset = self.offset + mid * RUNTIME_FUNCTION_SIZE;
            let addr = self.bytes.pread_with::<u32>(offset, scroll::LE)?;
            base = if addr > rva { base } else { mid };
            size -= half;
        }

        let offset = self.offset + base * RUNTIME_FUNCTION_SIZE;
        let addr = self.bytes.pread_with::<u32>(offset, scroll::LE)?;
        let function = match addr.cmp(&rva) {
            Ordering::Less | Ordering::Equal => self.get_function(base)?,
            Ordering::Greater if base == 0 => return Ok(None),
            Ordering::Greater => self.get_function(base - 1)?,
        };

        if function.end_address > rva {
            Ok(Some(function))
        } else {
            Ok(None)
        }
    }

    /// Resolves unwind information for the given function entry.
    pub fn get_unwind_info(
        &self,
        function: RuntimeFunction,
        sections: &[section_table::SectionTable],
    ) -> error::Result<UnwindInfo<'a>> {
        self.get_unwind_info_with_opts(function, sections, &options::ParseOptions::default())
    }

    /// Resolves unwind information for the given function entry.
    pub fn get_unwind_info_with_opts(
        &self,
        mut function: RuntimeFunction,
        sections: &[section_table::SectionTable],
        opts: &options::ParseOptions,
    ) -> error::Result<UnwindInfo<'a>> {
        while function.unwind_info_address % 2 != 0 {
            let rva = (function.unwind_info_address & !1) as usize;
            function = self.get_function_by_rva_with_opts(rva, sections, opts)?;
        }

        let rva = function.unwind_info_address as usize;
        let offset =
            utils::find_offset(rva, sections, self.file_alignment, opts).ok_or_else(|| {
                error::Error::Malformed(format!("cannot map unwind rva ({:#x}) into offset", rva))
            })?;

        UnwindInfo::parse(self.bytes, offset)
    }

    #[allow(dead_code)]
    fn get_function_by_rva(
        &self,
        rva: usize,
        sections: &[section_table::SectionTable],
    ) -> error::Result<RuntimeFunction> {
        self.get_function_by_rva_with_opts(rva, sections, &options::ParseOptions::default())
    }

    fn get_function_by_rva_with_opts(
        &self,
        rva: usize,
        sections: &[section_table::SectionTable],
        opts: &options::ParseOptions,
    ) -> error::Result<RuntimeFunction> {
        let offset =
            utils::find_offset(rva, sections, self.file_alignment, opts).ok_or_else(|| {
                error::Error::Malformed(format!(
                    "cannot map exception rva ({:#x}) into offset",
                    rva
                ))
            })?;

        self.get_function_by_offset(offset)
    }

    #[inline]
    fn get_function_by_offset(&self, offset: usize) -> error::Result<RuntimeFunction> {
        debug_assert!((offset - self.offset) % RUNTIME_FUNCTION_SIZE == 0);
        debug_assert!(offset < self.offset + self.size);

        Ok(self.bytes.pread_with(offset, scroll::LE)?)
    }
}

impl fmt::Debug for ExceptionData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ExceptionData")
            .field("file_alignment", &self.file_alignment)
            .field("offset", &format_args!("{:#x}", self.offset))
            .field("size", &format_args!("{:#x}", self.size))
            .field("len", &self.len())
            .finish()
    }
}

impl<'a> IntoIterator for &'_ ExceptionData<'a> {
    type Item = error::Result<RuntimeFunction>;
    type IntoIter = RuntimeFunctionIterator<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.functions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of_runtime_function() {
        assert_eq!(
            std::mem::size_of::<RuntimeFunction>(),
            RUNTIME_FUNCTION_SIZE
        );
    }

    // Tests disabled until there is a solution for handling binary test data
    // See https://github.com/m4b/goblin/issues/185

    // macro_rules! microsoft_symbol {
    //     ($name:literal, $id:literal) => {{
    //         use std::fs::File;
    //         use std::path::Path;

    //         let path = Path::new(concat!("cache/", $name));
    //         if !path.exists() {
    //             let url = format!(
    //                 "https://msdl.microsoft.com/download/symbols/{}/{}/{}",
    //                 $name, $id, $name
    //             );

    //             let mut response = reqwest::get(&url).expect(concat!("get ", $name));
    //             let mut target = File::create(path).expect(concat!("create ", $name));
    //             response
    //                 .copy_to(&mut target)
    //                 .expect(concat!("download ", $name));
    //         }

    //         std::fs::read(path).expect(concat!("open ", $name))
    //     }};
    // }

    // lazy_static::lazy_static! {
    //     static ref PE_DATA: Vec<u8> = microsoft_symbol!("WSHTCPIP.DLL", "4a5be0b77000");
    // }

    // #[test]
    // fn test_parse() {
    //     let pe = PE::parse(&PE_DATA).expect("parse PE");
    //     let exception_data = pe.exception_data.expect("get exception data");

    //     assert_eq!(exception_data.len(), 19);
    //     assert!(!exception_data.is_empty());
    // }

    // #[test]
    // fn test_iter_functions() {
    //     let pe = PE::parse(&PE_DATA).expect("parse PE");
    //     let exception_data = pe.exception_data.expect("get exception data");

    //     let functions: Vec<RuntimeFunction> = exception_data
    //         .functions()
    //         .map(|result| result.expect("parse runtime function"))
    //         .collect();

    //     assert_eq!(functions.len(), 19);

    //     let expected = RuntimeFunction {
    //         begin_address: 0x1355,
    //         end_address: 0x1420,
    //         unwind_info_address: 0x4019,
    //     };

    //     assert_eq!(functions[4], expected);
    // }

    // #[test]
    // fn test_get_function() {
    //     let pe = PE::parse(&PE_DATA).expect("parse PE");
    //     let exception_data = pe.exception_data.expect("get exception data");

    //     let expected = RuntimeFunction {
    //         begin_address: 0x1355,
    //         end_address: 0x1420,
    //         unwind_info_address: 0x4019,
    //     };

    //     assert_eq!(
    //         exception_data.get_function(4).expect("find function"),
    //         expected
    //     );
    // }

    // #[test]
    // fn test_find_function() {
    //     let pe = PE::parse(&PE_DATA).expect("parse PE");
    //     let exception_data = pe.exception_data.expect("get exception data");

    //     let expected = RuntimeFunction {
    //         begin_address: 0x1355,
    //         end_address: 0x1420,
    //         unwind_info_address: 0x4019,
    //     };

    //     assert_eq!(
    //         exception_data.find_function(0x1400).expect("find function"),
    //         Some(expected)
    //     );
    // }

    // #[test]
    // fn test_find_function_none() {
    //     let pe = PE::parse(&PE_DATA).expect("parse PE");
    //     let exception_data = pe.exception_data.expect("get exception data");

    //     // 0x1d00 is the end address of the last function.

    //     assert_eq!(
    //         exception_data.find_function(0x1d00).expect("find function"),
    //         None
    //     );
    // }

    // #[test]
    // fn test_get_unwind_info() {
    //     let pe = PE::parse(&PE_DATA).expect("parse PE");
    //     let exception_data = pe.exception_data.expect("get exception data");

    //     // runtime function #0 directly refers to unwind info
    //     let rt_function = RuntimeFunction {
    //         begin_address: 0x1010,
    //         end_address: 0x1090,
    //         unwind_info_address: 0x25d8,
    //     };

    //     let unwind_info = exception_data
    //         .get_unwind_info(rt_function, &pe.sections)
    //         .expect("get unwind info");

    //     // Unwind codes just used to assert that the right unwind info was resolved
    //     let expected = &[4, 98];

    //     assert_eq!(unwind_info.code_bytes, expected);
    // }

    // #[test]
    // fn test_get_unwind_info_redirect() {
    //     let pe = PE::parse(&PE_DATA).expect("parse PE");
    //     let exception_data = pe.exception_data.expect("get exception data");

    //     // runtime function #4 has a redirect (unwind_info_address & 1).
    //     let rt_function = RuntimeFunction {
    //         begin_address: 0x1355,
    //         end_address: 0x1420,
    //         unwind_info_address: 0x4019,
    //     };

    //     let unwind_info = exception_data
    //         .get_unwind_info(rt_function, &pe.sections)
    //         .expect("get unwind info");

    //     // Unwind codes just used to assert that the right unwind info was resolved
    //     let expected = &[
    //         28, 100, 15, 0, 28, 84, 14, 0, 28, 52, 12, 0, 28, 82, 24, 240, 22, 224, 20, 208, 18,
    //         192, 16, 112,
    //     ];

    //     assert_eq!(unwind_info.code_bytes, expected);
    // }

    #[test]
    fn test_iter_unwind_codes() {
        let unwind_info = UnwindInfo {
            version: 1,
            size_of_prolog: 4,
            frame_register: Register(0),
            frame_register_offset: 0,
            chained_info: None,
            handler: None,
            code_bytes: &[4, 98],
        };

        let unwind_codes: Vec<UnwindCode> = unwind_info
            .unwind_codes()
            .map(|result| result.expect("parse unwind code"))
            .collect();

        assert_eq!(unwind_codes.len(), 1);

        let expected = UnwindCode {
            code_offset: 4,
            operation: UnwindOperation::Alloc(56),
        };

        assert_eq!(unwind_codes[0], expected);
    }
}
