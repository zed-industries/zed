//! A frontend for building Cranelift IR from other languages.
use crate::ssa::{SSABuilder, SideEffects};
use crate::variable::Variable;
use alloc::vec::Vec;
use core::fmt::{self, Debug};
use cranelift_codegen::cursor::{Cursor, CursorPosition, FuncCursor};
use cranelift_codegen::entity::{EntityRef, EntitySet, PrimaryMap, SecondaryMap};
use cranelift_codegen::ir;
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::{
    AbiParam, Block, DataFlowGraph, DynamicStackSlot, DynamicStackSlotData, ExtFuncData,
    ExternalName, FuncRef, Function, GlobalValue, GlobalValueData, Inst, InstBuilder,
    InstBuilderBase, InstructionData, JumpTable, JumpTableData, LibCall, MemFlags, RelSourceLoc,
    SigRef, Signature, StackSlot, StackSlotData, Type, Value, ValueLabel, ValueLabelAssignments,
    ValueLabelStart, types,
};
use cranelift_codegen::isa::TargetFrontendConfig;
use cranelift_codegen::packed_option::PackedOption;
use cranelift_codegen::traversals::Dfs;
use smallvec::SmallVec;

mod safepoints;

/// Structure used for translating a series of functions into Cranelift IR.
///
/// In order to reduce memory reallocations when compiling multiple functions,
/// [`FunctionBuilderContext`] holds various data structures which are cleared between
/// functions, rather than dropped, preserving the underlying allocations.
#[derive(Default)]
pub struct FunctionBuilderContext {
    ssa: SSABuilder,
    status: SecondaryMap<Block, BlockStatus>,
    variables: PrimaryMap<Variable, Type>,
    stack_map_vars: EntitySet<Variable>,
    stack_map_values: EntitySet<Value>,
    safepoints: safepoints::SafepointSpiller,
}

/// Temporary object used to build a single Cranelift IR [`Function`].
pub struct FunctionBuilder<'a> {
    /// The function currently being built.
    /// This field is public so the function can be re-borrowed.
    pub func: &'a mut Function,

    /// Source location to assign to all new instructions.
    srcloc: ir::SourceLoc,

    func_ctx: &'a mut FunctionBuilderContext,
    position: PackedOption<Block>,
}

#[derive(Clone, Default, Eq, PartialEq)]
enum BlockStatus {
    /// No instructions have been added.
    #[default]
    Empty,
    /// Some instructions have been added, but no terminator.
    Partial,
    /// A terminator has been added; no further instructions may be added.
    Filled,
}

impl FunctionBuilderContext {
    /// Creates a [`FunctionBuilderContext`] structure. The structure is automatically cleared after
    /// each [`FunctionBuilder`] completes translating a function.
    pub fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        let FunctionBuilderContext {
            ssa,
            status,
            variables,
            stack_map_vars,
            stack_map_values,
            safepoints,
        } = self;
        ssa.clear();
        status.clear();
        variables.clear();
        stack_map_values.clear();
        stack_map_vars.clear();
        safepoints.clear();
    }

    fn is_empty(&self) -> bool {
        self.ssa.is_empty() && self.status.is_empty() && self.variables.is_empty()
    }
}

/// Implementation of the [`InstBuilder`] that has
/// one convenience method per Cranelift IR instruction.
pub struct FuncInstBuilder<'short, 'long: 'short> {
    builder: &'short mut FunctionBuilder<'long>,
    block: Block,
}

impl<'short, 'long> FuncInstBuilder<'short, 'long> {
    fn new(builder: &'short mut FunctionBuilder<'long>, block: Block) -> Self {
        Self { builder, block }
    }
}

impl<'short, 'long> InstBuilderBase<'short> for FuncInstBuilder<'short, 'long> {
    fn data_flow_graph(&self) -> &DataFlowGraph {
        &self.builder.func.dfg
    }

    fn data_flow_graph_mut(&mut self) -> &mut DataFlowGraph {
        &mut self.builder.func.dfg
    }

    // This implementation is richer than `InsertBuilder` because we use the data of the
    // instruction being inserted to add related info to the DFG and the SSA building system,
    // and perform debug sanity checks.
    fn build(self, data: InstructionData, ctrl_typevar: Type) -> (Inst, &'short mut DataFlowGraph) {
        // We only insert the Block in the layout when an instruction is added to it
        self.builder.ensure_inserted_block();

        let inst = self.builder.func.dfg.make_inst(data);
        self.builder.func.dfg.make_inst_results(inst, ctrl_typevar);
        self.builder.func.layout.append_inst(inst, self.block);
        if !self.builder.srcloc.is_default() {
            self.builder.func.set_srcloc(inst, self.builder.srcloc);
        }

        match &self.builder.func.dfg.insts[inst] {
            ir::InstructionData::Jump {
                destination: dest, ..
            } => {
                // If the user has supplied jump arguments we must adapt the arguments of
                // the destination block
                let block = dest.block(&self.builder.func.dfg.value_lists);
                self.builder.declare_successor(block, inst);
            }

            ir::InstructionData::Brif {
                blocks: [branch_then, branch_else],
                ..
            } => {
                let block_then = branch_then.block(&self.builder.func.dfg.value_lists);
                let block_else = branch_else.block(&self.builder.func.dfg.value_lists);

                self.builder.declare_successor(block_then, inst);
                if block_then != block_else {
                    self.builder.declare_successor(block_else, inst);
                }
            }

            ir::InstructionData::BranchTable { table, .. } => {
                let pool = &self.builder.func.dfg.value_lists;

                // Unlike most other jumps/branches and like try_call,
                // jump tables are capable of having the same successor appear
                // multiple times, so we must deduplicate.
                let mut unique = EntitySet::<Block>::new();
                for dest_block in self
                    .builder
                    .func
                    .stencil
                    .dfg
                    .jump_tables
                    .get(*table)
                    .expect("you are referencing an undeclared jump table")
                    .all_branches()
                {
                    let block = dest_block.block(pool);
                    if !unique.insert(block) {
                        continue;
                    }

                    // Call `declare_block_predecessor` instead of `declare_successor` for
                    // avoiding the borrow checker.
                    self.builder
                        .func_ctx
                        .ssa
                        .declare_block_predecessor(block, inst);
                }
            }

            ir::InstructionData::TryCall { exception, .. }
            | ir::InstructionData::TryCallIndirect { exception, .. } => {
                let pool = &self.builder.func.dfg.value_lists;

                // Unlike most other jumps/branches and like br_table,
                // exception tables are capable of having the same successor
                // appear multiple times, so we must deduplicate.
                let mut unique = EntitySet::<Block>::new();
                for dest_block in self
                    .builder
                    .func
                    .stencil
                    .dfg
                    .exception_tables
                    .get(*exception)
                    .expect("you are referencing an undeclared exception table")
                    .all_branches()
                {
                    let block = dest_block.block(pool);
                    if !unique.insert(block) {
                        continue;
                    }

                    // Call `declare_block_predecessor` instead of `declare_successor` for
                    // avoiding the borrow checker.
                    self.builder
                        .func_ctx
                        .ssa
                        .declare_block_predecessor(block, inst);
                }
            }

            inst => assert!(!inst.opcode().is_branch()),
        }

        if data.opcode().is_terminator() {
            self.builder.fill_current_block()
        }
        (inst, &mut self.builder.func.dfg)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// An error encountered when calling [`FunctionBuilder::try_use_var`].
pub enum UseVariableError {
    UsedBeforeDeclared(Variable),
}

impl fmt::Display for UseVariableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UseVariableError::UsedBeforeDeclared(variable) => {
                write!(
                    f,
                    "variable {} was used before it was defined",
                    variable.index()
                )?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for UseVariableError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// An error encountered when defining the initial value of a variable.
pub enum DefVariableError {
    /// The variable was instantiated with a value of the wrong type.
    ///
    /// note: to obtain the type of the value, you can call
    /// [`cranelift_codegen::ir::dfg::DataFlowGraph::value_type`] (using the
    /// `FunctionBuilder.func.dfg` field)
    TypeMismatch(Variable, Value),
    /// The value was defined (in a call to [`FunctionBuilder::def_var`]) before
    /// it was declared (in a call to [`FunctionBuilder::declare_var`]).
    DefinedBeforeDeclared(Variable),
}

impl fmt::Display for DefVariableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefVariableError::TypeMismatch(variable, value) => {
                write!(
                    f,
                    "the types of variable {} and value {} are not the same.
                    The `Value` supplied to `def_var` must be of the same type as
                    the variable was declared to be of in `declare_var`.",
                    variable.index(),
                    value.as_u32()
                )?;
            }
            DefVariableError::DefinedBeforeDeclared(variable) => {
                write!(
                    f,
                    "the value of variable {} was declared before it was defined",
                    variable.index()
                )?;
            }
        }
        Ok(())
    }
}

/// This module allows you to create a function in Cranelift IR in a straightforward way, hiding
/// all the complexity of its internal representation.
///
/// The module is parametrized by one type which is the representation of variables in your
/// origin language. It offers a way to conveniently append instruction to your program flow.
/// You are responsible to split your instruction flow into extended blocks (declared with
/// [`create_block`](Self::create_block)) whose properties are:
///
/// - branch and jump instructions can only point at the top of extended blocks;
/// - the last instruction of each block is a terminator instruction which has no natural successor,
///   and those instructions can only appear at the end of extended blocks.
///
/// The parameters of Cranelift IR instructions are Cranelift IR values, which can only be created
/// as results of other Cranelift IR instructions. To be able to create variables redefined multiple
/// times in your program, use the [`def_var`](Self::def_var) and [`use_var`](Self::use_var) command,
/// that will maintain the correspondence between your variables and Cranelift IR SSA values.
///
/// The first block for which you call [`switch_to_block`](Self::switch_to_block) will be assumed to
/// be the beginning of the function.
///
/// At creation, a [`FunctionBuilder`] instance borrows an already allocated `Function` which it
/// modifies with the information stored in the mutable borrowed
/// [`FunctionBuilderContext`]. The function passed in argument should be newly created with
/// [`Function::with_name_signature()`], whereas the [`FunctionBuilderContext`] can be kept as is
/// between two function translations.
///
/// # Errors
///
/// The functions below will panic in debug mode whenever you try to modify the Cranelift IR
/// function in a way that violate the coherence of the code. For instance: switching to a new
/// [`Block`] when you haven't filled the current one with a terminator instruction, inserting a
/// return instruction with arguments that don't match the function's signature.
impl<'a> FunctionBuilder<'a> {
    /// Creates a new [`FunctionBuilder`] structure that will operate on a [`Function`] using a
    /// [`FunctionBuilderContext`].
    pub fn new(func: &'a mut Function, func_ctx: &'a mut FunctionBuilderContext) -> Self {
        debug_assert!(func_ctx.is_empty());
        Self {
            func,
            srcloc: Default::default(),
            func_ctx,
            position: Default::default(),
        }
    }

    /// Get the block that this builder is currently at.
    pub fn current_block(&self) -> Option<Block> {
        self.position.expand()
    }

    /// Set the source location that should be assigned to all new instructions.
    pub fn set_srcloc(&mut self, srcloc: ir::SourceLoc) {
        self.srcloc = srcloc;
    }

    /// Creates a new [`Block`] and returns its reference.
    pub fn create_block(&mut self) -> Block {
        let block = self.func.dfg.make_block();
        self.func_ctx.ssa.declare_block(block);
        block
    }

    /// Mark a block as "cold".
    ///
    /// This will try to move it out of the ordinary path of execution
    /// when lowered to machine code.
    pub fn set_cold_block(&mut self, block: Block) {
        self.func.layout.set_cold(block);
    }

    /// Insert `block` in the layout *after* the existing block `after`.
    pub fn insert_block_after(&mut self, block: Block, after: Block) {
        self.func.layout.insert_block_after(block, after);
    }

    /// After the call to this function, new instructions will be inserted into the designated
    /// block, in the order they are declared. You must declare the types of the [`Block`] arguments
    /// you will use here.
    ///
    /// When inserting the terminator instruction (which doesn't have a fallthrough to its immediate
    /// successor), the block will be declared filled and it will not be possible to append
    /// instructions to it.
    pub fn switch_to_block(&mut self, block: Block) {
        log::trace!("switch to {block:?}");

        // First we check that the previous block has been filled.
        debug_assert!(
            self.position.is_none()
                || self.is_unreachable()
                || self.is_pristine(self.position.unwrap())
                || self.is_filled(self.position.unwrap()),
            "you have to fill your block before switching"
        );
        // We cannot switch to a filled block
        debug_assert!(
            !self.is_filled(block),
            "you cannot switch to a block which is already filled"
        );

        // Then we change the cursor position.
        self.position = PackedOption::from(block);
    }

    /// Declares that all the predecessors of this block are known.
    ///
    /// Function to call with `block` as soon as the last branch instruction to `block` has been
    /// created. Forgetting to call this method on every block will cause inconsistencies in the
    /// produced functions.
    pub fn seal_block(&mut self, block: Block) {
        let side_effects = self.func_ctx.ssa.seal_block(block, self.func);
        self.handle_ssa_side_effects(side_effects);
    }

    /// Effectively calls [seal_block](Self::seal_block) on all unsealed blocks in the function.
    ///
    /// It's more efficient to seal [`Block`]s as soon as possible, during
    /// translation, but for frontends where this is impractical to do, this
    /// function can be used at the end of translating all blocks to ensure
    /// that everything is sealed.
    pub fn seal_all_blocks(&mut self) {
        let side_effects = self.func_ctx.ssa.seal_all_blocks(self.func);
        self.handle_ssa_side_effects(side_effects);
    }

    /// Declares the type of a variable.
    ///
    /// This allows the variable to be defined and used later (by calling
    /// [`FunctionBuilder::def_var`] and [`FunctionBuilder::use_var`]
    /// respectively).
    pub fn declare_var(&mut self, ty: Type) -> Variable {
        self.func_ctx.variables.push(ty)
    }

    /// Declare that all uses of the given variable must be included in stack
    /// map metadata.
    ///
    /// All values that are uses of this variable will be spilled to the stack
    /// before each safepoint and their location on the stack included in stack
    /// maps. Stack maps allow the garbage collector to identify the on-stack GC
    /// roots.
    ///
    /// This does not affect any pre-existing uses of the variable.
    ///
    /// # Panics
    ///
    /// Panics if the variable's type is larger than 16 bytes or if this
    /// variable has not been declared yet.
    pub fn declare_var_needs_stack_map(&mut self, var: Variable) {
        log::trace!("declare_var_needs_stack_map({var:?})");
        let ty = self.func_ctx.variables[var];
        assert!(ty != types::INVALID);
        assert!(ty.bytes() <= 16);
        self.func_ctx.stack_map_vars.insert(var);
    }

    /// Returns the Cranelift IR necessary to use a previously defined user
    /// variable, returning an error if this is not possible.
    pub fn try_use_var(&mut self, var: Variable) -> Result<Value, UseVariableError> {
        // Assert that we're about to add instructions to this block using the definition of the
        // given variable. ssa.use_var is the only part of this crate which can add block parameters
        // behind the caller's back. If we disallow calling append_block_param as soon as use_var is
        // called, then we enforce a strict separation between user parameters and SSA parameters.
        self.ensure_inserted_block();

        let (val, side_effects) = {
            let ty = *self
                .func_ctx
                .variables
                .get(var)
                .ok_or(UseVariableError::UsedBeforeDeclared(var))?;
            debug_assert_ne!(
                ty,
                types::INVALID,
                "variable {var:?} is used but its type has not been declared"
            );
            self.func_ctx
                .ssa
                .use_var(self.func, var, ty, self.position.unwrap())
        };
        self.handle_ssa_side_effects(side_effects);

        Ok(val)
    }

    /// Returns the Cranelift IR value corresponding to the utilization at the current program
    /// position of a previously defined user variable.
    pub fn use_var(&mut self, var: Variable) -> Value {
        self.try_use_var(var).unwrap_or_else(|_| {
            panic!("variable {var:?} is used but its type has not been declared")
        })
    }

    /// Registers a new definition of a user variable. This function will return
    /// an error if the value supplied does not match the type the variable was
    /// declared to have.
    pub fn try_def_var(&mut self, var: Variable, val: Value) -> Result<(), DefVariableError> {
        log::trace!("try_def_var: {var:?} = {val:?}");

        let var_ty = *self
            .func_ctx
            .variables
            .get(var)
            .ok_or(DefVariableError::DefinedBeforeDeclared(var))?;
        if var_ty != self.func.dfg.value_type(val) {
            return Err(DefVariableError::TypeMismatch(var, val));
        }

        self.func_ctx.ssa.def_var(var, val, self.position.unwrap());
        Ok(())
    }

    /// Register a new definition of a user variable. The type of the value must be
    /// the same as the type registered for the variable.
    pub fn def_var(&mut self, var: Variable, val: Value) {
        self.try_def_var(var, val)
            .unwrap_or_else(|error| match error {
                DefVariableError::TypeMismatch(var, val) => {
                    panic!("declared type of variable {var:?} doesn't match type of value {val}");
                }
                DefVariableError::DefinedBeforeDeclared(var) => {
                    panic!("variable {var:?} is used but its type has not been declared");
                }
            })
    }

    /// Set label for [`Value`]
    ///
    /// This will not do anything unless
    /// [`func.dfg.collect_debug_info`](DataFlowGraph::collect_debug_info) is called first.
    pub fn set_val_label(&mut self, val: Value, label: ValueLabel) {
        if let Some(values_labels) = self.func.stencil.dfg.values_labels.as_mut() {
            use alloc::collections::btree_map::Entry;

            let start = ValueLabelStart {
                from: RelSourceLoc::from_base_offset(self.func.params.base_srcloc(), self.srcloc),
                label,
            };

            match values_labels.entry(val) {
                Entry::Occupied(mut e) => match e.get_mut() {
                    ValueLabelAssignments::Starts(starts) => starts.push(start),
                    _ => panic!("Unexpected ValueLabelAssignments at this stage"),
                },
                Entry::Vacant(e) => {
                    e.insert(ValueLabelAssignments::Starts(vec![start]));
                }
            }
        }
    }

    /// Declare that the given value is a GC reference that requires inclusion
    /// in a stack map when it is live across GC safepoints.
    ///
    /// At the current moment, values that need inclusion in stack maps are
    /// spilled before safepoints, but they are not reloaded afterwards. This
    /// means that moving GCs are not yet supported, however the intention is to
    /// add this support in the near future.
    ///
    /// # Panics
    ///
    /// Panics if `val` is larger than 16 bytes.
    pub fn declare_value_needs_stack_map(&mut self, val: Value) {
        log::trace!("declare_value_needs_stack_map({val:?})");

        // We rely on these properties in `insert_safepoint_spills`.
        let size = self.func.dfg.value_type(val).bytes();
        assert!(size <= 16);
        assert!(size.is_power_of_two());

        self.func_ctx.stack_map_values.insert(val);
    }

    /// Creates a jump table in the function, to be used by [`br_table`](InstBuilder::br_table) instructions.
    pub fn create_jump_table(&mut self, data: JumpTableData) -> JumpTable {
        self.func.create_jump_table(data)
    }

    /// Creates a sized stack slot in the function, to be used by [`stack_load`](InstBuilder::stack_load),
    /// [`stack_store`](InstBuilder::stack_store) and [`stack_addr`](InstBuilder::stack_addr) instructions.
    pub fn create_sized_stack_slot(&mut self, data: StackSlotData) -> StackSlot {
        self.func.create_sized_stack_slot(data)
    }

    /// Creates a dynamic stack slot in the function, to be used by
    /// [`dynamic_stack_load`](InstBuilder::dynamic_stack_load),
    /// [`dynamic_stack_store`](InstBuilder::dynamic_stack_store) and
    /// [`dynamic_stack_addr`](InstBuilder::dynamic_stack_addr) instructions.
    pub fn create_dynamic_stack_slot(&mut self, data: DynamicStackSlotData) -> DynamicStackSlot {
        self.func.create_dynamic_stack_slot(data)
    }

    /// Adds a signature which can later be used to declare an external function import.
    pub fn import_signature(&mut self, signature: Signature) -> SigRef {
        self.func.import_signature(signature)
    }

    /// Declare an external function import.
    pub fn import_function(&mut self, data: ExtFuncData) -> FuncRef {
        self.func.import_function(data)
    }

    /// Declares a global value accessible to the function.
    pub fn create_global_value(&mut self, data: GlobalValueData) -> GlobalValue {
        self.func.create_global_value(data)
    }

    /// Returns an object with the [`InstBuilder`]
    /// trait that allows to conveniently append an instruction to the current [`Block`] being built.
    pub fn ins<'short>(&'short mut self) -> FuncInstBuilder<'short, 'a> {
        let block = self
            .position
            .expect("Please call switch_to_block before inserting instructions");
        FuncInstBuilder::new(self, block)
    }

    /// Make sure that the current block is inserted in the layout.
    pub fn ensure_inserted_block(&mut self) {
        let block = self.position.unwrap();
        if self.is_pristine(block) {
            if !self.func.layout.is_block_inserted(block) {
                self.func.layout.append_block(block);
            }
            self.func_ctx.status[block] = BlockStatus::Partial;
        } else {
            debug_assert!(
                !self.is_filled(block),
                "you cannot add an instruction to a block already filled"
            );
        }
    }

    /// Returns a [`FuncCursor`] pointed at the current position ready for inserting instructions.
    ///
    /// This can be used to insert SSA code that doesn't need to access locals and that doesn't
    /// need to know about [`FunctionBuilder`] at all.
    pub fn cursor(&mut self) -> FuncCursor<'_> {
        self.ensure_inserted_block();
        FuncCursor::new(self.func)
            .with_srcloc(self.srcloc)
            .at_bottom(self.position.unwrap())
    }

    /// Append parameters to the given [`Block`] corresponding to the function
    /// parameters. This can be used to set up the block parameters for the
    /// entry block.
    pub fn append_block_params_for_function_params(&mut self, block: Block) {
        debug_assert!(
            !self.func_ctx.ssa.has_any_predecessors(block),
            "block parameters for function parameters should only be added to the entry block"
        );

        // These parameters count as "user" parameters here because they aren't
        // inserted by the SSABuilder.
        debug_assert!(
            self.is_pristine(block),
            "You can't add block parameters after adding any instruction"
        );

        for argtyp in &self.func.stencil.signature.params {
            self.func
                .stencil
                .dfg
                .append_block_param(block, argtyp.value_type);
        }
    }

    /// Append parameters to the given [`Block`] corresponding to the function
    /// return values. This can be used to set up the block parameters for a
    /// function exit block.
    pub fn append_block_params_for_function_returns(&mut self, block: Block) {
        // These parameters count as "user" parameters here because they aren't
        // inserted by the SSABuilder.
        debug_assert!(
            self.is_pristine(block),
            "You can't add block parameters after adding any instruction"
        );

        for argtyp in &self.func.stencil.signature.returns {
            self.func
                .stencil
                .dfg
                .append_block_param(block, argtyp.value_type);
        }
    }

    /// Declare that translation of the current function is complete.
    ///
    /// This resets the state of the [`FunctionBuilderContext`] in preparation to
    /// be used for another function.
    pub fn finalize(mut self) {
        // Check that all the `Block`s are filled and sealed.
        #[cfg(debug_assertions)]
        {
            for block in self.func_ctx.status.keys() {
                if !self.is_pristine(block) {
                    assert!(
                        self.func_ctx.ssa.is_sealed(block),
                        "FunctionBuilder finalized, but block {block} is not sealed",
                    );
                    assert!(
                        self.is_filled(block),
                        "FunctionBuilder finalized, but block {block} is not filled",
                    );
                }
            }
        }

        // In debug mode, check that all blocks are valid basic blocks.
        #[cfg(debug_assertions)]
        {
            // Iterate manually to provide more helpful error messages.
            for block in self.func_ctx.status.keys() {
                if let Err((inst, msg)) = self.func.is_block_basic(block) {
                    let inst_str = self.func.dfg.display_inst(inst);
                    panic!("{block} failed basic block invariants on {inst_str}: {msg}");
                }
            }
        }

        // Propagate the needs-stack-map bit from variables to each of their
        // associated values.
        for var in self.func_ctx.stack_map_vars.iter() {
            for val in self.func_ctx.ssa.values_for_var(var) {
                log::trace!("propagating needs-stack-map from {var:?} to {val:?}");
                debug_assert_eq!(self.func.dfg.value_type(val), self.func_ctx.variables[var]);
                self.func_ctx.stack_map_values.insert(val);
            }
        }

        // If we have any values that need inclusion in stack maps, then we need
        // to run our pass to spill those values to the stack at safepoints and
        // generate stack maps.
        if !self.func_ctx.stack_map_values.is_empty() {
            self.func_ctx
                .safepoints
                .run(&mut self.func, &self.func_ctx.stack_map_values);
        }

        // Clear the state (but preserve the allocated buffers) in preparation
        // for translation another function.
        self.func_ctx.clear();
    }
}

/// All the functions documented in the previous block are write-only and help you build a valid
/// Cranelift IR functions via multiple debug asserts. However, you might need to improve the
/// performance of your translation perform more complex transformations to your Cranelift IR
/// function. The functions below help you inspect the function you're creating and modify it
/// in ways that can be unsafe if used incorrectly.
impl<'a> FunctionBuilder<'a> {
    /// Retrieves all the parameters for a [`Block`] currently inferred from the jump instructions
    /// inserted that target it and the SSA construction.
    pub fn block_params(&self, block: Block) -> &[Value] {
        self.func.dfg.block_params(block)
    }

    /// Retrieves the signature with reference `sigref` previously added with
    /// [`import_signature`](Self::import_signature).
    pub fn signature(&self, sigref: SigRef) -> Option<&Signature> {
        self.func.dfg.signatures.get(sigref)
    }

    /// Creates a parameter for a specific [`Block`] by appending it to the list of already existing
    /// parameters.
    ///
    /// **Note:** this function has to be called at the creation of the `Block` before adding
    /// instructions to it, otherwise this could interfere with SSA construction.
    pub fn append_block_param(&mut self, block: Block, ty: Type) -> Value {
        debug_assert!(
            self.is_pristine(block),
            "You can't add block parameters after adding any instruction"
        );
        self.func.dfg.append_block_param(block, ty)
    }

    /// Returns the result values of an instruction.
    pub fn inst_results(&self, inst: Inst) -> &[Value] {
        self.func.dfg.inst_results(inst)
    }

    /// Changes the destination of a jump instruction after creation.
    ///
    /// **Note:** You are responsible for maintaining the coherence with the arguments of
    /// other jump instructions.
    pub fn change_jump_destination(&mut self, inst: Inst, old_block: Block, new_block: Block) {
        let dfg = &mut self.func.dfg;
        for block in
            dfg.insts[inst].branch_destination_mut(&mut dfg.jump_tables, &mut dfg.exception_tables)
        {
            if block.block(&dfg.value_lists) == old_block {
                self.func_ctx.ssa.remove_block_predecessor(old_block, inst);
                block.set_block(new_block, &mut dfg.value_lists);
                self.func_ctx.ssa.declare_block_predecessor(new_block, inst);
            }
        }
    }

    /// Returns `true` if and only if the current [`Block`] is sealed and has no predecessors declared.
    ///
    /// The entry block of a function is never unreachable.
    pub fn is_unreachable(&self) -> bool {
        let is_entry = match self.func.layout.entry_block() {
            None => false,
            Some(entry) => self.position.unwrap() == entry,
        };
        !is_entry
            && self.func_ctx.ssa.is_sealed(self.position.unwrap())
            && !self
                .func_ctx
                .ssa
                .has_any_predecessors(self.position.unwrap())
    }

    /// Returns `true` if and only if no instructions have been added since the last call to
    /// [`switch_to_block`](Self::switch_to_block).
    fn is_pristine(&self, block: Block) -> bool {
        self.func_ctx.status[block] == BlockStatus::Empty
    }

    /// Returns `true` if and only if a terminator instruction has been inserted since the
    /// last call to [`switch_to_block`](Self::switch_to_block).
    fn is_filled(&self, block: Block) -> bool {
        self.func_ctx.status[block] == BlockStatus::Filled
    }
}

/// Helper functions
impl<'a> FunctionBuilder<'a> {
    /// Calls libc.memcpy
    ///
    /// Copies the `size` bytes from `src` to `dest`, assumes that `src + size`
    /// won't overlap onto `dest`. If `dest` and `src` overlap, the behavior is
    /// undefined. Applications in which `dest` and `src` might overlap should
    /// use `call_memmove` instead.
    pub fn call_memcpy(
        &mut self,
        config: TargetFrontendConfig,
        dest: Value,
        src: Value,
        size: Value,
    ) {
        let pointer_type = config.pointer_type();
        let signature = {
            let mut s = Signature::new(config.default_call_conv);
            s.params.push(AbiParam::new(pointer_type));
            s.params.push(AbiParam::new(pointer_type));
            s.params.push(AbiParam::new(pointer_type));
            s.returns.push(AbiParam::new(pointer_type));
            self.import_signature(s)
        };

        let libc_memcpy = self.import_function(ExtFuncData {
            name: ExternalName::LibCall(LibCall::Memcpy),
            signature,
            colocated: false,
        });

        self.ins().call(libc_memcpy, &[dest, src, size]);
    }

    /// Optimised memcpy or memmove for small copies.
    ///
    /// # Codegen safety
    ///
    /// The following properties must hold to prevent UB:
    ///
    /// * `src_align` and `dest_align` are an upper-bound on the alignment of `src` respectively `dest`.
    /// * If `non_overlapping` is true, then this must be correct.
    pub fn emit_small_memory_copy(
        &mut self,
        config: TargetFrontendConfig,
        dest: Value,
        src: Value,
        size: u64,
        dest_align: u8,
        src_align: u8,
        non_overlapping: bool,
        mut flags: MemFlags,
    ) {
        // Currently the result of guess work, not actual profiling.
        const THRESHOLD: u64 = 4;

        if size == 0 {
            return;
        }

        let access_size = greatest_divisible_power_of_two(size);
        assert!(
            access_size.is_power_of_two(),
            "`size` is not a power of two"
        );
        assert!(
            access_size >= u64::from(::core::cmp::min(src_align, dest_align)),
            "`size` is smaller than `dest` and `src`'s alignment value."
        );

        let (access_size, int_type) = if access_size <= 8 {
            (access_size, Type::int((access_size * 8) as u16).unwrap())
        } else {
            (8, types::I64)
        };

        let load_and_store_amount = size / access_size;

        if load_and_store_amount > THRESHOLD {
            let size_value = self.ins().iconst(config.pointer_type(), size as i64);
            if non_overlapping {
                self.call_memcpy(config, dest, src, size_value);
            } else {
                self.call_memmove(config, dest, src, size_value);
            }
            return;
        }

        if u64::from(src_align) >= access_size && u64::from(dest_align) >= access_size {
            flags.set_aligned();
        }

        // Load all of the memory first. This is necessary in case `dest` overlaps.
        // It can also improve performance a bit.
        let registers: smallvec::SmallVec<[_; THRESHOLD as usize]> = (0..load_and_store_amount)
            .map(|i| {
                let offset = (access_size * i) as i32;
                (self.ins().load(int_type, flags, src, offset), offset)
            })
            .collect();

        for (value, offset) in registers {
            self.ins().store(flags, value, dest, offset);
        }
    }

    /// Calls libc.memset
    ///
    /// Writes `size` bytes of i8 value `ch` to memory starting at `buffer`.
    pub fn call_memset(
        &mut self,
        config: TargetFrontendConfig,
        buffer: Value,
        ch: Value,
        size: Value,
    ) {
        let pointer_type = config.pointer_type();
        let signature = {
            let mut s = Signature::new(config.default_call_conv);
            s.params.push(AbiParam::new(pointer_type));
            s.params.push(AbiParam::new(types::I32));
            s.params.push(AbiParam::new(pointer_type));
            s.returns.push(AbiParam::new(pointer_type));
            self.import_signature(s)
        };

        let libc_memset = self.import_function(ExtFuncData {
            name: ExternalName::LibCall(LibCall::Memset),
            signature,
            colocated: false,
        });

        let ch = self.ins().uextend(types::I32, ch);
        self.ins().call(libc_memset, &[buffer, ch, size]);
    }

    /// Calls libc.memset
    ///
    /// Writes `size` bytes of value `ch` to memory starting at `buffer`.
    pub fn emit_small_memset(
        &mut self,
        config: TargetFrontendConfig,
        buffer: Value,
        ch: u8,
        size: u64,
        buffer_align: u8,
        mut flags: MemFlags,
    ) {
        // Currently the result of guess work, not actual profiling.
        const THRESHOLD: u64 = 4;

        if size == 0 {
            return;
        }

        let access_size = greatest_divisible_power_of_two(size);
        assert!(
            access_size.is_power_of_two(),
            "`size` is not a power of two"
        );
        assert!(
            access_size >= u64::from(buffer_align),
            "`size` is smaller than `dest` and `src`'s alignment value."
        );

        let (access_size, int_type) = if access_size <= 8 {
            (access_size, Type::int((access_size * 8) as u16).unwrap())
        } else {
            (8, types::I64)
        };

        let load_and_store_amount = size / access_size;

        if load_and_store_amount > THRESHOLD {
            let ch = self.ins().iconst(types::I8, i64::from(ch));
            let size = self.ins().iconst(config.pointer_type(), size as i64);
            self.call_memset(config, buffer, ch, size);
        } else {
            if u64::from(buffer_align) >= access_size {
                flags.set_aligned();
            }

            let ch = u64::from(ch);
            let raw_value = if int_type == types::I64 {
                ch * 0x0101010101010101_u64
            } else if int_type == types::I32 {
                ch * 0x01010101_u64
            } else if int_type == types::I16 {
                (ch << 8) | ch
            } else {
                assert_eq!(int_type, types::I8);
                ch
            };

            let value = self.ins().iconst(int_type, raw_value as i64);
            for i in 0..load_and_store_amount {
                let offset = (access_size * i) as i32;
                self.ins().store(flags, value, buffer, offset);
            }
        }
    }

    /// Calls libc.memmove
    ///
    /// Copies `size` bytes from memory starting at `source` to memory starting
    /// at `dest`. `source` is always read before writing to `dest`.
    pub fn call_memmove(
        &mut self,
        config: TargetFrontendConfig,
        dest: Value,
        source: Value,
        size: Value,
    ) {
        let pointer_type = config.pointer_type();
        let signature = {
            let mut s = Signature::new(config.default_call_conv);
            s.params.push(AbiParam::new(pointer_type));
            s.params.push(AbiParam::new(pointer_type));
            s.params.push(AbiParam::new(pointer_type));
            s.returns.push(AbiParam::new(pointer_type));
            self.import_signature(s)
        };

        let libc_memmove = self.import_function(ExtFuncData {
            name: ExternalName::LibCall(LibCall::Memmove),
            signature,
            colocated: false,
        });

        self.ins().call(libc_memmove, &[dest, source, size]);
    }

    /// Calls libc.memcmp
    ///
    /// Compares `size` bytes from memory starting at `left` to memory starting
    /// at `right`. Returns `0` if all `n` bytes are equal.  If the first difference
    /// is at offset `i`, returns a positive integer if `ugt(left[i], right[i])`
    /// and a negative integer if `ult(left[i], right[i])`.
    ///
    /// Returns a C `int`, which is currently always [`types::I32`].
    pub fn call_memcmp(
        &mut self,
        config: TargetFrontendConfig,
        left: Value,
        right: Value,
        size: Value,
    ) -> Value {
        let pointer_type = config.pointer_type();
        let signature = {
            let mut s = Signature::new(config.default_call_conv);
            s.params.reserve(3);
            s.params.push(AbiParam::new(pointer_type));
            s.params.push(AbiParam::new(pointer_type));
            s.params.push(AbiParam::new(pointer_type));
            s.returns.push(AbiParam::new(types::I32));
            self.import_signature(s)
        };

        let libc_memcmp = self.import_function(ExtFuncData {
            name: ExternalName::LibCall(LibCall::Memcmp),
            signature,
            colocated: false,
        });

        let call = self.ins().call(libc_memcmp, &[left, right, size]);
        self.func.dfg.first_result(call)
    }

    /// Optimised [`Self::call_memcmp`] for small copies.
    ///
    /// This implements the byte slice comparison `int_cc(left[..size], right[..size])`.
    ///
    /// `left_align` and `right_align` are the statically-known alignments of the
    /// `left` and `right` pointers respectively.  These are used to know whether
    /// to mark `load`s as aligned.  It's always fine to pass `1` for these, but
    /// passing something higher than the true alignment may trap or otherwise
    /// misbehave as described in [`MemFlags::aligned`].
    ///
    /// Note that `memcmp` is a *big-endian* and *unsigned* comparison.
    /// As such, this panics when called with `IntCC::Signed*`.
    pub fn emit_small_memory_compare(
        &mut self,
        config: TargetFrontendConfig,
        int_cc: IntCC,
        left: Value,
        right: Value,
        size: u64,
        left_align: std::num::NonZeroU8,
        right_align: std::num::NonZeroU8,
        flags: MemFlags,
    ) -> Value {
        use IntCC::*;
        let (zero_cc, empty_imm) = match int_cc {
            //
            Equal => (Equal, 1),
            NotEqual => (NotEqual, 0),

            UnsignedLessThan => (SignedLessThan, 0),
            UnsignedGreaterThanOrEqual => (SignedGreaterThanOrEqual, 1),
            UnsignedGreaterThan => (SignedGreaterThan, 0),
            UnsignedLessThanOrEqual => (SignedLessThanOrEqual, 1),

            SignedLessThan
            | SignedGreaterThanOrEqual
            | SignedGreaterThan
            | SignedLessThanOrEqual => {
                panic!("Signed comparison {int_cc} not supported by memcmp")
            }
        };

        if size == 0 {
            return self.ins().iconst(types::I8, empty_imm);
        }

        // Future work could consider expanding this to handle more-complex scenarios.
        if let Some(small_type) = size.try_into().ok().and_then(Type::int_with_byte_size) {
            if let Equal | NotEqual = zero_cc {
                let mut left_flags = flags;
                if size == left_align.get() as u64 {
                    left_flags.set_aligned();
                }
                let mut right_flags = flags;
                if size == right_align.get() as u64 {
                    right_flags.set_aligned();
                }
                let left_val = self.ins().load(small_type, left_flags, left, 0);
                let right_val = self.ins().load(small_type, right_flags, right, 0);
                return self.ins().icmp(int_cc, left_val, right_val);
            } else if small_type == types::I8 {
                // Once the big-endian loads from wasmtime#2492 are implemented in
                // the backends, we could easily handle comparisons for more sizes here.
                // But for now, just handle single bytes where we don't need to worry.

                let mut aligned_flags = flags;
                aligned_flags.set_aligned();
                let left_val = self.ins().load(small_type, aligned_flags, left, 0);
                let right_val = self.ins().load(small_type, aligned_flags, right, 0);
                return self.ins().icmp(int_cc, left_val, right_val);
            }
        }

        let pointer_type = config.pointer_type();
        let size = self.ins().iconst(pointer_type, size as i64);
        let cmp = self.call_memcmp(config, left, right, size);
        self.ins().icmp_imm(zero_cc, cmp, 0)
    }
}

fn greatest_divisible_power_of_two(size: u64) -> u64 {
    (size as i64 & -(size as i64)) as u64
}

// Helper functions
impl<'a> FunctionBuilder<'a> {
    /// A Block is 'filled' when a terminator instruction is present.
    fn fill_current_block(&mut self) {
        self.func_ctx.status[self.position.unwrap()] = BlockStatus::Filled;
    }

    fn declare_successor(&mut self, dest_block: Block, jump_inst: Inst) {
        self.func_ctx
            .ssa
            .declare_block_predecessor(dest_block, jump_inst);
    }

    fn handle_ssa_side_effects(&mut self, side_effects: SideEffects) {
        let SideEffects {
            instructions_added_to_blocks,
        } = side_effects;

        for modified_block in instructions_added_to_blocks {
            if self.is_pristine(modified_block) {
                self.func_ctx.status[modified_block] = BlockStatus::Partial;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::greatest_divisible_power_of_two;
    use crate::Variable;
    use crate::frontend::{
        DefVariableError, FunctionBuilder, FunctionBuilderContext, UseVariableError,
    };
    use alloc::string::ToString;
    use cranelift_codegen::ir::condcodes::IntCC;
    use cranelift_codegen::ir::{
        AbiParam, BlockCall, ExceptionTableData, ExtFuncData, ExternalName, Function, InstBuilder,
        MemFlags, Signature, UserExternalName, UserFuncName, Value, types::*,
    };
    use cranelift_codegen::isa::{CallConv, TargetFrontendConfig, TargetIsa};
    use cranelift_codegen::settings;
    use cranelift_codegen::verifier::verify_function;
    use target_lexicon::PointerWidth;

    fn sample_function(lazy_seal: bool) {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.returns.push(AbiParam::new(I32));
        sig.params.push(AbiParam::new(I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let block1 = builder.create_block();
            let block2 = builder.create_block();
            let block3 = builder.create_block();
            let x = builder.declare_var(I32);
            let y = builder.declare_var(I32);
            let z = builder.declare_var(I32);

            builder.append_block_params_for_function_params(block0);

            builder.switch_to_block(block0);
            if !lazy_seal {
                builder.seal_block(block0);
            }
            {
                let tmp = builder.block_params(block0)[0]; // the first function parameter
                builder.def_var(x, tmp);
            }
            {
                let tmp = builder.ins().iconst(I32, 2);
                builder.def_var(y, tmp);
            }
            {
                let arg1 = builder.use_var(x);
                let arg2 = builder.use_var(y);
                let tmp = builder.ins().iadd(arg1, arg2);
                builder.def_var(z, tmp);
            }
            builder.ins().jump(block1, &[]);

            builder.switch_to_block(block1);
            {
                let arg1 = builder.use_var(y);
                let arg2 = builder.use_var(z);
                let tmp = builder.ins().iadd(arg1, arg2);
                builder.def_var(z, tmp);
            }
            {
                let arg = builder.use_var(y);
                builder.ins().brif(arg, block3, &[], block2, &[]);
            }

            builder.switch_to_block(block2);
            if !lazy_seal {
                builder.seal_block(block2);
            }
            {
                let arg1 = builder.use_var(z);
                let arg2 = builder.use_var(x);
                let tmp = builder.ins().isub(arg1, arg2);
                builder.def_var(z, tmp);
            }
            {
                let arg = builder.use_var(y);
                builder.ins().return_(&[arg]);
            }

            builder.switch_to_block(block3);
            if !lazy_seal {
                builder.seal_block(block3);
            }

            {
                let arg1 = builder.use_var(y);
                let arg2 = builder.use_var(x);
                let tmp = builder.ins().isub(arg1, arg2);
                builder.def_var(y, tmp);
            }
            builder.ins().jump(block1, &[]);
            if !lazy_seal {
                builder.seal_block(block1);
            }

            if lazy_seal {
                builder.seal_all_blocks();
            }

            builder.finalize();
        }

        let flags = settings::Flags::new(settings::builder());
        // println!("{}", func.display(None));
        if let Err(errors) = verify_function(&func, &flags) {
            panic!("{}\n{}", func.display(), errors)
        }
    }

    #[test]
    fn sample() {
        sample_function(false)
    }

    #[test]
    fn sample_with_lazy_seal() {
        sample_function(true)
    }

    #[track_caller]
    fn check(func: &Function, expected_ir: &str) {
        let expected_ir = expected_ir.trim();
        let actual_ir = func.display().to_string();
        let actual_ir = actual_ir.trim();
        assert!(
            expected_ir == actual_ir,
            "Expected:\n{expected_ir}\nGot:\n{actual_ir}"
        );
    }

    /// Helper function to construct a fixed frontend configuration.
    fn systemv_frontend_config() -> TargetFrontendConfig {
        TargetFrontendConfig {
            default_call_conv: CallConv::SystemV,
            pointer_width: PointerWidth::U64,
            page_size_align_log2: 12,
        }
    }

    #[test]
    fn memcpy() {
        let frontend_config = systemv_frontend_config();
        let mut sig = Signature::new(frontend_config.default_call_conv);
        sig.returns.push(AbiParam::new(I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let x = builder.declare_var(frontend_config.pointer_type());
            let y = builder.declare_var(frontend_config.pointer_type());
            let _z = builder.declare_var(I32);

            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            let src = builder.use_var(x);
            let dest = builder.use_var(y);
            let size = builder.use_var(y);
            builder.call_memcpy(frontend_config, dest, src, size);
            builder.ins().return_(&[size]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            "function %sample() -> i32 system_v {
    sig0 = (i64, i64, i64) -> i64 system_v
    fn0 = %Memcpy sig0

block0:
    v4 = iconst.i64 0
    v1 -> v4
    v3 = iconst.i64 0
    v0 -> v3
    v2 = call fn0(v1, v0, v1)  ; v1 = 0, v0 = 0, v1 = 0
    return v1  ; v1 = 0
}
",
        );
    }

    #[test]
    fn small_memcpy() {
        let frontend_config = systemv_frontend_config();
        let mut sig = Signature::new(frontend_config.default_call_conv);
        sig.returns.push(AbiParam::new(I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let x = builder.declare_var(frontend_config.pointer_type());
            let y = builder.declare_var(frontend_config.pointer_type());

            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            let src = builder.use_var(x);
            let dest = builder.use_var(y);
            let size = 8;
            builder.emit_small_memory_copy(
                frontend_config,
                dest,
                src,
                size,
                8,
                8,
                true,
                MemFlags::new(),
            );
            builder.ins().return_(&[dest]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            "function %sample() -> i32 system_v {
block0:
    v4 = iconst.i64 0
    v1 -> v4
    v3 = iconst.i64 0
    v0 -> v3
    v2 = load.i64 aligned v0  ; v0 = 0
    store aligned v2, v1  ; v1 = 0
    return v1  ; v1 = 0
}
",
        );
    }

    #[test]
    fn not_so_small_memcpy() {
        let frontend_config = systemv_frontend_config();
        let mut sig = Signature::new(frontend_config.default_call_conv);
        sig.returns.push(AbiParam::new(I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let x = builder.declare_var(frontend_config.pointer_type());
            let y = builder.declare_var(frontend_config.pointer_type());
            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            let src = builder.use_var(x);
            let dest = builder.use_var(y);
            let size = 8192;
            builder.emit_small_memory_copy(
                frontend_config,
                dest,
                src,
                size,
                8,
                8,
                true,
                MemFlags::new(),
            );
            builder.ins().return_(&[dest]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            "function %sample() -> i32 system_v {
    sig0 = (i64, i64, i64) -> i64 system_v
    fn0 = %Memcpy sig0

block0:
    v5 = iconst.i64 0
    v1 -> v5
    v4 = iconst.i64 0
    v0 -> v4
    v2 = iconst.i64 8192
    v3 = call fn0(v1, v0, v2)  ; v1 = 0, v0 = 0, v2 = 8192
    return v1  ; v1 = 0
}
",
        );
    }

    #[test]
    fn small_memset() {
        let frontend_config = systemv_frontend_config();
        let mut sig = Signature::new(frontend_config.default_call_conv);
        sig.returns.push(AbiParam::new(I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let y = builder.declare_var(frontend_config.pointer_type());
            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            let dest = builder.use_var(y);
            let size = 8;
            builder.emit_small_memset(frontend_config, dest, 1, size, 8, MemFlags::new());
            builder.ins().return_(&[dest]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            "function %sample() -> i32 system_v {
block0:
    v2 = iconst.i64 0
    v0 -> v2
    v1 = iconst.i64 0x0101_0101_0101_0101
    store aligned v1, v0  ; v1 = 0x0101_0101_0101_0101, v0 = 0
    return v0  ; v0 = 0
}
",
        );
    }

    #[test]
    fn not_so_small_memset() {
        let frontend_config = systemv_frontend_config();
        let mut sig = Signature::new(frontend_config.default_call_conv);
        sig.returns.push(AbiParam::new(I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let y = builder.declare_var(frontend_config.pointer_type());
            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            let dest = builder.use_var(y);
            let size = 8192;
            builder.emit_small_memset(frontend_config, dest, 1, size, 8, MemFlags::new());
            builder.ins().return_(&[dest]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            "function %sample() -> i32 system_v {
    sig0 = (i64, i32, i64) -> i64 system_v
    fn0 = %Memset sig0

block0:
    v5 = iconst.i64 0
    v0 -> v5
    v1 = iconst.i8 1
    v2 = iconst.i64 8192
    v3 = uextend.i32 v1  ; v1 = 1
    v4 = call fn0(v0, v3, v2)  ; v0 = 0, v2 = 8192
    return v0  ; v0 = 0
}
",
        );
    }

    #[test]
    fn memcmp() {
        use core::str::FromStr;
        use cranelift_codegen::isa;

        let shared_builder = settings::builder();
        let shared_flags = settings::Flags::new(shared_builder);

        let triple =
            ::target_lexicon::Triple::from_str("x86_64").expect("Couldn't create x86_64 triple");

        let target = isa::lookup(triple)
            .ok()
            .map(|b| b.finish(shared_flags))
            .expect("This test requires x86_64 support.")
            .expect("Should be able to create backend with default flags");

        let mut sig = Signature::new(target.default_call_conv());
        sig.returns.push(AbiParam::new(I32));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let x = builder.declare_var(target.pointer_type());
            let y = builder.declare_var(target.pointer_type());
            let z = builder.declare_var(target.pointer_type());
            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            let left = builder.use_var(x);
            let right = builder.use_var(y);
            let size = builder.use_var(z);
            let cmp = builder.call_memcmp(target.frontend_config(), left, right, size);
            builder.ins().return_(&[cmp]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            "function %sample() -> i32 system_v {
    sig0 = (i64, i64, i64) -> i32 system_v
    fn0 = %Memcmp sig0

block0:
    v6 = iconst.i64 0
    v2 -> v6
    v5 = iconst.i64 0
    v1 -> v5
    v4 = iconst.i64 0
    v0 -> v4
    v3 = call fn0(v0, v1, v2)  ; v0 = 0, v1 = 0, v2 = 0
    return v3
}
",
        );
    }

    #[test]
    fn small_memcmp_zero_size() {
        let align_eight = std::num::NonZeroU8::new(8).unwrap();
        small_memcmp_helper(
            "
block0:
    v4 = iconst.i64 0
    v1 -> v4
    v3 = iconst.i64 0
    v0 -> v3
    v2 = iconst.i8 1
    return v2  ; v2 = 1",
            |builder, target, x, y| {
                builder.emit_small_memory_compare(
                    target.frontend_config(),
                    IntCC::UnsignedGreaterThanOrEqual,
                    x,
                    y,
                    0,
                    align_eight,
                    align_eight,
                    MemFlags::new(),
                )
            },
        );
    }

    #[test]
    fn small_memcmp_byte_ugt() {
        let align_one = std::num::NonZeroU8::new(1).unwrap();
        small_memcmp_helper(
            "
block0:
    v6 = iconst.i64 0
    v1 -> v6
    v5 = iconst.i64 0
    v0 -> v5
    v2 = load.i8 aligned v0  ; v0 = 0
    v3 = load.i8 aligned v1  ; v1 = 0
    v4 = icmp ugt v2, v3
    return v4",
            |builder, target, x, y| {
                builder.emit_small_memory_compare(
                    target.frontend_config(),
                    IntCC::UnsignedGreaterThan,
                    x,
                    y,
                    1,
                    align_one,
                    align_one,
                    MemFlags::new(),
                )
            },
        );
    }

    #[test]
    fn small_memcmp_aligned_eq() {
        let align_four = std::num::NonZeroU8::new(4).unwrap();
        small_memcmp_helper(
            "
block0:
    v6 = iconst.i64 0
    v1 -> v6
    v5 = iconst.i64 0
    v0 -> v5
    v2 = load.i32 aligned v0  ; v0 = 0
    v3 = load.i32 aligned v1  ; v1 = 0
    v4 = icmp eq v2, v3
    return v4",
            |builder, target, x, y| {
                builder.emit_small_memory_compare(
                    target.frontend_config(),
                    IntCC::Equal,
                    x,
                    y,
                    4,
                    align_four,
                    align_four,
                    MemFlags::new(),
                )
            },
        );
    }

    #[test]
    fn small_memcmp_ipv6_ne() {
        let align_two = std::num::NonZeroU8::new(2).unwrap();
        small_memcmp_helper(
            "
block0:
    v6 = iconst.i64 0
    v1 -> v6
    v5 = iconst.i64 0
    v0 -> v5
    v2 = load.i128 v0  ; v0 = 0
    v3 = load.i128 v1  ; v1 = 0
    v4 = icmp ne v2, v3
    return v4",
            |builder, target, x, y| {
                builder.emit_small_memory_compare(
                    target.frontend_config(),
                    IntCC::NotEqual,
                    x,
                    y,
                    16,
                    align_two,
                    align_two,
                    MemFlags::new(),
                )
            },
        );
    }

    #[test]
    fn small_memcmp_odd_size_uge() {
        let one = std::num::NonZeroU8::new(1).unwrap();
        small_memcmp_helper(
            "
    sig0 = (i64, i64, i64) -> i32 system_v
    fn0 = %Memcmp sig0

block0:
    v6 = iconst.i64 0
    v1 -> v6
    v5 = iconst.i64 0
    v0 -> v5
    v2 = iconst.i64 3
    v3 = call fn0(v0, v1, v2)  ; v0 = 0, v1 = 0, v2 = 3
    v4 = icmp_imm sge v3, 0
    return v4",
            |builder, target, x, y| {
                builder.emit_small_memory_compare(
                    target.frontend_config(),
                    IntCC::UnsignedGreaterThanOrEqual,
                    x,
                    y,
                    3,
                    one,
                    one,
                    MemFlags::new(),
                )
            },
        );
    }

    fn small_memcmp_helper(
        expected: &str,
        f: impl FnOnce(&mut FunctionBuilder, &dyn TargetIsa, Value, Value) -> Value,
    ) {
        use core::str::FromStr;
        use cranelift_codegen::isa;

        let shared_builder = settings::builder();
        let shared_flags = settings::Flags::new(shared_builder);

        let triple =
            ::target_lexicon::Triple::from_str("x86_64").expect("Couldn't create x86_64 triple");

        let target = isa::lookup(triple)
            .ok()
            .map(|b| b.finish(shared_flags))
            .expect("This test requires x86_64 support.")
            .expect("Should be able to create backend with default flags");

        let mut sig = Signature::new(target.default_call_conv());
        sig.returns.push(AbiParam::new(I8));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let x = builder.declare_var(target.pointer_type());
            let y = builder.declare_var(target.pointer_type());
            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            let left = builder.use_var(x);
            let right = builder.use_var(y);
            let ret = f(&mut builder, &*target, left, right);
            builder.ins().return_(&[ret]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            &format!("function %sample() -> i8 system_v {{{expected}\n}}\n"),
        );
    }

    #[test]
    fn undef_vector_vars() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.returns.push(AbiParam::new(I8X16));
        sig.returns.push(AbiParam::new(I8X16));
        sig.returns.push(AbiParam::new(F32X4));

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            let a = builder.declare_var(I8X16);
            let b = builder.declare_var(I8X16);
            let c = builder.declare_var(F32X4);
            builder.switch_to_block(block0);

            let a = builder.use_var(a);
            let b = builder.use_var(b);
            let c = builder.use_var(c);
            builder.ins().return_(&[a, b, c]);

            builder.seal_all_blocks();
            builder.finalize();
        }

        check(
            &func,
            "function %sample() -> i8x16, i8x16, f32x4 system_v {
    const0 = 0x00000000000000000000000000000000

block0:
    v5 = f32const 0.0
    v6 = splat.f32x4 v5  ; v5 = 0.0
    v2 -> v6
    v4 = vconst.i8x16 const0
    v1 -> v4
    v3 = vconst.i8x16 const0
    v0 -> v3
    return v0, v1, v2  ; v0 = const0, v1 = const0
}
",
        );
    }

    #[test]
    fn test_greatest_divisible_power_of_two() {
        assert_eq!(64, greatest_divisible_power_of_two(64));
        assert_eq!(16, greatest_divisible_power_of_two(48));
        assert_eq!(8, greatest_divisible_power_of_two(24));
        assert_eq!(1, greatest_divisible_power_of_two(25));
    }

    #[test]
    fn try_use_var() {
        let sig = Signature::new(CallConv::SystemV);

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);
        {
            let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

            let block0 = builder.create_block();
            builder.append_block_params_for_function_params(block0);
            builder.switch_to_block(block0);

            assert_eq!(
                builder.try_use_var(Variable::from_u32(0)),
                Err(UseVariableError::UsedBeforeDeclared(Variable::from_u32(0)))
            );

            let value = builder.ins().iconst(cranelift_codegen::ir::types::I32, 0);

            assert_eq!(
                builder.try_def_var(Variable::from_u32(0), value),
                Err(DefVariableError::DefinedBeforeDeclared(Variable::from_u32(
                    0
                )))
            );
        }
    }

    #[test]
    fn test_builder_with_iconst_and_negative_constant() {
        let sig = Signature::new(CallConv::SystemV);
        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);

        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let block0 = builder.create_block();
        builder.switch_to_block(block0);
        builder.ins().iconst(I32, -1);
        builder.ins().return_(&[]);

        builder.seal_all_blocks();
        builder.finalize();

        let flags = cranelift_codegen::settings::Flags::new(cranelift_codegen::settings::builder());
        let ctx = cranelift_codegen::Context::for_function(func);
        ctx.verify(&flags).expect("should be valid");

        check(
            &ctx.func,
            "function %sample() system_v {
block0:
    v0 = iconst.i32 -1
    return
}",
        );
    }

    #[test]
    fn try_call() {
        let mut sig = Signature::new(CallConv::SystemV);
        sig.params.push(AbiParam::new(I8));
        sig.returns.push(AbiParam::new(I32));
        let mut fn_ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(UserFuncName::testcase("sample"), sig);

        let sig0 = func.import_signature(Signature::new(CallConv::SystemV));
        let name = func.declare_imported_user_function(UserExternalName::new(0, 0));
        let fn0 = func.import_function(ExtFuncData {
            name: ExternalName::User(name),
            signature: sig0,
            colocated: false,
        });

        let mut builder = FunctionBuilder::new(&mut func, &mut fn_ctx);

        let block0 = builder.create_block();
        let block1 = builder.create_block();
        let block2 = builder.create_block();
        let block3 = builder.create_block();

        let my_var = builder.declare_var(I32);

        builder.switch_to_block(block0);
        let branch_val = builder.append_block_param(block0, I8);
        builder.ins().brif(branch_val, block1, &[], block2, &[]);

        builder.switch_to_block(block1);
        let one = builder.ins().iconst(I32, 1);
        builder.def_var(my_var, one);

        let normal_return = BlockCall::new(block3, [], &mut builder.func.dfg.value_lists);
        let exception_table = builder
            .func
            .dfg
            .exception_tables
            .push(ExceptionTableData::new(sig0, normal_return, []));
        builder.ins().try_call(fn0, &[], exception_table);

        builder.switch_to_block(block2);
        let two = builder.ins().iconst(I32, 2);
        builder.def_var(my_var, two);

        let normal_return = BlockCall::new(block3, [], &mut builder.func.dfg.value_lists);
        let exception_table = builder
            .func
            .dfg
            .exception_tables
            .push(ExceptionTableData::new(sig0, normal_return, []));
        builder.ins().try_call(fn0, &[], exception_table);

        builder.switch_to_block(block3);
        let ret_val = builder.use_var(my_var);
        builder.ins().return_(&[ret_val]);

        builder.seal_all_blocks();
        builder.finalize();

        let flags = cranelift_codegen::settings::Flags::new(cranelift_codegen::settings::builder());
        let ctx = cranelift_codegen::Context::for_function(func);
        ctx.verify(&flags).expect("should be valid");

        check(
            &ctx.func,
            "function %sample(i8) -> i32 system_v {
    sig0 = () system_v
    fn0 = u0:0 sig0

block0(v0: i8):
    brif v0, block1, block2

block1:
    v1 = iconst.i32 1
    try_call fn0(), sig0, block3(v1), []  ; v1 = 1

block2:
    v2 = iconst.i32 2
    try_call fn0(), sig0, block3(v2), []  ; v2 = 2

block3(v3: i32):
    return v3
}",
        );
    }
}
