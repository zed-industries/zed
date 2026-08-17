use crate::cdsl::operands::{OperandKind, OperandKindFields};

/// Small helper to initialize an OperandBuilder with the right kind, for a given name and doc.
fn new(format_field_name: &'static str, rust_type: &'static str, doc: &'static str) -> OperandKind {
    OperandKind::new(
        format_field_name,
        rust_type,
        OperandKindFields::EntityRef,
        doc,
    )
}

pub(crate) struct EntityRefs {
    /// A reference to a basic block in the same function, with its arguments provided.
    /// This is primarily used in control flow instructions.
    pub(crate) block_call: OperandKind,

    /// A reference to a basic block in the same function, with its arguments provided.
    /// This is primarily used in control flow instructions.
    pub(crate) block_then: OperandKind,

    /// A reference to a basic block in the same function, with its arguments provided.
    /// This is primarily used in control flow instructions.
    pub(crate) block_else: OperandKind,

    /// A reference to a stack slot declared in the function preamble.
    pub(crate) stack_slot: OperandKind,

    /// A reference to a dynamic_stack slot declared in the function preamble.
    pub(crate) dynamic_stack_slot: OperandKind,

    /// A reference to a global value.
    pub(crate) global_value: OperandKind,

    /// A reference to a function signature declared in the function preamble.
    /// This is used to provide the call signature in a call_indirect instruction.
    pub(crate) sig_ref: OperandKind,

    /// A reference to an external function declared in the function preamble.
    /// This is used to provide the callee and signature in a call instruction.
    pub(crate) func_ref: OperandKind,

    /// A reference to a jump table declared in the function preamble.
    pub(crate) jump_table: OperandKind,

    /// A reference to an exception table declared in the function preamble.
    pub(crate) exception_table: OperandKind,

    /// A variable-sized list of value operands. Use for Block and function call arguments.
    pub(crate) varargs: OperandKind,

    /// A constant stored in the constant pool.
    ///
    /// This operand is used to pass constants to instructions like `vconst`
    /// while storing the actual bytes in the constant pool.
    pub(crate) pool_constant: OperandKind,

    /// An unsigned 128-bit immediate integer operand, stored out-of-line in the
    /// `DataFlowGraph::immediates` pool.
    ///
    /// This operand is used to pass entire 128-bit vectors as immediates to instructions like
    /// `shuffle` and `mask`.
    pub(crate) uimm128: OperandKind,
}

impl EntityRefs {
    pub fn new() -> Self {
        Self {
            block_call: new(
                "destination",
                "ir::BlockCall",
                "a basic block in the same function, with its arguments provided.",
            ),

            block_then: new(
                "block_then",
                "ir::BlockCall",
                "a basic block in the same function, with its arguments provided.",
            ),

            block_else: new(
                "block_else",
                "ir::BlockCall",
                "a basic block in the same function, with its arguments provided.",
            ),

            stack_slot: new("stack_slot", "ir::StackSlot", "A stack slot"),

            dynamic_stack_slot: new(
                "dynamic_stack_slot",
                "ir::DynamicStackSlot",
                "A dynamic stack slot",
            ),

            global_value: new("global_value", "ir::GlobalValue", "A global value."),

            sig_ref: new("sig_ref", "ir::SigRef", "A function signature."),

            func_ref: new("func_ref", "ir::FuncRef", "An external function."),

            jump_table: new("table", "ir::JumpTable", "A jump table."),

            exception_table: new("exception", "ir::ExceptionTable", "An exception table."),

            varargs: OperandKind::new(
                "",
                "&[Value]",
                OperandKindFields::VariableArgs,
                r#"
                        A variable size list of `value` operands.

                        Use this to represent arguments passed to a function call, arguments
                        passed to a basic block, or a variable number of results
                        returned from an instruction.
                    "#,
            ),

            pool_constant: new(
                "constant_handle",
                "ir::Constant",
                "A constant stored in the constant pool.",
            ),

            uimm128: new(
                "imm",
                "ir::Immediate",
                "A 128-bit immediate unsigned integer.",
            ),
        }
    }
}
