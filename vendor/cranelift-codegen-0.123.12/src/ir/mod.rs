//! Representation of Cranelift IR functions.

mod atomic_rmw_op;
mod builder;
pub mod condcodes;
pub mod constant;
pub mod dfg;
pub mod dynamic_type;
pub mod entities;
mod exception_table;
mod extfunc;
mod extname;
pub mod function;
mod globalvalue;
pub mod immediates;
pub mod instructions;
pub mod jumptable;
pub(crate) mod known_symbol;
pub mod layout;
pub(crate) mod libcall;
mod memflags;
mod memtype;
pub mod pcc;
mod progpoint;
mod sourceloc;
pub mod stackslot;
mod trapcode;
pub mod types;
mod user_stack_maps;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

pub use crate::ir::atomic_rmw_op::AtomicRmwOp;
pub use crate::ir::builder::{
    InsertBuilder, InstBuilder, InstBuilderBase, InstInserterBase, ReplaceBuilder,
};
pub use crate::ir::constant::{ConstantData, ConstantPool};
pub use crate::ir::dfg::{BlockData, DataFlowGraph, ValueDef};
pub use crate::ir::dynamic_type::{DynamicTypeData, DynamicTypes, dynamic_to_fixed};
pub use crate::ir::entities::{
    Block, Constant, DynamicStackSlot, DynamicType, ExceptionTable, ExceptionTag, FuncRef,
    GlobalValue, Immediate, Inst, JumpTable, MemoryType, SigRef, StackSlot, UserExternalNameRef,
    Value,
};
pub use crate::ir::exception_table::{ExceptionTableData, ExceptionTableItem};
pub use crate::ir::extfunc::{
    AbiParam, ArgumentExtension, ArgumentPurpose, ExtFuncData, Signature,
};
pub use crate::ir::extname::{ExternalName, UserExternalName, UserFuncName};
pub use crate::ir::function::Function;
pub use crate::ir::globalvalue::GlobalValueData;
pub use crate::ir::instructions::{
    BlockArg, BlockCall, InstructionData, Opcode, ValueList, ValueListPool, VariableArgs,
};
pub use crate::ir::jumptable::JumpTableData;
pub use crate::ir::known_symbol::KnownSymbol;
pub use crate::ir::layout::Layout;
pub use crate::ir::libcall::{LibCall, get_probestack_funcref};
pub use crate::ir::memflags::{AliasRegion, Endianness, MemFlags};
pub use crate::ir::memtype::{MemoryTypeData, MemoryTypeField};
pub use crate::ir::pcc::{BaseExpr, Expr, Fact, FactContext, PccError, PccResult};
pub use crate::ir::progpoint::ProgramPoint;
pub use crate::ir::sourceloc::RelSourceLoc;
pub use crate::ir::sourceloc::SourceLoc;
pub use crate::ir::stackslot::{
    DynamicStackSlotData, DynamicStackSlots, StackSlotData, StackSlotKind, StackSlots,
};
pub use crate::ir::trapcode::TrapCode;
pub use crate::ir::types::Type;
pub(crate) use crate::ir::user_stack_maps::UserStackMapEntryVec;
pub use crate::ir::user_stack_maps::{UserStackMap, UserStackMapEntry};

use crate::entity::{PrimaryMap, SecondaryMap, entity_impl};

/// Map of jump tables.
pub type JumpTables = PrimaryMap<JumpTable, JumpTableData>;

/// Map of exception tables.
pub type ExceptionTables = PrimaryMap<ExceptionTable, ExceptionTableData>;

/// Source locations for instructions.
pub(crate) type SourceLocs = SecondaryMap<Inst, RelSourceLoc>;

/// Marked with a label value.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ValueLabel(u32);
entity_impl!(ValueLabel, "VL");

/// A label of a Value.
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub struct ValueLabelStart {
    /// Source location when it is in effect
    pub from: RelSourceLoc,

    /// The label index.
    pub label: ValueLabel,
}

/// Value label assignments: label starts or value aliases.
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
pub enum ValueLabelAssignments {
    /// Original value labels assigned at transform.
    Starts(alloc::vec::Vec<ValueLabelStart>),

    /// A value alias to original value.
    Alias {
        /// Source location when it is in effect
        from: RelSourceLoc,

        /// The label index.
        value: Value,
    },
}
