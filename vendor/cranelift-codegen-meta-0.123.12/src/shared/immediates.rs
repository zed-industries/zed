use crate::cdsl::operands::{EnumValues, OperandKind, OperandKindFields};

use std::collections::HashMap;

pub(crate) struct Immediates {
    /// A 64-bit immediate integer operand.
    ///
    /// This type of immediate integer can interact with SSA values with any IntType type.
    pub imm64: OperandKind,

    /// An unsigned 8-bit immediate integer operand.
    ///
    /// This small operand is used to indicate lane indexes in SIMD vectors and immediate bit
    /// counts on shift instructions.
    pub uimm8: OperandKind,

    /// A 32-bit immediate signed offset.
    ///
    /// This is used to represent an immediate address offset in load/store instructions.
    pub offset32: OperandKind,

    /// A 16-bit immediate floating point operand.
    ///
    /// IEEE 754-2008 binary16 interchange format.
    pub ieee16: OperandKind,

    /// A 32-bit immediate floating point operand.
    ///
    /// IEEE 754-2008 binary32 interchange format.
    pub ieee32: OperandKind,

    /// A 64-bit immediate floating point operand.
    ///
    /// IEEE 754-2008 binary64 interchange format.
    pub ieee64: OperandKind,

    /// A condition code for comparing integer values.
    ///
    /// This enumerated operand kind is used for the `icmp` instruction and corresponds to the
    /// condcodes::IntCC` Rust type.
    pub intcc: OperandKind,

    /// A condition code for comparing floating point values.
    ///
    /// This enumerated operand kind is used for the `fcmp` instruction and corresponds to the
    /// `condcodes::FloatCC` Rust type.
    pub floatcc: OperandKind,

    /// Flags for memory operations like `load` and `store`.
    pub memflags: OperandKind,

    /// A trap code indicating the reason for trapping.
    ///
    /// The Rust enum type also has a `User(u16)` variant for user-provided trap codes.
    pub trapcode: OperandKind,

    /// A code indicating the arithmetic operation to perform in an atomic_rmw memory access.
    pub atomic_rmw_op: OperandKind,
}

fn new_imm(
    format_field_name: &'static str,
    rust_type: &'static str,
    doc: &'static str,
) -> OperandKind {
    OperandKind::new(
        format_field_name,
        rust_type,
        OperandKindFields::ImmValue,
        doc,
    )
}
fn new_enum(
    format_field_name: &'static str,
    rust_type: &'static str,
    values: EnumValues,
    doc: &'static str,
) -> OperandKind {
    OperandKind::new(
        format_field_name,
        rust_type,
        OperandKindFields::ImmEnum(values),
        doc,
    )
}

impl Immediates {
    pub fn new() -> Self {
        Self {
            imm64: new_imm(
                "imm",
                "ir::immediates::Imm64",
                "A 64-bit immediate integer.",
            ),
            uimm8: new_imm(
                "imm",
                "ir::immediates::Uimm8",
                "An 8-bit immediate unsigned integer.",
            ),
            offset32: new_imm(
                "offset",
                "ir::immediates::Offset32",
                "A 32-bit immediate signed offset.",
            ),
            ieee16: new_imm(
                "imm",
                "ir::immediates::Ieee16",
                "A 16-bit immediate floating point number.",
            ),
            ieee32: new_imm(
                "imm",
                "ir::immediates::Ieee32",
                "A 32-bit immediate floating point number.",
            ),
            ieee64: new_imm(
                "imm",
                "ir::immediates::Ieee64",
                "A 64-bit immediate floating point number.",
            ),
            intcc: {
                let mut intcc_values = HashMap::new();
                intcc_values.insert("eq", "Equal");
                intcc_values.insert("ne", "NotEqual");
                intcc_values.insert("sge", "SignedGreaterThanOrEqual");
                intcc_values.insert("sgt", "SignedGreaterThan");
                intcc_values.insert("sle", "SignedLessThanOrEqual");
                intcc_values.insert("slt", "SignedLessThan");
                intcc_values.insert("uge", "UnsignedGreaterThanOrEqual");
                intcc_values.insert("ugt", "UnsignedGreaterThan");
                intcc_values.insert("ule", "UnsignedLessThanOrEqual");
                intcc_values.insert("ult", "UnsignedLessThan");
                new_enum(
                    "cond",
                    "ir::condcodes::IntCC",
                    intcc_values,
                    "An integer comparison condition code.",
                )
            },

            floatcc: {
                let mut floatcc_values = HashMap::new();
                floatcc_values.insert("ord", "Ordered");
                floatcc_values.insert("uno", "Unordered");
                floatcc_values.insert("eq", "Equal");
                floatcc_values.insert("ne", "NotEqual");
                floatcc_values.insert("one", "OrderedNotEqual");
                floatcc_values.insert("ueq", "UnorderedOrEqual");
                floatcc_values.insert("lt", "LessThan");
                floatcc_values.insert("le", "LessThanOrEqual");
                floatcc_values.insert("gt", "GreaterThan");
                floatcc_values.insert("ge", "GreaterThanOrEqual");
                floatcc_values.insert("ult", "UnorderedOrLessThan");
                floatcc_values.insert("ule", "UnorderedOrLessThanOrEqual");
                floatcc_values.insert("ugt", "UnorderedOrGreaterThan");
                floatcc_values.insert("uge", "UnorderedOrGreaterThanOrEqual");
                new_enum(
                    "cond",
                    "ir::condcodes::FloatCC",
                    floatcc_values,
                    "A floating point comparison condition code",
                )
            },

            memflags: new_imm("flags", "ir::MemFlags", "Memory operation flags"),

            trapcode: {
                let mut trapcode_values = HashMap::new();
                trapcode_values.insert("stk_ovf", "STACK_OVERFLOW");
                trapcode_values.insert("heap_oob", "HEAP_OUT_OF_BOUNDS");
                trapcode_values.insert("int_ovf", "INTEGER_OVERFLOW");
                trapcode_values.insert("int_divz", "INTEGER_DIVISION_BY_ZERO");
                trapcode_values.insert("bad_toint", "BAD_CONVERSION_TO_INTEGER");
                new_enum(
                    "code",
                    "ir::TrapCode",
                    trapcode_values,
                    "A trap reason code.",
                )
            },
            atomic_rmw_op: {
                let mut atomic_rmw_op_values = HashMap::new();
                atomic_rmw_op_values.insert("add", "Add");
                atomic_rmw_op_values.insert("sub", "Sub");
                atomic_rmw_op_values.insert("and", "And");
                atomic_rmw_op_values.insert("nand", "Nand");
                atomic_rmw_op_values.insert("or", "Or");
                atomic_rmw_op_values.insert("xor", "Xor");
                atomic_rmw_op_values.insert("xchg", "Xchg");
                atomic_rmw_op_values.insert("umin", "Umin");
                atomic_rmw_op_values.insert("umax", "Umax");
                atomic_rmw_op_values.insert("smin", "Smin");
                atomic_rmw_op_values.insert("smax", "Smax");
                new_enum(
                    "op",
                    "ir::AtomicRmwOp",
                    atomic_rmw_op_values,
                    "Atomic Read-Modify-Write Ops",
                )
            },
        }
    }
}
