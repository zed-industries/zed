//! Decoding support for pulley bytecode.

use core::ptr::NonNull;

use alloc::vec::Vec;
use cranelift_bitset::ScalarBitSet;
use cranelift_bitset::scalar::ScalarBitSetStorage;

use crate::imms::*;
use crate::opcode::*;
use crate::regs::*;

/// Either an `Ok(T)` or an `Err(DecodingError)`.
pub type Result<T, E = DecodingError> = core::result::Result<T, E>;

/// An error when decoding Pulley bytecode.
pub enum DecodingError {
    /// Reached the end of the bytecode stream before we finished decoding a
    /// single bytecode.
    UnexpectedEof {
        /// The position in the bytecode stream where this error occurred.
        position: usize,
    },

    /// Found an invalid opcode.
    InvalidOpcode {
        /// The position in the bytecode stream where this error occurred.
        position: usize,
        /// The invalid opcode that was found.
        code: u8,
    },

    /// Found an invalid extended opcode.
    InvalidExtendedOpcode {
        /// The position in the bytecode stream where this error occurred.
        position: usize,
        /// The invalid extended opcode that was found.
        code: u16,
    },

    /// Found an invalid register.
    InvalidReg {
        /// The position in the bytecode stream where this error occurred.
        position: usize,
        /// The invalid register that was found.
        reg: u8,
    },
}

impl core::fmt::Debug for DecodingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for DecodingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnexpectedEof { position } => {
                write!(f, "unexpected end-of-file at bytecode offset {position:#x}")
            }
            Self::InvalidOpcode { position, code } => {
                write!(
                    f,
                    "found invalid opcode {code:#x} at bytecode offset {position:#x}"
                )
            }
            Self::InvalidExtendedOpcode { position, code } => {
                write!(
                    f,
                    "found invalid opcode {code:#x} at bytecode offset {position:#x}"
                )
            }
            Self::InvalidReg { position, reg } => {
                write!(
                    f,
                    "found invalid register {reg:#x} at bytecode offset {position:#x}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecodingError {}

/// An abstraction over any kind of bytecode stream.
///
/// There are two primary implementations:
///
/// 1. `SafeBytecodeStream`: A thin wrapper around an index into a `&[u8]`. This
///    implementation is 100% safe code.
///
/// 2. `UnsafeBytecodeStream`: A thin wrapper over a raw pointer. This
///    implementation is wildly unsafe and will result in memory unsafety and
///    other terrors when given invalid bytecode, or even valid bytecode
///    encoding a program that itself does not preserve memory safety.
pub trait BytecodeStream: Copy {
    /// The type of error that this bytecode stream produces on invalid
    /// operations.
    type Error;

    /// Create an "unexpected end-of-stream" error at the current position.
    fn unexpected_eof(&self) -> Self::Error;

    /// Create an "invalid opcode" error at the current position.
    fn invalid_opcode(&self, code: u8) -> Self::Error;

    /// Create an "invalid extended opcode" error at the current position.
    fn invalid_extended_opcode(&self, code: u16) -> Self::Error;

    /// Create an "invalid register" error at the current position.
    fn invalid_reg(&self, reg: u8) -> Self::Error;

    /// Read `N` bytes from this bytecode stream, advancing the stream's
    /// position at the same time.
    fn read<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;
}

/// A 100% safe implementation of a bytecode stream.
///
/// This is a thin wrapper around an index into a `&[u8]`.
#[derive(Clone, Copy, Debug)]
pub struct SafeBytecodeStream<'a> {
    bytecode: &'a [u8],
    position: usize,
}

impl<'a> SafeBytecodeStream<'a> {
    /// Create a new `SafeBytecodeStream` from the given slice and with an
    /// initial position pointing at the start of the slice.
    pub fn new(bytecode: &'a [u8]) -> Self {
        Self {
            bytecode,
            position: 0,
        }
    }

    /// Get this stream's current position within its underlying slice.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get this stream's underlying bytecode slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytecode
    }
}

impl BytecodeStream for SafeBytecodeStream<'_> {
    fn read<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let (bytes, rest) = self
            .bytecode
            .split_first_chunk()
            .ok_or_else(|| self.unexpected_eof())?;
        self.bytecode = rest;
        self.position += N;
        Ok(*bytes)
    }

    type Error = DecodingError;

    fn unexpected_eof(&self) -> Self::Error {
        DecodingError::UnexpectedEof {
            position: self.position,
        }
    }

    fn invalid_opcode(&self, code: u8) -> Self::Error {
        DecodingError::InvalidOpcode {
            position: self.position - 1,
            code,
        }
    }

    fn invalid_extended_opcode(&self, code: u16) -> Self::Error {
        DecodingError::InvalidExtendedOpcode {
            position: self.position,
            code,
        }
    }

    fn invalid_reg(&self, reg: u8) -> Self::Error {
        DecodingError::InvalidReg {
            position: self.position,
            reg,
        }
    }
}

/// An uninhabited type that cannot be constructed at runtime.
#[derive(Debug)]
pub enum Uninhabited {}

/// An unsafe bytecode stream.
///
/// This is a wrapper over a raw pointer to bytecode somewhere in memory.
#[derive(Clone, Copy, Debug)]
pub struct UnsafeBytecodeStream(NonNull<u8>);

impl UnsafeBytecodeStream {
    /// Construct a new `UnsafeBytecodeStream` pointing at the given PC.
    ///
    /// # Safety
    ///
    /// The given `pc` must point to valid Pulley bytecode, and it is the
    /// caller's responsibility to ensure that the resulting
    /// `UnsafeBytecodeStream` is only used to access the valid bytecode. For
    /// example, if the current bytecode instruction unconditionally jumps to a
    /// new PC, this stream must not be used to read just after the
    /// unconditional jump instruction because there is no guarantee that that
    /// memory is part of the bytecode stream or not.
    pub unsafe fn new(pc: NonNull<u8>) -> Self {
        UnsafeBytecodeStream(pc)
    }

    /// Get a new `UnsafeBytecodeStream` pointing at the bytecode that is at the
    /// given relative offset from this stream's current position.
    ///
    /// # Safety
    ///
    /// Same as the `new` constructor. May only be used when it is guaranteed
    /// that the address at `self._as_ptr() + offset` contains valid Pulley
    /// bytecode.
    pub unsafe fn offset(&self, offset: isize) -> Self {
        UnsafeBytecodeStream(unsafe { NonNull::new_unchecked(self.0.as_ptr().offset(offset)) })
    }

    /// Get this stream's underlying raw pointer.
    pub fn as_ptr(&self) -> NonNull<u8> {
        self.0
    }
}

impl BytecodeStream for UnsafeBytecodeStream {
    fn read<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let bytes = unsafe { self.0.cast::<[u8; N]>().as_ptr().read() };
        self.0 = unsafe { NonNull::new_unchecked(self.0.as_ptr().add(N)) };
        Ok(bytes)
    }

    type Error = Uninhabited;

    fn unexpected_eof(&self) -> Self::Error {
        unsafe { crate::unreachable_unchecked() }
    }

    fn invalid_opcode(&self, _code: u8) -> Self::Error {
        unsafe { crate::unreachable_unchecked() }
    }

    fn invalid_extended_opcode(&self, _code: u16) -> Self::Error {
        unsafe { crate::unreachable_unchecked() }
    }

    fn invalid_reg(&self, _reg: u8) -> Self::Error {
        unsafe { crate::unreachable_unchecked() }
    }
}

/// Anything that can be decoded from a bytecode stream, e.g. opcodes,
/// immediates, registers, etc...
pub trait Decode: Sized {
    /// Decode this type from the given bytecode stream.
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream;
}

impl Decode for u8 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        bytecode.read::<1>().map(|a| a[0])
    }
}

impl Decode for u16 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(u16::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for u32 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(u32::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for u64 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(u64::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for u128 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(u128::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for i8 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        bytecode.read::<1>().map(|a| a[0] as i8)
    }
}

impl Decode for i16 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(i16::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for i32 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(i32::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for i64 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(i64::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for i128 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(i128::from_le_bytes(bytecode.read()?))
    }
}

impl Decode for XReg {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        let byte = u8::decode(bytecode)?;
        XReg::new(byte).ok_or_else(|| bytecode.invalid_reg(byte))
    }
}

impl Decode for FReg {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        let byte = u8::decode(bytecode)?;
        FReg::new(byte).ok_or_else(|| bytecode.invalid_reg(byte))
    }
}

impl Decode for VReg {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        let byte = u8::decode(bytecode)?;
        VReg::new(byte).ok_or_else(|| bytecode.invalid_reg(byte))
    }
}

impl Decode for PcRelOffset {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        i32::decode(bytecode).map(|x| Self::from(x))
    }
}

impl Decode for Opcode {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        let byte = u8::decode(bytecode)?;
        match Opcode::new(byte) {
            Some(v) => Ok(v),
            None => Err(bytecode.invalid_opcode(byte)),
        }
    }
}

impl Decode for ExtendedOpcode {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        let word = u16::decode(bytecode)?;
        match ExtendedOpcode::new(word) {
            Some(v) => Ok(v),
            None => Err(bytecode.invalid_extended_opcode(word)),
        }
    }
}

impl<D: Reg, S1: Reg, S2: Reg> Decode for BinaryOperands<D, S1, S2> {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        u16::decode(bytecode).map(|bits| Self::from_bits(bits))
    }
}

impl<D: Reg, S1: Reg> Decode for BinaryOperands<D, S1, U6> {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        u16::decode(bytecode).map(|bits| Self::from_bits(bits))
    }
}

impl<S: Decode + ScalarBitSetStorage> Decode for ScalarBitSet<S> {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        S::decode(bytecode).map(ScalarBitSet::from)
    }
}

impl<R: Reg + Decode> Decode for UpperRegSet<R> {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        ScalarBitSet::decode(bytecode).map(Self::from)
    }
}

impl Decode for AddrO32 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(AddrO32 {
            addr: XReg::decode(bytecode)?,
            offset: i32::decode(bytecode)?,
        })
    }
}

impl Decode for AddrZ {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(AddrZ {
            addr: XReg::decode(bytecode)?,
            offset: i32::decode(bytecode)?,
        })
    }
}

impl Decode for AddrG32 {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(AddrG32::from_bits(u32::decode(bytecode)?))
    }
}

impl Decode for AddrG32Bne {
    fn decode<T>(bytecode: &mut T) -> Result<Self, T::Error>
    where
        T: BytecodeStream,
    {
        Ok(AddrG32Bne::from_bits(u32::decode(bytecode)?))
    }
}

/// A Pulley bytecode decoder.
///
/// Does not materialize bytecode instructions, instead all decoding methods are
/// given an `OpVisitor` implementation and the appropriate visitor methods are
/// called upon decoding an instruction. This minimizes the amount of times we
/// branch on the opcode, avoids constructing temporary storage, and plays well
/// with our variable-length instruction encoding.
#[derive(Default)]
pub struct Decoder {
    _private: (),
}

impl Decoder {
    /// Create a new decoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode all instructions in the visitor's bytecode stream.
    ///
    /// The associated visitor method is invoked after each instruction is
    /// decoded.
    pub fn decode_all<'a, V>(visitor: &mut V) -> Result<Vec<V::Return>>
    where
        V: OpVisitor<BytecodeStream = SafeBytecodeStream<'a>> + ExtendedOpVisitor,
    {
        let mut decoder = Decoder::new();
        let mut results = Vec::new();

        while !visitor.bytecode().as_slice().is_empty() {
            results.push(decoder.decode_one(visitor)?);
        }

        Ok(results)
    }
}

/// An `OpVisitor` combinator to sequence one visitor and then another.
pub struct SequencedVisitor<'a, F, V1, V2> {
    join: F,
    v1: &'a mut V1,
    v2: &'a mut V2,
}

impl<'a, F, V1, V2> SequencedVisitor<'a, F, V1, V2> {
    /// Create a new sequenced visitor.
    ///
    /// The given `join` function is used to combine the results of each
    /// sub-visitor so that it can be returned from this combined visitor.
    pub fn new(join: F, v1: &'a mut V1, v2: &'a mut V2) -> Self {
        SequencedVisitor { join, v1, v2 }
    }
}

macro_rules! define_decoder {
    (
        $(
            $( #[$attr:meta] )*
                $snake_name:ident = $name:ident $( {
                $(
                    $( #[$field_attr:meta] )*
                    $field:ident : $field_ty:ty
                ),*
            } )? ;
        )*
    ) => {
        impl Decoder {
            /// Decode one instruction from the visitor's bytestream.
            ///
            /// Upon decoding, the visitor's associated callback is invoked and
            /// the results returned.
            #[inline(always)]
            pub fn decode_one<V>(
                &mut self,
                visitor: &mut V,
            ) -> Result<V::Return, <V::BytecodeStream as BytecodeStream>::Error>
            where
                V: OpVisitor + ExtendedOpVisitor,
            {
                visitor.before_visit();

                let byte = u8::decode(visitor.bytecode())?;
                let opcode = Opcode::new(byte).ok_or_else(|| {
                    visitor.bytecode().invalid_opcode(byte)
                })?;

                match opcode {
                    $(
                        Opcode::$name => {
                            $(
                                $(
                                    let $field = <$field_ty>::decode(
                                        visitor.bytecode(),
                                    )?;
                                )*
                            )?

                            let ret = visitor.$snake_name($( $( $field ),* )?);
                            visitor.after_visit();
                            Ok(ret)
                        },
                    )*
                    Opcode::ExtendedOp => {
                        decode_one_extended(visitor)
                    }
                }
            }
        }

        /// Callbacks upon decoding instructions from bytecode.
        ///
        /// Implement this trait for your type, give an instance of your type to
        /// a `Decoder` method, and the `Decoder` will invoke the associated
        /// method for each instruction that it decodes. For example, if the
        /// `Decoder` decodes an `xadd32` instruction, then it will invoke the
        /// `xadd32` visitor method, passing along any decoded immediates,
        /// operands, etc... as arguments.
        pub trait OpVisitor {
            /// The type of this visitor's bytecode stream.
            type BytecodeStream: BytecodeStream;

            /// Get this visitor's underlying bytecode stream.
            fn bytecode(&mut self) -> &mut Self::BytecodeStream;

            /// The type of values returned by each visitor method.
            type Return;

            /// A callback invoked before starting to decode an instruction.
            ///
            /// Does nothing by default.
            fn before_visit(&mut self) {}

            /// A callback invoked after an instruction has been completely
            /// decoded.
            ///
            /// Does nothing by default.
            fn after_visit(&mut self) {}

            $(
                $( #[$attr] )*
                fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return;
            )*
        }

        impl<F, T, V1, V2> OpVisitor for SequencedVisitor<'_, F, V1, V2>
        where
            F: FnMut(V1::Return, V2::Return) -> T,
            V1: OpVisitor,
            V2: OpVisitor<BytecodeStream = V1::BytecodeStream>,
        {
            type BytecodeStream = V1::BytecodeStream;

            fn bytecode(&mut self) -> &mut Self::BytecodeStream {
                self.v1.bytecode()
            }

            type Return = T;

            fn before_visit(&mut self) {
                self.v1.before_visit();
                self.v2.before_visit();
            }

            fn after_visit(&mut self) {
                *self.v2.bytecode() = *self.v1.bytecode();
                self.v1.after_visit();
                self.v2.after_visit();
            }

            $(
                $( #[$attr] )*
                fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return {
                    let a = self.v1.$snake_name( $( $( $field , )* )? );
                    let b = self.v2.$snake_name( $( $( $field , )* )? );
                    (self.join)(a, b)
                }
            )*
        }
    };
}
for_each_op!(define_decoder);

macro_rules! define_extended_decoder {
    (
        $(
            $( #[$attr:meta] )*
                $snake_name:ident = $name:ident $( {
                $(
                    $( #[$field_attr:meta] )*
                    $field:ident : $field_ty:ty
                ),*
            } )? ;
        )*
    ) => {
        /// Like `OpVisitor` but for extended operations.
        pub trait ExtendedOpVisitor: OpVisitor {
            $(
                $( #[$attr] )*
                fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return;
            )*
        }

        fn decode_one_extended<V>(
            visitor: &mut V,
        ) -> Result<V::Return, <V::BytecodeStream as BytecodeStream>::Error>
        where
            V: ExtendedOpVisitor,
        {
            let code = u16::decode(visitor.bytecode())?;
            let opcode = ExtendedOpcode::new(code).ok_or_else(|| {
                visitor.bytecode().invalid_extended_opcode(code)
            })?;

            match opcode {
                $(
                    ExtendedOpcode::$name => {
                        $(
                            $(
                                let $field = <$field_ty>::decode(
                                    visitor.bytecode(),
                                )?;
                            )*
                        )?

                        let ret = visitor.$snake_name($( $( $field ),* )?);
                        visitor.after_visit();
                        Ok(ret)
                    }
                )*
            }
        }


        impl<F, T, V1, V2> ExtendedOpVisitor for SequencedVisitor<'_, F, V1, V2>
        where
            F: FnMut(V1::Return, V2::Return) -> T,
            V1: ExtendedOpVisitor,
            V2: ExtendedOpVisitor<BytecodeStream = V1::BytecodeStream>,
        {
            $(
                $( #[$attr] )*
                fn $snake_name(&mut self $( $( , $field : $field_ty )* )? ) -> Self::Return {
                    let a = self.v1.$snake_name( $( $( $field , )* )? );
                    let b = self.v2.$snake_name( $( $( $field , )* )? );
                    (self.join)(a, b)
                }
            )*
        }
    };
}
for_each_extended_op!(define_extended_decoder);

/// Functions for decoding the operands of an instruction, assuming the opcode
/// has already been decoded.
pub mod operands {
    use super::*;

    macro_rules! define_operands_decoder {
        (
            $(
                $( #[$attr:meta] )*
                    $snake_name:ident = $name:ident $( {
                    $(
                        $( #[$field_attr:meta] )*
                        $field:ident : $field_ty:ty
                    ),*
                } )? ;
            )*
        ) => {
            $(
                #[allow(unused_variables, reason = "macro-generated")]
                #[expect(missing_docs, reason = "macro-generated")]
                pub fn $snake_name<T: BytecodeStream>(pc: &mut T) -> Result<($($($field_ty,)*)?), T::Error> {
                    Ok((($($((<$field_ty>::decode(pc))?,)*)?)))
                }
            )*
        };
    }

    for_each_op!(define_operands_decoder);

    /// Decode an extended opcode from `pc` to match the payload of the
    /// "extended" opcode.
    pub fn extended<T: BytecodeStream>(pc: &mut T) -> Result<(ExtendedOpcode,), T::Error> {
        Ok((ExtendedOpcode::decode(pc)?,))
    }

    for_each_extended_op!(define_operands_decoder);
}
