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

use crate::limits::{MAX_WASM_CATCHES, MAX_WASM_HANDLERS};
use crate::prelude::*;
use crate::{BinaryReader, BinaryReaderError, FromReader, Result, ValType};
use core::{fmt, mem};

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

/// The kind of a control flow `Frame`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FrameKind {
    /// A Wasm `block` control block.
    Block,
    /// A Wasm `if` control block.
    If,
    /// A Wasm `else` control block.
    Else,
    /// A Wasm `loop` control block.
    Loop,
    /// A Wasm `try` control block.
    ///
    /// # Note
    ///
    /// This belongs to the Wasm exception handling proposal.
    TryTable,
    /// A Wasm legacy `try` control block.
    ///
    /// # Note
    ///
    /// See: `WasmFeatures::legacy_exceptions` Note in `crates/wasmparser/src/features.rs`
    LegacyTry,
    /// A Wasm legacy `catch` control block.
    ///
    /// # Note
    ///
    /// See: `WasmFeatures::legacy_exceptions` Note in `crates/wasmparser/src/features.rs`
    LegacyCatch,
    /// A Wasm legacy `catch_all` control block.
    ///
    /// # Note
    ///
    /// See: `WasmFeatures::legacy_exceptions` Note in `crates/wasmparser/src/features.rs`
    LegacyCatchAll,
}

/// Represents a memory immediate in a WebAssembly memory instruction.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl PartialEq<Self> for BrTable<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cnt == other.cnt
            && self.default == other.default
            && self.reader.remaining_buffer() == other.reader.remaining_buffer()
    }
}

impl Eq for BrTable<'_> {}

impl<'a> BrTable<'a> {
    /// Returns the number of `br_table` entries, not including the default
    /// label
    pub fn len(&self) -> u32 {
        self.cnt
    }

    /// Returns whether `BrTable` doesn't have any labels apart from the default one.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the default target of this `br_table` instruction.
    pub fn default(&self) -> u32 {
        self.default
    }

    /// Returns the list of targets that this `br_table` instruction will be
    /// jumping to.
    ///
    /// This method will return an iterator which parses each target of this
    /// `br_table` except the default target. The returned iterator will
    /// yield `self.len()` elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wasmparser::{BinaryReader, OperatorsReader, Operator};
    ///
    /// let buf = [0x0e, 0x02, 0x01, 0x02, 0x00];
    /// let mut reader = OperatorsReader::new(BinaryReader::new(&buf, 0));
    /// let op = reader.read().unwrap();
    /// if let Operator::BrTable { targets } = op {
    ///     let targets = targets.targets().collect::<Result<Vec<_>, _>>().unwrap();
    ///     assert_eq!(targets, [1, 2]);
    /// }
    /// ```
    pub fn targets(&self) -> BrTableTargets<'_> {
        BrTableTargets {
            reader: self.reader.clone(),
            remaining: self.cnt,
        }
    }
}

/// An iterator over the targets of a [`BrTable`].
///
/// # Note
///
/// This iterator parses each target of the underlying `br_table`
/// except for the default target.
/// The iterator will yield exactly as many targets as the `br_table` has.
pub struct BrTableTargets<'a> {
    reader: crate::BinaryReader<'a>,
    remaining: u32,
}

impl<'a> Iterator for BrTableTargets<'a> {
    type Item = Result<u32>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = usize::try_from(self.remaining).unwrap_or_else(|error| {
            panic!("could not convert remaining `u32` into `usize`: {error}")
        });
        (remaining, Some(remaining))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            if !self.reader.eof() {
                return Some(Err(BinaryReaderError::new(
                    "trailing data in br_table",
                    self.reader.original_position(),
                )));
            }
            return None;
        }
        self.remaining -= 1;
        Some(self.reader.read_var_u32())
    }
}

impl fmt::Debug for BrTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("BrTable");
        f.field("count", &self.cnt);
        f.field("default", &self.default);
        match self.targets().collect::<Result<Vec<_>>>() {
            Ok(targets) => {
                f.field("targets", &targets);
            }
            Err(_) => {
                f.field("reader", &self.reader);
            }
        }
        f.finish()
    }
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

impl From<f32> for Ieee32 {
    fn from(value: f32) -> Self {
        Ieee32 {
            0: u32::from_le_bytes(value.to_le_bytes()),
        }
    }
}

impl From<Ieee32> for f32 {
    fn from(bits: Ieee32) -> f32 {
        f32::from_bits(bits.bits())
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

impl From<f64> for Ieee64 {
    fn from(value: f64) -> Self {
        Ieee64 {
            0: u64::from_le_bytes(value.to_le_bytes()),
        }
    }
}

impl From<Ieee64> for f64 {
    fn from(bits: Ieee64) -> f64 {
        f64::from_bits(bits.bits())
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

impl From<V128> for i128 {
    fn from(bits: V128) -> i128 {
        bits.i128()
    }
}

impl From<V128> for u128 {
    fn from(bits: V128) -> u128 {
        u128::from_le_bytes(bits.0)
    }
}

/// Represents the memory ordering for atomic instructions.
///
/// For an in-depth explanation of memory orderings, see the C++ documentation
/// for [`memory_order`] or the Rust documentation for [`atomic::Ordering`].
///
/// [`memory_order`]: https://en.cppreference.com/w/cpp/atomic/memory_order
/// [`atomic::Ordering`]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Ordering {
    /// For a load, it acquires; this orders all operations before the last
    /// "releasing" store. For a store, it releases; this orders all operations
    /// before it at the next "acquiring" load.
    AcqRel,
    /// Like `AcqRel` but all threads see all sequentially consistent operations
    /// in the same order.
    SeqCst,
}

macro_rules! define_operator {
    ($(@$proposal:ident $op:ident $({ $($payload:tt)* })? => $visit:ident ($($ann:tt)*))*) => {
        /// Instructions as defined [here].
        ///
        /// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
        #[derive(Debug, Clone, Eq, PartialEq)]
        #[allow(missing_docs)]
        #[non_exhaustive]
        pub enum Operator<'a> {
            $(
                $op $({ $($payload)* })?,
            )*
        }
    }
}
crate::for_each_operator!(define_operator);

/// A trait representing the stack of frames within a function.
///
/// The [`BinaryReader::visit_operator`] and
/// [`OperatorsReaders`](crate::OperatorsReader) type use
/// information about the current frame kind to enforce the syntactic
/// requirements of the binary format.
pub trait FrameStack {
    /// The current frame kind.
    fn current_frame(&self) -> Option<FrameKind>;
}

/// The Wasm control stack for the [`OperatorsReader`].
#[derive(Debug, Default, Clone)]
pub struct ControlStack {
    /// All frames on the control stack exclusing the top-most frame.
    frames: Vec<FrameKind>,
    /// The top-most frame on the control stack if any.
    top: Option<FrameKind>,
}

impl ControlStack {
    /// Resets `self` but keeps heap allocations.
    pub fn clear(&mut self) {
        self.frames.clear();
        self.top = None;
    }

    /// Returns `true` if `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    /// Pushes the `frame` to `self`.
    #[inline]
    pub fn push(&mut self, frame: FrameKind) {
        if let Some(old_top) = self.top.replace(frame) {
            self.frames.push(old_top);
        }
    }

    /// Pops the top-most [`FrameKind`] from `self`.
    pub fn pop(&mut self) -> Option<FrameKind> {
        mem::replace(&mut self.top, self.frames.pop())
    }

    /// Returns the top-mot [`FrameKind`].
    #[inline]
    pub fn last(&self) -> Option<FrameKind> {
        self.top
    }
}

/// Adapters from VisitOperators to FrameStacks
struct FrameStackAdapter<'a, T> {
    stack: &'a mut ControlStack,
    visitor: &'a mut T,
}

impl<T> FrameStack for FrameStackAdapter<'_, T> {
    fn current_frame(&self) -> Option<FrameKind> {
        self.stack.last()
    }
}

struct SingleFrameAdapter<'a, T> {
    current_frame: FrameKind,
    visitor: &'a mut T,
}

impl<T> FrameStack for SingleFrameAdapter<'_, T> {
    fn current_frame(&self) -> Option<FrameKind> {
        Some(self.current_frame)
    }
}

/// A reader for a core WebAssembly function's operators. The [`OperatorsReader`] internally
/// maintains a stack of the kinds of frames within an expression or function body.
/// This is necessary to enforce the syntactic requirements of the binary format.
/// The BinaryReader can also be used to read the operators by providing an external [`FrameStack`] instance.
#[derive(Clone)]
pub struct OperatorsReader<'a> {
    reader: BinaryReader<'a>,
    stack: ControlStack,
}

/// External handle to the internal allocations used by the OperatorsReader
///
/// This is created with either the `Default` implementation or with
/// [`OperatorsReader::into_allocations`]. It is then passed as an argument to
/// [`OperatorsReader::new`] to provide a means of reusing allocations
/// between each expression or function body.
#[derive(Default)]
pub struct OperatorsReaderAllocations(ControlStack);

impl<'a> OperatorsReader<'a> {
    /// Creates a new reader for an expression (instruction sequence).
    ///
    /// This method, in conjunction with [`OperatorsReader::into_allocations`],
    /// provides a means to reuse allocations across reading each
    /// individual expression or function body. Note that it is also sufficient
    /// to call this method with `Default::default()` if no prior allocations are
    /// available.
    pub fn new(reader: BinaryReader<'a>) -> Self {
        Self::new_with_allocs(reader, Default::default())
    }

    /// Same as [`OperatorsReader::new`] except that the
    /// [`OperatorsReaderAllocations`] can be specified here to amortize the
    /// cost of them over multiple readers.
    pub fn new_with_allocs(
        reader: BinaryReader<'a>,
        mut allocs: OperatorsReaderAllocations,
    ) -> Self {
        allocs.0.clear();
        allocs.0.push(FrameKind::Block);
        Self {
            reader,
            stack: allocs.0,
        }
    }

    /// Get binary reader
    pub fn get_binary_reader(&self) -> BinaryReader<'a> {
        self.reader.clone()
    }

    /// Determines if the reader is at the end of the operators.
    pub fn eof(&self) -> bool {
        self.reader.eof()
    }

    /// Gets the original position of the reader.
    pub fn original_position(&self) -> usize {
        self.reader.original_position()
    }

    /// Returns whether there is an `end` opcode followed by eof remaining in
    /// this reader.
    pub fn is_end_then_eof(&self) -> bool {
        self.reader.is_end_then_eof()
    }

    /// Consumes this reader and returns the underlying allocations that
    /// were used to store the frame stack.
    ///
    /// The returned value here can be paired with
    /// [`OperatorsReader::new`] to reuse the allocations already
    /// created by this reader.
    pub fn into_allocations(self) -> OperatorsReaderAllocations {
        OperatorsReaderAllocations(self.stack)
    }

    /// Reads the next available `Operator`.
    ///
    /// # Errors
    ///
    /// If `OperatorsReader` has less bytes remaining than required to parse
    /// the `Operator`, or if the input is malformed.
    pub fn read(&mut self) -> Result<Operator<'a>> {
        self.visit_operator(&mut OperatorFactory)
    }

    /// Visit the next available operator with the specified [`VisitOperator`] instance.
    ///
    /// Note that this does not implicitly propagate any additional information such as instruction
    /// offsets. In order to do so, consider storing such data within the visitor before visiting.
    ///
    /// # Errors
    ///
    /// If `OperatorsReader` has less bytes remaining than required to parse the `Operator`,
    /// or if the input is malformed.
    ///
    /// # Examples
    ///
    /// Store an offset for use in diagnostics or any other purposes:
    ///
    /// ```
    /// # use wasmparser::{OperatorsReader, VisitOperator, Result, for_each_visit_operator};
    ///
    /// pub fn dump(mut reader: OperatorsReader) -> Result<()> {
    ///     let mut visitor = Dumper { offset: 0 };
    ///     while !reader.eof() {
    ///         visitor.offset = reader.original_position();
    ///         reader.visit_operator(&mut visitor)?;
    ///     }
    ///     Ok(())
    /// }
    ///
    /// struct Dumper {
    ///     offset: usize
    /// }
    ///
    /// macro_rules! define_visit_operator {
    ///     ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
    ///         $(
    ///             fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
    ///                 println!("{}: {}", self.offset, stringify!($visit));
    ///             }
    ///         )*
    ///     }
    /// }
    ///
    /// impl<'a> VisitOperator<'a> for Dumper {
    ///     type Output = ();
    ///     for_each_visit_operator!(define_visit_operator);
    /// }
    ///
    /// ```
    pub fn visit_operator<T>(&mut self, visitor: &mut T) -> Result<<T as VisitOperator<'a>>::Output>
    where
        T: VisitOperator<'a>,
    {
        self.reader.visit_operator(&mut FrameStackAdapter {
            stack: &mut self.stack,
            visitor,
        })
    }

    /// Reads an operator with its offset.
    pub fn read_with_offset(&mut self) -> Result<(Operator<'a>, usize)> {
        let pos = self.reader.original_position();
        Ok((self.read()?, pos))
    }

    /// Converts to an iterator of operators paired with offsets.
    pub fn into_iter_with_offsets(self) -> OperatorsIteratorWithOffsets<'a> {
        OperatorsIteratorWithOffsets {
            reader: self,
            err: false,
        }
    }

    pub(crate) fn skip_const_expr(&mut self) -> Result<()> {
        // TODO add skip_operator() method and/or validate ConstExpr operators.
        loop {
            if let Operator::End = self.read()? {
                if self.current_frame().is_some() {
                    bail!(
                        self.original_position(),
                        "control frames remain at end of expression"
                    );
                }
                return Ok(());
            }
        }
    }

    /// Function that must be called after the last opcode has been processed.
    ///
    /// This function returns an error if there is extra data after the operators.
    /// It does *not* check the binary format requirement that if the data count
    /// section is absent, a data index may not occur in the code section.
    pub fn finish(&self) -> Result<()> {
        self.reader.finish_expression(self)
    }
}

impl<'a> FrameStack for OperatorsReader<'a> {
    fn current_frame(&self) -> Option<FrameKind> {
        self.stack.last()
    }
}

impl<'a> IntoIterator for OperatorsReader<'a> {
    type Item = Result<Operator<'a>>;
    type IntoIter = OperatorsIterator<'a>;

    /// Reads content of the code section.
    ///
    /// # Examples
    /// ```
    /// # use wasmparser::{Operator, CodeSectionReader, Result, BinaryReader};
    /// # let data: &[u8] = &[
    /// #     0x01, 0x03, 0x00, 0x01, 0x0b];
    /// let reader = BinaryReader::new(data, 0);
    /// let code_reader = CodeSectionReader::new(reader).unwrap();
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

impl<'a> OperatorsIterator<'a> {
    /// Consumes this iterator and returns the underlying allocations.
    /// See [`OperatorsReader::into_allocations`].
    pub fn into_allocations(self) -> OperatorsReaderAllocations {
        self.reader.into_allocations()
    }
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

impl<'a> OperatorsIteratorWithOffsets<'a> {
    /// Consumes this iterator and returns the underlying allocations.
    /// See [`OperatorsReader::into_allocations`].
    pub fn into_allocations(self) -> OperatorsReaderAllocations {
        self.reader.into_allocations()
    }
}

impl<'a> Iterator for OperatorsIteratorWithOffsets<'a> {
    type Item = Result<(Operator<'a>, usize)>;

    /// Reads content of the code section with offsets.
    ///
    /// # Examples
    /// ```
    /// use wasmparser::{Operator, CodeSectionReader, Result, BinaryReader};
    /// # let data: &[u8] = &[
    /// #     0x01, 0x03, 0x00, /* offset = 23 */ 0x01, 0x0b];
    /// let reader = BinaryReader::new(data, 20);
    /// let code_reader = CodeSectionReader::new(reader).unwrap();
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
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
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
            ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {{
                match op {
                    $( Operator::$op $({ $($arg),* })? => self.$visit($($($arg.clone()),*)?), )*
                    #[cfg(feature = "simd")]
                    other => visit_simd_operator(self, other),
                }
            }};
        }
        crate::for_each_visit_operator!(visit_operator)
    }

    /// Returns a mutable reference to a [`VisitSimdOperator`] visitor.
    ///
    /// - If an implementer does _not_ want to support Wasm `simd` proposal
    ///   nothing has to be done since the default implementation already suffices.
    /// - If an implementer _does_ want to support Wasm `simd` proposal this
    ///   method usually is implemented as `Some(self)` where the implementing
    ///   type (`Self`) typically also implements `VisitSimdOperator`.
    ///
    /// # Example
    ///
    /// ```
    /// # macro_rules! define_visit_operator {
    /// #     ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
    /// #         $( fn $visit(&mut self $($(,$arg: $argty)*)?) {} )*
    /// #     }
    /// # }
    /// # use wasmparser::{VisitOperator, VisitSimdOperator};
    /// pub struct MyVisitor;
    ///
    /// impl<'a> VisitOperator<'a> for MyVisitor {
    ///     type Output = ();
    ///
    ///     fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
    ///         Some(self)
    ///     }
    ///
    ///     // implement remaining visitation methods here ...
    ///     # wasmparser::for_each_visit_operator!(define_visit_operator);
    /// }
    ///
    /// impl VisitSimdOperator<'_> for MyVisitor {
    ///     // implement SIMD visitation methods here ...
    ///     # wasmparser::for_each_visit_simd_operator!(define_visit_operator);
    /// }
    /// ```
    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        None
    }

    crate::for_each_visit_operator!(define_visit_operator);
}

/// Special handler for visiting `simd` and `relaxed-simd` [`Operator`] variants.
#[cfg(feature = "simd")]
fn visit_simd_operator<'a, V>(visitor: &mut V, op: &Operator<'a>) -> V::Output
where
    V: VisitOperator<'a> + ?Sized,
{
    let Some(simd_visitor) = visitor.simd_visitor() else {
        panic!("missing SIMD visitor to visit operator: {op:?}")
    };
    macro_rules! visit_simd_operator {
        ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {{
            match op {
                $( Operator::$op $({ $($arg),* })? => simd_visitor.$visit($($($arg.clone()),*)?), )*
                unexpected => unreachable!("unexpected non-SIMD operator: {unexpected:?}"),
            }
        }};
    }
    crate::for_each_visit_simd_operator!(visit_simd_operator)
}

/// Trait implemented by types that can visit all Wasm `simd` and `relaxed-simd` [`Operator`]s.
#[cfg(feature = "simd")]
#[allow(missing_docs)]
pub trait VisitSimdOperator<'a>: VisitOperator<'a> {
    crate::for_each_visit_simd_operator!(define_visit_operator);
}

macro_rules! define_visit_operator_delegate {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
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
    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = V::Output>> {
        V::simd_visitor(*self)
    }
    crate::for_each_visit_operator!(define_visit_operator_delegate);
}

#[cfg(feature = "simd")]
impl<'a, 'b, V: VisitSimdOperator<'a> + ?Sized> VisitSimdOperator<'a> for &'b mut V {
    crate::for_each_visit_simd_operator!(define_visit_operator_delegate);
}

impl<'a, V: VisitOperator<'a> + ?Sized> VisitOperator<'a> for Box<V> {
    type Output = V::Output;
    fn visit_operator(&mut self, op: &Operator<'a>) -> Self::Output {
        V::visit_operator(&mut *self, op)
    }
    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = V::Output>> {
        V::simd_visitor(&mut *self)
    }
    crate::for_each_visit_operator!(define_visit_operator_delegate);
}

#[cfg(feature = "simd")]
impl<'a, V: VisitSimdOperator<'a> + ?Sized> VisitSimdOperator<'a> for Box<V> {
    crate::for_each_visit_simd_operator!(define_visit_operator_delegate);
}

/// A `try_table` entries representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TryTable {
    /// The block type describing the try block itself.
    pub ty: BlockType,
    /// Outer blocks which will receive exceptions.
    pub catches: Vec<Catch>,
}

/// Catch clauses that can be specified in [`TryTable`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

/// A representation of dispatch tables on `resume` and `resume_throw`
/// instructions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeTable {
    /// Either the outer blocks which will handle suspensions or
    /// "switch-to" handlers.
    pub handlers: Vec<Handle>,
}

/// Handle clauses that can be specified in [`ResumeTable`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Handle {
    /// Equivalent of `(on $tag $lbl)`.
    OnLabel { tag: u32, label: u32 },
    /// Equivalent of `(on $tag switch)`.
    OnSwitch { tag: u32 },
}

impl ResumeTable {
    /// Returns the number of entries in the table.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }
}

impl<'a> FromReader<'a> for ResumeTable {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        let handlers = reader
            .read_iter(MAX_WASM_HANDLERS, "resume table")?
            .collect::<Result<_>>()?;
        let table = ResumeTable { handlers };
        Ok(table)
    }
}

impl<'a> FromReader<'a> for Handle {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => Handle::OnLabel {
                tag: reader.read_var_u32()?,
                label: reader.read_var_u32()?,
            },
            0x01 => Handle::OnSwitch {
                tag: reader.read_var_u32()?,
            },
            x => return reader.invalid_leading_byte(x, "on clause"),
        })
    }
}

/// A factory to construct [`Operator`] instances via the [`VisitOperator`] trait.
struct OperatorFactory;

macro_rules! define_visit_operator {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Operator<'a> {
                Operator::$op $({ $($arg),* })?
            }
        )*
    }
}

impl<'a> VisitOperator<'a> for OperatorFactory {
    type Output = Operator<'a>;

    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        Some(self)
    }

    crate::for_each_visit_operator!(define_visit_operator);
}

#[cfg(feature = "simd")]
impl<'a> VisitSimdOperator<'a> for OperatorFactory {
    crate::for_each_visit_simd_operator!(define_visit_operator);
}

macro_rules! define_visit_operator_stack_adapter {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> T::Output {
                define_visit_operator_stack_adapter!(@visit self $visit $($($arg,)*)?)
            }
        )*
    };

    (@visit $self:ident visit_block $($rest:tt)*) => {{
	    $self.stack.push(FrameKind::Block);
	    $self.visitor.visit_block( $($rest)* )
    }};

    (@visit $self:ident visit_loop $($rest:tt)*) => {{
	    $self.stack.push(FrameKind::Loop);
	    $self.visitor.visit_loop( $($rest)* )
    }};

    (@visit $self:ident visit_if $($rest:tt)*) => {{
	    $self.stack.push(FrameKind::If);
	    $self.visitor.visit_if( $($rest)* )
    }};

    (@visit $self:ident visit_else $($rest:tt)*) => {{
	    $self.stack.pop();
	    $self.stack.push(FrameKind::Else);
	    $self.visitor.visit_else( $($rest)* )
    }};

    (@visit $self:ident visit_try $($rest:tt)*) => {{
	    $self.stack.push(FrameKind::LegacyTry);
	    $self.visitor.visit_try( $($rest)* )
    }};

    (@visit $self:ident visit_catch $($rest:tt)*) => {{
	    $self.stack.pop();
	    $self.stack.push(FrameKind::LegacyCatch);
	    $self.visitor.visit_catch( $($rest)* )
    }};

    (@visit $self:ident visit_catch_all $($rest:tt)*) => {{
	    $self.stack.pop();
	    $self.stack.push(FrameKind::LegacyCatchAll);
	    $self.visitor.visit_catch_all( $($rest)* )
    }};

    (@visit $self:ident visit_try_table $($rest:tt)*) => {{
	    $self.stack.push(FrameKind::TryTable);
	    $self.visitor.visit_try_table( $($rest)* )
    }};

    (@visit $self:ident visit_delegate $($rest:tt)*) => {{
	    $self.stack.pop();
	    $self.visitor.visit_delegate( $($rest)* )
    }};

    (@visit $self:ident visit_end $($rest:tt)*) => {{
	    $self.stack.pop();
	    $self.visitor.visit_end( $($rest)* )
    }};

    (@visit $self:ident $visit:ident $($rest:tt)*) => {
	$self.visitor.$visit( $($rest)* )
    };
}

impl<'a, T: VisitOperator<'a>> VisitOperator<'a> for FrameStackAdapter<'_, T> {
    type Output = T::Output;

    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        self.visitor.simd_visitor()
    }

    crate::for_each_visit_operator!(define_visit_operator_stack_adapter);
}

macro_rules! define_passthrough_visit_operator {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> T::Output {
		self.visitor.$visit( $($($arg,)*)? )
            }
        )*
    };
}

impl<'a, T: VisitOperator<'a>> VisitOperator<'a> for SingleFrameAdapter<'_, T> {
    type Output = T::Output;

    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        self.visitor.simd_visitor()
    }

    crate::for_each_visit_operator!(define_passthrough_visit_operator);
}

impl<'a> BinaryReader<'a> {
    /// Peeks at the next available `Operator`, given a borrowed `FrameStack`.
    ///
    /// # Errors
    ///
    /// If `BinaryReader` has less bytes remaining than required to parse
    /// the `Operator`, or if the input is malformed.
    pub fn peek_operator<T: FrameStack>(&self, stack: &T) -> Result<Operator<'a>> {
        self.clone().visit_operator(&mut SingleFrameAdapter {
            current_frame: stack.current_frame().ok_or_else(|| {
                format_err!(
                    self.original_position(),
                    "operators remaining after end of function body or expression"
                )
            })?,
            visitor: &mut OperatorFactory,
        })
    }
}
