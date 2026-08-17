//! Data flow graph tracking Instructions, Values, and blocks.

use crate::entity::{self, PrimaryMap, SecondaryMap};
use crate::ir;
use crate::ir::builder::ReplaceBuilder;
use crate::ir::dynamic_type::{DynamicTypeData, DynamicTypes};
use crate::ir::instructions::{CallInfo, InstructionData};
use crate::ir::pcc::Fact;
use crate::ir::user_stack_maps::{UserStackMapEntry, UserStackMapEntryVec};
use crate::ir::{
    Block, BlockArg, BlockCall, ConstantData, ConstantPool, DynamicType, ExceptionTables,
    ExtFuncData, FuncRef, Immediate, Inst, JumpTables, RelSourceLoc, SigRef, Signature, Type,
    Value, ValueLabelAssignments, ValueList, ValueListPool, types,
};
use crate::packed_option::ReservedValue;
use crate::write::write_operands;
use core::fmt;
use core::iter;
use core::mem;
use core::ops::{Index, IndexMut};
use core::u16;

use alloc::collections::BTreeMap;
#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Storage for instructions within the DFG.
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Insts(PrimaryMap<Inst, InstructionData>);

/// Allow immutable access to instructions via indexing.
impl Index<Inst> for Insts {
    type Output = InstructionData;

    fn index(&self, inst: Inst) -> &InstructionData {
        self.0.index(inst)
    }
}

/// Allow mutable access to instructions via indexing.
impl IndexMut<Inst> for Insts {
    fn index_mut(&mut self, inst: Inst) -> &mut InstructionData {
        self.0.index_mut(inst)
    }
}

/// Storage for basic blocks within the DFG.
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct Blocks(PrimaryMap<Block, BlockData>);

impl Blocks {
    /// Create a new basic block.
    pub fn add(&mut self) -> Block {
        self.0.push(BlockData::new())
    }

    /// Get the total number of basic blocks created in this function, whether they are
    /// currently inserted in the layout or not.
    ///
    /// This is intended for use with `SecondaryMap::with_capacity`.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Reserves capacity for at least `additional` more elements to be
    /// inserted.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Returns `true` if the given block reference is valid.
    pub fn is_valid(&self, block: Block) -> bool {
        self.0.is_valid(block)
    }

    /// Iterate over all blocks, regardless whether a block is actually inserted
    /// in the layout or not.
    ///
    /// Iterates in creation order, not layout order.
    pub fn iter(&self) -> impl Iterator<Item = Block> {
        self.0.keys()
    }
}

impl Index<Block> for Blocks {
    type Output = BlockData;

    fn index(&self, block: Block) -> &BlockData {
        &self.0[block]
    }
}

impl IndexMut<Block> for Blocks {
    fn index_mut(&mut self, block: Block) -> &mut BlockData {
        &mut self.0[block]
    }
}

/// A data flow graph defines all instructions and basic blocks in a function as well as
/// the data flow dependencies between them. The DFG also tracks values which can be either
/// instruction results or block parameters.
///
/// The layout of blocks in the function and of instructions in each block is recorded by the
/// `Layout` data structure which forms the other half of the function representation.
///
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct DataFlowGraph {
    /// Data about all of the instructions in the function, including opcodes and operands.
    /// The instructions in this map are not in program order. That is tracked by `Layout`, along
    /// with the block containing each instruction.
    pub insts: Insts,

    /// List of result values for each instruction.
    ///
    /// This map gets resized automatically by `make_inst()` so it is always in sync with the
    /// primary `insts` map.
    results: SecondaryMap<Inst, ValueList>,

    /// User-defined stack maps.
    user_stack_maps: alloc::collections::BTreeMap<Inst, UserStackMapEntryVec>,

    /// basic blocks in the function and their parameters.
    ///
    /// This map is not in program order. That is handled by `Layout`, and so is the sequence of
    /// instructions contained in each block.
    pub blocks: Blocks,

    /// Dynamic types created.
    pub dynamic_types: DynamicTypes,

    /// Memory pool of value lists.
    ///
    /// The `ValueList` references into this pool appear in many places:
    ///
    /// - Instructions in `insts` that don't have room for their entire argument list inline.
    /// - Instruction result values in `results`.
    /// - block parameters in `blocks`.
    pub value_lists: ValueListPool,

    /// Primary value table with entries for all values.
    values: PrimaryMap<Value, ValueDataPacked>,

    /// Facts: proof-carrying-code assertions about values.
    pub facts: SecondaryMap<Value, Option<Fact>>,

    /// Function signature table. These signatures are referenced by indirect call instructions as
    /// well as the external function references.
    pub signatures: PrimaryMap<SigRef, Signature>,

    /// External function references. These are functions that can be called directly.
    pub ext_funcs: PrimaryMap<FuncRef, ExtFuncData>,

    /// Saves Value labels.
    pub values_labels: Option<BTreeMap<Value, ValueLabelAssignments>>,

    /// Constants used within the function.
    pub constants: ConstantPool,

    /// Stores large immediates that otherwise will not fit on InstructionData.
    pub immediates: PrimaryMap<Immediate, ConstantData>,

    /// Jump tables used in this function.
    pub jump_tables: JumpTables,

    /// Exception tables used in this function.
    pub exception_tables: ExceptionTables,
}

impl DataFlowGraph {
    /// Create a new empty `DataFlowGraph`.
    pub fn new() -> Self {
        Self {
            insts: Insts(PrimaryMap::new()),
            results: SecondaryMap::new(),
            user_stack_maps: alloc::collections::BTreeMap::new(),
            blocks: Blocks(PrimaryMap::new()),
            dynamic_types: DynamicTypes::new(),
            value_lists: ValueListPool::new(),
            values: PrimaryMap::new(),
            facts: SecondaryMap::new(),
            signatures: PrimaryMap::new(),
            ext_funcs: PrimaryMap::new(),
            values_labels: None,
            constants: ConstantPool::new(),
            immediates: PrimaryMap::new(),
            jump_tables: JumpTables::new(),
            exception_tables: ExceptionTables::new(),
        }
    }

    /// Clear everything.
    pub fn clear(&mut self) {
        self.insts.0.clear();
        self.results.clear();
        self.user_stack_maps.clear();
        self.blocks.0.clear();
        self.dynamic_types.clear();
        self.value_lists.clear();
        self.values.clear();
        self.signatures.clear();
        self.ext_funcs.clear();
        self.values_labels = None;
        self.constants.clear();
        self.immediates.clear();
        self.jump_tables.clear();
        self.facts.clear();
    }

    /// Get the total number of instructions created in this function, whether they are currently
    /// inserted in the layout or not.
    ///
    /// This is intended for use with `SecondaryMap::with_capacity`.
    pub fn num_insts(&self) -> usize {
        self.insts.0.len()
    }

    /// Returns `true` if the given instruction reference is valid.
    pub fn inst_is_valid(&self, inst: Inst) -> bool {
        self.insts.0.is_valid(inst)
    }

    /// Get the total number of basic blocks created in this function, whether they are
    /// currently inserted in the layout or not.
    ///
    /// This is intended for use with `SecondaryMap::with_capacity`.
    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    /// Returns `true` if the given block reference is valid.
    pub fn block_is_valid(&self, block: Block) -> bool {
        self.blocks.is_valid(block)
    }

    /// Make a BlockCall, bundling together the block and its arguments.
    pub fn block_call<'a>(
        &mut self,
        block: Block,
        args: impl IntoIterator<Item = &'a BlockArg>,
    ) -> BlockCall {
        BlockCall::new(block, args.into_iter().copied(), &mut self.value_lists)
    }

    /// Get the total number of values.
    pub fn num_values(&self) -> usize {
        self.values.len()
    }

    /// Get an iterator over all values and their definitions.
    pub fn values_and_defs(&self) -> impl Iterator<Item = (Value, ValueDef)> + '_ {
        self.values().map(|value| (value, self.value_def(value)))
    }

    /// Starts collection of debug information.
    pub fn collect_debug_info(&mut self) {
        if self.values_labels.is_none() {
            self.values_labels = Some(Default::default());
        }
    }

    /// Inserts a `ValueLabelAssignments::Alias` for `to_alias` if debug info
    /// collection is enabled.
    pub fn add_value_label_alias(&mut self, to_alias: Value, from: RelSourceLoc, value: Value) {
        if let Some(values_labels) = self.values_labels.as_mut() {
            values_labels.insert(to_alias, ir::ValueLabelAssignments::Alias { from, value });
        }
    }
}

/// Resolve value aliases.
///
/// Find the original SSA value that `value` aliases, or None if an
/// alias cycle is detected.
fn maybe_resolve_aliases(
    values: &PrimaryMap<Value, ValueDataPacked>,
    value: Value,
) -> Option<Value> {
    let mut v = value;

    // Note that values may be empty here.
    for _ in 0..=values.len() {
        if let ValueData::Alias { original, .. } = ValueData::from(values[v]) {
            v = original;
        } else {
            return Some(v);
        }
    }

    None
}

/// Resolve value aliases.
///
/// Find the original SSA value that `value` aliases.
fn resolve_aliases(values: &PrimaryMap<Value, ValueDataPacked>, value: Value) -> Value {
    if let Some(v) = maybe_resolve_aliases(values, value) {
        v
    } else {
        panic!("Value alias loop detected for {value}");
    }
}

/// Iterator over all Values in a DFG.
pub struct Values<'a> {
    inner: entity::Iter<'a, Value, ValueDataPacked>,
}

/// Check for non-values.
fn valid_valuedata(data: ValueDataPacked) -> bool {
    let data = ValueData::from(data);
    if let ValueData::Alias {
        ty: types::INVALID,
        original,
    } = data
    {
        if original == Value::reserved_value() {
            return false;
        }
    }
    true
}

impl<'a> Iterator for Values<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .by_ref()
            .find(|kv| valid_valuedata(*kv.1))
            .map(|kv| kv.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ExactSizeIterator for Values<'_> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Handling values.
///
/// Values are either block parameters or instruction results.
impl DataFlowGraph {
    /// Allocate an extended value entry.
    fn make_value(&mut self, data: ValueData) -> Value {
        self.values.push(data.into())
    }

    /// The number of values defined in this DFG.
    pub fn len_values(&self) -> usize {
        self.values.len()
    }

    /// Get an iterator over all values.
    pub fn values<'a>(&'a self) -> Values<'a> {
        Values {
            inner: self.values.iter(),
        }
    }

    /// Check if a value reference is valid.
    pub fn value_is_valid(&self, v: Value) -> bool {
        self.values.is_valid(v)
    }

    /// Check whether a value is valid and not an alias.
    pub fn value_is_real(&self, value: Value) -> bool {
        // Deleted or unused values are also stored as aliases so this excludes
        // those as well.
        self.value_is_valid(value) && !matches!(self.values[value].into(), ValueData::Alias { .. })
    }

    /// Get the type of a value.
    pub fn value_type(&self, v: Value) -> Type {
        self.values[v].ty()
    }

    /// Get the definition of a value.
    ///
    /// This is either the instruction that defined it or the Block that has the value as an
    /// parameter.
    pub fn value_def(&self, v: Value) -> ValueDef {
        match ValueData::from(self.values[v]) {
            ValueData::Inst { inst, num, .. } => ValueDef::Result(inst, num as usize),
            ValueData::Param { block, num, .. } => ValueDef::Param(block, num as usize),
            ValueData::Alias { original, .. } => {
                // Make sure we only recurse one level. `resolve_aliases` has safeguards to
                // detect alias loops without overrunning the stack.
                self.value_def(self.resolve_aliases(original))
            }
            ValueData::Union { x, y, .. } => ValueDef::Union(x, y),
        }
    }

    /// Determine if `v` is an attached instruction result / block parameter.
    ///
    /// An attached value can't be attached to something else without first being detached.
    ///
    /// Value aliases are not considered to be attached to anything. Use `resolve_aliases()` to
    /// determine if the original aliased value is attached.
    pub fn value_is_attached(&self, v: Value) -> bool {
        use self::ValueData::*;
        match ValueData::from(self.values[v]) {
            Inst { inst, num, .. } => Some(&v) == self.inst_results(inst).get(num as usize),
            Param { block, num, .. } => Some(&v) == self.block_params(block).get(num as usize),
            Alias { .. } => false,
            Union { .. } => false,
        }
    }

    /// Resolve value aliases.
    ///
    /// Find the original SSA value that `value` aliases.
    pub fn resolve_aliases(&self, value: Value) -> Value {
        resolve_aliases(&self.values, value)
    }

    /// Replace all uses of value aliases with their resolved values, and delete
    /// the aliases.
    pub fn resolve_all_aliases(&mut self) {
        let invalid_value = ValueDataPacked::from(ValueData::Alias {
            ty: types::INVALID,
            original: Value::reserved_value(),
        });

        // Rewrite each chain of aliases. Update every alias along the chain
        // into an alias directly to the final value. Due to updating every
        // alias that it looks at, this loop runs in time linear in the number
        // of values.
        for mut src in self.values.keys() {
            let value_data = self.values[src];
            if value_data == invalid_value {
                continue;
            }
            if let ValueData::Alias { mut original, .. } = value_data.into() {
                // We don't use the type after this, we just need some place to
                // store the resolved aliases temporarily.
                let resolved = ValueDataPacked::from(ValueData::Alias {
                    ty: types::INVALID,
                    original: resolve_aliases(&self.values, original),
                });
                // Walk the chain again, splatting the new alias everywhere.
                // resolve_aliases panics if there's an alias cycle, so we don't
                // need to guard against cycles here.
                loop {
                    self.values[src] = resolved;
                    src = original;
                    if let ValueData::Alias { original: next, .. } = self.values[src].into() {
                        original = next;
                    } else {
                        break;
                    }
                }
            }
        }

        // Now aliases don't point to other aliases, so we can replace any use
        // of an alias with the final value in constant time.

        // Rewrite InstructionData in `self.insts`.
        for inst in self.insts.0.values_mut() {
            inst.map_values(
                &mut self.value_lists,
                &mut self.jump_tables,
                &mut self.exception_tables,
                |arg| {
                    if let ValueData::Alias { original, .. } = self.values[arg].into() {
                        original
                    } else {
                        arg
                    }
                },
            );
        }

        // - `results` and block-params in `blocks` are not aliases, by
        //   definition.
        // - `dynamic_types` has no values.
        // - `value_lists` can only be accessed via references from elsewhere.
        // - `values` only has value references in aliases (which we've
        //   removed), and unions (but the egraph pass ensures there are no
        //   aliases before creating unions).

        // Merge `facts` from any alias onto the aliased value. Note that if
        // there was a chain of aliases, at this point every alias that was in
        // the chain points to the same final value, so their facts will all be
        // merged together.
        for value in self.facts.keys() {
            if let ValueData::Alias { original, .. } = self.values[value].into() {
                if let Some(new_fact) = self.facts[value].take() {
                    match &mut self.facts[original] {
                        Some(old_fact) => *old_fact = Fact::intersect(old_fact, &new_fact),
                        old_fact => *old_fact = Some(new_fact),
                    }
                }
            }
        }

        // - `signatures` and `ext_funcs` have no values.

        if let Some(values_labels) = &mut self.values_labels {
            // Debug info is best-effort. If any is attached to value aliases,
            // just discard it.
            values_labels.retain(|&k, _| !matches!(self.values[k].into(), ValueData::Alias { .. }));

            // If debug-info says a value should have the same labels as another
            // value, then make sure that target is not a value alias.
            for value_label in values_labels.values_mut() {
                if let ValueLabelAssignments::Alias { value, .. } = value_label {
                    if let ValueData::Alias { original, .. } = self.values[*value].into() {
                        *value = original;
                    }
                }
            }
        }

        // - `constants` and `immediates` have no values.
        // - `jump_tables` is updated together with instruction-data above.

        // Delete all aliases now that there are no uses left.
        for value in self.values.values_mut() {
            if let ValueData::Alias { .. } = ValueData::from(*value) {
                *value = invalid_value;
            }
        }
    }

    /// Turn a value into an alias of another.
    ///
    /// Change the `dest` value to behave as an alias of `src`. This means that all uses of `dest`
    /// will behave as if they used that value `src`.
    ///
    /// The `dest` value can't be attached to an instruction or block.
    pub fn change_to_alias(&mut self, dest: Value, src: Value) {
        debug_assert!(!self.value_is_attached(dest));
        // Try to create short alias chains by finding the original source value.
        // This also avoids the creation of loops.
        let original = self.resolve_aliases(src);
        debug_assert_ne!(
            dest, original,
            "Aliasing {dest} to {src} would create a loop"
        );
        let ty = self.value_type(original);
        debug_assert_eq!(
            self.value_type(dest),
            ty,
            "Aliasing {} to {} would change its type {} to {}",
            dest,
            src,
            self.value_type(dest),
            ty
        );
        debug_assert_ne!(ty, types::INVALID);

        self.values[dest] = ValueData::Alias { ty, original }.into();
    }

    /// Replace the results of one instruction with aliases to the results of another.
    ///
    /// Change all the results of `dest_inst` to behave as aliases of
    /// corresponding results of `src_inst`, as if calling change_to_alias for
    /// each.
    ///
    /// After calling this instruction, `dest_inst` will have had its results
    /// cleared, so it likely needs to be removed from the graph.
    ///
    pub fn replace_with_aliases(&mut self, dest_inst: Inst, original_inst: Inst) {
        debug_assert_ne!(
            dest_inst, original_inst,
            "Replacing {dest_inst} with itself would create a loop"
        );

        let dest_results = self.results[dest_inst].as_slice(&self.value_lists);
        let original_results = self.results[original_inst].as_slice(&self.value_lists);

        debug_assert_eq!(
            dest_results.len(),
            original_results.len(),
            "Replacing {dest_inst} with {original_inst} would produce a different number of results."
        );

        for (&dest, &original) in dest_results.iter().zip(original_results) {
            let ty = self.value_type(original);
            debug_assert_eq!(
                self.value_type(dest),
                ty,
                "Aliasing {} to {} would change its type {} to {}",
                dest,
                original,
                self.value_type(dest),
                ty
            );
            debug_assert_ne!(ty, types::INVALID);

            self.values[dest] = ValueData::Alias { ty, original }.into();
        }

        self.clear_results(dest_inst);
    }

    /// Get the stack map entries associated with the given instruction.
    pub fn user_stack_map_entries(&self, inst: Inst) -> Option<&[UserStackMapEntry]> {
        self.user_stack_maps.get(&inst).map(|es| &**es)
    }

    /// Append a new stack map entry for the given call instruction.
    ///
    /// # Panics
    ///
    /// Panics if the given instruction is not a (non-tail) call instruction.
    pub fn append_user_stack_map_entry(&mut self, inst: Inst, entry: UserStackMapEntry) {
        let opcode = self.insts[inst].opcode();
        assert!(opcode.is_safepoint());
        self.user_stack_maps.entry(inst).or_default().push(entry);
    }

    /// Append multiple stack map entries for the given call instruction.
    ///
    /// # Panics
    ///
    /// Panics if the given instruction is not a (non-tail) call instruction.
    pub fn append_user_stack_map_entries(
        &mut self,
        inst: Inst,
        entries: impl IntoIterator<Item = UserStackMapEntry>,
    ) {
        for entry in entries {
            self.append_user_stack_map_entry(inst, entry);
        }
    }

    /// Take the stack map entries for a given instruction, leaving the
    /// instruction without stack maps.
    pub(crate) fn take_user_stack_map_entries(
        &mut self,
        inst: Inst,
    ) -> Option<UserStackMapEntryVec> {
        self.user_stack_maps.remove(&inst)
    }
}

/// Where did a value come from?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueDef {
    /// Value is the n'th result of an instruction.
    Result(Inst, usize),
    /// Value is the n'th parameter to a block.
    Param(Block, usize),
    /// Value is a union of two other values.
    Union(Value, Value),
}

impl ValueDef {
    /// Unwrap the instruction where the value was defined, or panic.
    pub fn unwrap_inst(&self) -> Inst {
        self.inst().expect("Value is not an instruction result")
    }

    /// Get the instruction where the value was defined, if any.
    pub fn inst(&self) -> Option<Inst> {
        match *self {
            Self::Result(inst, _) => Some(inst),
            _ => None,
        }
    }

    /// Unwrap the block there the parameter is defined, or panic.
    pub fn unwrap_block(&self) -> Block {
        match *self {
            Self::Param(block, _) => block,
            _ => panic!("Value is not a block parameter"),
        }
    }

    /// Get the number component of this definition.
    ///
    /// When multiple values are defined at the same program point, this indicates the index of
    /// this value.
    pub fn num(self) -> usize {
        match self {
            Self::Result(_, n) | Self::Param(_, n) => n,
            Self::Union(_, _) => 0,
        }
    }
}

/// Internal table storage for extended values.
#[derive(Clone, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
enum ValueData {
    /// Value is defined by an instruction.
    Inst { ty: Type, num: u16, inst: Inst },

    /// Value is a block parameter.
    Param { ty: Type, num: u16, block: Block },

    /// Value is an alias of another value.
    /// An alias value can't be linked as an instruction result or block parameter. It is used as a
    /// placeholder when the original instruction or block has been rewritten or modified.
    Alias { ty: Type, original: Value },

    /// Union is a "fork" in representation: the value can be
    /// represented as either of the values named here. This is used
    /// for aegraph (acyclic egraph) representation in the DFG.
    Union { ty: Type, x: Value, y: Value },
}

/// Bit-packed version of ValueData, for efficiency.
///
/// Layout:
///
/// ```plain
///        | tag:2 |  type:14        |    x:24       | y:24          |
///
/// Inst       00     ty               inst output     inst index
/// Param      01     ty               blockparam num  block index
/// Alias      10     ty               0               value index
/// Union      11     ty               first value     second value
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
struct ValueDataPacked(u64);

/// Encodes a value in 0..2^32 into 0..2^n, where n is less than 32
/// (and is implied by `mask`), by translating 2^32-1 (0xffffffff)
/// into 2^n-1 and panic'ing on 2^n..2^32-1.
fn encode_narrow_field(x: u32, bits: u8) -> u32 {
    let max = (1 << bits) - 1;
    if x == 0xffff_ffff {
        max
    } else {
        debug_assert!(
            x < max,
            "{x} does not fit into {bits} bits (must be less than {max} to \
             allow for a 0xffffffff sentinel)"
        );
        x
    }
}

/// The inverse of the above `encode_narrow_field`: unpacks 2^n-1 into
/// 2^32-1.
fn decode_narrow_field(x: u32, bits: u8) -> u32 {
    if x == (1 << bits) - 1 { 0xffff_ffff } else { x }
}

impl ValueDataPacked {
    const Y_SHIFT: u8 = 0;
    const Y_BITS: u8 = 24;
    const X_SHIFT: u8 = Self::Y_SHIFT + Self::Y_BITS;
    const X_BITS: u8 = 24;
    const TYPE_SHIFT: u8 = Self::X_SHIFT + Self::X_BITS;
    const TYPE_BITS: u8 = 14;
    const TAG_SHIFT: u8 = Self::TYPE_SHIFT + Self::TYPE_BITS;
    const TAG_BITS: u8 = 2;

    const TAG_INST: u64 = 0;
    const TAG_PARAM: u64 = 1;
    const TAG_ALIAS: u64 = 2;
    const TAG_UNION: u64 = 3;

    fn make(tag: u64, ty: Type, x: u32, y: u32) -> ValueDataPacked {
        debug_assert!(tag < (1 << Self::TAG_BITS));
        debug_assert!(ty.repr() < (1 << Self::TYPE_BITS));

        let x = encode_narrow_field(x, Self::X_BITS);
        let y = encode_narrow_field(y, Self::Y_BITS);

        ValueDataPacked(
            (tag << Self::TAG_SHIFT)
                | ((ty.repr() as u64) << Self::TYPE_SHIFT)
                | ((x as u64) << Self::X_SHIFT)
                | ((y as u64) << Self::Y_SHIFT),
        )
    }

    #[inline(always)]
    fn field(self, shift: u8, bits: u8) -> u64 {
        (self.0 >> shift) & ((1 << bits) - 1)
    }

    #[inline(always)]
    fn ty(self) -> Type {
        let ty = self.field(ValueDataPacked::TYPE_SHIFT, ValueDataPacked::TYPE_BITS) as u16;
        Type::from_repr(ty)
    }

    #[inline(always)]
    fn set_type(&mut self, ty: Type) {
        self.0 &= !(((1 << Self::TYPE_BITS) - 1) << Self::TYPE_SHIFT);
        self.0 |= (ty.repr() as u64) << Self::TYPE_SHIFT;
    }
}

impl From<ValueData> for ValueDataPacked {
    fn from(data: ValueData) -> Self {
        match data {
            ValueData::Inst { ty, num, inst } => {
                Self::make(Self::TAG_INST, ty, num.into(), inst.as_bits())
            }
            ValueData::Param { ty, num, block } => {
                Self::make(Self::TAG_PARAM, ty, num.into(), block.as_bits())
            }
            ValueData::Alias { ty, original } => {
                Self::make(Self::TAG_ALIAS, ty, 0, original.as_bits())
            }
            ValueData::Union { ty, x, y } => {
                Self::make(Self::TAG_UNION, ty, x.as_bits(), y.as_bits())
            }
        }
    }
}

impl From<ValueDataPacked> for ValueData {
    fn from(data: ValueDataPacked) -> Self {
        let tag = data.field(ValueDataPacked::TAG_SHIFT, ValueDataPacked::TAG_BITS);
        let ty = u16::try_from(data.field(ValueDataPacked::TYPE_SHIFT, ValueDataPacked::TYPE_BITS))
            .expect("Mask should ensure result fits in a u16");
        let x = u32::try_from(data.field(ValueDataPacked::X_SHIFT, ValueDataPacked::X_BITS))
            .expect("Mask should ensure result fits in a u32");
        let y = u32::try_from(data.field(ValueDataPacked::Y_SHIFT, ValueDataPacked::Y_BITS))
            .expect("Mask should ensure result fits in a u32");

        let ty = Type::from_repr(ty);
        match tag {
            ValueDataPacked::TAG_INST => ValueData::Inst {
                ty,
                num: u16::try_from(x).expect("Inst result num should fit in u16"),
                inst: Inst::from_bits(decode_narrow_field(y, ValueDataPacked::Y_BITS)),
            },
            ValueDataPacked::TAG_PARAM => ValueData::Param {
                ty,
                num: u16::try_from(x).expect("Blockparam index should fit in u16"),
                block: Block::from_bits(decode_narrow_field(y, ValueDataPacked::Y_BITS)),
            },
            ValueDataPacked::TAG_ALIAS => ValueData::Alias {
                ty,
                original: Value::from_bits(decode_narrow_field(y, ValueDataPacked::Y_BITS)),
            },
            ValueDataPacked::TAG_UNION => ValueData::Union {
                ty,
                x: Value::from_bits(decode_narrow_field(x, ValueDataPacked::X_BITS)),
                y: Value::from_bits(decode_narrow_field(y, ValueDataPacked::Y_BITS)),
            },
            _ => panic!("Invalid tag {} in ValueDataPacked 0x{:x}", tag, data.0),
        }
    }
}

/// Instructions.
///
impl DataFlowGraph {
    /// Create a new instruction.
    ///
    /// The type of the first result is indicated by `data.ty`. If the
    /// instruction produces multiple results, also call
    /// `make_inst_results` to allocate value table entries. (It is
    /// always safe to call `make_inst_results`, regardless of how
    /// many results the instruction has.)
    pub fn make_inst(&mut self, data: InstructionData) -> Inst {
        let n = self.num_insts() + 1;
        self.results.resize(n);
        self.insts.0.push(data)
    }

    /// Declares a dynamic vector type
    pub fn make_dynamic_ty(&mut self, data: DynamicTypeData) -> DynamicType {
        self.dynamic_types.push(data)
    }

    /// Returns an object that displays `inst`.
    pub fn display_inst<'a>(&'a self, inst: Inst) -> DisplayInst<'a> {
        DisplayInst(self, inst)
    }

    /// Returns an object that displays the given `value`'s defining instruction.
    ///
    /// Panics if the value is not defined by an instruction (i.e. it is a basic
    /// block argument).
    pub fn display_value_inst(&self, value: Value) -> DisplayInst<'_> {
        match self.value_def(value) {
            ir::ValueDef::Result(inst, _) => self.display_inst(inst),
            ir::ValueDef::Param(_, _) => panic!("value is not defined by an instruction"),
            ir::ValueDef::Union(_, _) => panic!("value is a union of two other values"),
        }
    }

    /// Construct a read-only visitor context for the values of this instruction.
    pub fn inst_values<'dfg>(
        &'dfg self,
        inst: Inst,
    ) -> impl DoubleEndedIterator<Item = Value> + 'dfg {
        self.inst_args(inst)
            .iter()
            .copied()
            .chain(
                self.insts[inst]
                    .branch_destination(&self.jump_tables, &self.exception_tables)
                    .into_iter()
                    .flat_map(|branch| {
                        branch
                            .args(&self.value_lists)
                            .filter_map(|arg| arg.as_value())
                    }),
            )
            .chain(
                self.insts[inst]
                    .exception_table()
                    .into_iter()
                    .flat_map(|et| self.exception_tables[et].contexts()),
            )
    }

    /// Map a function over the values of the instruction.
    pub fn map_inst_values<F>(&mut self, inst: Inst, body: F)
    where
        F: FnMut(Value) -> Value,
    {
        self.insts[inst].map_values(
            &mut self.value_lists,
            &mut self.jump_tables,
            &mut self.exception_tables,
            body,
        );
    }

    /// Overwrite the instruction's value references with values from the iterator.
    /// NOTE: the iterator provided is expected to yield at least as many values as the instruction
    /// currently has.
    pub fn overwrite_inst_values<I>(&mut self, inst: Inst, mut values: I)
    where
        I: Iterator<Item = Value>,
    {
        self.insts[inst].map_values(
            &mut self.value_lists,
            &mut self.jump_tables,
            &mut self.exception_tables,
            |_| values.next().unwrap(),
        );
    }

    /// Get all value arguments on `inst` as a slice.
    pub fn inst_args(&self, inst: Inst) -> &[Value] {
        self.insts[inst].arguments(&self.value_lists)
    }

    /// Get all value arguments on `inst` as a mutable slice.
    pub fn inst_args_mut(&mut self, inst: Inst) -> &mut [Value] {
        self.insts[inst].arguments_mut(&mut self.value_lists)
    }

    /// Get the fixed value arguments on `inst` as a slice.
    pub fn inst_fixed_args(&self, inst: Inst) -> &[Value] {
        let num_fixed_args = self.insts[inst]
            .opcode()
            .constraints()
            .num_fixed_value_arguments();
        &self.inst_args(inst)[..num_fixed_args]
    }

    /// Get the fixed value arguments on `inst` as a mutable slice.
    pub fn inst_fixed_args_mut(&mut self, inst: Inst) -> &mut [Value] {
        let num_fixed_args = self.insts[inst]
            .opcode()
            .constraints()
            .num_fixed_value_arguments();
        &mut self.inst_args_mut(inst)[..num_fixed_args]
    }

    /// Get the variable value arguments on `inst` as a slice.
    pub fn inst_variable_args(&self, inst: Inst) -> &[Value] {
        let num_fixed_args = self.insts[inst]
            .opcode()
            .constraints()
            .num_fixed_value_arguments();
        &self.inst_args(inst)[num_fixed_args..]
    }

    /// Get the variable value arguments on `inst` as a mutable slice.
    pub fn inst_variable_args_mut(&mut self, inst: Inst) -> &mut [Value] {
        let num_fixed_args = self.insts[inst]
            .opcode()
            .constraints()
            .num_fixed_value_arguments();
        &mut self.inst_args_mut(inst)[num_fixed_args..]
    }

    /// Create result values for an instruction that produces multiple results.
    ///
    /// Instructions that produce no result values only need to be created with `make_inst`,
    /// otherwise call `make_inst_results` to allocate value table entries for the results.
    ///
    /// The result value types are determined from the instruction's value type constraints and the
    /// provided `ctrl_typevar` type for polymorphic instructions. For non-polymorphic
    /// instructions, `ctrl_typevar` is ignored, and `INVALID` can be used.
    ///
    /// The type of the first result value is also set, even if it was already set in the
    /// `InstructionData` passed to `make_inst`. If this function is called with a single-result
    /// instruction, that is the only effect.
    pub fn make_inst_results(&mut self, inst: Inst, ctrl_typevar: Type) -> usize {
        self.make_inst_results_reusing(inst, ctrl_typevar, iter::empty())
    }

    /// Create result values for `inst`, reusing the provided detached values.
    ///
    /// Create a new set of result values for `inst` using `ctrl_typevar` to determine the result
    /// types. Any values provided by `reuse` will be reused. When `reuse` is exhausted or when it
    /// produces `None`, a new value is created.
    pub fn make_inst_results_reusing<I>(
        &mut self,
        inst: Inst,
        ctrl_typevar: Type,
        reuse: I,
    ) -> usize
    where
        I: Iterator<Item = Option<Value>>,
    {
        self.clear_results(inst);

        let mut reuse = reuse.fuse();
        let result_tys: SmallVec<[_; 16]> = self.inst_result_types(inst, ctrl_typevar).collect();

        for (expected, &ty) in result_tys.iter().enumerate() {
            let num = u16::try_from(expected).expect("Result value index should fit in u16");
            let value_data = ValueData::Inst { ty, num, inst };
            let v = if let Some(Some(v)) = reuse.next() {
                debug_assert_eq!(self.value_type(v), ty, "Reused {ty} is wrong type");
                debug_assert!(!self.value_is_attached(v));
                self.values[v] = value_data.into();
                v
            } else {
                self.make_value(value_data)
            };
            let actual = self.results[inst].push(v, &mut self.value_lists);
            debug_assert_eq!(expected, actual);
        }

        result_tys.len()
    }

    /// Create a `ReplaceBuilder` that will replace `inst` with a new instruction in place.
    pub fn replace(&mut self, inst: Inst) -> ReplaceBuilder<'_> {
        ReplaceBuilder::new(self, inst)
    }

    /// Clear the list of result values from `inst`.
    ///
    /// This leaves `inst` without any result values. New result values can be created by calling
    /// `make_inst_results` or by using a `replace(inst)` builder.
    pub fn clear_results(&mut self, inst: Inst) {
        self.results[inst].clear(&mut self.value_lists)
    }

    /// Replace an instruction result with a new value of type `new_type`.
    ///
    /// The `old_value` must be an attached instruction result.
    ///
    /// The old value is left detached, so it should probably be changed into something else.
    ///
    /// Returns the new value.
    pub fn replace_result(&mut self, old_value: Value, new_type: Type) -> Value {
        let (num, inst) = match ValueData::from(self.values[old_value]) {
            ValueData::Inst { num, inst, .. } => (num, inst),
            _ => panic!("{old_value} is not an instruction result value"),
        };
        let new_value = self.make_value(ValueData::Inst {
            ty: new_type,
            num,
            inst,
        });
        let num = num as usize;
        let attached = mem::replace(
            self.results[inst]
                .get_mut(num, &mut self.value_lists)
                .expect("Replacing detached result"),
            new_value,
        );
        debug_assert_eq!(
            attached,
            old_value,
            "{} wasn't detached from {}",
            old_value,
            self.display_inst(inst)
        );
        new_value
    }

    /// Clone an instruction, attaching new result `Value`s and
    /// returning them.
    pub fn clone_inst(&mut self, inst: Inst) -> Inst {
        // First, add a clone of the InstructionData.
        let inst_data = self.insts[inst];
        // If the `inst_data` has a reference to a ValueList, clone it
        // as well, because we can't share these (otherwise mutating
        // one would affect the other).
        let inst_data = inst_data.deep_clone(&mut self.value_lists);
        let new_inst = self.make_inst(inst_data);
        // Get the controlling type variable.
        let ctrl_typevar = self.ctrl_typevar(inst);
        // Create new result values.
        let num_results = self.make_inst_results(new_inst, ctrl_typevar);
        // Copy over PCC facts, if any.
        for i in 0..num_results {
            let old_result = self.inst_results(inst)[i];
            let new_result = self.inst_results(new_inst)[i];
            self.facts[new_result] = self.facts[old_result].clone();
        }
        new_inst
    }

    /// Get the first result of an instruction.
    ///
    /// This function panics if the instruction doesn't have any result.
    pub fn first_result(&self, inst: Inst) -> Value {
        self.results[inst]
            .first(&self.value_lists)
            .unwrap_or_else(|| panic!("{inst} has no results"))
    }

    /// Test if `inst` has any result values currently.
    pub fn has_results(&self, inst: Inst) -> bool {
        !self.results[inst].is_empty()
    }

    /// Return all the results of an instruction.
    pub fn inst_results(&self, inst: Inst) -> &[Value] {
        self.results[inst].as_slice(&self.value_lists)
    }

    /// Return all the results of an instruction as ValueList.
    pub fn inst_results_list(&self, inst: Inst) -> ValueList {
        self.results[inst]
    }

    /// Create a union of two values.
    pub fn union(&mut self, x: Value, y: Value) -> Value {
        // Get the type.
        let ty = self.value_type(x);
        debug_assert_eq!(ty, self.value_type(y));
        self.make_value(ValueData::Union { ty, x, y })
    }

    /// Get the call signature of a direct or indirect call instruction.
    /// Returns `None` if `inst` is not a call instruction.
    pub fn call_signature(&self, inst: Inst) -> Option<SigRef> {
        match self.insts[inst].analyze_call(&self.value_lists, &self.exception_tables) {
            CallInfo::NotACall => None,
            CallInfo::Direct(f, _) => Some(self.ext_funcs[f].signature),
            CallInfo::DirectWithSig(_, s, _) => Some(s),
            CallInfo::Indirect(s, _) => Some(s),
        }
    }

    /// Like `call_signature` but returns none for tail call
    /// instructions and try-call (exception-handling invoke)
    /// instructions.
    fn non_tail_call_or_try_call_signature(&self, inst: Inst) -> Option<SigRef> {
        let sig = self.call_signature(inst)?;
        match self.insts[inst].opcode() {
            ir::Opcode::ReturnCall | ir::Opcode::ReturnCallIndirect => None,
            ir::Opcode::TryCall | ir::Opcode::TryCallIndirect => None,
            _ => Some(sig),
        }
    }

    // Only for use by the verifier. Everyone else should just use
    // `dfg.inst_results(inst).len()`.
    pub(crate) fn num_expected_results_for_verifier(&self, inst: Inst) -> usize {
        match self.non_tail_call_or_try_call_signature(inst) {
            Some(sig) => self.signatures[sig].returns.len(),
            None => {
                let constraints = self.insts[inst].opcode().constraints();
                constraints.num_fixed_results()
            }
        }
    }

    /// Get the result types of the given instruction.
    pub fn inst_result_types<'a>(
        &'a self,
        inst: Inst,
        ctrl_typevar: Type,
    ) -> impl iter::ExactSizeIterator<Item = Type> + 'a {
        return match self.non_tail_call_or_try_call_signature(inst) {
            Some(sig) => InstResultTypes::Signature(self, sig, 0),
            None => {
                let constraints = self.insts[inst].opcode().constraints();
                InstResultTypes::Constraints(constraints, ctrl_typevar, 0)
            }
        };

        enum InstResultTypes<'a> {
            Signature(&'a DataFlowGraph, SigRef, usize),
            Constraints(ir::instructions::OpcodeConstraints, Type, usize),
        }

        impl Iterator for InstResultTypes<'_> {
            type Item = Type;

            fn next(&mut self) -> Option<Type> {
                match self {
                    InstResultTypes::Signature(dfg, sig, i) => {
                        let param = dfg.signatures[*sig].returns.get(*i)?;
                        *i += 1;
                        Some(param.value_type)
                    }
                    InstResultTypes::Constraints(constraints, ctrl_ty, i) => {
                        if *i < constraints.num_fixed_results() {
                            let ty = constraints.result_type(*i, *ctrl_ty);
                            *i += 1;
                            Some(ty)
                        } else {
                            None
                        }
                    }
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = match self {
                    InstResultTypes::Signature(dfg, sig, i) => {
                        dfg.signatures[*sig].returns.len() - *i
                    }
                    InstResultTypes::Constraints(constraints, _, i) => {
                        constraints.num_fixed_results() - *i
                    }
                };
                (len, Some(len))
            }
        }

        impl ExactSizeIterator for InstResultTypes<'_> {}
    }

    /// Compute the type of an instruction result from opcode constraints and call signatures.
    ///
    /// This computes the same sequence of result types that `make_inst_results()` above would
    /// assign to the created result values, but it does not depend on `make_inst_results()` being
    /// called first.
    ///
    /// Returns `None` if asked about a result index that is too large.
    pub fn compute_result_type(
        &self,
        inst: Inst,
        result_idx: usize,
        ctrl_typevar: Type,
    ) -> Option<Type> {
        self.inst_result_types(inst, ctrl_typevar).nth(result_idx)
    }

    /// Get the controlling type variable, or `INVALID` if `inst` isn't polymorphic.
    pub fn ctrl_typevar(&self, inst: Inst) -> Type {
        let constraints = self.insts[inst].opcode().constraints();

        if !constraints.is_polymorphic() {
            types::INVALID
        } else if constraints.requires_typevar_operand() {
            // Not all instruction formats have a designated operand, but in that case
            // `requires_typevar_operand()` should never be true.
            self.value_type(
                self.insts[inst]
                    .typevar_operand(&self.value_lists)
                    .unwrap_or_else(|| {
                        panic!(
                            "Instruction format for {:?} doesn't have a designated operand",
                            self.insts[inst]
                        )
                    }),
            )
        } else {
            self.value_type(self.first_result(inst))
        }
    }
}

/// basic blocks.
impl DataFlowGraph {
    /// Create a new basic block.
    pub fn make_block(&mut self) -> Block {
        self.blocks.add()
    }

    /// Get the number of parameters on `block`.
    pub fn num_block_params(&self, block: Block) -> usize {
        self.blocks[block].params(&self.value_lists).len()
    }

    /// Get the parameters on `block`.
    pub fn block_params(&self, block: Block) -> &[Value] {
        self.blocks[block].params(&self.value_lists)
    }

    /// Get the types of the parameters on `block`.
    pub fn block_param_types(&self, block: Block) -> impl Iterator<Item = Type> + '_ {
        self.block_params(block).iter().map(|&v| self.value_type(v))
    }

    /// Append a parameter with type `ty` to `block`.
    pub fn append_block_param(&mut self, block: Block, ty: Type) -> Value {
        let param = self.values.next_key();
        let num = self.blocks[block].params.push(param, &mut self.value_lists);
        debug_assert!(num <= u16::MAX as usize, "Too many parameters on block");
        self.make_value(ValueData::Param {
            ty,
            num: num as u16,
            block,
        })
    }

    /// Removes `val` from `block`'s parameters by swapping it with the last parameter on `block`.
    /// Returns the position of `val` before removal.
    ///
    /// *Important*: to ensure O(1) deletion, this method swaps the removed parameter with the
    /// last `block` parameter. This can disrupt all the branch instructions jumping to this
    /// `block` for which you have to change the branch argument order if necessary.
    ///
    /// Panics if `val` is not a block parameter.
    pub fn swap_remove_block_param(&mut self, val: Value) -> usize {
        let (block, num) =
            if let ValueData::Param { num, block, .. } = ValueData::from(self.values[val]) {
                (block, num)
            } else {
                panic!("{val} must be a block parameter");
            };
        self.blocks[block]
            .params
            .swap_remove(num as usize, &mut self.value_lists);
        if let Some(last_arg_val) = self.blocks[block]
            .params
            .get(num as usize, &self.value_lists)
        {
            // We update the position of the old last arg.
            let mut last_arg_data = ValueData::from(self.values[last_arg_val]);
            if let ValueData::Param { num: old_num, .. } = &mut last_arg_data {
                *old_num = num;
                self.values[last_arg_val] = last_arg_data.into();
            } else {
                panic!("{last_arg_val} should be a Block parameter");
            }
        }
        num as usize
    }

    /// Removes `val` from `block`'s parameters by a standard linear time list removal which
    /// preserves ordering. Also updates the values' data.
    pub fn remove_block_param(&mut self, val: Value) {
        let (block, num) =
            if let ValueData::Param { num, block, .. } = ValueData::from(self.values[val]) {
                (block, num)
            } else {
                panic!("{val} must be a block parameter");
            };
        self.blocks[block]
            .params
            .remove(num as usize, &mut self.value_lists);
        for index in num..(self.num_block_params(block) as u16) {
            let packed = &mut self.values[self.blocks[block]
                .params
                .get(index as usize, &self.value_lists)
                .unwrap()];
            let mut data = ValueData::from(*packed);
            match &mut data {
                ValueData::Param { num, .. } => {
                    *num -= 1;
                    *packed = data.into();
                }
                _ => panic!(
                    "{} must be a block parameter",
                    self.blocks[block]
                        .params
                        .get(index as usize, &self.value_lists)
                        .unwrap()
                ),
            }
        }
    }

    /// Append an existing value to `block`'s parameters.
    ///
    /// The appended value can't already be attached to something else.
    ///
    /// In almost all cases, you should be using `append_block_param()` instead of this method.
    pub fn attach_block_param(&mut self, block: Block, param: Value) {
        debug_assert!(!self.value_is_attached(param));
        let num = self.blocks[block].params.push(param, &mut self.value_lists);
        debug_assert!(num <= u16::MAX as usize, "Too many parameters on block");
        let ty = self.value_type(param);
        self.values[param] = ValueData::Param {
            ty,
            num: num as u16,
            block,
        }
        .into();
    }

    /// Replace a block parameter with a new value of type `ty`.
    ///
    /// The `old_value` must be an attached block parameter. It is removed from its place in the list
    /// of parameters and replaced by a new value of type `new_type`. The new value gets the same
    /// position in the list, and other parameters are not disturbed.
    ///
    /// The old value is left detached, so it should probably be changed into something else.
    ///
    /// Returns the new value.
    pub fn replace_block_param(&mut self, old_value: Value, new_type: Type) -> Value {
        // Create new value identical to the old one except for the type.
        let (block, num) =
            if let ValueData::Param { num, block, .. } = ValueData::from(self.values[old_value]) {
                (block, num)
            } else {
                panic!("{old_value} must be a block parameter");
            };
        let new_arg = self.make_value(ValueData::Param {
            ty: new_type,
            num,
            block,
        });

        self.blocks[block]
            .params
            .as_mut_slice(&mut self.value_lists)[num as usize] = new_arg;
        new_arg
    }

    /// Detach all the parameters from `block` and return them as a `ValueList`.
    ///
    /// This is a quite low-level operation. Sensible things to do with the detached block parameters
    /// is to put them back on the same block with `attach_block_param()` or change them into aliases
    /// with `change_to_alias()`.
    pub fn detach_block_params(&mut self, block: Block) -> ValueList {
        self.blocks[block].params.take()
    }

    /// Detach all of an instruction's result values.
    ///
    /// This is a quite low-level operation. A sensible thing to do with the
    /// detached results is to change them into aliases with
    /// `change_to_alias()`.
    pub fn detach_inst_results(&mut self, inst: Inst) {
        self.results[inst].clear(&mut self.value_lists);
    }

    /// Merge the facts for two values. If both values have facts and
    /// they differ, both values get a special "conflict" fact that is
    /// never satisfied.
    pub fn merge_facts(&mut self, a: Value, b: Value) {
        let a = self.resolve_aliases(a);
        let b = self.resolve_aliases(b);
        match (&self.facts[a], &self.facts[b]) {
            (Some(a), Some(b)) if a == b => { /* nothing */ }
            (None, None) => { /* nothing */ }
            (Some(a), None) => {
                self.facts[b] = Some(a.clone());
            }
            (None, Some(b)) => {
                self.facts[a] = Some(b.clone());
            }
            (Some(a_fact), Some(b_fact)) => {
                assert_eq!(self.value_type(a), self.value_type(b));
                let merged = Fact::intersect(a_fact, b_fact);
                crate::trace!(
                    "facts merge on {} and {}: {:?}, {:?} -> {:?}",
                    a,
                    b,
                    a_fact,
                    b_fact,
                    merged,
                );
                self.facts[a] = Some(merged.clone());
                self.facts[b] = Some(merged);
            }
        }
    }
}

/// Contents of a basic block.
///
/// Parameters on a basic block are values that dominate everything in the block. All
/// branches to this block must provide matching arguments, and the arguments to the entry block must
/// match the function arguments.
#[derive(Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct BlockData {
    /// List of parameters to this block.
    params: ValueList,
}

impl BlockData {
    fn new() -> Self {
        Self {
            params: ValueList::new(),
        }
    }

    /// Get the parameters on `block`.
    pub fn params<'a>(&self, pool: &'a ValueListPool) -> &'a [Value] {
        self.params.as_slice(pool)
    }
}

/// Object that can display an instruction.
pub struct DisplayInst<'a>(&'a DataFlowGraph, Inst);

impl<'a> fmt::Display for DisplayInst<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dfg = self.0;
        let inst = self.1;

        if let Some((first, rest)) = dfg.inst_results(inst).split_first() {
            write!(f, "{first}")?;
            for v in rest {
                write!(f, ", {v}")?;
            }
            write!(f, " = ")?;
        }

        let typevar = dfg.ctrl_typevar(inst);
        if typevar.is_invalid() {
            write!(f, "{}", dfg.insts[inst].opcode())?;
        } else {
            write!(f, "{}.{}", dfg.insts[inst].opcode(), typevar)?;
        }
        write_operands(f, dfg, inst)
    }
}

/// Parser routines. These routines should not be used outside the parser.
impl DataFlowGraph {
    /// Set the type of a value. This is only for use in the parser, which needs
    /// to create invalid values for index padding which may be reassigned later.
    #[cold]
    fn set_value_type_for_parser(&mut self, v: Value, t: Type) {
        assert_eq!(
            self.value_type(v),
            types::INVALID,
            "this function is only for assigning types to previously invalid values"
        );
        self.values[v].set_type(t);
    }

    /// Check that the given concrete `Type` has been defined in the function.
    pub fn check_dynamic_type(&mut self, ty: Type) -> Option<Type> {
        debug_assert!(ty.is_dynamic_vector());
        if self
            .dynamic_types
            .values()
            .any(|dyn_ty_data| dyn_ty_data.concrete().unwrap() == ty)
        {
            Some(ty)
        } else {
            None
        }
    }

    /// Create result values for `inst`, reusing the provided detached values.
    /// This is similar to `make_inst_results_reusing` except it's only for use
    /// in the parser, which needs to reuse previously invalid values.
    #[cold]
    pub fn make_inst_results_for_parser(
        &mut self,
        inst: Inst,
        ctrl_typevar: Type,
        reuse: &[Value],
    ) -> usize {
        let mut reuse_iter = reuse.iter().copied();
        let result_tys: SmallVec<[_; 16]> = self.inst_result_types(inst, ctrl_typevar).collect();
        for ty in result_tys {
            if ty.is_dynamic_vector() {
                self.check_dynamic_type(ty)
                    .unwrap_or_else(|| panic!("Use of undeclared dynamic type: {ty}"));
            }
            if let Some(v) = reuse_iter.next() {
                self.set_value_type_for_parser(v, ty);
            }
        }

        self.make_inst_results_reusing(inst, ctrl_typevar, reuse.iter().map(|x| Some(*x)))
    }

    /// Similar to `append_block_param`, append a parameter with type `ty` to
    /// `block`, but using value `val`. This is only for use by the parser to
    /// create parameters with specific values.
    #[cold]
    pub fn append_block_param_for_parser(&mut self, block: Block, ty: Type, val: Value) {
        let num = self.blocks[block].params.push(val, &mut self.value_lists);
        assert!(num <= u16::MAX as usize, "Too many parameters on block");
        self.values[val] = ValueData::Param {
            ty,
            num: num as u16,
            block,
        }
        .into();
    }

    /// Create a new value alias. This is only for use by the parser to create
    /// aliases with specific values, and the printer for testing.
    #[cold]
    pub fn make_value_alias_for_serialization(&mut self, src: Value, dest: Value) {
        assert_ne!(src, Value::reserved_value());
        assert_ne!(dest, Value::reserved_value());

        let ty = if self.values.is_valid(src) {
            self.value_type(src)
        } else {
            // As a special case, if we can't resolve the aliasee yet, use INVALID
            // temporarily. It will be resolved later in parsing.
            types::INVALID
        };
        let data = ValueData::Alias { ty, original: src };
        self.values[dest] = data.into();
    }

    /// If `v` is already defined as an alias, return its destination value.
    /// Otherwise return None. This allows the parser to coalesce identical
    /// alias definitions, and the printer to identify an alias's immediate target.
    #[cold]
    pub fn value_alias_dest_for_serialization(&self, v: Value) -> Option<Value> {
        if let ValueData::Alias { original, .. } = ValueData::from(self.values[v]) {
            Some(original)
        } else {
            None
        }
    }

    /// Compute the type of an alias. This is only for use in the parser.
    /// Returns false if an alias cycle was encountered.
    #[cold]
    pub fn set_alias_type_for_parser(&mut self, v: Value) -> bool {
        if let Some(resolved) = maybe_resolve_aliases(&self.values, v) {
            let old_ty = self.value_type(v);
            let new_ty = self.value_type(resolved);
            if old_ty == types::INVALID {
                self.set_value_type_for_parser(v, new_ty);
            } else {
                assert_eq!(old_ty, new_ty);
            }
            true
        } else {
            false
        }
    }

    /// Create an invalid value, to pad the index space. This is only for use by
    /// the parser to pad out the value index space.
    #[cold]
    pub fn make_invalid_value_for_parser(&mut self) {
        let data = ValueData::Alias {
            ty: types::INVALID,
            original: Value::reserved_value(),
        };
        self.make_value(data);
    }

    /// Check if a value reference is valid, while being aware of aliases which
    /// may be unresolved while parsing.
    #[cold]
    pub fn value_is_valid_for_parser(&self, v: Value) -> bool {
        if !self.value_is_valid(v) {
            return false;
        }
        if let ValueData::Alias { ty, .. } = ValueData::from(self.values[v]) {
            ty != types::INVALID
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cursor::{Cursor, FuncCursor};
    use crate::ir::{Function, Opcode, TrapCode};
    use alloc::string::ToString;

    #[test]
    fn make_inst() {
        let mut dfg = DataFlowGraph::new();

        let idata = InstructionData::UnaryImm {
            opcode: Opcode::Iconst,
            imm: 0.into(),
        };
        let inst = dfg.make_inst(idata);

        dfg.make_inst_results(inst, types::I32);
        assert_eq!(inst.to_string(), "inst0");
        assert_eq!(dfg.display_inst(inst).to_string(), "v0 = iconst.i32 0");

        // Immutable reference resolution.
        {
            let immdfg = &dfg;
            let ins = &immdfg.insts[inst];
            assert_eq!(ins.opcode(), Opcode::Iconst);
        }

        // Results.
        let val = dfg.first_result(inst);
        assert_eq!(dfg.inst_results(inst), &[val]);

        assert_eq!(dfg.value_def(val), ValueDef::Result(inst, 0));
        assert_eq!(dfg.value_type(val), types::I32);

        // Replacing results.
        assert!(dfg.value_is_attached(val));
        let v2 = dfg.replace_result(val, types::F64);
        assert!(!dfg.value_is_attached(val));
        assert!(dfg.value_is_attached(v2));
        assert_eq!(dfg.inst_results(inst), &[v2]);
        assert_eq!(dfg.value_def(v2), ValueDef::Result(inst, 0));
        assert_eq!(dfg.value_type(v2), types::F64);
    }

    #[test]
    fn no_results() {
        let mut dfg = DataFlowGraph::new();

        let idata = InstructionData::Trap {
            opcode: Opcode::Trap,
            code: TrapCode::unwrap_user(1),
        };
        let inst = dfg.make_inst(idata);
        assert_eq!(dfg.display_inst(inst).to_string(), "trap user1");

        // Result slice should be empty.
        assert_eq!(dfg.inst_results(inst), &[]);
    }

    #[test]
    fn block() {
        let mut dfg = DataFlowGraph::new();

        let block = dfg.make_block();
        assert_eq!(block.to_string(), "block0");
        assert_eq!(dfg.num_block_params(block), 0);
        assert_eq!(dfg.block_params(block), &[]);
        assert!(dfg.detach_block_params(block).is_empty());
        assert_eq!(dfg.num_block_params(block), 0);
        assert_eq!(dfg.block_params(block), &[]);

        let arg1 = dfg.append_block_param(block, types::F32);
        assert_eq!(arg1.to_string(), "v0");
        assert_eq!(dfg.num_block_params(block), 1);
        assert_eq!(dfg.block_params(block), &[arg1]);

        let arg2 = dfg.append_block_param(block, types::I16);
        assert_eq!(arg2.to_string(), "v1");
        assert_eq!(dfg.num_block_params(block), 2);
        assert_eq!(dfg.block_params(block), &[arg1, arg2]);

        assert_eq!(dfg.value_def(arg1), ValueDef::Param(block, 0));
        assert_eq!(dfg.value_def(arg2), ValueDef::Param(block, 1));
        assert_eq!(dfg.value_type(arg1), types::F32);
        assert_eq!(dfg.value_type(arg2), types::I16);

        // Swap the two block parameters.
        let vlist = dfg.detach_block_params(block);
        assert_eq!(dfg.num_block_params(block), 0);
        assert_eq!(dfg.block_params(block), &[]);
        assert_eq!(vlist.as_slice(&dfg.value_lists), &[arg1, arg2]);
        dfg.attach_block_param(block, arg2);
        let arg3 = dfg.append_block_param(block, types::I32);
        dfg.attach_block_param(block, arg1);
        assert_eq!(dfg.block_params(block), &[arg2, arg3, arg1]);
    }

    #[test]
    fn replace_block_params() {
        let mut dfg = DataFlowGraph::new();

        let block = dfg.make_block();
        let arg1 = dfg.append_block_param(block, types::F32);

        let new1 = dfg.replace_block_param(arg1, types::I64);
        assert_eq!(dfg.value_type(arg1), types::F32);
        assert_eq!(dfg.value_type(new1), types::I64);
        assert_eq!(dfg.block_params(block), &[new1]);

        dfg.attach_block_param(block, arg1);
        assert_eq!(dfg.block_params(block), &[new1, arg1]);

        let new2 = dfg.replace_block_param(arg1, types::I8);
        assert_eq!(dfg.value_type(arg1), types::F32);
        assert_eq!(dfg.value_type(new2), types::I8);
        assert_eq!(dfg.block_params(block), &[new1, new2]);

        dfg.attach_block_param(block, arg1);
        assert_eq!(dfg.block_params(block), &[new1, new2, arg1]);

        let new3 = dfg.replace_block_param(new2, types::I16);
        assert_eq!(dfg.value_type(new1), types::I64);
        assert_eq!(dfg.value_type(new2), types::I8);
        assert_eq!(dfg.value_type(new3), types::I16);
        assert_eq!(dfg.block_params(block), &[new1, new3, arg1]);
    }

    #[test]
    fn swap_remove_block_params() {
        let mut dfg = DataFlowGraph::new();

        let block = dfg.make_block();
        let arg1 = dfg.append_block_param(block, types::F32);
        let arg2 = dfg.append_block_param(block, types::F32);
        let arg3 = dfg.append_block_param(block, types::F32);
        assert_eq!(dfg.block_params(block), &[arg1, arg2, arg3]);

        dfg.swap_remove_block_param(arg1);
        assert_eq!(dfg.value_is_attached(arg1), false);
        assert_eq!(dfg.value_is_attached(arg2), true);
        assert_eq!(dfg.value_is_attached(arg3), true);
        assert_eq!(dfg.block_params(block), &[arg3, arg2]);
        dfg.swap_remove_block_param(arg2);
        assert_eq!(dfg.value_is_attached(arg2), false);
        assert_eq!(dfg.value_is_attached(arg3), true);
        assert_eq!(dfg.block_params(block), &[arg3]);
        dfg.swap_remove_block_param(arg3);
        assert_eq!(dfg.value_is_attached(arg3), false);
        assert_eq!(dfg.block_params(block), &[]);
    }

    #[test]
    fn aliases() {
        use crate::ir::InstBuilder;
        use crate::ir::condcodes::IntCC;

        let mut func = Function::new();
        let block0 = func.dfg.make_block();
        let mut pos = FuncCursor::new(&mut func);
        pos.insert_block(block0);

        // Build a little test program.
        let v1 = pos.ins().iconst(types::I32, 42);

        // Make sure we can resolve value aliases even when values is empty.
        assert_eq!(pos.func.dfg.resolve_aliases(v1), v1);

        let arg0 = pos.func.dfg.append_block_param(block0, types::I32);
        let (s, c) = pos.ins().uadd_overflow(v1, arg0);
        let iadd = match pos.func.dfg.value_def(s) {
            ValueDef::Result(i, 0) => i,
            _ => panic!(),
        };

        // Remove `c` from the result list.
        pos.func.stencil.dfg.results[iadd].remove(1, &mut pos.func.stencil.dfg.value_lists);

        // Replace `uadd_overflow` with a normal `iadd` and an `icmp`.
        pos.func.dfg.replace(iadd).iadd(v1, arg0);
        let c2 = pos.ins().icmp(IntCC::Equal, s, v1);
        pos.func.dfg.change_to_alias(c, c2);

        assert_eq!(pos.func.dfg.resolve_aliases(c2), c2);
        assert_eq!(pos.func.dfg.resolve_aliases(c), c2);
    }

    #[test]
    fn cloning() {
        use crate::ir::InstBuilder;

        let mut func = Function::new();
        let mut sig = Signature::new(crate::isa::CallConv::SystemV);
        sig.params.push(ir::AbiParam::new(types::I32));
        let sig = func.import_signature(sig);
        let block0 = func.dfg.make_block();
        let mut pos = FuncCursor::new(&mut func);
        pos.insert_block(block0);
        let v1 = pos.ins().iconst(types::I32, 0);
        let v2 = pos.ins().iconst(types::I32, 1);
        let call_inst = pos.ins().call_indirect(sig, v1, &[v1]);
        let func = pos.func;

        let call_inst_dup = func.dfg.clone_inst(call_inst);
        func.dfg.inst_args_mut(call_inst)[0] = v2;
        assert_eq!(v1, func.dfg.inst_args(call_inst_dup)[0]);
    }
}
