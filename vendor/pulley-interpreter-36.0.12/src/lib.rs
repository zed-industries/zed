//! The pulley bytecode for fast interpreters.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(pulley_tail_calls, feature(explicit_tail_calls))]
#![cfg_attr(pulley_tail_calls, allow(incomplete_features, unstable_features))]
#![deny(missing_docs)]
#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(feature = "decode")]
extern crate alloc;

/// Calls the given macro with each opcode.
///
/// # Instruction Guidelines
///
/// We're inventing an instruction set here which naturally brings a whole set
/// of design questions. Note that this is explicitly intended to be only ever
/// used for Pulley where there are a different set of design constraints than
/// other instruction sets (e.g. general-purpose CPU ISAs). Some examples of
/// constraints for Pulley are:
///
/// * Instructions must be portable to many architectures.
/// * The Pulley ISA is mostly target-independent as the compilation target is
///   currently only parameterized on pointer width and endianness.
/// * Pulley instructions should be balance of time-to-decode and code size. For
///   example super fancy bit-packing tricks might be tough to decode in
///   software but might be worthwhile if it's quite common and greatly reduces
///   the size of bytecode. There's not a hard-and-fast answer here, but a
///   balance to be made.
/// * Many "macro ops" are present to reduce the size of compiled bytecode so
///   there is a wide set of duplicate functionality between opcodes (and this
///   is expected).
///
/// Given all this it's also useful to have a set of guidelines used to name and
/// develop Pulley instructions. As of the time of this writing it's still
/// pretty early days for Pulley so some of these guidelines may change over
/// time. Additionally instructions don't necessarily all follow these
/// conventions and that may also change over time. With that in mind, here's a
/// rough set of guidelines:
///
/// * Most instructions are prefixed with `x`, `f`, or `v`, indicating which
///   type of register they're operating on. (e.g. `xadd32` operates on the `x`
///   integer registers and `fadd32` operates on the `f` float registers).
///
/// * Most instructions are suffixed or otherwise contain the bit width they're
///   operating on. For example `xadd32` is a 32-bit addition.
///
/// * If an instruction operates on signed or unsigned data (such as division
///   and remainder), then the instruction is suffixed with `_s` or `_u`.
///
/// * Instructions operate on either 32 or 64-bit parts of a register.
///   Instructions modifying only 32-bits of a register always modify the "low"
///   part of a register and leave the upper part unmodified. This is intended
///   to help 32-bit platforms where if most operations are 32-bit there's no
///   need for extra instructions to sign or zero extend and modify the upper
///   half of the register.
///
/// * Binops use `BinaryOperands<T>` for the destination and argument registers.
///
/// * Instructions operating on memory contain a few pieces of information:
///
///   ```text
///   xload16le_u32_o32
///   │└─┬┘└┤└┤ └┬┘ └┬┘
///   │  │  │ │  │   ▼
///   │  │  │ │  │   addressing mode
///   │  │  │ │  ▼
///   │  │  │ │  width of register modified + sign-extension (optional)
///   │  │  │ ▼
///   │  │  │ endianness of the operation (le/be)
///   │  │  ▼
///   │  │  bit-width of the operation
///   │  ▼
///   │  what's happening (load/store)
///   ▼
///   register being operated on (x/f/z)
///   ```
///
/// More guidelines might get added here over time, and if you have any
/// questions feel free to raise them and we can try to add them here as well!
#[macro_export]
macro_rules! for_each_op {
    ( $macro:ident ) => {
        $macro! {
            /// Transfer control the address in the `lr` register.
            ret = Ret;

            /// Transfer control to the PC at the given offset and set the `lr`
            /// register to the PC just after this instruction.
            ///
            /// This instruction generally assumes that the Pulley ABI is being
            /// respected where arguments are in argument registers (starting at
            /// x0 for integer arguments) and results are in result registers.
            /// This instruction itself assume that all arguments are already in
            /// their registers. Subsequent instructions below enable moving
            /// arguments into the correct registers as part of the same call
            /// instruction.
            call = Call { offset: PcRelOffset };
            /// Like `call`, but also `x0 = arg1`
            call1 = Call1 { arg1: XReg, offset: PcRelOffset };
            /// Like `call`, but also `x0, x1 = arg1, arg2`
            call2 = Call2 { arg1: XReg, arg2: XReg, offset: PcRelOffset };
            /// Like `call`, but also `x0, x1, x2 = arg1, arg2, arg3`
            call3 = Call3 { arg1: XReg, arg2: XReg, arg3: XReg, offset: PcRelOffset };
            /// Like `call`, but also `x0, x1, x2, x3 = arg1, arg2, arg3, arg4`
            call4 = Call4 { arg1: XReg, arg2: XReg, arg3: XReg, arg4: XReg, offset: PcRelOffset };

            /// Transfer control to the PC in `reg` and set `lr` to the PC just
            /// after this instruction.
            call_indirect = CallIndirect { reg: XReg };

            /// Unconditionally transfer control to the PC at the given offset.
            jump = Jump { offset: PcRelOffset };

            /// Unconditionally transfer control to the PC at specified
            /// register.
            xjump = XJump { reg: XReg };

            /// Conditionally transfer control to the given PC offset if
            /// `low32(cond)` contains a non-zero value.
            br_if32 = BrIf { cond: XReg, offset: PcRelOffset };

            /// Conditionally transfer control to the given PC offset if
            /// `low32(cond)` contains a zero value.
            br_if_not32 = BrIfNot { cond: XReg, offset: PcRelOffset };

            /// Branch if `a == b`.
            br_if_xeq32 = BrIfXeq32 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if `a != `b.
            br_if_xneq32 = BrIfXneq32 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if signed `a < b`.
            br_if_xslt32 = BrIfXslt32 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if signed `a <= b`.
            br_if_xslteq32 = BrIfXslteq32 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if unsigned `a < b`.
            br_if_xult32 = BrIfXult32 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if unsigned `a <= b`.
            br_if_xulteq32 = BrIfXulteq32 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if `a == b`.
            br_if_xeq64 = BrIfXeq64 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if `a != `b.
            br_if_xneq64 = BrIfXneq64 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if signed `a < b`.
            br_if_xslt64 = BrIfXslt64 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if signed `a <= b`.
            br_if_xslteq64 = BrIfXslteq64 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if unsigned `a < b`.
            br_if_xult64 = BrIfXult64 { a: XReg, b: XReg, offset: PcRelOffset };
            /// Branch if unsigned `a <= b`.
            br_if_xulteq64 = BrIfXulteq64 { a: XReg, b: XReg, offset: PcRelOffset };

            /// Branch if `a == b`.
            br_if_xeq32_i8 = BrIfXeq32I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if `a == b`.
            br_if_xeq32_i32 = BrIfXeq32I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if `a != `b.
            br_if_xneq32_i8 = BrIfXneq32I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if `a != `b.
            br_if_xneq32_i32 = BrIfXneq32I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a < b`.
            br_if_xslt32_i8 = BrIfXslt32I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a < b`.
            br_if_xslt32_i32 = BrIfXslt32I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a > b`.
            br_if_xsgt32_i8 = BrIfXsgt32I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a > b`.
            br_if_xsgt32_i32 = BrIfXsgt32I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a <= b`.
            br_if_xslteq32_i8 = BrIfXslteq32I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a <= b`.
            br_if_xslteq32_i32 = BrIfXslteq32I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a >= b`.
            br_if_xsgteq32_i8 = BrIfXsgteq32I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a >= b`.
            br_if_xsgteq32_i32 = BrIfXsgteq32I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if unsigned `a < b`.
            br_if_xult32_u8 = BrIfXult32U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a < b`.
            br_if_xult32_u32 = BrIfXult32U32 { a: XReg, b: u32, offset: PcRelOffset };
            /// Branch if unsigned `a <= b`.
            br_if_xulteq32_u8 = BrIfXulteq32U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a <= b`.
            br_if_xulteq32_u32 = BrIfXulteq32U32 { a: XReg, b: u32, offset: PcRelOffset };
            /// Branch if unsigned `a > b`.
            br_if_xugt32_u8 = BrIfXugt32U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a > b`.
            br_if_xugt32_u32 = BrIfXugt32U32 { a: XReg, b: u32, offset: PcRelOffset };
            /// Branch if unsigned `a >= b`.
            br_if_xugteq32_u8 = BrIfXugteq32U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a >= b`.
            br_if_xugteq32_u32 = BrIfXugteq32U32 { a: XReg, b: u32, offset: PcRelOffset };

            /// Branch if `a == b`.
            br_if_xeq64_i8 = BrIfXeq64I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if `a == b`.
            br_if_xeq64_i32 = BrIfXeq64I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if `a != `b.
            br_if_xneq64_i8 = BrIfXneq64I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if `a != `b.
            br_if_xneq64_i32 = BrIfXneq64I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a < b`.
            br_if_xslt64_i8 = BrIfXslt64I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a < b`.
            br_if_xslt64_i32 = BrIfXslt64I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a > b`.
            br_if_xsgt64_i8 = BrIfXsgt64I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a > b`.
            br_if_xsgt64_i32 = BrIfXsgt64I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a <= b`.
            br_if_xslteq64_i8 = BrIfXslteq64I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a <= b`.
            br_if_xslteq64_i32 = BrIfXslteq64I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if signed `a >= b`.
            br_if_xsgteq64_i8 = BrIfXsgteq64I8 { a: XReg, b: i8, offset: PcRelOffset };
            /// Branch if signed `a >= b`.
            br_if_xsgteq64_i32 = BrIfXsgteq64I32 { a: XReg, b: i32, offset: PcRelOffset };
            /// Branch if unsigned `a < b`.
            br_if_xult64_u8 = BrIfXult64U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a < b`.
            br_if_xult64_u32 = BrIfXult64U32 { a: XReg, b: u32, offset: PcRelOffset };
            /// Branch if unsigned `a <= b`.
            br_if_xulteq64_u8 = BrIfXulteq64U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a <= b`.
            br_if_xulteq64_u32 = BrIfXulteq64U32 { a: XReg, b: u32, offset: PcRelOffset };
            /// Branch if unsigned `a > b`.
            br_if_xugt64_u8 = BrIfXugt64U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a > b`.
            br_if_xugt64_u32 = BrIfXugt64U32 { a: XReg, b: u32, offset: PcRelOffset };
            /// Branch if unsigned `a >= b`.
            br_if_xugteq64_u8 = BrIfXugteq64U8 { a: XReg, b: u8, offset: PcRelOffset };
            /// Branch if unsigned `a >= b`.
            br_if_xugteq64_u32 = BrIfXugteq64U32 { a: XReg, b: u32, offset: PcRelOffset };

            /// Branch to the label indicated by `low32(idx)`.
            ///
            /// After this instruction are `amt` instances of `PcRelOffset`
            /// and the `idx` selects which one will be branched to. The value
            /// of `idx` is clamped to `amt - 1` (e.g. the last offset is the
            /// "default" one.
            br_table32 = BrTable32 { idx: XReg, amt: u32 };

            /// Move between `x` registers.
            xmov = Xmov { dst: XReg, src: XReg };

            /// Set `dst = 0`
            xzero = Xzero { dst: XReg };
            /// Set `dst = 1`
            xone = Xone { dst: XReg };
            /// Set `dst = sign_extend(imm8)`.
            xconst8 = Xconst8 { dst: XReg, imm: i8 };
            /// Set `dst = sign_extend(imm16)`.
            xconst16 = Xconst16 { dst: XReg, imm: i16 };
            /// Set `dst = sign_extend(imm32)`.
            xconst32 = Xconst32 { dst: XReg, imm: i32 };
            /// Set `dst = imm64`.
            xconst64 = Xconst64 { dst: XReg, imm: i64 };

            /// 32-bit wrapping addition: `low32(dst) = low32(src1) + low32(src2)`.
            ///
            /// The upper 32-bits of `dst` are unmodified.
            xadd32 = Xadd32 { operands: BinaryOperands<XReg> };
            /// Same as `xadd32` but `src2` is a zero-extended 8-bit immediate.
            xadd32_u8 = Xadd32U8 { dst: XReg, src1: XReg, src2: u8 };
            /// Same as `xadd32` but `src2` is a 32-bit immediate.
            xadd32_u32 = Xadd32U32 { dst: XReg, src1: XReg, src2: u32 };

            /// 64-bit wrapping addition: `dst = src1 + src2`.
            xadd64 = Xadd64 { operands: BinaryOperands<XReg> };
            /// Same as `xadd64` but `src2` is a zero-extended 8-bit immediate.
            xadd64_u8 = Xadd64U8 { dst: XReg, src1: XReg, src2: u8 };
            /// Same as `xadd64` but `src2` is a zero-extended 32-bit immediate.
            xadd64_u32 = Xadd64U32 { dst: XReg, src1: XReg, src2: u32 };

            /// `low32(dst) = low32(src1) * low32(src2) + low32(src3)`
            xmadd32 = Xmadd32 { dst: XReg, src1: XReg, src2: XReg, src3: XReg };
            /// `dst = src1 * src2 + src3`
            xmadd64 = Xmadd64 { dst: XReg, src1: XReg, src2: XReg, src3: XReg };

            /// 32-bit wrapping subtraction: `low32(dst) = low32(src1) - low32(src2)`.
            ///
            /// The upper 32-bits of `dst` are unmodified.
            xsub32 = Xsub32 { operands: BinaryOperands<XReg> };
            /// Same as `xsub32` but `src2` is a zero-extended 8-bit immediate.
            xsub32_u8 = Xsub32U8 { dst: XReg, src1: XReg, src2: u8 };
            /// Same as `xsub32` but `src2` is a 32-bit immediate.
            xsub32_u32 = Xsub32U32 { dst: XReg, src1: XReg, src2: u32 };

            /// 64-bit wrapping subtraction: `dst = src1 - src2`.
            xsub64 = Xsub64 { operands: BinaryOperands<XReg> };
            /// Same as `xsub64` but `src2` is a zero-extended 8-bit immediate.
            xsub64_u8 = Xsub64U8 { dst: XReg, src1: XReg, src2: u8 };
            /// Same as `xsub64` but `src2` is a zero-extended 32-bit immediate.
            xsub64_u32 = Xsub64U32 { dst: XReg, src1: XReg, src2: u32 };

            /// `low32(dst) = low32(src1) * low32(src2)`
            xmul32 = XMul32 { operands: BinaryOperands<XReg> };
            /// Same as `xmul64` but `src2` is a sign-extended 8-bit immediate.
            xmul32_s8 = Xmul32S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xmul32` but `src2` is a sign-extended 32-bit immediate.
            xmul32_s32 = Xmul32S32 { dst: XReg, src1: XReg, src2: i32 };

            /// `dst = src1 * src2`
            xmul64 = XMul64 { operands: BinaryOperands<XReg> };
            /// Same as `xmul64` but `src2` is a sign-extended 8-bit immediate.
            xmul64_s8 = Xmul64S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xmul64` but `src2` is a sign-extended 64-bit immediate.
            xmul64_s32 = Xmul64S32 { dst: XReg, src1: XReg, src2: i32 };

            /// `low32(dst) = trailing_zeros(low32(src))`
            xctz32 = Xctz32 { dst: XReg, src: XReg };
            /// `dst = trailing_zeros(src)`
            xctz64 = Xctz64 { dst: XReg, src: XReg };

            /// `low32(dst) = leading_zeros(low32(src))`
            xclz32 = Xclz32 { dst: XReg, src: XReg };
            /// `dst = leading_zeros(src)`
            xclz64 = Xclz64 { dst: XReg, src: XReg };

            /// `low32(dst) = count_ones(low32(src))`
            xpopcnt32 = Xpopcnt32 { dst: XReg, src: XReg };
            /// `dst = count_ones(src)`
            xpopcnt64 = Xpopcnt64 { dst: XReg, src: XReg };

            /// `low32(dst) = rotate_left(low32(src1), low32(src2))`
            xrotl32 = Xrotl32 { operands: BinaryOperands<XReg> };
            /// `dst = rotate_left(src1, src2)`
            xrotl64 = Xrotl64 { operands: BinaryOperands<XReg> };

            /// `low32(dst) = rotate_right(low32(src1), low32(src2))`
            xrotr32 = Xrotr32 { operands: BinaryOperands<XReg> };
            /// `dst = rotate_right(src1, src2)`
            xrotr64 = Xrotr64 { operands: BinaryOperands<XReg> };

            /// `low32(dst) = low32(src1) << low5(src2)`
            xshl32 = Xshl32 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) >> low5(src2)`
            xshr32_s = Xshr32S { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) >> low5(src2)`
            xshr32_u = Xshr32U { operands: BinaryOperands<XReg> };
            /// `dst = src1 << low5(src2)`
            xshl64 = Xshl64 { operands: BinaryOperands<XReg> };
            /// `dst = src1 >> low6(src2)`
            xshr64_s = Xshr64S { operands: BinaryOperands<XReg> };
            /// `dst = src1 >> low6(src2)`
            xshr64_u = Xshr64U { operands: BinaryOperands<XReg> };

            /// `low32(dst) = low32(src1) << low5(src2)`
            xshl32_u6 = Xshl32U6 { operands: BinaryOperands<XReg, XReg, U6> };
            /// `low32(dst) = low32(src1) >> low5(src2)`
            xshr32_s_u6 = Xshr32SU6 { operands: BinaryOperands<XReg, XReg, U6> };
            /// `low32(dst) = low32(src1) >> low5(src2)`
            xshr32_u_u6 = Xshr32UU6 { operands: BinaryOperands<XReg, XReg, U6> };
            /// `dst = src1 << low5(src2)`
            xshl64_u6 = Xshl64U6 { operands: BinaryOperands<XReg, XReg, U6> };
            /// `dst = src1 >> low6(src2)`
            xshr64_s_u6 = Xshr64SU6 { operands: BinaryOperands<XReg, XReg, U6> };
            /// `dst = src1 >> low6(src2)`
            xshr64_u_u6 = Xshr64UU6 { operands: BinaryOperands<XReg, XReg, U6> };

            /// `low32(dst) = -low32(src)`
            xneg32 = Xneg32 { dst: XReg, src: XReg };
            /// `dst = -src`
            xneg64 = Xneg64 { dst: XReg, src: XReg };

            /// `low32(dst) = src1 == src2`
            xeq64 = Xeq64 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = src1 != src2`
            xneq64 = Xneq64 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = src1 < src2` (signed)
            xslt64 = Xslt64 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = src1 <= src2` (signed)
            xslteq64 = Xslteq64 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = src1 < src2` (unsigned)
            xult64 = Xult64 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = src1 <= src2` (unsigned)
            xulteq64 = Xulteq64 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) == low32(src2)`
            xeq32 = Xeq32 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) != low32(src2)`
            xneq32 = Xneq32 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) < low32(src2)` (signed)
            xslt32 = Xslt32 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) <= low32(src2)` (signed)
            xslteq32 = Xslteq32 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) < low32(src2)` (unsigned)
            xult32 = Xult32 { operands: BinaryOperands<XReg> };
            /// `low32(dst) = low32(src1) <= low32(src2)` (unsigned)
            xulteq32 = Xulteq32 { operands: BinaryOperands<XReg> };

            // Loads/stores with various addressing modes. Note that each style
            // of addressing mode is split to its own suite of instructions to
            // simplify the implementation of each opcode and avoid internal
            // branching when using one addressing mode vs another.
            //
            // Note that big-endian, float, and vector loads are deferred to
            // the "extended" opcode set below.

            /// `low32(dst) = zext_8_32(*addr)`
            xload8_u32_o32 = XLoad8U32O32 { dst: XReg, addr: AddrO32 };
            /// `low32(dst) = sext_8_32(*addr)`
            xload8_s32_o32 = XLoad8S32O32 { dst: XReg, addr: AddrO32 };
            /// `low32(dst) = o32ext_16_32(*addr)`
            xload16le_u32_o32 = XLoad16LeU32O32 { dst: XReg, addr: AddrO32 };
            /// `low32(dst) = sext_16_32(*addr)`
            xload16le_s32_o32 = XLoad16LeS32O32 { dst: XReg, addr: AddrO32 };
            /// `low32(dst) = *addr`
            xload32le_o32 = XLoad32LeO32 { dst: XReg, addr: AddrO32 };
            /// `dst = *addr`
            xload64le_o32 = XLoad64LeO32 { dst: XReg, addr: AddrO32 };
            /// `*addr = low8(src)`
            xstore8_o32 = XStore8O32 { addr: AddrO32, src: XReg };
            /// `*addr = low16(src)`
            xstore16le_o32 = XStore16LeO32 { addr: AddrO32, src: XReg };
            /// `*addr = low32(src)`
            xstore32le_o32 = XStore32LeO32 { addr: AddrO32, src: XReg };
            /// `*addr = src`
            xstore64le_o32 = XStore64LeO32 { addr: AddrO32, src: XReg };

            /// `low32(dst) = zext_8_32(*addr)`
            xload8_u32_z = XLoad8U32Z { dst: XReg, addr: AddrZ };
            /// `low32(dst) = sext_8_32(*addr)`
            xload8_s32_z = XLoad8S32Z { dst: XReg, addr: AddrZ };
            /// `low32(dst) = zext_16_32(*addr)`
            xload16le_u32_z = XLoad16LeU32Z { dst: XReg, addr: AddrZ };
            /// `low32(dst) = sext_16_32(*addr)`
            xload16le_s32_z = XLoad16LeS32Z { dst: XReg, addr: AddrZ };
            /// `low32(dst) = *addr`
            xload32le_z = XLoad32LeZ { dst: XReg, addr: AddrZ };
            /// `dst = *addr`
            xload64le_z = XLoad64LeZ { dst: XReg, addr: AddrZ };
            /// `*addr = low8(src)`
            xstore8_z = XStore8Z { addr: AddrZ, src: XReg };
            /// `*addr = low16(src)`
            xstore16le_z = XStore16LeZ { addr: AddrZ, src: XReg };
            /// `*addr = low32(src)`
            xstore32le_z = XStore32LeZ { addr: AddrZ, src: XReg };
            /// `*addr = src`
            xstore64le_z = XStore64LeZ { addr: AddrZ, src: XReg };

            /// `low32(dst) = zext_8_32(*addr)`
            xload8_u32_g32 = XLoad8U32G32 { dst: XReg, addr: AddrG32 };
            /// `low32(dst) = sext_8_32(*addr)`
            xload8_s32_g32 = XLoad8S32G32 { dst: XReg, addr: AddrG32 };
            /// `low32(dst) = zext_16_32(*addr)`
            xload16le_u32_g32 = XLoad16LeU32G32 { dst: XReg, addr: AddrG32 };
            /// `low32(dst) = sext_16_32(*addr)`
            xload16le_s32_g32 = XLoad16LeS32G32 { dst: XReg, addr: AddrG32 };
            /// `low32(dst) = *addr`
            xload32le_g32 = XLoad32LeG32 { dst: XReg, addr: AddrG32 };
            /// `dst = *addr`
            xload64le_g32 = XLoad64LeG32 { dst: XReg, addr: AddrG32 };
            /// `*addr = low8(src)`
            xstore8_g32 = XStore8G32 { addr: AddrG32, src: XReg };
            /// `*addr = low16(src)`
            xstore16le_g32 = XStore16LeG32 { addr: AddrG32, src: XReg };
            /// `*addr = low32(src)`
            xstore32le_g32 = XStore32LeG32 { addr: AddrG32, src: XReg };
            /// `*addr = src`
            xstore64le_g32 = XStore64LeG32 { addr: AddrG32, src: XReg };

            /// `low32(dst) = zext_8_32(*addr)`
            xload8_u32_g32bne = XLoad8U32G32Bne { dst: XReg, addr: AddrG32Bne };
            /// `low32(dst) = sext_8_32(*addr)`
            xload8_s32_g32bne = XLoad8S32G32Bne { dst: XReg, addr: AddrG32Bne };
            /// `low32(dst) = zext_16_32(*addr)`
            xload16le_u32_g32bne = XLoad16LeU32G32Bne { dst: XReg, addr: AddrG32Bne };
            /// `low32(dst) = sext_16_32(*addr)`
            xload16le_s32_g32bne = XLoad16LeS32G32Bne { dst: XReg, addr: AddrG32Bne };
            /// `low32(dst) = *addr`
            xload32le_g32bne = XLoad32LeG32Bne { dst: XReg, addr: AddrG32Bne };
            /// `dst = *addr`
            xload64le_g32bne = XLoad64LeG32Bne { dst: XReg, addr: AddrG32Bne };
            /// `*addr = low8(src)`
            xstore8_g32bne = XStore8G32Bne { addr: AddrG32Bne, src: XReg };
            /// `*addr = low16(src)`
            xstore16le_g32bne = XStore16LeG32Bne { addr: AddrG32Bne, src: XReg };
            /// `*addr = low32(src)`
            xstore32le_g32bne = XStore32LeG32Bne { addr: AddrG32Bne, src: XReg };
            /// `*addr = src`
            xstore64le_g32bne = XStore64LeG32Bne { addr: AddrG32Bne, src: XReg };


            /// `push lr; push fp; fp = sp`
            push_frame = PushFrame ;
            /// `sp = fp; pop fp; pop lr`
            pop_frame = PopFrame ;

            /// Macro-instruction to enter a function, allocate some stack, and
            /// then save some registers.
            ///
            /// This is equivalent to `push_frame`, `stack_alloc32 amt`, then
            /// saving all of `regs` to the top of the stack just allocated.
            push_frame_save = PushFrameSave { amt: u16, regs: UpperRegSet<XReg> };
            /// Inverse of `push_frame_save`. Restores `regs` from the top of
            /// the stack, then runs `stack_free32 amt`, then runs `pop_frame`.
            pop_frame_restore = PopFrameRestore { amt: u16, regs: UpperRegSet<XReg> };

            /// `sp = sp.checked_sub(amt)`
            stack_alloc32 = StackAlloc32 { amt: u32 };

            /// `sp = sp + amt`
            stack_free32 = StackFree32 { amt: u32 };

            /// `dst = zext(low8(src))`
            zext8 = Zext8 { dst: XReg, src: XReg };
            /// `dst = zext(low16(src))`
            zext16 = Zext16 { dst: XReg, src: XReg };
            /// `dst = zext(low32(src))`
            zext32 = Zext32 { dst: XReg, src: XReg };
            /// `dst = sext(low8(src))`
            sext8 = Sext8 { dst: XReg, src: XReg };
            /// `dst = sext(low16(src))`
            sext16 = Sext16 { dst: XReg, src: XReg };
            /// `dst = sext(low32(src))`
            sext32 = Sext32 { dst: XReg, src: XReg };

            /// `low32(dst) = |low32(src)|`
            xabs32 = XAbs32 { dst: XReg, src: XReg };
            /// `dst = |src|`
            xabs64 = XAbs64 { dst: XReg, src: XReg };

            /// `low32(dst) = low32(src1) / low32(src2)` (signed)
            xdiv32_s = XDiv32S { operands: BinaryOperands<XReg> };

            /// `dst = src1 / src2` (signed)
            xdiv64_s = XDiv64S { operands: BinaryOperands<XReg> };

            /// `low32(dst) = low32(src1) / low32(src2)` (unsigned)
            xdiv32_u = XDiv32U { operands: BinaryOperands<XReg> };

            /// `dst = src1 / src2` (unsigned)
            xdiv64_u = XDiv64U { operands: BinaryOperands<XReg> };

            /// `low32(dst) = low32(src1) % low32(src2)` (signed)
            xrem32_s = XRem32S { operands: BinaryOperands<XReg> };

            /// `dst = src1 / src2` (signed)
            xrem64_s = XRem64S { operands: BinaryOperands<XReg> };

            /// `low32(dst) = low32(src1) % low32(src2)` (unsigned)
            xrem32_u = XRem32U { operands: BinaryOperands<XReg> };

            /// `dst = src1 / src2` (unsigned)
            xrem64_u = XRem64U { operands: BinaryOperands<XReg> };

            /// `low32(dst) = low32(src1) & low32(src2)`
            xband32 = XBand32 { operands: BinaryOperands<XReg> };
            /// Same as `xband64` but `src2` is a sign-extended 8-bit immediate.
            xband32_s8 = Xband32S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xband32` but `src2` is a sign-extended 32-bit immediate.
            xband32_s32 = Xband32S32 { dst: XReg, src1: XReg, src2: i32 };
            /// `dst = src1 & src2`
            xband64 = XBand64 { operands: BinaryOperands<XReg> };
            /// Same as `xband64` but `src2` is a sign-extended 8-bit immediate.
            xband64_s8 = Xband64S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xband64` but `src2` is a sign-extended 32-bit immediate.
            xband64_s32 = Xband64S32 { dst: XReg, src1: XReg, src2: i32 };
            /// `low32(dst) = low32(src1) | low32(src2)`
            xbor32 = XBor32 { operands: BinaryOperands<XReg> };
            /// Same as `xbor64` but `src2` is a sign-extended 8-bit immediate.
            xbor32_s8 = Xbor32S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xbor32` but `src2` is a sign-extended 32-bit immediate.
            xbor32_s32 = Xbor32S32 { dst: XReg, src1: XReg, src2: i32 };
            /// `dst = src1 | src2`
            xbor64 = XBor64 { operands: BinaryOperands<XReg> };
            /// Same as `xbor64` but `src2` is a sign-extended 8-bit immediate.
            xbor64_s8 = Xbor64S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xbor64` but `src2` is a sign-extended 32-bit immediate.
            xbor64_s32 = Xbor64S32 { dst: XReg, src1: XReg, src2: i32 };

            /// `low32(dst) = low32(src1) ^ low32(src2)`
            xbxor32 = XBxor32 { operands: BinaryOperands<XReg> };
            /// Same as `xbxor64` but `src2` is a sign-extended 8-bit immediate.
            xbxor32_s8 = Xbxor32S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xbxor32` but `src2` is a sign-extended 32-bit immediate.
            xbxor32_s32 = Xbxor32S32 { dst: XReg, src1: XReg, src2: i32 };
            /// `dst = src1 ^ src2`
            xbxor64 = XBxor64 { operands: BinaryOperands<XReg> };
            /// Same as `xbxor64` but `src2` is a sign-extended 8-bit immediate.
            xbxor64_s8 = Xbxor64S8 { dst: XReg, src1: XReg, src2: i8 };
            /// Same as `xbxor64` but `src2` is a sign-extended 32-bit immediate.
            xbxor64_s32 = Xbxor64S32 { dst: XReg, src1: XReg, src2: i32 };

            /// `low32(dst) = !low32(src1)`
            xbnot32 = XBnot32 { dst: XReg, src: XReg };
            /// `dst = !src1`
            xbnot64 = XBnot64 { dst: XReg, src: XReg };

            /// `low32(dst) = min(low32(src1), low32(src2))` (unsigned)
            xmin32_u = Xmin32U { operands: BinaryOperands<XReg> };
            /// `low32(dst) = min(low32(src1), low32(src2))` (signed)
            xmin32_s = Xmin32S { operands: BinaryOperands<XReg> };
            /// `low32(dst) = max(low32(src1), low32(src2))` (unsigned)
            xmax32_u = Xmax32U { operands: BinaryOperands<XReg> };
            /// `low32(dst) = max(low32(src1), low32(src2))` (signed)
            xmax32_s = Xmax32S { operands: BinaryOperands<XReg> };
            /// `dst = min(src1, src2)` (unsigned)
            xmin64_u = Xmin64U { operands: BinaryOperands<XReg> };
            /// `dst = min(src1, src2)` (signed)
            xmin64_s = Xmin64S { operands: BinaryOperands<XReg> };
            /// `dst = max(src1, src2)` (unsigned)
            xmax64_u = Xmax64U { operands: BinaryOperands<XReg> };
            /// `dst = max(src1, src2)` (signed)
            xmax64_s = Xmax64S { operands: BinaryOperands<XReg> };

            /// `low32(dst) = low32(cond) ? low32(if_nonzero) : low32(if_zero)`
            xselect32 = XSelect32 { dst: XReg, cond: XReg, if_nonzero: XReg, if_zero: XReg };
            /// `dst = low32(cond) ? if_nonzero : if_zero`
            xselect64 = XSelect64 { dst: XReg, cond: XReg, if_nonzero: XReg, if_zero: XReg };
        }
    };
}

/// Calls the given macro with each extended opcode.
#[macro_export]
macro_rules! for_each_extended_op {
    ( $macro:ident ) => {
        $macro! {
            /// Raise a trap.
            trap = Trap;

            /// Do nothing.
            nop = Nop;

            /// A special opcode to halt interpreter execution and yield control
            /// back to the host.
            ///
            /// This opcode results in `DoneReason::CallIndirectHost` where the
            /// `id` here is shepherded along to the embedder. It's up to the
            /// embedder to determine what to do with the `id` and the current
            /// state of registers and the stack.
            ///
            /// In Wasmtime this is used to implement interpreter-to-host calls.
            /// This is modeled as a `call` instruction where the first
            /// parameter is the native function pointer to invoke and all
            /// remaining parameters for the native function are in following
            /// parameter positions (e.g. `x1`, `x2`, ...). The results of the
            /// host call are then store in `x0`.
            ///
            /// Handling this in Wasmtime is done through a "relocation" which
            /// is resolved at link-time when raw bytecode from Cranelift is
            /// assembled into the final object that Wasmtime will interpret.
            call_indirect_host = CallIndirectHost { id: u8 };

            /// Gets the special "fp" register and moves it into `dst`.
            xmov_fp = XmovFp { dst: XReg };

            /// Gets the special "lr" register and moves it into `dst`.
            xmov_lr = XmovLr { dst: XReg };

            /// `dst = byteswap(low32(src))`
            bswap32 = Bswap32 { dst: XReg, src: XReg };
            /// `dst = byteswap(src)`
            bswap64 = Bswap64 { dst: XReg, src: XReg };

            /// 32-bit checked unsigned addition: `low32(dst) = low32(src1) +
            /// low32(src2)`.
            ///
            /// The upper 32-bits of `dst` are unmodified. Traps if the addition
            /// overflows.
            xadd32_uoverflow_trap = Xadd32UoverflowTrap { operands: BinaryOperands<XReg> };

            /// 64-bit checked unsigned addition: `dst = src1 + src2`.
            xadd64_uoverflow_trap = Xadd64UoverflowTrap { operands: BinaryOperands<XReg> };

            /// `dst = high64(src1 * src2)` (signed)
            xmulhi64_s = XMulHi64S { operands: BinaryOperands<XReg> };
            /// `dst = high64(src1 * src2)` (unsigned)
            xmulhi64_u = XMulHi64U { operands: BinaryOperands<XReg> };

            /// low32(dst) = if low32(src) == 0 { 0 } else { -1 }
            xbmask32 = Xbmask32 { dst: XReg, src: XReg };
            /// dst = if src == 0 { 0 } else { -1 }
            xbmask64 = Xbmask64 { dst: XReg, src: XReg };

            // Big-endian loads/stores of X-registers using the "o32"
            // addressing mode

            /// `low32(dst) = zext(*addr)`
            xload16be_u32_o32 = XLoad16BeU32O32 { dst: XReg, addr: AddrO32 };
            /// `low32(dst) = sext(*addr)`
            xload16be_s32_o32 = XLoad16BeS32O32 { dst: XReg, addr: AddrO32 };
            /// `low32(dst) = zext(*addr)`
            xload32be_o32 = XLoad32BeO32 { dst: XReg, addr: AddrO32 };
            /// `dst = *addr`
            xload64be_o32 = XLoad64BeO32 { dst: XReg, addr: AddrO32 };
            /// `*addr = low16(src)`
            xstore16be_o32 = XStore16BeO32 { addr: AddrO32, src: XReg };
            /// `*addr = low32(src)`
            xstore32be_o32 = XStore32BeO32 { addr: AddrO32, src: XReg };
            /// `*addr = low64(src)`
            xstore64be_o32 = XStore64BeO32 { addr: AddrO32, src: XReg };

            // Big and little endian float loads/stores. Note that the "Z"
            // addressing mode only has little-endian variants.

            /// `low32(dst) = zext(*addr)`
            fload32be_o32 = Fload32BeO32 { dst: FReg, addr: AddrO32 };
            /// `dst = *addr`
            fload64be_o32 = Fload64BeO32 { dst: FReg, addr: AddrO32 };
            /// `*addr = low32(src)`
            fstore32be_o32 = Fstore32BeO32 { addr: AddrO32, src: FReg };
            /// `*addr = src`
            fstore64be_o32 = Fstore64BeO32 { addr: AddrO32, src: FReg };

            /// `low32(dst) = zext(*addr)`
            fload32le_o32 = Fload32LeO32 { dst: FReg, addr: AddrO32 };
            /// `dst = *addr`
            fload64le_o32 = Fload64LeO32 { dst: FReg, addr: AddrO32 };
            /// `*addr = low32(src)`
            fstore32le_o32 = Fstore32LeO32 { addr: AddrO32, src: FReg };
            /// `*addr = src`
            fstore64le_o32 = Fstore64LeO32 { addr: AddrO32, src: FReg };

            /// `low32(dst) = zext(*addr)`
            fload32le_z = Fload32LeZ { dst: FReg, addr: AddrZ };
            /// `dst = *addr`
            fload64le_z = Fload64LeZ { dst: FReg, addr: AddrZ };
            /// `*addr = low32(src)`
            fstore32le_z = Fstore32LeZ { addr: AddrZ, src: FReg };
            /// `*addr = src`
            fstore64le_z = Fstore64LeZ { addr: AddrZ, src: FReg };

            /// `low32(dst) = zext(*addr)`
            fload32le_g32 = Fload32LeG32 { dst: FReg, addr: AddrG32 };
            /// `dst = *addr`
            fload64le_g32 = Fload64LeG32 { dst: FReg, addr: AddrG32 };
            /// `*addr = low32(src)`
            fstore32le_g32 = Fstore32LeG32 { addr: AddrG32, src: FReg };
            /// `*addr = src`
            fstore64le_g32 = Fstore64LeG32 { addr: AddrG32, src: FReg };

            // Vector loads/stores. Note that big-endian variants are all
            // omitted.

            /// `dst = *addr`
            vload128le_o32 = VLoad128O32 { dst: VReg, addr: AddrO32 };
            /// `*addr = src`
            vstore128le_o32 = Vstore128LeO32 { addr: AddrO32, src: VReg };
            /// `dst = *(ptr + offset)`
            vload128le_z = VLoad128Z { dst: VReg, addr: AddrZ };
            /// `*(ptr + offset) = src`
            vstore128le_z = Vstore128LeZ { addr: AddrZ, src: VReg };
            /// `dst = *(ptr + offset)`
            vload128le_g32 = VLoad128G32 { dst: VReg, addr: AddrG32 };
            /// `*(ptr + offset) = src`
            vstore128le_g32 = Vstore128LeG32 { addr: AddrG32, src: VReg };

            /// Move between `f` registers.
            fmov = Fmov { dst: FReg, src: FReg };
            /// Move between `v` registers.
            vmov = Vmov { dst: VReg, src: VReg };

            /// `low32(dst) = bitcast low32(src) as i32`
            bitcast_int_from_float_32 = BitcastIntFromFloat32 { dst: XReg, src: FReg };
            /// `dst = bitcast src as i64`
            bitcast_int_from_float_64 = BitcastIntFromFloat64 { dst: XReg, src: FReg };
            /// `low32(dst) = bitcast low32(src) as f32`
            bitcast_float_from_int_32 = BitcastFloatFromInt32 { dst: FReg, src: XReg };
            /// `dst = bitcast src as f64`
            bitcast_float_from_int_64 = BitcastFloatFromInt64 { dst: FReg, src: XReg };

            /// `low32(dst) = bits`
            fconst32 = FConst32 { dst: FReg, bits: u32 };
            /// `dst = bits`
            fconst64 = FConst64 { dst: FReg, bits: u64 };

            /// `low32(dst) = zext(src1 == src2)`
            feq32 = Feq32 { dst: XReg, src1: FReg, src2: FReg };
            /// `low32(dst) = zext(src1 != src2)`
            fneq32 = Fneq32 { dst: XReg, src1: FReg, src2: FReg };
            /// `low32(dst) = zext(src1 < src2)`
            flt32 = Flt32 { dst: XReg, src1: FReg, src2: FReg };
            /// `low32(dst) = zext(src1 <= src2)`
            flteq32 = Flteq32 { dst: XReg, src1: FReg, src2: FReg };
            /// `low32(dst) = zext(src1 == src2)`
            feq64 = Feq64 { dst: XReg, src1: FReg, src2: FReg };
            /// `low32(dst) = zext(src1 != src2)`
            fneq64 = Fneq64 { dst: XReg, src1: FReg, src2: FReg };
            /// `low32(dst) = zext(src1 < src2)`
            flt64 = Flt64 { dst: XReg, src1: FReg, src2: FReg };
            /// `low32(dst) = zext(src1 <= src2)`
            flteq64 = Flteq64 { dst: XReg, src1: FReg, src2: FReg };

            /// `low32(dst) = low32(cond) ? low32(if_nonzero) : low32(if_zero)`
            fselect32 = FSelect32 { dst: FReg, cond: XReg, if_nonzero: FReg, if_zero: FReg };
            /// `dst = low32(cond) ? if_nonzero : if_zero`
            fselect64 = FSelect64 { dst: FReg, cond: XReg, if_nonzero: FReg, if_zero: FReg };

            /// `low32(dst) = demote(src)`
            f32_from_f64 = F32FromF64 { dst: FReg, src: FReg };
            /// `(st) = promote(low32(src))`
            f64_from_f32 = F64FromF32 { dst: FReg, src: FReg };

            /// `low32(dst) = checked_f32_from_signed(low32(src))`
            f32_from_x32_s = F32FromX32S { dst: FReg, src: XReg };
            /// `low32(dst) = checked_f32_from_unsigned(low32(src))`
            f32_from_x32_u = F32FromX32U { dst: FReg, src: XReg };
            /// `low32(dst) = checked_f32_from_signed(src)`
            f32_from_x64_s = F32FromX64S { dst: FReg, src: XReg };
            /// `low32(dst) = checked_f32_from_unsigned(src)`
            f32_from_x64_u = F32FromX64U { dst: FReg, src: XReg };
            /// `dst = checked_f64_from_signed(low32(src))`
            f64_from_x32_s = F64FromX32S { dst: FReg, src: XReg };
            /// `dst = checked_f64_from_unsigned(low32(src))`
            f64_from_x32_u = F64FromX32U { dst: FReg, src: XReg };
            /// `dst = checked_f64_from_signed(src)`
            f64_from_x64_s = F64FromX64S { dst: FReg, src: XReg };
            /// `dst = checked_f64_from_unsigned(src)`
            f64_from_x64_u = F64FromX64U { dst: FReg, src: XReg };

            /// `low32(dst) = checked_signed_from_f32(low32(src))`
            x32_from_f32_s = X32FromF32S { dst: XReg, src: FReg };
            /// `low32(dst) = checked_unsigned_from_f32(low32(src))`
            x32_from_f32_u = X32FromF32U { dst: XReg, src: FReg };
            /// `low32(dst) = checked_signed_from_f64(src)`
            x32_from_f64_s = X32FromF64S { dst: XReg, src: FReg };
            /// `low32(dst) = checked_unsigned_from_f64(src)`
            x32_from_f64_u = X32FromF64U { dst: XReg, src: FReg };
            /// `dst = checked_signed_from_f32(low32(src))`
            x64_from_f32_s = X64FromF32S { dst: XReg, src: FReg };
            /// `dst = checked_unsigned_from_f32(low32(src))`
            x64_from_f32_u = X64FromF32U { dst: XReg, src: FReg };
            /// `dst = checked_signed_from_f64(src)`
            x64_from_f64_s = X64FromF64S { dst: XReg, src: FReg };
            /// `dst = checked_unsigned_from_f64(src)`
            x64_from_f64_u = X64FromF64U { dst: XReg, src: FReg };

            /// `low32(dst) = saturating_signed_from_f32(low32(src))`
            x32_from_f32_s_sat = X32FromF32SSat { dst: XReg, src: FReg };
            /// `low32(dst) = saturating_unsigned_from_f32(low32(src))`
            x32_from_f32_u_sat = X32FromF32USat { dst: XReg, src: FReg };
            /// `low32(dst) = saturating_signed_from_f64(src)`
            x32_from_f64_s_sat = X32FromF64SSat { dst: XReg, src: FReg };
            /// `low32(dst) = saturating_unsigned_from_f64(src)`
            x32_from_f64_u_sat = X32FromF64USat { dst: XReg, src: FReg };
            /// `dst = saturating_signed_from_f32(low32(src))`
            x64_from_f32_s_sat = X64FromF32SSat { dst: XReg, src: FReg };
            /// `dst = saturating_unsigned_from_f32(low32(src))`
            x64_from_f32_u_sat = X64FromF32USat { dst: XReg, src: FReg };
            /// `dst = saturating_signed_from_f64(src)`
            x64_from_f64_s_sat = X64FromF64SSat { dst: XReg, src: FReg };
            /// `dst = saturating_unsigned_from_f64(src)`
            x64_from_f64_u_sat = X64FromF64USat { dst: XReg, src: FReg };

            /// `low32(dst) = copysign(low32(src1), low32(src2))`
            fcopysign32 = FCopySign32 { operands: BinaryOperands<FReg> };
            /// `dst = copysign(src1, src2)`
            fcopysign64 = FCopySign64 { operands: BinaryOperands<FReg> };

            /// `low32(dst) = low32(src1) + low32(src2)`
            fadd32 = Fadd32 { operands: BinaryOperands<FReg> };
            /// `low32(dst) = low32(src1) - low32(src2)`
            fsub32 = Fsub32 { operands: BinaryOperands<FReg> };
            /// `low128(dst) = low128(src1) - low128(src2)`
            vsubf32x4 = Vsubf32x4 { operands: BinaryOperands<VReg> };
            /// `low32(dst) = low32(src1) * low32(src2)`
            fmul32 = Fmul32 { operands: BinaryOperands<FReg> };
            /// `low128(dst) = low128(src1) * low128(src2)`
            vmulf32x4 = Vmulf32x4 { operands: BinaryOperands<VReg> };
            /// `low32(dst) = low32(src1) / low32(src2)`
            fdiv32 = Fdiv32 { operands: BinaryOperands<FReg> };
            /// `low128(dst) = low128(src1) / low128(src2)`
            vdivf32x4 = Vdivf32x4 { operands: BinaryOperands<VReg> };
            /// `low32(dst) = ieee_maximum(low32(src1), low32(src2))`
            fmaximum32 = Fmaximum32 { operands: BinaryOperands<FReg> };
            /// `low32(dst) = ieee_minimum(low32(src1), low32(src2))`
            fminimum32 = Fminimum32 { operands: BinaryOperands<FReg> };
            /// `low32(dst) = ieee_trunc(low32(src))`
            ftrunc32 = Ftrunc32 { dst: FReg, src: FReg };
            /// `low128(dst) = ieee_trunc(low128(src))`
            vtrunc32x4 = Vtrunc32x4 { dst: VReg, src: VReg };
            /// `low128(dst) = ieee_trunc(low128(src))`
            vtrunc64x2 = Vtrunc64x2 { dst: VReg, src: VReg };
            /// `low32(dst) = ieee_floor(low32(src))`
            ffloor32 = Ffloor32 { dst: FReg, src: FReg };
            /// `low128(dst) = ieee_floor(low128(src))`
            vfloor32x4 = Vfloor32x4 { dst: VReg, src: VReg };
            /// `low128(dst) = ieee_floor(low128(src))`
            vfloor64x2 = Vfloor64x2 { dst: VReg, src: VReg };
            /// `low32(dst) = ieee_ceil(low32(src))`
            fceil32 = Fceil32 { dst: FReg, src: FReg };
            /// `low128(dst) = ieee_ceil(low128(src))`
            vceil32x4 = Vceil32x4 { dst: VReg, src: VReg };
            /// `low128(dst) = ieee_ceil(low128(src))`
            vceil64x2 = Vceil64x2 { dst: VReg, src: VReg };
            /// `low32(dst) = ieee_nearest(low32(src))`
            fnearest32 = Fnearest32 { dst: FReg, src: FReg };
            /// `low32(dst) = ieee_sqrt(low32(src))`
            fsqrt32 = Fsqrt32 { dst: FReg, src: FReg };
            /// `low32(dst) = ieee_sqrt(low32(src))`
            vsqrt32x4 = Vsqrt32x4 { dst: VReg, src: VReg };
            /// `low32(dst) = ieee_sqrt(low32(src))`
            vsqrt64x2 = Vsqrt64x2 { dst: VReg, src: VReg };
            /// `low32(dst) = -low32(src)`
            fneg32 = Fneg32 { dst: FReg, src: FReg };
            /// `low128(dst) = -low128(src)`
            vnegf32x4 = Vnegf32x4 { dst: VReg, src: VReg };
            /// `low32(dst) = |low32(src)|`
            fabs32 = Fabs32 { dst: FReg, src: FReg };

            /// `dst = src1 + src2`
            fadd64 = Fadd64 { operands: BinaryOperands<FReg> };
            /// `dst = src1 - src2`
            fsub64 = Fsub64 { operands: BinaryOperands<FReg> };
            /// `dst = src1 * src2`
            fmul64 = Fmul64 { operands: BinaryOperands<FReg> };
            /// `dst = src1 / src2`
            fdiv64 = Fdiv64 { operands: BinaryOperands<FReg> };
            /// `dst = src1 / src2`
            vdivf64x2 = VDivF64x2 { operands: BinaryOperands<VReg> };
            /// `dst = ieee_maximum(src1, src2)`
            fmaximum64 = Fmaximum64 { operands: BinaryOperands<FReg> };
            /// `dst = ieee_minimum(src1, src2)`
            fminimum64 = Fminimum64 { operands: BinaryOperands<FReg> };
            /// `dst = ieee_trunc(src)`
            ftrunc64 = Ftrunc64 { dst: FReg, src: FReg };
            /// `dst = ieee_floor(src)`
            ffloor64 = Ffloor64 { dst: FReg, src: FReg };
            /// `dst = ieee_ceil(src)`
            fceil64 = Fceil64 { dst: FReg, src: FReg };
            /// `dst = ieee_nearest(src)`
            fnearest64 = Fnearest64 { dst: FReg, src: FReg };
            /// `low128(dst) = ieee_nearest(low128(src))`
            vnearest32x4 = Vnearest32x4 { dst: VReg, src: VReg };
            /// `low128(dst) = ieee_nearest(low128(src))`
            vnearest64x2 = Vnearest64x2 { dst: VReg, src: VReg };
            /// `dst = ieee_sqrt(src)`
            fsqrt64 = Fsqrt64 { dst: FReg, src: FReg };
            /// `dst = -src`
            fneg64 = Fneg64 { dst: FReg, src: FReg };
            /// `dst = |src|`
            fabs64 = Fabs64 { dst: FReg, src: FReg };

            /// `dst = imm`
            vconst128 = Vconst128 { dst: VReg, imm: u128 };

            /// `dst = src1 + src2`
            vaddi8x16 = VAddI8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src1 + src2`
            vaddi16x8 = VAddI16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src1 + src2`
            vaddi32x4 = VAddI32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src1 + src2`
            vaddi64x2 = VAddI64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src1 + src2`
            vaddf32x4 = VAddF32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src1 + src2`
            vaddf64x2 = VAddF64x2 { operands: BinaryOperands<VReg> };

            /// `dst = satruating_add(src1, src2)`
            vaddi8x16_sat = VAddI8x16Sat { operands: BinaryOperands<VReg> };
            /// `dst = satruating_add(src1, src2)`
            vaddu8x16_sat = VAddU8x16Sat { operands: BinaryOperands<VReg> };
            /// `dst = satruating_add(src1, src2)`
            vaddi16x8_sat = VAddI16x8Sat { operands: BinaryOperands<VReg> };
            /// `dst = satruating_add(src1, src2)`
            vaddu16x8_sat = VAddU16x8Sat { operands: BinaryOperands<VReg> };

            /// `dst = [src1[0] + src1[1], ..., src2[6] + src2[7]]`
            vaddpairwisei16x8_s = VAddpairwiseI16x8S { operands: BinaryOperands<VReg> };
            /// `dst = [src1[0] + src1[1], ..., src2[2] + src2[3]]`
            vaddpairwisei32x4_s = VAddpairwiseI32x4S { operands: BinaryOperands<VReg> };

            /// `dst = src1 << src2`
            vshli8x16 = VShlI8x16 { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 << src2`
            vshli16x8 = VShlI16x8 { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 << src2`
            vshli32x4 = VShlI32x4 { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 << src2`
            vshli64x2 = VShlI64x2 { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (signed)
            vshri8x16_s = VShrI8x16S { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (signed)
            vshri16x8_s = VShrI16x8S { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (signed)
            vshri32x4_s = VShrI32x4S { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (signed)
            vshri64x2_s = VShrI64x2S { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (unsigned)
            vshri8x16_u = VShrI8x16U { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (unsigned)
            vshri16x8_u = VShrI16x8U { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (unsigned)
            vshri32x4_u = VShrI32x4U { operands: BinaryOperands<VReg, VReg, XReg> };
            /// `dst = src1 >> src2` (unsigned)
            vshri64x2_u = VShrI64x2U { operands: BinaryOperands<VReg, VReg, XReg> };

            /// `dst = splat(low8(src))`
            vsplatx8 = VSplatX8 { dst: VReg, src: XReg };
            /// `dst = splat(low16(src))`
            vsplatx16 = VSplatX16 { dst: VReg, src: XReg };
            /// `dst = splat(low32(src))`
            vsplatx32 = VSplatX32 { dst: VReg, src: XReg };
            /// `dst = splat(src)`
            vsplatx64 = VSplatX64 { dst: VReg, src: XReg };
            /// `dst = splat(low32(src))`
            vsplatf32 = VSplatF32 { dst: VReg, src: FReg };
            /// `dst = splat(src)`
            vsplatf64 = VSplatF64 { dst: VReg, src: FReg };

            /// Load the 64-bit source as i8x8 and sign-extend to i16x8.
            vload8x8_s_z = VLoad8x8SZ { dst: VReg, addr: AddrZ };
            /// Load the 64-bit source as u8x8 and zero-extend to i16x8.
            vload8x8_u_z = VLoad8x8UZ { dst: VReg, addr: AddrZ };
            /// Load the 64-bit source as i16x4 and sign-extend to i32x4.
            vload16x4le_s_z = VLoad16x4LeSZ { dst: VReg, addr: AddrZ };
            /// Load the 64-bit source as u16x4 and zero-extend to i32x4.
            vload16x4le_u_z = VLoad16x4LeUZ { dst: VReg, addr: AddrZ };
            /// Load the 64-bit source as i32x2 and sign-extend to i64x2.
            vload32x2le_s_z = VLoad32x2LeSZ { dst: VReg, addr: AddrZ };
            /// Load the 64-bit source as u32x2 and zero-extend to i64x2.
            vload32x2le_u_z = VLoad32x2LeUZ { dst: VReg, addr: AddrZ };

            /// `dst = src1 & src2`
            vband128 = VBand128 { operands: BinaryOperands<VReg> };
            /// `dst = src1 | src2`
            vbor128 = VBor128 { operands: BinaryOperands<VReg> };
            /// `dst = src1 ^ src2`
            vbxor128 = VBxor128 { operands: BinaryOperands<VReg> };
            /// `dst = !src1`
            vbnot128 = VBnot128 { dst: VReg, src: VReg };
            /// `dst = (c & x) | (!c & y)`
            vbitselect128 = VBitselect128 { dst: VReg, c: VReg, x: VReg, y: VReg };
            /// Collect high bits of each lane into the low 32-bits of the
            /// destination.
            vbitmask8x16 = Vbitmask8x16 { dst: XReg, src: VReg };
            /// Collect high bits of each lane into the low 32-bits of the
            /// destination.
            vbitmask16x8 = Vbitmask16x8 { dst: XReg, src: VReg };
            /// Collect high bits of each lane into the low 32-bits of the
            /// destination.
            vbitmask32x4 = Vbitmask32x4 { dst: XReg, src: VReg };
            /// Collect high bits of each lane into the low 32-bits of the
            /// destination.
            vbitmask64x2 = Vbitmask64x2 { dst: XReg, src: VReg };
            /// Store whether all lanes are nonzero in `dst`.
            valltrue8x16 = Valltrue8x16 { dst: XReg, src: VReg };
            /// Store whether all lanes are nonzero in `dst`.
            valltrue16x8 = Valltrue16x8 { dst: XReg, src: VReg };
            /// Store whether all lanes are nonzero in `dst`.
            valltrue32x4 = Valltrue32x4 { dst: XReg, src: VReg };
            /// Store whether any lanes are nonzero in `dst`.
            valltrue64x2 = Valltrue64x2 { dst: XReg, src: VReg };
            /// Store whether any lanes are nonzero in `dst`.
            vanytrue8x16 = Vanytrue8x16 { dst: XReg, src: VReg };
            /// Store whether any lanes are nonzero in `dst`.
            vanytrue16x8 = Vanytrue16x8 { dst: XReg, src: VReg };
            /// Store whether any lanes are nonzero in `dst`.
            vanytrue32x4 = Vanytrue32x4 { dst: XReg, src: VReg };
            /// Store whether any lanes are nonzero in `dst`.
            vanytrue64x2 = Vanytrue64x2 { dst: XReg, src: VReg };

            /// Int-to-float conversion (same as `f32_from_x32_s`)
            vf32x4_from_i32x4_s = VF32x4FromI32x4S { dst: VReg, src: VReg };
            /// Int-to-float conversion (same as `f32_from_x32_u`)
            vf32x4_from_i32x4_u = VF32x4FromI32x4U { dst: VReg, src: VReg };
            /// Int-to-float conversion (same as `f64_from_x64_s`)
            vf64x2_from_i64x2_s = VF64x2FromI64x2S { dst: VReg, src: VReg };
            /// Int-to-float conversion (same as `f64_from_x64_u`)
            vf64x2_from_i64x2_u = VF64x2FromI64x2U { dst: VReg, src: VReg };
            /// Float-to-int conversion (same as `x32_from_f32_s`
            vi32x4_from_f32x4_s = VI32x4FromF32x4S { dst: VReg, src: VReg };
            /// Float-to-int conversion (same as `x32_from_f32_u`
            vi32x4_from_f32x4_u = VI32x4FromF32x4U { dst: VReg, src: VReg };
            /// Float-to-int conversion (same as `x64_from_f64_s`
            vi64x2_from_f64x2_s = VI64x2FromF64x2S { dst: VReg, src: VReg };
            /// Float-to-int conversion (same as `x64_from_f64_u`
            vi64x2_from_f64x2_u = VI64x2FromF64x2U { dst: VReg, src: VReg };

            /// Widens the low lanes of the input vector, as signed, to twice
            /// the width.
            vwidenlow8x16_s = VWidenLow8x16S { dst: VReg, src: VReg };
            /// Widens the low lanes of the input vector, as unsigned, to twice
            /// the width.
            vwidenlow8x16_u = VWidenLow8x16U { dst: VReg, src: VReg };
            /// Widens the low lanes of the input vector, as signed, to twice
            /// the width.
            vwidenlow16x8_s = VWidenLow16x8S { dst: VReg, src: VReg };
            /// Widens the low lanes of the input vector, as unsigned, to twice
            /// the width.
            vwidenlow16x8_u = VWidenLow16x8U { dst: VReg, src: VReg };
            /// Widens the low lanes of the input vector, as signed, to twice
            /// the width.
            vwidenlow32x4_s = VWidenLow32x4S { dst: VReg, src: VReg };
            /// Widens the low lanes of the input vector, as unsigned, to twice
            /// the width.
            vwidenlow32x4_u = VWidenLow32x4U { dst: VReg, src: VReg };
            /// Widens the high lanes of the input vector, as signed, to twice
            /// the width.
            vwidenhigh8x16_s = VWidenHigh8x16S { dst: VReg, src: VReg };
            /// Widens the high lanes of the input vector, as unsigned, to twice
            /// the width.
            vwidenhigh8x16_u = VWidenHigh8x16U { dst: VReg, src: VReg };
            /// Widens the high lanes of the input vector, as signed, to twice
            /// the width.
            vwidenhigh16x8_s = VWidenHigh16x8S { dst: VReg, src: VReg };
            /// Widens the high lanes of the input vector, as unsigned, to twice
            /// the width.
            vwidenhigh16x8_u = VWidenHigh16x8U { dst: VReg, src: VReg };
            /// Widens the high lanes of the input vector, as signed, to twice
            /// the width.
            vwidenhigh32x4_s = VWidenHigh32x4S { dst: VReg, src: VReg };
            /// Widens the high lanes of the input vector, as unsigned, to twice
            /// the width.
            vwidenhigh32x4_u = VWidenHigh32x4U { dst: VReg, src: VReg };

            /// Narrows the two 16x8 vectors, assuming all input lanes are
            /// signed, to half the width. Narrowing is signed and saturating.
            vnarrow16x8_s = Vnarrow16x8S { operands: BinaryOperands<VReg> };
            /// Narrows the two 16x8 vectors, assuming all input lanes are
            /// signed, to half the width. Narrowing is unsigned and saturating.
            vnarrow16x8_u = Vnarrow16x8U { operands: BinaryOperands<VReg> };
            /// Narrows the two 32x4 vectors, assuming all input lanes are
            /// signed, to half the width. Narrowing is signed and saturating.
            vnarrow32x4_s = Vnarrow32x4S { operands: BinaryOperands<VReg> };
            /// Narrows the two 32x4 vectors, assuming all input lanes are
            /// signed, to half the width. Narrowing is unsigned and saturating.
            vnarrow32x4_u = Vnarrow32x4U { operands: BinaryOperands<VReg> };
            /// Narrows the two 64x2 vectors, assuming all input lanes are
            /// signed, to half the width. Narrowing is signed and saturating.
            vnarrow64x2_s = Vnarrow64x2S { operands: BinaryOperands<VReg> };
            /// Narrows the two 64x2 vectors, assuming all input lanes are
            /// signed, to half the width. Narrowing is unsigned and saturating.
            vnarrow64x2_u = Vnarrow64x2U { operands: BinaryOperands<VReg> };
            /// Narrows the two 64x2 vectors, assuming all input lanes are
            /// unsigned, to half the width. Narrowing is unsigned and saturating.
            vunarrow64x2_u = Vunarrow64x2U { operands: BinaryOperands<VReg> };
            /// Promotes the low two lanes of the f32x4 input to f64x2.
            vfpromotelow = VFpromoteLow { dst: VReg, src: VReg };
            /// Demotes the two f64x2 lanes to f32x2 and then extends with two
            /// more zero lanes.
            vfdemote = VFdemote { dst: VReg, src: VReg };

            /// `dst = src1 - src2`
            vsubi8x16 = VSubI8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src1 - src2`
            vsubi16x8 = VSubI16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src1 - src2`
            vsubi32x4 = VSubI32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src1 - src2`
            vsubi64x2 = VSubI64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src1 - src2`
            vsubf64x2 = VSubF64x2 { operands: BinaryOperands<VReg> };

            /// `dst = saturating_sub(src1, src2)`
            vsubi8x16_sat = VSubI8x16Sat { operands: BinaryOperands<VReg> };
            /// `dst = saturating_sub(src1, src2)`
            vsubu8x16_sat = VSubU8x16Sat { operands: BinaryOperands<VReg> };
            /// `dst = saturating_sub(src1, src2)`
            vsubi16x8_sat = VSubI16x8Sat { operands: BinaryOperands<VReg> };
            /// `dst = saturating_sub(src1, src2)`
            vsubu16x8_sat = VSubU16x8Sat { operands: BinaryOperands<VReg> };

            /// `dst = src1 * src2`
            vmuli8x16 = VMulI8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src1 * src2`
            vmuli16x8 = VMulI16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src1 * src2`
            vmuli32x4 = VMulI32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src1 * src2`
            vmuli64x2 = VMulI64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src1 * src2`
            vmulf64x2 = VMulF64x2 { operands: BinaryOperands<VReg> };

            /// `dst = signed_saturate(src1 * src2 + (1 << (Q - 1)) >> Q)`
            vqmulrsi16x8 = VQmulrsI16x8 { operands: BinaryOperands<VReg> };

            /// `dst = count_ones(src)`
            vpopcnt8x16 = VPopcnt8x16 { dst: VReg, src: VReg };

            /// `low32(dst) = zext(src[lane])`
            xextractv8x16 = XExtractV8x16 { dst: XReg, src: VReg, lane: u8 };
            /// `low32(dst) = zext(src[lane])`
            xextractv16x8 = XExtractV16x8 { dst: XReg, src: VReg, lane: u8 };
            /// `low32(dst) = src[lane]`
            xextractv32x4 = XExtractV32x4 { dst: XReg, src: VReg, lane: u8 };
            /// `dst = src[lane]`
            xextractv64x2 = XExtractV64x2 { dst: XReg, src: VReg, lane: u8 };
            /// `low32(dst) = src[lane]`
            fextractv32x4 = FExtractV32x4 { dst: FReg, src: VReg, lane: u8 };
            /// `dst = src[lane]`
            fextractv64x2 = FExtractV64x2 { dst: FReg, src: VReg, lane: u8 };

            /// `dst = src1; dst[lane] = src2`
            vinsertx8 = VInsertX8 { operands: BinaryOperands<VReg, VReg, XReg>, lane: u8 };
            /// `dst = src1; dst[lane] = src2`
            vinsertx16 = VInsertX16 { operands: BinaryOperands<VReg, VReg, XReg>, lane: u8 };
            /// `dst = src1; dst[lane] = src2`
            vinsertx32 = VInsertX32 { operands: BinaryOperands<VReg, VReg, XReg>, lane: u8 };
            /// `dst = src1; dst[lane] = src2`
            vinsertx64 = VInsertX64 { operands: BinaryOperands<VReg, VReg, XReg>, lane: u8 };
            /// `dst = src1; dst[lane] = src2`
            vinsertf32 = VInsertF32 { operands: BinaryOperands<VReg, VReg, FReg>, lane: u8 };
            /// `dst = src1; dst[lane] = src2`
            vinsertf64 = VInsertF64 { operands: BinaryOperands<VReg, VReg, FReg>, lane: u8 };

            /// `dst = src == dst`
            veq8x16 = Veq8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src != dst`
            vneq8x16 = Vneq8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (signed)
            vslt8x16 = Vslt8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (signed)
            vslteq8x16 = Vslteq8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (unsigned)
            vult8x16 = Vult8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (unsigned)
            vulteq8x16 = Vulteq8x16 { operands: BinaryOperands<VReg> };
            /// `dst = src == dst`
            veq16x8 = Veq16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src != dst`
            vneq16x8 = Vneq16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (signed)
            vslt16x8 = Vslt16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (signed)
            vslteq16x8 = Vslteq16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (unsigned)
            vult16x8 = Vult16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (unsigned)
            vulteq16x8 = Vulteq16x8 { operands: BinaryOperands<VReg> };
            /// `dst = src == dst`
            veq32x4 = Veq32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src != dst`
            vneq32x4 = Vneq32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (signed)
            vslt32x4 = Vslt32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (signed)
            vslteq32x4 = Vslteq32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (unsigned)
            vult32x4 = Vult32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (unsigned)
            vulteq32x4 = Vulteq32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src == dst`
            veq64x2 = Veq64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src != dst`
            vneq64x2 = Vneq64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (signed)
            vslt64x2 = Vslt64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (signed)
            vslteq64x2 = Vslteq64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst` (unsigned)
            vult64x2 = Vult64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst` (unsigned)
            vulteq64x2 = Vulteq64x2 { operands: BinaryOperands<VReg> };

            /// `dst = -src`
            vneg8x16 = Vneg8x16 { dst: VReg, src: VReg };
            /// `dst = -src`
            vneg16x8 = Vneg16x8 { dst: VReg, src: VReg };
            /// `dst = -src`
            vneg32x4 = Vneg32x4 { dst: VReg, src: VReg };
            /// `dst = -src`
            vneg64x2 = Vneg64x2 { dst: VReg, src: VReg };
            /// `dst = -src`
            vnegf64x2 = VnegF64x2 { dst: VReg, src: VReg };

            /// `dst = min(src1, src2)` (signed)
            vmin8x16_s = Vmin8x16S { operands: BinaryOperands<VReg> };
            /// `dst = min(src1, src2)` (unsigned)
            vmin8x16_u = Vmin8x16U { operands: BinaryOperands<VReg> };
            /// `dst = min(src1, src2)` (signed)
            vmin16x8_s = Vmin16x8S { operands: BinaryOperands<VReg> };
            /// `dst = min(src1, src2)` (unsigned)
            vmin16x8_u = Vmin16x8U { operands: BinaryOperands<VReg> };
            /// `dst = max(src1, src2)` (signed)
            vmax8x16_s = Vmax8x16S { operands: BinaryOperands<VReg> };
            /// `dst = max(src1, src2)` (unsigned)
            vmax8x16_u = Vmax8x16U { operands: BinaryOperands<VReg> };
            /// `dst = max(src1, src2)` (signed)
            vmax16x8_s = Vmax16x8S { operands: BinaryOperands<VReg> };
            /// `dst = max(src1, src2)` (unsigned)
            vmax16x8_u = Vmax16x8U { operands: BinaryOperands<VReg> };

            /// `dst = min(src1, src2)` (signed)
            vmin32x4_s = Vmin32x4S { operands: BinaryOperands<VReg> };
            /// `dst = min(src1, src2)` (unsigned)
            vmin32x4_u = Vmin32x4U { operands: BinaryOperands<VReg> };
            /// `dst = max(src1, src2)` (signed)
            vmax32x4_s = Vmax32x4S { operands: BinaryOperands<VReg> };
            /// `dst = max(src1, src2)` (unsigned)
            vmax32x4_u = Vmax32x4U { operands: BinaryOperands<VReg> };

            /// `dst = |src|`
            vabs8x16 = Vabs8x16 { dst: VReg, src: VReg };
            /// `dst = |src|`
            vabs16x8 = Vabs16x8 { dst: VReg, src: VReg };
            /// `dst = |src|`
            vabs32x4 = Vabs32x4 { dst: VReg, src: VReg };
            /// `dst = |src|`
            vabs64x2 = Vabs64x2 { dst: VReg, src: VReg };

            /// `dst = |src|`
            vabsf32x4 = Vabsf32x4 { dst: VReg, src: VReg };
            /// `dst = |src|`
            vabsf64x2 = Vabsf64x2 { dst: VReg, src: VReg };
            /// `dst = ieee_maximum(src1, src2)`
            vmaximumf32x4 = Vmaximumf32x4 { operands: BinaryOperands<VReg> };
            /// `dst = ieee_maximum(src1, src2)`
            vmaximumf64x2 = Vmaximumf64x2 { operands: BinaryOperands<VReg> };
            /// `dst = ieee_minimum(src1, src2)`
            vminimumf32x4 = Vminimumf32x4 { operands: BinaryOperands<VReg> };
            /// `dst = ieee_minimum(src1, src2)`
            vminimumf64x2 = Vminimumf64x2 { operands: BinaryOperands<VReg> };

            /// `dst = shuffle(src1, src2, mask)`
            vshuffle = VShuffle { dst: VReg, src1: VReg, src2: VReg, mask: u128 };

            /// `dst = swizzle(src1, src2)`
            vswizzlei8x16 = Vswizzlei8x16 { operands: BinaryOperands<VReg> };

            /// `dst = (src1 + src2 + 1) // 2`
            vavground8x16 = Vavground8x16 { operands: BinaryOperands<VReg> };
            /// `dst = (src1 + src2 + 1) // 2`
            vavground16x8 = Vavground16x8 { operands: BinaryOperands<VReg> };

            /// `dst = src == dst`
            veqf32x4 = VeqF32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src != dst`
            vneqf32x4 = VneqF32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst`
            vltf32x4 = VltF32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst`
            vlteqf32x4 = VlteqF32x4 { operands: BinaryOperands<VReg> };
            /// `dst = src == dst`
            veqf64x2 = VeqF64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src != dst`
            vneqf64x2 = VneqF64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src < dst`
            vltf64x2 = VltF64x2 { operands: BinaryOperands<VReg> };
            /// `dst = src <= dst`
            vlteqf64x2 = VlteqF64x2 { operands: BinaryOperands<VReg> };

            /// `dst = ieee_fma(a, b, c)`
            vfma32x4 = Vfma32x4 { dst: VReg, a: VReg, b: VReg, c: VReg };
            /// `dst = ieee_fma(a, b, c)`
            vfma64x2 = Vfma64x2 { dst: VReg, a: VReg, b: VReg, c: VReg };

            /// `dst = low32(cond) ? if_nonzero : if_zero`
            vselect = Vselect { dst: VReg, cond: XReg, if_nonzero: VReg, if_zero: VReg };

            /// `dst_hi:dst_lo = lhs_hi:lhs_lo + rhs_hi:rhs_lo`
            xadd128 = Xadd128 {
                dst_lo: XReg,
                dst_hi: XReg,
                lhs_lo: XReg,
                lhs_hi: XReg,
                rhs_lo: XReg,
                rhs_hi: XReg
            };
            /// `dst_hi:dst_lo = lhs_hi:lhs_lo - rhs_hi:rhs_lo`
            xsub128 = Xsub128 {
                dst_lo: XReg,
                dst_hi: XReg,
                lhs_lo: XReg,
                lhs_hi: XReg,
                rhs_lo: XReg,
                rhs_hi: XReg
            };
            /// `dst_hi:dst_lo = sext(lhs) * sext(rhs)`
            xwidemul64_s = Xwidemul64S {
                dst_lo: XReg,
                dst_hi: XReg,
                lhs: XReg,
                rhs: XReg
            };
            /// `dst_hi:dst_lo = zext(lhs) * zext(rhs)`
            xwidemul64_u = Xwidemul64U {
                dst_lo: XReg,
                dst_hi: XReg,
                lhs: XReg,
                rhs: XReg
            };
        }
    };
}

#[cfg(feature = "decode")]
pub mod decode;
#[cfg(feature = "disas")]
pub mod disas;
#[cfg(feature = "encode")]
pub mod encode;
#[cfg(feature = "interp")]
pub mod interp;
#[cfg(feature = "profile")]
pub mod profile;
#[cfg(all(not(feature = "profile"), feature = "interp"))]
mod profile_disabled;
#[cfg(all(not(feature = "profile"), feature = "interp"))]
use profile_disabled as profile;

pub mod regs;
pub use regs::*;

pub mod imms;
pub use imms::*;

pub mod op;
pub use op::*;

pub mod opcode;
pub use opcode::*;

#[cfg(any(feature = "encode", feature = "decode"))]
pub(crate) unsafe fn unreachable_unchecked<T>() -> T {
    #[cfg(debug_assertions)]
    unreachable!();

    #[cfg(not(debug_assertions))]
    unsafe {
        core::hint::unreachable_unchecked()
    }
}
