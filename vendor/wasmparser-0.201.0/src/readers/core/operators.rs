/* Copyright 2018 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::limits::MAX_WASM_CATCHES;
use crate::{BinaryReader, BinaryReaderError, FromReader, Result, ValType};

/// Represents a block type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlockType {
    /// The block produces consumes nor produces any values.
    Empty,
    /// The block produces a singular value of the given type ([] -> \[t]).
    Type(ValType),
    /// The block is described by a function type.
    ///
    /// The index is to a function type in the types section.
    FuncType(u32),
}

/// Represents a memory immediate in a WebAssembly memory instruction.
#[derive(Debug, Copy, Clone)]
pub struct MemArg {
    /// Alignment, stored as `n` where the actual alignment is `2^n`
    pub align: u8,
    /// Maximum alignment, stored as `n` where the actual alignment is `2^n`.
    ///
    /// Note that this field is not actually read from the binary format, it
    /// will be a constant depending on which instruction this `MemArg` is a
    /// payload for.
    pub max_align: u8,
    /// A fixed byte-offset that this memory immediate specifies.
    ///
    /// Note that the memory64 proposal can specify a full 64-bit byte offset
    /// while otherwise only 32-bit offsets are allowed. Once validated
    /// memory immediates for 32-bit memories are guaranteed to be at most
    /// `u32::MAX` whereas 64-bit memories can use the full 64-bits.
    pub offset: u64,
    /// The index of the memory this immediate points to.
    ///
    /// Note that this points within the module's own memory index space, and
    /// is always zero unless the multi-memory proposal of WebAssembly is
    /// enabled.
    pub memory: u32,
}

/// A br_table entries representation.
#[derive(Clone)]
pub struct BrTable<'a> {
    pub(crate) reader: crate::BinaryReader<'a>,
    pub(crate) cnt: u32,
    pub(crate) default: u32,
}

/// An IEEE binary32 immediate floating point value, represented as a u32
/// containing the bit pattern.
///
/// All bit patterns are allowed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ieee32(pub(crate) u32);

impl Ieee32 {
    /// Gets the underlying bits of the 32-bit float.
    pub fn bits(self) -> u32 {
        self.0
    }
}

/// An IEEE binary64 immediate floating point value, represented as a u64
/// containing the bit pattern.
///
/// All bit patterns are allowed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ieee64(pub(crate) u64);

impl Ieee64 {
    /// Gets the underlying bits of the 64-bit float.
    pub fn bits(self) -> u64 {
        self.0
    }
}

/// Represents a 128-bit vector value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct V128(pub(crate) [u8; 16]);

impl V128 {
    /// Gets the bytes of the vector value.
    pub fn bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Gets a signed 128-bit integer value from the vector's bytes.
    pub fn i128(&self) -> i128 {
        i128::from_le_bytes(self.0)
    }
}

macro_rules! define_operator {
    ($(@$proposal:ident $op:ident $({ $($payload:tt)* })? => $visit:ident)*) => {
        /// Instructions as defined [here].
        ///
        /// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
        #[derive(Debug, Clone)]
        #[allow(missing_docs)]
        pub enum Operator<'a> {
            $(
                $op $({ $($payload)* })?,
            )*
        }
    }
}
for_each_operator!(define_operator);

/// A reader for a core WebAssembly function's operators.
#[derive(Clone)]
pub struct OperatorsReader<'a> {
    pub(crate) reader: BinaryReader<'a>,
}

impl<'a> OperatorsReader<'a> {
    pub(crate) fn new(reader: BinaryReader<'a>) -> OperatorsReader<'a> {
        OperatorsReader { reader }
    }

    /// Determines if the reader is at the end of the operators.
    pub fn eof(&self) -> bool {
        self.reader.eof()
    }

    /// Gets the original position of the reader.
    pub fn original_position(&self) -> usize {
        self.reader.original_position()
    }

    /// Whether or not to allow 64-bit memory arguments in the
    /// the operators being read.
    ///
    /// This is intended to be `true` when support for the memory64
    /// WebAssembly proposal is also enabled.
    pub fn allow_memarg64(&mut self, allow: bool) {
        self.reader.allow_memarg64(allow);
    }

    /// Ensures the reader is at the end.
    ///
    /// This function returns an error if there is extra data after the operators.
    pub fn ensure_end(&self) -> Result<()> {
        if self.eof() {
            return Ok(());
        }
        Err(BinaryReaderError::new(
            "unexpected data at the end of operators",
            self.reader.original_position(),
        ))
    }

    /// Reads an operator from the reader.
    pub fn read(&mut self) -> Result<Operator<'a>> {
        self.reader.read_operator()
    }

    /// Converts to an iterator of operators paired with offsets.
    pub fn into_iter_with_offsets(self) -> OperatorsIteratorWithOffsets<'a> {
        OperatorsIteratorWithOffsets {
            reader: self,
            err: false,
        }
    }

    /// Reads an operator with its offset.
    pub fn read_with_offset(&mut self) -> Result<(Operator<'a>, usize)> {
        let pos = self.reader.original_position();
        Ok((self.read()?, pos))
    }

    /// Visit a single operator with the specified [`VisitOperator`] instance.
    ///
    /// See [`BinaryReader::visit_operator`] for more information.
    pub fn visit_operator<T>(&mut self, visitor: &mut T) -> Result<<T as VisitOperator<'a>>::Output>
    where
        T: VisitOperator<'a>,
    {
        self.reader.visit_operator(visitor)
    }

    /// Gets a binary reader from this operators reader.
    pub fn get_binary_reader(&self) -> BinaryReader<'a> {
        self.reader.clone()
    }
}

impl<'a> IntoIterator for OperatorsReader<'a> {
    type Item = Result<Operator<'a>>;
    type IntoIter = OperatorsIterator<'a>;

    /// Reads content of the code section.
    ///
    /// # Examples
    /// ```
    /// use wasmparser::{Operator, CodeSectionReader, Result};
    /// # let data: &[u8] = &[
    /// #     0x01, 0x03, 0x00, 0x01, 0x0b];
    /// let code_reader = CodeSectionReader::new(data, 0).unwrap();
    /// for body in code_reader {
    ///     let body = body.expect("function body");
    ///     let mut op_reader = body.get_operators_reader().expect("op reader");
    ///     let ops = op_reader.into_iter().collect::<Result<Vec<Operator>>>().expect("ops");
    ///     assert!(
    ///         if let [Operator::Nop, Operator::End] = ops.as_slice() { true } else { false },
    ///         "found {:?}",
    ///         ops
    ///     );
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        OperatorsIterator {
            reader: self,
            err: false,
        }
    }
}

/// An iterator over a function's operators.
pub struct OperatorsIterator<'a> {
    reader: OperatorsReader<'a>,
    err: bool,
}

impl<'a> Iterator for OperatorsIterator<'a> {
    type Item = Result<Operator<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.err || self.reader.eof() {
            return None;
        }
        let result = self.reader.read();
        self.err = result.is_err();
        Some(result)
    }
}

/// An iterator over a function's operators with offsets.
pub struct OperatorsIteratorWithOffsets<'a> {
    reader: OperatorsReader<'a>,
    err: bool,
}

impl<'a> Iterator for OperatorsIteratorWithOffsets<'a> {
    type Item = Result<(Operator<'a>, usize)>;

    /// Reads content of the code section with offsets.
    ///
    /// # Examples
    /// ```
    /// use wasmparser::{Operator, CodeSectionReader, Result};
    /// # let data: &[u8] = &[
    /// #     0x01, 0x03, 0x00, /* offset = 23 */ 0x01, 0x0b];
    /// let code_reader = CodeSectionReader::new(data, 20).unwrap();
    /// for body in code_reader {
    ///     let body = body.expect("function body");
    ///     let mut op_reader = body.get_operators_reader().expect("op reader");
    ///     let ops = op_reader.into_iter_with_offsets().collect::<Result<Vec<(Operator, usize)>>>().expect("ops");
    ///     assert!(
    ///         if let [(Operator::Nop, 23), (Operator::End, 24)] = ops.as_slice() { true } else { false },
    ///         "found {:?}",
    ///         ops
    ///     );
    /// }
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        if self.err || self.reader.eof() {
            return None;
        }
        let result = self.reader.read_with_offset();
        self.err = result.is_err();
        Some(result)
    }
}

macro_rules! define_visit_operator {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output;
        )*
    }
}

/// Trait implemented by types that can visit all [`Operator`] variants.
#[allow(missing_docs)]
pub trait VisitOperator<'a> {
    /// The result type of the visitor.
    type Output: 'a;

    /// Visits the [`Operator`] `op` using the given `offset`.
    ///
    /// # Note
    ///
    /// This is a convenience method that is intended for non-performance
    /// critical use cases. For performance critical implementations users
    /// are recommended to directly use the respective `visit` methods or
    /// implement [`VisitOperator`] on their own.
    fn visit_operator(&mut self, op: &Operator<'a>) -> Self::Output {
        macro_rules! visit_operator {
            ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
                match op {
                    $(
                        Operator::$op $({ $($arg),* })? => self.$visit($($($arg.clone()),*)?),
                    )*
                }
            }

        }
        for_each_operator!(visit_operator)
    }

    for_each_operator!(define_visit_operator);
}

macro_rules! define_visit_operator_delegate {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
                V::$visit(&mut *self, $($($arg),*)?)
            }
        )*
    }
}

impl<'a, 'b, V: VisitOperator<'a> + ?Sized> VisitOperator<'a> for &'b mut V {
    type Output = V::Output;
    fn visit_operator(&mut self, op: &Operator<'a>) -> Self::Output {
        V::visit_operator(*self, op)
    }
    for_each_operator!(define_visit_operator_delegate);
}

impl<'a, V: VisitOperator<'a> + ?Sized> VisitOperator<'a> for Box<V> {
    type Output = V::Output;
    fn visit_operator(&mut self, op: &Operator<'a>) -> Self::Output {
        V::visit_operator(&mut *self, op)
    }
    for_each_operator!(define_visit_operator_delegate);
}

/// A `try_table` entries representation.
#[derive(Clone, Debug)]
pub struct TryTable {
    /// The block type describing the try block itself.
    pub ty: BlockType,
    /// Outer blocks which will receive exceptions.
    pub catches: Vec<Catch>,
}

/// Catch clauses that can be specified in [`TryTable`].
#[derive(Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum Catch {
    /// Equivalent of `catch`
    One { tag: u32, label: u32 },
    /// Equivalent of `catch_ref`
    OneRef { tag: u32, label: u32 },
    /// Equivalent of `catch_all`
    All { label: u32 },
    /// Equivalent of `catch_all_ref`
    AllRef { label: u32 },
}

impl<'a> FromReader<'a> for TryTable {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let ty = reader.read_block_type()?;
        let catches = reader
            .read_iter(MAX_WASM_CATCHES, "catches")?
            .collect::<Result<_>>()?;
        Ok(TryTable { ty, catches })
    }
}

impl<'a> FromReader<'a> for Catch {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => Catch::One {
                tag: reader.read_var_u32()?,
                label: reader.read_var_u32()?,
            },
            0x01 => Catch::OneRef {
                tag: reader.read_var_u32()?,
                label: reader.read_var_u32()?,
            },
            0x02 => Catch::All {
                label: reader.read_var_u32()?,
            },
            0x03 => Catch::AllRef {
                label: reader.read_var_u32()?,
            },

            x => return reader.invalid_leading_byte(x, "catch"),
        })
    }
}
