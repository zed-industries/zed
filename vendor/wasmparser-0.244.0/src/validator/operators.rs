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

#[cfg(feature = "simd")]
use crate::VisitSimdOperator;
use crate::{
    AbstractHeapType, BinaryReaderError, BlockType, BrTable, Catch, ContType, FieldType, FrameKind,
    FrameStack, FuncType, GlobalType, Handle, HeapType, Ieee32, Ieee64, MemArg, ModuleArity,
    RefType, Result, ResumeTable, StorageType, StructType, SubType, TableType, TryTable,
    UnpackedIndex, ValType, VisitOperator, WasmFeatures, WasmModuleResources,
    limits::MAX_WASM_FUNCTION_LOCALS,
};
use crate::{CompositeInnerType, Ordering, prelude::*};
use core::ops::{Deref, DerefMut};
use core::{cmp, iter, mem};

#[cfg(feature = "simd")]
mod simd;

pub(crate) struct OperatorValidator {
    pub(super) locals: Locals,
    local_inits: LocalInits,

    // This is a list of flags for wasm features which are used to gate various
    // instructions.
    pub(crate) features: WasmFeatures,

    // Temporary storage used during `match_stack_operands`
    popped_types_tmp: Vec<MaybeType>,

    /// The `control` list is the list of blocks that we're currently in.
    control: Vec<Frame>,
    /// The `operands` is the current type stack.
    operands: Vec<MaybeType>,

    /// Whether validation is happening in a shared context.
    shared: bool,

    /// A trace of all operand push/pop operations performed while validating an
    /// opcode. This is then compared to the arity that we report to double
    /// check that arity report's correctness. `true` is "push" and `false` is
    /// "pop".
    #[cfg(debug_assertions)]
    pub(crate) pop_push_log: Vec<bool>,
}

/// Captures the initialization of non-defaultable locals.
struct LocalInits {
    /// Records if a local is already initialized.
    local_inits: Vec<bool>,
    /// When `local_inits` is modified, the relevant `index` is recorded
    /// here to be undone when control pops.
    inits: Vec<u32>,
    /// The index of the first non-defaultable local.
    ///
    /// # Note
    ///
    /// This is an optimization so that we only have to perform expensive
    /// look-ups for locals that have a local index equal to or higher than this.
    first_non_default_local: u32,
}

impl Default for LocalInits {
    fn default() -> Self {
        Self {
            local_inits: Vec::default(),
            inits: Vec::default(),
            first_non_default_local: u32::MAX,
        }
    }
}

impl LocalInits {
    /// Defines new function local parameters.
    pub fn define_params(&mut self, count: usize) {
        let Some(new_len) = self.local_inits.len().checked_add(count) else {
            panic!("tried to define too many function locals as parameters: {count}");
        };
        self.local_inits.resize(new_len, true);
    }

    /// Defines `count` function locals of type `ty`.
    pub fn define_locals(&mut self, count: u32, ty: ValType) {
        let Ok(count) = usize::try_from(count) else {
            panic!("tried to define too many function locals: {count}");
        };
        let len = self.local_inits.len();
        let Some(new_len) = len.checked_add(count) else {
            panic!("tried to define too many function locals: {count}");
        };
        let is_defaultable = ty.is_defaultable();
        if !is_defaultable && self.first_non_default_local == u32::MAX {
            self.first_non_default_local = len as u32;
        }
        self.local_inits.resize(new_len, is_defaultable);
    }

    /// Returns `true` if the local at `local_index` has already been initialized.
    #[inline]
    pub fn is_uninit(&self, local_index: u32) -> bool {
        if local_index < self.first_non_default_local {
            return false;
        }
        !self.local_inits[local_index as usize]
    }

    /// Marks the local at `local_index` as initialized.
    #[inline]
    pub fn set_init(&mut self, local_index: u32) {
        if self.is_uninit(local_index) {
            self.local_inits[local_index as usize] = true;
            self.inits.push(local_index);
        }
    }

    /// Registers a new control frame and returns its `height`.
    pub fn push_ctrl(&mut self) -> usize {
        self.inits.len()
    }

    /// Pops a control frame via its `height`.
    ///
    /// This uninitializes all locals that have been initialized within it.
    pub fn pop_ctrl(&mut self, height: usize) {
        for local_index in self.inits.split_off(height) {
            self.local_inits[local_index as usize] = false;
        }
    }

    /// Clears the [`LocalInits`].
    ///
    /// After this operation `self` will be empty and ready for reuse.
    pub fn clear(&mut self) {
        self.local_inits.clear();
        self.inits.clear();
        self.first_non_default_local = u32::MAX;
    }

    /// Returns `true` if `self` is empty.
    pub fn is_empty(&self) -> bool {
        self.local_inits.is_empty()
    }
}

// No science was performed in the creation of this number, feel free to change
// it if you so like.
const MAX_LOCALS_TO_TRACK: u32 = 50;

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
    uncached: Vec<(u32, ValType)>,
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
    local_inits: LocalInits,
    locals_first: Vec<ValType>,
    locals_uncached: Vec<(u32, ValType)>,
}

/// Type storage within the validator.
///
/// When managing the operand stack in unreachable code, the validator may not
/// fully know an operand's type. this unknown state is known as the `bottom`
/// type in the WebAssembly specification. Validating further instructions may
/// give us more information; either partial (`PartialRef`) or fully known.
#[derive(Debug, Copy, Clone)]
enum MaybeType<T = ValType> {
    /// The operand has no available type information due to unreachable code.
    ///
    /// This state represents "unknown" and corresponds to the `bottom` type in
    /// the WebAssembly specification. There are no constraints on what this
    /// type may be and it can match any other type during validation.
    Bottom,
    /// The operand is known to be a reference and we may know its abstract
    /// type.
    ///
    /// This state is not fully `Known`, however, because its type can be
    /// interpreted as either:
    /// - `shared` or not-`shared`
    /// -  nullable or not nullable
    ///
    /// No further refinements are required for WebAssembly instructions today
    /// but this may grow in the future.
    UnknownRef(Option<AbstractHeapType>),
    /// The operand is known to have type `T`.
    Known(T),
}

// The validator is pretty performance-sensitive and `MaybeType` is the main
// unit of storage, so assert that it doesn't exceed 4 bytes which is the
// current expected size.
#[test]
fn assert_maybe_type_small() {
    assert!(core::mem::size_of::<MaybeType>() == 8);
}

impl core::fmt::Display for MaybeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MaybeType::Bottom => write!(f, "bot"),
            MaybeType::UnknownRef(ty) => {
                write!(f, "(ref shared? ")?;
                match ty {
                    Some(ty) => write!(f, "{}bot", ty.as_str(true))?,
                    None => write!(f, "bot")?,
                }
                write!(f, ")")
            }
            MaybeType::Known(ty) => core::fmt::Display::fmt(ty, f),
        }
    }
}

impl From<ValType> for MaybeType {
    fn from(ty: ValType) -> MaybeType {
        MaybeType::Known(ty)
    }
}

impl From<RefType> for MaybeType {
    fn from(ty: RefType) -> MaybeType {
        let ty: ValType = ty.into();
        ty.into()
    }
}
impl From<MaybeType<RefType>> for MaybeType<ValType> {
    fn from(ty: MaybeType<RefType>) -> MaybeType<ValType> {
        match ty {
            MaybeType::Bottom => MaybeType::Bottom,
            MaybeType::UnknownRef(ty) => MaybeType::UnknownRef(ty),
            MaybeType::Known(t) => MaybeType::Known(t.into()),
        }
    }
}

impl MaybeType<RefType> {
    fn as_non_null(&self) -> MaybeType<RefType> {
        match self {
            MaybeType::Bottom => MaybeType::Bottom,
            MaybeType::UnknownRef(ty) => MaybeType::UnknownRef(*ty),
            MaybeType::Known(ty) => MaybeType::Known(ty.as_non_null()),
        }
    }

    fn is_maybe_shared(&self, resources: &impl WasmModuleResources) -> Option<bool> {
        match self {
            MaybeType::Bottom => None,
            MaybeType::UnknownRef(_) => None,
            MaybeType::Known(ty) => Some(resources.is_shared(*ty)),
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
            locals_first,
            locals_uncached,
        } = allocs;
        debug_assert!(popped_types_tmp.is_empty());
        debug_assert!(control.is_empty());
        debug_assert!(operands.is_empty());
        debug_assert!(local_inits.is_empty());
        debug_assert!(local_inits.is_empty());
        debug_assert!(locals_first.is_empty());
        debug_assert!(locals_uncached.is_empty());
        OperatorValidator {
            locals: Locals {
                num_locals: 0,
                first: locals_first,
                uncached: locals_uncached,
            },
            local_inits,
            features: *features,
            popped_types_tmp,
            operands,
            control,
            shared: false,
            #[cfg(debug_assertions)]
            pop_push_log: vec![],
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

        // Retrieve the function's type via index (`ty`); the `offset` is
        // necessary due to `sub_type_at`'s error messaging.
        let sub_ty = OperatorValidatorTemp {
            offset,
            inner: &mut ret,
            resources,
        }
        .sub_type_at(ty)?;

        // Set up the function's locals.
        if let CompositeInnerType::Func(func_ty) = &sub_ty.composite_type.inner {
            for ty in func_ty.params() {
                ret.locals.define(1, *ty);
            }
            ret.local_inits.define_params(func_ty.params().len());
        } else {
            bail!(offset, "expected func type at index {ty}, found {sub_ty}")
        }

        // If we're in a shared function, ensure we do not access unshared
        // objects.
        if sub_ty.composite_type.shared {
            ret.shared = true;
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
        self.local_inits.define_locals(count, ty);
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
            MaybeType::Known(t) => Some(*t),
            MaybeType::Bottom | MaybeType::UnknownRef(..) => None,
        })
    }

    /// Returns the number of frames on the control flow stack.
    pub fn control_stack_height(&self) -> usize {
        self.control.len()
    }

    /// Validates a relative jump to the `depth` specified.
    ///
    /// Returns the type signature of the block that we're jumping to as well
    /// as the kind of block if the jump is valid. Otherwise returns an error.
    pub(crate) fn jump(&self, depth: u32) -> Option<(BlockType, FrameKind)> {
        assert!(!self.control.is_empty());
        let i = (self.control.len() - 1).checked_sub(depth as usize)?;
        let frame = &self.control[i];
        Some((frame.block_type, frame.kind))
    }

    pub fn get_frame(&self, depth: usize) -> Option<&Frame> {
        self.control.iter().rev().nth(depth)
    }

    /// Create a temporary [`OperatorValidatorTemp`] for validation.
    pub fn with_resources<'a, 'validator, 'resources, T>(
        &'validator mut self,
        resources: &'resources T,
        offset: usize,
    ) -> impl VisitOperator<'a, Output = Result<()>> + ModuleArity + FrameStack + 'validator
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

    /// Same as `with_resources` above but guarantees it's able to visit simd
    /// operators as well.
    #[cfg(feature = "simd")]
    pub fn with_resources_simd<'a, 'validator, 'resources, T>(
        &'validator mut self,
        resources: &'resources T,
        offset: usize,
    ) -> impl VisitSimdOperator<'a, Output = Result<()>> + ModuleArity + 'validator
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

    pub fn into_allocations(mut self) -> OperatorValidatorAllocations {
        fn clear<T>(mut tmp: Vec<T>) -> Vec<T> {
            tmp.clear();
            tmp
        }
        OperatorValidatorAllocations {
            popped_types_tmp: clear(self.popped_types_tmp),
            control: clear(self.control),
            operands: clear(self.operands),
            local_inits: {
                self.local_inits.clear();
                self.local_inits
            },
            locals_first: clear(self.locals.first),
            locals_uncached: clear(self.locals.uncached),
        }
    }

    fn record_pop(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.pop_push_log.push(false);
        }
    }

    fn record_push(&mut self) {
        #[cfg(debug_assertions)]
        {
            self.pop_push_log.push(true);
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
                MaybeType::Known(ValType::Ref(r)) => match r.heap_type() {
                    HeapType::Concrete(index) | HeapType::Exact(index) => {
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
        self.record_push();
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

    fn push_exact_ref(&mut self, nullable: bool, type_index: u32) -> Result<()> {
        let mut heap_ty = HeapType::Exact(UnpackedIndex::Module(type_index));

        // Canonicalize the module index into an id.
        self.resources.check_heap_type(&mut heap_ty, self.offset)?;
        debug_assert!(matches!(heap_ty, HeapType::Exact(UnpackedIndex::Id(_))));

        let ref_ty = RefType::new(nullable, heap_ty).ok_or_else(|| {
            format_err!(self.offset, "implementation limit: type index too large")
        })?;

        self.push_operand(ref_ty)
    }

    fn push_exact_ref_if_available(&mut self, nullable: bool, type_index: u32) -> Result<()> {
        if self.features.custom_descriptors() {
            self.push_exact_ref(nullable, type_index)
        } else {
            self.push_concrete_ref(nullable, type_index)
        }
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

    fn pop_concrete_or_exact_ref(
        &mut self,
        nullable: bool,
        type_index: u32,
    ) -> Result<(MaybeType, bool)> {
        let ty = self.pop_concrete_ref(nullable, type_index)?;
        let is_exact = match ty {
            MaybeType::Known(ValType::Ref(rt)) if rt.is_exact_type_ref() || rt.is_none_ref() => {
                let mut heap_ty = HeapType::Exact(UnpackedIndex::Module(type_index));
                self.resources.check_heap_type(&mut heap_ty, self.offset)?;
                let expected = RefType::new(nullable, heap_ty).ok_or_else(|| {
                    format_err!(self.offset, "implementation limit: type index too large")
                })?;
                self.resources.is_subtype(rt.into(), expected.into())
            }
            MaybeType::Bottom => true,
            _ => false,
        };
        Ok((ty, is_exact))
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
            Some(MaybeType::Known(actual_ty)) => {
                if Some(actual_ty) == expected {
                    if let Some(control) = self.control.last() {
                        if self.operands.len() >= control.height {
                            self.record_pop();
                            return Ok(MaybeType::Known(actual_ty));
                        }
                    }
                }
                Some(MaybeType::Known(actual_ty))
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
        let control = self.control.last().unwrap();
        let actual = if self.operands.len() == control.height && control.unreachable {
            MaybeType::Bottom
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
                (MaybeType::Bottom, _) => {}

                // The "heap bottom" type only matches other references types,
                // but not any integer types. Note that if the heap bottom is
                // known to have a specific abstract heap type then a subtype
                // check is performed against hte expected type.
                (MaybeType::UnknownRef(actual_ty), ValType::Ref(expected)) => {
                    if let Some(actual) = actual_ty {
                        let expected_shared = self.resources.is_shared(expected);
                        let actual = RefType::new(
                            false,
                            HeapType::Abstract {
                                shared: expected_shared,
                                ty: actual,
                            },
                        )
                        .unwrap();
                        if !self.resources.is_subtype(actual.into(), expected.into()) {
                            bail!(
                                self.offset,
                                "type mismatch: expected {}, found {}",
                                ty_to_str(expected.into()),
                                ty_to_str(actual.into())
                            );
                        }
                    }
                }

                // Use the `is_subtype` predicate to test if a found type matches
                // the expectation.
                (MaybeType::Known(actual), expected) => {
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
                    MaybeType::UnknownRef(..),
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
        self.record_pop();
        Ok(actual)
    }

    /// Match expected vs. actual operand.
    fn match_operand(
        &mut self,
        actual: ValType,
        expected: ValType,
    ) -> Result<(), BinaryReaderError> {
        self.push_operand(actual)?;
        self.pop_operand(Some(expected))?;
        Ok(())
    }

    /// Match a type sequence to the top of the stack.
    fn match_stack_operands(
        &mut self,
        expected_tys: impl PreciseIterator<Item = ValType> + 'resources,
    ) -> Result<()> {
        let mut popped_types_tmp = mem::take(&mut self.popped_types_tmp);
        debug_assert!(popped_types_tmp.is_empty());
        popped_types_tmp.reserve(expected_tys.len());

        for expected_ty in expected_tys.rev() {
            let actual_ty = self.pop_operand(Some(expected_ty))?;
            popped_types_tmp.push(actual_ty);
        }
        for ty in popped_types_tmp.drain(..).rev() {
            self.push_operand(ty)?;
        }

        debug_assert!(self.popped_types_tmp.is_empty());
        self.popped_types_tmp = popped_types_tmp;
        Ok(())
    }

    /// Pop a reference type from the operand stack.
    fn pop_ref(&mut self, expected: Option<RefType>) -> Result<MaybeType<RefType>> {
        match self.pop_operand(expected.map(|t| t.into()))? {
            MaybeType::Bottom => Ok(MaybeType::UnknownRef(None)),
            MaybeType::UnknownRef(ty) => Ok(MaybeType::UnknownRef(ty)),
            MaybeType::Known(ValType::Ref(rt)) => Ok(MaybeType::Known(rt)),
            MaybeType::Known(ty) => bail!(
                self.offset,
                "type mismatch: expected ref but found {}",
                ty_to_str(ty)
            ),
        }
    }

    /// Pop a reference type from the operand stack, checking if it is a subtype
    /// of a nullable type of `expected` or the shared version of `expected`.
    ///
    /// This function returns the popped reference type and its `shared`-ness,
    /// saving extra lookups for concrete types.
    fn pop_maybe_shared_ref(&mut self, expected: AbstractHeapType) -> Result<MaybeType<RefType>> {
        let actual = match self.pop_ref(None)? {
            MaybeType::Bottom => return Ok(MaybeType::Bottom),
            MaybeType::UnknownRef(None) => return Ok(MaybeType::UnknownRef(None)),
            MaybeType::UnknownRef(Some(actual)) => {
                if !actual.is_subtype_of(expected) {
                    bail!(
                        self.offset,
                        "type mismatch: expected subtype of {}, found {}",
                        expected.as_str(false),
                        actual.as_str(false),
                    )
                }
                return Ok(MaybeType::UnknownRef(Some(actual)));
            }
            MaybeType::Known(ty) => ty,
        };
        // Change our expectation based on whether we're dealing with an actual
        // shared or unshared type.
        let is_actual_shared = self.resources.is_shared(actual);
        let expected = RefType::new(
            true,
            HeapType::Abstract {
                shared: is_actual_shared,
                ty: expected,
            },
        )
        .unwrap();

        // Check (again) that the actual type is a subtype of the expected type.
        // Note that `_pop_operand` already does this kind of thing but we leave
        // that for a future refactoring (TODO).
        if !self.resources.is_subtype(actual.into(), expected.into()) {
            bail!(
                self.offset,
                "type mismatch: expected subtype of {expected}, found {actual}",
            )
        }
        Ok(MaybeType::Known(actual))
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
        let control = self.control.last_mut().unwrap();
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
        let init_height = self.local_inits.push_ctrl();
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
        let frame = self.control.last().unwrap();
        let ty = frame.block_type;
        let height = frame.height;
        let init_height = frame.init_height;

        // reset_locals in the spec
        self.local_inits.pop_ctrl(init_height);

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
        match self.inner.jump(depth) {
            Some(tup) => Ok(tup),
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
            bail!(
                self.offset,
                "invalid memop alignment: alignment must not be larger than natural"
            );
        }
        if index_ty == ValType::I32 && memarg.offset > u64::from(u32::MAX) {
            bail!(self.offset, "offset out of range: must be <= 2**32");
        }
        Ok(index_ty)
    }

    fn check_floats_enabled(&self) -> Result<()> {
        if !self.features.floats() {
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

    /// Validates a block type, primarily with various in-flight proposals.
    fn check_block_type(&self, ty: &mut BlockType) -> Result<()> {
        match ty {
            BlockType::Empty => Ok(()),
            BlockType::Type(t) => self
                .resources
                .check_value_type(t, &self.features, self.offset),
            BlockType::FuncType(idx) => {
                if !self.features.multi_value() {
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

    /// Returns the corresponding function type for the `func` item located at
    /// `function_index`.
    fn type_of_function(&self, function_index: u32) -> Result<&'resources FuncType> {
        if let Some(type_index) = self.resources.type_index_of_function(function_index) {
            self.func_type_at(type_index)
        } else {
            bail!(
                self.offset,
                "unknown function {function_index}: function index out of bounds",
            )
        }
    }

    /// Checks a call-style instruction which will be invoking the function `ty`
    /// specified.
    ///
    /// This will pop parameters from the operand stack for the function's
    /// parameters and then push the results of the function on the stack.
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

    /// Similar to `check_call_ty` except used for tail-call instructions.
    fn check_return_call_ty(&mut self, ty: &FuncType) -> Result<()> {
        self.check_func_type_same_results(ty)?;
        for &ty in ty.params().iter().rev() {
            debug_assert_type_indices_are_ids(ty);
            self.pop_operand(Some(ty))?;
        }

        // Match the results with this function's.
        for &ty in ty.results() {
            debug_assert_type_indices_are_ids(ty);
            self.push_operand(ty)?;
        }
        self.check_return()?;

        Ok(())
    }

    /// Checks the immediate `type_index` of a `call_ref`-style instruction
    /// (also `return_call_ref`).
    ///
    /// This will validate that the value on the stack is a `(ref type_index)`
    /// or a subtype. This will then return the corresponding function type used
    /// for this call (to be used with `check_call_ty` or
    /// `check_return_call_ty`).
    fn check_call_ref_ty(&mut self, type_index: u32) -> Result<&'resources FuncType> {
        let unpacked_index = UnpackedIndex::Module(type_index);
        let mut hty = HeapType::Concrete(unpacked_index);
        self.resources.check_heap_type(&mut hty, self.offset)?;
        let expected = RefType::new(true, hty).expect("hty should be previously validated");
        self.pop_ref(Some(expected))?;
        self.func_type_at(type_index)
    }

    /// Validates the immediate operands of a `call_indirect` or
    /// `return_call_indirect` instruction.
    ///
    /// This will validate that `table_index` is valid and a funcref table. It
    /// will additionally pop the index argument which is used to index into the
    /// table.
    ///
    /// The return value of this function is the function type behind
    /// `type_index` which must then be passed to `check_{call,return_call}_ty`.
    fn check_call_indirect_ty(
        &mut self,
        type_index: u32,
        table_index: u32,
    ) -> Result<&'resources FuncType> {
        let tab = self.table_type_at(table_index)?;
        if !self
            .resources
            .is_subtype(ValType::Ref(tab.element_type), ValType::FUNCREF)
        {
            bail!(
                self.offset,
                "type mismatch: indirect calls must go through a table with type <= funcref",
            );
        }
        self.pop_operand(Some(tab.index_type()))?;
        self.func_type_at(type_index)
    }

    /// Validates a `return` instruction, popping types from the operand
    /// stack that the function needs.
    fn check_return(&mut self) -> Result<()> {
        assert!(!self.control.is_empty());
        for ty in self.results(self.control[0].block_type)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.unreachable()?;
        Ok(())
    }

    /// Check that the given type has the same result types as the current
    /// function's results.
    fn check_func_type_same_results(&self, callee_ty: &FuncType) -> Result<()> {
        assert!(!self.control.is_empty());
        let caller_rets = self.results(self.control[0].block_type)?;
        if callee_ty.results().len() != caller_rets.len()
            || !caller_rets
                .zip(callee_ty.results())
                .all(|(caller_ty, callee_ty)| self.resources.is_subtype(*callee_ty, caller_ty))
        {
            let caller_rets = self
                .results(self.control[0].block_type)?
                .map(|ty| format!("{ty}"))
                .collect::<Vec<_>>()
                .join(" ");
            let callee_rets = callee_ty
                .results()
                .iter()
                .map(|ty| format!("{ty}"))
                .collect::<Vec<_>>()
                .join(" ");
            bail!(
                self.offset,
                "type mismatch: current function requires result type \
                 [{caller_rets}] but callee returns [{callee_rets}]"
            );
        }
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

    /// Checks the validity of a common float conversion operator.
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

    /// Checks the validity of atomic binary operator on memory.
    fn check_atomic_binary_memory_op(&mut self, memarg: MemArg, op_ty: ValType) -> Result<()> {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(op_ty))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(op_ty)?;
        Ok(())
    }

    /// Checks the validity of an atomic compare exchange operator on memories.
    fn check_atomic_binary_memory_cmpxchg(&mut self, memarg: MemArg, op_ty: ValType) -> Result<()> {
        let ty = self.check_shared_memarg(memarg)?;
        self.pop_operand(Some(op_ty))?;
        self.pop_operand(Some(op_ty))?;
        self.pop_operand(Some(ty))?;
        self.push_operand(op_ty)?;
        Ok(())
    }

    /// Common helper for `ref.test` and `ref.cast` downcasting/checking
    /// instructions. Returns the given `heap_type` as a `ValType`.
    fn check_downcast(&mut self, nullable: bool, mut heap_type: HeapType) -> Result<RefType> {
        self.resources
            .check_heap_type(&mut heap_type, self.offset)?;

        let sub_ty = RefType::new(nullable, heap_type).ok_or_else(|| {
            BinaryReaderError::new("implementation limit: type index too large", self.offset)
        })?;
        let sup_ty = RefType::new(true, self.resources.top_type(&heap_type))
            .expect("can't panic with non-concrete heap types");

        self.pop_ref(Some(sup_ty))?;
        Ok(sub_ty)
    }

    /// Common helper for both nullable and non-nullable variants of `ref.test`
    /// instructions.
    fn check_ref_test(&mut self, nullable: bool, heap_type: HeapType) -> Result<()> {
        self.check_downcast(nullable, heap_type)?;
        self.push_operand(ValType::I32)
    }

    /// Common helper for both nullable and non-nullable variants of `ref.cast`
    /// instructions.
    fn check_ref_cast(&mut self, nullable: bool, heap_type: HeapType) -> Result<()> {
        let sub_ty = self.check_downcast(nullable, heap_type)?;
        self.push_operand(sub_ty)
    }

    /// Common helper to check type hierarchy for `br_on_cast` operators.
    fn check_br_on_cast_type_hierarchy(
        &self,
        from_ref_type: RefType,
        to_ref_type: RefType,
    ) -> Result<()> {
        if self.features.custom_descriptors() {
            // The constraint C |- rt_2 <: rt_1 on branching cast instructions
            // before the custom descriptors proposal is relaxed to the constraint
            // that rt_1 and rt_2 share some arbitrary valid supertype rt', i.e.
            // that rt_1 and rt_2 must be in the same heap type hierarchy.
            let from_ref_type_top = self.resources.top_type(&from_ref_type.heap_type());
            let to_ref_type_top = self.resources.top_type(&to_ref_type.heap_type());
            if from_ref_type_top != to_ref_type_top {
                bail!(
                    self.offset,
                    "type mismatch: {from_ref_type} and {to_ref_type} have different heap type hierarchies"
                );
            }
            return Ok(());
        }

        if !self
            .resources
            .is_subtype(to_ref_type.into(), from_ref_type.into())
        {
            bail!(
                self.offset,
                "type mismatch: expected {from_ref_type}, found {to_ref_type}"
            );
        }
        Ok(())
    }

    /// Common helper to check descriptor for the specified type.
    fn check_descriptor(&self, heap_type: HeapType) -> Result<u32> {
        Ok(match heap_type {
            HeapType::Exact(idx) | HeapType::Concrete(idx) => {
                if let Some(descriptor_idx) = self
                    .sub_type_at(idx.as_module_index().unwrap())?
                    .composite_type
                    .descriptor_idx
                {
                    u32::try_from(crate::validator::types::TypeIdentifier::index(
                        &descriptor_idx.as_core_type_id().unwrap(),
                    ))
                    .unwrap()
                } else {
                    bail!(self.offset, "cast target must have descriptor")
                }
            }
            _ => bail!(self.offset, "unexpected heap type"),
        })
    }

    fn check_maybe_exact_descriptor_ref(&mut self, heap_type: HeapType) -> Result<bool> {
        let descriptor_idx = self.check_descriptor(heap_type)?;
        let (ty, _is_exact) = self.pop_concrete_or_exact_ref(true, descriptor_idx)?;
        let is_exact = if let HeapType::Exact(_) = heap_type {
            let mut descriptor_ty = HeapType::Exact(UnpackedIndex::Module(descriptor_idx));
            self.resources
                .check_heap_type(&mut descriptor_ty, self.offset)?;
            let descriptor_ty = ValType::Ref(
                RefType::new(true, descriptor_ty)
                    .expect("existing heap types should be within our limits"),
            );

            match ty {
                MaybeType::Known(actual) if !self.resources.is_subtype(actual, descriptor_ty) => {
                    bail!(
                        self.offset,
                        "type mismatch: expected descriptor of exact type {descriptor_ty} found {actual}",
                    );
                }
                _ => (),
            }
            true
        } else {
            false
        };
        Ok(is_exact)
    }

    /// Common helper for both nullable and non-nullable variants of `ref.cast_desc`
    /// instructions.
    fn check_ref_cast_desc(&mut self, nullable: bool, heap_type: HeapType) -> Result<()> {
        let is_exact = self.check_maybe_exact_descriptor_ref(heap_type)?;

        self.check_downcast(nullable, heap_type)?;

        let idx = {
            let mut heap_type = heap_type;
            self.resources
                .check_heap_type(&mut heap_type, self.offset)?;
            match heap_type {
                HeapType::Concrete(index) | HeapType::Exact(index) => {
                    index.pack().ok_or_else(|| {
                        BinaryReaderError::new(
                            "implementation limit: type index too large",
                            self.offset,
                        )
                    })?
                }
                _ => panic!(),
            }
        };

        self.push_operand(if is_exact {
            RefType::exact(nullable, idx)
        } else {
            RefType::concrete(nullable, idx)
        })
    }

    /// Common helper for checking the types of globals accessed with atomic RMW
    /// instructions, which only allow `i32` and `i64`.
    fn check_atomic_global_rmw_ty(&self, global_index: u32) -> Result<ValType> {
        let ty = self.global_type_at(global_index)?.content_type;
        if !(ty == ValType::I32 || ty == ValType::I64) {
            bail!(
                self.offset,
                "invalid type: `global.atomic.rmw.*` only allows `i32` and `i64`"
            );
        }
        Ok(ty)
    }

    /// Common helper for checking the types of structs accessed with atomic RMW
    /// instructions, which only allow `i32` and `i64` types.
    fn check_struct_atomic_rmw(
        &mut self,
        op: &'static str,
        struct_type_index: u32,
        field_index: u32,
    ) -> Result<()> {
        let field = self.mutable_struct_field_at(struct_type_index, field_index)?;
        let field_ty = match field.element_type {
            StorageType::Val(ValType::I32) => ValType::I32,
            StorageType::Val(ValType::I64) => ValType::I64,
            _ => bail!(
                self.offset,
                "invalid type: `struct.atomic.rmw.{}` only allows `i32` and `i64`",
                op
            ),
        };
        self.pop_operand(Some(field_ty))?;
        self.pop_concrete_ref(true, struct_type_index)?;
        self.push_operand(field_ty)?;
        Ok(())
    }

    /// Common helper for checking the types of arrays accessed with atomic RMW
    /// instructions, which only allow `i32` and `i64`.
    fn check_array_atomic_rmw(&mut self, op: &'static str, type_index: u32) -> Result<()> {
        let field = self.mutable_array_type_at(type_index)?;
        let elem_ty = match field.element_type {
            StorageType::Val(ValType::I32) => ValType::I32,
            StorageType::Val(ValType::I64) => ValType::I64,
            _ => bail!(
                self.offset,
                "invalid type: `array.atomic.rmw.{}` only allows `i32` and `i64`",
                op
            ),
        };
        self.pop_operand(Some(elem_ty))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        self.push_operand(elem_ty)?;
        Ok(())
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
        if let CompositeInnerType::Struct(struct_ty) = &sub_ty.composite_type.inner {
            if self.inner.shared && !sub_ty.composite_type.shared {
                bail!(
                    self.offset,
                    "shared functions cannot access unshared structs",
                );
            }
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

    fn mutable_struct_field_at(
        &self,
        struct_type_index: u32,
        field_index: u32,
    ) -> Result<FieldType> {
        let field = self.struct_field_at(struct_type_index, field_index)?;
        if !field.mutable {
            bail!(
                self.offset,
                "invalid struct modification: struct field is immutable"
            )
        }
        Ok(field)
    }

    fn array_type_at(&self, at: u32) -> Result<FieldType> {
        let sub_ty = self.sub_type_at(at)?;
        if let CompositeInnerType::Array(array_ty) = &sub_ty.composite_type.inner {
            if self.inner.shared && !sub_ty.composite_type.shared {
                bail!(
                    self.offset,
                    "shared functions cannot access unshared arrays",
                );
            }
            Ok(array_ty.0)
        } else {
            bail!(
                self.offset,
                "expected array type at index {at}, found {sub_ty}"
            )
        }
    }

    fn mutable_array_type_at(&self, at: u32) -> Result<FieldType> {
        let field = self.array_type_at(at)?;
        if !field.mutable {
            bail!(
                self.offset,
                "invalid array modification: array is immutable"
            )
        }
        Ok(field)
    }

    fn func_type_at(&self, at: u32) -> Result<&'resources FuncType> {
        let sub_ty = self.sub_type_at(at)?;
        if let CompositeInnerType::Func(func_ty) = &sub_ty.composite_type.inner {
            if self.inner.shared && !sub_ty.composite_type.shared {
                bail!(
                    self.offset,
                    "shared functions cannot access unshared functions",
                );
            }
            Ok(func_ty)
        } else {
            bail!(
                self.offset,
                "expected func type at index {at}, found {sub_ty}"
            )
        }
    }

    fn cont_type_at(&self, at: u32) -> Result<&ContType> {
        let sub_ty = self.sub_type_at(at)?;
        if let CompositeInnerType::Cont(cont_ty) = &sub_ty.composite_type.inner {
            if self.inner.shared && !sub_ty.composite_type.shared {
                bail!(
                    self.offset,
                    "shared continuations cannot access unshared continuations",
                );
            }
            Ok(cont_ty)
        } else {
            bail!(self.offset, "non-continuation type {at}",)
        }
    }

    fn func_type_of_cont_type(&self, cont_ty: &ContType) -> &'resources FuncType {
        let func_id = cont_ty.0.as_core_type_id().expect("valid core type id");
        self.resources.sub_type_at_id(func_id).unwrap_func()
    }

    fn tag_at(&self, at: u32) -> Result<&'resources FuncType> {
        self.resources
            .tag_at(at)
            .ok_or_else(|| format_err!(self.offset, "unknown tag {}: tag index out of bounds", at))
    }

    // Similar to `tag_at`, but checks that the result type is
    // empty. This is necessary when enabling the stack switching
    // feature as it allows non-empty result types on tags.
    fn exception_tag_at(&self, at: u32) -> Result<&'resources FuncType> {
        let func_ty = self.tag_at(at)?;
        if func_ty.results().len() != 0 {
            bail!(
                self.offset,
                "invalid exception type: non-empty tag result type"
            );
        }
        Ok(func_ty)
    }

    fn global_type_at(&self, at: u32) -> Result<GlobalType> {
        if let Some(ty) = self.resources.global_at(at) {
            if self.inner.shared && !ty.shared {
                bail!(
                    self.offset,
                    "shared functions cannot access unshared globals",
                );
            }
            Ok(ty)
        } else {
            bail!(self.offset, "unknown global: global index out of bounds");
        }
    }

    /// Validates that the `table` is valid and returns the type it points to.
    fn table_type_at(&self, table: u32) -> Result<TableType> {
        match self.resources.table_at(table) {
            Some(ty) => {
                if self.inner.shared && !ty.shared {
                    bail!(
                        self.offset,
                        "shared functions cannot access unshared tables",
                    );
                }
                Ok(ty)
            }
            None => bail!(
                self.offset,
                "unknown table {table}: table index out of bounds"
            ),
        }
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

    fn check_data_segment(&self, data_index: u32) -> Result<()> {
        match self.resources.data_count() {
            None => bail!(self.offset, "data count section required"),
            Some(count) if data_index < count => Ok(()),
            Some(_) => bail!(self.offset, "unknown data segment {data_index}"),
        }
    }

    fn check_resume_table(
        &mut self,
        table: ResumeTable,
        type_index: u32, // The type index annotation on the `resume` instruction, which `table` appears on.
    ) -> Result<&'resources FuncType> {
        let cont_ty = self.cont_type_at(type_index)?;
        // ts1 -> ts2
        let old_func_ty = self.func_type_of_cont_type(cont_ty);
        for handle in table.handlers {
            match handle {
                Handle::OnLabel { tag, label } => {
                    // ts1' -> ts2'
                    let tag_ty = self.tag_at(tag)?;
                    // ts1'' (ref (cont $ft))
                    let block = self.jump(label)?;
                    // Pop the continuation reference.
                    match self.label_types(block.0, block.1)?.last() {
                        Some(ValType::Ref(rt)) if rt.is_concrete_type_ref() => {
                            let sub_ty = self.resources.sub_type_at_id(
                                rt.type_index()
                                    .unwrap()
                                    .as_core_type_id()
                                    .expect("canonicalized index"),
                            );
                            let new_cont = if let CompositeInnerType::Cont(cont) =
                                &sub_ty.composite_type.inner
                            {
                                cont
                            } else {
                                bail!(self.offset, "non-continuation type");
                            };
                            let new_func_ty = self.func_type_of_cont_type(&new_cont);
                            // Check that (ts2' -> ts2) <: $ft
                            if new_func_ty.params().len() != tag_ty.results().len()
                                || !self.is_subtype_many(new_func_ty.params(), tag_ty.results())
                                || old_func_ty.results().len() != new_func_ty.results().len()
                                || !self
                                    .is_subtype_many(old_func_ty.results(), new_func_ty.results())
                            {
                                bail!(self.offset, "type mismatch in continuation type")
                            }
                            let expected_nargs = tag_ty.params().len() + 1;
                            let actual_nargs = self.label_types(block.0, block.1)?.len();
                            if actual_nargs != expected_nargs {
                                bail!(
                                    self.offset,
                                    "type mismatch: expected {expected_nargs} label result(s), but label is annotated with {actual_nargs} results"
                                )
                            }

                            let labeltys =
                                self.label_types(block.0, block.1)?.take(expected_nargs - 1);

                            // Check that ts1'' <: ts1'.
                            for (tagty, &lblty) in labeltys.zip(tag_ty.params()) {
                                if !self.resources.is_subtype(lblty, tagty) {
                                    bail!(
                                        self.offset,
                                        "type mismatch between tag type and label type"
                                    )
                                }
                            }
                        }
                        Some(ty) => {
                            bail!(self.offset, "type mismatch: {}", ty_to_str(ty))
                        }
                        _ => bail!(
                            self.offset,
                            "type mismatch: instruction requires continuation reference type but label has none"
                        ),
                    }
                }
                Handle::OnSwitch { tag } => {
                    let tag_ty = self.tag_at(tag)?;
                    if tag_ty.params().len() != 0 {
                        bail!(self.offset, "type mismatch: non-empty tag parameter type")
                    }
                }
            }
        }
        Ok(old_func_ty)
    }

    /// Applies `is_subtype` pointwise two equally sized collections
    /// (i.e. equally sized after skipped elements).
    fn is_subtype_many(&mut self, ts1: &[ValType], ts2: &[ValType]) -> bool {
        debug_assert!(ts1.len() == ts2.len());
        ts1.iter()
            .zip(ts2.iter())
            .all(|(ty1, ty2)| self.resources.is_subtype(*ty1, *ty2))
    }

    fn check_binop128(&mut self) -> Result<()> {
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ValType::I64))?;
        self.push_operand(ValType::I64)?;
        self.push_operand(ValType::I64)?;
        Ok(())
    }

    fn check_i64_mul_wide(&mut self) -> Result<()> {
        self.pop_operand(Some(ValType::I64))?;
        self.pop_operand(Some(ValType::I64))?;
        self.push_operand(ValType::I64)?;
        self.push_operand(ValType::I64)?;
        Ok(())
    }

    fn check_enabled(&self, flag: bool, desc: &str) -> Result<()> {
        if flag {
            return Ok(());
        }
        bail!(self.offset, "{desc} support is not enabled");
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

#[cfg_attr(not(feature = "simd"), allow(unused_macro_rules))]
macro_rules! validate_proposal {
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) -> Result<()> {
                validate_proposal!(validate self $proposal / $op);
                self.0.$visit($( $($arg),* )?)
            }
        )*
    };

    (validate self mvp / $op:ident) => {};

    // These opcodes are handled specially below as they were introduced in the
    // bulk memory proposal but are gated by the `bulk_memory_opt`
    // "sub-proposal".
    (validate self $proposal:ident / MemoryFill) => {};
    (validate self $proposal:ident / MemoryCopy) => {};

    (validate $self:ident $proposal:ident / $op:ident) => {
        $self.0.check_enabled($self.0.features.$proposal(), validate_proposal!(desc $proposal))?
    };

    (desc simd) => ("SIMD");
    (desc relaxed_simd) => ("relaxed SIMD");
    (desc threads) => ("threads");
    (desc shared_everything_threads) => ("shared-everything-threads");
    (desc saturating_float_to_int) => ("saturating float to int conversions");
    (desc reference_types) => ("reference types");
    (desc bulk_memory) => ("bulk memory");
    (desc sign_extension) => ("sign extension operations");
    (desc exceptions) => ("exceptions");
    (desc tail_call) => ("tail calls");
    (desc function_references) => ("function references");
    (desc memory_control) => ("memory control");
    (desc gc) => ("gc");
    (desc legacy_exceptions) => ("legacy exceptions");
    (desc stack_switching) => ("stack switching");
    (desc wide_arithmetic) => ("wide arithmetic");
    (desc custom_descriptors) => ("custom descriptors operations");
}

impl<'a, T> VisitOperator<'a> for WasmProposalValidator<'_, '_, T>
where
    T: WasmModuleResources,
{
    type Output = Result<()>;

    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        Some(self)
    }

    crate::for_each_visit_operator!(validate_proposal);
}

#[cfg(feature = "simd")]
impl<'a, T> VisitSimdOperator<'a> for WasmProposalValidator<'_, '_, T>
where
    T: WasmModuleResources,
{
    crate::for_each_visit_simd_operator!(validate_proposal);
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

    #[cfg(feature = "simd")]
    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        Some(self)
    }

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
        let exn_type = ValType::from(RefType::EXN);
        for catch in ty.catches {
            match catch {
                Catch::One { tag, label } => {
                    let tag = self.exception_tag_at(tag)?;
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
                        self.match_operand(*actual, expected)?;
                    }
                }
                Catch::OneRef { tag, label } => {
                    let tag = self.exception_tag_at(tag)?;
                    let (ty, kind) = self.jump(label)?;
                    let tag_params = tag.params().iter().copied();
                    let label_types = self.label_types(ty, kind)?;
                    if tag_params.len() + 1 != label_types.len() {
                        bail!(
                            self.offset,
                            "type mismatch: catch_ref label must have one \
                             more type than tag types",
                        );
                    }
                    for (expected_label_type, actual_tag_param) in
                        label_types.zip(tag_params.chain([exn_type]))
                    {
                        self.match_operand(actual_tag_param, expected_label_type)?;
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
                    let ty = match (types.next(), types.next()) {
                        (Some(ty), None) => ty,
                        _ => {
                            bail!(
                                self.offset,
                                "type mismatch: catch_all_ref label must have \
                                 exactly one result type"
                            );
                        }
                    };
                    if !self.resources.is_subtype(exn_type, ty) {
                        bail!(
                            self.offset,
                            "type mismatch: catch_all_ref label must a \
                             subtype of (ref exn)"
                        );
                    }
                }
            }
        }
        self.push_ctrl(FrameKind::TryTable, ty.ty)?;
        Ok(())
    }
    fn visit_throw(&mut self, index: u32) -> Self::Output {
        // Check values associated with the exception.
        let ty = self.exception_tag_at(index)?;
        for ty in ty.clone().params().iter().rev() {
            self.pop_operand(Some(*ty))?;
        }
        // this should be validated when the tag was defined in the module
        debug_assert!(ty.results().is_empty());
        self.unreachable()?;
        Ok(())
    }
    fn visit_throw_ref(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::EXNREF))?;
        self.unreachable()?;
        Ok(())
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
        if self.control.is_empty() {
            assert_ne!(self.offset, 0);
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
            self.match_stack_operands(label_tys)?;
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
        let ty = self.type_of_function(function_index)?;
        self.check_call_ty(ty)?;
        Ok(())
    }
    fn visit_return_call(&mut self, function_index: u32) -> Self::Output {
        let ty = self.type_of_function(function_index)?;
        self.check_return_call_ty(ty)?;
        Ok(())
    }
    fn visit_call_ref(&mut self, type_index: u32) -> Self::Output {
        let ty = self.check_call_ref_ty(type_index)?;
        self.check_call_ty(ty)?;
        Ok(())
    }
    fn visit_return_call_ref(&mut self, type_index: u32) -> Self::Output {
        let ty = self.check_call_ref_ty(type_index)?;
        self.check_return_call_ty(ty)?;
        Ok(())
    }
    fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> Self::Output {
        let ty = self.check_call_indirect_ty(type_index, table_index)?;
        self.check_call_ty(ty)?;
        Ok(())
    }
    fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> Self::Output {
        let ty = self.check_call_indirect_ty(type_index, table_index)?;
        self.check_return_call_ty(ty)?;
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
            (MaybeType::UnknownRef(..), _)
            | (_, MaybeType::UnknownRef(..))
            | (MaybeType::Known(ValType::Ref(_)), _)
            | (_, MaybeType::Known(ValType::Ref(_))) => {
                bail!(
                    self.offset,
                    "type mismatch: select only takes integral types"
                )
            }

            // If one operand is the "bottom" type then whatever the other
            // operand is is the result of the `select`
            (MaybeType::Bottom, t) | (t, MaybeType::Bottom) => t,

            // Otherwise these are two integral types and they must match for
            // `select` to typecheck.
            (t @ MaybeType::Known(t1), MaybeType::Known(t2)) => {
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
    fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> Self::Output {
        debug_assert!(tys.len() != 1);
        bail!(self.offset, "invalid result arity");
    }
    fn visit_local_get(&mut self, local_index: u32) -> Self::Output {
        let ty = self.local(local_index)?;
        debug_assert_type_indices_are_ids(ty);
        if self.local_inits.is_uninit(local_index) {
            bail!(self.offset, "uninitialized local: {}", local_index);
        }
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_local_set(&mut self, local_index: u32) -> Self::Output {
        let ty = self.local(local_index)?;
        self.pop_operand(Some(ty))?;
        self.local_inits.set_init(local_index);
        Ok(())
    }
    fn visit_local_tee(&mut self, local_index: u32) -> Self::Output {
        let expected_ty = self.local(local_index)?;
        self.pop_operand(Some(expected_ty))?;
        self.local_inits.set_init(local_index);
        self.push_operand(expected_ty)?;
        Ok(())
    }
    fn visit_global_get(&mut self, global_index: u32) -> Self::Output {
        let ty = self.global_type_at(global_index)?.content_type;
        debug_assert_type_indices_are_ids(ty);
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_global_atomic_get(&mut self, _ordering: Ordering, global_index: u32) -> Self::Output {
        self.visit_global_get(global_index)?;
        // No validation of `ordering` is needed because `global.atomic.get` can
        // be used on both shared and unshared globals. But we do need to limit
        // which types can be used with this instruction.
        let ty = self.global_type_at(global_index)?.content_type;
        let supertype = RefType::ANYREF.into();
        if !(ty == ValType::I32 || ty == ValType::I64 || self.resources.is_subtype(ty, supertype)) {
            bail!(
                self.offset,
                "invalid type: `global.atomic.get` only allows `i32`, `i64` and subtypes of `anyref`"
            );
        }
        Ok(())
    }
    fn visit_global_set(&mut self, global_index: u32) -> Self::Output {
        let ty = self.global_type_at(global_index)?;
        if !ty.mutable {
            bail!(
                self.offset,
                "global is immutable: cannot modify it with `global.set`"
            );
        }
        self.pop_operand(Some(ty.content_type))?;
        Ok(())
    }
    fn visit_global_atomic_set(&mut self, _ordering: Ordering, global_index: u32) -> Self::Output {
        self.visit_global_set(global_index)?;
        // No validation of `ordering` is needed because `global.atomic.get` can
        // be used on both shared and unshared globals.
        let ty = self.global_type_at(global_index)?.content_type;
        let supertype = RefType::ANYREF.into();
        if !(ty == ValType::I32 || ty == ValType::I64 || self.resources.is_subtype(ty, supertype)) {
            bail!(
                self.offset,
                "invalid type: `global.atomic.set` only allows `i32`, `i64` and subtypes of `anyref`"
            );
        }
        Ok(())
    }
    fn visit_global_atomic_rmw_add(
        &mut self,
        _ordering: crate::Ordering,
        global_index: u32,
    ) -> Self::Output {
        let ty = self.check_atomic_global_rmw_ty(global_index)?;
        self.check_unary_op(ty)
    }
    fn visit_global_atomic_rmw_sub(
        &mut self,
        _ordering: crate::Ordering,
        global_index: u32,
    ) -> Self::Output {
        let ty = self.check_atomic_global_rmw_ty(global_index)?;
        self.check_unary_op(ty)
    }
    fn visit_global_atomic_rmw_and(
        &mut self,
        _ordering: crate::Ordering,
        global_index: u32,
    ) -> Self::Output {
        let ty = self.check_atomic_global_rmw_ty(global_index)?;
        self.check_unary_op(ty)
    }
    fn visit_global_atomic_rmw_or(
        &mut self,
        _ordering: crate::Ordering,
        global_index: u32,
    ) -> Self::Output {
        let ty = self.check_atomic_global_rmw_ty(global_index)?;
        self.check_unary_op(ty)
    }
    fn visit_global_atomic_rmw_xor(
        &mut self,
        _ordering: crate::Ordering,
        global_index: u32,
    ) -> Self::Output {
        let ty = self.check_atomic_global_rmw_ty(global_index)?;
        self.check_unary_op(ty)
    }
    fn visit_global_atomic_rmw_xchg(
        &mut self,
        _ordering: crate::Ordering,
        global_index: u32,
    ) -> Self::Output {
        let ty = self.global_type_at(global_index)?.content_type;
        if !(ty == ValType::I32
            || ty == ValType::I64
            || self.resources.is_subtype(ty, RefType::ANYREF.into()))
        {
            bail!(
                self.offset,
                "invalid type: `global.atomic.rmw.xchg` only allows `i32`, `i64` and subtypes of `anyref`"
            );
        }
        self.check_unary_op(ty)
    }
    fn visit_global_atomic_rmw_cmpxchg(
        &mut self,
        _ordering: crate::Ordering,
        global_index: u32,
    ) -> Self::Output {
        let ty = self.global_type_at(global_index)?.content_type;
        if !(ty == ValType::I32
            || ty == ValType::I64
            || self.resources.is_subtype(ty, RefType::EQREF.into()))
        {
            bail!(
                self.offset,
                "invalid type: `global.atomic.rmw.cmpxchg` only allows `i32`, `i64` and subtypes of `eqref`"
            );
        }
        self.check_binary_op(ty)
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
    fn visit_memory_size(&mut self, mem: u32) -> Self::Output {
        let index_ty = self.check_memory_index(mem)?;
        self.push_operand(index_ty)?;
        Ok(())
    }
    fn visit_memory_grow(&mut self, mem: u32) -> Self::Output {
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
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_cmpxchg(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_cmpxchg(memarg, ValType::I32)
    }
    fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_cmpxchg(memarg, ValType::I32)
    }
    fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_cmpxchg(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_cmpxchg(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_cmpxchg(memarg, ValType::I64)
    }
    fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_cmpxchg(memarg, ValType::I64)
    }
    fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> Self::Output {
        self.check_atomic_binary_memory_op(memarg, ValType::I32)
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
        let ty = self.pop_ref(None)?.as_non_null();
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_br_on_null(&mut self, relative_depth: u32) -> Self::Output {
        let ref_ty = self.pop_ref(None)?.as_non_null();
        let (ft, kind) = self.jump(relative_depth)?;
        let label_types = self.label_types(ft, kind)?;
        self.pop_push_label_types(label_types)?;
        self.push_operand(ref_ty)?;
        Ok(())
    }
    fn visit_br_on_non_null(&mut self, relative_depth: u32) -> Self::Output {
        let (ft, kind) = self.jump(relative_depth)?;

        let mut label_types = self.label_types(ft, kind)?;
        let expected = match label_types.next_back() {
            None => bail!(
                self.offset,
                "type mismatch: br_on_non_null target has no label types",
            ),
            Some(ValType::Ref(ty)) => ty,
            Some(_) => bail!(
                self.offset,
                "type mismatch: br_on_non_null target does not end with heap type",
            ),
        };
        self.pop_ref(Some(expected.nullable()))?;

        self.pop_push_label_types(label_types)?;
        Ok(())
    }
    fn visit_ref_is_null(&mut self) -> Self::Output {
        self.pop_ref(None)?;
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
        let hty = if self.features.custom_descriptors()
            && self.resources.has_function_exact_type(function_index)
        {
            HeapType::Exact(index)
        } else {
            HeapType::Concrete(index)
        };
        let ty = ValType::Ref(RefType::new(false, hty).ok_or_else(|| {
            BinaryReaderError::new("implementation limit: type index too large", self.offset)
        })?);
        self.push_operand(ty)?;
        Ok(())
    }
    fn visit_ref_eq(&mut self) -> Self::Output {
        let a = self.pop_maybe_shared_ref(AbstractHeapType::Eq)?;
        let b = self.pop_maybe_shared_ref(AbstractHeapType::Eq)?;
        let a_is_shared = a.is_maybe_shared(&self.resources);
        let b_is_shared = b.is_maybe_shared(&self.resources);
        match (a_is_shared, b_is_shared) {
            // One or both of the types are from unreachable code; assume
            // the shared-ness matches.
            (None, Some(_)) | (Some(_), None) | (None, None) => {}

            (Some(is_a_shared), Some(is_b_shared)) => {
                if is_a_shared != is_b_shared {
                    bail!(
                        self.offset,
                        "type mismatch: expected `ref.eq` types to match `shared`-ness"
                    );
                }
            }
        }
        self.push_operand(ValType::I32)
    }
    fn visit_memory_init(&mut self, segment: u32, mem: u32) -> Self::Output {
        let ty = self.check_memory_index(mem)?;
        self.check_data_segment(segment)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ty))?;
        Ok(())
    }
    fn visit_data_drop(&mut self, segment: u32) -> Self::Output {
        self.check_data_segment(segment)?;
        Ok(())
    }
    fn visit_memory_copy(&mut self, dst: u32, src: u32) -> Self::Output {
        self.check_enabled(self.features.bulk_memory_opt(), "bulk memory")?;
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
        self.check_enabled(self.features.bulk_memory_opt(), "bulk memory")?;
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
        let table = self.table_type_at(table)?;
        let segment_ty = self.element_type_at(segment)?;
        if !self
            .resources
            .is_subtype(ValType::Ref(segment_ty), ValType::Ref(table.element_type))
        {
            bail!(self.offset, "type mismatch");
        }
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(table.index_type()))?;
        Ok(())
    }
    fn visit_elem_drop(&mut self, segment: u32) -> Self::Output {
        self.element_type_at(segment)?;
        Ok(())
    }
    fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> Self::Output {
        let src = self.table_type_at(src_table)?;
        let dst = self.table_type_at(dst_table)?;
        if !self.resources.is_subtype(
            ValType::Ref(src.element_type),
            ValType::Ref(dst.element_type),
        ) {
            bail!(self.offset, "type mismatch");
        }

        // The length operand here is the smaller of src/dst, which is
        // i32 if one is i32
        self.pop_operand(Some(match src.index_type() {
            ValType::I32 => ValType::I32,
            _ => dst.index_type(),
        }))?;

        // ... and the offset into each table is required to be
        // whatever the indexing type is for that table
        self.pop_operand(Some(src.index_type()))?;
        self.pop_operand(Some(dst.index_type()))?;
        Ok(())
    }
    fn visit_table_get(&mut self, table: u32) -> Self::Output {
        let table = self.table_type_at(table)?;
        debug_assert_type_indices_are_ids(table.element_type.into());
        self.pop_operand(Some(table.index_type()))?;
        self.push_operand(table.element_type)?;
        Ok(())
    }
    fn visit_table_atomic_get(&mut self, _ordering: Ordering, table: u32) -> Self::Output {
        self.visit_table_get(table)?;
        // No validation of `ordering` is needed because `table.atomic.get` can
        // be used on both shared and unshared tables. But we do need to limit
        // which types can be used with this instruction.
        let ty = self.table_type_at(table)?.element_type;
        let supertype = RefType::ANYREF.shared().unwrap();
        if !self.resources.is_subtype(ty.into(), supertype.into()) {
            bail!(
                self.offset,
                "invalid type: `table.atomic.get` only allows subtypes of `anyref`"
            );
        }
        Ok(())
    }
    fn visit_table_set(&mut self, table: u32) -> Self::Output {
        let table = self.table_type_at(table)?;
        debug_assert_type_indices_are_ids(table.element_type.into());
        self.pop_operand(Some(table.element_type.into()))?;
        self.pop_operand(Some(table.index_type()))?;
        Ok(())
    }
    fn visit_table_atomic_set(&mut self, _ordering: Ordering, table: u32) -> Self::Output {
        self.visit_table_set(table)?;
        // No validation of `ordering` is needed because `table.atomic.set` can
        // be used on both shared and unshared tables. But we do need to limit
        // which types can be used with this instruction.
        let ty = self.table_type_at(table)?.element_type;
        let supertype = RefType::ANYREF.shared().unwrap();
        if !self.resources.is_subtype(ty.into(), supertype.into()) {
            bail!(
                self.offset,
                "invalid type: `table.atomic.set` only allows subtypes of `anyref`"
            );
        }
        Ok(())
    }
    fn visit_table_grow(&mut self, table: u32) -> Self::Output {
        let table = self.table_type_at(table)?;
        debug_assert_type_indices_are_ids(table.element_type.into());
        self.pop_operand(Some(table.index_type()))?;
        self.pop_operand(Some(table.element_type.into()))?;
        self.push_operand(table.index_type())?;
        Ok(())
    }
    fn visit_table_size(&mut self, table: u32) -> Self::Output {
        let table = self.table_type_at(table)?;
        self.push_operand(table.index_type())?;
        Ok(())
    }
    fn visit_table_fill(&mut self, table: u32) -> Self::Output {
        let table = self.table_type_at(table)?;
        debug_assert_type_indices_are_ids(table.element_type.into());
        self.pop_operand(Some(table.index_type()))?;
        self.pop_operand(Some(table.element_type.into()))?;
        self.pop_operand(Some(table.index_type()))?;
        Ok(())
    }
    fn visit_table_atomic_rmw_xchg(&mut self, _ordering: Ordering, table: u32) -> Self::Output {
        let table = self.table_type_at(table)?;
        let elem_ty = table.element_type.into();
        debug_assert_type_indices_are_ids(elem_ty);
        let supertype = RefType::ANYREF.shared().unwrap();
        if !self.resources.is_subtype(elem_ty, supertype.into()) {
            bail!(
                self.offset,
                "invalid type: `table.atomic.rmw.xchg` only allows subtypes of `anyref`"
            );
        }
        self.pop_operand(Some(elem_ty))?;
        self.pop_operand(Some(table.index_type()))?;
        self.push_operand(elem_ty)?;
        Ok(())
    }
    fn visit_table_atomic_rmw_cmpxchg(&mut self, _ordering: Ordering, table: u32) -> Self::Output {
        let table = self.table_type_at(table)?;
        let elem_ty = table.element_type.into();
        debug_assert_type_indices_are_ids(elem_ty);
        let supertype = RefType::EQREF.shared().unwrap();
        if !self.resources.is_subtype(elem_ty, supertype.into()) {
            bail!(
                self.offset,
                "invalid type: `table.atomic.rmw.cmpxchg` only allows subtypes of `eqref`"
            );
        }
        self.pop_operand(Some(elem_ty))?;
        self.pop_operand(Some(elem_ty))?;
        self.pop_operand(Some(table.index_type()))?;
        self.push_operand(elem_ty)?;
        Ok(())
    }
    fn visit_struct_new(&mut self, struct_type_index: u32) -> Self::Output {
        if let Some(_) = self
            .sub_type_at(struct_type_index)?
            .composite_type
            .descriptor_idx
        {
            bail!(
                self.offset,
                "type with descriptor requires descriptor allocation: `struct.new` with type {struct_type_index}"
            );
        }

        let struct_ty = self.struct_type_at(struct_type_index)?;
        for ty in struct_ty.fields.iter().rev() {
            self.pop_operand(Some(ty.element_type.unpack()))?;
        }
        self.push_exact_ref_if_available(false, struct_type_index)?;
        Ok(())
    }
    fn visit_struct_new_default(&mut self, type_index: u32) -> Self::Output {
        if let Some(_) = self.sub_type_at(type_index)?.composite_type.descriptor_idx {
            bail!(
                self.offset,
                "type with descriptor requires descriptor allocation: `struct.new_default` with type {type_index}"
            );
        }

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
        self.push_exact_ref_if_available(false, type_index)?;
        Ok(())
    }
    fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> Self::Output {
        if let Some(descriptor_idx) = self
            .sub_type_at(struct_type_index)?
            .composite_type
            .descriptor_idx
        {
            let ty = ValType::Ref(RefType::exact(true, descriptor_idx));
            self.pop_operand(Some(ty))?;
        } else {
            bail!(
                self.offset,
                "invalid `struct.new_desc`: type {struct_type_index} is not described"
            );
        }
        let struct_ty = self.struct_type_at(struct_type_index)?;
        for ty in struct_ty.fields.iter().rev() {
            self.pop_operand(Some(ty.element_type.unpack()))?;
        }
        self.push_exact_ref_if_available(false, struct_type_index)?;
        Ok(())
    }
    fn visit_struct_new_default_desc(&mut self, type_index: u32) -> Self::Output {
        if let Some(descriptor_idx) = self.sub_type_at(type_index)?.composite_type.descriptor_idx {
            let ty = ValType::Ref(RefType::exact(true, descriptor_idx));
            self.pop_operand(Some(ty))?;
        } else {
            bail!(
                self.offset,
                "invalid `struct.new_default_desc`: type {type_index} is not described"
            );
        }
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
        self.push_exact_ref_if_available(false, type_index)?;
        Ok(())
    }
    fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> Self::Output {
        let field_ty = self.struct_field_at(struct_type_index, field_index)?;
        if field_ty.element_type.is_packed() {
            bail!(
                self.offset,
                "can only use struct `get` with non-packed storage types"
            )
        }
        self.pop_concrete_ref(true, struct_type_index)?;
        self.push_operand(field_ty.element_type.unpack())
    }
    fn visit_struct_atomic_get(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.visit_struct_get(struct_type_index, field_index)?;
        // The `atomic` version has some additional type restrictions.
        let ty = self
            .struct_field_at(struct_type_index, field_index)?
            .element_type;
        let is_valid_type = match ty {
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::ANYREF.shared().unwrap().into()),
            _ => false,
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `struct.atomic.get` only allows `i32`, `i64` and subtypes of `anyref`"
            );
        }
        Ok(())
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
    fn visit_struct_atomic_get_s(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.visit_struct_get_s(struct_type_index, field_index)?;
        // This instruction has the same type restrictions as the non-`atomic` version.
        debug_assert!(matches!(
            self.struct_field_at(struct_type_index, field_index)?
                .element_type,
            StorageType::I8 | StorageType::I16
        ));
        Ok(())
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
    fn visit_struct_atomic_get_u(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.visit_struct_get_s(struct_type_index, field_index)?;
        // This instruction has the same type restrictions as the non-`atomic` version.
        debug_assert!(matches!(
            self.struct_field_at(struct_type_index, field_index)?
                .element_type,
            StorageType::I8 | StorageType::I16
        ));
        Ok(())
    }
    fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> Self::Output {
        let field_ty = self.mutable_struct_field_at(struct_type_index, field_index)?;
        self.pop_operand(Some(field_ty.element_type.unpack()))?;
        self.pop_concrete_ref(true, struct_type_index)?;
        Ok(())
    }
    fn visit_struct_atomic_set(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.visit_struct_set(struct_type_index, field_index)?;
        // The `atomic` version has some additional type restrictions.
        let ty = self
            .struct_field_at(struct_type_index, field_index)?
            .element_type;
        let is_valid_type = match ty {
            StorageType::I8 | StorageType::I16 => true,
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::ANYREF.shared().unwrap().into()),
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `struct.atomic.set` only allows `i8`, `i16`, `i32`, `i64` and subtypes of `anyref`"
            );
        }
        Ok(())
    }
    fn visit_struct_atomic_rmw_add(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.check_struct_atomic_rmw("add", struct_type_index, field_index)
    }
    fn visit_struct_atomic_rmw_sub(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.check_struct_atomic_rmw("sub", struct_type_index, field_index)
    }
    fn visit_struct_atomic_rmw_and(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.check_struct_atomic_rmw("and", struct_type_index, field_index)
    }
    fn visit_struct_atomic_rmw_or(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.check_struct_atomic_rmw("or", struct_type_index, field_index)
    }
    fn visit_struct_atomic_rmw_xor(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        self.check_struct_atomic_rmw("xor", struct_type_index, field_index)
    }
    fn visit_struct_atomic_rmw_xchg(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        let field = self.mutable_struct_field_at(struct_type_index, field_index)?;
        let is_valid_type = match field.element_type {
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::ANYREF.shared().unwrap().into()),
            _ => false,
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `struct.atomic.rmw.xchg` only allows `i32`, `i64` and subtypes of `anyref`"
            );
        }
        let field_ty = field.element_type.unpack();
        self.pop_operand(Some(field_ty))?;
        self.pop_concrete_ref(true, struct_type_index)?;
        self.push_operand(field_ty)?;
        Ok(())
    }
    fn visit_struct_atomic_rmw_cmpxchg(
        &mut self,
        _ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    ) -> Self::Output {
        let field = self.mutable_struct_field_at(struct_type_index, field_index)?;
        let is_valid_type = match field.element_type {
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::EQREF.shared().unwrap().into()),
            _ => false,
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `struct.atomic.rmw.cmpxchg` only allows `i32`, `i64` and subtypes of `eqref`"
            );
        }
        let field_ty = field.element_type.unpack();
        self.pop_operand(Some(field_ty))?;
        self.pop_operand(Some(field_ty))?;
        self.pop_concrete_ref(true, struct_type_index)?;
        self.push_operand(field_ty)?;
        Ok(())
    }
    fn visit_array_new(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(array_ty.element_type.unpack()))?;
        self.push_exact_ref_if_available(false, type_index)
    }
    fn visit_array_new_default(&mut self, type_index: u32) -> Self::Output {
        let ty = self.array_type_at(type_index)?;
        let val_ty = ty.element_type.unpack();
        if !val_ty.is_defaultable() {
            bail!(
                self.offset,
                "invalid `array.new_default`: {val_ty} field is not defaultable"
            );
        }
        self.pop_operand(Some(ValType::I32))?;
        self.push_exact_ref_if_available(false, type_index)
    }
    fn visit_array_new_fixed(&mut self, type_index: u32, n: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.element_type.unpack();
        for _ in 0..n {
            self.pop_operand(Some(elem_ty))?;
        }
        self.push_exact_ref_if_available(false, type_index)
    }
    fn visit_array_new_data(&mut self, type_index: u32, data_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.element_type.unpack();
        match elem_ty {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => {}
            ValType::Ref(_) => bail!(
                self.offset,
                "type mismatch: array.new_data can only create arrays with numeric and vector elements"
            ),
        }
        self.check_data_segment(data_index)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.push_exact_ref_if_available(false, type_index)
    }
    fn visit_array_new_elem(&mut self, type_index: u32, elem_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let array_ref_ty = match array_ty.element_type.unpack() {
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
        self.push_exact_ref_if_available(false, type_index)
    }
    fn visit_array_get(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.element_type;
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
    fn visit_array_atomic_get(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.visit_array_get(type_index)?;
        // The `atomic` version has some additional type restrictions.
        let elem_ty = self.array_type_at(type_index)?.element_type;
        let is_valid_type = match elem_ty {
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::ANYREF.shared().unwrap().into()),
            _ => false,
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `array.atomic.get` only allows `i32`, `i64` and subtypes of `anyref`"
            );
        }
        Ok(())
    }
    fn visit_array_get_s(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.element_type;
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
    fn visit_array_atomic_get_s(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.visit_array_get_s(type_index)?;
        // This instruction has the same type restrictions as the non-`atomic` version.
        debug_assert!(matches!(
            self.array_type_at(type_index)?.element_type,
            StorageType::I8 | StorageType::I16
        ));
        Ok(())
    }
    fn visit_array_get_u(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.array_type_at(type_index)?;
        let elem_ty = array_ty.element_type;
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
    fn visit_array_atomic_get_u(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.visit_array_get_u(type_index)?;
        // This instruction has the same type restrictions as the non-`atomic` version.
        debug_assert!(matches!(
            self.array_type_at(type_index)?.element_type,
            StorageType::I8 | StorageType::I16
        ));
        Ok(())
    }
    fn visit_array_set(&mut self, type_index: u32) -> Self::Output {
        let array_ty = self.mutable_array_type_at(type_index)?;
        self.pop_operand(Some(array_ty.element_type.unpack()))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        Ok(())
    }
    fn visit_array_atomic_set(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.visit_array_set(type_index)?;
        // The `atomic` version has some additional type restrictions.
        let elem_ty = self.array_type_at(type_index)?.element_type;
        let is_valid_type = match elem_ty {
            StorageType::I8 | StorageType::I16 => true,
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::ANYREF.shared().unwrap().into()),
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `array.atomic.set` only allows `i8`, `i16`, `i32`, `i64` and subtypes of `anyref`"
            );
        }
        Ok(())
    }
    fn visit_array_len(&mut self) -> Self::Output {
        self.pop_maybe_shared_ref(AbstractHeapType::Array)?;
        self.push_operand(ValType::I32)
    }
    fn visit_array_fill(&mut self, array_type_index: u32) -> Self::Output {
        let array_ty = self.mutable_array_type_at(array_type_index)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(array_ty.element_type.unpack()))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, array_type_index)?;
        Ok(())
    }
    fn visit_array_copy(&mut self, type_index_dst: u32, type_index_src: u32) -> Self::Output {
        let array_ty_dst = self.mutable_array_type_at(type_index_dst)?;
        let array_ty_src = self.array_type_at(type_index_src)?;
        match (array_ty_dst.element_type, array_ty_src.element_type) {
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
        let array_ty = self.mutable_array_type_at(array_type_index)?;
        let val_ty = array_ty.element_type.unpack();
        match val_ty {
            ValType::I32 | ValType::I64 | ValType::F32 | ValType::F64 | ValType::V128 => {}
            ValType::Ref(_) => bail!(
                self.offset,
                "invalid array.init_data: array type is not numeric or vector"
            ),
        }
        self.check_data_segment(array_data_index)?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, array_type_index)?;
        Ok(())
    }
    fn visit_array_init_elem(&mut self, type_index: u32, elem_index: u32) -> Self::Output {
        let array_ty = self.mutable_array_type_at(type_index)?;
        let array_ref_ty = match array_ty.element_type.unpack() {
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
    fn visit_array_atomic_rmw_add(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.check_array_atomic_rmw("add", type_index)
    }
    fn visit_array_atomic_rmw_sub(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.check_array_atomic_rmw("sub", type_index)
    }
    fn visit_array_atomic_rmw_and(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.check_array_atomic_rmw("and", type_index)
    }
    fn visit_array_atomic_rmw_or(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.check_array_atomic_rmw("or", type_index)
    }
    fn visit_array_atomic_rmw_xor(&mut self, _ordering: Ordering, type_index: u32) -> Self::Output {
        self.check_array_atomic_rmw("xor", type_index)
    }
    fn visit_array_atomic_rmw_xchg(
        &mut self,
        _ordering: Ordering,
        type_index: u32,
    ) -> Self::Output {
        let field = self.mutable_array_type_at(type_index)?;
        let is_valid_type = match field.element_type {
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::ANYREF.shared().unwrap().into()),
            _ => false,
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `array.atomic.rmw.xchg` only allows `i32`, `i64` and subtypes of `anyref`"
            );
        }
        let elem_ty = field.element_type.unpack();
        self.pop_operand(Some(elem_ty))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        self.push_operand(elem_ty)?;
        Ok(())
    }
    fn visit_array_atomic_rmw_cmpxchg(
        &mut self,
        _ordering: Ordering,
        type_index: u32,
    ) -> Self::Output {
        let field = self.mutable_array_type_at(type_index)?;
        let is_valid_type = match field.element_type {
            StorageType::Val(ValType::I32) | StorageType::Val(ValType::I64) => true,
            StorageType::Val(v) => self
                .resources
                .is_subtype(v, RefType::EQREF.shared().unwrap().into()),
            _ => false,
        };
        if !is_valid_type {
            bail!(
                self.offset,
                "invalid type: `array.atomic.rmw.cmpxchg` only allows `i32`, `i64` and subtypes of `eqref`"
            );
        }
        let elem_ty = field.element_type.unpack();
        self.pop_operand(Some(elem_ty))?;
        self.pop_operand(Some(elem_ty))?;
        self.pop_operand(Some(ValType::I32))?;
        self.pop_concrete_ref(true, type_index)?;
        self.push_operand(elem_ty)?;
        Ok(())
    }
    fn visit_any_convert_extern(&mut self) -> Self::Output {
        let any_ref = match self.pop_maybe_shared_ref(AbstractHeapType::Extern)? {
            MaybeType::Bottom | MaybeType::UnknownRef(_) => {
                MaybeType::UnknownRef(Some(AbstractHeapType::Any))
            }
            MaybeType::Known(ty) => {
                let shared = self.resources.is_shared(ty);
                let heap_type = HeapType::Abstract {
                    shared,
                    ty: AbstractHeapType::Any,
                };
                let any_ref = RefType::new(ty.is_nullable(), heap_type).unwrap();
                MaybeType::Known(any_ref)
            }
        };
        self.push_operand(any_ref)
    }
    fn visit_extern_convert_any(&mut self) -> Self::Output {
        let extern_ref = match self.pop_maybe_shared_ref(AbstractHeapType::Any)? {
            MaybeType::Bottom | MaybeType::UnknownRef(_) => {
                MaybeType::UnknownRef(Some(AbstractHeapType::Extern))
            }
            MaybeType::Known(ty) => {
                let shared = self.resources.is_shared(ty);
                let heap_type = HeapType::Abstract {
                    shared,
                    ty: AbstractHeapType::Extern,
                };
                let extern_ref = RefType::new(ty.is_nullable(), heap_type).unwrap();
                MaybeType::Known(extern_ref)
            }
        };
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

        self.check_br_on_cast_type_hierarchy(from_ref_type, to_ref_type)?;

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
                "type mismatch: br_on_cast to label with empty types, must have a reference type"
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

        self.check_br_on_cast_type_hierarchy(from_ref_type, to_ref_type)?;

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
    fn visit_ref_i31_shared(&mut self) -> Self::Output {
        self.pop_operand(Some(ValType::I32))?;
        self.push_operand(ValType::Ref(
            RefType::I31.shared().expect("i31 is abstract"),
        ))
    }
    fn visit_i31_get_s(&mut self) -> Self::Output {
        self.pop_maybe_shared_ref(AbstractHeapType::I31)?;
        self.push_operand(ValType::I32)
    }
    fn visit_i31_get_u(&mut self) -> Self::Output {
        self.pop_maybe_shared_ref(AbstractHeapType::I31)?;
        self.push_operand(ValType::I32)
    }
    fn visit_try(&mut self, mut ty: BlockType) -> Self::Output {
        self.check_block_type(&mut ty)?;
        for ty in self.params(ty)?.rev() {
            self.pop_operand(Some(ty))?;
        }
        self.push_ctrl(FrameKind::LegacyTry, ty)?;
        Ok(())
    }
    fn visit_catch(&mut self, index: u32) -> Self::Output {
        let frame = self.pop_ctrl()?;
        if frame.kind != FrameKind::LegacyTry && frame.kind != FrameKind::LegacyCatch {
            bail!(self.offset, "catch found outside of an `try` block");
        }
        // Start a new frame and push `exnref` value.
        let height = self.operands.len();
        let init_height = self.local_inits.push_ctrl();
        self.control.push(Frame {
            kind: FrameKind::LegacyCatch,
            block_type: frame.block_type,
            height,
            unreachable: false,
            init_height,
        });
        // Push exception argument types.
        let ty = self.exception_tag_at(index)?;
        for ty in ty.params() {
            self.push_operand(*ty)?;
        }
        Ok(())
    }
    fn visit_rethrow(&mut self, relative_depth: u32) -> Self::Output {
        // This is not a jump, but we need to check that the `rethrow`
        // targets an actual `catch` to get the exception.
        let (_, kind) = self.jump(relative_depth)?;
        if kind != FrameKind::LegacyCatch && kind != FrameKind::LegacyCatchAll {
            bail!(
                self.offset,
                "invalid rethrow label: target was not a `catch` block"
            );
        }
        self.unreachable()?;
        Ok(())
    }
    fn visit_delegate(&mut self, relative_depth: u32) -> Self::Output {
        let frame = self.pop_ctrl()?;
        if frame.kind != FrameKind::LegacyTry {
            bail!(self.offset, "delegate found outside of an `try` block");
        }
        // This operation is not a jump, but we need to check the
        // depth for validity
        let _ = self.jump(relative_depth)?;
        for ty in self.results(frame.block_type)? {
            self.push_operand(ty)?;
        }
        Ok(())
    }
    fn visit_catch_all(&mut self) -> Self::Output {
        let frame = self.pop_ctrl()?;
        if frame.kind == FrameKind::LegacyCatchAll {
            bail!(self.offset, "only one catch_all allowed per `try` block");
        } else if frame.kind != FrameKind::LegacyTry && frame.kind != FrameKind::LegacyCatch {
            bail!(self.offset, "catch_all found outside of a `try` block");
        }
        let height = self.operands.len();
        let init_height = self.local_inits.push_ctrl();
        self.control.push(Frame {
            kind: FrameKind::LegacyCatchAll,
            block_type: frame.block_type,
            height,
            unreachable: false,
            init_height,
        });
        Ok(())
    }
    fn visit_cont_new(&mut self, type_index: u32) -> Self::Output {
        let cont_ty = self.cont_type_at(type_index)?;
        let rt = RefType::concrete(true, cont_ty.0);
        self.pop_ref(Some(rt))?;
        self.push_concrete_ref(false, type_index)?;
        Ok(())
    }
    fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> Self::Output {
        // [ts1 ts1'] -> [ts2]
        let arg_cont = self.cont_type_at(argument_index)?;
        let arg_func = self.func_type_of_cont_type(arg_cont);
        // [ts1''] -> [ts2']
        let res_cont = self.cont_type_at(result_index)?;
        let res_func = self.func_type_of_cont_type(res_cont);

        // Verify that the argument's domain is at least as large as the
        // result's domain.
        if arg_func.params().len() < res_func.params().len() {
            bail!(self.offset, "type mismatch in continuation arguments");
        }

        let argcnt = arg_func.params().len() - res_func.params().len();

        // Check that [ts1'] -> [ts2] <: [ts1''] -> [ts2']
        if !self.is_subtype_many(res_func.params(), &arg_func.params()[argcnt..])
            || arg_func.results().len() != res_func.results().len()
            || !self.is_subtype_many(arg_func.results(), res_func.results())
        {
            bail!(self.offset, "type mismatch in continuation types");
        }

        // Check that the continuation is available on the stack.
        self.pop_concrete_ref(true, argument_index)?;

        // Check that the argument prefix is available on the stack.
        for &ty in arg_func.params().iter().take(argcnt).rev() {
            self.pop_operand(Some(ty))?;
        }

        // Construct the result type.
        self.push_concrete_ref(false, result_index)?;

        Ok(())
    }
    fn visit_suspend(&mut self, tag_index: u32) -> Self::Output {
        let ft = &self.tag_at(tag_index)?;
        for &ty in ft.params().iter().rev() {
            self.pop_operand(Some(ty))?;
        }
        for &ty in ft.results() {
            self.push_operand(ty)?;
        }
        Ok(())
    }
    fn visit_resume(&mut self, type_index: u32, table: ResumeTable) -> Self::Output {
        // [ts1] -> [ts2]
        let ft = self.check_resume_table(table, type_index)?;
        self.pop_concrete_ref(true, type_index)?;
        // Check that ts1 are available on the stack.
        for &ty in ft.params().iter().rev() {
            self.pop_operand(Some(ty))?;
        }

        // Make ts2 available on the stack.
        for &ty in ft.results() {
            self.push_operand(ty)?;
        }
        Ok(())
    }
    fn visit_resume_throw(
        &mut self,
        type_index: u32,
        tag_index: u32,
        table: ResumeTable,
    ) -> Self::Output {
        // [ts1] -> [ts2]
        let ft = self.check_resume_table(table, type_index)?;
        // [ts1'] -> []
        let tag_ty = self.exception_tag_at(tag_index)?;
        if tag_ty.results().len() != 0 {
            bail!(self.offset, "type mismatch: non-empty tag result type")
        }
        self.pop_concrete_ref(true, type_index)?;
        // Check that ts1' are available on the stack.
        for &ty in tag_ty.params().iter().rev() {
            self.pop_operand(Some(ty))?;
        }

        // Make ts2 available on the stack.
        for &ty in ft.results() {
            self.push_operand(ty)?;
        }
        Ok(())
    }
    fn visit_switch(&mut self, type_index: u32, tag_index: u32) -> Self::Output {
        // [t1* (ref null $ct2)] -> [te1*]
        let cont_ty = self.cont_type_at(type_index)?;
        let func_ty = self.func_type_of_cont_type(cont_ty);
        // [] -> [t*]
        let tag_ty = self.tag_at(tag_index)?;
        if tag_ty.params().len() != 0 {
            bail!(self.offset, "type mismatch: non-empty tag parameter type")
        }
        // Extract the other continuation reference
        match func_ty.params().last() {
            Some(ValType::Ref(rt)) if rt.is_concrete_type_ref() => {
                let other_cont_id = rt
                    .type_index()
                    .unwrap()
                    .unpack()
                    .as_core_type_id()
                    .expect("expected canonicalized index");
                let sub_ty = self.resources.sub_type_at_id(other_cont_id);
                let other_cont_ty =
                    if let CompositeInnerType::Cont(cont) = &sub_ty.composite_type.inner {
                        cont
                    } else {
                        bail!(self.offset, "non-continuation type");
                    };
                let other_func_ty = self.func_type_of_cont_type(&other_cont_ty);
                if func_ty.results().len() != tag_ty.results().len()
                    || !self.is_subtype_many(func_ty.results(), tag_ty.results())
                    || other_func_ty.results().len() != tag_ty.results().len()
                    || !self.is_subtype_many(tag_ty.results(), other_func_ty.results())
                {
                    bail!(self.offset, "type mismatch in continuation types")
                }

                // Pop the continuation reference.
                self.pop_concrete_ref(true, type_index)?;

                // Check that the arguments t1* are available on the
                // stack.
                for &ty in func_ty.params().iter().rev().skip(1) {
                    self.pop_operand(Some(ty))?;
                }

                // Make the results t2* available on the stack.
                for &ty in other_func_ty.params() {
                    self.push_operand(ty)?;
                }
            }
            Some(ty) => bail!(
                self.offset,
                "type mismatch: expected a continuation reference, found {}",
                ty_to_str(*ty)
            ),
            None => bail!(
                self.offset,
                "type mismatch: instruction requires a continuation reference"
            ),
        }
        Ok(())
    }
    fn visit_i64_add128(&mut self) -> Result<()> {
        self.check_binop128()
    }
    fn visit_i64_sub128(&mut self) -> Result<()> {
        self.check_binop128()
    }
    fn visit_i64_mul_wide_s(&mut self) -> Result<()> {
        self.check_i64_mul_wide()
    }
    fn visit_i64_mul_wide_u(&mut self) -> Result<()> {
        self.check_i64_mul_wide()
    }

    fn visit_ref_get_desc(&mut self, type_index: u32) -> Self::Output {
        let (_, is_exact) = self.pop_concrete_or_exact_ref(true, type_index)?;
        match self.sub_type_at(type_index)?.composite_type.descriptor_idx {
            Some(descriptor_idx) => {
                let ref_ty = if is_exact {
                    RefType::exact(false, descriptor_idx)
                } else {
                    RefType::concrete(false, descriptor_idx)
                };
                self.push_operand(ref_ty)
            }
            None => bail!(self.offset, "expected type with descriptor"),
        }
    }

    fn visit_ref_cast_desc_non_null(&mut self, heap_type: HeapType) -> Self::Output {
        self.check_ref_cast_desc(false, heap_type)
    }
    fn visit_ref_cast_desc_nullable(&mut self, heap_type: HeapType) -> Self::Output {
        self.check_ref_cast_desc(true, heap_type)
    }
    fn visit_br_on_cast_desc(
        &mut self,
        relative_depth: u32,
        mut from_ref_type: RefType,
        mut to_ref_type: RefType,
    ) -> Self::Output {
        let described_ty = to_ref_type.heap_type();

        self.resources
            .check_ref_type(&mut from_ref_type, self.offset)?;
        self.resources
            .check_ref_type(&mut to_ref_type, self.offset)?;

        self.check_br_on_cast_type_hierarchy(from_ref_type, to_ref_type)?;

        self.check_maybe_exact_descriptor_ref(described_ty)?;

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
                "type mismatch: br_on_cast to label with empty types, must have a reference type"
            ),
        };

        self.pop_push_label_types(label_types)?;
        let diff_ty = RefType::difference(from_ref_type, to_ref_type);
        self.push_operand(diff_ty)?;
        Ok(())
    }
    fn visit_br_on_cast_desc_fail(
        &mut self,
        relative_depth: u32,
        mut from_ref_type: RefType,
        mut to_ref_type: RefType,
    ) -> Self::Output {
        let described_ty = to_ref_type.heap_type();

        self.resources
            .check_ref_type(&mut from_ref_type, self.offset)?;
        self.resources
            .check_ref_type(&mut to_ref_type, self.offset)?;

        self.check_br_on_cast_type_hierarchy(from_ref_type, to_ref_type)?;

        self.check_maybe_exact_descriptor_ref(described_ty)?;

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

trait PreciseIterator: ExactSizeIterator + DoubleEndedIterator + Clone + core::fmt::Debug {}
impl<T: ExactSizeIterator + DoubleEndedIterator + Clone + core::fmt::Debug> PreciseIterator for T {}

impl Locals {
    /// Defines another group of `count` local variables of type `ty`.
    ///
    /// Returns `true` if the definition was successful. Local variable
    /// definition is unsuccessful in case the amount of total variables
    /// after definition exceeds the allowed maximum number.
    fn define(&mut self, count: u32, ty: ValType) -> bool {
        if count == 0 {
            return true;
        }
        let vacant_first = MAX_LOCALS_TO_TRACK.saturating_sub(self.num_locals);
        match self.num_locals.checked_add(count) {
            Some(num_locals) if num_locals > MAX_WASM_FUNCTION_LOCALS => return false,
            None => return false,
            Some(num_locals) => self.num_locals = num_locals,
        };
        let push_to_first = cmp::min(vacant_first, count);
        self.first
            .extend(iter::repeat(ty).take(push_to_first as usize));
        let num_uncached = count - push_to_first;
        if num_uncached > 0 {
            let max_uncached_idx = self.num_locals - 1;
            self.uncached.push((max_uncached_idx, ty));
        }
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
        match self.uncached.binary_search_by_key(&idx, |(idx, _)| *idx) {
            // If this index would be inserted at the end of the list, then the
            // index is out of bounds and we return an error.
            Err(i) if i == self.uncached.len() => None,

            // If `Ok` is returned we found the index exactly, or if `Err` is
            // returned the position is the one which is the least index
            // greater that `idx`, which is still the type of `idx` according
            // to our "compressed" representation. In both cases we access the
            // list at index `i`.
            Ok(i) | Err(i) => Some(self.uncached[i].1),
        }
    }
}

impl<R> ModuleArity for WasmProposalValidator<'_, '_, R>
where
    R: WasmModuleResources,
{
    fn tag_type_arity(&self, at: u32) -> Option<(u32, u32)> {
        self.0
            .resources
            .tag_at(at)
            .map(|x| (x.params().len() as u32, x.results().len() as u32))
    }

    fn type_index_of_function(&self, function_idx: u32) -> Option<u32> {
        self.0.resources.type_index_of_function(function_idx)
    }

    fn sub_type_at(&self, type_idx: u32) -> Option<&SubType> {
        Some(self.0.sub_type_at(type_idx).ok()?)
    }

    fn func_type_of_cont_type(&self, c: &ContType) -> Option<&FuncType> {
        Some(self.0.func_type_of_cont_type(c))
    }

    fn sub_type_of_ref_type(&self, rt: &RefType) -> Option<&SubType> {
        let id = rt.type_index()?.as_core_type_id()?;
        Some(self.0.resources.sub_type_at_id(id))
    }

    fn control_stack_height(&self) -> u32 {
        self.0.control.len() as u32
    }

    fn label_block(&self, depth: u32) -> Option<(BlockType, FrameKind)> {
        self.0.jump(depth).ok()
    }
}

impl<R> FrameStack for WasmProposalValidator<'_, '_, R>
where
    R: WasmModuleResources,
{
    fn current_frame(&self) -> Option<FrameKind> {
        Some(self.0.control.last()?.kind)
    }
}
