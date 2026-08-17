use crate::{encode_section, Encode, HeapType, RefType, Section, SectionId, ValType};
use std::borrow::Cow;

/// An encoder for the code section.
///
/// Code sections are only supported for modules.
///
/// # Example
///
/// ```
/// use wasm_encoder::{
///     CodeSection, Function, FunctionSection, Instruction, Module,
///     TypeSection, ValType
/// };
///
/// let mut types = TypeSection::new();
/// types.function(vec![], vec![ValType::I32]);
///
/// let mut functions = FunctionSection::new();
/// let type_index = 0;
/// functions.function(type_index);
///
/// let locals = vec![];
/// let mut func = Function::new(locals);
/// func.instruction(&Instruction::I32Const(42));
/// let mut code = CodeSection::new();
/// code.function(&func);
///
/// let mut module = Module::new();
/// module
///     .section(&types)
///     .section(&functions)
///     .section(&code);
///
/// let wasm_bytes = module.finish();
/// ```
#[derive(Clone, Default, Debug)]
pub struct CodeSection {
    bytes: Vec<u8>,
    num_added: u32,
}

impl CodeSection {
    /// Create a new code section encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of functions in the section.
    pub fn len(&self) -> u32 {
        self.num_added
    }

    /// The number of bytes already added to this section.
    ///
    /// This number doesn't include the vector length that precedes the
    /// code entries, since it has a variable size that isn't known until all
    /// functions are added.
    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    /// Determines if the section is empty.
    pub fn is_empty(&self) -> bool {
        self.num_added == 0
    }

    /// Write a function body into this code section.
    pub fn function(&mut self, func: &Function) -> &mut Self {
        func.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }

    /// Add a raw byte slice into this code section as a function body.
    ///
    /// The length prefix of the function body will be automatically prepended,
    /// and should not be included in the raw byte slice.
    ///
    /// # Example
    ///
    /// You can use the `raw` method to copy an already-encoded function body
    /// into a new code section encoder:
    ///
    /// ```
    /// //                  id, size, # entries, entry
    /// let code_section = [10, 6,    1,         4, 0, 65, 0, 11];
    ///
    /// // Parse the code section.
    /// let reader = wasmparser::CodeSectionReader::new(&code_section, 0).unwrap();
    /// let body = reader.into_iter().next().unwrap().unwrap();
    /// let body_range = body.range();
    ///
    /// // Add the body to a new code section encoder by copying bytes rather
    /// // than re-parsing and re-encoding it.
    /// let mut encoder = wasm_encoder::CodeSection::new();
    /// encoder.raw(&code_section[body_range.start..body_range.end]);
    /// ```
    pub fn raw(&mut self, data: &[u8]) -> &mut Self {
        data.encode(&mut self.bytes);
        self.num_added += 1;
        self
    }
}

impl Encode for CodeSection {
    fn encode(&self, sink: &mut Vec<u8>) {
        encode_section(sink, self.num_added, &self.bytes);
    }
}

impl Section for CodeSection {
    fn id(&self) -> u8 {
        SectionId::Code.into()
    }
}

/// An encoder for a function body within the code section.
///
/// # Example
///
/// ```
/// use wasm_encoder::{CodeSection, Function, Instruction};
///
/// // Define the function body for:
/// //
/// //     (func (param i32 i32) (result i32)
/// //       local.get 0
/// //       local.get 1
/// //       i32.add)
/// let locals = vec![];
/// let mut func = Function::new(locals);
/// func.instruction(&Instruction::LocalGet(0));
/// func.instruction(&Instruction::LocalGet(1));
/// func.instruction(&Instruction::I32Add);
///
/// // Add our function to the code section.
/// let mut code = CodeSection::new();
/// code.function(&func);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    bytes: Vec<u8>,
}

impl Function {
    /// Create a new function body with the given locals.
    ///
    /// The argument is an iterator over `(N, Ty)`, which defines
    /// that the next `N` locals will be of type `Ty`.
    ///
    /// For example, a function with locals 0 and 1 of type I32 and
    /// local 2 of type F32 would be created as:
    ///
    /// ```
    /// # use wasm_encoder::{Function, ValType};
    /// let f = Function::new([(2, ValType::I32), (1, ValType::F32)]);
    /// ```
    ///
    /// For more information about the code section (and function definition) in the WASM binary format
    /// see the [WebAssembly spec](https://webassembly.github.io/spec/core/binary/modules.html#binary-func)
    pub fn new<L>(locals: L) -> Self
    where
        L: IntoIterator<Item = (u32, ValType)>,
        L::IntoIter: ExactSizeIterator,
    {
        let locals = locals.into_iter();
        let mut bytes = vec![];
        locals.len().encode(&mut bytes);
        for (count, ty) in locals {
            count.encode(&mut bytes);
            ty.encode(&mut bytes);
        }
        Function { bytes }
    }

    /// Create a function from a list of locals' types.
    ///
    /// Unlike [`Function::new`], this constructor simply takes a list of types
    /// which are in order associated with locals.
    ///
    /// For example:
    ///
    ///  ```
    /// # use wasm_encoder::{Function, ValType};
    /// let f = Function::new([(2, ValType::I32), (1, ValType::F32)]);
    /// let g = Function::new_with_locals_types([
    ///     ValType::I32, ValType::I32, ValType::F32
    /// ]);
    ///
    /// assert_eq!(f, g)
    /// ```
    pub fn new_with_locals_types<L>(locals: L) -> Self
    where
        L: IntoIterator<Item = ValType>,
    {
        let locals = locals.into_iter();

        let mut locals_collected: Vec<(u32, ValType)> = vec![];
        for l in locals {
            if let Some((last_count, last_type)) = locals_collected.last_mut() {
                if l == *last_type {
                    // Increment the count of consecutive locals of this type
                    *last_count += 1;
                    continue;
                }
            }
            // If we didn't increment, a new type of local appeared
            locals_collected.push((1, l));
        }

        Function::new(locals_collected)
    }

    /// Write an instruction into this function body.
    pub fn instruction(&mut self, instruction: &Instruction) -> &mut Self {
        instruction.encode(&mut self.bytes);
        self
    }

    /// Add raw bytes to this function's body.
    pub fn raw<B>(&mut self, bytes: B) -> &mut Self
    where
        B: IntoIterator<Item = u8>,
    {
        self.bytes.extend(bytes);
        self
    }

    /// The number of bytes already added to this function.
    ///
    /// This number doesn't include the variable-width size field that `encode`
    /// will write before the added bytes, since the size of that field isn't
    /// known until all the instructions are added to this function.
    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }
}

impl Encode for Function {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.bytes.encode(sink);
    }
}

/// The immediate for a memory instruction.
#[derive(Clone, Copy, Debug)]
pub struct MemArg {
    /// A static offset to add to the instruction's dynamic address operand.
    ///
    /// This is a `u64` field for the memory64 proposal, but 32-bit memories
    /// limit offsets to at most `u32::MAX` bytes. This will be encoded as a LEB
    /// but it won't generate a valid module if an offset is specified which is
    /// larger than the maximum size of the index space for the memory indicated
    /// by `memory_index`.
    pub offset: u64,
    /// The expected alignment of the instruction's dynamic address operand
    /// (expressed the exponent of a power of two).
    pub align: u32,
    /// The index of the memory this instruction is operating upon.
    pub memory_index: u32,
}

impl Encode for MemArg {
    fn encode(&self, sink: &mut Vec<u8>) {
        if self.memory_index == 0 {
            self.align.encode(sink);
            self.offset.encode(sink);
        } else {
            (self.align | (1 << 6)).encode(sink);
            self.memory_index.encode(sink);
            self.offset.encode(sink);
        }
    }
}

/// Describe an unchecked SIMD lane index.
pub type Lane = u8;

/// The type for a `block`/`if`/`loop`.
#[derive(Clone, Copy, Debug)]
pub enum BlockType {
    /// `[] -> []`
    Empty,
    /// `[] -> [t]`
    Result(ValType),
    /// The `n`th function type.
    FunctionType(u32),
}

impl Encode for BlockType {
    fn encode(&self, sink: &mut Vec<u8>) {
        match *self {
            Self::Empty => sink.push(0x40),
            Self::Result(ty) => ty.encode(sink),
            Self::FunctionType(f) => (f as i64).encode(sink),
        }
    }
}

/// WebAssembly instructions.
#[derive(Clone, Debug)]
#[non_exhaustive]
#[allow(missing_docs, non_camel_case_types)]
pub enum Instruction<'a> {
    // Control instructions.
    Unreachable,
    Nop,
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,
    Br(u32),
    BrIf(u32),
    BrTable(Cow<'a, [u32]>, u32),
    BrOnNull(u32),
    BrOnNonNull(u32),
    Return,
    Call(u32),
    CallRef(u32),
    CallIndirect {
        ty: u32,
        table: u32,
    },
    ReturnCallRef(u32),
    ReturnCall(u32),
    ReturnCallIndirect {
        ty: u32,
        table: u32,
    },
    TryTable(BlockType, Cow<'a, [Catch]>),
    Throw(u32),
    ThrowRef,

    // Deprecated exception-handling instructions
    Try(BlockType),
    Delegate(u32),
    Catch(u32),
    CatchAll,
    Rethrow(u32),

    // Parametric instructions.
    Drop,
    Select,

    // Variable instructions.
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // Memory instructions.
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemorySize(u32),
    MemoryGrow(u32),
    MemoryInit {
        mem: u32,
        data_index: u32,
    },
    DataDrop(u32),
    MemoryCopy {
        src_mem: u32,
        dst_mem: u32,
    },
    MemoryFill(u32),
    MemoryDiscard(u32),

    // Numeric instructions.
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,

    // Reference types instructions.
    TypedSelect(ValType),
    RefNull(HeapType),
    RefIsNull,
    RefFunc(u32),
    RefEq,
    RefAsNonNull,

    // GC types instructions.
    StructNew(u32),
    StructNewDefault(u32),
    StructGet {
        struct_type_index: u32,
        field_index: u32,
    },
    StructGetS {
        struct_type_index: u32,
        field_index: u32,
    },
    StructGetU {
        struct_type_index: u32,
        field_index: u32,
    },
    StructSet {
        struct_type_index: u32,
        field_index: u32,
    },

    ArrayNew(u32),
    ArrayNewDefault(u32),
    ArrayNewFixed {
        array_type_index: u32,
        array_size: u32,
    },
    ArrayNewData {
        array_type_index: u32,
        array_data_index: u32,
    },
    ArrayNewElem {
        array_type_index: u32,
        array_elem_index: u32,
    },
    ArrayGet(u32),
    ArrayGetS(u32),
    ArrayGetU(u32),
    ArraySet(u32),
    ArrayLen,
    ArrayFill(u32),
    ArrayCopy {
        array_type_index_dst: u32,
        array_type_index_src: u32,
    },
    ArrayInitData {
        array_type_index: u32,
        array_data_index: u32,
    },
    ArrayInitElem {
        array_type_index: u32,
        array_elem_index: u32,
    },
    RefTestNonNull(HeapType),
    RefTestNullable(HeapType),
    RefCastNonNull(HeapType),
    RefCastNullable(HeapType),
    BrOnCast {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
    BrOnCastFail {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
    AnyConvertExtern,
    ExternConvertAny,

    RefI31,
    I31GetS,
    I31GetU,

    // Bulk memory instructions.
    TableInit {
        elem_index: u32,
        table: u32,
    },
    ElemDrop(u32),
    TableFill(u32),
    TableSet(u32),
    TableGet(u32),
    TableGrow(u32),
    TableSize(u32),
    TableCopy {
        src_table: u32,
        dst_table: u32,
    },

    // SIMD instructions.
    V128Load(MemArg),
    V128Load8x8S(MemArg),
    V128Load8x8U(MemArg),
    V128Load16x4S(MemArg),
    V128Load16x4U(MemArg),
    V128Load32x2S(MemArg),
    V128Load32x2U(MemArg),
    V128Load8Splat(MemArg),
    V128Load16Splat(MemArg),
    V128Load32Splat(MemArg),
    V128Load64Splat(MemArg),
    V128Load32Zero(MemArg),
    V128Load64Zero(MemArg),
    V128Store(MemArg),
    V128Load8Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Load16Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Load32Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Load64Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Store8Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Store16Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Store32Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Store64Lane {
        memarg: MemArg,
        lane: Lane,
    },
    V128Const(i128),
    I8x16Shuffle([Lane; 16]),
    I8x16ExtractLaneS(Lane),
    I8x16ExtractLaneU(Lane),
    I8x16ReplaceLane(Lane),
    I16x8ExtractLaneS(Lane),
    I16x8ExtractLaneU(Lane),
    I16x8ReplaceLane(Lane),
    I32x4ExtractLane(Lane),
    I32x4ReplaceLane(Lane),
    I64x2ExtractLane(Lane),
    I64x2ReplaceLane(Lane),
    F32x4ExtractLane(Lane),
    F32x4ReplaceLane(Lane),
    F64x2ExtractLane(Lane),
    F64x2ReplaceLane(Lane),
    I8x16Swizzle,
    I8x16Splat,
    I16x8Splat,
    I32x4Splat,
    I64x2Splat,
    F32x4Splat,
    F64x2Splat,
    I8x16Eq,
    I8x16Ne,
    I8x16LtS,
    I8x16LtU,
    I8x16GtS,
    I8x16GtU,
    I8x16LeS,
    I8x16LeU,
    I8x16GeS,
    I8x16GeU,
    I16x8Eq,
    I16x8Ne,
    I16x8LtS,
    I16x8LtU,
    I16x8GtS,
    I16x8GtU,
    I16x8LeS,
    I16x8LeU,
    I16x8GeS,
    I16x8GeU,
    I32x4Eq,
    I32x4Ne,
    I32x4LtS,
    I32x4LtU,
    I32x4GtS,
    I32x4GtU,
    I32x4LeS,
    I32x4LeU,
    I32x4GeS,
    I32x4GeU,
    I64x2Eq,
    I64x2Ne,
    I64x2LtS,
    I64x2GtS,
    I64x2LeS,
    I64x2GeS,
    F32x4Eq,
    F32x4Ne,
    F32x4Lt,
    F32x4Gt,
    F32x4Le,
    F32x4Ge,
    F64x2Eq,
    F64x2Ne,
    F64x2Lt,
    F64x2Gt,
    F64x2Le,
    F64x2Ge,
    V128Not,
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    V128Bitselect,
    V128AnyTrue,
    I8x16Abs,
    I8x16Neg,
    I8x16Popcnt,
    I8x16AllTrue,
    I8x16Bitmask,
    I8x16NarrowI16x8S,
    I8x16NarrowI16x8U,
    I8x16Shl,
    I8x16ShrS,
    I8x16ShrU,
    I8x16Add,
    I8x16AddSatS,
    I8x16AddSatU,
    I8x16Sub,
    I8x16SubSatS,
    I8x16SubSatU,
    I8x16MinS,
    I8x16MinU,
    I8x16MaxS,
    I8x16MaxU,
    I8x16AvgrU,
    I16x8ExtAddPairwiseI8x16S,
    I16x8ExtAddPairwiseI8x16U,
    I16x8Abs,
    I16x8Neg,
    I16x8Q15MulrSatS,
    I16x8AllTrue,
    I16x8Bitmask,
    I16x8NarrowI32x4S,
    I16x8NarrowI32x4U,
    I16x8ExtendLowI8x16S,
    I16x8ExtendHighI8x16S,
    I16x8ExtendLowI8x16U,
    I16x8ExtendHighI8x16U,
    I16x8Shl,
    I16x8ShrS,
    I16x8ShrU,
    I16x8Add,
    I16x8AddSatS,
    I16x8AddSatU,
    I16x8Sub,
    I16x8SubSatS,
    I16x8SubSatU,
    I16x8Mul,
    I16x8MinS,
    I16x8MinU,
    I16x8MaxS,
    I16x8MaxU,
    I16x8AvgrU,
    I16x8ExtMulLowI8x16S,
    I16x8ExtMulHighI8x16S,
    I16x8ExtMulLowI8x16U,
    I16x8ExtMulHighI8x16U,
    I32x4ExtAddPairwiseI16x8S,
    I32x4ExtAddPairwiseI16x8U,
    I32x4Abs,
    I32x4Neg,
    I32x4AllTrue,
    I32x4Bitmask,
    I32x4ExtendLowI16x8S,
    I32x4ExtendHighI16x8S,
    I32x4ExtendLowI16x8U,
    I32x4ExtendHighI16x8U,
    I32x4Shl,
    I32x4ShrS,
    I32x4ShrU,
    I32x4Add,
    I32x4Sub,
    I32x4Mul,
    I32x4MinS,
    I32x4MinU,
    I32x4MaxS,
    I32x4MaxU,
    I32x4DotI16x8S,
    I32x4ExtMulLowI16x8S,
    I32x4ExtMulHighI16x8S,
    I32x4ExtMulLowI16x8U,
    I32x4ExtMulHighI16x8U,
    I64x2Abs,
    I64x2Neg,
    I64x2AllTrue,
    I64x2Bitmask,
    I64x2ExtendLowI32x4S,
    I64x2ExtendHighI32x4S,
    I64x2ExtendLowI32x4U,
    I64x2ExtendHighI32x4U,
    I64x2Shl,
    I64x2ShrS,
    I64x2ShrU,
    I64x2Add,
    I64x2Sub,
    I64x2Mul,
    I64x2ExtMulLowI32x4S,
    I64x2ExtMulHighI32x4S,
    I64x2ExtMulLowI32x4U,
    I64x2ExtMulHighI32x4U,
    F32x4Ceil,
    F32x4Floor,
    F32x4Trunc,
    F32x4Nearest,
    F32x4Abs,
    F32x4Neg,
    F32x4Sqrt,
    F32x4Add,
    F32x4Sub,
    F32x4Mul,
    F32x4Div,
    F32x4Min,
    F32x4Max,
    F32x4PMin,
    F32x4PMax,
    F64x2Ceil,
    F64x2Floor,
    F64x2Trunc,
    F64x2Nearest,
    F64x2Abs,
    F64x2Neg,
    F64x2Sqrt,
    F64x2Add,
    F64x2Sub,
    F64x2Mul,
    F64x2Div,
    F64x2Min,
    F64x2Max,
    F64x2PMin,
    F64x2PMax,
    I32x4TruncSatF32x4S,
    I32x4TruncSatF32x4U,
    F32x4ConvertI32x4S,
    F32x4ConvertI32x4U,
    I32x4TruncSatF64x2SZero,
    I32x4TruncSatF64x2UZero,
    F64x2ConvertLowI32x4S,
    F64x2ConvertLowI32x4U,
    F32x4DemoteF64x2Zero,
    F64x2PromoteLowF32x4,

    // Relaxed simd proposal
    I8x16RelaxedSwizzle,
    I32x4RelaxedTruncF32x4S,
    I32x4RelaxedTruncF32x4U,
    I32x4RelaxedTruncF64x2SZero,
    I32x4RelaxedTruncF64x2UZero,
    F32x4RelaxedMadd,
    F32x4RelaxedNmadd,
    F64x2RelaxedMadd,
    F64x2RelaxedNmadd,
    I8x16RelaxedLaneselect,
    I16x8RelaxedLaneselect,
    I32x4RelaxedLaneselect,
    I64x2RelaxedLaneselect,
    F32x4RelaxedMin,
    F32x4RelaxedMax,
    F64x2RelaxedMin,
    F64x2RelaxedMax,
    I16x8RelaxedQ15mulrS,
    I16x8RelaxedDotI8x16I7x16S,
    I32x4RelaxedDotI8x16I7x16AddS,

    // Atomic instructions (the threads proposal)
    MemoryAtomicNotify(MemArg),
    MemoryAtomicWait32(MemArg),
    MemoryAtomicWait64(MemArg),
    AtomicFence,
    I32AtomicLoad(MemArg),
    I64AtomicLoad(MemArg),
    I32AtomicLoad8U(MemArg),
    I32AtomicLoad16U(MemArg),
    I64AtomicLoad8U(MemArg),
    I64AtomicLoad16U(MemArg),
    I64AtomicLoad32U(MemArg),
    I32AtomicStore(MemArg),
    I64AtomicStore(MemArg),
    I32AtomicStore8(MemArg),
    I32AtomicStore16(MemArg),
    I64AtomicStore8(MemArg),
    I64AtomicStore16(MemArg),
    I64AtomicStore32(MemArg),
    I32AtomicRmwAdd(MemArg),
    I64AtomicRmwAdd(MemArg),
    I32AtomicRmw8AddU(MemArg),
    I32AtomicRmw16AddU(MemArg),
    I64AtomicRmw8AddU(MemArg),
    I64AtomicRmw16AddU(MemArg),
    I64AtomicRmw32AddU(MemArg),
    I32AtomicRmwSub(MemArg),
    I64AtomicRmwSub(MemArg),
    I32AtomicRmw8SubU(MemArg),
    I32AtomicRmw16SubU(MemArg),
    I64AtomicRmw8SubU(MemArg),
    I64AtomicRmw16SubU(MemArg),
    I64AtomicRmw32SubU(MemArg),
    I32AtomicRmwAnd(MemArg),
    I64AtomicRmwAnd(MemArg),
    I32AtomicRmw8AndU(MemArg),
    I32AtomicRmw16AndU(MemArg),
    I64AtomicRmw8AndU(MemArg),
    I64AtomicRmw16AndU(MemArg),
    I64AtomicRmw32AndU(MemArg),
    I32AtomicRmwOr(MemArg),
    I64AtomicRmwOr(MemArg),
    I32AtomicRmw8OrU(MemArg),
    I32AtomicRmw16OrU(MemArg),
    I64AtomicRmw8OrU(MemArg),
    I64AtomicRmw16OrU(MemArg),
    I64AtomicRmw32OrU(MemArg),
    I32AtomicRmwXor(MemArg),
    I64AtomicRmwXor(MemArg),
    I32AtomicRmw8XorU(MemArg),
    I32AtomicRmw16XorU(MemArg),
    I64AtomicRmw8XorU(MemArg),
    I64AtomicRmw16XorU(MemArg),
    I64AtomicRmw32XorU(MemArg),
    I32AtomicRmwXchg(MemArg),
    I64AtomicRmwXchg(MemArg),
    I32AtomicRmw8XchgU(MemArg),
    I32AtomicRmw16XchgU(MemArg),
    I64AtomicRmw8XchgU(MemArg),
    I64AtomicRmw16XchgU(MemArg),
    I64AtomicRmw32XchgU(MemArg),
    I32AtomicRmwCmpxchg(MemArg),
    I64AtomicRmwCmpxchg(MemArg),
    I32AtomicRmw8CmpxchgU(MemArg),
    I32AtomicRmw16CmpxchgU(MemArg),
    I64AtomicRmw8CmpxchgU(MemArg),
    I64AtomicRmw16CmpxchgU(MemArg),
    I64AtomicRmw32CmpxchgU(MemArg),
}

impl Encode for Instruction<'_> {
    fn encode(&self, sink: &mut Vec<u8>) {
        match *self {
            // Control instructions.
            Instruction::Unreachable => sink.push(0x00),
            Instruction::Nop => sink.push(0x01),
            Instruction::Block(bt) => {
                sink.push(0x02);
                bt.encode(sink);
            }
            Instruction::Loop(bt) => {
                sink.push(0x03);
                bt.encode(sink);
            }
            Instruction::If(bt) => {
                sink.push(0x04);
                bt.encode(sink);
            }
            Instruction::Else => sink.push(0x05),
            Instruction::Try(bt) => {
                sink.push(0x06);
                bt.encode(sink);
            }
            Instruction::Catch(t) => {
                sink.push(0x07);
                t.encode(sink);
            }
            Instruction::Throw(t) => {
                sink.push(0x08);
                t.encode(sink);
            }
            Instruction::Rethrow(l) => {
                sink.push(0x09);
                l.encode(sink);
            }
            Instruction::ThrowRef => {
                sink.push(0x0A);
            }
            Instruction::End => sink.push(0x0B),
            Instruction::Br(l) => {
                sink.push(0x0C);
                l.encode(sink);
            }
            Instruction::BrIf(l) => {
                sink.push(0x0D);
                l.encode(sink);
            }
            Instruction::BrTable(ref ls, l) => {
                sink.push(0x0E);
                ls.encode(sink);
                l.encode(sink);
            }
            Instruction::BrOnNull(l) => {
                sink.push(0xD5);
                l.encode(sink);
            }
            Instruction::BrOnNonNull(l) => {
                sink.push(0xD6);
                l.encode(sink);
            }
            Instruction::Return => sink.push(0x0F),
            Instruction::Call(f) => {
                sink.push(0x10);
                f.encode(sink);
            }
            Instruction::CallRef(ty) => {
                sink.push(0x14);
                ty.encode(sink);
            }
            Instruction::CallIndirect { ty, table } => {
                sink.push(0x11);
                ty.encode(sink);
                table.encode(sink);
            }
            Instruction::ReturnCallRef(ty) => {
                sink.push(0x15);
                ty.encode(sink);
            }

            Instruction::ReturnCall(f) => {
                sink.push(0x12);
                f.encode(sink);
            }
            Instruction::ReturnCallIndirect { ty, table } => {
                sink.push(0x13);
                ty.encode(sink);
                table.encode(sink);
            }
            Instruction::Delegate(l) => {
                sink.push(0x18);
                l.encode(sink);
            }
            Instruction::CatchAll => {
                sink.push(0x19);
            }

            // Parametric instructions.
            Instruction::Drop => sink.push(0x1A),
            Instruction::Select => sink.push(0x1B),
            Instruction::TypedSelect(ty) => {
                sink.push(0x1c);
                [ty].encode(sink);
            }

            Instruction::TryTable(ty, ref catches) => {
                sink.push(0x1f);
                ty.encode(sink);
                catches.encode(sink);
            }

            // Variable instructions.
            Instruction::LocalGet(l) => {
                sink.push(0x20);
                l.encode(sink);
            }
            Instruction::LocalSet(l) => {
                sink.push(0x21);
                l.encode(sink);
            }
            Instruction::LocalTee(l) => {
                sink.push(0x22);
                l.encode(sink);
            }
            Instruction::GlobalGet(g) => {
                sink.push(0x23);
                g.encode(sink);
            }
            Instruction::GlobalSet(g) => {
                sink.push(0x24);
                g.encode(sink);
            }
            Instruction::TableGet(table) => {
                sink.push(0x25);
                table.encode(sink);
            }
            Instruction::TableSet(table) => {
                sink.push(0x26);
                table.encode(sink);
            }

            // Memory instructions.
            Instruction::I32Load(m) => {
                sink.push(0x28);
                m.encode(sink);
            }
            Instruction::I64Load(m) => {
                sink.push(0x29);
                m.encode(sink);
            }
            Instruction::F32Load(m) => {
                sink.push(0x2A);
                m.encode(sink);
            }
            Instruction::F64Load(m) => {
                sink.push(0x2B);
                m.encode(sink);
            }
            Instruction::I32Load8S(m) => {
                sink.push(0x2C);
                m.encode(sink);
            }
            Instruction::I32Load8U(m) => {
                sink.push(0x2D);
                m.encode(sink);
            }
            Instruction::I32Load16S(m) => {
                sink.push(0x2E);
                m.encode(sink);
            }
            Instruction::I32Load16U(m) => {
                sink.push(0x2F);
                m.encode(sink);
            }
            Instruction::I64Load8S(m) => {
                sink.push(0x30);
                m.encode(sink);
            }
            Instruction::I64Load8U(m) => {
                sink.push(0x31);
                m.encode(sink);
            }
            Instruction::I64Load16S(m) => {
                sink.push(0x32);
                m.encode(sink);
            }
            Instruction::I64Load16U(m) => {
                sink.push(0x33);
                m.encode(sink);
            }
            Instruction::I64Load32S(m) => {
                sink.push(0x34);
                m.encode(sink);
            }
            Instruction::I64Load32U(m) => {
                sink.push(0x35);
                m.encode(sink);
            }
            Instruction::I32Store(m) => {
                sink.push(0x36);
                m.encode(sink);
            }
            Instruction::I64Store(m) => {
                sink.push(0x37);
                m.encode(sink);
            }
            Instruction::F32Store(m) => {
                sink.push(0x38);
                m.encode(sink);
            }
            Instruction::F64Store(m) => {
                sink.push(0x39);
                m.encode(sink);
            }
            Instruction::I32Store8(m) => {
                sink.push(0x3A);
                m.encode(sink);
            }
            Instruction::I32Store16(m) => {
                sink.push(0x3B);
                m.encode(sink);
            }
            Instruction::I64Store8(m) => {
                sink.push(0x3C);
                m.encode(sink);
            }
            Instruction::I64Store16(m) => {
                sink.push(0x3D);
                m.encode(sink);
            }
            Instruction::I64Store32(m) => {
                sink.push(0x3E);
                m.encode(sink);
            }
            Instruction::MemorySize(i) => {
                sink.push(0x3F);
                i.encode(sink);
            }
            Instruction::MemoryGrow(i) => {
                sink.push(0x40);
                i.encode(sink);
            }
            Instruction::MemoryInit { mem, data_index } => {
                sink.push(0xfc);
                sink.push(0x08);
                data_index.encode(sink);
                mem.encode(sink);
            }
            Instruction::DataDrop(data) => {
                sink.push(0xfc);
                sink.push(0x09);
                data.encode(sink);
            }
            Instruction::MemoryCopy { src_mem, dst_mem } => {
                sink.push(0xfc);
                sink.push(0x0a);
                dst_mem.encode(sink);
                src_mem.encode(sink);
            }
            Instruction::MemoryFill(mem) => {
                sink.push(0xfc);
                sink.push(0x0b);
                mem.encode(sink);
            }
            Instruction::MemoryDiscard(mem) => {
                sink.push(0xfc);
                sink.push(0x12);
                mem.encode(sink);
            }

            // Numeric instructions.
            Instruction::I32Const(x) => {
                sink.push(0x41);
                x.encode(sink);
            }
            Instruction::I64Const(x) => {
                sink.push(0x42);
                x.encode(sink);
            }
            Instruction::F32Const(x) => {
                sink.push(0x43);
                let x = x.to_bits();
                sink.extend(x.to_le_bytes().iter().copied());
            }
            Instruction::F64Const(x) => {
                sink.push(0x44);
                let x = x.to_bits();
                sink.extend(x.to_le_bytes().iter().copied());
            }
            Instruction::I32Eqz => sink.push(0x45),
            Instruction::I32Eq => sink.push(0x46),
            Instruction::I32Ne => sink.push(0x47),
            Instruction::I32LtS => sink.push(0x48),
            Instruction::I32LtU => sink.push(0x49),
            Instruction::I32GtS => sink.push(0x4A),
            Instruction::I32GtU => sink.push(0x4B),
            Instruction::I32LeS => sink.push(0x4C),
            Instruction::I32LeU => sink.push(0x4D),
            Instruction::I32GeS => sink.push(0x4E),
            Instruction::I32GeU => sink.push(0x4F),
            Instruction::I64Eqz => sink.push(0x50),
            Instruction::I64Eq => sink.push(0x51),
            Instruction::I64Ne => sink.push(0x52),
            Instruction::I64LtS => sink.push(0x53),
            Instruction::I64LtU => sink.push(0x54),
            Instruction::I64GtS => sink.push(0x55),
            Instruction::I64GtU => sink.push(0x56),
            Instruction::I64LeS => sink.push(0x57),
            Instruction::I64LeU => sink.push(0x58),
            Instruction::I64GeS => sink.push(0x59),
            Instruction::I64GeU => sink.push(0x5A),
            Instruction::F32Eq => sink.push(0x5B),
            Instruction::F32Ne => sink.push(0x5C),
            Instruction::F32Lt => sink.push(0x5D),
            Instruction::F32Gt => sink.push(0x5E),
            Instruction::F32Le => sink.push(0x5F),
            Instruction::F32Ge => sink.push(0x60),
            Instruction::F64Eq => sink.push(0x61),
            Instruction::F64Ne => sink.push(0x62),
            Instruction::F64Lt => sink.push(0x63),
            Instruction::F64Gt => sink.push(0x64),
            Instruction::F64Le => sink.push(0x65),
            Instruction::F64Ge => sink.push(0x66),
            Instruction::I32Clz => sink.push(0x67),
            Instruction::I32Ctz => sink.push(0x68),
            Instruction::I32Popcnt => sink.push(0x69),
            Instruction::I32Add => sink.push(0x6A),
            Instruction::I32Sub => sink.push(0x6B),
            Instruction::I32Mul => sink.push(0x6C),
            Instruction::I32DivS => sink.push(0x6D),
            Instruction::I32DivU => sink.push(0x6E),
            Instruction::I32RemS => sink.push(0x6F),
            Instruction::I32RemU => sink.push(0x70),
            Instruction::I32And => sink.push(0x71),
            Instruction::I32Or => sink.push(0x72),
            Instruction::I32Xor => sink.push(0x73),
            Instruction::I32Shl => sink.push(0x74),
            Instruction::I32ShrS => sink.push(0x75),
            Instruction::I32ShrU => sink.push(0x76),
            Instruction::I32Rotl => sink.push(0x77),
            Instruction::I32Rotr => sink.push(0x78),
            Instruction::I64Clz => sink.push(0x79),
            Instruction::I64Ctz => sink.push(0x7A),
            Instruction::I64Popcnt => sink.push(0x7B),
            Instruction::I64Add => sink.push(0x7C),
            Instruction::I64Sub => sink.push(0x7D),
            Instruction::I64Mul => sink.push(0x7E),
            Instruction::I64DivS => sink.push(0x7F),
            Instruction::I64DivU => sink.push(0x80),
            Instruction::I64RemS => sink.push(0x81),
            Instruction::I64RemU => sink.push(0x82),
            Instruction::I64And => sink.push(0x83),
            Instruction::I64Or => sink.push(0x84),
            Instruction::I64Xor => sink.push(0x85),
            Instruction::I64Shl => sink.push(0x86),
            Instruction::I64ShrS => sink.push(0x87),
            Instruction::I64ShrU => sink.push(0x88),
            Instruction::I64Rotl => sink.push(0x89),
            Instruction::I64Rotr => sink.push(0x8A),
            Instruction::F32Abs => sink.push(0x8B),
            Instruction::F32Neg => sink.push(0x8C),
            Instruction::F32Ceil => sink.push(0x8D),
            Instruction::F32Floor => sink.push(0x8E),
            Instruction::F32Trunc => sink.push(0x8F),
            Instruction::F32Nearest => sink.push(0x90),
            Instruction::F32Sqrt => sink.push(0x91),
            Instruction::F32Add => sink.push(0x92),
            Instruction::F32Sub => sink.push(0x93),
            Instruction::F32Mul => sink.push(0x94),
            Instruction::F32Div => sink.push(0x95),
            Instruction::F32Min => sink.push(0x96),
            Instruction::F32Max => sink.push(0x97),
            Instruction::F32Copysign => sink.push(0x98),
            Instruction::F64Abs => sink.push(0x99),
            Instruction::F64Neg => sink.push(0x9A),
            Instruction::F64Ceil => sink.push(0x9B),
            Instruction::F64Floor => sink.push(0x9C),
            Instruction::F64Trunc => sink.push(0x9D),
            Instruction::F64Nearest => sink.push(0x9E),
            Instruction::F64Sqrt => sink.push(0x9F),
            Instruction::F64Add => sink.push(0xA0),
            Instruction::F64Sub => sink.push(0xA1),
            Instruction::F64Mul => sink.push(0xA2),
            Instruction::F64Div => sink.push(0xA3),
            Instruction::F64Min => sink.push(0xA4),
            Instruction::F64Max => sink.push(0xA5),
            Instruction::F64Copysign => sink.push(0xA6),
            Instruction::I32WrapI64 => sink.push(0xA7),
            Instruction::I32TruncF32S => sink.push(0xA8),
            Instruction::I32TruncF32U => sink.push(0xA9),
            Instruction::I32TruncF64S => sink.push(0xAA),
            Instruction::I32TruncF64U => sink.push(0xAB),
            Instruction::I64ExtendI32S => sink.push(0xAC),
            Instruction::I64ExtendI32U => sink.push(0xAD),
            Instruction::I64TruncF32S => sink.push(0xAE),
            Instruction::I64TruncF32U => sink.push(0xAF),
            Instruction::I64TruncF64S => sink.push(0xB0),
            Instruction::I64TruncF64U => sink.push(0xB1),
            Instruction::F32ConvertI32S => sink.push(0xB2),
            Instruction::F32ConvertI32U => sink.push(0xB3),
            Instruction::F32ConvertI64S => sink.push(0xB4),
            Instruction::F32ConvertI64U => sink.push(0xB5),
            Instruction::F32DemoteF64 => sink.push(0xB6),
            Instruction::F64ConvertI32S => sink.push(0xB7),
            Instruction::F64ConvertI32U => sink.push(0xB8),
            Instruction::F64ConvertI64S => sink.push(0xB9),
            Instruction::F64ConvertI64U => sink.push(0xBA),
            Instruction::F64PromoteF32 => sink.push(0xBB),
            Instruction::I32ReinterpretF32 => sink.push(0xBC),
            Instruction::I64ReinterpretF64 => sink.push(0xBD),
            Instruction::F32ReinterpretI32 => sink.push(0xBE),
            Instruction::F64ReinterpretI64 => sink.push(0xBF),
            Instruction::I32Extend8S => sink.push(0xC0),
            Instruction::I32Extend16S => sink.push(0xC1),
            Instruction::I64Extend8S => sink.push(0xC2),
            Instruction::I64Extend16S => sink.push(0xC3),
            Instruction::I64Extend32S => sink.push(0xC4),

            Instruction::I32TruncSatF32S => {
                sink.push(0xFC);
                sink.push(0x00);
            }
            Instruction::I32TruncSatF32U => {
                sink.push(0xFC);
                sink.push(0x01);
            }
            Instruction::I32TruncSatF64S => {
                sink.push(0xFC);
                sink.push(0x02);
            }
            Instruction::I32TruncSatF64U => {
                sink.push(0xFC);
                sink.push(0x03);
            }
            Instruction::I64TruncSatF32S => {
                sink.push(0xFC);
                sink.push(0x04);
            }
            Instruction::I64TruncSatF32U => {
                sink.push(0xFC);
                sink.push(0x05);
            }
            Instruction::I64TruncSatF64S => {
                sink.push(0xFC);
                sink.push(0x06);
            }
            Instruction::I64TruncSatF64U => {
                sink.push(0xFC);
                sink.push(0x07);
            }

            // Reference types instructions.
            Instruction::RefNull(ty) => {
                sink.push(0xd0);
                ty.encode(sink);
            }
            Instruction::RefIsNull => sink.push(0xd1),
            Instruction::RefFunc(f) => {
                sink.push(0xd2);
                f.encode(sink);
            }
            Instruction::RefEq => sink.push(0xd3),
            Instruction::RefAsNonNull => sink.push(0xd4),

            // GC instructions.
            Instruction::StructNew(type_index) => {
                sink.push(0xfb);
                sink.push(0x00);
                type_index.encode(sink);
            }
            Instruction::StructNewDefault(type_index) => {
                sink.push(0xfb);
                sink.push(0x01);
                type_index.encode(sink);
            }
            Instruction::StructGet {
                struct_type_index,
                field_index,
            } => {
                sink.push(0xfb);
                sink.push(0x02);
                struct_type_index.encode(sink);
                field_index.encode(sink);
            }
            Instruction::StructGetS {
                struct_type_index,
                field_index,
            } => {
                sink.push(0xfb);
                sink.push(0x03);
                struct_type_index.encode(sink);
                field_index.encode(sink);
            }
            Instruction::StructGetU {
                struct_type_index,
                field_index,
            } => {
                sink.push(0xfb);
                sink.push(0x04);
                struct_type_index.encode(sink);
                field_index.encode(sink);
            }
            Instruction::StructSet {
                struct_type_index,
                field_index,
            } => {
                sink.push(0xfb);
                sink.push(0x05);
                struct_type_index.encode(sink);
                field_index.encode(sink);
            }
            Instruction::ArrayNew(type_index) => {
                sink.push(0xfb);
                sink.push(0x06);
                type_index.encode(sink);
            }
            Instruction::ArrayNewDefault(type_index) => {
                sink.push(0xfb);
                sink.push(0x07);
                type_index.encode(sink);
            }
            Instruction::ArrayNewFixed {
                array_type_index,
                array_size,
            } => {
                sink.push(0xfb);
                sink.push(0x08);
                array_type_index.encode(sink);
                array_size.encode(sink);
            }
            Instruction::ArrayNewData {
                array_type_index,
                array_data_index,
            } => {
                sink.push(0xfb);
                sink.push(0x09);
                array_type_index.encode(sink);
                array_data_index.encode(sink);
            }
            Instruction::ArrayNewElem {
                array_type_index,
                array_elem_index,
            } => {
                sink.push(0xfb);
                sink.push(0x0a);
                array_type_index.encode(sink);
                array_elem_index.encode(sink);
            }
            Instruction::ArrayGet(type_index) => {
                sink.push(0xfb);
                sink.push(0x0b);
                type_index.encode(sink);
            }
            Instruction::ArrayGetS(type_index) => {
                sink.push(0xfb);
                sink.push(0x0c);
                type_index.encode(sink);
            }
            Instruction::ArrayGetU(type_index) => {
                sink.push(0xfb);
                sink.push(0x0d);
                type_index.encode(sink);
            }
            Instruction::ArraySet(type_index) => {
                sink.push(0xfb);
                sink.push(0x0e);
                type_index.encode(sink);
            }
            Instruction::ArrayLen => {
                sink.push(0xfb);
                sink.push(0x0f);
            }
            Instruction::ArrayFill(type_index) => {
                sink.push(0xfb);
                sink.push(0x10);
                type_index.encode(sink);
            }
            Instruction::ArrayCopy {
                array_type_index_dst,
                array_type_index_src,
            } => {
                sink.push(0xfb);
                sink.push(0x11);
                array_type_index_dst.encode(sink);
                array_type_index_src.encode(sink);
            }
            Instruction::ArrayInitData {
                array_type_index,
                array_data_index,
            } => {
                sink.push(0xfb);
                sink.push(0x12);
                array_type_index.encode(sink);
                array_data_index.encode(sink);
            }
            Instruction::ArrayInitElem {
                array_type_index,
                array_elem_index,
            } => {
                sink.push(0xfb);
                sink.push(0x13);
                array_type_index.encode(sink);
                array_elem_index.encode(sink);
            }
            Instruction::RefTestNonNull(heap_type) => {
                sink.push(0xfb);
                sink.push(0x14);
                heap_type.encode(sink);
            }
            Instruction::RefTestNullable(heap_type) => {
                sink.push(0xfb);
                sink.push(0x15);
                heap_type.encode(sink);
            }
            Instruction::RefCastNonNull(heap_type) => {
                sink.push(0xfb);
                sink.push(0x16);
                heap_type.encode(sink);
            }
            Instruction::RefCastNullable(heap_type) => {
                sink.push(0xfb);
                sink.push(0x17);
                heap_type.encode(sink);
            }
            Instruction::BrOnCast {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => {
                sink.push(0xfb);
                sink.push(0x18);
                let cast_flags =
                    (from_ref_type.nullable as u8) | ((to_ref_type.nullable as u8) << 1);
                sink.push(cast_flags);
                relative_depth.encode(sink);
                from_ref_type.heap_type.encode(sink);
                to_ref_type.heap_type.encode(sink);
            }
            Instruction::BrOnCastFail {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => {
                sink.push(0xfb);
                sink.push(0x19);
                let cast_flags =
                    (from_ref_type.nullable as u8) | ((to_ref_type.nullable as u8) << 1);
                sink.push(cast_flags);
                relative_depth.encode(sink);
                from_ref_type.heap_type.encode(sink);
                to_ref_type.heap_type.encode(sink);
            }
            Instruction::AnyConvertExtern => {
                sink.push(0xfb);
                sink.push(0x1a);
            }
            Instruction::ExternConvertAny => {
                sink.push(0xfb);
                sink.push(0x1b);
            }
            Instruction::RefI31 => {
                sink.push(0xfb);
                sink.push(0x1c);
            }
            Instruction::I31GetS => {
                sink.push(0xfb);
                sink.push(0x1d);
            }
            Instruction::I31GetU => {
                sink.push(0xfb);
                sink.push(0x1e);
            }

            // Bulk memory instructions.
            Instruction::TableInit { elem_index, table } => {
                sink.push(0xfc);
                sink.push(0x0c);
                elem_index.encode(sink);
                table.encode(sink);
            }
            Instruction::ElemDrop(segment) => {
                sink.push(0xfc);
                sink.push(0x0d);
                segment.encode(sink);
            }
            Instruction::TableCopy {
                src_table,
                dst_table,
            } => {
                sink.push(0xfc);
                sink.push(0x0e);
                dst_table.encode(sink);
                src_table.encode(sink);
            }
            Instruction::TableGrow(table) => {
                sink.push(0xfc);
                sink.push(0x0f);
                table.encode(sink);
            }
            Instruction::TableSize(table) => {
                sink.push(0xfc);
                sink.push(0x10);
                table.encode(sink);
            }
            Instruction::TableFill(table) => {
                sink.push(0xfc);
                sink.push(0x11);
                table.encode(sink);
            }

            // SIMD instructions.
            Instruction::V128Load(memarg) => {
                sink.push(0xFD);
                0x00u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load8x8S(memarg) => {
                sink.push(0xFD);
                0x01u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load8x8U(memarg) => {
                sink.push(0xFD);
                0x02u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load16x4S(memarg) => {
                sink.push(0xFD);
                0x03u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load16x4U(memarg) => {
                sink.push(0xFD);
                0x04u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load32x2S(memarg) => {
                sink.push(0xFD);
                0x05u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load32x2U(memarg) => {
                sink.push(0xFD);
                0x06u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load8Splat(memarg) => {
                sink.push(0xFD);
                0x07u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load16Splat(memarg) => {
                sink.push(0xFD);
                0x08u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load32Splat(memarg) => {
                sink.push(0xFD);
                0x09u32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load64Splat(memarg) => {
                sink.push(0xFD);
                0x0Au32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Store(memarg) => {
                sink.push(0xFD);
                0x0Bu32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Const(x) => {
                sink.push(0xFD);
                0x0Cu32.encode(sink);
                sink.extend(x.to_le_bytes().iter().copied());
            }
            Instruction::I8x16Shuffle(lanes) => {
                sink.push(0xFD);
                0x0Du32.encode(sink);
                assert!(lanes.iter().all(|l: &u8| *l < 32));
                sink.extend(lanes.iter().copied());
            }
            Instruction::I8x16Swizzle => {
                sink.push(0xFD);
                0x0Eu32.encode(sink);
            }
            Instruction::I8x16Splat => {
                sink.push(0xFD);
                0x0Fu32.encode(sink);
            }
            Instruction::I16x8Splat => {
                sink.push(0xFD);
                0x10u32.encode(sink);
            }
            Instruction::I32x4Splat => {
                sink.push(0xFD);
                0x11u32.encode(sink);
            }
            Instruction::I64x2Splat => {
                sink.push(0xFD);
                0x12u32.encode(sink);
            }
            Instruction::F32x4Splat => {
                sink.push(0xFD);
                0x13u32.encode(sink);
            }
            Instruction::F64x2Splat => {
                sink.push(0xFD);
                0x14u32.encode(sink);
            }
            Instruction::I8x16ExtractLaneS(lane) => {
                sink.push(0xFD);
                0x15u32.encode(sink);
                assert!(lane < 16);
                sink.push(lane);
            }
            Instruction::I8x16ExtractLaneU(lane) => {
                sink.push(0xFD);
                0x16u32.encode(sink);
                assert!(lane < 16);
                sink.push(lane);
            }
            Instruction::I8x16ReplaceLane(lane) => {
                sink.push(0xFD);
                0x17u32.encode(sink);
                assert!(lane < 16);
                sink.push(lane);
            }
            Instruction::I16x8ExtractLaneS(lane) => {
                sink.push(0xFD);
                0x18u32.encode(sink);
                assert!(lane < 8);
                sink.push(lane);
            }
            Instruction::I16x8ExtractLaneU(lane) => {
                sink.push(0xFD);
                0x19u32.encode(sink);
                assert!(lane < 8);
                sink.push(lane);
            }
            Instruction::I16x8ReplaceLane(lane) => {
                sink.push(0xFD);
                0x1Au32.encode(sink);
                assert!(lane < 8);
                sink.push(lane);
            }
            Instruction::I32x4ExtractLane(lane) => {
                sink.push(0xFD);
                0x1Bu32.encode(sink);
                assert!(lane < 4);
                sink.push(lane);
            }
            Instruction::I32x4ReplaceLane(lane) => {
                sink.push(0xFD);
                0x1Cu32.encode(sink);
                assert!(lane < 4);
                sink.push(lane);
            }
            Instruction::I64x2ExtractLane(lane) => {
                sink.push(0xFD);
                0x1Du32.encode(sink);
                assert!(lane < 2);
                sink.push(lane);
            }
            Instruction::I64x2ReplaceLane(lane) => {
                sink.push(0xFD);
                0x1Eu32.encode(sink);
                assert!(lane < 2);
                sink.push(lane);
            }
            Instruction::F32x4ExtractLane(lane) => {
                sink.push(0xFD);
                0x1Fu32.encode(sink);
                assert!(lane < 4);
                sink.push(lane);
            }
            Instruction::F32x4ReplaceLane(lane) => {
                sink.push(0xFD);
                0x20u32.encode(sink);
                assert!(lane < 4);
                sink.push(lane);
            }
            Instruction::F64x2ExtractLane(lane) => {
                sink.push(0xFD);
                0x21u32.encode(sink);
                assert!(lane < 2);
                sink.push(lane);
            }
            Instruction::F64x2ReplaceLane(lane) => {
                sink.push(0xFD);
                0x22u32.encode(sink);
                assert!(lane < 2);
                sink.push(lane);
            }

            Instruction::I8x16Eq => {
                sink.push(0xFD);
                0x23u32.encode(sink);
            }
            Instruction::I8x16Ne => {
                sink.push(0xFD);
                0x24u32.encode(sink);
            }
            Instruction::I8x16LtS => {
                sink.push(0xFD);
                0x25u32.encode(sink);
            }
            Instruction::I8x16LtU => {
                sink.push(0xFD);
                0x26u32.encode(sink);
            }
            Instruction::I8x16GtS => {
                sink.push(0xFD);
                0x27u32.encode(sink);
            }
            Instruction::I8x16GtU => {
                sink.push(0xFD);
                0x28u32.encode(sink);
            }
            Instruction::I8x16LeS => {
                sink.push(0xFD);
                0x29u32.encode(sink);
            }
            Instruction::I8x16LeU => {
                sink.push(0xFD);
                0x2Au32.encode(sink);
            }
            Instruction::I8x16GeS => {
                sink.push(0xFD);
                0x2Bu32.encode(sink);
            }
            Instruction::I8x16GeU => {
                sink.push(0xFD);
                0x2Cu32.encode(sink);
            }
            Instruction::I16x8Eq => {
                sink.push(0xFD);
                0x2Du32.encode(sink);
            }
            Instruction::I16x8Ne => {
                sink.push(0xFD);
                0x2Eu32.encode(sink);
            }
            Instruction::I16x8LtS => {
                sink.push(0xFD);
                0x2Fu32.encode(sink);
            }
            Instruction::I16x8LtU => {
                sink.push(0xFD);
                0x30u32.encode(sink);
            }
            Instruction::I16x8GtS => {
                sink.push(0xFD);
                0x31u32.encode(sink);
            }
            Instruction::I16x8GtU => {
                sink.push(0xFD);
                0x32u32.encode(sink);
            }
            Instruction::I16x8LeS => {
                sink.push(0xFD);
                0x33u32.encode(sink);
            }
            Instruction::I16x8LeU => {
                sink.push(0xFD);
                0x34u32.encode(sink);
            }
            Instruction::I16x8GeS => {
                sink.push(0xFD);
                0x35u32.encode(sink);
            }
            Instruction::I16x8GeU => {
                sink.push(0xFD);
                0x36u32.encode(sink);
            }
            Instruction::I32x4Eq => {
                sink.push(0xFD);
                0x37u32.encode(sink);
            }
            Instruction::I32x4Ne => {
                sink.push(0xFD);
                0x38u32.encode(sink);
            }
            Instruction::I32x4LtS => {
                sink.push(0xFD);
                0x39u32.encode(sink);
            }
            Instruction::I32x4LtU => {
                sink.push(0xFD);
                0x3Au32.encode(sink);
            }
            Instruction::I32x4GtS => {
                sink.push(0xFD);
                0x3Bu32.encode(sink);
            }
            Instruction::I32x4GtU => {
                sink.push(0xFD);
                0x3Cu32.encode(sink);
            }
            Instruction::I32x4LeS => {
                sink.push(0xFD);
                0x3Du32.encode(sink);
            }
            Instruction::I32x4LeU => {
                sink.push(0xFD);
                0x3Eu32.encode(sink);
            }
            Instruction::I32x4GeS => {
                sink.push(0xFD);
                0x3Fu32.encode(sink);
            }
            Instruction::I32x4GeU => {
                sink.push(0xFD);
                0x40u32.encode(sink);
            }
            Instruction::F32x4Eq => {
                sink.push(0xFD);
                0x41u32.encode(sink);
            }
            Instruction::F32x4Ne => {
                sink.push(0xFD);
                0x42u32.encode(sink);
            }
            Instruction::F32x4Lt => {
                sink.push(0xFD);
                0x43u32.encode(sink);
            }
            Instruction::F32x4Gt => {
                sink.push(0xFD);
                0x44u32.encode(sink);
            }
            Instruction::F32x4Le => {
                sink.push(0xFD);
                0x45u32.encode(sink);
            }
            Instruction::F32x4Ge => {
                sink.push(0xFD);
                0x46u32.encode(sink);
            }
            Instruction::F64x2Eq => {
                sink.push(0xFD);
                0x47u32.encode(sink);
            }
            Instruction::F64x2Ne => {
                sink.push(0xFD);
                0x48u32.encode(sink);
            }
            Instruction::F64x2Lt => {
                sink.push(0xFD);
                0x49u32.encode(sink);
            }
            Instruction::F64x2Gt => {
                sink.push(0xFD);
                0x4Au32.encode(sink);
            }
            Instruction::F64x2Le => {
                sink.push(0xFD);
                0x4Bu32.encode(sink);
            }
            Instruction::F64x2Ge => {
                sink.push(0xFD);
                0x4Cu32.encode(sink);
            }
            Instruction::V128Not => {
                sink.push(0xFD);
                0x4Du32.encode(sink);
            }
            Instruction::V128And => {
                sink.push(0xFD);
                0x4Eu32.encode(sink);
            }
            Instruction::V128AndNot => {
                sink.push(0xFD);
                0x4Fu32.encode(sink);
            }
            Instruction::V128Or => {
                sink.push(0xFD);
                0x50u32.encode(sink);
            }
            Instruction::V128Xor => {
                sink.push(0xFD);
                0x51u32.encode(sink);
            }
            Instruction::V128Bitselect => {
                sink.push(0xFD);
                0x52u32.encode(sink);
            }
            Instruction::V128AnyTrue => {
                sink.push(0xFD);
                0x53u32.encode(sink);
            }
            Instruction::I8x16Abs => {
                sink.push(0xFD);
                0x60u32.encode(sink);
            }
            Instruction::I8x16Neg => {
                sink.push(0xFD);
                0x61u32.encode(sink);
            }
            Instruction::I8x16Popcnt => {
                sink.push(0xFD);
                0x62u32.encode(sink);
            }
            Instruction::I8x16AllTrue => {
                sink.push(0xFD);
                0x63u32.encode(sink);
            }
            Instruction::I8x16Bitmask => {
                sink.push(0xFD);
                0x64u32.encode(sink);
            }
            Instruction::I8x16NarrowI16x8S => {
                sink.push(0xFD);
                0x65u32.encode(sink);
            }
            Instruction::I8x16NarrowI16x8U => {
                sink.push(0xFD);
                0x66u32.encode(sink);
            }
            Instruction::I8x16Shl => {
                sink.push(0xFD);
                0x6bu32.encode(sink);
            }
            Instruction::I8x16ShrS => {
                sink.push(0xFD);
                0x6cu32.encode(sink);
            }
            Instruction::I8x16ShrU => {
                sink.push(0xFD);
                0x6du32.encode(sink);
            }
            Instruction::I8x16Add => {
                sink.push(0xFD);
                0x6eu32.encode(sink);
            }
            Instruction::I8x16AddSatS => {
                sink.push(0xFD);
                0x6fu32.encode(sink);
            }
            Instruction::I8x16AddSatU => {
                sink.push(0xFD);
                0x70u32.encode(sink);
            }
            Instruction::I8x16Sub => {
                sink.push(0xFD);
                0x71u32.encode(sink);
            }
            Instruction::I8x16SubSatS => {
                sink.push(0xFD);
                0x72u32.encode(sink);
            }
            Instruction::I8x16SubSatU => {
                sink.push(0xFD);
                0x73u32.encode(sink);
            }
            Instruction::I8x16MinS => {
                sink.push(0xFD);
                0x76u32.encode(sink);
            }
            Instruction::I8x16MinU => {
                sink.push(0xFD);
                0x77u32.encode(sink);
            }
            Instruction::I8x16MaxS => {
                sink.push(0xFD);
                0x78u32.encode(sink);
            }
            Instruction::I8x16MaxU => {
                sink.push(0xFD);
                0x79u32.encode(sink);
            }
            Instruction::I8x16AvgrU => {
                sink.push(0xFD);
                0x7Bu32.encode(sink);
            }
            Instruction::I16x8ExtAddPairwiseI8x16S => {
                sink.push(0xFD);
                0x7Cu32.encode(sink);
            }
            Instruction::I16x8ExtAddPairwiseI8x16U => {
                sink.push(0xFD);
                0x7Du32.encode(sink);
            }
            Instruction::I32x4ExtAddPairwiseI16x8S => {
                sink.push(0xFD);
                0x7Eu32.encode(sink);
            }
            Instruction::I32x4ExtAddPairwiseI16x8U => {
                sink.push(0xFD);
                0x7Fu32.encode(sink);
            }
            Instruction::I16x8Abs => {
                sink.push(0xFD);
                0x80u32.encode(sink);
            }
            Instruction::I16x8Neg => {
                sink.push(0xFD);
                0x81u32.encode(sink);
            }
            Instruction::I16x8Q15MulrSatS => {
                sink.push(0xFD);
                0x82u32.encode(sink);
            }
            Instruction::I16x8AllTrue => {
                sink.push(0xFD);
                0x83u32.encode(sink);
            }
            Instruction::I16x8Bitmask => {
                sink.push(0xFD);
                0x84u32.encode(sink);
            }
            Instruction::I16x8NarrowI32x4S => {
                sink.push(0xFD);
                0x85u32.encode(sink);
            }
            Instruction::I16x8NarrowI32x4U => {
                sink.push(0xFD);
                0x86u32.encode(sink);
            }
            Instruction::I16x8ExtendLowI8x16S => {
                sink.push(0xFD);
                0x87u32.encode(sink);
            }
            Instruction::I16x8ExtendHighI8x16S => {
                sink.push(0xFD);
                0x88u32.encode(sink);
            }
            Instruction::I16x8ExtendLowI8x16U => {
                sink.push(0xFD);
                0x89u32.encode(sink);
            }
            Instruction::I16x8ExtendHighI8x16U => {
                sink.push(0xFD);
                0x8Au32.encode(sink);
            }
            Instruction::I16x8Shl => {
                sink.push(0xFD);
                0x8Bu32.encode(sink);
            }
            Instruction::I16x8ShrS => {
                sink.push(0xFD);
                0x8Cu32.encode(sink);
            }
            Instruction::I16x8ShrU => {
                sink.push(0xFD);
                0x8Du32.encode(sink);
            }
            Instruction::I16x8Add => {
                sink.push(0xFD);
                0x8Eu32.encode(sink);
            }
            Instruction::I16x8AddSatS => {
                sink.push(0xFD);
                0x8Fu32.encode(sink);
            }
            Instruction::I16x8AddSatU => {
                sink.push(0xFD);
                0x90u32.encode(sink);
            }
            Instruction::I16x8Sub => {
                sink.push(0xFD);
                0x91u32.encode(sink);
            }
            Instruction::I16x8SubSatS => {
                sink.push(0xFD);
                0x92u32.encode(sink);
            }
            Instruction::I16x8SubSatU => {
                sink.push(0xFD);
                0x93u32.encode(sink);
            }
            Instruction::I16x8Mul => {
                sink.push(0xFD);
                0x95u32.encode(sink);
            }
            Instruction::I16x8MinS => {
                sink.push(0xFD);
                0x96u32.encode(sink);
            }
            Instruction::I16x8MinU => {
                sink.push(0xFD);
                0x97u32.encode(sink);
            }
            Instruction::I16x8MaxS => {
                sink.push(0xFD);
                0x98u32.encode(sink);
            }
            Instruction::I16x8MaxU => {
                sink.push(0xFD);
                0x99u32.encode(sink);
            }
            Instruction::I16x8AvgrU => {
                sink.push(0xFD);
                0x9Bu32.encode(sink);
            }
            Instruction::I16x8ExtMulLowI8x16S => {
                sink.push(0xFD);
                0x9Cu32.encode(sink);
            }
            Instruction::I16x8ExtMulHighI8x16S => {
                sink.push(0xFD);
                0x9Du32.encode(sink);
            }
            Instruction::I16x8ExtMulLowI8x16U => {
                sink.push(0xFD);
                0x9Eu32.encode(sink);
            }
            Instruction::I16x8ExtMulHighI8x16U => {
                sink.push(0xFD);
                0x9Fu32.encode(sink);
            }
            Instruction::I32x4Abs => {
                sink.push(0xFD);
                0xA0u32.encode(sink);
            }
            Instruction::I32x4Neg => {
                sink.push(0xFD);
                0xA1u32.encode(sink);
            }
            Instruction::I32x4AllTrue => {
                sink.push(0xFD);
                0xA3u32.encode(sink);
            }
            Instruction::I32x4Bitmask => {
                sink.push(0xFD);
                0xA4u32.encode(sink);
            }
            Instruction::I32x4ExtendLowI16x8S => {
                sink.push(0xFD);
                0xA7u32.encode(sink);
            }
            Instruction::I32x4ExtendHighI16x8S => {
                sink.push(0xFD);
                0xA8u32.encode(sink);
            }
            Instruction::I32x4ExtendLowI16x8U => {
                sink.push(0xFD);
                0xA9u32.encode(sink);
            }
            Instruction::I32x4ExtendHighI16x8U => {
                sink.push(0xFD);
                0xAAu32.encode(sink);
            }
            Instruction::I32x4Shl => {
                sink.push(0xFD);
                0xABu32.encode(sink);
            }
            Instruction::I32x4ShrS => {
                sink.push(0xFD);
                0xACu32.encode(sink);
            }
            Instruction::I32x4ShrU => {
                sink.push(0xFD);
                0xADu32.encode(sink);
            }
            Instruction::I32x4Add => {
                sink.push(0xFD);
                0xAEu32.encode(sink);
            }
            Instruction::I32x4Sub => {
                sink.push(0xFD);
                0xB1u32.encode(sink);
            }
            Instruction::I32x4Mul => {
                sink.push(0xFD);
                0xB5u32.encode(sink);
            }
            Instruction::I32x4MinS => {
                sink.push(0xFD);
                0xB6u32.encode(sink);
            }
            Instruction::I32x4MinU => {
                sink.push(0xFD);
                0xB7u32.encode(sink);
            }
            Instruction::I32x4MaxS => {
                sink.push(0xFD);
                0xB8u32.encode(sink);
            }
            Instruction::I32x4MaxU => {
                sink.push(0xFD);
                0xB9u32.encode(sink);
            }
            Instruction::I32x4DotI16x8S => {
                sink.push(0xFD);
                0xBAu32.encode(sink);
            }
            Instruction::I32x4ExtMulLowI16x8S => {
                sink.push(0xFD);
                0xBCu32.encode(sink);
            }
            Instruction::I32x4ExtMulHighI16x8S => {
                sink.push(0xFD);
                0xBDu32.encode(sink);
            }
            Instruction::I32x4ExtMulLowI16x8U => {
                sink.push(0xFD);
                0xBEu32.encode(sink);
            }
            Instruction::I32x4ExtMulHighI16x8U => {
                sink.push(0xFD);
                0xBFu32.encode(sink);
            }
            Instruction::I64x2Abs => {
                sink.push(0xFD);
                0xC0u32.encode(sink);
            }
            Instruction::I64x2Neg => {
                sink.push(0xFD);
                0xC1u32.encode(sink);
            }
            Instruction::I64x2AllTrue => {
                sink.push(0xFD);
                0xC3u32.encode(sink);
            }
            Instruction::I64x2Bitmask => {
                sink.push(0xFD);
                0xC4u32.encode(sink);
            }
            Instruction::I64x2ExtendLowI32x4S => {
                sink.push(0xFD);
                0xC7u32.encode(sink);
            }
            Instruction::I64x2ExtendHighI32x4S => {
                sink.push(0xFD);
                0xC8u32.encode(sink);
            }
            Instruction::I64x2ExtendLowI32x4U => {
                sink.push(0xFD);
                0xC9u32.encode(sink);
            }
            Instruction::I64x2ExtendHighI32x4U => {
                sink.push(0xFD);
                0xCAu32.encode(sink);
            }
            Instruction::I64x2Shl => {
                sink.push(0xFD);
                0xCBu32.encode(sink);
            }
            Instruction::I64x2ShrS => {
                sink.push(0xFD);
                0xCCu32.encode(sink);
            }
            Instruction::I64x2ShrU => {
                sink.push(0xFD);
                0xCDu32.encode(sink);
            }
            Instruction::I64x2Add => {
                sink.push(0xFD);
                0xCEu32.encode(sink);
            }
            Instruction::I64x2Sub => {
                sink.push(0xFD);
                0xD1u32.encode(sink);
            }
            Instruction::I64x2Mul => {
                sink.push(0xFD);
                0xD5u32.encode(sink);
            }
            Instruction::I64x2ExtMulLowI32x4S => {
                sink.push(0xFD);
                0xDCu32.encode(sink);
            }
            Instruction::I64x2ExtMulHighI32x4S => {
                sink.push(0xFD);
                0xDDu32.encode(sink);
            }
            Instruction::I64x2ExtMulLowI32x4U => {
                sink.push(0xFD);
                0xDEu32.encode(sink);
            }
            Instruction::I64x2ExtMulHighI32x4U => {
                sink.push(0xFD);
                0xDFu32.encode(sink);
            }
            Instruction::F32x4Ceil => {
                sink.push(0xFD);
                0x67u32.encode(sink);
            }
            Instruction::F32x4Floor => {
                sink.push(0xFD);
                0x68u32.encode(sink);
            }
            Instruction::F32x4Trunc => {
                sink.push(0xFD);
                0x69u32.encode(sink);
            }
            Instruction::F32x4Nearest => {
                sink.push(0xFD);
                0x6Au32.encode(sink);
            }
            Instruction::F32x4Abs => {
                sink.push(0xFD);
                0xE0u32.encode(sink);
            }
            Instruction::F32x4Neg => {
                sink.push(0xFD);
                0xE1u32.encode(sink);
            }
            Instruction::F32x4Sqrt => {
                sink.push(0xFD);
                0xE3u32.encode(sink);
            }
            Instruction::F32x4Add => {
                sink.push(0xFD);
                0xE4u32.encode(sink);
            }
            Instruction::F32x4Sub => {
                sink.push(0xFD);
                0xE5u32.encode(sink);
            }
            Instruction::F32x4Mul => {
                sink.push(0xFD);
                0xE6u32.encode(sink);
            }
            Instruction::F32x4Div => {
                sink.push(0xFD);
                0xE7u32.encode(sink);
            }
            Instruction::F32x4Min => {
                sink.push(0xFD);
                0xE8u32.encode(sink);
            }
            Instruction::F32x4Max => {
                sink.push(0xFD);
                0xE9u32.encode(sink);
            }
            Instruction::F32x4PMin => {
                sink.push(0xFD);
                0xEAu32.encode(sink);
            }
            Instruction::F32x4PMax => {
                sink.push(0xFD);
                0xEBu32.encode(sink);
            }
            Instruction::F64x2Ceil => {
                sink.push(0xFD);
                0x74u32.encode(sink);
            }
            Instruction::F64x2Floor => {
                sink.push(0xFD);
                0x75u32.encode(sink);
            }
            Instruction::F64x2Trunc => {
                sink.push(0xFD);
                0x7Au32.encode(sink);
            }
            Instruction::F64x2Nearest => {
                sink.push(0xFD);
                0x94u32.encode(sink);
            }
            Instruction::F64x2Abs => {
                sink.push(0xFD);
                0xECu32.encode(sink);
            }
            Instruction::F64x2Neg => {
                sink.push(0xFD);
                0xEDu32.encode(sink);
            }
            Instruction::F64x2Sqrt => {
                sink.push(0xFD);
                0xEFu32.encode(sink);
            }
            Instruction::F64x2Add => {
                sink.push(0xFD);
                0xF0u32.encode(sink);
            }
            Instruction::F64x2Sub => {
                sink.push(0xFD);
                0xF1u32.encode(sink);
            }
            Instruction::F64x2Mul => {
                sink.push(0xFD);
                0xF2u32.encode(sink);
            }
            Instruction::F64x2Div => {
                sink.push(0xFD);
                0xF3u32.encode(sink);
            }
            Instruction::F64x2Min => {
                sink.push(0xFD);
                0xF4u32.encode(sink);
            }
            Instruction::F64x2Max => {
                sink.push(0xFD);
                0xF5u32.encode(sink);
            }
            Instruction::F64x2PMin => {
                sink.push(0xFD);
                0xF6u32.encode(sink);
            }
            Instruction::F64x2PMax => {
                sink.push(0xFD);
                0xF7u32.encode(sink);
            }
            Instruction::I32x4TruncSatF32x4S => {
                sink.push(0xFD);
                0xF8u32.encode(sink);
            }
            Instruction::I32x4TruncSatF32x4U => {
                sink.push(0xFD);
                0xF9u32.encode(sink);
            }
            Instruction::F32x4ConvertI32x4S => {
                sink.push(0xFD);
                0xFAu32.encode(sink);
            }
            Instruction::F32x4ConvertI32x4U => {
                sink.push(0xFD);
                0xFBu32.encode(sink);
            }
            Instruction::I32x4TruncSatF64x2SZero => {
                sink.push(0xFD);
                0xFCu32.encode(sink);
            }
            Instruction::I32x4TruncSatF64x2UZero => {
                sink.push(0xFD);
                0xFDu32.encode(sink);
            }
            Instruction::F64x2ConvertLowI32x4S => {
                sink.push(0xFD);
                0xFEu32.encode(sink);
            }
            Instruction::F64x2ConvertLowI32x4U => {
                sink.push(0xFD);
                0xFFu32.encode(sink);
            }
            Instruction::F32x4DemoteF64x2Zero => {
                sink.push(0xFD);
                0x5Eu32.encode(sink);
            }
            Instruction::F64x2PromoteLowF32x4 => {
                sink.push(0xFD);
                0x5Fu32.encode(sink);
            }
            Instruction::V128Load32Zero(memarg) => {
                sink.push(0xFD);
                0x5Cu32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load64Zero(memarg) => {
                sink.push(0xFD);
                0x5Du32.encode(sink);
                memarg.encode(sink);
            }
            Instruction::V128Load8Lane { memarg, lane } => {
                sink.push(0xFD);
                0x54u32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 16);
                sink.push(lane);
            }
            Instruction::V128Load16Lane { memarg, lane } => {
                sink.push(0xFD);
                0x55u32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 8);
                sink.push(lane);
            }
            Instruction::V128Load32Lane { memarg, lane } => {
                sink.push(0xFD);
                0x56u32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 4);
                sink.push(lane);
            }
            Instruction::V128Load64Lane { memarg, lane } => {
                sink.push(0xFD);
                0x57u32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 2);
                sink.push(lane);
            }
            Instruction::V128Store8Lane { memarg, lane } => {
                sink.push(0xFD);
                0x58u32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 16);
                sink.push(lane);
            }
            Instruction::V128Store16Lane { memarg, lane } => {
                sink.push(0xFD);
                0x59u32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 8);
                sink.push(lane);
            }
            Instruction::V128Store32Lane { memarg, lane } => {
                sink.push(0xFD);
                0x5Au32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 4);
                sink.push(lane);
            }
            Instruction::V128Store64Lane { memarg, lane } => {
                sink.push(0xFD);
                0x5Bu32.encode(sink);
                memarg.encode(sink);
                assert!(lane < 2);
                sink.push(lane);
            }
            Instruction::I64x2Eq => {
                sink.push(0xFD);
                0xD6u32.encode(sink);
            }
            Instruction::I64x2Ne => {
                sink.push(0xFD);
                0xD7u32.encode(sink);
            }
            Instruction::I64x2LtS => {
                sink.push(0xFD);
                0xD8u32.encode(sink);
            }
            Instruction::I64x2GtS => {
                sink.push(0xFD);
                0xD9u32.encode(sink);
            }
            Instruction::I64x2LeS => {
                sink.push(0xFD);
                0xDAu32.encode(sink);
            }
            Instruction::I64x2GeS => {
                sink.push(0xFD);
                0xDBu32.encode(sink);
            }
            Instruction::I8x16RelaxedSwizzle => {
                sink.push(0xFD);
                0x100u32.encode(sink);
            }
            Instruction::I32x4RelaxedTruncF32x4S => {
                sink.push(0xFD);
                0x101u32.encode(sink);
            }
            Instruction::I32x4RelaxedTruncF32x4U => {
                sink.push(0xFD);
                0x102u32.encode(sink);
            }
            Instruction::I32x4RelaxedTruncF64x2SZero => {
                sink.push(0xFD);
                0x103u32.encode(sink);
            }
            Instruction::I32x4RelaxedTruncF64x2UZero => {
                sink.push(0xFD);
                0x104u32.encode(sink);
            }
            Instruction::F32x4RelaxedMadd => {
                sink.push(0xFD);
                0x105u32.encode(sink);
            }
            Instruction::F32x4RelaxedNmadd => {
                sink.push(0xFD);
                0x106u32.encode(sink);
            }
            Instruction::F64x2RelaxedMadd => {
                sink.push(0xFD);
                0x107u32.encode(sink);
            }
            Instruction::F64x2RelaxedNmadd => {
                sink.push(0xFD);
                0x108u32.encode(sink);
            }
            Instruction::I8x16RelaxedLaneselect => {
                sink.push(0xFD);
                0x109u32.encode(sink);
            }
            Instruction::I16x8RelaxedLaneselect => {
                sink.push(0xFD);
                0x10Au32.encode(sink);
            }
            Instruction::I32x4RelaxedLaneselect => {
                sink.push(0xFD);
                0x10Bu32.encode(sink);
            }
            Instruction::I64x2RelaxedLaneselect => {
                sink.push(0xFD);
                0x10Cu32.encode(sink);
            }
            Instruction::F32x4RelaxedMin => {
                sink.push(0xFD);
                0x10Du32.encode(sink);
            }
            Instruction::F32x4RelaxedMax => {
                sink.push(0xFD);
                0x10Eu32.encode(sink);
            }
            Instruction::F64x2RelaxedMin => {
                sink.push(0xFD);
                0x10Fu32.encode(sink);
            }
            Instruction::F64x2RelaxedMax => {
                sink.push(0xFD);
                0x110u32.encode(sink);
            }
            Instruction::I16x8RelaxedQ15mulrS => {
                sink.push(0xFD);
                0x111u32.encode(sink);
            }
            Instruction::I16x8RelaxedDotI8x16I7x16S => {
                sink.push(0xFD);
                0x112u32.encode(sink);
            }
            Instruction::I32x4RelaxedDotI8x16I7x16AddS => {
                sink.push(0xFD);
                0x113u32.encode(sink);
            }

            // Atmoic instructions from the thread proposal
            Instruction::MemoryAtomicNotify(memarg) => {
                sink.push(0xFE);
                sink.push(0x00);
                memarg.encode(sink);
            }
            Instruction::MemoryAtomicWait32(memarg) => {
                sink.push(0xFE);
                sink.push(0x01);
                memarg.encode(sink);
            }
            Instruction::MemoryAtomicWait64(memarg) => {
                sink.push(0xFE);
                sink.push(0x02);
                memarg.encode(sink);
            }
            Instruction::AtomicFence => {
                sink.push(0xFE);
                sink.push(0x03);
                sink.push(0x00);
            }
            Instruction::I32AtomicLoad(memarg) => {
                sink.push(0xFE);
                sink.push(0x10);
                memarg.encode(sink);
            }
            Instruction::I64AtomicLoad(memarg) => {
                sink.push(0xFE);
                sink.push(0x11);
                memarg.encode(sink);
            }
            Instruction::I32AtomicLoad8U(memarg) => {
                sink.push(0xFE);
                sink.push(0x12);
                memarg.encode(sink);
            }
            Instruction::I32AtomicLoad16U(memarg) => {
                sink.push(0xFE);
                sink.push(0x13);
                memarg.encode(sink);
            }
            Instruction::I64AtomicLoad8U(memarg) => {
                sink.push(0xFE);
                sink.push(0x14);
                memarg.encode(sink);
            }
            Instruction::I64AtomicLoad16U(memarg) => {
                sink.push(0xFE);
                sink.push(0x15);
                memarg.encode(sink);
            }
            Instruction::I64AtomicLoad32U(memarg) => {
                sink.push(0xFE);
                sink.push(0x16);
                memarg.encode(sink);
            }
            Instruction::I32AtomicStore(memarg) => {
                sink.push(0xFE);
                sink.push(0x17);
                memarg.encode(sink);
            }
            Instruction::I64AtomicStore(memarg) => {
                sink.push(0xFE);
                sink.push(0x18);
                memarg.encode(sink);
            }
            Instruction::I32AtomicStore8(memarg) => {
                sink.push(0xFE);
                sink.push(0x19);
                memarg.encode(sink);
            }
            Instruction::I32AtomicStore16(memarg) => {
                sink.push(0xFE);
                sink.push(0x1A);
                memarg.encode(sink);
            }
            Instruction::I64AtomicStore8(memarg) => {
                sink.push(0xFE);
                sink.push(0x1B);
                memarg.encode(sink);
            }
            Instruction::I64AtomicStore16(memarg) => {
                sink.push(0xFE);
                sink.push(0x1C);
                memarg.encode(sink);
            }
            Instruction::I64AtomicStore32(memarg) => {
                sink.push(0xFE);
                sink.push(0x1D);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmwAdd(memarg) => {
                sink.push(0xFE);
                sink.push(0x1E);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmwAdd(memarg) => {
                sink.push(0xFE);
                sink.push(0x1F);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw8AddU(memarg) => {
                sink.push(0xFE);
                sink.push(0x20);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw16AddU(memarg) => {
                sink.push(0xFE);
                sink.push(0x21);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw8AddU(memarg) => {
                sink.push(0xFE);
                sink.push(0x22);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw16AddU(memarg) => {
                sink.push(0xFE);
                sink.push(0x23);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw32AddU(memarg) => {
                sink.push(0xFE);
                sink.push(0x24);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmwSub(memarg) => {
                sink.push(0xFE);
                sink.push(0x25);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmwSub(memarg) => {
                sink.push(0xFE);
                sink.push(0x26);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw8SubU(memarg) => {
                sink.push(0xFE);
                sink.push(0x27);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw16SubU(memarg) => {
                sink.push(0xFE);
                sink.push(0x28);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw8SubU(memarg) => {
                sink.push(0xFE);
                sink.push(0x29);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw16SubU(memarg) => {
                sink.push(0xFE);
                sink.push(0x2A);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw32SubU(memarg) => {
                sink.push(0xFE);
                sink.push(0x2B);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmwAnd(memarg) => {
                sink.push(0xFE);
                sink.push(0x2C);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmwAnd(memarg) => {
                sink.push(0xFE);
                sink.push(0x2D);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw8AndU(memarg) => {
                sink.push(0xFE);
                sink.push(0x2E);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw16AndU(memarg) => {
                sink.push(0xFE);
                sink.push(0x2F);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw8AndU(memarg) => {
                sink.push(0xFE);
                sink.push(0x30);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw16AndU(memarg) => {
                sink.push(0xFE);
                sink.push(0x31);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw32AndU(memarg) => {
                sink.push(0xFE);
                sink.push(0x32);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmwOr(memarg) => {
                sink.push(0xFE);
                sink.push(0x33);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmwOr(memarg) => {
                sink.push(0xFE);
                sink.push(0x34);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw8OrU(memarg) => {
                sink.push(0xFE);
                sink.push(0x35);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw16OrU(memarg) => {
                sink.push(0xFE);
                sink.push(0x36);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw8OrU(memarg) => {
                sink.push(0xFE);
                sink.push(0x37);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw16OrU(memarg) => {
                sink.push(0xFE);
                sink.push(0x38);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw32OrU(memarg) => {
                sink.push(0xFE);
                sink.push(0x39);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmwXor(memarg) => {
                sink.push(0xFE);
                sink.push(0x3A);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmwXor(memarg) => {
                sink.push(0xFE);
                sink.push(0x3B);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw8XorU(memarg) => {
                sink.push(0xFE);
                sink.push(0x3C);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw16XorU(memarg) => {
                sink.push(0xFE);
                sink.push(0x3D);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw8XorU(memarg) => {
                sink.push(0xFE);
                sink.push(0x3E);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw16XorU(memarg) => {
                sink.push(0xFE);
                sink.push(0x3F);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw32XorU(memarg) => {
                sink.push(0xFE);
                sink.push(0x40);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmwXchg(memarg) => {
                sink.push(0xFE);
                sink.push(0x41);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmwXchg(memarg) => {
                sink.push(0xFE);
                sink.push(0x42);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw8XchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x43);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw16XchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x44);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw8XchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x45);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw16XchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x46);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw32XchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x47);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmwCmpxchg(memarg) => {
                sink.push(0xFE);
                sink.push(0x48);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmwCmpxchg(memarg) => {
                sink.push(0xFE);
                sink.push(0x49);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw8CmpxchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x4A);
                memarg.encode(sink);
            }
            Instruction::I32AtomicRmw16CmpxchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x4B);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw8CmpxchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x4C);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw16CmpxchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x4D);
                memarg.encode(sink);
            }
            Instruction::I64AtomicRmw32CmpxchgU(memarg) => {
                sink.push(0xFE);
                sink.push(0x4E);
                memarg.encode(sink);
            }
        }
    }
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Catch {
    One { tag: u32, label: u32 },
    OneRef { tag: u32, label: u32 },
    All { label: u32 },
    AllRef { label: u32 },
}

impl Encode for Catch {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Catch::One { tag, label } => {
                sink.push(0x00);
                tag.encode(sink);
                label.encode(sink);
            }
            Catch::OneRef { tag, label } => {
                sink.push(0x01);
                tag.encode(sink);
                label.encode(sink);
            }
            Catch::All { label } => {
                sink.push(0x02);
                label.encode(sink);
            }
            Catch::AllRef { label } => {
                sink.push(0x03);
                label.encode(sink);
            }
        }
    }
}

/// A constant expression.
///
/// Usable in contexts such as offsets or initializers.
#[derive(Debug)]
pub struct ConstExpr {
    bytes: Vec<u8>,
}

impl ConstExpr {
    /// Create a new empty constant expression builder.
    pub fn empty() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Create a constant expression with the specified raw encoding of instructions.
    pub fn raw(bytes: impl IntoIterator<Item = u8>) -> Self {
        Self {
            bytes: bytes.into_iter().collect(),
        }
    }

    fn new_insn(insn: Instruction) -> Self {
        let mut bytes = vec![];
        insn.encode(&mut bytes);
        Self { bytes }
    }

    fn with_insn(mut self, insn: Instruction) -> Self {
        insn.encode(&mut self.bytes);
        self
    }

    /// Create a constant expression containing a single `global.get` instruction.
    pub fn global_get(index: u32) -> Self {
        Self::new_insn(Instruction::GlobalGet(index))
    }

    /// Create a constant expression containing a single `ref.null` instruction.
    pub fn ref_null(ty: HeapType) -> Self {
        Self::new_insn(Instruction::RefNull(ty))
    }

    /// Create a constant expression containing a single `ref.func` instruction.
    pub fn ref_func(func: u32) -> Self {
        Self::new_insn(Instruction::RefFunc(func))
    }

    /// Create a constant expression containing a single `i32.const` instruction.
    pub fn i32_const(value: i32) -> Self {
        Self::new_insn(Instruction::I32Const(value))
    }

    /// Create a constant expression containing a single `i64.const` instruction.
    pub fn i64_const(value: i64) -> Self {
        Self::new_insn(Instruction::I64Const(value))
    }

    /// Create a constant expression containing a single `f32.const` instruction.
    pub fn f32_const(value: f32) -> Self {
        Self::new_insn(Instruction::F32Const(value))
    }

    /// Create a constant expression containing a single `f64.const` instruction.
    pub fn f64_const(value: f64) -> Self {
        Self::new_insn(Instruction::F64Const(value))
    }

    /// Create a constant expression containing a single `v128.const` instruction.
    pub fn v128_const(value: i128) -> Self {
        Self::new_insn(Instruction::V128Const(value))
    }

    /// Add a `global.get` instruction to this constant expression.
    pub fn with_global_get(self, index: u32) -> Self {
        self.with_insn(Instruction::GlobalGet(index))
    }

    /// Add a `ref.null` instruction to this constant expression.
    pub fn with_ref_null(self, ty: HeapType) -> Self {
        self.with_insn(Instruction::RefNull(ty))
    }

    /// Add a `ref.func` instruction to this constant expression.
    pub fn with_ref_func(self, func: u32) -> Self {
        self.with_insn(Instruction::RefFunc(func))
    }

    /// Add an `i32.const` instruction to this constant expression.
    pub fn with_i32_const(self, value: i32) -> Self {
        self.with_insn(Instruction::I32Const(value))
    }

    /// Add an `i64.const` instruction to this constant expression.
    pub fn with_i64_const(self, value: i64) -> Self {
        self.with_insn(Instruction::I64Const(value))
    }

    /// Add a `f32.const` instruction to this constant expression.
    pub fn with_f32_const(self, value: f32) -> Self {
        self.with_insn(Instruction::F32Const(value))
    }

    /// Add a `f64.const` instruction to this constant expression.
    pub fn with_f64_const(self, value: f64) -> Self {
        self.with_insn(Instruction::F64Const(value))
    }

    /// Add a `v128.const` instruction to this constant expression.
    pub fn with_v128_const(self, value: i128) -> Self {
        self.with_insn(Instruction::V128Const(value))
    }

    /// Add an `i32.add` instruction to this constant expression.
    pub fn with_i32_add(self) -> Self {
        self.with_insn(Instruction::I32Add)
    }

    /// Add an `i32.sub` instruction to this constant expression.
    pub fn with_i32_sub(self) -> Self {
        self.with_insn(Instruction::I32Sub)
    }

    /// Add an `i32.mul` instruction to this constant expression.
    pub fn with_i32_mul(self) -> Self {
        self.with_insn(Instruction::I32Mul)
    }

    /// Add an `i64.add` instruction to this constant expression.
    pub fn with_i64_add(self) -> Self {
        self.with_insn(Instruction::I64Add)
    }

    /// Add an `i64.sub` instruction to this constant expression.
    pub fn with_i64_sub(self) -> Self {
        self.with_insn(Instruction::I64Sub)
    }

    /// Add an `i64.mul` instruction to this constant expression.
    pub fn with_i64_mul(self) -> Self {
        self.with_insn(Instruction::I64Mul)
    }

    /// Returns the function, if any, referenced by this global.
    pub fn get_ref_func(&self) -> Option<u32> {
        let prefix = *self.bytes.get(0)?;
        // 0xd2 == `ref.func` opcode, and if that's found then load the leb
        // corresponding to the function index.
        if prefix != 0xd2 {
            return None;
        }
        leb128::read::unsigned(&mut &self.bytes[1..])
            .ok()?
            .try_into()
            .ok()
    }
}

impl Encode for ConstExpr {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.extend(&self.bytes);
        Instruction::End.encode(sink);
    }
}

/// An error when converting a `wasmparser::ConstExpr` into a
/// `wasm_encoder::ConstExpr`.
#[cfg(feature = "wasmparser")]
#[derive(Debug)]
pub enum ConstExprConversionError {
    /// There was an error when parsing the const expression.
    ParseError(wasmparser::BinaryReaderError),

    /// The const expression is invalid: not actually constant or something like
    /// that.
    Invalid,

    /// There was a type reference that was canonicalized and no longer
    /// references an index into a module's types space, so we cannot encode it
    /// into a Wasm binary again.
    CanonicalizedTypeReference,
}

#[cfg(feature = "wasmparser")]
impl std::fmt::Display for ConstExprConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(_e) => {
                write!(f, "There was an error when parsing the const expression")
            }
            Self::Invalid => write!(f, "The const expression was invalid"),
            Self::CanonicalizedTypeReference => write!(
                f,
                "There was a canonicalized type reference without type index information"
            ),
        }
    }
}

#[cfg(feature = "wasmparser")]
impl std::error::Error for ConstExprConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseError(e) => Some(e),
            Self::Invalid | Self::CanonicalizedTypeReference => None,
        }
    }
}

#[cfg(feature = "wasmparser")]
impl<'a> TryFrom<wasmparser::ConstExpr<'a>> for ConstExpr {
    type Error = ConstExprConversionError;

    fn try_from(const_expr: wasmparser::ConstExpr) -> Result<Self, Self::Error> {
        let mut ops = const_expr.get_operators_reader().into_iter();

        let result = match ops.next() {
            Some(Ok(wasmparser::Operator::I32Const { value })) => ConstExpr::i32_const(value),
            Some(Ok(wasmparser::Operator::I64Const { value })) => ConstExpr::i64_const(value),
            Some(Ok(wasmparser::Operator::F32Const { value })) => {
                ConstExpr::f32_const(value.bits() as _)
            }
            Some(Ok(wasmparser::Operator::F64Const { value })) => {
                ConstExpr::f64_const(value.bits() as _)
            }
            Some(Ok(wasmparser::Operator::V128Const { value })) => {
                ConstExpr::v128_const(i128::from_le_bytes(*value.bytes()))
            }
            Some(Ok(wasmparser::Operator::RefNull { hty })) => ConstExpr::ref_null(
                HeapType::try_from(hty)
                    .map_err(|_| ConstExprConversionError::CanonicalizedTypeReference)?,
            ),
            Some(Ok(wasmparser::Operator::RefFunc { function_index })) => {
                ConstExpr::ref_func(function_index)
            }
            Some(Ok(wasmparser::Operator::GlobalGet { global_index })) => {
                ConstExpr::global_get(global_index)
            }

            // TODO: support the extended-const proposal.
            Some(Ok(_op)) => return Err(ConstExprConversionError::Invalid),

            Some(Err(e)) => return Err(ConstExprConversionError::ParseError(e)),
            None => return Err(ConstExprConversionError::Invalid),
        };

        match (ops.next(), ops.next()) {
            (Some(Ok(wasmparser::Operator::End)), None) => Ok(result),
            _ => Err(ConstExprConversionError::Invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn function_new_with_locals_test() {
        use super::*;

        // Test the algorithm for conversion is correct
        let f1 = Function::new_with_locals_types([
            ValType::I32,
            ValType::I32,
            ValType::I64,
            ValType::F32,
            ValType::F32,
            ValType::F32,
            ValType::I32,
            ValType::I64,
            ValType::I64,
        ]);
        let f2 = Function::new([
            (2, ValType::I32),
            (1, ValType::I64),
            (3, ValType::F32),
            (1, ValType::I32),
            (2, ValType::I64),
        ]);

        assert_eq!(f1.bytes, f2.bytes)
    }
}
