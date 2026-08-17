/* Copyright 2019 Mozilla Foundation
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

// The basic validation algorithm here is copied from the "Validation
// Algorithm" section of the WebAssembly specification -
// https://webassembly.github.io/spec/core/appendix/algorithm.html.
//
// That algorithm is followed pretty closely here, namely `push_operand`,
// `pop_operand`, `push_ctrl`, and `pop_ctrl`. If anything here is a bit
// confusing it's recommended to read over that section to see how it maps to
// the various methods here.

use crate::{
    limits::MAX_WASM_FUNCTION_LOCALS, ArrayType, BinaryReaderError, BlockType, BrTable, Catch,
    CompositeType, FieldType, FuncType, HeapType, Ieee32, Ieee64, MemArg, RefType, Result,
    StorageType, StructType, SubType, TryTable, UnpackedIndex, ValType, VisitOperator,
    WasmFeatures, WasmModuleResources, V128,
};
use std::ops::{Deref, DerefMut};

pub(crate) struct OperatorValidator {
    pub(super) locals: Locals,
    pub(super) local_inits: Vec<bool>,

    // This is a list of flags for wasm features which are used to gate various
    // instructions.
    pub(crate) features: WasmFeatures,

    // Temporary storage used during `pop_push_label_types` and various
    // branching instructions.
    popped_types_tmp: Vec<MaybeType>,

    /// The `control` list is the list of blocks that we're currently in.
    control: Vec<Frame>,
    /// The `operands` is the current type stack.
    operands: Vec<MaybeType>,
    /// When local_inits is modified, the relevant index is recorded here to be
    /// undone when control pops
    inits: Vec<u32>,

    /// Offset of the `end` instruction which emptied the `control` stack, which
    /// must be the end of the function.
    end_which_emptied_control: Option<usize>,
}

// No science was performed in the creation of this number, feel free to change
// it if you so like.
const MAX_LOCALS_TO_TRACK: usize = 50;

pub(super) struct Locals {
    // Total number of locals in the function.
    num_locals: u32,

    // The first MAX_LOCALS_TO_TRACK locals in a function. This is used to
    // optimize the theoretically common case where most functions don't have
    // many locals and don't need a full binary search in the entire local space
    // below.
    first: Vec<ValType>,

    // This is a "compressed" list of locals for this function. The list of
    // locals are represented as a list of tuples. The second element is the
    // type of the local, and the first element is monotonically increasing as
    // you visit elements of this list. The first element is the maximum index
    // of the local, after the previous index, of the type specified.
    //
    // This allows us to do a binary search on the list for a local's index for
    // `local.{get,set,tee}`. We do a binary search for the index desired, and
    // it either lies in a "hole" where the maximum index is specified later,
    // or it's at the end of the list meaning it's out of bounds.
    all: Vec<(u32, ValType)>,
}

/// A Wasm control flow block on the control flow stack during Wasm validation.
//
// # Dev. Note
//
// This structure corresponds to `ctrl_frame` as specified at in the validation
// appendix of the wasm spec
#[derive(Debug, Copy, Clone)]
pub struct Frame {
    /// Indicator for what kind of instruction pushed this frame.
    pub kind: FrameKind,
    /// The type signature of this frame, represented as a singular return type
    /// or a type index pointing into the module's types.
    pub block_type: BlockType,
    /// The index, below which, this frame cannot modify the operand stack.
    pub height: usize,
    /// Whether this frame is unreachable so far.
    pub unreachable: bool,
    /// The number of initializations in the stack at the time of its creation
    pub init_height: usize,
}

/// The kind of a control flow [`Frame`].
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
}

struct OperatorValidatorTemp<'validator, 'resources, T> {
    offset: usize,
    inner: &'validator mut OperatorValidator,
    resources: &'resources T,
}

#[derive(Default)]
pub struct OperatorValidatorAllocations {
    popped_types_tmp: Vec<MaybeType>,
    control: Vec<Frame>,
    operands: Vec<MaybeType>,
    local_inits: Vec<bool>,
    inits: Vec<u32>,
    locals_first: Vec<ValType>,
    locals_all: Vec<(u32, ValType)>,
}

/// Type storage within the validator.
///
/// This is used to manage the operand stack and notably isn't just `ValType` to
/// handle unreachable code and the "bottom" type.
#[derive(Debug, Copy, Clone)]
enum MaybeType {
    Bot,
    HeapBot,
    Type(ValType),
}

// The validator is pretty performance-sensitive and `MaybeType` is the main
// unit of storage, so assert that it doesn't exceed 4 bytes which is the
// current expected size.
const _: () = {
    assert!(std::mem::size_of::<MaybeType>() == 4);
};

impl std::fmt::Display for MaybeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeType::Bot => write!(f, "bot"),
            MaybeType::HeapBot => write!(f, "heap-bot"),
            MaybeType::Type(ty) => std::fmt::Display::fmt(ty, f),
        }
    }
}

impl From<ValType> for MaybeType {
    fn from(ty: ValType) -> MaybeType {
        MaybeType::Type(ty)
    }
}

impl From<RefType> for MaybeType {
    fn from(ty: RefType) -> MaybeType {
        let ty: ValType = ty.into();
        ty.into()
    }
}

impl MaybeType {
    fn as_type(&self) -> Option<ValType> {
        match *self {
            Self::Type(ty) => Some(ty),
            Self::Bot | Self::HeapBot => None,
        }
    }
}

impl OperatorValidator {
    fn new(features: &WasmFeatures, allocs: OperatorValidatorAllocations) -> Self {
        let OperatorValidatorAllocations {
            popped_types_tmp,
            control,
            operands,
            local_inits,
            inits,
            locals_first,
            locals_all,
        } = allocs;
        debug_assert!(popped_types_tmp.is_empty());
        debug_assert!(control.is_empty());
        debug_assert!(operands.is_empty());
        debug_assert!(local_inits.is_empty());
        debug_assert!(inits.is_empty());
        debug_assert!(locals_first.is_empty());
        debug_assert!(locals_all.is_empty());
        OperatorValidator {
            locals: Locals {
                num_locals: 0,
                first: locals_first,
                all: locals_all,
            },
            local_inits,
            inits,
            features: *features,
            popped_types_tmp,
            operands,
            control,
            end_which_emptied_control: None,
        }
    }

    /// Creates a new operator validator which will be used to validate a
    /// function whose type is the `ty` index specified.
    ///
    /// The `resources` are used to learn about the function type underlying
    /// `ty`.
    pub fn new_func<T>(
        ty: u32,
        offset: usize,
        features: &WasmFeatures,
        resources: &T,
        allocs: OperatorValidatorAllocations,
    ) -> Result<Self>
    where
        T: WasmModuleResources,
    {
        let mut ret = OperatorValidator::new(features, allocs);
        ret.control.push(Frame {
            kind: FrameKind::Block,
            block_type: BlockType::FuncType(ty),
            height: 0,
            unreachable: false,
            init_height: 0,
        });
        let params = OperatorValidatorTemp {
            // This offset is used by the `func_type_at` and `inputs`.
            offset,
            inner: &mut ret,
            resources,
        }
        .func_type_at(ty)?
        .params();
        for ty in params {
            ret.locals.define(1, *ty);
            ret.local_inits.push(true);
        }
        Ok(ret)
    }

    /// Creates a new operator validator which will be used to validate an
    /// `init_expr` constant expression which should result in the `ty`
    /// specified.
    pub fn new_const_expr(
        features: &WasmFeatures,
        ty: ValType,
        allocs: OperatorValidatorAllocations,
    ) -> Self {
        let mut ret = OperatorValidator::new(features, allocs);
        ret.control.push(Frame {
            kind: FrameKind::Block,
            block_type: BlockType::Type(ty),
            height: 0,
            unreachable: false,
            init_height: 0,
        });
        ret
    }

    pub fn define_locals(
        &mut self,
        offset: usize,
        count: u32,
        mut ty: ValType,
        resources: &impl WasmModuleResources,
    ) -> Result<()> {
        resources.check_value_type(&mut ty, &self.features, offset)?;
        if count == 0 {
            return Ok(());
        }
        if !self.locals.define(count, ty) {
            return Err(BinaryReaderError::new(
                "too many locals: locals exceed maximum",
                offset,
            ));
        }
        self.local_inits
            .resize(self.local_inits.len() + count as usize, ty.is_defaultable());
        Ok(())
    }

    /// Returns the current operands stack height.
    pub fn operand_stack_height(&self) -> usize {
        self.operands.len()
    }

    /// Returns the optional value type of the value operand at the given
    /// `depth` from the top of the operand stack.
    ///
    /// - Returns `None` if the `depth` is out of bounds.
    /// - Returns `Some(None)` if there is a value with unknown type
    /// at the given `depth`.
    ///
    /// # Note
    ///
    /// A `depth` of 0 will refer to the last operand on the stack.
    pub fn peek_operand_at(&self, depth: usize) -> Option<Option<ValType>> {
        Some(match self.operands.iter().rev().nth(depth)? {
            MaybeType::Type(t) => Some(*t),
            MaybeType::Bot | MaybeType::HeapBot => None,
        })
    }

    /// Returns the number of frames on the control flow stack.
    pub fn control_stack_height(&self) -> usize {
        self.control.len()
    }

    pub fn get_frame(&self, depth: usize) -> Option<&Frame> {
        self.control.iter().rev().nth(depth)
    }

    /// Create a temporary [`OperatorValidatorTemp`] for validation.
    pub fn with_resources<'a, 'validator, 'resources, T>(
        &'validator mut self,
        resources: &'resources T,
        offset: usize,
    ) -> impl VisitOperator<'a, Output = Result<()>> + 'validator
    where
        T: WasmModuleResources,
        'resources: 'validator,
    {
        WasmProposalValidator(OperatorValidatorTemp {
            offset,
            inner: self,
            resources,
        })
    }

    pub fn finish(&mut self, offset: usize) -> Result<()> {
        if self.control.last().is_some() {
            bail!(
                offset,
                "control frames remain at end of function: END opcode expected"
            );
        }

        // The `end` opcode is one byte which means that the `offset` here
        // should point just beyond the `end` opcode which emptied the control
        // stack. If not that means more instructions were present after the
        // control stack was emptied.
        if offset != self.end_which_emptied_control.unwrap() + 1 {
            return Err(self.err_beyond_end(offset));
        }
        Ok(())
    }

    fn err_beyond_end(&self, offset: usize) -> BinaryReaderError {
        format_err!(offset, "operators remaining after end of function")
    }

    pub fn into_allocations(self) -> OperatorValidatorAllocations {
        fn clear<T>(mut tmp: Vec<T>) -> Vec<T> {
            tmp.clear();
            tmp
        }
        OperatorValidatorAllocations {
            popped_types_tmp: clear(self.popped_types_tmp),
            control: clear(self.control),
            operands: clear(self.operands),
            local_inits: clear(self.local_inits),
            inits: clear(self.inits),
            locals_first: clear(self.locals.first),
            locals_all: clear(self.locals.all),
        }
    }
}

impl<R> Deref for OperatorValidatorTemp<'_, '_, R> {
    type Target = OperatorValidator;
    fn deref(&self) -> &OperatorValidator {
        self.inner
    }
}

impl<R> DerefMut for OperatorValidatorTemp<'_, '_, R> {
    fn deref_mut(&mut self) -> &mut OperatorValidator {
        self.inner
    }
}

impl<'resources, R> OperatorValidatorTemp<'_, 'resources, R>
where
    R: WasmModuleResources,
{
    /// Pushes a type onto the operand stack.
    ///
    /// This is used by instructions to represent a value that is pushed to the
    /// operand stack. This can fail, but only if `Type` is feature gated.
    /// Otherwise the push operation always succeeds.
    fn push_operand<T>(&mut self, ty: T) -> Result<()>
    where
        T: Into<MaybeType>,
    {
        let maybe_ty = ty.into();

        if cfg!(debug_assertions) {
            match maybe_ty {
                MaybeType::Type(ValType::Ref(r)) => match r.heap_type() {
                    HeapType::Concrete(index) => {
                        debug_assert!(
                            matches!(index, UnpackedIndex::Id(_)),
                            "only ref types referencing `CoreTypeId`s can \
                             be pushed to the operand stack"
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        self.operands.push(maybe_ty);
        Ok(())
    }

    fn push_concrete_ref(&mut self, nullable: bool, type_index: u32) -> Result<()> {
        let mut heap_ty = HeapType::Concrete(UnpackedIndex::Module(type_index));

        // Canonicalize the module index into an id.
        self.resources.check_heap_type(&mut heap_ty, self.offset)?;
        debug_assert!(matches!(heap_ty, HeapType::Concrete(UnpackedIndex::Id(_))));

        let ref_ty = RefType::new(nullable, heap_ty).ok_or_else(|| {
            format_err!(self.offset, "implementation limit: type index too large")
        })?;

        self.push_operand(ref_ty)
    }

    fn pop_concrete_ref(&mut self, nullable: bool, type_index: u32) -> Result<MaybeType> {
        let mut heap_ty = HeapType::Concrete(UnpackedIndex::Module(type_index));

        // Canonicalize the module index into an id.
        self.resources.check_heap_type(&mut heap_ty, self.offset)?;
        debug_assert!(matches!(heap_ty, HeapType::Concrete(UnpackedIndex::Id(_))));

        let ref_ty = RefType::new(nullable, heap_ty).ok_or_else(|| {
            format_err!(self.offset, "implementation limit: type index too large")
        })?;

        self.pop_operand(Some(ref_ty.into()))
    }

    /// Pop the given label types, checking that they are indeed present on the
    /// stack, and then push them back on again.
    fn pop_push_label_types(
        &mut self,
        label_types: impl PreciseIterator<Item = ValType>,
    ) -> Result<()> {
        for ty in label_types.clone().rev() {
            self.pop_operand(Some(ty))?;
        }
        for ty in label_types {
            self.push_operand(ty)?;
        }
        Ok(())
    }

    /// Attempts to pop a type from the operand stack.
    ///
    /// This function is used to remove types from the operand stack. The
    /// `expected` argument can be used to indicate that a type is required, or
    /// simply that something is needed to be popped.
    ///
    /// If `expected` is `Some(T)` then this will be guaranteed to return
    /// `T`, and it will only return success if the current block is
    /// unreachable or if `T` was found at the top of the operand stack.
    ///
    /// If `expected` is `None` then it indicates that something must be on the
    /// operand stack, but it doesn't matter what's on the operand stack. This
    /// is useful for polymorphic instructions like `select`.
    ///
    /// If `Some(T)` is returned then `T` was popped from the operand stack and
    /// matches `expected`. If `None` is returned then it means that `None` was
    /// expected and a type was successfully popped, but its exact type is
    /// indeterminate because the current block is unreachable.
    fn pop_operand(&mut self, expected: Option<ValType>) -> Result<MaybeType> {
        // This method is one of the hottest methods in the validator so to
        // improve codegen this method contains a fast-path success case where
        // if the top operand on the stack is as expected it's returned
        // immediately. This is the most common case where the stack will indeed
        // have the expected type and all we need to do is pop it off.
        //
        // Note that this still has to be careful to be correct, though. For
        // efficiency an operand is unconditionally popped and on success it is
        // matched against the state of the world to see if we could actually
        // pop it. If we shouldn't have popped it then it's passed to the slow
        // path to get pushed back onto the stack.
        let popped = match self.operands.pop() {
            Some(MaybeType::Type(actual_ty)) => {
                if Some(actual_ty) == expected {
                    if let Some(control) = self.control.last() {
                        if self.operands.len() >= control.height {
                            return Ok(MaybeType::Type(actual_ty));
                        }
                    }
                }
                Some(MaybeType::Type(actual_ty))
            }
            other => other,
        };

        self._pop_operand(expected, popped)
    }

    // This is the "real" implementation of `pop_operand` which is 100%
    // spec-compliant with little attention paid to efficiency since this is the
    // slow-path from the actual `pop_operand` function above.
    #[cold]
    fn _pop_operand(
        &mut self,
        expected: Option<ValType>,
        popped: Option<MaybeType>,
    ) -> Result<MaybeType> {
        self.operands.extend(popped);
        let control = match self.control.last() {
            Some(c) => c,
            None => return Err(self.err_beyond_end(self.offset)),
        };
        let actual = if self.operands.len() == control.height && control.unreachable {
            MaybeType::Bot
        } else {
            if self.operands.len() == control.height {
                let desc = match expected {
                    Some(ty) => ty_to_str(ty),
                    None => "a type".into(),
                };
                bail!(
                    self.offset,
                    "type mismatch: expected {desc} but nothing on stack"
                )
            } else {
                self.operands.pop().unwrap()
            }
        };
        if let Some(expected) = expected {
            match (actual, expected) {
                // The bottom type matches all expectations
                (MaybeType::Bot, _)
                // The "heap bottom" type only matches other references types,
                // but not any integer types.
                | (MaybeType::HeapBot, ValType::Ref(_)) => {}

                // Use the `is_subtype` predicate to test if a found type matches
                // the expectation.
                (MaybeType::Type(actual), expected) => {
                    if !self.resources.is_subtype(actual, expected) {
                        bail!(
                            self.offset,
                            "type mismatch: expected {}, found {}",
                            ty_to_str(expected),
                            ty_to_str(actual)
                        );
                    }
                }

                // A "heap bottom" type cannot match any numeric types.
                (
                    MaybeType::HeapBot,
                    ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128,
                ) => {
                    bail!(
                        self.offset,
                        "type mismatch: expected {}, found heap type",
                        ty_to_str(expected)
                    )
                }
            }
        }
        Ok(actual)
    }

    fn pop_ref(&mut self) -> Result<Option<RefType>> {
        match self.pop_operand(None)? {
            MaybeType::Bot | MaybeType::HeapBot => Ok(None),
            MaybeType::Type(ValType::Ref(rt)) => Ok(Some(rt)),
            MaybeType::Type(ty) => bail!(
                self.offset,
                "type mismatch: expected ref but found {}",
                ty_to_str(ty)
            ),
        }
    }

    /// Fetches the type for the local at `idx`, returning an error if it's out
    /// of bounds.
    fn local(&self, idx: u32) -> Result<ValType> {
        match self.locals.get(idx) {
            Some(ty) => Ok(ty),
            None => bail!(
                self.offset,
                "unknown local {}: local index out of bounds",
                idx
            ),
        }
    }

    /// Flags the current control frame as unreachable, additionally truncating
    /// the currently active operand stack.
    fn unreachable(&mut self) -> Result<()> {
        let control = match self.control.last_mut() {
            Some(frame) => frame,
            None => return Err(self.err_beyond_end(self.offset)),
        };
        control.unreachable = true;
        let new_height = control.height;
        self.operands.truncate(new_height);
        Ok(())
    }

    /// Pushes a new frame onto the control stack.
    ///
    /// This operation is used when entering a new block such as an if, loop,
    /// or block itself. The `kind` of block is specified which indicates how
    /// breaks interact with this block's type. Additionally the type signature
    /// of the block is specified by `ty`.
    fn push_ctrl(&mut self, kind: FrameKind, ty: BlockType) -> Result<()> {
        // Push a new frame which has a snapshot of the height of the current
        // operand stack.
        let height = self.operands.len();
        let init_height = self.inits.len();
        self.control.push(Frame {
            kind,
            block_type: ty,
            height,
            unreachable: false,
            init_height,
        });
        // All of the parameters are now also available in this control frame,
        // so we push them here in order.
        for ty in self.params(ty)? {
            self.push_operand(ty)?;
        }
        Ok(())
    }

    /// Pops a frame from the control stack.
    ///
    /// This function is used when exiting a block and leaves a block scope.
    /// Internally this will validate that blocks have the correct result type.
    fn pop_ctrl(&mut self) -> Result<Frame> {
        // Read the expected type and expected height of the operand stack the
        // end of the frame.
        let frame = match self.control.last() {
            Some(f) => f,
            None => return Err(self.err_beyond_end(self.offset)),
        };
        let ty = frame.block_type;
        let height = frame.height;
        let init_height = frame.init_height;

        // reset_locals in the spec
        for init in self.inits.split_off(init_height) {
            self.local_inits[init as usize] = false;
        }

        // Pop all the result types, in reverse order, from the operand stack.
        // These types will, possibly, be transferred to the next frame.
        for ty in self.results(ty)?.rev() {
            self.pop_operand(Some(ty))?;
        }

        // Make sure that the operand stack has returned to is original
        // height...
        if self.operands.len() != height {
            bail!(
                self.offset,
                "type mismatch: values remaining on stack at end of block"
            );
        }

        // And then we can remove it!
        Ok(self.control.pop().unwrap())
    }

    /// Validates a relative jump to the `depth` specified.
    ///
    /// Returns the type signature of the block that we're jumping to as well
    /// as the kind of block if the jump is valid. Otherwise returns an error.
    fn jump(&self, depth: u32) -> Result<(BlockType, FrameKind)> {
        if self.control.is_empty() {
            return Err(self.err_beyond_end(self.offset));
        }
        match (self.control.len() - 1).checked_sub(depth as usize) {
            Some(i) => {
                let frame = &self.control[i];
                Ok((frame.block_type, frame.kind))
            }
            None => bail!(self.offset, "unknown label: branch depth too large"),
        }
    }

    /// Validates that `memory_index` is valid in this module, and returns the
    /// type of address used to index the memory specified.
    fn check_memory_index(&self, memory_index: u32) -> Result<ValType> {
        match self.resources.memory_at(memory_index) {
            Some(mem) => Ok(mem.index_type()),
            None => bail!(self.offset, "unknown memory {}", memory_index),
        }
    }

    /// Validates a `memarg for alignment and such (also the memory it
    /// references), and returns the type of index used to address the memory.
    fn check_memarg(&self, memarg: MemArg) -> Result<ValType> {
        let index_ty = self.check_memory_index(memarg.memory)?;
        if memarg.align > memarg.max_align {
            bail!(self.offset, "alignment must not be larger than natural");
        }
        if index_ty == ValType::I32 && memarg.offset > u64::from(u32::MAX) {
            bail!(self.offset, "offset out of range: must be <= 2**32");
        }
        Ok(index_ty)
    }

    fn check_floats_enabled(&self) -> Result<()> {
        if !self.features.floats {
            bail!(self.offset, "floating-point instruction disallowed");
        }
        Ok(())
    }

    fn check_shared_memarg(&self, memarg: MemArg) -> Result<ValType> {
        if memarg.align != memarg.max_align {
            bail!(
                self.offset,
                "atomic instructions must always specify maximum alignment"
            );
        }
        self.check_memory_index(memarg.memory)
    }

    fn check_simd_lane_index(&self, index: u8, max: u8) -> Result<()> {
        if index >= max {
            bail!(self.offset, "SIMD index out of bounds");
        }
        Ok(())
    }

    /// Validates a block type, primarily with various in-flight proposals.
    fn check_block_type(&self, ty: &mut BlockType) -> Result<()> {
        match ty {
            BlockType::Empty => Ok(()),
            BlockType::Type(t) => self
                .resources
                .check_value_type(t, &self.features, self.offset),
            BlockType::FuncType(idx) => {
                if !self.features.multi_value {
                    bail!(
                        self.offset,
                        "blocks, loops, and ifs may only produce a resulttype \
                         when multi-value is not enabled",
                    );
                }
                self.func_type_at(*idx)?;
                Ok(())
            }
        }
    }

    /// Validates a `call` instruction, ensuring that the function index is
    /// in-bounds and the right types are on the stack to call the function.
    fn check_call(&mut self, function_index: u32) -> Result<()> {
        let ty = match self.resources.type_of_function(function_index) {
            Some(i) => i,
            None => {
                bail!(
                    self.offset,
                    "unknown function {function_index}: function index out of bounds",
                );
            }
        };
        self.check_call_ty(ty)
    }

    fn check_call_type_index(&mut self, type_index: u32) -> Result<()> {
        let ty = self.func_type_at(type_index)?;
        self.check_call_ty(ty)
    }

    fn check_call_ty(&mut self, ty: &FuncType) -> Result<()> {
        for &ty in ty.params().iter().rev() {
            debug_assert_type_indices_are_ids(ty);
            self.pop_operand(Some(ty))?;
        }
        for &ty in ty.results() {
            debug_assert_type_indices_are_ids(ty);
            self.push_operand(ty)?;
        }
        Ok(())
    }

    /// Validates a call to an indirect function, very similar to `check_call`.
    fn check_call_indirect(&mut self, index: u32, table_index: u32) -> Result<()> {
        match self.resources.table_at(table_index) {
            None => {
                bail!(self.offset, "unknown table: table index out of bounds");
            }
            Some(tab) => {
                if !self
                    .resources
                    .is_subtype(ValType::Ref(tab.element_type), ValType::FUNCREF)
                {
                    bail!(
                        self.offset,
                        "indirect calls must go through a table with type <= funcref",
                    );
                }
            }
        }
        let ty = self.func_type_at(index)?;
        self.pop_operand(Some(ValType::I32))?;
        for ty in ty.clone().params().iter().rev() {
            self.pop_operand(Some(*ty))?;
        }
        for ty in ty.results() {
            self.push_operand(*ty)?;
        }
        Ok(())
    }

    /// Validates a `return` instruction, popping types from the operand
    /// stack that the function needs.
    fn check_return(&mut self) -> Result<()> {
        if self.control.is_empty() {
            return Err(self.err_beyond_end(self.offset));
        }
        for ty in self.results(self.control[0].block_type)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.unreachable()?;
        Ok(())
    }

    /// Checks the validity of a common comparison operator.
    fn check_cmp_op(&mut self, ty: ValType) -> Result<()> {
        self.pop_operand(Some(ty))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }

    /// Checks the validity of a common float comparison operator.
    fn check_fcmp_op(&mut self, ty: ValType) -> Result<()> {
        debug_assert!(matches!(ty, ValType::F32 | ValType::F64));
        self.check_floats_enabled()?;
        self.check_cmp_op(ty)
    }

    /// Checks the validity of a common unary operator.
    fn check_unary_op(&mut self, ty: ValType) -> Result<()> {
        self.pop_operand(Some(ty))?;
        self.push_operand(ty)?;
        Ok(())
    }

    /// Checks the validity of a common unary float operator.
    fn check_funary_op(&mut self, ty: ValType) -> Result<()> {
        debug_assert!(matches!(ty, ValType::F32 | ValType::F64));
        self.check_floats_enabled()?;
        self.check_unary_op(ty)
    }

    /// Checks the validity of a common conversion operator.
    fn check_conversion_op(&mut self, into: ValType, from: ValType) -> Result<()> {
        self.pop_operand(Some(from))?;
        self.push_operand(into)?;
        Ok(())
    }

    /// Checks the validity of a common conversion operator.
    fn check_fconversion_op(&mut self, into: ValType, from: ValType) -> Result<()> {
        debug_assert!(matches!(into, ValType::F32 | ValType::F64));
        self.check_floats_enabled()?;
        self.check_conversion_op(into, from)
    }

    /// Checks the validity of a common binary operator.
    fn check_binary_op(&mut self, ty: ValType) -> Result<()> {
        self.pop_operand(Some(ty))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ty)?;
        Ok(())
    }

    /// Checks the validity of a common binary float operator.
    fn check_fbinary_op(&mut self, ty: ValType) -> Result<()> {
        debug_assert!(matches!(ty, ValType::F32 | ValType::F64));
        self.check_floats_enabled()?;
        self.check_binary_op(ty)
    }

    /// Checks the validity of an atomic load operator.
    fn check_atomic_load(&mut self, memarg: MemArg, load_ty: ValType) -> Result<()> {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(load_ty)?;
        Ok(())
    }

    /// Checks the validity of an atomic store operator.
    fn check_atomic_store(&mut self, memarg: MemArg, store_ty: ValType) -> Result<()> {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(store_ty))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }

    /// Checks the validity of a common atomic binary operator.
    fn check_atomic_binary_op(&mut self, memarg: MemArg, op_ty: ValType) -> Result<()> {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(op_ty))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(op_ty)?;
        Ok(())
    }

    /// Checks the validity of an atomic compare exchange operator.
    fn check_atomic_binary_cmpxchg(&mut self, memarg: MemArg, op_ty: ValType) -> Result<()> {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(op_ty))?;
        self.pop_operand(Some(op_ty))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(op_ty)?;
        Ok(())
    }

    /// Checks a [`V128`] splat operator.
    fn check_v128_splat(&mut self, src_ty: ValType) -> Result<()> {
        self.pop_operand(Some(src_ty))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }

    /// Checks a [`V128`] binary operator.
    fn check_v128_binary_op(&mut self) -> Result<()> {
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }

    /// Checks a [`V128`] binary float operator.
    fn check_v128_fbinary_op(&mut self) -> Result<()> {
        self.check_floats_enabled()?;
        self.check_v128_binary_op()
    }

    /// Checks a [`V128`] binary operator.
    fn check_v128_unary_op(&mut self) -> Result<()> {
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }

    /// Checks a [`V128`] binary operator.
    fn check_v128_funary_op(&mut self) -> Result<()> {
        self.check_floats_enabled()?;
        self.check_v128_unary_op()
    }

    /// Checks a [`V128`] relaxed ternary operator.
    fn check_v128_ternary_op(&mut self) -> Result<()> {
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }

    /// Checks a [`V128`] relaxed ternary operator.
    fn check_v128_bitmask_op(&mut self) -> Result<()> {
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }

    /// Checks a [`V128`] relaxed ternary operator.
    fn check_v128_shift_op(&mut self) -> Result<()> {
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }

    /// Checks a [`V128`] common load operator.
    fn check_v128_load_op(&mut self, memarg: MemArg) -> Result<()> {
        let idx = self.check_memarg(memarg)?;
        self.pop_operand(Some(idx))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }

    /// Common helper for `ref.test` and `ref.cast` downcasting/checking
    /// instructions. Returns the given `heap_type` as a `ValType`.
    fn check_downcast(
        &mut self,
        nullable: bool,
        mut heap_type: HeapType,
        inst_name: &str,
    ) -> Result<ValType> {
        self.resources
            .check_heap_type(&mut heap_type, self.offset)?;

        let sub_ty = RefType::new(nullable, heap_type)
            .map(ValType::from)
            .ok_or_else(|| {
                BinaryReaderError::new("implementation limit: type index too large", self.offset)
            })?;

        let sup_ty = self.pop_ref()?.unwrap_or_else(|| {
            sub_ty
                .as_reference_type()
                .expect("we created this as a reference just above")
        });
        let sup_ty = RefType::new(true, self.resources.top_type(&sup_ty.heap_type()))
            .expect("can't panic with non-concrete heap types");

        if !self.resources.is_subtype(sub_ty, sup_ty.into()) {
            bail!(
                self.offset,
                "{inst_name}'s heap type must be a sub type of the type on the stack: \
                 {sub_ty} is not a sub type of {sup_ty}"
            );
        }

        Ok(sub_ty)
    }

    /// Common helper for both nullable and non-nullable variants of `ref.test`
    /// instructions.
    fn check_ref_test(&mut self, nullable: bool, heap_type: HeapType) -> Result<()> {
        self.check_downcast(nullable, heap_type, "ref.test")?;
        self.push_operand(ValType::I32)
    }

    /// Common helper for both nullable and non-nullable variants of `ref.cast`
    /// instructions.
    fn check_ref_cast(&mut self, nullable: bool, heap_type: HeapType) -> Result<()> {
        let sub_ty = self.check_downcast(nullable, heap_type, "ref.cast")?;
        self.push_operand(sub_ty)
    }

    fn element_type_at(&self, elem_index: u32) -> Result<RefType> {
        match self.resources.element_type_at(elem_index) {
            Some(ty) => Ok(ty),
            None => bail!(
                self.offset,
                "unknown elem segment {}: segment index out of bounds",
                elem_index
            ),
        }
    }

    fn sub_type_at(&self, at: u32) -> Result<&'resources SubType> {
        self.resources
            .sub_type_at(at)
            .ok_or_else(|| format_err!(self.offset, "unknown type: type index out of bounds"))
    }

    fn struct_type_at(&self, at: u32) -> Result<&'resources StructType> {
        let sub_ty = self.sub_type_at(at)?;
        if let CompositeType::Struct(struct_ty) = &sub_ty.composite_type {
            Ok(struct_ty)
        } else {
            bail!(
                self.offset,
                "expected struct type at index {at}, found {sub_ty}"
            )
        }
    }

    fn struct_field_at(&self, struct_type_index: u32, field_index: u32) -> Result<FieldType> {
        let field_index = usize::try_from(field_index).map_err(|_| {
            BinaryReaderError::new("unknown field: field index out of bounds", self.offset)
        })?;
        self.struct_type_at(struct_type_index)?
            .fields
            .get(field_index)
            .copied()
            .ok_or_else(|| {
                BinaryReaderError::new("unknown field: field index out of bounds", self.offset)
            })
    }

    fn array_type_at(&self, at: u32) -> Result<&'resources ArrayType> {
        let sub_ty = self.sub_type_at(at)?;
        if let CompositeType::Array(array_ty) = &sub_ty.composite_type {
            Ok(array_ty)
        } else {
            bail!(
                self.offset,
                "expected array type at index {at}, found {sub_ty}"
            )
        }
    }

    fn func_type_at(&self, at: u32) -> Result<&'resources FuncType> {
        let sub_ty = self.sub_type_at(at)?;
        if let CompositeType::Func(func_ty) = &sub_ty.composite_type {
            Ok(func_ty)
        } else {
            bail!(
                self.offset,
                "expected func type at index {at}, found {sub_ty}"
            )
        }
    }

    fn tag_at(&self, at: u32) -> Result<&'resources FuncType> {
        self.resources
            .tag_at(at)
            .ok_or_else(|| format_err!(self.offset, "unknown tag {}: tag index out of bounds", at))
    }

    fn params(&self, ty: BlockType) -> Result<impl PreciseIterator<Item = ValType> + 'resources> {
        Ok(match ty {
            BlockType::Empty | BlockType::Type(_) => Either::B(None.into_iter()),
            BlockType::FuncType(t) => Either::A(self.func_type_at(t)?.params().iter().copied()),
        })
    }

    fn results(&self, ty: BlockType) -> Result<impl PreciseIterator<Item = ValType> + 'resources> {
        Ok(match ty {
            BlockType::Empty => Either::B(None.into_iter()),
            BlockType::Type(t) => Either::B(Some(t).into_iter()),
            BlockType::FuncType(t) => Either::A(self.func_type_at(t)?.results().iter().copied()),
        })
    }

    fn label_types(
        &self,
        ty: BlockType,
        kind: FrameKind,
    ) -> Result<impl PreciseIterator<Item = ValType> + 'resources> {
        Ok(match kind {
            FrameKind::Loop => Either::A(self.params(ty)?),
            _ => Either::B(self.results(ty)?),
        })
    }
}

pub fn ty_to_str(ty: ValType) -> &'static str {
    match ty {
        ValType::I32 => "i32",
        ValType::I64 => "i64",
        ValType::F32 => "f32",
        ValType::F64 => "f64",
        ValType::V128 => "v128",
        ValType::Ref(r) => r.wat(),
    }
}

/// A wrapper "visitor" around the real operator validator internally which
/// exists to check that the required wasm feature is enabled to proceed with
/// validation.
///
/// This validator is macro-generated to ensure that the proposal listed in this
/// crate's macro matches the one that's validated here. Each instruction's
/// visit method validates the specified proposal is enabled and then delegates
/// to `OperatorValidatorTemp` to perform the actual opcode validation.
struct WasmProposalValidator<'validator, 'resources, T>(
    OperatorValidatorTemp<'validator, 'resources, T>,
);

impl<T> WasmProposalValidator<'_, '_, T> {
    fn check_enabled(&self, flag: bool, desc: &str) -> Result<()> {
        if flag {
            return Ok(());
        }
        bail!(self.0.offset, "{desc} support is not enabled");
    }
}

macro_rules! validate_proposal {
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Result<()> {
                validate_proposal!(validate self $proposal);
                self.0.$visit($( $($arg),* )?)
            }
        )*
    };

    (validate self mvp) => {};
    (validate $self:ident $proposal:ident) => {
        $self.check_enabled($self.0.features.$proposal, validate_proposal!(desc $proposal))?
    };

    (desc simd) => ("SIMD");
    (desc relaxed_simd) => ("relaxed SIMD");
    (desc threads) => ("threads");
    (desc saturating_float_to_int) => ("saturating float to int conversions");
    (desc reference_types) => ("reference types");
    (desc bulk_memory) => ("bulk memory");
    (desc sign_extension) => ("sign extension operations");
    (desc exceptions) => ("exceptions");
    (desc tail_call) => ("tail calls");
    (desc function_references) => ("function references");
    (desc memory_control) => ("memory control");
    (desc gc) => ("gc");
}

impl<'a, T> VisitOperator<'a> for WasmProposalValidator<'_, '_, T>
where
    T: WasmModuleResources,
{
    type Output = Result<()>;

    for_each_operator!(validate_proposal);
}

#[track_caller]
#[inline]
fn debug_assert_type_indices_are_ids(ty: ValType) {
    if cfg!(debug_assertions) {
        if let ValType::Ref(r) = ty {
            if let HeapType::Concrete(idx) = r.heap_type() {
                debug_assert!(
                    matches!(idx, UnpackedIndex::Id(_)),
                    "type reference should be a `CoreTypeId`, found {idx:?}"
                );
            }
        }
    }
}

impl<'a, T> VisitOperator<'a> for OperatorValidatorTemp<'_, '_, T>
where
    T: WasmModuleResources,
{
    type Output = Result<()>;

    fn visit_nop(&mut self) -> Self::Output {
        Ok(())
    }
    fn visit_unreachable(&mut self) -> Self::Output {
        self.unreachable()?;
        Ok(())
    }
    fn visit_block(&mut self, mut ty: BlockType) -> Self::Output {
        self.check_block_type(&mut ty)?;
        for ty in self.params(ty)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.push_ctrl(FrameKind::Block, ty)?;
        Ok(())
    }
    fn visit_loop(&mut self, mut ty: BlockType) -> Self::Output {
        self.check_block_type(&mut ty)?;
        for ty in self.params(ty)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.push_ctrl(FrameKind::Loop, ty)?;
        Ok(())
    }
    fn visit_if(&mut self, mut ty: BlockType) -> Self::Output {
        self.check_block_type(&mut ty)?;
        self.pop_operand(Some(ValType::I32))?;
        for ty in self.params(ty)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.push_ctrl(FrameKind::If, ty)?;
        Ok(())
    }
    fn visit_else(&mut self) -> Self::Output {
        let frame = self.pop_ctrl()?;
        if frame.kind != FrameKind::If {
            bail!(self.offset, "else found outside of an `if` block");
        }
        self.push_ctrl(FrameKind::Else, frame.block_type)?;
        Ok(())
    }
    fn visit_try_table(&mut self, mut ty: TryTable) -> Self::Output {
        self.check_block_type(&mut ty.ty)?;
        for ty in self.params(ty.ty)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        for catch in ty.catches {
            match catch {
                Catch::One { tag, label } => {
                    let tag = self.tag_at(tag)?;
                    let (ty, kind) = self.jump(label)?;
                    let params = tag.params();
                    let types = self.label_types(ty, kind)?;
                    if params.len() != types.len() {
                        bail!(
                            self.offset,
                            "type mismatch: catch label must have same number of types as tag"
                        );
                    }
                    for (expected, actual) in types.zip(params) {
                        self.push_operand(*actual)?;
                        self.pop_operand(Some(expected))?;
                    }
                }
                Catch::OneRef { tag, label } => {
                    let tag = self.tag_at(tag)?;
                    let (ty, kind) = self.jump(label)?;
                    let params = tag.params().iter().copied();
                    let types = self.label_types(ty, kind)?;
                    if params.len() + 1 != types.len() {
                        bail!(
                            self.offset,
                            "type mismatch: catch_ref label must have one \
                             more type than tag types",
                        );
                    }
                    for (expected, actual) in types.zip(params.chain([ValType::EXNREF])) {
                        self.push_operand(actual)?;
                        self.pop_operand(Some(expected))?;
                    }
                }

                Catch::All { label } => {
                    let (ty, kind) = self.jump(label)?;
                    if self.label_types(ty, kind)?.len() != 0 {
                        bail!(
                            self.offset,
                            "type mismatch: catch_all label must have no result types"
                        );
                    }
                }

                Catch::AllRef { label } => {
                    let (ty, kind) = self.jump(label)?;
                    let mut types = self.label_types(ty, kind)?;
                    match (types.next(), types.next()) {
                        (Some(ValType::EXNREF), None) => {}
                        _ => {
                            bail!(
                                self.offset,
                                "type mismatch: catch_all_ref label must have \
                                 one exnref result type"
                            );
                        }
                    }
                }
            }
        }
        self.push_ctrl(FrameKind::TryTable, ty.ty)?;
        Ok(())
    }
    fn visit_throw(&mut self, index: u32) -> Self::Output {
        // Check values associated with the exception.
        let ty = self.tag_at(index)?;
        for ty in ty.clone().params().iter().rev() {
            self.pop_operand(Some(*ty))?;
        }
        if ty.results().len() > 0 {
            bail!(
                self.offset,
                "result type expected to be empty for exception"
            );
        }
        self.unreachable()?;
        Ok(())
    }
    fn visit_throw_ref(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::EXNREF))?;
        self.unreachable()?;
        Ok(())
    }
    fn visit_try(&mut self, _: BlockType) -> Self::Output {
        bail!(self.offset, "unimplemented validation of deprecated opcode")
    }
    fn visit_catch(&mut self, _: u32) -> Self::Output {
        bail!(self.offset, "unimplemented validation of deprecated opcode")
    }
    fn visit_rethrow(&mut self, _: u32) -> Self::Output {
        bail!(self.offset, "unimplemented validation of deprecated opcode")
    }
    fn visit_delegate(&mut self, _: u32) -> Self::Output {
        bail!(self.offset, "unimplemented validation of deprecated opcode")
    }
    fn visit_catch_all(&mut self) -> Self::Output {
        bail!(self.offset, "unimplemented validation of deprecated opcode")
    }
    fn visit_end(&mut self) -> Self::Output {
        let mut frame = self.pop_ctrl()?;

        // Note that this `if` isn't included in the appendix right
        // now, but it's used to allow for `if` statements that are
        // missing an `else` block which have the same parameter/return
        // types on the block (since that's valid).
        if frame.kind == FrameKind::If {
            self.push_ctrl(FrameKind::Else, frame.block_type)?;
            frame = self.pop_ctrl()?;
        }
        for ty in self.results(frame.block_type)? {
            self.push_operand(ty)?;
        }

        if self.control.is_empty() && self.end_which_emptied_control.is_none() {
            assert_ne!(self.offset, 0);
            self.end_which_emptied_control = Some(self.offset);
        }
        Ok(())
    }
    fn visit_br(&mut self, relative_depth: u32) -> Self::Output {
        let (ty, kind) = self.jump(relative_depth)?;
        for ty in self.label_types(ty, kind)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.unreachable()?;
        Ok(())
    }
    fn visit_br_if(&mut self, relative_depth: u32) -> Self::Output {
        self.pop_operand(Some(ValType::I32))?;
        let (ty, kind) = self.jump(relative_depth)?;
        let label_types = self.label_types(ty, kind)?;
        self.pop_push_label_types(label_types)?;
        Ok(())
    }
    fn visit_br_table(&mut self, table: BrTable) -> Self::Output {
        self.pop_operand(Some(ValType::I32))?;
        let default = self.jump(table.default())?;
        let default_types = self.label_types(default.0, default.1)?;
        for element in table.targets() {
            let relative_depth = element?;
            let block = self.jump(relative_depth)?;
            let label_tys = self.label_types(block.0, block.1)?;
            if label_tys.len() != default_types.len() {
                bail!(
                    self.offset,
                    "type mismatch: br_table target labels have different number of types"
                );
            }

            debug_assert!(self.popped_types_tmp.is_empty());
            self.popped_types_tmp.reserve(label_tys.len());
            for expected_ty in label_tys.rev() {
                let actual_ty = self.pop_operand(Some(expected_ty))?;
                self.popped_types_tmp.push(actual_ty);
            }
            for ty in self.inner.popped_types_tmp.drain(..).rev() {
                self.inner.operands.push(ty.into());
            }
        }
        for ty in default_types.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.unreachable()?;
        Ok(())
    }
    fn visit_return(&mut self) -> Self::Output {
        self.check_return()?;
        Ok(())
    }
    fn visit_call(&mut self, function_index: u32) -> Self::Output {
        self.check_call(function_index)?;
        Ok(())
    }
    fn visit_return_call(&mut self, function_index: u32) -> Self::Output {
        self.check_call(function_index)?;
        self.check_return()?;
        Ok(())
    }
    fn visit_call_ref(&mut self, type_index: u32) -> Self::Output {
        let unpacked_index = UnpackedIndex::Module(type_index);
        let mut hty = HeapType::Concrete(unpacked_index);
        self.resources.check_heap_type(&mut hty, self.offset)?;
        // If `None` is popped then that means a "bottom" type was popped which
        // is always considered equivalent to the `hty` tag.
        if let Some(rt) = self.pop_ref()? {
            let expected = RefType::new(true, hty).expect("hty should be previously validated");
            let expected = ValType::Ref(expected);
            if !self.resources.is_subtype(ValType::Ref(rt), expected) {
                bail!(
                    self.offset,
                    "type mismatch: funcref on stack does not match specified type",
                );
            }
        }
        self.check_call_type_index(type_index)
    }
    fn visit_return_call_ref(&mut self, type_index: u32) -> Self::Output {
        self.visit_call_ref(type_index)?;
        self.check_return()
    }
    fn visit_call_indirect(
        &mut self,
        index: u32,
        table_index: u32,
        table_byte: u8,
    ) -> Self::Output {
        if table_byte != 0 && !self.features.reference_types {
            bail!(
                self.offset,
                "reference-types not enabled: zero byte expected"
            );
        }
        self.check_call_indirect(index, table_index)?;
        Ok(())
    }
    fn visit_return_call_indirect(&mut self, index: u32, table_index: u32) -> Self::Output {
        self.check_call_indirect(index, table_index)?;
        self.check_return()?;
        Ok(())
    }
    fn visit_drop(&mut self) -> Self::Output {
        self.pop_operand(None)?;
        Ok(())
    }
    fn visit_select(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::I32))?;
        let ty1 = self.pop_operand(None)?;
        let ty2 = self.pop_operand(None)?;

        let ty = match (ty1, ty2) {
            // All heap-related types aren't allowed with the `select`
            // instruction
            (MaybeType::HeapBot, _)
            | (_, MaybeType::HeapBot)
            | (MaybeType::Type(ValType::Ref(_)), _)
            | (_, MaybeType::Type(ValType::Ref(_))) => {
                bail!(
                    self.offset,
                    "type mismatch: select only takes integral types"
                )
            }

            // If one operand is the "bottom" type then whatever the other
            // operand is is the result of the `select`
            (MaybeType::Bot, t) | (t, MaybeType::Bot) => t,

            // Otherwise these are two integral types and they must match for
            // `select` to typecheck.
            (t @ MaybeType::Type(t1), MaybeType::Type(t2)) => {
                if t1 != t2 {
                    bail!(
                        self.offset,
                        "type mismatch: select operands have different types"
                    );
                }
                t
            }
        };
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_typed_select(&mut self, mut ty: ValType) -> Self::Output {
        self.resources
            .check_value_type(&mut ty, &self.features, self.offset)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_local_get(&mut self, local_index: u32) -> Self::Output {
        let ty = self.local(local_index)?;
        debug_assert_type_indices_are_ids(ty);
        if !self.local_inits[local_index as usize] {
            bail!(self.offset, "uninitialized local: {}", local_index);
        }
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_local_set(&mut self, local_index: u32) -> Self::Output {
        let ty = self.local(local_index)?;
        self.pop_operand(Some(ty))?;
        if !self.local_inits[local_index as usize] {
            self.local_inits[local_index as usize] = true;
            self.inits.push(local_index);
        }
        Ok(())
    }
    fn visit_local_tee(&mut self, local_index: u32) -> Self::Output {
        let expected_ty = self.local(local_index)?;
        let actual_ty = self.pop_operand(Some(expected_ty))?;
        if !self.local_inits[local_index as usize] {
            self.local_inits[local_index as usize] = true;
            self.inits.push(local_index);
        }

        self.push_operand(actual_ty)?;
        Ok(())
    }
    fn visit_global_get(&mut self, global_index: u32) -> Self::Output {
        if let Some(ty) = self.resources.global_at(global_index) {
            let ty = ty.content_type;
            debug_assert_type_indices_are_ids(ty);
            self.push_operand(ty)?;
        } else {
            bail!(self.offset, "unknown global: global index out of bounds");
        };
        Ok(())
    }
    fn visit_global_set(&mut self, global_index: u32) -> Self::Output {
        if let Some(ty) = self.resources.global_at(global_index) {
            if !ty.mutable {
                bail!(
                    self.offset,
                    "global is immutable: cannot modify it with `global.set`"
                );
            }
            self.pop_operand(Some(ty.content_type))?;
        } else {
            bail!(self.offset, "unknown global: global index out of bounds");
        };
        Ok(())
    }
    fn visit_i32_load(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i64_load(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I64)?;
        Ok(())
    }
    fn visit_f32_load(&mut self, memarg: MemArg) -> Self::Output {
        self.check_floats_enabled()?;
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::F32)?;
        Ok(())
    }
    fn visit_f64_load(&mut self, memarg: MemArg) -> Self::Output {
        self.check_floats_enabled()?;
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::F64)?;
        Ok(())
    }
    fn visit_i32_load8_s(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i32_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.visit_i32_load8_s(memarg)
    }
    fn visit_i32_load16_s(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i32_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.visit_i32_load16_s(memarg)
    }
    fn visit_i64_load8_s(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I64)?;
        Ok(())
    }
    fn visit_i64_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.visit_i64_load8_s(memarg)
    }
    fn visit_i64_load16_s(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I64)?;
        Ok(())
    }
    fn visit_i64_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.visit_i64_load16_s(memarg)
    }
    fn visit_i64_load32_s(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I64)?;
        Ok(())
    }
    fn visit_i64_load32_u(&mut self, memarg: MemArg) -> Self::Output {
        self.visit_i64_load32_s(memarg)
    }
    fn visit_i32_store(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_i64_store(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_f32_store(&mut self, memarg: MemArg) -> Self::Output {
        self.check_floats_enabled()?;
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::F32))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_f64_store(&mut self, memarg: MemArg) -> Self::Output {
        self.check_floats_enabled()?;
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::F64))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_i32_store8(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_i32_store16(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_i64_store8(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_i64_store16(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_i64_store32(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_memory_size(&mut self, mem: u32, mem_byte: u8) -> Self::Output {
        if mem_byte != 0 && !self.features.multi_memory {
            bail!(self.offset, "multi-memory not enabled: zero byte expected");
        }
        let index_ty = self.check_memory_index(mem)?;
        self.push_operand(index_ty)?;
        Ok(())
    }
    fn visit_memory_grow(&mut self, mem: u32, mem_byte: u8) -> Self::Output {
        if mem_byte != 0 && !self.features.multi_memory {
            bail!(self.offset, "multi-memory not enabled: zero byte expected");
        }
        let index_ty = self.check_memory_index(mem)?;
        self.pop_operand(Some(index_ty))?;
        self.push_operand(index_ty)?;
        Ok(())
    }
    fn visit_i32_const(&mut self, _value: i32) -> Self::Output {
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i64_const(&mut self, _value: i64) -> Self::Output {
        self.push_operand(ValType::I64)?;
        Ok(())
    }
    fn visit_f32_const(&mut self, _value: Ieee32) -> Self::Output {
        self.check_floats_enabled()?;
        self.push_operand(ValType::F32)?;
        Ok(())
    }
    fn visit_f64_const(&mut self, _value: Ieee64) -> Self::Output {
        self.check_floats_enabled()?;
        self.push_operand(ValType::F64)?;
        Ok(())
    }
    fn visit_i32_eqz(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::I32))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i32_eq(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_ne(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_lt_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_lt_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_gt_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_gt_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_le_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_le_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_ge_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i32_ge_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I32)
    }
    fn visit_i64_eqz(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::I64))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i64_eq(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_ne(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_lt_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_lt_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_gt_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_gt_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_le_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_le_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_ge_s(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_i64_ge_u(&mut self) -> Self::Output {
        self.check_cmp_op(ValType::I64)
    }
    fn visit_f32_eq(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F32)
    }
    fn visit_f32_ne(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F32)
    }
    fn visit_f32_lt(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F32)
    }
    fn visit_f32_gt(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F32)
    }
    fn visit_f32_le(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F32)
    }
    fn visit_f32_ge(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F32)
    }
    fn visit_f64_eq(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F64)
    }
    fn visit_f64_ne(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F64)
    }
    fn visit_f64_lt(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F64)
    }
    fn visit_f64_gt(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F64)
    }
    fn visit_f64_le(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F64)
    }
    fn visit_f64_ge(&mut self) -> Self::Output {
        self.check_fcmp_op(ValType::F64)
    }
    fn visit_i32_clz(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I32)
    }
    fn visit_i32_ctz(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I32)
    }
    fn visit_i32_popcnt(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I32)
    }
    fn visit_i32_add(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_sub(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_mul(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_div_s(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_div_u(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_rem_s(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_rem_u(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_and(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_or(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_xor(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_shl(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_shr_s(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_shr_u(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_rotl(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i32_rotr(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I32)
    }
    fn visit_i64_clz(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I64)
    }
    fn visit_i64_ctz(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I64)
    }
    fn visit_i64_popcnt(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I64)
    }
    fn visit_i64_add(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_sub(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_mul(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_div_s(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_div_u(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_rem_s(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_rem_u(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_and(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_or(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_xor(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_shl(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_shr_s(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_shr_u(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_rotl(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_i64_rotr(&mut self) -> Self::Output {
        self.check_binary_op(ValType::I64)
    }
    fn visit_f32_abs(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F32)
    }
    fn visit_f32_neg(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F32)
    }
    fn visit_f32_ceil(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F32)
    }
    fn visit_f32_floor(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F32)
    }
    fn visit_f32_trunc(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F32)
    }
    fn visit_f32_nearest(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F32)
    }
    fn visit_f32_sqrt(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F32)
    }
    fn visit_f32_add(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F32)
    }
    fn visit_f32_sub(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F32)
    }
    fn visit_f32_mul(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F32)
    }
    fn visit_f32_div(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F32)
    }
    fn visit_f32_min(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F32)
    }
    fn visit_f32_max(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F32)
    }
    fn visit_f32_copysign(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F32)
    }
    fn visit_f64_abs(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F64)
    }
    fn visit_f64_neg(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F64)
    }
    fn visit_f64_ceil(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F64)
    }
    fn visit_f64_floor(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F64)
    }
    fn visit_f64_trunc(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F64)
    }
    fn visit_f64_nearest(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F64)
    }
    fn visit_f64_sqrt(&mut self) -> Self::Output {
        self.check_funary_op(ValType::F64)
    }
    fn visit_f64_add(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F64)
    }
    fn visit_f64_sub(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F64)
    }
    fn visit_f64_mul(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F64)
    }
    fn visit_f64_div(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F64)
    }
    fn visit_f64_min(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F64)
    }
    fn visit_f64_max(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F64)
    }
    fn visit_f64_copysign(&mut self) -> Self::Output {
        self.check_fbinary_op(ValType::F64)
    }
    fn visit_i32_wrap_i64(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::I64)
    }
    fn visit_i32_trunc_f32_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F32)
    }
    fn visit_i32_trunc_f32_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F32)
    }
    fn visit_i32_trunc_f64_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F64)
    }
    fn visit_i32_trunc_f64_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F64)
    }
    fn visit_i64_extend_i32_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::I32)
    }
    fn visit_i64_extend_i32_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::I32)
    }
    fn visit_i64_trunc_f32_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F32)
    }
    fn visit_i64_trunc_f32_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F32)
    }
    fn visit_i64_trunc_f64_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F64)
    }
    fn visit_i64_trunc_f64_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F64)
    }
    fn visit_f32_convert_i32_s(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F32, ValType::I32)
    }
    fn visit_f32_convert_i32_u(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F32, ValType::I32)
    }
    fn visit_f32_convert_i64_s(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F32, ValType::I64)
    }
    fn visit_f32_convert_i64_u(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F32, ValType::I64)
    }
    fn visit_f32_demote_f64(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F32, ValType::F64)
    }
    fn visit_f64_convert_i32_s(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F64, ValType::I32)
    }
    fn visit_f64_convert_i32_u(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F64, ValType::I32)
    }
    fn visit_f64_convert_i64_s(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F64, ValType::I64)
    }
    fn visit_f64_convert_i64_u(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F64, ValType::I64)
    }
    fn visit_f64_promote_f32(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F64, ValType::F32)
    }
    fn visit_i32_reinterpret_f32(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F32)
    }
    fn visit_i64_reinterpret_f64(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F64)
    }
    fn visit_f32_reinterpret_i32(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F32, ValType::I32)
    }
    fn visit_f64_reinterpret_i64(&mut self) -> Self::Output {
        self.check_fconversion_op(ValType::F64, ValType::I64)
    }
    fn visit_i32_trunc_sat_f32_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F32)
    }
    fn visit_i32_trunc_sat_f32_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F32)
    }
    fn visit_i32_trunc_sat_f64_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F64)
    }
    fn visit_i32_trunc_sat_f64_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I32, ValType::F64)
    }
    fn visit_i64_trunc_sat_f32_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F32)
    }
    fn visit_i64_trunc_sat_f32_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F32)
    }
    fn visit_i64_trunc_sat_f64_s(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F64)
    }
    fn visit_i64_trunc_sat_f64_u(&mut self) -> Self::Output {
        self.check_conversion_op(ValType::I64, ValType::F64)
    }
    fn visit_i32_extend8_s(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I32)
    }
    fn visit_i32_extend16_s(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I32)
    }
    fn visit_i64_extend8_s(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I64)
    }
    fn visit_i64_extend16_s(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I64)
    }
    fn visit_i64_extend32_s(&mut self) -> Self::Output {
        self.check_unary_op(ValType::I64)
    }
    fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_load(memarg, ValType::I32)
    }
    fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_load(memarg, ValType::I32)
    }
    fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_load(memarg, ValType::I32)
    }
    fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_load(memarg, ValType::I64)
    }
    fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_load(memarg, ValType::I64)
    }
    fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_load(memarg, ValType::I64)
    }
    fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_load(memarg, ValType::I64)
    }
    fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_store(memarg, ValType::I32)
    }
    fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_store(memarg, ValType::I32)
    }
    fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_store(memarg, ValType::I32)
    }
    fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_store(memarg, ValType::I64)
    }
    fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_store(memarg, ValType::I64)
    }
    fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_store(memarg, ValType::I64)
    }
    fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_store(memarg, ValType::I64)
    }
    fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_cmpxchg(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_cmpxchg(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_cmpxchg(memarg, ValType::I32)
    }
    fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_cmpxchg(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_cmpxchg(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_cmpxchg(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_cmpxchg(memarg, ValType::I64)
    }
    fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_op(memarg, ValType::I32)
    }
    fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_atomic_fence(&mut self) -> Self::Output {
        Ok(())
    }
    fn visit_ref_null(&mut self, mut heap_type: HeapType) -> Self::Output {
        if let Some(ty) = RefType::new(true, heap_type) {
            self.features
                .check_ref_type(ty)
                .map_err(|e| BinaryReaderError::new(e, self.offset))?;
        }
        self.resources
            .check_heap_type(&mut heap_type, self.offset)?;
        let ty = ValType::Ref(
            RefType::new(true, heap_type).expect("existing heap types should be within our limits"),
        );
        self.push_operand(ty)?;
        Ok(())
    }

    fn visit_ref_as_non_null(&mut self) -> Self::Output {
        let ty = match self.pop_ref()? {
            Some(ty) => MaybeType::Type(ValType::Ref(ty.as_non_null())),
            None => MaybeType::HeapBot,
        };
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_br_on_null(&mut self, relative_depth: u32) -> Self::Output {
        let ref_ty = match self.pop_ref()? {
            None => MaybeType::HeapBot,
            Some(ty) => MaybeType::Type(ValType::Ref(ty.as_non_null())),
        };
        let (ft, kind) = self.jump(relative_depth)?;
        let label_types = self.label_types(ft, kind)?;
        self.pop_push_label_types(label_types)?;
        self.push_operand(ref_ty)?;
        Ok(())
    }
    fn visit_br_on_non_null(&mut self, relative_depth: u32) -> Self::Output {
        let ty = self.pop_ref()?;
        let (ft, kind) = self.jump(relative_depth)?;

        let mut label_types = self.label_types(ft, kind)?;
        match (label_types.next_back(), ty) {
            (None, _) => bail!(
                self.offset,
                "type mismatch: br_on_non_null target has no label types",
            ),
            (Some(ValType::Ref(_)), None) => {}
            (Some(rt1 @ ValType::Ref(_)), Some(rt0)) => {
                // Switch rt0, our popped type, to a non-nullable type and
                // perform the match because if the branch is taken it's a
                // non-null value.
                let ty = rt0.as_non_null();
                if !self.resources.is_subtype(ty.into(), rt1) {
                    bail!(
                        self.offset,
                        "type mismatch: expected {} but found {}",
                        ty_to_str(rt0.into()),
                        ty_to_str(rt1)
                    )
                }
            }
            (Some(_), _) => bail!(
                self.offset,
                "type mismatch: br_on_non_null target does not end with heap type",
            ),
        }

        self.pop_push_label_types(label_types)?;
        Ok(())
    }
    fn visit_ref_is_null(&mut self) -> Self::Output {
        self.pop_ref()?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_ref_func(&mut self, function_index: u32) -> Self::Output {
        let type_id = match self.resources.type_id_of_function(function_index) {
            Some(id) => id,
            None => bail!(
                self.offset,
                "unknown function {}: function index out of bounds",
                function_index,
            ),
        };
        if !self.resources.is_function_referenced(function_index) {
            bail!(self.offset, "undeclared function reference");
        }

        let index = UnpackedIndex::Id(type_id);
        let ty = ValType::Ref(
            RefType::new(false, HeapType::Concrete(index)).ok_or_else(|| {
                BinaryReaderError::new("implementation limit: type index too large", self.offset)
            })?,
        );
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_ref_eq(&mut self) -> Self::Output {
        self.pop_operand(Some(RefType::EQ.nullable().into()))?;
        self.pop_operand(Some(RefType::EQ.nullable().into()))?;
        self.push_operand(ValType::I32)
    }
    fn visit_v128_load(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_store(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_v128_const(&mut self, _value: V128) -> Self::Output {
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_i8x16_splat(&mut self) -> Self::Output {
        self.check_v128_splat(ValType::I32)
    }
    fn visit_i16x8_splat(&mut self) -> Self::Output {
        self.check_v128_splat(ValType::I32)
    }
    fn visit_i32x4_splat(&mut self) -> Self::Output {
        self.check_v128_splat(ValType::I32)
    }
    fn visit_i64x2_splat(&mut self) -> Self::Output {
        self.check_v128_splat(ValType::I64)
    }
    fn visit_f32x4_splat(&mut self) -> Self::Output {
        self.check_floats_enabled()?;
        self.check_v128_splat(ValType::F32)
    }
    fn visit_f64x2_splat(&mut self) -> Self::Output {
        self.check_floats_enabled()?;
        self.check_v128_splat(ValType::F64)
    }
    fn visit_i8x16_extract_lane_s(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 16)?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i8x16_extract_lane_u(&mut self, lane: u8) -> Self::Output {
        self.visit_i8x16_extract_lane_s(lane)
    }
    fn visit_i16x8_extract_lane_s(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 8)?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i16x8_extract_lane_u(&mut self, lane: u8) -> Self::Output {
        self.visit_i16x8_extract_lane_s(lane)
    }
    fn visit_i32x4_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 4)?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_i8x16_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 16)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_i16x8_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 8)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_i32x4_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 4)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_i64x2_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 2)?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::I64)?;
        Ok(())
    }
    fn visit_i64x2_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.check_simd_lane_index(lane, 2)?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_f32x4_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.check_floats_enabled()?;
        self.check_simd_lane_index(lane, 4)?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::F32)?;
        Ok(())
    }
    fn visit_f32x4_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.check_floats_enabled()?;
        self.check_simd_lane_index(lane, 4)?;
        self.pop_operand(Some(ValType::F32))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_f64x2_extract_lane(&mut self, lane: u8) -> Self::Output {
        self.check_floats_enabled()?;
        self.check_simd_lane_index(lane, 2)?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::F64)?;
        Ok(())
    }
    fn visit_f64x2_replace_lane(&mut self, lane: u8) -> Self::Output {
        self.check_floats_enabled()?;
        self.check_simd_lane_index(lane, 2)?;
        self.pop_operand(Some(ValType::F64))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_f32x4_eq(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_ne(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_lt(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_gt(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_le(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_ge(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_eq(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_ne(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_lt(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_gt(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_le(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_ge(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_add(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_sub(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_mul(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_div(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_min(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_max(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_pmin(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f32x4_pmax(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_add(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_sub(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_mul(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_div(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_min(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_max(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_pmin(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_f64x2_pmax(&mut self) -> Self::Output {
        self.check_v128_fbinary_op()
    }
    fn visit_i8x16_eq(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_ne(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_lt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_lt_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_gt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_gt_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_le_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_le_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_ge_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_ge_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_eq(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_ne(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_lt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_lt_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_gt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_gt_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_le_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_le_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_ge_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_ge_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_eq(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_ne(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_lt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_lt_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_gt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_gt_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_le_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_le_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_ge_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_ge_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_eq(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_ne(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_lt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_gt_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_le_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_ge_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_v128_and(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_v128_andnot(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_v128_or(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_v128_xor(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_add(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_add_sat_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_add_sat_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_sub(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_sub_sat_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_sub_sat_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_min_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_min_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_max_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_max_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_add(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_add_sat_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_add_sat_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_sub(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_sub_sat_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_sub_sat_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_mul(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_min_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_min_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_max_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_max_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_add(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_sub(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_mul(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_min_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_min_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_max_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_max_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_dot_i16x8_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_add(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_sub(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_mul(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_avgr_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_avgr_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_narrow_i16x8_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i8x16_narrow_i16x8_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_narrow_i32x4_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_narrow_i32x4_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_extmul_low_i8x16_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_extmul_high_i8x16_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_extmul_low_i8x16_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_extmul_high_i8x16_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_extmul_low_i16x8_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_extmul_high_i16x8_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_extmul_low_i16x8_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_extmul_high_i16x8_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_extmul_low_i32x4_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_extmul_high_i32x4_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_extmul_low_i32x4_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i64x2_extmul_high_i32x4_u(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_q15mulr_sat_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_f32x4_ceil(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_floor(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_trunc(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_nearest(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_ceil(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_floor(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_trunc(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_nearest(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_abs(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_neg(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_sqrt(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_abs(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_neg(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_sqrt(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_demote_f64x2_zero(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_promote_low_f32x4(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_convert_low_i32x4_s(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f64x2_convert_low_i32x4_u(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_i32x4_trunc_sat_f32x4_s(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_i32x4_trunc_sat_f32x4_u(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_i32x4_trunc_sat_f64x2_s_zero(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_i32x4_trunc_sat_f64x2_u_zero(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_convert_i32x4_s(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_f32x4_convert_i32x4_u(&mut self) -> Self::Output {
        self.check_v128_funary_op()
    }
    fn visit_v128_not(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i8x16_abs(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i8x16_neg(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i8x16_popcnt(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_abs(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_neg(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_abs(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_neg(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i64x2_abs(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i64x2_neg(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_extend_low_i8x16_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_extend_high_i8x16_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_extend_low_i8x16_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_extend_high_i8x16_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_extend_low_i16x8_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_extend_high_i16x8_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_extend_low_i16x8_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_extend_high_i16x8_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i64x2_extend_low_i32x4_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i64x2_extend_high_i32x4_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i64x2_extend_low_i32x4_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i64x2_extend_high_i32x4_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_extadd_pairwise_i8x16_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i16x8_extadd_pairwise_i8x16_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_extadd_pairwise_i16x8_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_extadd_pairwise_i16x8_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_v128_bitselect(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_i8x16_relaxed_swizzle(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_i32x4_relaxed_trunc_f32x4_s(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_relaxed_trunc_f32x4_u(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_relaxed_trunc_f64x2_s_zero(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_i32x4_relaxed_trunc_f64x2_u_zero(&mut self) -> Self::Output {
        self.check_v128_unary_op()
    }
    fn visit_f32x4_relaxed_madd(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_f32x4_relaxed_nmadd(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_f64x2_relaxed_madd(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_f64x2_relaxed_nmadd(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_i8x16_relaxed_laneselect(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_i16x8_relaxed_laneselect(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_i32x4_relaxed_laneselect(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_i64x2_relaxed_laneselect(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_f32x4_relaxed_min(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_f32x4_relaxed_max(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_f64x2_relaxed_min(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_f64x2_relaxed_max(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_relaxed_q15mulr_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i16x8_relaxed_dot_i8x16_i7x16_s(&mut self) -> Self::Output {
        self.check_v128_binary_op()
    }
    fn visit_i32x4_relaxed_dot_i8x16_i7x16_add_s(&mut self) -> Self::Output {
        self.check_v128_ternary_op()
    }
    fn visit_v128_any_true(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i8x16_all_true(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i8x16_bitmask(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i16x8_all_true(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i16x8_bitmask(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i32x4_all_true(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i32x4_bitmask(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i64x2_all_true(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i64x2_bitmask(&mut self) -> Self::Output {
        self.check_v128_bitmask_op()
    }
    fn visit_i8x16_shl(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i8x16_shr_s(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i8x16_shr_u(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i16x8_shl(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i16x8_shr_s(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i16x8_shr_u(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i32x4_shl(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i32x4_shr_s(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i32x4_shr_u(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i64x2_shl(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i64x2_shr_s(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i64x2_shr_u(&mut self) -> Self::Output {
        self.check_v128_shift_op()
    }
    fn visit_i8x16_swizzle(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_i8x16_shuffle(&mut self, lanes: [u8; 16]) -> Self::Output {
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(ValType::V128))?;
        for i in lanes {
            self.check_simd_lane_index(i, 32)?;
        }
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_load8_splat(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_load16_splat(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_load32_splat(&mut self, memarg: MemArg) -> Self::Output {
        let ty = self.check_memarg(memarg)?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_load32_zero(&mut self, memarg: MemArg) -> Self::Output {
        self.visit_v128_load32_splat(memarg)
    }
    fn visit_v128_load64_splat(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load64_zero(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load8x8_s(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load8x8_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load16x4_s(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load16x4_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load32x2_s(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load32x2_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_v128_load_op(memarg)
    }
    fn visit_v128_load8_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 16)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_load16_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 8)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_load32_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 4)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_load64_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 2)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        self.push_operand(ValType::V128)?;
        Ok(())
    }
    fn visit_v128_store8_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 16)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        Ok(())
    }
    fn visit_v128_store16_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 8)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        Ok(())
    }
    fn visit_v128_store32_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 4)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        Ok(())
    }
    fn visit_v128_store64_lane(&mut self, memarg: MemArg, lane: u8) -> Self::Output {
        let idx = self.check_memarg(memarg)?;
        self.check_simd_lane_index(lane, 2)?;
        self.pop_operand(Some(ValType::V128))?;
        self.pop_operand(Some(idx))?;
        Ok(())
    }
    fn visit_memory_init(&mut self, segment: u32, mem: u32) -> Self::Output {
        let ty = self.check_memory_index(mem)?;
        match self.resources.data_count() {
            None => bail!(self.offset, "data count section required"),
            Some(count) if segment < count => {}
            Some(_) => bail!(self.offset, "unknown data segment {}", segment),
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_data_drop(&mut self, segment: u32) -> Self::Output {
        match self.resources.data_count() {
            None => bail!(self.offset, "data count section required"),
            Some(count) if segment < count => {}
            Some(_) => bail!(self.offset, "unknown data segment {}", segment),
        }
        Ok(())
    }
    fn visit_memory_copy(&mut self, dst: u32, src: u32) -> Self::Output {
        let dst_ty = self.check_memory_index(dst)?;
        let src_ty = self.check_memory_index(src)?;

        // The length operand here is the smaller of src/dst, which is
        // i32 if one is i32
        self.pop_operand(Some(match src_ty {
            ValType::I32 => ValType::I32,
            _ => dst_ty,
        }))?;

        // ... and the offset into each memory is required to be
        // whatever the indexing type is for that memory
        self.pop_operand(Some(src_ty))?;
        self.pop_operand(Some(dst_ty))?;
        Ok(())
    }
    fn visit_memory_fill(&mut self, mem: u32) -> Self::Output {
        let ty = self.check_memory_index(mem)?;
        self.pop_operand(Some(ty))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_memory_discard(&mut self, mem: u32) -> Self::Output {
        let ty = self.check_memory_index(mem)?;
        self.pop_operand(Some(ty))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_table_init(&mut self, segment: u32, table: u32) -> Self::Output {
        if table > 0 {}
        let table = match self.resources.table_at(table) {
            Some(table) => table,
            None => bail!(
                self.offset,
                "unknown table {}: table index out of bounds",
                table
            ),
        };
        let segment_ty = self.element_type_at(segment)?;
        if !self
            .resources
            .is_subtype(ValType::Ref(segment_ty), ValType::Ref(table.element_type))
        {
            bail!(self.offset, "type mismatch");
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        Ok(())
    }
    fn visit_elem_drop(&mut self, segment: u32) -> Self::Output {
        if segment >= self.resources.element_count() {
            bail!(
                self.offset,
                "unknown elem segment {}: segment index out of bounds",
                segment
            );
        }
        Ok(())
    }
    fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> Self::Output {
        if src_table > 0 || dst_table > 0 {}
        let (src, dst) = match (
            self.resources.table_at(src_table),
            self.resources.table_at(dst_table),
        ) {
            (Some(a), Some(b)) => (a, b),
            _ => bail!(self.offset, "table index out of bounds"),
        };
        if !self.resources.is_subtype(
            ValType::Ref(src.element_type),
            ValType::Ref(dst.element_type),
        ) {
            bail!(self.offset, "type mismatch");
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        Ok(())
    }
    fn visit_table_get(&mut self, table: u32) -> Self::Output {
        let ty = match self.resources.table_at(table) {
            Some(ty) => ty.element_type,
            None => bail!(self.offset, "table index out of bounds"),
        };
        let ty = ValType::Ref(ty);
        debug_assert_type_indices_are_ids(ty);
        self.pop_operand(Some(ValType::I32))?;
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_table_set(&mut self, table: u32) -> Self::Output {
        let ty = match self.resources.table_at(table) {
            Some(ty) => ValType::Ref(ty.element_type),
            None => bail!(self.offset, "table index out of bounds"),
        };
        debug_assert_type_indices_are_ids(ty);
        self.pop_operand(Some(ty))?;
        self.pop_operand(Some(ValType::I32))?;
        Ok(())
    }
    fn visit_table_grow(&mut self, table: u32) -> Self::Output {
        let ty = match self.resources.table_at(table) {
            Some(ty) => ValType::Ref(ty.element_type),
            None => bail!(self.offset, "table index out of bounds"),
        };
        debug_assert_type_indices_are_ids(ty);
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_table_size(&mut self, table: u32) -> Self::Output {
        if self.resources.table_at(table).is_none() {
            bail!(self.offset, "table index out of bounds");
        }
        self.push_operand(ValType::I32)?;
        Ok(())
    }
    fn visit_table_fill(&mut self, table: u32) -> Self::Output {
        let ty = match self.resources.table_at(table) {
            Some(ty) => ValType::Ref(ty.element_type),
            None => bail!(self.offset, "table index out of bounds"),
        };
        debug_assert_type_indices_are_ids(ty);
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        self.pop_operand(Some(ValType::I32))?;
        Ok(())
    }
    fn visit_struct_new(&mut self, struct_type_index: u32) -> Self::Output {
        let struct_ty = self.struct_type_at(struct_type_index)?;
        for ty in struct_ty.fields.iter().rev() {
            self.pop_operand(Some(ty.element_type.unpack()))?;
        }
        self.push_concrete_ref(false, struct_type_index)?;
        Ok(())
    }
    fn visit_struct_new_default(&mut self, type_index: u32) -> Self::Output {
        let ty = self.struct_type_at(type_index)?;
        for field in ty.fields.iter() {
            let val_ty = field.element_type.unpack();
            if !val_ty.is_defaultable() {
                bail!(
                    self.offset,
                    "invalid `struct.new_default`: {val_ty} field is not defaultable"
                );
            }
        }
        self.push_concrete_ref(false, type_index)?;
        Ok(())
    }
    fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> Self::Output {
        let field_ty = self.struct_field_at(struct_type_index, field_index)?;
        if field_ty.element_type.is_packed() {
            bail!(
                self.offset,
                "can only use struct.get with non-packed storage types"
            )
        }
        self.pop_concrete_ref(true, struct_type_index)?;
        self.push_operand(field_ty.element_type.unpack())
    }
    fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> Self::Output {
        let field_ty = self.struct_field_at(struct_type_index, field_index)?;
        if !field_ty.element_type.is_packed() {
            bail!(
                self.offset,
                "cannot use struct.get_s with non-packed storage types"
            )
        }
        self.pop_concrete_ref(true, struct_type_index)?;
        self.push_operand(field_ty.element_type.unpack())
    }
    fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> Self::Output {
        let field_ty = self.struct_field_at(struct_type_index, field_index)?;
        if !field_ty.element_type.is_packed() {
            bail!(
                self.offset,
                "cannot use struct.get_u with non-packed storage types"
            )
        }
        self.pop_concrete_ref(true, struct_type_index)?;
        self.push_operand(field_ty.element_type.unpack())
    }
    fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> Self::Output {
        let field_ty = self.struct_field_at(struct_type_index, field_index)?;
        if !field_ty.mutable {
            bail!(self.offset, "invalid struct.set: struct field is immutable")
        }
        self.pop_operand(Some(field_ty.element_type.unpack()))?;
        self.pop_concrete_ref(true, struct_type_index)?;
        Ok(())
    }
    fn visit_array_new(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(array_ty.0.element_type.unpack()))?;
        self.push_concrete_ref(false, type_index)
    }
    fn visit_array_new_default(&mut self, type_index: u32) -> Self::Output {
        let ty = self.array_type_at(type_index)?;
        let val_ty = ty.0.element_type.unpack();
        if !val_ty.is_defaultable() {
            bail!(
                self.offset,
                "invalid `array.new_default`: {val_ty} field is not defaultable"
            );
        }
        self.pop_operand(Some(ValType::I32))?;
        self.push_concrete_ref(false, type_index)
    }
    fn visit_array_new_fixed(&mut self, type_index: u32, n: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.0.element_type.unpack();
        for _ in 0..n {
            self.pop_operand(Some(elem_ty))?;
        }
        self.push_concrete_ref(false, type_index)
    }
    fn visit_array_new_data(&mut self, type_index: u32, data_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.0.element_type.unpack();
        match elem_ty {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => {}
            ValType::Ref(_) => bail!(
                self.offset,
                "type mismatch: array.new_data can only create arrays with numeric and vector elements"
            ),
        }
        match self.resources.data_count() {
            None => bail!(self.offset, "data count section required"),
            Some(count) if data_index < count => {}
            Some(_) => bail!(self.offset, "unknown data segment {}", data_index),
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.push_concrete_ref(false, type_index)
    }
    fn visit_array_new_elem(&mut self, type_index: u32, elem_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let array_ref_ty = match array_ty.0.element_type.unpack() {
            ValType::Ref(rt) => rt,
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => bail!(
                self.offset,
                "type mismatch: array.new_elem can only create arrays with reference elements"
            ),
        };
        let elem_ref_ty = self.element_type_at(elem_index)?;
        if !self
            .resources
            .is_subtype(elem_ref_ty.into(), array_ref_ty.into())
        {
            bail!(
                self.offset,
                "invalid array.new_elem instruction: element segment {elem_index} type mismatch: \
                 expected {array_ref_ty}, found {elem_ref_ty}"
            )
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.push_concrete_ref(false, type_index)
    }
    fn visit_array_get(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.0.element_type;
        if elem_ty.is_packed() {
            bail!(
                self.offset,
                "cannot use array.get with packed storage types"
            )
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        self.push_operand(elem_ty.unpack())
    }
    fn visit_array_get_s(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.0.element_type;
        if !elem_ty.is_packed() {
            bail!(
                self.offset,
                "cannot use array.get_s with non-packed storage types"
            )
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        self.push_operand(elem_ty.unpack())
    }
    fn visit_array_get_u(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.0.element_type;
        if !elem_ty.is_packed() {
            bail!(
                self.offset,
                "cannot use array.get_u with non-packed storage types"
            )
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        self.push_operand(elem_ty.unpack())
    }
    fn visit_array_set(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        if !array_ty.0.mutable {
            bail!(self.offset, "invalid array.set: array is immutable")
        }
        self.pop_operand(Some(array_ty.0.element_type.unpack()))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        Ok(())
    }
    fn visit_array_len(&mut self) -> Self::Output {
        self.pop_operand(Some(RefType::ARRAY.nullable().into()))?;
        self.push_operand(ValType::I32)
    }
    fn visit_array_fill(&mut self, array_type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(array_type_index)?;
        if !array_ty.0.mutable {
            bail!(self.offset, "invalid array.fill: array is immutable");
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(array_ty.0.element_type.unpack()))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, array_type_index)?;
        Ok(())
    }
    fn visit_array_copy(&mut self, type_index_dst: u32, type_index_src: u32) -> Self::Output {
        let array_ty_dst = self.array_type_at(type_index_dst)?;
        if !array_ty_dst.0.mutable {
            bail!(
                self.offset,
                "invalid array.copy: destination array is immutable"
            );
        }
        let array_ty_src = self.array_type_at(type_index_src)?;
        match (array_ty_dst.0.element_type, array_ty_src.0.element_type) {
            (StorageType::I8, StorageType::I8) => {}
            (StorageType::I8, ty) => bail!(
                self.offset,
                "array types do not match: expected i8, found {ty}"
            ),
            (StorageType::I16, StorageType::I16) => {}
            (StorageType::I16, ty) => bail!(
                self.offset,
                "array types do not match: expected i16, found {ty}"
            ),
            (StorageType::Val(dst), StorageType::Val(src)) => {
                if !self.resources.is_subtype(src, dst) {
                    bail!(
                        self.offset,
                        "array types do not match: expected {dst}, found {src}"
                    )
                }
            }
            (StorageType::Val(dst), src) => {
                bail!(
                    self.offset,
                    "array types do not match: expected {dst}, found {src}"
                )
            }
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index_src)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index_dst)?;
        Ok(())
    }
    fn visit_array_init_data(
        &mut self,
        array_type_index: u32,
        array_data_index: u32,
    ) -> Self::Output {
        let array_ty = self.array_type_at(array_type_index)?;
        if !array_ty.0.mutable {
            bail!(self.offset, "invalid array.init_data: array is immutable");
        }
        let val_ty = array_ty.0.element_type.unpack();
        match val_ty {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => {}
            ValType::Ref(_) => bail!(
                self.offset,
                "invalid array.init_data: array type is not numeric or vector"
            ),
        }
        match self.resources.data_count() {
            None => bail!(self.offset, "data count section required"),
            Some(count) if array_data_index < count => {}
            Some(_) => bail!(self.offset, "unknown data segment {}", array_data_index),
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, array_type_index)?;
        Ok(())
    }
    fn visit_array_init_elem(&mut self, type_index: u32, elem_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        if !array_ty.0.mutable {
            bail!(self.offset, "invalid array.init_data: array is immutable");
        }
        let array_ref_ty = match array_ty.0.element_type.unpack() {
            ValType::Ref(rt) => rt,
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => bail!(
                self.offset,
                "type mismatch: array.init_elem can only create arrays with reference elements"
            ),
        };
        let elem_ref_ty = self.element_type_at(elem_index)?;
        if !self
            .resources
            .is_subtype(elem_ref_ty.into(), array_ref_ty.into())
        {
            bail!(
                self.offset,
                "invalid array.init_elem instruction: element segment {elem_index} type mismatch: \
                 expected {array_ref_ty}, found {elem_ref_ty}"
            )
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        Ok(())
    }
    fn visit_any_convert_extern(&mut self) -> Self::Output {
        let extern_ref = self.pop_operand(Some(RefType::EXTERNREF.into()))?;
        let is_nullable = extern_ref
            .as_type()
            .map_or(false, |ty| ty.as_reference_type().unwrap().is_nullable());
        let any_ref = RefType::new(is_nullable, HeapType::Any).unwrap();
        self.push_operand(any_ref)
    }
    fn visit_extern_convert_any(&mut self) -> Self::Output {
        let any_ref = self.pop_operand(Some(RefType::ANY.nullable().into()))?;
        let is_nullable = any_ref
            .as_type()
            .map_or(false, |ty| ty.as_reference_type().unwrap().is_nullable());
        let extern_ref = RefType::new(is_nullable, HeapType::Extern).unwrap();
        self.push_operand(extern_ref)
    }
    fn visit_ref_test_non_null(&mut self, heap_type: HeapType) -> Self::Output {
        self.check_ref_test(false, heap_type)
    }
    fn visit_ref_test_nullable(&mut self, heap_type: HeapType) -> Self::Output {
        self.check_ref_test(true, heap_type)
    }
    fn visit_ref_cast_non_null(&mut self, heap_type: HeapType) -> Self::Output {
        self.check_ref_cast(false, heap_type)
    }
    fn visit_ref_cast_nullable(&mut self, heap_type: HeapType) -> Self::Output {
        self.check_ref_cast(true, heap_type)
    }
    fn visit_br_on_cast(
        &mut self,
        relative_depth: u32,
        mut from_ref_type: RefType,
        mut to_ref_type: RefType,
    ) -> Self::Output {
        self.resources
            .check_ref_type(&mut from_ref_type, self.offset)?;
        self.resources
            .check_ref_type(&mut to_ref_type, self.offset)?;

        if !self
            .resources
            .is_subtype(to_ref_type.into(), from_ref_type.into())
        {
            bail!(
                self.offset,
                "type mismatch: expected {from_ref_type}, found {to_ref_type}"
            );
        }

        let (block_ty, frame_kind) = self.jump(relative_depth)?;
        let mut label_types = self.label_types(block_ty, frame_kind)?;

        match label_types.next_back() {
            Some(label_ty) if self.resources.is_subtype(to_ref_type.into(), label_ty) => {
                self.pop_operand(Some(from_ref_type.into()))?;
            }
            Some(label_ty) => bail!(
                self.offset,
                "type mismatch: casting to type {to_ref_type}, but it does not match \
                 label result type {label_ty}"
            ),
            None => bail!(
                self.offset,
                "type mismtach: br_on_cast to label with empty types, must have a reference type"
            ),
        };

        self.pop_push_label_types(label_types)?;
        let diff_ty = RefType::difference(from_ref_type, to_ref_type);
        self.push_operand(diff_ty)?;
        Ok(())
    }
    fn visit_br_on_cast_fail(
        &mut self,
        relative_depth: u32,
        mut from_ref_type: RefType,
        mut to_ref_type: RefType,
    ) -> Self::Output {
        self.resources
            .check_ref_type(&mut from_ref_type, self.offset)?;
        self.resources
            .check_ref_type(&mut to_ref_type, self.offset)?;

        if !self
            .resources
            .is_subtype(to_ref_type.into(), from_ref_type.into())
        {
            bail!(
                self.offset,
                "type mismatch: expected {from_ref_type}, found {to_ref_type}"
            );
        }

        let (block_ty, frame_kind) = self.jump(relative_depth)?;
        let mut label_tys = self.label_types(block_ty, frame_kind)?;

        let diff_ty = RefType::difference(from_ref_type, to_ref_type);
        match label_tys.next_back() {
            Some(label_ty) if self.resources.is_subtype(diff_ty.into(), label_ty) => {
                self.pop_operand(Some(from_ref_type.into()))?;
            }
            Some(label_ty) => bail!(
                self.offset,
                "type mismatch: expected label result type {label_ty}, found {diff_ty}"
            ),
            None => bail!(
                self.offset,
                "type mismatch: expected a reference type, found nothing"
            ),
        }

        self.pop_push_label_types(label_tys)?;
        self.push_operand(to_ref_type)?;
        Ok(())
    }
    fn visit_ref_i31(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::I32))?;
        self.push_operand(ValType::Ref(RefType::I31))
    }
    fn visit_i31_get_s(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::Ref(RefType::I31REF)))?;
        self.push_operand(ValType::I32)
    }
    fn visit_i31_get_u(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::Ref(RefType::I31REF)))?;
        self.push_operand(ValType::I32)
    }
}

#[derive(Clone, Debug)]
enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> Iterator for Either<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;
    fn next(&mut self) -> Option<A::Item> {
        match self {
            Either::A(a) => a.next(),
            Either::B(b) => b.next(),
        }
    }
}

impl<A, B> DoubleEndedIterator for Either<A, B>
where
    A: DoubleEndedIterator,
    B: DoubleEndedIterator<Item = A::Item>,
{
    fn next_back(&mut self) -> Option<A::Item> {
        match self {
            Either::A(a) => a.next_back(),
            Either::B(b) => b.next_back(),
        }
    }
}

impl<A, B> ExactSizeIterator for Either<A, B>
where
    A: ExactSizeIterator,
    B: ExactSizeIterator<Item = A::Item>,
{
    fn len(&self) -> usize {
        match self {
            Either::A(a) => a.len(),
            Either::B(b) => b.len(),
        }
    }
}

trait PreciseIterator: ExactSizeIterator + DoubleEndedIterator + Clone + std::fmt::Debug {}
impl<T: ExactSizeIterator + DoubleEndedIterator + Clone + std::fmt::Debug> PreciseIterator for T {}

impl Locals {
    /// Defines another group of `count` local variables of type `ty`.
    ///
    /// Returns `true` if the definition was successful. Local variable
    /// definition is unsuccessful in case the amount of total variables
    /// after definition exceeds the allowed maximum number.
    fn define(&mut self, count: u32, ty: ValType) -> bool {
        match self.num_locals.checked_add(count) {
            Some(n) => self.num_locals = n,
            None => return false,
        }
        if self.num_locals > (MAX_WASM_FUNCTION_LOCALS as u32) {
            return false;
        }
        for _ in 0..count {
            if self.first.len() >= MAX_LOCALS_TO_TRACK {
                break;
            }
            self.first.push(ty);
        }
        self.all.push((self.num_locals - 1, ty));
        true
    }

    /// Returns the number of defined local variables.
    pub(super) fn len_locals(&self) -> u32 {
        self.num_locals
    }

    /// Returns the type of the local variable at the given index if any.
    #[inline]
    pub(super) fn get(&self, idx: u32) -> Option<ValType> {
        match self.first.get(idx as usize) {
            Some(ty) => Some(*ty),
            None => self.get_bsearch(idx),
        }
    }

    fn get_bsearch(&self, idx: u32) -> Option<ValType> {
        match self.all.binary_search_by_key(&idx, |(idx, _)| *idx) {
            // If this index would be inserted at the end of the list, then the
            // index is out of bounds and we return an error.
            Err(i) if i == self.all.len() => None,

            // If `Ok` is returned we found the index exactly, or if `Err` is
            // returned the position is the one which is the least index
            // greater that `idx`, which is still the type of `idx` according
            // to our "compressed" representation. In both cases we access the
            // list at index `i`.
            Ok(i) | Err(i) => Some(self.all[i].1),
        }
    }
}
