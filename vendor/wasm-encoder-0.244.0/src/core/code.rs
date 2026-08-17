use crate::{
    Encode, HeapType, InstructionSink, RefType, Section, SectionId, ValType, encode_section,
};
use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;

/// An encoder for the code section.
///
/// Code sections are only supported for modules.
///
/// # Example
///
/// ```
/// use wasm_encoder::{
///     CodeSection, Function, FunctionSection, Module,
///     TypeSection, ValType
/// };
///
/// let mut types = TypeSection::new();
/// types.ty().function(vec![], vec![ValType::I32]);
///
/// let mut functions = FunctionSection::new();
/// let type_index = 0;
/// functions.function(type_index);
///
/// let locals = vec![];
/// let mut func = Function::new(locals);
/// func.instructions().i32_const(42);
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
    /// # use wasmparser::{BinaryReader, CodeSectionReader};
    /// //                  id, size, # entries, entry
    /// let code_section = [10, 6,    1,         4, 0, 65, 0, 11];
    ///
    /// // Parse the code section.
    /// let reader = BinaryReader::new(&code_section, 0);
    /// let reader = CodeSectionReader::new(reader).unwrap();
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
/// use wasm_encoder::{CodeSection, Function};
///
/// // Define the function body for:
/// //
/// //     (func (param i32 i32) (result i32)
/// //       local.get 0
/// //       local.get 1
/// //       i32.add)
/// let locals = vec![];
/// let mut func = Function::new(locals);
/// func.instructions()
///     .local_get(0)
///     .local_get(1)
///     .i32_add();
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

    /// Get an instruction encoder for this function body.
    pub fn instructions(&mut self) -> InstructionSink<'_> {
        InstructionSink::new(&mut self.bytes)
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

    /// Unwraps and returns the raw byte encoding of this function.
    ///
    /// This encoding doesn't include the variable-width size field
    /// that `encode` will write before the added bytes. As such, its
    /// length will match the return value of [`Function::byte_len`].
    ///
    /// # Use Case
    ///
    /// This raw byte form is suitable for later using with
    /// [`CodeSection::raw`]. Note that it *differs* from what results
    /// from [`Function::encode`] precisely due to the *lack* of the
    /// length prefix; [`CodeSection::raw`] will use this. Using
    /// [`Function::encode`] instead produces bytes that cannot be fed
    /// into other wasm-encoder types without stripping off the length
    /// prefix, which is awkward and error-prone.
    ///
    /// This method combined with [`CodeSection::raw`] may be useful
    /// together if one wants to save the result of function encoding
    /// and use it later: for example, caching the result of some code
    /// generation process.
    ///
    /// For example:
    ///
    /// ```
    /// # use wasm_encoder::{CodeSection, Function};
    /// let mut f = Function::new([]);
    /// f.instructions().end();
    /// let bytes = f.into_raw_body();
    /// // (save `bytes` somewhere for later use)
    /// let mut code = CodeSection::new();
    /// code.raw(&bytes[..]);
    ///
    /// assert_eq!(2, bytes.len());  // Locals count, then `end`
    /// assert_eq!(3, code.byte_len()); // Function length byte, function body
    /// ```
    pub fn into_raw_body(self) -> Vec<u8> {
        self.bytes
    }
}

impl Encode for Function {
    fn encode(&self, sink: &mut Vec<u8>) {
        self.bytes.encode(sink);
    }
}

/// An IEEE binary32 immediate floating point value, represented as a u32
/// containing the bit pattern.
///
/// All bit patterns are allowed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ieee32(pub(crate) u32);

impl Ieee32 {
    /// Creates a new Ieee32
    pub const fn new(bits: u32) -> Self {
        Ieee32(bits)
    }

    /// Gets the underlying bits of the 32-bit float.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl From<f32> for Ieee32 {
    fn from(value: f32) -> Self {
        Ieee32(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl From<Ieee32> for f32 {
    fn from(bits: Ieee32) -> f32 {
        f32::from_bits(bits.bits())
    }
}

impl Encode for Ieee32 {
    fn encode(&self, sink: &mut Vec<u8>) {
        let bits = self.bits();
        sink.extend(bits.to_le_bytes())
    }
}

/// An IEEE binary64 immediate floating point value, represented as a u64
/// containing the bit pattern.
///
/// All bit patterns are allowed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ieee64(pub(crate) u64);

impl Ieee64 {
    /// Creates a new Ieee64
    pub const fn new(bits: u64) -> Self {
        Ieee64(bits)
    }

    /// Gets the underlying bits of the 64-bit float.
    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl From<f64> for Ieee64 {
    fn from(value: f64) -> Self {
        Ieee64(u64::from_le_bytes(value.to_le_bytes()))
    }
}

impl From<Ieee64> for f64 {
    fn from(bits: Ieee64) -> f64 {
        f64::from_bits(bits.bits())
    }
}

impl Encode for Ieee64 {
    fn encode(&self, sink: &mut Vec<u8>) {
        let bits = self.bits();
        sink.extend(bits.to_le_bytes())
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

/// The memory ordering for atomic instructions.
///
/// For an in-depth explanation of memory orderings, see the C++ documentation
/// for [`memory_order`] or the Rust documentation for [`atomic::Ordering`].
///
/// [`memory_order`]: https://en.cppreference.com/w/cpp/atomic/memory_order
/// [`atomic::Ordering`]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
#[derive(Clone, Copy, Debug)]
pub enum Ordering {
    /// For a load, it acquires; this orders all operations before the last
    /// "releasing" store. For a store, it releases; this orders all operations
    /// before it at the next "acquiring" load.
    AcqRel,
    /// Like `AcqRel` but all threads see all sequentially consistent operations
    /// in the same order.
    SeqCst,
}

impl Encode for Ordering {
    fn encode(&self, sink: &mut Vec<u8>) {
        let flag: u8 = match self {
            Ordering::SeqCst => 0,
            Ordering::AcqRel => 1,
        };
        sink.push(flag);
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
        type_index: u32,
        table_index: u32,
    },
    ReturnCallRef(u32),
    ReturnCall(u32),
    ReturnCallIndirect {
        type_index: u32,
        table_index: u32,
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
    F32Const(Ieee32),
    F64Const(Ieee64),
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
    TypedSelectMulti(Cow<'a, [ValType]>),
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
    StructNewDesc(u32),
    StructNewDefaultDesc(u32),

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

    // More atomic instructions (the shared-everything-threads proposal)
    GlobalAtomicGet {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicSet {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwAdd {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwSub {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwAnd {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwOr {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwXor {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwXchg {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwCmpxchg {
        ordering: Ordering,
        global_index: u32,
    },
    TableAtomicGet {
        ordering: Ordering,
        table_index: u32,
    },
    TableAtomicSet {
        ordering: Ordering,
        table_index: u32,
    },
    TableAtomicRmwXchg {
        ordering: Ordering,
        table_index: u32,
    },
    TableAtomicRmwCmpxchg {
        ordering: Ordering,
        table_index: u32,
    },
    StructAtomicGet {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicGetS {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicGetU {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicSet {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwAdd {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwSub {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwAnd {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwOr {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwXor {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwXchg {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwCmpxchg {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    ArrayAtomicGet {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicGetS {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicGetU {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicSet {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwAdd {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwSub {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwAnd {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwOr {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwXor {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwXchg {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwCmpxchg {
        ordering: Ordering,
        array_type_index: u32,
    },
    RefI31Shared,
    // Stack switching
    ContNew(u32),
    ContBind {
        argument_index: u32,
        result_index: u32,
    },
    Suspend(u32),
    Resume {
        cont_type_index: u32,
        resume_table: Cow<'a, [Handle]>,
    },
    ResumeThrow {
        cont_type_index: u32,
        tag_index: u32,
        resume_table: Cow<'a, [Handle]>,
    },
    Switch {
        cont_type_index: u32,
        tag_index: u32,
    },

    // Wide Arithmetic
    I64Add128,
    I64Sub128,
    I64MulWideS,
    I64MulWideU,

    RefGetDesc(u32),
    RefCastDescNonNull(HeapType),
    RefCastDescNullable(HeapType),
    BrOnCastDesc {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
    BrOnCastDescFail {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
}

impl Encode for Instruction<'_> {
    fn encode(&self, bytes: &mut Vec<u8>) {
        let mut sink = InstructionSink::new(bytes);
        match *self {
            // Control instructions.
            Instruction::Unreachable => sink.unreachable(),
            Instruction::Nop => sink.nop(),
            Instruction::Block(bt) => sink.block(bt),
            Instruction::Loop(bt) => sink.loop_(bt),
            Instruction::If(bt) => sink.if_(bt),
            Instruction::Else => sink.else_(),
            Instruction::Try(bt) => sink.try_(bt),
            Instruction::Catch(t) => sink.catch(t),
            Instruction::Throw(t) => sink.throw(t),
            Instruction::Rethrow(l) => sink.rethrow(l),
            Instruction::ThrowRef => sink.throw_ref(),
            Instruction::End => sink.end(),
            Instruction::Br(l) => sink.br(l),
            Instruction::BrIf(l) => sink.br_if(l),
            Instruction::BrTable(ref ls, l) => sink.br_table(ls.iter().copied(), l),
            Instruction::BrOnNull(l) => sink.br_on_null(l),
            Instruction::BrOnNonNull(l) => sink.br_on_non_null(l),
            Instruction::Return => sink.return_(),
            Instruction::Call(f) => sink.call(f),
            Instruction::CallRef(ty) => sink.call_ref(ty),
            Instruction::CallIndirect {
                type_index,
                table_index,
            } => sink.call_indirect(table_index, type_index),
            Instruction::ReturnCallRef(ty) => sink.return_call_ref(ty),

            Instruction::ReturnCall(f) => sink.return_call(f),
            Instruction::ReturnCallIndirect {
                type_index,
                table_index,
            } => sink.return_call_indirect(table_index, type_index),
            Instruction::Delegate(l) => sink.delegate(l),
            Instruction::CatchAll => sink.catch_all(),

            // Parametric instructions.
            Instruction::Drop => sink.drop(),
            Instruction::Select => sink.select(),
            Instruction::TypedSelect(ty) => sink.typed_select(ty),
            Instruction::TypedSelectMulti(ref tys) => sink.typed_select_multi(tys.as_ref()),

            Instruction::TryTable(ty, ref catches) => sink.try_table(ty, catches.iter().cloned()),

            // Variable instructions.
            Instruction::LocalGet(l) => sink.local_get(l),
            Instruction::LocalSet(l) => sink.local_set(l),
            Instruction::LocalTee(l) => sink.local_tee(l),
            Instruction::GlobalGet(g) => sink.global_get(g),
            Instruction::GlobalSet(g) => sink.global_set(g),
            Instruction::TableGet(table) => sink.table_get(table),
            Instruction::TableSet(table) => sink.table_set(table),

            // Memory instructions.
            Instruction::I32Load(m) => sink.i32_load(m),
            Instruction::I64Load(m) => sink.i64_load(m),
            Instruction::F32Load(m) => sink.f32_load(m),
            Instruction::F64Load(m) => sink.f64_load(m),
            Instruction::I32Load8S(m) => sink.i32_load8_s(m),
            Instruction::I32Load8U(m) => sink.i32_load8_u(m),
            Instruction::I32Load16S(m) => sink.i32_load16_s(m),
            Instruction::I32Load16U(m) => sink.i32_load16_u(m),
            Instruction::I64Load8S(m) => sink.i64_load8_s(m),
            Instruction::I64Load8U(m) => sink.i64_load8_u(m),
            Instruction::I64Load16S(m) => sink.i64_load16_s(m),
            Instruction::I64Load16U(m) => sink.i64_load16_u(m),
            Instruction::I64Load32S(m) => sink.i64_load32_s(m),
            Instruction::I64Load32U(m) => sink.i64_load32_u(m),
            Instruction::I32Store(m) => sink.i32_store(m),
            Instruction::I64Store(m) => sink.i64_store(m),
            Instruction::F32Store(m) => sink.f32_store(m),
            Instruction::F64Store(m) => sink.f64_store(m),
            Instruction::I32Store8(m) => sink.i32_store8(m),
            Instruction::I32Store16(m) => sink.i32_store16(m),
            Instruction::I64Store8(m) => sink.i64_store8(m),
            Instruction::I64Store16(m) => sink.i64_store16(m),
            Instruction::I64Store32(m) => sink.i64_store32(m),
            Instruction::MemorySize(i) => sink.memory_size(i),
            Instruction::MemoryGrow(i) => sink.memory_grow(i),
            Instruction::MemoryInit { mem, data_index } => sink.memory_init(mem, data_index),
            Instruction::DataDrop(data) => sink.data_drop(data),
            Instruction::MemoryCopy { src_mem, dst_mem } => sink.memory_copy(dst_mem, src_mem),
            Instruction::MemoryFill(mem) => sink.memory_fill(mem),
            Instruction::MemoryDiscard(mem) => sink.memory_discard(mem),

            // Numeric instructions.
            Instruction::I32Const(x) => sink.i32_const(x),
            Instruction::I64Const(x) => sink.i64_const(x),
            Instruction::F32Const(x) => sink.f32_const(x),
            Instruction::F64Const(x) => sink.f64_const(x),
            Instruction::I32Eqz => sink.i32_eqz(),
            Instruction::I32Eq => sink.i32_eq(),
            Instruction::I32Ne => sink.i32_ne(),
            Instruction::I32LtS => sink.i32_lt_s(),
            Instruction::I32LtU => sink.i32_lt_u(),
            Instruction::I32GtS => sink.i32_gt_s(),
            Instruction::I32GtU => sink.i32_gt_u(),
            Instruction::I32LeS => sink.i32_le_s(),
            Instruction::I32LeU => sink.i32_le_u(),
            Instruction::I32GeS => sink.i32_ge_s(),
            Instruction::I32GeU => sink.i32_ge_u(),
            Instruction::I64Eqz => sink.i64_eqz(),
            Instruction::I64Eq => sink.i64_eq(),
            Instruction::I64Ne => sink.i64_ne(),
            Instruction::I64LtS => sink.i64_lt_s(),
            Instruction::I64LtU => sink.i64_lt_u(),
            Instruction::I64GtS => sink.i64_gt_s(),
            Instruction::I64GtU => sink.i64_gt_u(),
            Instruction::I64LeS => sink.i64_le_s(),
            Instruction::I64LeU => sink.i64_le_u(),
            Instruction::I64GeS => sink.i64_ge_s(),
            Instruction::I64GeU => sink.i64_ge_u(),
            Instruction::F32Eq => sink.f32_eq(),
            Instruction::F32Ne => sink.f32_ne(),
            Instruction::F32Lt => sink.f32_lt(),
            Instruction::F32Gt => sink.f32_gt(),
            Instruction::F32Le => sink.f32_le(),
            Instruction::F32Ge => sink.f32_ge(),
            Instruction::F64Eq => sink.f64_eq(),
            Instruction::F64Ne => sink.f64_ne(),
            Instruction::F64Lt => sink.f64_lt(),
            Instruction::F64Gt => sink.f64_gt(),
            Instruction::F64Le => sink.f64_le(),
            Instruction::F64Ge => sink.f64_ge(),
            Instruction::I32Clz => sink.i32_clz(),
            Instruction::I32Ctz => sink.i32_ctz(),
            Instruction::I32Popcnt => sink.i32_popcnt(),
            Instruction::I32Add => sink.i32_add(),
            Instruction::I32Sub => sink.i32_sub(),
            Instruction::I32Mul => sink.i32_mul(),
            Instruction::I32DivS => sink.i32_div_s(),
            Instruction::I32DivU => sink.i32_div_u(),
            Instruction::I32RemS => sink.i32_rem_s(),
            Instruction::I32RemU => sink.i32_rem_u(),
            Instruction::I32And => sink.i32_and(),
            Instruction::I32Or => sink.i32_or(),
            Instruction::I32Xor => sink.i32_xor(),
            Instruction::I32Shl => sink.i32_shl(),
            Instruction::I32ShrS => sink.i32_shr_s(),
            Instruction::I32ShrU => sink.i32_shr_u(),
            Instruction::I32Rotl => sink.i32_rotl(),
            Instruction::I32Rotr => sink.i32_rotr(),
            Instruction::I64Clz => sink.i64_clz(),
            Instruction::I64Ctz => sink.i64_ctz(),
            Instruction::I64Popcnt => sink.i64_popcnt(),
            Instruction::I64Add => sink.i64_add(),
            Instruction::I64Sub => sink.i64_sub(),
            Instruction::I64Mul => sink.i64_mul(),
            Instruction::I64DivS => sink.i64_div_s(),
            Instruction::I64DivU => sink.i64_div_u(),
            Instruction::I64RemS => sink.i64_rem_s(),
            Instruction::I64RemU => sink.i64_rem_u(),
            Instruction::I64And => sink.i64_and(),
            Instruction::I64Or => sink.i64_or(),
            Instruction::I64Xor => sink.i64_xor(),
            Instruction::I64Shl => sink.i64_shl(),
            Instruction::I64ShrS => sink.i64_shr_s(),
            Instruction::I64ShrU => sink.i64_shr_u(),
            Instruction::I64Rotl => sink.i64_rotl(),
            Instruction::I64Rotr => sink.i64_rotr(),
            Instruction::F32Abs => sink.f32_abs(),
            Instruction::F32Neg => sink.f32_neg(),
            Instruction::F32Ceil => sink.f32_ceil(),
            Instruction::F32Floor => sink.f32_floor(),
            Instruction::F32Trunc => sink.f32_trunc(),
            Instruction::F32Nearest => sink.f32_nearest(),
            Instruction::F32Sqrt => sink.f32_sqrt(),
            Instruction::F32Add => sink.f32_add(),
            Instruction::F32Sub => sink.f32_sub(),
            Instruction::F32Mul => sink.f32_mul(),
            Instruction::F32Div => sink.f32_div(),
            Instruction::F32Min => sink.f32_min(),
            Instruction::F32Max => sink.f32_max(),
            Instruction::F32Copysign => sink.f32_copysign(),
            Instruction::F64Abs => sink.f64_abs(),
            Instruction::F64Neg => sink.f64_neg(),
            Instruction::F64Ceil => sink.f64_ceil(),
            Instruction::F64Floor => sink.f64_floor(),
            Instruction::F64Trunc => sink.f64_trunc(),
            Instruction::F64Nearest => sink.f64_nearest(),
            Instruction::F64Sqrt => sink.f64_sqrt(),
            Instruction::F64Add => sink.f64_add(),
            Instruction::F64Sub => sink.f64_sub(),
            Instruction::F64Mul => sink.f64_mul(),
            Instruction::F64Div => sink.f64_div(),
            Instruction::F64Min => sink.f64_min(),
            Instruction::F64Max => sink.f64_max(),
            Instruction::F64Copysign => sink.f64_copysign(),
            Instruction::I32WrapI64 => sink.i32_wrap_i64(),
            Instruction::I32TruncF32S => sink.i32_trunc_f32_s(),
            Instruction::I32TruncF32U => sink.i32_trunc_f32_u(),
            Instruction::I32TruncF64S => sink.i32_trunc_f64_s(),
            Instruction::I32TruncF64U => sink.i32_trunc_f64_u(),
            Instruction::I64ExtendI32S => sink.i64_extend_i32_s(),
            Instruction::I64ExtendI32U => sink.i64_extend_i32_u(),
            Instruction::I64TruncF32S => sink.i64_trunc_f32_s(),
            Instruction::I64TruncF32U => sink.i64_trunc_f32_u(),
            Instruction::I64TruncF64S => sink.i64_trunc_f64_s(),
            Instruction::I64TruncF64U => sink.i64_trunc_f64_u(),
            Instruction::F32ConvertI32S => sink.f32_convert_i32_s(),
            Instruction::F32ConvertI32U => sink.f32_convert_i32_u(),
            Instruction::F32ConvertI64S => sink.f32_convert_i64_s(),
            Instruction::F32ConvertI64U => sink.f32_convert_i64_u(),
            Instruction::F32DemoteF64 => sink.f32_demote_f64(),
            Instruction::F64ConvertI32S => sink.f64_convert_i32_s(),
            Instruction::F64ConvertI32U => sink.f64_convert_i32_u(),
            Instruction::F64ConvertI64S => sink.f64_convert_i64_s(),
            Instruction::F64ConvertI64U => sink.f64_convert_i64_u(),
            Instruction::F64PromoteF32 => sink.f64_promote_f32(),
            Instruction::I32ReinterpretF32 => sink.i32_reinterpret_f32(),
            Instruction::I64ReinterpretF64 => sink.i64_reinterpret_f64(),
            Instruction::F32ReinterpretI32 => sink.f32_reinterpret_i32(),
            Instruction::F64ReinterpretI64 => sink.f64_reinterpret_i64(),
            Instruction::I32Extend8S => sink.i32_extend8_s(),
            Instruction::I32Extend16S => sink.i32_extend16_s(),
            Instruction::I64Extend8S => sink.i64_extend8_s(),
            Instruction::I64Extend16S => sink.i64_extend16_s(),
            Instruction::I64Extend32S => sink.i64_extend32_s(),

            Instruction::I32TruncSatF32S => sink.i32_trunc_sat_f32_s(),
            Instruction::I32TruncSatF32U => sink.i32_trunc_sat_f32_u(),
            Instruction::I32TruncSatF64S => sink.i32_trunc_sat_f64_s(),
            Instruction::I32TruncSatF64U => sink.i32_trunc_sat_f64_u(),
            Instruction::I64TruncSatF32S => sink.i64_trunc_sat_f32_s(),
            Instruction::I64TruncSatF32U => sink.i64_trunc_sat_f32_u(),
            Instruction::I64TruncSatF64S => sink.i64_trunc_sat_f64_s(),
            Instruction::I64TruncSatF64U => sink.i64_trunc_sat_f64_u(),

            // Reference types instructions.
            Instruction::RefNull(ty) => sink.ref_null(ty),
            Instruction::RefIsNull => sink.ref_is_null(),
            Instruction::RefFunc(f) => sink.ref_func(f),
            Instruction::RefEq => sink.ref_eq(),
            Instruction::RefAsNonNull => sink.ref_as_non_null(),

            // GC instructions.
            Instruction::StructNew(type_index) => sink.struct_new(type_index),
            Instruction::StructNewDefault(type_index) => sink.struct_new_default(type_index),
            Instruction::StructGet {
                struct_type_index,
                field_index,
            } => sink.struct_get(struct_type_index, field_index),
            Instruction::StructGetS {
                struct_type_index,
                field_index,
            } => sink.struct_get_s(struct_type_index, field_index),
            Instruction::StructGetU {
                struct_type_index,
                field_index,
            } => sink.struct_get_u(struct_type_index, field_index),
            Instruction::StructSet {
                struct_type_index,
                field_index,
            } => sink.struct_set(struct_type_index, field_index),
            Instruction::StructNewDesc(type_index) => sink.struct_new_desc(type_index),
            Instruction::StructNewDefaultDesc(type_index) => {
                sink.struct_new_default_desc(type_index)
            }
            Instruction::ArrayNew(type_index) => sink.array_new(type_index),
            Instruction::ArrayNewDefault(type_index) => sink.array_new_default(type_index),
            Instruction::ArrayNewFixed {
                array_type_index,
                array_size,
            } => sink.array_new_fixed(array_type_index, array_size),
            Instruction::ArrayNewData {
                array_type_index,
                array_data_index,
            } => sink.array_new_data(array_type_index, array_data_index),
            Instruction::ArrayNewElem {
                array_type_index,
                array_elem_index,
            } => sink.array_new_elem(array_type_index, array_elem_index),
            Instruction::ArrayGet(type_index) => sink.array_get(type_index),
            Instruction::ArrayGetS(type_index) => sink.array_get_s(type_index),
            Instruction::ArrayGetU(type_index) => sink.array_get_u(type_index),
            Instruction::ArraySet(type_index) => sink.array_set(type_index),
            Instruction::ArrayLen => sink.array_len(),
            Instruction::ArrayFill(type_index) => sink.array_fill(type_index),
            Instruction::ArrayCopy {
                array_type_index_dst,
                array_type_index_src,
            } => sink.array_copy(array_type_index_dst, array_type_index_src),
            Instruction::ArrayInitData {
                array_type_index,
                array_data_index,
            } => sink.array_init_data(array_type_index, array_data_index),
            Instruction::ArrayInitElem {
                array_type_index,
                array_elem_index,
            } => sink.array_init_elem(array_type_index, array_elem_index),
            Instruction::RefTestNonNull(heap_type) => sink.ref_test_non_null(heap_type),
            Instruction::RefTestNullable(heap_type) => sink.ref_test_nullable(heap_type),
            Instruction::RefCastNonNull(heap_type) => sink.ref_cast_non_null(heap_type),
            Instruction::RefCastNullable(heap_type) => sink.ref_cast_nullable(heap_type),
            Instruction::BrOnCast {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => sink.br_on_cast(relative_depth, from_ref_type, to_ref_type),
            Instruction::BrOnCastFail {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => sink.br_on_cast_fail(relative_depth, from_ref_type, to_ref_type),
            Instruction::AnyConvertExtern => sink.any_convert_extern(),
            Instruction::ExternConvertAny => sink.extern_convert_any(),
            Instruction::RefI31 => sink.ref_i31(),
            Instruction::I31GetS => sink.i31_get_s(),
            Instruction::I31GetU => sink.i31_get_u(),

            // Bulk memory instructions.
            Instruction::TableInit { elem_index, table } => sink.table_init(table, elem_index),
            Instruction::ElemDrop(segment) => sink.elem_drop(segment),
            Instruction::TableCopy {
                src_table,
                dst_table,
            } => sink.table_copy(dst_table, src_table),
            Instruction::TableGrow(table) => sink.table_grow(table),
            Instruction::TableSize(table) => sink.table_size(table),
            Instruction::TableFill(table) => sink.table_fill(table),

            // SIMD instructions.
            Instruction::V128Load(memarg) => sink.v128_load(memarg),
            Instruction::V128Load8x8S(memarg) => sink.v128_load8x8_s(memarg),
            Instruction::V128Load8x8U(memarg) => sink.v128_load8x8_u(memarg),
            Instruction::V128Load16x4S(memarg) => sink.v128_load16x4_s(memarg),
            Instruction::V128Load16x4U(memarg) => sink.v128_load16x4_u(memarg),
            Instruction::V128Load32x2S(memarg) => sink.v128_load32x2_s(memarg),
            Instruction::V128Load32x2U(memarg) => sink.v128_load32x2_u(memarg),
            Instruction::V128Load8Splat(memarg) => sink.v128_load8_splat(memarg),
            Instruction::V128Load16Splat(memarg) => sink.v128_load16_splat(memarg),
            Instruction::V128Load32Splat(memarg) => sink.v128_load32_splat(memarg),
            Instruction::V128Load64Splat(memarg) => sink.v128_load64_splat(memarg),
            Instruction::V128Store(memarg) => sink.v128_store(memarg),
            Instruction::V128Const(x) => sink.v128_const(x),
            Instruction::I8x16Shuffle(lanes) => sink.i8x16_shuffle(lanes),
            Instruction::I8x16Swizzle => sink.i8x16_swizzle(),
            Instruction::I8x16Splat => sink.i8x16_splat(),
            Instruction::I16x8Splat => sink.i16x8_splat(),
            Instruction::I32x4Splat => sink.i32x4_splat(),
            Instruction::I64x2Splat => sink.i64x2_splat(),
            Instruction::F32x4Splat => sink.f32x4_splat(),
            Instruction::F64x2Splat => sink.f64x2_splat(),
            Instruction::I8x16ExtractLaneS(lane) => sink.i8x16_extract_lane_s(lane),
            Instruction::I8x16ExtractLaneU(lane) => sink.i8x16_extract_lane_u(lane),
            Instruction::I8x16ReplaceLane(lane) => sink.i8x16_replace_lane(lane),
            Instruction::I16x8ExtractLaneS(lane) => sink.i16x8_extract_lane_s(lane),
            Instruction::I16x8ExtractLaneU(lane) => sink.i16x8_extract_lane_u(lane),
            Instruction::I16x8ReplaceLane(lane) => sink.i16x8_replace_lane(lane),
            Instruction::I32x4ExtractLane(lane) => sink.i32x4_extract_lane(lane),
            Instruction::I32x4ReplaceLane(lane) => sink.i32x4_replace_lane(lane),
            Instruction::I64x2ExtractLane(lane) => sink.i64x2_extract_lane(lane),
            Instruction::I64x2ReplaceLane(lane) => sink.i64x2_replace_lane(lane),
            Instruction::F32x4ExtractLane(lane) => sink.f32x4_extract_lane(lane),
            Instruction::F32x4ReplaceLane(lane) => sink.f32x4_replace_lane(lane),
            Instruction::F64x2ExtractLane(lane) => sink.f64x2_extract_lane(lane),
            Instruction::F64x2ReplaceLane(lane) => sink.f64x2_replace_lane(lane),

            Instruction::I8x16Eq => sink.i8x16_eq(),
            Instruction::I8x16Ne => sink.i8x16_ne(),
            Instruction::I8x16LtS => sink.i8x16_lt_s(),
            Instruction::I8x16LtU => sink.i8x16_lt_u(),
            Instruction::I8x16GtS => sink.i8x16_gt_s(),
            Instruction::I8x16GtU => sink.i8x16_gt_u(),
            Instruction::I8x16LeS => sink.i8x16_le_s(),
            Instruction::I8x16LeU => sink.i8x16_le_u(),
            Instruction::I8x16GeS => sink.i8x16_ge_s(),
            Instruction::I8x16GeU => sink.i8x16_ge_u(),
            Instruction::I16x8Eq => sink.i16x8_eq(),
            Instruction::I16x8Ne => sink.i16x8_ne(),
            Instruction::I16x8LtS => sink.i16x8_lt_s(),
            Instruction::I16x8LtU => sink.i16x8_lt_u(),
            Instruction::I16x8GtS => sink.i16x8_gt_s(),
            Instruction::I16x8GtU => sink.i16x8_gt_u(),
            Instruction::I16x8LeS => sink.i16x8_le_s(),
            Instruction::I16x8LeU => sink.i16x8_le_u(),
            Instruction::I16x8GeS => sink.i16x8_ge_s(),
            Instruction::I16x8GeU => sink.i16x8_ge_u(),
            Instruction::I32x4Eq => sink.i32x4_eq(),
            Instruction::I32x4Ne => sink.i32x4_ne(),
            Instruction::I32x4LtS => sink.i32x4_lt_s(),
            Instruction::I32x4LtU => sink.i32x4_lt_u(),
            Instruction::I32x4GtS => sink.i32x4_gt_s(),
            Instruction::I32x4GtU => sink.i32x4_gt_u(),
            Instruction::I32x4LeS => sink.i32x4_le_s(),
            Instruction::I32x4LeU => sink.i32x4_le_u(),
            Instruction::I32x4GeS => sink.i32x4_ge_s(),
            Instruction::I32x4GeU => sink.i32x4_ge_u(),
            Instruction::F32x4Eq => sink.f32x4_eq(),
            Instruction::F32x4Ne => sink.f32x4_ne(),
            Instruction::F32x4Lt => sink.f32x4_lt(),
            Instruction::F32x4Gt => sink.f32x4_gt(),
            Instruction::F32x4Le => sink.f32x4_le(),
            Instruction::F32x4Ge => sink.f32x4_ge(),
            Instruction::F64x2Eq => sink.f64x2_eq(),
            Instruction::F64x2Ne => sink.f64x2_ne(),
            Instruction::F64x2Lt => sink.f64x2_lt(),
            Instruction::F64x2Gt => sink.f64x2_gt(),
            Instruction::F64x2Le => sink.f64x2_le(),
            Instruction::F64x2Ge => sink.f64x2_ge(),
            Instruction::V128Not => sink.v128_not(),
            Instruction::V128And => sink.v128_and(),
            Instruction::V128AndNot => sink.v128_andnot(),
            Instruction::V128Or => sink.v128_or(),
            Instruction::V128Xor => sink.v128_xor(),
            Instruction::V128Bitselect => sink.v128_bitselect(),
            Instruction::V128AnyTrue => sink.v128_any_true(),
            Instruction::I8x16Abs => sink.i8x16_abs(),
            Instruction::I8x16Neg => sink.i8x16_neg(),
            Instruction::I8x16Popcnt => sink.i8x16_popcnt(),
            Instruction::I8x16AllTrue => sink.i8x16_all_true(),
            Instruction::I8x16Bitmask => sink.i8x16_bitmask(),
            Instruction::I8x16NarrowI16x8S => sink.i8x16_narrow_i16x8_s(),
            Instruction::I8x16NarrowI16x8U => sink.i8x16_narrow_i16x8_u(),
            Instruction::I8x16Shl => sink.i8x16_shl(),
            Instruction::I8x16ShrS => sink.i8x16_shr_s(),
            Instruction::I8x16ShrU => sink.i8x16_shr_u(),
            Instruction::I8x16Add => sink.i8x16_add(),
            Instruction::I8x16AddSatS => sink.i8x16_add_sat_s(),
            Instruction::I8x16AddSatU => sink.i8x16_add_sat_u(),
            Instruction::I8x16Sub => sink.i8x16_sub(),
            Instruction::I8x16SubSatS => sink.i8x16_sub_sat_s(),
            Instruction::I8x16SubSatU => sink.i8x16_sub_sat_u(),
            Instruction::I8x16MinS => sink.i8x16_min_s(),
            Instruction::I8x16MinU => sink.i8x16_min_u(),
            Instruction::I8x16MaxS => sink.i8x16_max_s(),
            Instruction::I8x16MaxU => sink.i8x16_max_u(),
            Instruction::I8x16AvgrU => sink.i8x16_avgr_u(),
            Instruction::I16x8ExtAddPairwiseI8x16S => sink.i16x8_extadd_pairwise_i8x16_s(),
            Instruction::I16x8ExtAddPairwiseI8x16U => sink.i16x8_extadd_pairwise_i8x16_u(),
            Instruction::I32x4ExtAddPairwiseI16x8S => sink.i32x4_extadd_pairwise_i16x8_s(),
            Instruction::I32x4ExtAddPairwiseI16x8U => sink.i32x4_extadd_pairwise_i16x8_u(),
            Instruction::I16x8Abs => sink.i16x8_abs(),
            Instruction::I16x8Neg => sink.i16x8_neg(),
            Instruction::I16x8Q15MulrSatS => sink.i16x8_q15mulr_sat_s(),
            Instruction::I16x8AllTrue => sink.i16x8_all_true(),
            Instruction::I16x8Bitmask => sink.i16x8_bitmask(),
            Instruction::I16x8NarrowI32x4S => sink.i16x8_narrow_i32x4_s(),
            Instruction::I16x8NarrowI32x4U => sink.i16x8_narrow_i32x4_u(),
            Instruction::I16x8ExtendLowI8x16S => sink.i16x8_extend_low_i8x16_s(),
            Instruction::I16x8ExtendHighI8x16S => sink.i16x8_extend_high_i8x16_s(),
            Instruction::I16x8ExtendLowI8x16U => sink.i16x8_extend_low_i8x16_u(),
            Instruction::I16x8ExtendHighI8x16U => sink.i16x8_extend_high_i8x16_u(),
            Instruction::I16x8Shl => sink.i16x8_shl(),
            Instruction::I16x8ShrS => sink.i16x8_shr_s(),
            Instruction::I16x8ShrU => sink.i16x8_shr_u(),
            Instruction::I16x8Add => sink.i16x8_add(),
            Instruction::I16x8AddSatS => sink.i16x8_add_sat_s(),
            Instruction::I16x8AddSatU => sink.i16x8_add_sat_u(),
            Instruction::I16x8Sub => sink.i16x8_sub(),
            Instruction::I16x8SubSatS => sink.i16x8_sub_sat_s(),
            Instruction::I16x8SubSatU => sink.i16x8_sub_sat_u(),
            Instruction::I16x8Mul => sink.i16x8_mul(),
            Instruction::I16x8MinS => sink.i16x8_min_s(),
            Instruction::I16x8MinU => sink.i16x8_min_u(),
            Instruction::I16x8MaxS => sink.i16x8_max_s(),
            Instruction::I16x8MaxU => sink.i16x8_max_u(),
            Instruction::I16x8AvgrU => sink.i16x8_avgr_u(),
            Instruction::I16x8ExtMulLowI8x16S => sink.i16x8_extmul_low_i8x16_s(),
            Instruction::I16x8ExtMulHighI8x16S => sink.i16x8_extmul_high_i8x16_s(),
            Instruction::I16x8ExtMulLowI8x16U => sink.i16x8_extmul_low_i8x16_u(),
            Instruction::I16x8ExtMulHighI8x16U => sink.i16x8_extmul_high_i8x16_u(),
            Instruction::I32x4Abs => sink.i32x4_abs(),
            Instruction::I32x4Neg => sink.i32x4_neg(),
            Instruction::I32x4AllTrue => sink.i32x4_all_true(),
            Instruction::I32x4Bitmask => sink.i32x4_bitmask(),
            Instruction::I32x4ExtendLowI16x8S => sink.i32x4_extend_low_i16x8_s(),
            Instruction::I32x4ExtendHighI16x8S => sink.i32x4_extend_high_i16x8_s(),
            Instruction::I32x4ExtendLowI16x8U => sink.i32x4_extend_low_i16x8_u(),
            Instruction::I32x4ExtendHighI16x8U => sink.i32x4_extend_high_i16x8_u(),
            Instruction::I32x4Shl => sink.i32x4_shl(),
            Instruction::I32x4ShrS => sink.i32x4_shr_s(),
            Instruction::I32x4ShrU => sink.i32x4_shr_u(),
            Instruction::I32x4Add => sink.i32x4_add(),
            Instruction::I32x4Sub => sink.i32x4_sub(),
            Instruction::I32x4Mul => sink.i32x4_mul(),
            Instruction::I32x4MinS => sink.i32x4_min_s(),
            Instruction::I32x4MinU => sink.i32x4_min_u(),
            Instruction::I32x4MaxS => sink.i32x4_max_s(),
            Instruction::I32x4MaxU => sink.i32x4_max_u(),
            Instruction::I32x4DotI16x8S => sink.i32x4_dot_i16x8_s(),
            Instruction::I32x4ExtMulLowI16x8S => sink.i32x4_extmul_low_i16x8_s(),
            Instruction::I32x4ExtMulHighI16x8S => sink.i32x4_extmul_high_i16x8_s(),
            Instruction::I32x4ExtMulLowI16x8U => sink.i32x4_extmul_low_i16x8_u(),
            Instruction::I32x4ExtMulHighI16x8U => sink.i32x4_extmul_high_i16x8_u(),
            Instruction::I64x2Abs => sink.i64x2_abs(),
            Instruction::I64x2Neg => sink.i64x2_neg(),
            Instruction::I64x2AllTrue => sink.i64x2_all_true(),
            Instruction::I64x2Bitmask => sink.i64x2_bitmask(),
            Instruction::I64x2ExtendLowI32x4S => sink.i64x2_extend_low_i32x4_s(),
            Instruction::I64x2ExtendHighI32x4S => sink.i64x2_extend_high_i32x4_s(),
            Instruction::I64x2ExtendLowI32x4U => sink.i64x2_extend_low_i32x4_u(),
            Instruction::I64x2ExtendHighI32x4U => sink.i64x2_extend_high_i32x4_u(),
            Instruction::I64x2Shl => sink.i64x2_shl(),
            Instruction::I64x2ShrS => sink.i64x2_shr_s(),
            Instruction::I64x2ShrU => sink.i64x2_shr_u(),
            Instruction::I64x2Add => sink.i64x2_add(),
            Instruction::I64x2Sub => sink.i64x2_sub(),
            Instruction::I64x2Mul => sink.i64x2_mul(),
            Instruction::I64x2ExtMulLowI32x4S => sink.i64x2_extmul_low_i32x4_s(),
            Instruction::I64x2ExtMulHighI32x4S => sink.i64x2_extmul_high_i32x4_s(),
            Instruction::I64x2ExtMulLowI32x4U => sink.i64x2_extmul_low_i32x4_u(),
            Instruction::I64x2ExtMulHighI32x4U => sink.i64x2_extmul_high_i32x4_u(),
            Instruction::F32x4Ceil => sink.f32x4_ceil(),
            Instruction::F32x4Floor => sink.f32x4_floor(),
            Instruction::F32x4Trunc => sink.f32x4_trunc(),
            Instruction::F32x4Nearest => sink.f32x4_nearest(),
            Instruction::F32x4Abs => sink.f32x4_abs(),
            Instruction::F32x4Neg => sink.f32x4_neg(),
            Instruction::F32x4Sqrt => sink.f32x4_sqrt(),
            Instruction::F32x4Add => sink.f32x4_add(),
            Instruction::F32x4Sub => sink.f32x4_sub(),
            Instruction::F32x4Mul => sink.f32x4_mul(),
            Instruction::F32x4Div => sink.f32x4_div(),
            Instruction::F32x4Min => sink.f32x4_min(),
            Instruction::F32x4Max => sink.f32x4_max(),
            Instruction::F32x4PMin => sink.f32x4_pmin(),
            Instruction::F32x4PMax => sink.f32x4_pmax(),
            Instruction::F64x2Ceil => sink.f64x2_ceil(),
            Instruction::F64x2Floor => sink.f64x2_floor(),
            Instruction::F64x2Trunc => sink.f64x2_trunc(),
            Instruction::F64x2Nearest => sink.f64x2_nearest(),
            Instruction::F64x2Abs => sink.f64x2_abs(),
            Instruction::F64x2Neg => sink.f64x2_neg(),
            Instruction::F64x2Sqrt => sink.f64x2_sqrt(),
            Instruction::F64x2Add => sink.f64x2_add(),
            Instruction::F64x2Sub => sink.f64x2_sub(),
            Instruction::F64x2Mul => sink.f64x2_mul(),
            Instruction::F64x2Div => sink.f64x2_div(),
            Instruction::F64x2Min => sink.f64x2_min(),
            Instruction::F64x2Max => sink.f64x2_max(),
            Instruction::F64x2PMin => sink.f64x2_pmin(),
            Instruction::F64x2PMax => sink.f64x2_pmax(),
            Instruction::I32x4TruncSatF32x4S => sink.i32x4_trunc_sat_f32x4_s(),
            Instruction::I32x4TruncSatF32x4U => sink.i32x4_trunc_sat_f32x4_u(),
            Instruction::F32x4ConvertI32x4S => sink.f32x4_convert_i32x4_s(),
            Instruction::F32x4ConvertI32x4U => sink.f32x4_convert_i32x4_u(),
            Instruction::I32x4TruncSatF64x2SZero => sink.i32x4_trunc_sat_f64x2_s_zero(),
            Instruction::I32x4TruncSatF64x2UZero => sink.i32x4_trunc_sat_f64x2_u_zero(),
            Instruction::F64x2ConvertLowI32x4S => sink.f64x2_convert_low_i32x4_s(),
            Instruction::F64x2ConvertLowI32x4U => sink.f64x2_convert_low_i32x4_u(),
            Instruction::F32x4DemoteF64x2Zero => sink.f32x4_demote_f64x2_zero(),
            Instruction::F64x2PromoteLowF32x4 => sink.f64x2_promote_low_f32x4(),
            Instruction::V128Load32Zero(memarg) => sink.v128_load32_zero(memarg),
            Instruction::V128Load64Zero(memarg) => sink.v128_load64_zero(memarg),
            Instruction::V128Load8Lane { memarg, lane } => sink.v128_load8_lane(memarg, lane),
            Instruction::V128Load16Lane { memarg, lane } => sink.v128_load16_lane(memarg, lane),
            Instruction::V128Load32Lane { memarg, lane } => sink.v128_load32_lane(memarg, lane),
            Instruction::V128Load64Lane { memarg, lane } => sink.v128_load64_lane(memarg, lane),
            Instruction::V128Store8Lane { memarg, lane } => sink.v128_store8_lane(memarg, lane),
            Instruction::V128Store16Lane { memarg, lane } => sink.v128_store16_lane(memarg, lane),
            Instruction::V128Store32Lane { memarg, lane } => sink.v128_store32_lane(memarg, lane),
            Instruction::V128Store64Lane { memarg, lane } => sink.v128_store64_lane(memarg, lane),
            Instruction::I64x2Eq => sink.i64x2_eq(),
            Instruction::I64x2Ne => sink.i64x2_ne(),
            Instruction::I64x2LtS => sink.i64x2_lt_s(),
            Instruction::I64x2GtS => sink.i64x2_gt_s(),
            Instruction::I64x2LeS => sink.i64x2_le_s(),
            Instruction::I64x2GeS => sink.i64x2_ge_s(),
            Instruction::I8x16RelaxedSwizzle => sink.i8x16_relaxed_swizzle(),
            Instruction::I32x4RelaxedTruncF32x4S => sink.i32x4_relaxed_trunc_f32x4_s(),
            Instruction::I32x4RelaxedTruncF32x4U => sink.i32x4_relaxed_trunc_f32x4_u(),
            Instruction::I32x4RelaxedTruncF64x2SZero => sink.i32x4_relaxed_trunc_f64x2_s_zero(),
            Instruction::I32x4RelaxedTruncF64x2UZero => sink.i32x4_relaxed_trunc_f64x2_u_zero(),
            Instruction::F32x4RelaxedMadd => sink.f32x4_relaxed_madd(),
            Instruction::F32x4RelaxedNmadd => sink.f32x4_relaxed_nmadd(),
            Instruction::F64x2RelaxedMadd => sink.f64x2_relaxed_madd(),
            Instruction::F64x2RelaxedNmadd => sink.f64x2_relaxed_nmadd(),
            Instruction::I8x16RelaxedLaneselect => sink.i8x16_relaxed_laneselect(),
            Instruction::I16x8RelaxedLaneselect => sink.i16x8_relaxed_laneselect(),
            Instruction::I32x4RelaxedLaneselect => sink.i32x4_relaxed_laneselect(),
            Instruction::I64x2RelaxedLaneselect => sink.i64x2_relaxed_laneselect(),
            Instruction::F32x4RelaxedMin => sink.f32x4_relaxed_min(),
            Instruction::F32x4RelaxedMax => sink.f32x4_relaxed_max(),
            Instruction::F64x2RelaxedMin => sink.f64x2_relaxed_min(),
            Instruction::F64x2RelaxedMax => sink.f64x2_relaxed_max(),
            Instruction::I16x8RelaxedQ15mulrS => sink.i16x8_relaxed_q15mulr_s(),
            Instruction::I16x8RelaxedDotI8x16I7x16S => sink.i16x8_relaxed_dot_i8x16_i7x16_s(),
            Instruction::I32x4RelaxedDotI8x16I7x16AddS => {
                sink.i32x4_relaxed_dot_i8x16_i7x16_add_s()
            }

            // Atomic instructions from the thread proposal
            Instruction::MemoryAtomicNotify(memarg) => sink.memory_atomic_notify(memarg),
            Instruction::MemoryAtomicWait32(memarg) => sink.memory_atomic_wait32(memarg),
            Instruction::MemoryAtomicWait64(memarg) => sink.memory_atomic_wait64(memarg),
            Instruction::AtomicFence => sink.atomic_fence(),
            Instruction::I32AtomicLoad(memarg) => sink.i32_atomic_load(memarg),
            Instruction::I64AtomicLoad(memarg) => sink.i64_atomic_load(memarg),
            Instruction::I32AtomicLoad8U(memarg) => sink.i32_atomic_load8_u(memarg),
            Instruction::I32AtomicLoad16U(memarg) => sink.i32_atomic_load16_u(memarg),
            Instruction::I64AtomicLoad8U(memarg) => sink.i64_atomic_load8_u(memarg),
            Instruction::I64AtomicLoad16U(memarg) => sink.i64_atomic_load16_u(memarg),
            Instruction::I64AtomicLoad32U(memarg) => sink.i64_atomic_load32_u(memarg),
            Instruction::I32AtomicStore(memarg) => sink.i32_atomic_store(memarg),
            Instruction::I64AtomicStore(memarg) => sink.i64_atomic_store(memarg),
            Instruction::I32AtomicStore8(memarg) => sink.i32_atomic_store8(memarg),
            Instruction::I32AtomicStore16(memarg) => sink.i32_atomic_store16(memarg),
            Instruction::I64AtomicStore8(memarg) => sink.i64_atomic_store8(memarg),
            Instruction::I64AtomicStore16(memarg) => sink.i64_atomic_store16(memarg),
            Instruction::I64AtomicStore32(memarg) => sink.i64_atomic_store32(memarg),
            Instruction::I32AtomicRmwAdd(memarg) => sink.i32_atomic_rmw_add(memarg),
            Instruction::I64AtomicRmwAdd(memarg) => sink.i64_atomic_rmw_add(memarg),
            Instruction::I32AtomicRmw8AddU(memarg) => sink.i32_atomic_rmw8_add_u(memarg),
            Instruction::I32AtomicRmw16AddU(memarg) => sink.i32_atomic_rmw16_add_u(memarg),
            Instruction::I64AtomicRmw8AddU(memarg) => sink.i64_atomic_rmw8_add_u(memarg),
            Instruction::I64AtomicRmw16AddU(memarg) => sink.i64_atomic_rmw16_add_u(memarg),
            Instruction::I64AtomicRmw32AddU(memarg) => sink.i64_atomic_rmw32_add_u(memarg),
            Instruction::I32AtomicRmwSub(memarg) => sink.i32_atomic_rmw_sub(memarg),
            Instruction::I64AtomicRmwSub(memarg) => sink.i64_atomic_rmw_sub(memarg),
            Instruction::I32AtomicRmw8SubU(memarg) => sink.i32_atomic_rmw8_sub_u(memarg),
            Instruction::I32AtomicRmw16SubU(memarg) => sink.i32_atomic_rmw16_sub_u(memarg),
            Instruction::I64AtomicRmw8SubU(memarg) => sink.i64_atomic_rmw8_sub_u(memarg),
            Instruction::I64AtomicRmw16SubU(memarg) => sink.i64_atomic_rmw16_sub_u(memarg),
            Instruction::I64AtomicRmw32SubU(memarg) => sink.i64_atomic_rmw32_sub_u(memarg),
            Instruction::I32AtomicRmwAnd(memarg) => sink.i32_atomic_rmw_and(memarg),
            Instruction::I64AtomicRmwAnd(memarg) => sink.i64_atomic_rmw_and(memarg),
            Instruction::I32AtomicRmw8AndU(memarg) => sink.i32_atomic_rmw8_and_u(memarg),
            Instruction::I32AtomicRmw16AndU(memarg) => sink.i32_atomic_rmw16_and_u(memarg),
            Instruction::I64AtomicRmw8AndU(memarg) => sink.i64_atomic_rmw8_and_u(memarg),
            Instruction::I64AtomicRmw16AndU(memarg) => sink.i64_atomic_rmw16_and_u(memarg),
            Instruction::I64AtomicRmw32AndU(memarg) => sink.i64_atomic_rmw32_and_u(memarg),
            Instruction::I32AtomicRmwOr(memarg) => sink.i32_atomic_rmw_or(memarg),
            Instruction::I64AtomicRmwOr(memarg) => sink.i64_atomic_rmw_or(memarg),
            Instruction::I32AtomicRmw8OrU(memarg) => sink.i32_atomic_rmw8_or_u(memarg),
            Instruction::I32AtomicRmw16OrU(memarg) => sink.i32_atomic_rmw16_or_u(memarg),
            Instruction::I64AtomicRmw8OrU(memarg) => sink.i64_atomic_rmw8_or_u(memarg),
            Instruction::I64AtomicRmw16OrU(memarg) => sink.i64_atomic_rmw16_or_u(memarg),
            Instruction::I64AtomicRmw32OrU(memarg) => sink.i64_atomic_rmw32_or_u(memarg),
            Instruction::I32AtomicRmwXor(memarg) => sink.i32_atomic_rmw_xor(memarg),
            Instruction::I64AtomicRmwXor(memarg) => sink.i64_atomic_rmw_xor(memarg),
            Instruction::I32AtomicRmw8XorU(memarg) => sink.i32_atomic_rmw8_xor_u(memarg),
            Instruction::I32AtomicRmw16XorU(memarg) => sink.i32_atomic_rmw16_xor_u(memarg),
            Instruction::I64AtomicRmw8XorU(memarg) => sink.i64_atomic_rmw8_xor_u(memarg),
            Instruction::I64AtomicRmw16XorU(memarg) => sink.i64_atomic_rmw16_xor_u(memarg),
            Instruction::I64AtomicRmw32XorU(memarg) => sink.i64_atomic_rmw32_xor_u(memarg),
            Instruction::I32AtomicRmwXchg(memarg) => sink.i32_atomic_rmw_xchg(memarg),
            Instruction::I64AtomicRmwXchg(memarg) => sink.i64_atomic_rmw_xchg(memarg),
            Instruction::I32AtomicRmw8XchgU(memarg) => sink.i32_atomic_rmw8_xchg_u(memarg),
            Instruction::I32AtomicRmw16XchgU(memarg) => sink.i32_atomic_rmw16_xchg_u(memarg),
            Instruction::I64AtomicRmw8XchgU(memarg) => sink.i64_atomic_rmw8_xchg_u(memarg),
            Instruction::I64AtomicRmw16XchgU(memarg) => sink.i64_atomic_rmw16_xchg_u(memarg),
            Instruction::I64AtomicRmw32XchgU(memarg) => sink.i64_atomic_rmw32_xchg_u(memarg),
            Instruction::I32AtomicRmwCmpxchg(memarg) => sink.i32_atomic_rmw_cmpxchg(memarg),
            Instruction::I64AtomicRmwCmpxchg(memarg) => sink.i64_atomic_rmw_cmpxchg(memarg),
            Instruction::I32AtomicRmw8CmpxchgU(memarg) => sink.i32_atomic_rmw8_cmpxchg_u(memarg),
            Instruction::I32AtomicRmw16CmpxchgU(memarg) => sink.i32_atomic_rmw16_cmpxchg_u(memarg),
            Instruction::I64AtomicRmw8CmpxchgU(memarg) => sink.i64_atomic_rmw8_cmpxchg_u(memarg),
            Instruction::I64AtomicRmw16CmpxchgU(memarg) => sink.i64_atomic_rmw16_cmpxchg_u(memarg),
            Instruction::I64AtomicRmw32CmpxchgU(memarg) => sink.i64_atomic_rmw32_cmpxchg_u(memarg),

            // Atomic instructions from the shared-everything-threads proposal
            Instruction::GlobalAtomicGet {
                ordering,
                global_index,
            } => sink.global_atomic_get(ordering, global_index),
            Instruction::GlobalAtomicSet {
                ordering,
                global_index,
            } => sink.global_atomic_set(ordering, global_index),
            Instruction::GlobalAtomicRmwAdd {
                ordering,
                global_index,
            } => sink.global_atomic_rmw_add(ordering, global_index),
            Instruction::GlobalAtomicRmwSub {
                ordering,
                global_index,
            } => sink.global_atomic_rmw_sub(ordering, global_index),
            Instruction::GlobalAtomicRmwAnd {
                ordering,
                global_index,
            } => sink.global_atomic_rmw_and(ordering, global_index),
            Instruction::GlobalAtomicRmwOr {
                ordering,
                global_index,
            } => sink.global_atomic_rmw_or(ordering, global_index),
            Instruction::GlobalAtomicRmwXor {
                ordering,
                global_index,
            } => sink.global_atomic_rmw_xor(ordering, global_index),
            Instruction::GlobalAtomicRmwXchg {
                ordering,
                global_index,
            } => sink.global_atomic_rmw_xchg(ordering, global_index),
            Instruction::GlobalAtomicRmwCmpxchg {
                ordering,
                global_index,
            } => sink.global_atomic_rmw_cmpxchg(ordering, global_index),
            Instruction::TableAtomicGet {
                ordering,
                table_index,
            } => sink.table_atomic_get(ordering, table_index),
            Instruction::TableAtomicSet {
                ordering,
                table_index,
            } => sink.table_atomic_set(ordering, table_index),
            Instruction::TableAtomicRmwXchg {
                ordering,
                table_index,
            } => sink.table_atomic_rmw_xchg(ordering, table_index),
            Instruction::TableAtomicRmwCmpxchg {
                ordering,
                table_index,
            } => sink.table_atomic_rmw_cmpxchg(ordering, table_index),
            Instruction::StructAtomicGet {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_get(ordering, struct_type_index, field_index),
            Instruction::StructAtomicGetS {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_get_s(ordering, struct_type_index, field_index),
            Instruction::StructAtomicGetU {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_get_u(ordering, struct_type_index, field_index),
            Instruction::StructAtomicSet {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_set(ordering, struct_type_index, field_index),
            Instruction::StructAtomicRmwAdd {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_rmw_add(ordering, struct_type_index, field_index),
            Instruction::StructAtomicRmwSub {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_rmw_sub(ordering, struct_type_index, field_index),
            Instruction::StructAtomicRmwAnd {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_rmw_and(ordering, struct_type_index, field_index),
            Instruction::StructAtomicRmwOr {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_rmw_or(ordering, struct_type_index, field_index),
            Instruction::StructAtomicRmwXor {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_rmw_xor(ordering, struct_type_index, field_index),
            Instruction::StructAtomicRmwXchg {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_rmw_xchg(ordering, struct_type_index, field_index),
            Instruction::StructAtomicRmwCmpxchg {
                ordering,
                struct_type_index,
                field_index,
            } => sink.struct_atomic_rmw_cmpxchg(ordering, struct_type_index, field_index),
            Instruction::ArrayAtomicGet {
                ordering,
                array_type_index,
            } => sink.array_atomic_get(ordering, array_type_index),
            Instruction::ArrayAtomicGetS {
                ordering,
                array_type_index,
            } => sink.array_atomic_get_s(ordering, array_type_index),
            Instruction::ArrayAtomicGetU {
                ordering,
                array_type_index,
            } => sink.array_atomic_get_u(ordering, array_type_index),
            Instruction::ArrayAtomicSet {
                ordering,
                array_type_index,
            } => sink.array_atomic_set(ordering, array_type_index),
            Instruction::ArrayAtomicRmwAdd {
                ordering,
                array_type_index,
            } => sink.array_atomic_rmw_add(ordering, array_type_index),
            Instruction::ArrayAtomicRmwSub {
                ordering,
                array_type_index,
            } => sink.array_atomic_rmw_sub(ordering, array_type_index),
            Instruction::ArrayAtomicRmwAnd {
                ordering,
                array_type_index,
            } => sink.array_atomic_rmw_and(ordering, array_type_index),
            Instruction::ArrayAtomicRmwOr {
                ordering,
                array_type_index,
            } => sink.array_atomic_rmw_or(ordering, array_type_index),
            Instruction::ArrayAtomicRmwXor {
                ordering,
                array_type_index,
            } => sink.array_atomic_rmw_xor(ordering, array_type_index),
            Instruction::ArrayAtomicRmwXchg {
                ordering,
                array_type_index,
            } => sink.array_atomic_rmw_xchg(ordering, array_type_index),
            Instruction::ArrayAtomicRmwCmpxchg {
                ordering,
                array_type_index,
            } => sink.array_atomic_rmw_cmpxchg(ordering, array_type_index),
            Instruction::RefI31Shared => sink.ref_i31_shared(),
            Instruction::ContNew(type_index) => sink.cont_new(type_index),
            Instruction::ContBind {
                argument_index,
                result_index,
            } => sink.cont_bind(argument_index, result_index),
            Instruction::Suspend(tag_index) => sink.suspend(tag_index),
            Instruction::Resume {
                cont_type_index,
                ref resume_table,
            } => sink.resume(cont_type_index, resume_table.iter().cloned()),
            Instruction::ResumeThrow {
                cont_type_index,
                tag_index,
                ref resume_table,
            } => sink.resume_throw(cont_type_index, tag_index, resume_table.iter().cloned()),
            Instruction::Switch {
                cont_type_index,
                tag_index,
            } => sink.switch(cont_type_index, tag_index),
            Instruction::I64Add128 => sink.i64_add128(),
            Instruction::I64Sub128 => sink.i64_sub128(),
            Instruction::I64MulWideS => sink.i64_mul_wide_s(),
            Instruction::I64MulWideU => sink.i64_mul_wide_u(),
            Instruction::RefGetDesc(type_index) => sink.ref_get_desc(type_index),
            Instruction::RefCastDescNonNull(hty) => sink.ref_cast_desc_non_null(hty),
            Instruction::RefCastDescNullable(hty) => sink.ref_cast_desc_nullable(hty),
            Instruction::BrOnCastDesc {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => sink.br_on_cast_desc(relative_depth, from_ref_type, to_ref_type),
            Instruction::BrOnCastDescFail {
                relative_depth,
                from_ref_type,
                to_ref_type,
            } => sink.br_on_cast_desc_fail(relative_depth, from_ref_type, to_ref_type),
        };
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

#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Handle {
    OnLabel { tag: u32, label: u32 },
    OnSwitch { tag: u32 },
}

impl Encode for Handle {
    fn encode(&self, sink: &mut Vec<u8>) {
        match self {
            Handle::OnLabel { tag, label } => {
                sink.push(0x00);
                tag.encode(sink);
                label.encode(sink);
            }
            Handle::OnSwitch { tag } => {
                sink.push(0x01);
                tag.encode(sink);
            }
        }
    }
}

/// A constant expression.
///
/// Usable in contexts such as offsets or initializers.
#[derive(Clone, Debug)]
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

    /// Create a constant expression with the sequence of instructions
    pub fn extended<'a>(insns: impl IntoIterator<Item = Instruction<'a>>) -> Self {
        let mut bytes = vec![];
        for insn in insns {
            insn.encode(&mut bytes);
        }
        Self { bytes }
    }

    fn new<F>(f: F) -> Self
    where
        for<'a, 'b> F: FnOnce(&'a mut InstructionSink<'b>) -> &'a mut InstructionSink<'b>,
    {
        let mut bytes = vec![];
        f(&mut InstructionSink::new(&mut bytes));
        Self { bytes }
    }

    fn with<F>(mut self, f: F) -> Self
    where
        for<'a, 'b> F: FnOnce(&'a mut InstructionSink<'b>) -> &'a mut InstructionSink<'b>,
    {
        f(&mut InstructionSink::new(&mut self.bytes));
        self
    }

    /// Create a constant expression containing a single `global.get` instruction.
    pub fn global_get(index: u32) -> Self {
        Self::new(|insn| insn.global_get(index))
    }

    /// Create a constant expression containing a single `ref.null` instruction.
    pub fn ref_null(ty: HeapType) -> Self {
        Self::new(|insn| insn.ref_null(ty))
    }

    /// Create a constant expression containing a single `ref.func` instruction.
    pub fn ref_func(func: u32) -> Self {
        Self::new(|insn| insn.ref_func(func))
    }

    /// Create a constant expression containing a single `i32.const` instruction.
    pub fn i32_const(value: i32) -> Self {
        Self::new(|insn| insn.i32_const(value))
    }

    /// Create a constant expression containing a single `i64.const` instruction.
    pub fn i64_const(value: i64) -> Self {
        Self::new(|insn| insn.i64_const(value))
    }

    /// Create a constant expression containing a single `f32.const` instruction.
    pub fn f32_const(value: Ieee32) -> Self {
        Self::new(|insn| insn.f32_const(value))
    }

    /// Create a constant expression containing a single `f64.const` instruction.
    pub fn f64_const(value: Ieee64) -> Self {
        Self::new(|insn| insn.f64_const(value))
    }

    /// Create a constant expression containing a single `v128.const` instruction.
    pub fn v128_const(value: i128) -> Self {
        Self::new(|insn| insn.v128_const(value))
    }

    /// Add a `global.get` instruction to this constant expression.
    pub fn with_global_get(self, index: u32) -> Self {
        self.with(|insn| insn.global_get(index))
    }

    /// Add a `ref.null` instruction to this constant expression.
    pub fn with_ref_null(self, ty: HeapType) -> Self {
        self.with(|insn| insn.ref_null(ty))
    }

    /// Add a `ref.func` instruction to this constant expression.
    pub fn with_ref_func(self, func: u32) -> Self {
        self.with(|insn| insn.ref_func(func))
    }

    /// Add an `i32.const` instruction to this constant expression.
    pub fn with_i32_const(self, value: i32) -> Self {
        self.with(|insn| insn.i32_const(value))
    }

    /// Add an `i64.const` instruction to this constant expression.
    pub fn with_i64_const(self, value: i64) -> Self {
        self.with(|insn| insn.i64_const(value))
    }

    /// Add a `f32.const` instruction to this constant expression.
    pub fn with_f32_const(self, value: Ieee32) -> Self {
        self.with(|insn| insn.f32_const(value))
    }

    /// Add a `f64.const` instruction to this constant expression.
    pub fn with_f64_const(self, value: Ieee64) -> Self {
        self.with(|insn| insn.f64_const(value))
    }

    /// Add a `v128.const` instruction to this constant expression.
    pub fn with_v128_const(self, value: i128) -> Self {
        self.with(|insn| insn.v128_const(value))
    }

    /// Add an `i32.add` instruction to this constant expression.
    pub fn with_i32_add(self) -> Self {
        self.with(|insn| insn.i32_add())
    }

    /// Add an `i32.sub` instruction to this constant expression.
    pub fn with_i32_sub(self) -> Self {
        self.with(|insn| insn.i32_sub())
    }

    /// Add an `i32.mul` instruction to this constant expression.
    pub fn with_i32_mul(self) -> Self {
        self.with(|insn| insn.i32_mul())
    }

    /// Add an `i64.add` instruction to this constant expression.
    pub fn with_i64_add(self) -> Self {
        self.with(|insn| insn.i64_add())
    }

    /// Add an `i64.sub` instruction to this constant expression.
    pub fn with_i64_sub(self) -> Self {
        self.with(|insn| insn.i64_sub())
    }

    /// Add an `i64.mul` instruction to this constant expression.
    pub fn with_i64_mul(self) -> Self {
        self.with(|insn| insn.i64_mul())
    }

    /// Returns the function, if any, referenced by this global.
    pub fn get_ref_func(&self) -> Option<u32> {
        let prefix = *self.bytes.get(0)?;
        // 0xd2 == `ref.func` opcode, and if that's found then load the leb
        // corresponding to the function index.
        if prefix != 0xd2 {
            return None;
        }
        leb128fmt::decode_uint_slice::<u32, 32>(&self.bytes[1..], &mut 0).ok()
    }
}

impl Encode for ConstExpr {
    fn encode(&self, sink: &mut Vec<u8>) {
        sink.extend(&self.bytes);
        InstructionSink::new(sink).end();
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

    #[test]
    fn func_raw_bytes() {
        use super::*;

        let mut f = Function::new([(1, ValType::I32), (1, ValType::F32)]);
        f.instructions().end();
        let mut code_from_func = CodeSection::new();
        code_from_func.function(&f);
        let bytes = f.into_raw_body();
        let mut code_from_raw = CodeSection::new();
        code_from_raw.raw(&bytes[..]);

        let mut c1 = vec![];
        code_from_func.encode(&mut c1);
        let mut c2 = vec![];
        code_from_raw.encode(&mut c2);
        assert_eq!(c1, c2);
    }
}
