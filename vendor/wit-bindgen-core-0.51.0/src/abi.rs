use std::fmt;
use std::iter;

pub use wit_parser::abi::{AbiVariant, FlatTypes, WasmSignature, WasmType};
use wit_parser::{
    Alignment, ArchitectureSize, ElementInfo, Enum, Flags, FlagsRepr, Function, Handle, Int,
    Record, Resolve, Result_, SizeAlign, Tuple, Type, TypeDefKind, TypeId, Variant, align_to_arch,
};

// Helper macro for defining instructions without having to have tons of
// exhaustive `match` statements to update
macro_rules! def_instruction {
    (
        $( #[$enum_attr:meta] )*
        pub enum $name:ident<'a> {
            $(
                $( #[$attr:meta] )*
                $variant:ident $( {
                    $($field:ident : $field_ty:ty $(,)* )*
                } )?
                    :
                [$num_popped:expr] => [$num_pushed:expr],
            )*
        }
    ) => {
        $( #[$enum_attr] )*
        pub enum $name<'a> {
            $(
                $( #[$attr] )*
                $variant $( {
                    $(
                        $field : $field_ty,
                    )*
                } )? ,
            )*
        }

        impl $name<'_> {
            /// How many operands does this instruction pop from the stack?
            #[allow(unused_variables, reason = "match arms bind fields for exhaustiveness, not usage")]
            pub fn operands_len(&self) -> usize {
                match self {
                    $(
                        Self::$variant $( {
                            $(
                                $field,
                            )*
                        } )? => $num_popped,
                    )*
                }
            }

            /// How many results does this instruction push onto the stack?
            #[allow(unused_variables, reason = "match arms bind fields for exhaustiveness, not usage")]
            pub fn results_len(&self) -> usize {
                match self {
                    $(
                        Self::$variant $( {
                            $(
                                $field,
                            )*
                        } )? => $num_pushed,
                    )*
                }
            }
        }
    };
}

def_instruction! {
    #[derive(Debug)]
    pub enum Instruction<'a> {
        /// Acquires the specified parameter and places it on the stack.
        /// Depending on the context this may refer to wasm parameters or
        /// interface types parameters.
        GetArg { nth: usize } : [0] => [1],

        // Integer const/manipulation instructions

        /// Pushes the constant `val` onto the stack.
        I32Const { val: i32 } : [0] => [1],
        /// Casts the top N items on the stack using the `Bitcast` enum
        /// provided. Consumes the same number of operands that this produces.
        Bitcasts { casts: &'a [Bitcast] } : [casts.len()] => [casts.len()],
        /// Pushes a number of constant zeros for each wasm type on the stack.
        ConstZero { tys: &'a [WasmType] } : [0] => [tys.len()],

        // Memory load/store instructions

        /// Pops a pointer from the stack and loads a little-endian `i32` from
        /// it, using the specified constant offset.
        I32Load { offset: ArchitectureSize } : [1] => [1],
        /// Pops a pointer from the stack and loads a little-endian `i8` from
        /// it, using the specified constant offset. The value loaded is the
        /// zero-extended to 32-bits
        I32Load8U { offset: ArchitectureSize } : [1] => [1],
        /// Pops a pointer from the stack and loads a little-endian `i8` from
        /// it, using the specified constant offset. The value loaded is the
        /// sign-extended to 32-bits
        I32Load8S { offset: ArchitectureSize } : [1] => [1],
        /// Pops a pointer from the stack and loads a little-endian `i16` from
        /// it, using the specified constant offset. The value loaded is the
        /// zero-extended to 32-bits
        I32Load16U { offset: ArchitectureSize } : [1] => [1],
        /// Pops a pointer from the stack and loads a little-endian `i16` from
        /// it, using the specified constant offset. The value loaded is the
        /// sign-extended to 32-bits
        I32Load16S { offset: ArchitectureSize } : [1] => [1],
        /// Pops a pointer from the stack and loads a little-endian `i64` from
        /// it, using the specified constant offset.
        I64Load { offset: ArchitectureSize } : [1] => [1],
        /// Pops a pointer from the stack and loads a little-endian `f32` from
        /// it, using the specified constant offset.
        F32Load { offset: ArchitectureSize } : [1] => [1],
        /// Pops a pointer from the stack and loads a little-endian `f64` from
        /// it, using the specified constant offset.
        F64Load { offset: ArchitectureSize } : [1] => [1],

        /// Like `I32Load` or `I64Load`, but for loading pointer values.
        PointerLoad { offset: ArchitectureSize } : [1] => [1],
        /// Like `I32Load` or `I64Load`, but for loading array length values.
        LengthLoad { offset: ArchitectureSize } : [1] => [1],

        /// Pops a pointer from the stack and then an `i32` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        I32Store { offset: ArchitectureSize } : [2] => [0],
        /// Pops a pointer from the stack and then an `i32` value.
        /// Stores the low 8 bits of the value in little-endian at the pointer
        /// specified plus the constant `offset`.
        I32Store8 { offset: ArchitectureSize } : [2] => [0],
        /// Pops a pointer from the stack and then an `i32` value.
        /// Stores the low 16 bits of the value in little-endian at the pointer
        /// specified plus the constant `offset`.
        I32Store16 { offset: ArchitectureSize } : [2] => [0],
        /// Pops a pointer from the stack and then an `i64` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        I64Store { offset: ArchitectureSize } : [2] => [0],
        /// Pops a pointer from the stack and then an `f32` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        F32Store { offset: ArchitectureSize } : [2] => [0],
        /// Pops a pointer from the stack and then an `f64` value.
        /// Stores the value in little-endian at the pointer specified plus the
        /// constant `offset`.
        F64Store { offset: ArchitectureSize } : [2] => [0],

        /// Like `I32Store` or `I64Store`, but for storing pointer values.
        PointerStore { offset: ArchitectureSize } : [2] => [0],
        /// Like `I32Store` or `I64Store`, but for storing array length values.
        LengthStore { offset: ArchitectureSize } : [2] => [0],

        // Scalar lifting/lowering

        /// Converts an interface type `char` value to a 32-bit integer
        /// representing the unicode scalar value.
        I32FromChar : [1] => [1],
        /// Converts an interface type `u64` value to a wasm `i64`.
        I64FromU64 : [1] => [1],
        /// Converts an interface type `s64` value to a wasm `i64`.
        I64FromS64 : [1] => [1],
        /// Converts an interface type `u32` value to a wasm `i32`.
        I32FromU32 : [1] => [1],
        /// Converts an interface type `s32` value to a wasm `i32`.
        I32FromS32 : [1] => [1],
        /// Converts an interface type `u16` value to a wasm `i32`.
        I32FromU16 : [1] => [1],
        /// Converts an interface type `s16` value to a wasm `i32`.
        I32FromS16 : [1] => [1],
        /// Converts an interface type `u8` value to a wasm `i32`.
        I32FromU8 : [1] => [1],
        /// Converts an interface type `s8` value to a wasm `i32`.
        I32FromS8 : [1] => [1],
        /// Conversion an interface type `f32` value to a wasm `f32`.
        ///
        /// This may be a noop for some implementations, but it's here in case the
        /// native language representation of `f32` is different than the wasm
        /// representation of `f32`.
        CoreF32FromF32 : [1] => [1],
        /// Conversion an interface type `f64` value to a wasm `f64`.
        ///
        /// This may be a noop for some implementations, but it's here in case the
        /// native language representation of `f64` is different than the wasm
        /// representation of `f64`.
        CoreF64FromF64 : [1] => [1],

        /// Converts a native wasm `i32` to an interface type `s8`.
        ///
        /// This will truncate the upper bits of the `i32`.
        S8FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `u8`.
        ///
        /// This will truncate the upper bits of the `i32`.
        U8FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `s16`.
        ///
        /// This will truncate the upper bits of the `i32`.
        S16FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `u16`.
        ///
        /// This will truncate the upper bits of the `i32`.
        U16FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `s32`.
        S32FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `u32`.
        U32FromI32 : [1] => [1],
        /// Converts a native wasm `i64` to an interface type `s64`.
        S64FromI64 : [1] => [1],
        /// Converts a native wasm `i64` to an interface type `u64`.
        U64FromI64 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `char`.
        ///
        /// It's safe to assume that the `i32` is indeed a valid unicode code point.
        CharFromI32 : [1] => [1],
        /// Converts a native wasm `f32` to an interface type `f32`.
        F32FromCoreF32 : [1] => [1],
        /// Converts a native wasm `f64` to an interface type `f64`.
        F64FromCoreF64 : [1] => [1],

        /// Creates a `bool` from an `i32` input, trapping if the `i32` isn't
        /// zero or one.
        BoolFromI32 : [1] => [1],
        /// Creates an `i32` from a `bool` input, must return 0 or 1.
        I32FromBool : [1] => [1],

        // lists

        /// Lowers a list where the element's layout in the native language is
        /// expected to match the canonical ABI definition of interface types.
        ///
        /// Pops a list value from the stack and pushes the pointer/length onto
        /// the stack. If `realloc` is set to `Some` then this is expected to
        /// *consume* the list which means that the data needs to be copied. An
        /// allocation/copy is expected when:
        ///
        /// * A host is calling a wasm export with a list (it needs to copy the
        ///   list in to the callee's module, allocating space with `realloc`)
        /// * A wasm export is returning a list (it's expected to use `realloc`
        ///   to give ownership of the list to the caller.
        /// * A host is returning a list in a import definition, meaning that
        ///   space needs to be allocated in the caller with `realloc`).
        ///
        /// A copy does not happen (e.g. `realloc` is `None`) when:
        ///
        /// * A wasm module calls an import with the list. In this situation
        ///   it's expected the caller will know how to access this module's
        ///   memory (e.g. the host has raw access or wasm-to-wasm communication
        ///   would copy the list).
        ///
        /// If `realloc` is `Some` then the adapter is not responsible for
        /// cleaning up this list because the other end is receiving the
        /// allocation. If `realloc` is `None` then the adapter is responsible
        /// for cleaning up any temporary allocation it created, if any.
        ListCanonLower {
            element: &'a Type,
            realloc: Option<&'a str>,
        } : [1] => [2],

        /// Same as `ListCanonLower`, but used for strings
        StringLower {
            realloc: Option<&'a str>,
        } : [1] => [2],

        /// Lowers a list where the element's layout in the native language is
        /// not expected to match the canonical ABI definition of interface
        /// types.
        ///
        /// Pops a list value from the stack and pushes the pointer/length onto
        /// the stack. This operation also pops a block from the block stack
        /// which is used as the iteration body of writing each element of the
        /// list consumed.
        ///
        /// The `realloc` field here behaves the same way as `ListCanonLower`.
        /// It's only set to `None` when a wasm module calls a declared import.
        /// Otherwise lowering in other contexts requires allocating memory for
        /// the receiver to own.
        ListLower {
            element: &'a Type,
            realloc: Option<&'a str>,
        } : [1] => [2],

        /// Lifts a list which has a canonical representation into an interface
        /// types value.
        ///
        /// The term "canonical" representation here means that the
        /// representation of the interface types value in the native language
        /// exactly matches the canonical ABI definition of the type.
        ///
        /// This will consume two `i32` values from the stack, a pointer and a
        /// length, and then produces an interface value list.
        ListCanonLift {
            element: &'a Type,
            ty: TypeId,
        } : [2] => [1],

        /// Same as `ListCanonLift`, but used for strings
        StringLift : [2] => [1],

        /// Lifts a list which into an interface types value.
        ///
        /// This will consume two `i32` values from the stack, a pointer and a
        /// length, and then produces an interface value list.
        ///
        /// This will also pop a block from the block stack which is how to
        /// read each individual element from the list.
        ListLift {
            element: &'a Type,
            ty: TypeId,
        } : [2] => [1],

        /// Pushes an operand onto the stack representing the list item from
        /// each iteration of the list.
        ///
        /// This is only used inside of blocks related to lowering lists.
        IterElem { element: &'a Type } : [0] => [1],

        /// Pushes an operand onto the stack representing the base pointer of
        /// the next element in a list.
        ///
        /// This is used for both lifting and lowering lists.
        IterBasePointer : [0] => [1],

        // records and tuples

        /// Pops a record value off the stack, decomposes the record to all of
        /// its fields, and then pushes the fields onto the stack.
        RecordLower {
            record: &'a Record,
            name: &'a str,
            ty: TypeId,
        } : [1] => [record.fields.len()],

        /// Pops all fields for a record off the stack and then composes them
        /// into a record.
        RecordLift {
            record: &'a Record,
            name: &'a str,
            ty: TypeId,
        } : [record.fields.len()] => [1],

        /// Create an `i32` from a handle.
        HandleLower {
            handle: &'a Handle,
            name: &'a str,
            ty: TypeId,
        } : [1] => [1],

        /// Create a handle from an `i32`.
        HandleLift {
            handle: &'a Handle,
            name: &'a str,
            ty: TypeId,
        } : [1] => [1],

        /// Create an `i32` from a future.
        FutureLower {
            payload: &'a Option<Type>,
            ty: TypeId,
        } : [1] => [1],

        /// Create a future from an `i32`.
        FutureLift {
            payload: &'a Option<Type>,
            ty: TypeId,
        } : [1] => [1],

        /// Create an `i32` from a stream.
        StreamLower {
            payload: &'a Option<Type>,
            ty: TypeId,
        } : [1] => [1],

        /// Create a stream from an `i32`.
        StreamLift {
            payload: &'a Option<Type>,
            ty: TypeId,
        } : [1] => [1],

        /// Create an `i32` from an error-context.
        ErrorContextLower : [1] => [1],

        /// Create a error-context from an `i32`.
        ErrorContextLift : [1] => [1],

        /// Pops a tuple value off the stack, decomposes the tuple to all of
        /// its fields, and then pushes the fields onto the stack.
        TupleLower {
            tuple: &'a Tuple,
            ty: TypeId,
        } : [1] => [tuple.types.len()],

        /// Pops all fields for a tuple off the stack and then composes them
        /// into a tuple.
        TupleLift {
            tuple: &'a Tuple,
            ty: TypeId,
        } : [tuple.types.len()] => [1],

        /// Converts a language-specific record-of-bools to a list of `i32`.
        FlagsLower {
            flags: &'a Flags,
            name: &'a str,
            ty: TypeId,
        } : [1] => [flags.repr().count()],
        /// Converts a list of native wasm `i32` to a language-specific
        /// record-of-bools.
        FlagsLift {
            flags: &'a Flags,
            name: &'a str,
            ty: TypeId,
        } : [flags.repr().count()] => [1],

        // variants

        /// This is a special instruction used for `VariantLower`
        /// instruction to determine the name of the payload, if present, to use
        /// within each block.
        ///
        /// Each sub-block will have this be the first instruction, and if it
        /// lowers a payload it will expect something bound to this name.
        VariantPayloadName : [0] => [1],

        /// Pops a variant off the stack as well as `ty.cases.len()` blocks
        /// from the code generator. Uses each of those blocks and the value
        /// from the stack to produce `nresults` of items.
        VariantLower {
            variant: &'a Variant,
            name: &'a str,
            ty: TypeId,
            results: &'a [WasmType],
        } : [1] => [results.len()],

        /// Pops an `i32` off the stack as well as `ty.cases.len()` blocks
        /// from the code generator. Uses each of those blocks and the value
        /// from the stack to produce a final variant.
        VariantLift {
            variant: &'a Variant,
            name: &'a str,
            ty: TypeId,
        } : [1] => [1],

        /// Pops an enum off the stack and pushes the `i32` representation.
        EnumLower {
            enum_: &'a Enum,
            name: &'a str,
            ty: TypeId,
        } : [1] => [1],

        /// Pops an `i32` off the stack and lifts it into the `enum` specified.
        EnumLift {
            enum_: &'a Enum,
            name: &'a str,
            ty: TypeId,
        } : [1] => [1],

        /// Specialization of `VariantLower` for specifically `option<T>` types,
        /// otherwise behaves the same as `VariantLower` (e.g. two blocks for
        /// the two cases.
        OptionLower {
            payload: &'a Type,
            ty: TypeId,
            results: &'a [WasmType],
        } : [1] => [results.len()],

        /// Specialization of `VariantLift` for specifically the `option<T>`
        /// type. Otherwise behaves the same as the `VariantLift` instruction
        /// with two blocks for the lift.
        OptionLift {
            payload: &'a Type,
            ty: TypeId,
        } : [1] => [1],

        /// Specialization of `VariantLower` for specifically `result<T, E>`
        /// types, otherwise behaves the same as `VariantLower` (e.g. two blocks
        /// for the two cases.
        ResultLower {
            result: &'a Result_
            ty: TypeId,
            results: &'a [WasmType],
        } : [1] => [results.len()],

        /// Specialization of `VariantLift` for specifically the `result<T,
        /// E>` type. Otherwise behaves the same as the `VariantLift`
        /// instruction with two blocks for the lift.
        ResultLift {
            result: &'a Result_,
            ty: TypeId,
        } : [1] => [1],

        // calling/control flow

        /// Represents a call to a raw WebAssembly API. The module/name are
        /// provided inline as well as the types if necessary.
        CallWasm {
            name: &'a str,
            sig: &'a WasmSignature,
        } : [sig.params.len()] => [sig.results.len()],

        /// Same as `CallWasm`, except the dual where an interface is being
        /// called rather than a raw wasm function.
        ///
        /// Note that this will be used for async functions, and `async_`
        /// indicates whether the function should be invoked in an async
        /// fashion.
        CallInterface {
            func: &'a Function,
            async_: bool,
        } : [func.params.len()] => [usize::from(func.result.is_some())],

        /// Returns `amt` values on the stack. This is always the last
        /// instruction.
        Return { amt: usize, func: &'a Function } : [*amt] => [0],

        /// Calls the `realloc` function specified in a malloc-like fashion
        /// allocating `size` bytes with alignment `align`.
        ///
        /// Pushes the returned pointer onto the stack.
        Malloc {
            realloc: &'static str,
            size: ArchitectureSize,
            align: Alignment,
        } : [0] => [1],

        /// Used exclusively for guest-code generation this indicates that
        /// the standard memory deallocation function needs to be invoked with
        /// the specified parameters.
        ///
        /// This will pop a pointer from the stack and push nothing.
        GuestDeallocate {
            size: ArchitectureSize,
            align: Alignment,
        } : [1] => [0],

        /// Used exclusively for guest-code generation this indicates that
        /// a string is being deallocated. The ptr/length are on the stack and
        /// are poppped off and used to deallocate the string.
        GuestDeallocateString : [2] => [0],

        /// Used exclusively for guest-code generation this indicates that
        /// a list is being deallocated. The ptr/length are on the stack and
        /// are poppped off and used to deallocate the list.
        ///
        /// This variant also pops a block off the block stack to be used as the
        /// body of the deallocation loop.
        GuestDeallocateList {
            element: &'a Type,
        } : [2] => [0],

        /// Used exclusively for guest-code generation this indicates that
        /// a variant is being deallocated. The integer discriminant is popped
        /// off the stack as well as `blocks` number of blocks popped from the
        /// blocks stack. The variant is used to select, at runtime, which of
        /// the blocks is executed to deallocate the variant.
        GuestDeallocateVariant {
            blocks: usize,
        } : [1] => [0],

        /// Deallocates the language-specific handle representation on the top
        /// of the stack. Used for async imports.
        DropHandle { ty: &'a Type } : [1] => [0],

        /// Call `task.return` for an async-lifted export.
        ///
        /// This will call core wasm import `name` which will be mapped to
        /// `task.return` later on. The function given has `params` as its
        /// parameters and it will return no results. This is used to pass the
        /// lowered representation of a function's results to `task.return`.
        AsyncTaskReturn { name: &'a str, params: &'a [WasmType] } : [params.len()] => [0],

        /// Force the evaluation of the specified number of expressions and push
        /// the results to the stack.
        ///
        /// This is useful prior to disposing of temporary variables and/or
        /// allocations which are referenced by one or more not-yet-evaluated
        /// expressions.
        Flush { amt: usize } : [*amt] => [*amt],
    }
}

#[derive(Debug, PartialEq)]
pub enum Bitcast {
    // Upcasts
    F32ToI32,
    F64ToI64,
    I32ToI64,
    F32ToI64,

    // Downcasts
    I32ToF32,
    I64ToF64,
    I64ToI32,
    I64ToF32,

    // PointerOrI64 conversions. These preserve provenance when the source
    // or destination is a pointer value.
    //
    // These are used when pointer values are being stored in
    // (ToP64) and loaded out of (P64To) PointerOrI64 values, so they
    // always have to preserve provenance when the value being loaded or
    // stored is a pointer.
    P64ToI64,
    I64ToP64,
    P64ToP,
    PToP64,

    // Pointer<->number conversions. These do not preserve provenance.
    //
    // These are used when integer or floating-point values are being stored in
    // (I32ToP/etc.) and loaded out of (PToI32/etc.) pointer values, so they
    // never have any provenance to preserve.
    I32ToP,
    PToI32,
    PToL,
    LToP,

    // Number<->Number conversions.
    I32ToL,
    LToI32,
    I64ToL,
    LToI64,

    // Multiple conversions in sequence.
    Sequence(Box<[Bitcast; 2]>),

    None,
}

/// Whether the glue code surrounding a call is lifting arguments and lowering
/// results or vice versa.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LiftLower {
    /// When the glue code lifts arguments and lowers results.
    ///
    /// ```text
    /// Wasm --lift-args--> SourceLanguage; call; SourceLanguage --lower-results--> Wasm
    /// ```
    LiftArgsLowerResults,
    /// When the glue code lowers arguments and lifts results.
    ///
    /// ```text
    /// SourceLanguage --lower-args--> Wasm; call; Wasm --lift-results--> SourceLanguage
    /// ```
    LowerArgsLiftResults,
}

/// Trait for language implementors to use to generate glue code between native
/// WebAssembly signatures and interface types signatures.
///
/// This is used as an implementation detail in interpreting the ABI between
/// interface types and wasm types. Eventually this will be driven by interface
/// types adapters themselves, but for now the ABI of a function dictates what
/// instructions are fed in.
///
/// Types implementing `Bindgen` are incrementally fed `Instruction` values to
/// generate code for. Instructions operate like a stack machine where each
/// instruction has a list of inputs and a list of outputs (provided by the
/// `emit` function).
pub trait Bindgen {
    /// The intermediate type for fragments of code for this type.
    ///
    /// For most languages `String` is a suitable intermediate type.
    type Operand: Clone + fmt::Debug;

    /// Emit code to implement the given instruction.
    ///
    /// Each operand is given in `operands` and can be popped off if ownership
    /// is required. It's guaranteed that `operands` has the appropriate length
    /// for the `inst` given, as specified with [`Instruction`].
    ///
    /// Each result variable should be pushed onto `results`. This function must
    /// push the appropriate number of results or binding generation will panic.
    fn emit(
        &mut self,
        resolve: &Resolve,
        inst: &Instruction<'_>,
        operands: &mut Vec<Self::Operand>,
        results: &mut Vec<Self::Operand>,
    );

    /// Gets a operand reference to the return pointer area.
    ///
    /// The provided size and alignment is for the function's return type.
    fn return_pointer(&mut self, size: ArchitectureSize, align: Alignment) -> Self::Operand;

    /// Enters a new block of code to generate code for.
    ///
    /// This is currently exclusively used for constructing variants. When a
    /// variant is constructed a block here will be pushed for each case of a
    /// variant, generating the code necessary to translate a variant case.
    ///
    /// Blocks are completed with `finish_block` below. It's expected that `emit`
    /// will always push code (if necessary) into the "current block", which is
    /// updated by calling this method and `finish_block` below.
    fn push_block(&mut self);

    /// Indicates to the code generator that a block is completed, and the
    /// `operand` specified was the resulting value of the block.
    ///
    /// This method will be used to compute the value of each arm of lifting a
    /// variant. The `operand` will be `None` if the variant case didn't
    /// actually have any type associated with it. Otherwise it will be `Some`
    /// as the last value remaining on the stack representing the value
    /// associated with a variant's `case`.
    ///
    /// It's expected that this will resume code generation in the previous
    /// block before `push_block` was called. This must also save the results
    /// of the current block internally for instructions like `ResultLift` to
    /// use later.
    fn finish_block(&mut self, operand: &mut Vec<Self::Operand>);

    /// Returns size information that was previously calculated for all types.
    fn sizes(&self) -> &SizeAlign;

    /// Returns whether or not the specified element type is represented in a
    /// "canonical" form for lists. This dictates whether the `ListCanonLower`
    /// and `ListCanonLift` instructions are used or not.
    fn is_list_canonical(&self, resolve: &Resolve, element: &Type) -> bool;
}

/// Generates an abstract sequence of instructions which represents this
/// function being adapted as an imported function.
///
/// The instructions here, when executed, will emulate a language with
/// interface types calling the concrete wasm implementation. The parameters
/// for the returned instruction sequence are the language's own
/// interface-types parameters. One instruction in the instruction stream
/// will be a `Call` which represents calling the actual raw wasm function
/// signature.
///
/// This function is useful, for example, if you're building a language
/// generator for WASI bindings. This will document how to translate
/// language-specific values into the wasm types to call a WASI function,
/// and it will also automatically convert the results of the WASI function
/// back to a language-specific value.
pub fn call(
    resolve: &Resolve,
    variant: AbiVariant,
    lift_lower: LiftLower,
    func: &Function,
    bindgen: &mut impl Bindgen,
    async_: bool,
) {
    Generator::new(resolve, bindgen).call(func, variant, lift_lower, async_);
}

pub fn lower_to_memory<B: Bindgen>(
    resolve: &Resolve,
    bindgen: &mut B,
    address: B::Operand,
    value: B::Operand,
    ty: &Type,
) {
    let mut generator = Generator::new(resolve, bindgen);
    // TODO: make this configurable? Right now this function is only called for
    // future/stream callbacks so it's appropriate to skip realloc here as it's
    // all "lower for wasm import", but this might get reused for something else
    // in the future.
    generator.realloc = Some(Realloc::Export("cabi_realloc"));
    generator.stack.push(value);
    generator.write_to_memory(ty, address, Default::default());
}

pub fn lower_flat<B: Bindgen>(
    resolve: &Resolve,
    bindgen: &mut B,
    value: B::Operand,
    ty: &Type,
) -> Vec<B::Operand> {
    let mut generator = Generator::new(resolve, bindgen);
    generator.stack.push(value);
    generator.realloc = Some(Realloc::Export("cabi_realloc"));
    generator.lower(ty);
    generator.stack
}

pub fn lift_from_memory<B: Bindgen>(
    resolve: &Resolve,
    bindgen: &mut B,
    address: B::Operand,
    ty: &Type,
) -> B::Operand {
    let mut generator = Generator::new(resolve, bindgen);
    generator.read_from_memory(ty, address, Default::default());
    generator.stack.pop().unwrap()
}

/// Used in a similar manner as the `Interface::call` function except is
/// used to generate the `post-return` callback for `func`.
///
/// This is only intended to be used in guest generators for exported
/// functions and will primarily generate `GuestDeallocate*` instructions,
/// plus others used as input to those instructions.
pub fn post_return(resolve: &Resolve, func: &Function, bindgen: &mut impl Bindgen) {
    Generator::new(resolve, bindgen).post_return(func);
}

/// Returns whether the `Function` specified needs a post-return function to
/// be generated in guest code.
///
/// This is used when the return value contains a memory allocation such as
/// a list or a string primarily.
pub fn guest_export_needs_post_return(resolve: &Resolve, func: &Function) -> bool {
    func.result
        .map(|t| needs_deallocate(resolve, &t, Deallocate::Lists))
        .unwrap_or(false)
}

pub fn guest_export_params_have_allocations(resolve: &Resolve, func: &Function) -> bool {
    func.params
        .iter()
        .any(|(_, t)| needs_deallocate(resolve, &t, Deallocate::Lists))
}

fn needs_deallocate(resolve: &Resolve, ty: &Type, what: Deallocate) -> bool {
    match ty {
        Type::String => true,
        Type::ErrorContext => true,
        Type::Id(id) => match &resolve.types[*id].kind {
            TypeDefKind::List(_) => true,
            TypeDefKind::Type(t) => needs_deallocate(resolve, t, what),
            TypeDefKind::Handle(Handle::Own(_)) => what.handles(),
            TypeDefKind::Handle(Handle::Borrow(_)) => false,
            TypeDefKind::Resource => false,
            TypeDefKind::Record(r) => r
                .fields
                .iter()
                .any(|f| needs_deallocate(resolve, &f.ty, what)),
            TypeDefKind::Tuple(t) => t.types.iter().any(|t| needs_deallocate(resolve, t, what)),
            TypeDefKind::Variant(t) => t
                .cases
                .iter()
                .filter_map(|t| t.ty.as_ref())
                .any(|t| needs_deallocate(resolve, t, what)),
            TypeDefKind::Option(t) => needs_deallocate(resolve, t, what),
            TypeDefKind::Result(t) => [&t.ok, &t.err]
                .iter()
                .filter_map(|t| t.as_ref())
                .any(|t| needs_deallocate(resolve, t, what)),
            TypeDefKind::Flags(_) | TypeDefKind::Enum(_) => false,
            TypeDefKind::Future(_) | TypeDefKind::Stream(_) => what.handles(),
            TypeDefKind::Unknown => unreachable!(),
            TypeDefKind::FixedSizeList(..) => todo!(),
            TypeDefKind::Map(..) => todo!(),
        },

        Type::Bool
        | Type::U8
        | Type::S8
        | Type::U16
        | Type::S16
        | Type::U32
        | Type::S32
        | Type::U64
        | Type::S64
        | Type::F32
        | Type::F64
        | Type::Char => false,
    }
}

/// Generate instructions in `bindgen` to deallocate all lists in `ptr` where
/// that's a pointer to a sequence of `types` stored in linear memory.
pub fn deallocate_lists_in_types<B: Bindgen>(
    resolve: &Resolve,
    types: &[Type],
    operands: &[B::Operand],
    indirect: bool,
    bindgen: &mut B,
) {
    Generator::new(resolve, bindgen).deallocate_in_types(
        types,
        operands,
        indirect,
        Deallocate::Lists,
    );
}

/// Generate instructions in `bindgen` to deallocate all lists in `ptr` where
/// that's a pointer to a sequence of `types` stored in linear memory.
pub fn deallocate_lists_and_own_in_types<B: Bindgen>(
    resolve: &Resolve,
    types: &[Type],
    operands: &[B::Operand],
    indirect: bool,
    bindgen: &mut B,
) {
    Generator::new(resolve, bindgen).deallocate_in_types(
        types,
        operands,
        indirect,
        Deallocate::ListsAndOwn,
    );
}

#[derive(Copy, Clone)]
pub enum Realloc {
    None,
    Export(&'static str),
}

/// What to deallocate in various `deallocate_*` methods.
#[derive(Copy, Clone)]
enum Deallocate {
    /// Only deallocate lists.
    Lists,
    /// Deallocate lists and owned resources such as `own<T>` and
    /// futures/streams.
    ListsAndOwn,
}

impl Deallocate {
    fn handles(&self) -> bool {
        match self {
            Deallocate::Lists => false,
            Deallocate::ListsAndOwn => true,
        }
    }
}

struct Generator<'a, B: Bindgen> {
    bindgen: &'a mut B,
    resolve: &'a Resolve,
    operands: Vec<B::Operand>,
    results: Vec<B::Operand>,
    stack: Vec<B::Operand>,
    return_pointer: Option<B::Operand>,
    realloc: Option<Realloc>,
}

const MAX_FLAT_PARAMS: usize = 16;
const MAX_FLAT_ASYNC_PARAMS: usize = 4;

impl<'a, B: Bindgen> Generator<'a, B> {
    fn new(resolve: &'a Resolve, bindgen: &'a mut B) -> Generator<'a, B> {
        Generator {
            resolve,
            bindgen,
            operands: Vec::new(),
            results: Vec::new(),
            stack: Vec::new(),
            return_pointer: None,
            realloc: None,
        }
    }

    fn call(&mut self, func: &Function, variant: AbiVariant, lift_lower: LiftLower, async_: bool) {
        let sig = self.resolve.wasm_signature(variant, func);

        // Lowering parameters calling a wasm import _or_ returning a result
        // from an async-lifted wasm export means we don't need to pass
        // ownership, but we pass ownership in all other cases.
        let realloc = match (variant, lift_lower, async_) {
            (AbiVariant::GuestImport, LiftLower::LowerArgsLiftResults, _)
            | (
                AbiVariant::GuestExport
                | AbiVariant::GuestExportAsync
                | AbiVariant::GuestExportAsyncStackful,
                LiftLower::LiftArgsLowerResults,
                true,
            ) => Realloc::None,
            _ => Realloc::Export("cabi_realloc"),
        };
        assert!(self.realloc.is_none());

        match lift_lower {
            LiftLower::LowerArgsLiftResults => {
                self.realloc = Some(realloc);

                // Create a function that performs individual lowering of operands
                let lower_to_memory = |self_: &mut Self, ptr: B::Operand| {
                    let mut offset = ArchitectureSize::default();
                    for (nth, (_, ty)) in func.params.iter().enumerate() {
                        self_.emit(&Instruction::GetArg { nth });
                        offset = align_to_arch(offset, self_.bindgen.sizes().align(ty));
                        self_.write_to_memory(ty, ptr.clone(), offset);
                        offset += self_.bindgen.sizes().size(ty);
                    }

                    self_.stack.push(ptr);
                };

                // Lower parameters
                if sig.indirect_params {
                    // If parameters are indirect space is
                    // allocated for them and each argument is lowered
                    // individually into memory.
                    let ElementInfo { size, align } = self
                        .bindgen
                        .sizes()
                        .record(func.params.iter().map(|t| &t.1));

                    // Resolve the pointer to the indirectly stored parameters
                    let ptr = match variant {
                        // When a wasm module calls an import it will provide
                        // space that isn't explicitly deallocated.
                        AbiVariant::GuestImport => self.bindgen.return_pointer(size, align),

                        AbiVariant::GuestImportAsync => {
                            todo!("direct param lowering for async guest import not implemented")
                        }

                        // When calling a wasm module from the outside, though,
                        // malloc needs to be called.
                        AbiVariant::GuestExport => {
                            self.emit(&Instruction::Malloc {
                                realloc: "cabi_realloc",
                                size,
                                align,
                            });
                            self.stack.pop().unwrap()
                        }

                        AbiVariant::GuestExportAsync | AbiVariant::GuestExportAsyncStackful => {
                            todo!("direct param lowering for async not implemented")
                        }
                    };

                    // Lower the parameters to memory
                    lower_to_memory(self, ptr);
                } else {
                    // ... otherwise arguments are direct,
                    // (there aren't too many) then we simply do a normal lower
                    // operation for them all.
                    for (nth, (_, ty)) in func.params.iter().enumerate() {
                        self.emit(&Instruction::GetArg { nth });
                        self.lower(ty);
                    }
                }
                self.realloc = None;

                // If necessary we may need to prepare a return pointer for this ABI.
                if variant == AbiVariant::GuestImport && sig.retptr {
                    let info = self.bindgen.sizes().params(&func.result);
                    let ptr = self.bindgen.return_pointer(info.size, info.align);
                    self.return_pointer = Some(ptr.clone());
                    self.stack.push(ptr);
                }

                // Call the Wasm function
                assert_eq!(self.stack.len(), sig.params.len());
                self.emit(&Instruction::CallWasm {
                    name: &func.name,
                    sig: &sig,
                });

                // Handle the result
                if sig.retptr {
                    // If there is a return pointer, we must get the pointer to where results
                    // should be stored, and store the results there?

                    let ptr = match variant {
                        // imports into guests means it's a wasm module
                        // calling an imported function. We supplied the
                        // return pointer as the last argument (saved in
                        // `self.return_pointer`) so we use that to read
                        // the result of the function from memory.
                        AbiVariant::GuestImport => {
                            assert!(sig.results.is_empty());
                            self.return_pointer.take().unwrap()
                        }

                        // guest exports means that this is a host
                        // calling wasm so wasm returned a pointer to where
                        // the result is stored
                        AbiVariant::GuestExport => self.stack.pop().unwrap(),

                        AbiVariant::GuestImportAsync
                        | AbiVariant::GuestExportAsync
                        | AbiVariant::GuestExportAsyncStackful => {
                            unreachable!()
                        }
                    };

                    if let (AbiVariant::GuestExport, true) = (variant, async_) {
                        // If we're dealing with an async function, the result should not be read from memory
                        // immediately, as it's the async call result
                        //
                        // We can leave the result of the call (the indication of what to do as an async call)
                        // on the stack as a return
                        self.stack.push(ptr);
                    } else {
                        // If we're not dealing with an async call, the result must be in memory at this point and can be read out
                        self.read_results_from_memory(
                            &func.result,
                            ptr.clone(),
                            ArchitectureSize::default(),
                        );
                        self.emit(&Instruction::Flush {
                            amt: usize::from(func.result.is_some()),
                        });
                    }
                } else {
                    // With no return pointer in use we can simply lift the
                    // result(s) of the function from the result of the core
                    // wasm function.
                    if let Some(ty) = &func.result {
                        self.lift(ty)
                    }
                }

                // Emit the function return
                self.emit(&Instruction::Return {
                    func,
                    amt: usize::from(func.result.is_some()),
                });
            }

            LiftLower::LiftArgsLowerResults => {
                let max_flat_params = match (variant, async_) {
                    (AbiVariant::GuestImportAsync, _is_async @ true) => MAX_FLAT_ASYNC_PARAMS,
                    _ => MAX_FLAT_PARAMS,
                };

                // Read parameters from memory
                let read_from_memory = |self_: &mut Self| {
                    let mut offset = ArchitectureSize::default();
                    let ptr = self_
                        .stack
                        .pop()
                        .expect("empty stack during read param from memory");
                    for (_, ty) in func.params.iter() {
                        offset = align_to_arch(offset, self_.bindgen.sizes().align(ty));
                        self_.read_from_memory(ty, ptr.clone(), offset);
                        offset += self_.bindgen.sizes().size(ty);
                    }
                };

                // Resolve parameters
                if sig.indirect_params {
                    // If parameters were passed indirectly, arguments must be
                    // read in succession from memory, with the pointer to the arguments
                    // being the first argument to the function.
                    self.emit(&Instruction::GetArg { nth: 0 });
                    read_from_memory(self);
                } else {
                    // ... otherwise, if parameters were passed directly then we lift each
                    // argument in succession from the component wasm types that
                    // make-up the type.
                    let mut offset = 0;
                    for (param_name, ty) in func.params.iter() {
                        let Some(types) = flat_types(self.resolve, ty, Some(max_flat_params))
                        else {
                            panic!(
                                "failed to flatten types during direct parameter lifting ('{param_name}' in func '{}')",
                                func.name
                            );
                        };
                        for _ in 0..types.len() {
                            self.emit(&Instruction::GetArg { nth: offset });
                            offset += 1;
                        }
                        self.lift(ty);
                    }
                }

                // ... and that allows us to call the interface types function
                self.emit(&Instruction::CallInterface { func, async_ });

                // The return value of an async function is *not* the result of the function
                // itself or a pointer but rather a status code.
                //
                // Asynchronous functions will call `task.return` after the
                // interface function completes, so lowering is conditional
                // based on slightly different logic for the `task.return`
                // intrinsic.
                //
                // Note that in the async import case teh code below deals with the CM function being lowered,
                // not the core function that is underneath that (i.e. func.result may be empty,
                // where the associated core function underneath must have a i32 status code result)
                let (lower_to_memory, async_flat_results) = match (async_, &func.result) {
                    // All async cases pass along the function results and flatten where necesary
                    (_is_async @ true, func_result) => {
                        let results = match &func_result {
                            Some(ty) => flat_types(self.resolve, ty, Some(max_flat_params)),
                            None => Some(Vec::new()),
                        };
                        (results.is_none(), Some(results))
                    }
                    // All other non-async cases
                    (_is_async @ false, _) => (sig.retptr, None),
                };

                // This was dynamically allocated by the caller (or async start
                // function) so after it's been read by the guest we need to
                // deallocate it.
                if let AbiVariant::GuestExport
                | AbiVariant::GuestExportAsync
                | AbiVariant::GuestExportAsyncStackful = variant
                {
                    if sig.indirect_params && !async_ {
                        let ElementInfo { size, align } = self
                            .bindgen
                            .sizes()
                            .record(func.params.iter().map(|t| &t.1));
                        self.emit(&Instruction::GetArg { nth: 0 });
                        self.emit(&Instruction::GuestDeallocate { size, align });
                    }
                }

                self.realloc = Some(realloc);

                // Perform memory lowing of relevant results, including out pointers as well as traditional results
                match (lower_to_memory, sig.retptr, variant) {
                    // For sync calls, if no lowering to memory is required and there *is* a return pointer in use
                    // then we need to lower then simply lower the result(s) and return that directly from the function.
                    (_lower_to_memory @ false, _, _) => {
                        if let Some(ty) = &func.result {
                            self.lower(ty);
                        }
                    }

                    // Lowering to memory for a guest import
                    //
                    // When a function is imported to a guest this means
                    // it's a host providing the implementation of the
                    // import. The result is stored in the pointer
                    // specified in the last argument, so we get the
                    // pointer here and then write the return value into
                    // it.
                    (
                        _lower_to_memory @ true,
                        _has_ret_ptr @ true,
                        AbiVariant::GuestImport | AbiVariant::GuestImportAsync,
                    ) => {
                        self.emit(&Instruction::GetArg {
                            nth: sig.params.len() - 1,
                        });
                        let ptr = self
                            .stack
                            .pop()
                            .expect("empty stack during result lower to memory");
                        self.write_params_to_memory(&func.result, ptr, Default::default());
                    }

                    // Lowering to memory for a guest export
                    //
                    // For a guest import this is a function defined in
                    // wasm, so we're returning a pointer where the
                    // value was stored at. Allocate some space here
                    // (statically) and then write the result into that
                    // memory, returning the pointer at the end.
                    (_lower_to_memory @ true, _, variant) => match variant {
                        AbiVariant::GuestExport | AbiVariant::GuestExportAsync => {
                            let ElementInfo { size, align } =
                                self.bindgen.sizes().params(&func.result);
                            let ptr = self.bindgen.return_pointer(size, align);
                            self.write_params_to_memory(
                                &func.result,
                                ptr.clone(),
                                Default::default(),
                            );
                            self.stack.push(ptr);
                        }
                        AbiVariant::GuestImport | AbiVariant::GuestImportAsync => {
                            unreachable!(
                                "lowering to memory cannot be performed without a return pointer ({async_note} func [{func_name}], variant {variant:#?})",
                                async_note = async_.then_some("async").unwrap_or("sync"),
                                func_name = func.name,
                            )
                        }
                        AbiVariant::GuestExportAsyncStackful => {
                            todo!("stackful exports are not yet supported")
                        }
                    },
                }

                // Build and emit the appropriate return
                match (variant, async_flat_results) {
                    // Async guest imports always return a i32 status code
                    (AbiVariant::GuestImport | AbiVariant::GuestImportAsync, None) if async_ => {
                        unreachable!("async guest imports must have a return")
                    }

                    // Async guest imports with results return the status code, not a pointer to any results
                    (AbiVariant::GuestImport | AbiVariant::GuestImportAsync, Some(results))
                        if async_ =>
                    {
                        let name = &format!("[task-return]{}", func.name);
                        let params = results.as_deref().unwrap_or_default();
                        self.emit(&Instruction::AsyncTaskReturn { name, params });
                    }

                    // All async/non-async cases with results that need to be returned
                    //
                    // In practice, async imports should not end up here, as the returned result of an
                    // async import is *not* a pointer but instead a status code.
                    (_, Some(results)) => {
                        let name = &format!("[task-return]{}", func.name);
                        let params = results.as_deref().unwrap_or(&[WasmType::Pointer]);
                        self.emit(&Instruction::AsyncTaskReturn { name, params });
                    }

                    // All async/non-async cases with no results
                    (_, None) => {
                        if async_ {
                            let name = &format!("[task-return]{}", func.name);
                            self.emit(&Instruction::AsyncTaskReturn {
                                name: name,
                                params: if sig.results.len() > MAX_FLAT_ASYNC_PARAMS {
                                    &[WasmType::Pointer]
                                } else {
                                    &sig.results
                                },
                            });
                        } else {
                            self.emit(&Instruction::Return {
                                func,
                                amt: sig.results.len(),
                            });
                        }
                    }
                }

                self.realloc = None;
            }
        }

        assert!(self.realloc.is_none());

        assert!(
            self.stack.is_empty(),
            "stack has {} items remaining: {:?}",
            self.stack.len(),
            self.stack,
        );
    }

    fn post_return(&mut self, func: &Function) {
        let sig = self.resolve.wasm_signature(AbiVariant::GuestExport, func);

        // Currently post-return is only used for lists and lists are always
        // returned indirectly through memory due to their flat representation
        // having more than one type. Assert that a return pointer is used,
        // though, in case this ever changes.
        assert!(sig.retptr);

        self.emit(&Instruction::GetArg { nth: 0 });
        let addr = self.stack.pop().unwrap();

        let mut types = Vec::new();
        types.extend(func.result);
        self.deallocate_in_types(&types, &[addr], true, Deallocate::Lists);

        self.emit(&Instruction::Return { func, amt: 0 });
    }

    fn deallocate_in_types(
        &mut self,
        types: &[Type],
        operands: &[B::Operand],
        indirect: bool,
        what: Deallocate,
    ) {
        if indirect {
            assert_eq!(operands.len(), 1);
            for (offset, ty) in self.bindgen.sizes().field_offsets(types) {
                self.deallocate_indirect(ty, operands[0].clone(), offset, what);
            }
            assert!(
                self.stack.is_empty(),
                "stack has {} items remaining",
                self.stack.len()
            );
        } else {
            let mut operands = operands;
            let mut operands_for_ty;
            for ty in types {
                let types = flat_types(self.resolve, ty, None).unwrap();
                (operands_for_ty, operands) = operands.split_at(types.len());
                self.stack.extend_from_slice(operands_for_ty);
                self.deallocate(ty, what);
                assert!(
                    self.stack.is_empty(),
                    "stack has {} items remaining",
                    self.stack.len()
                );
            }
            assert!(operands.is_empty());
        }
    }

    fn emit(&mut self, inst: &Instruction<'_>) {
        self.operands.clear();
        self.results.clear();

        let operands_len = inst.operands_len();
        assert!(
            self.stack.len() >= operands_len,
            "not enough operands on stack for {:?}: have {} need {operands_len}",
            inst,
            self.stack.len(),
        );
        self.operands
            .extend(self.stack.drain((self.stack.len() - operands_len)..));
        self.results.reserve(inst.results_len());

        self.bindgen
            .emit(self.resolve, inst, &mut self.operands, &mut self.results);

        assert_eq!(
            self.results.len(),
            inst.results_len(),
            "{:?} expected {} results, got {}",
            inst,
            inst.results_len(),
            self.results.len()
        );
        self.stack.append(&mut self.results);
    }

    fn push_block(&mut self) {
        self.bindgen.push_block();
    }

    fn finish_block(&mut self, size: usize) {
        self.operands.clear();
        assert!(
            size <= self.stack.len(),
            "not enough operands on stack for finishing block",
        );
        self.operands
            .extend(self.stack.drain((self.stack.len() - size)..));
        self.bindgen.finish_block(&mut self.operands);
    }

    fn lower(&mut self, ty: &Type) {
        use Instruction::*;

        match *ty {
            Type::Bool => self.emit(&I32FromBool),
            Type::S8 => self.emit(&I32FromS8),
            Type::U8 => self.emit(&I32FromU8),
            Type::S16 => self.emit(&I32FromS16),
            Type::U16 => self.emit(&I32FromU16),
            Type::S32 => self.emit(&I32FromS32),
            Type::U32 => self.emit(&I32FromU32),
            Type::S64 => self.emit(&I64FromS64),
            Type::U64 => self.emit(&I64FromU64),
            Type::Char => self.emit(&I32FromChar),
            Type::F32 => self.emit(&CoreF32FromF32),
            Type::F64 => self.emit(&CoreF64FromF64),
            Type::String => {
                let realloc = self.list_realloc();
                self.emit(&StringLower { realloc });
            }
            Type::ErrorContext => self.emit(&ErrorContextLower),
            Type::Id(id) => match &self.resolve.types[id].kind {
                TypeDefKind::Type(t) => self.lower(t),
                TypeDefKind::List(element) => {
                    let realloc = self.list_realloc();
                    if self.bindgen.is_list_canonical(self.resolve, element) {
                        self.emit(&ListCanonLower { element, realloc });
                    } else {
                        self.push_block();
                        self.emit(&IterElem { element });
                        self.emit(&IterBasePointer);
                        let addr = self.stack.pop().unwrap();
                        self.write_to_memory(element, addr, Default::default());
                        self.finish_block(0);
                        self.emit(&ListLower { element, realloc });
                    }
                }
                TypeDefKind::Handle(handle) => {
                    let (Handle::Own(ty) | Handle::Borrow(ty)) = handle;
                    self.emit(&HandleLower {
                        handle,
                        ty: id,
                        name: self.resolve.types[*ty].name.as_deref().unwrap(),
                    });
                }
                TypeDefKind::Resource => {
                    todo!();
                }
                TypeDefKind::Record(record) => {
                    self.emit(&RecordLower {
                        record,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                    let values = self
                        .stack
                        .drain(self.stack.len() - record.fields.len()..)
                        .collect::<Vec<_>>();
                    for (field, value) in record.fields.iter().zip(values) {
                        self.stack.push(value);
                        self.lower(&field.ty);
                    }
                }
                TypeDefKind::Tuple(tuple) => {
                    self.emit(&TupleLower { tuple, ty: id });
                    let values = self
                        .stack
                        .drain(self.stack.len() - tuple.types.len()..)
                        .collect::<Vec<_>>();
                    for (ty, value) in tuple.types.iter().zip(values) {
                        self.stack.push(value);
                        self.lower(ty);
                    }
                }

                TypeDefKind::Flags(flags) => {
                    self.emit(&FlagsLower {
                        flags,
                        ty: id,
                        name: self.resolve.types[id].name.as_ref().unwrap(),
                    });
                }

                TypeDefKind::Variant(v) => {
                    let results =
                        self.lower_variant_arms(ty, v.cases.iter().map(|c| c.ty.as_ref()));
                    self.emit(&VariantLower {
                        variant: v,
                        ty: id,
                        results: &results,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }
                TypeDefKind::Enum(enum_) => {
                    self.emit(&EnumLower {
                        enum_,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }
                TypeDefKind::Option(t) => {
                    let results = self.lower_variant_arms(ty, [None, Some(t)]);
                    self.emit(&OptionLower {
                        payload: t,
                        ty: id,
                        results: &results,
                    });
                }
                TypeDefKind::Result(r) => {
                    let results = self.lower_variant_arms(ty, [r.ok.as_ref(), r.err.as_ref()]);
                    self.emit(&ResultLower {
                        result: r,
                        ty: id,
                        results: &results,
                    });
                }
                TypeDefKind::Future(ty) => {
                    self.emit(&FutureLower {
                        payload: ty,
                        ty: id,
                    });
                }
                TypeDefKind::Stream(ty) => {
                    self.emit(&StreamLower {
                        payload: ty,
                        ty: id,
                    });
                }
                TypeDefKind::Unknown => unreachable!(),
                TypeDefKind::FixedSizeList(..) => todo!(),
                TypeDefKind::Map(..) => todo!(),
            },
        }
    }

    fn lower_variant_arms<'b>(
        &mut self,
        ty: &Type,
        cases: impl IntoIterator<Item = Option<&'b Type>>,
    ) -> Vec<WasmType> {
        use Instruction::*;
        let results = flat_types(self.resolve, ty, None).unwrap();
        let mut casts = Vec::new();
        for (i, ty) in cases.into_iter().enumerate() {
            self.push_block();
            self.emit(&VariantPayloadName);
            let payload_name = self.stack.pop().unwrap();
            self.emit(&I32Const { val: i as i32 });
            let mut pushed = 1;
            if let Some(ty) = ty {
                // Using the payload of this block we lower the type to
                // raw wasm values.
                self.stack.push(payload_name);
                self.lower(ty);

                // Determine the types of all the wasm values we just
                // pushed, and record how many. If we pushed too few
                // then we'll need to push some zeros after this.
                let temp = flat_types(self.resolve, ty, None).unwrap();
                pushed += temp.len();

                // For all the types pushed we may need to insert some
                // bitcasts. This will go through and cast everything
                // to the right type to ensure all blocks produce the
                // same set of results.
                casts.truncate(0);
                for (actual, expected) in temp.iter().zip(&results[1..]) {
                    casts.push(cast(*actual, *expected));
                }
                if casts.iter().any(|c| *c != Bitcast::None) {
                    self.emit(&Bitcasts { casts: &casts });
                }
            }

            // If we haven't pushed enough items in this block to match
            // what other variants are pushing then we need to push
            // some zeros.
            if pushed < results.len() {
                self.emit(&ConstZero {
                    tys: &results[pushed..],
                });
            }
            self.finish_block(results.len());
        }
        results
    }

    fn list_realloc(&self) -> Option<&'static str> {
        match self.realloc.expect("realloc should be configured") {
            Realloc::None => None,
            Realloc::Export(s) => Some(s),
        }
    }

    /// Note that in general everything in this function is the opposite of the
    /// `lower` function above. This is intentional and should be kept this way!
    fn lift(&mut self, ty: &Type) {
        use Instruction::*;

        match *ty {
            Type::Bool => self.emit(&BoolFromI32),
            Type::S8 => self.emit(&S8FromI32),
            Type::U8 => self.emit(&U8FromI32),
            Type::S16 => self.emit(&S16FromI32),
            Type::U16 => self.emit(&U16FromI32),
            Type::S32 => self.emit(&S32FromI32),
            Type::U32 => self.emit(&U32FromI32),
            Type::S64 => self.emit(&S64FromI64),
            Type::U64 => self.emit(&U64FromI64),
            Type::Char => self.emit(&CharFromI32),
            Type::F32 => self.emit(&F32FromCoreF32),
            Type::F64 => self.emit(&F64FromCoreF64),
            Type::String => self.emit(&StringLift),
            Type::ErrorContext => self.emit(&ErrorContextLift),
            Type::Id(id) => match &self.resolve.types[id].kind {
                TypeDefKind::Type(t) => self.lift(t),
                TypeDefKind::List(element) => {
                    if self.bindgen.is_list_canonical(self.resolve, element) {
                        self.emit(&ListCanonLift { element, ty: id });
                    } else {
                        self.push_block();
                        self.emit(&IterBasePointer);
                        let addr = self.stack.pop().unwrap();
                        self.read_from_memory(element, addr, Default::default());
                        self.finish_block(1);
                        self.emit(&ListLift { element, ty: id });
                    }
                }
                TypeDefKind::Handle(handle) => {
                    let (Handle::Own(ty) | Handle::Borrow(ty)) = handle;
                    self.emit(&HandleLift {
                        handle,
                        ty: id,
                        name: self.resolve.types[*ty].name.as_deref().unwrap(),
                    });
                }
                TypeDefKind::Resource => {
                    todo!();
                }
                TypeDefKind::Record(record) => {
                    self.flat_for_each_record_type(
                        ty,
                        record.fields.iter().map(|f| &f.ty),
                        Self::lift,
                    );
                    self.emit(&RecordLift {
                        record,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }
                TypeDefKind::Tuple(tuple) => {
                    self.flat_for_each_record_type(ty, tuple.types.iter(), Self::lift);
                    self.emit(&TupleLift { tuple, ty: id });
                }
                TypeDefKind::Flags(flags) => {
                    self.emit(&FlagsLift {
                        flags,
                        ty: id,
                        name: self.resolve.types[id].name.as_ref().unwrap(),
                    });
                }

                TypeDefKind::Variant(v) => {
                    self.flat_for_each_variant_arm(
                        ty,
                        true,
                        v.cases.iter().map(|c| c.ty.as_ref()),
                        Self::lift,
                    );
                    self.emit(&VariantLift {
                        variant: v,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }

                TypeDefKind::Enum(enum_) => {
                    self.emit(&EnumLift {
                        enum_,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }

                TypeDefKind::Option(t) => {
                    self.flat_for_each_variant_arm(ty, true, [None, Some(t)], Self::lift);
                    self.emit(&OptionLift { payload: t, ty: id });
                }

                TypeDefKind::Result(r) => {
                    self.flat_for_each_variant_arm(
                        ty,
                        true,
                        [r.ok.as_ref(), r.err.as_ref()],
                        Self::lift,
                    );
                    self.emit(&ResultLift { result: r, ty: id });
                }

                TypeDefKind::Future(ty) => {
                    self.emit(&FutureLift {
                        payload: ty,
                        ty: id,
                    });
                }
                TypeDefKind::Stream(ty) => {
                    self.emit(&StreamLift {
                        payload: ty,
                        ty: id,
                    });
                }
                TypeDefKind::Unknown => unreachable!(),
                TypeDefKind::FixedSizeList(..) => todo!(),
                TypeDefKind::Map(..) => todo!(),
            },
        }
    }

    fn flat_for_each_record_type<'b>(
        &mut self,
        container: &Type,
        types: impl Iterator<Item = &'b Type>,
        mut iter: impl FnMut(&mut Self, &Type),
    ) {
        let temp = flat_types(self.resolve, container, None).unwrap();
        let mut args = self
            .stack
            .drain(self.stack.len() - temp.len()..)
            .collect::<Vec<_>>();
        for ty in types {
            let temp = flat_types(self.resolve, ty, None).unwrap();
            self.stack.extend(args.drain(..temp.len()));
            iter(self, ty);
        }
    }

    fn flat_for_each_variant_arm<'b>(
        &mut self,
        ty: &Type,
        blocks_with_type_have_result: bool,
        cases: impl IntoIterator<Item = Option<&'b Type>>,
        mut iter: impl FnMut(&mut Self, &Type),
    ) {
        let params = flat_types(self.resolve, ty, None).unwrap();
        let mut casts = Vec::new();
        let block_inputs = self
            .stack
            .drain(self.stack.len() + 1 - params.len()..)
            .collect::<Vec<_>>();
        for ty in cases {
            self.push_block();
            if let Some(ty) = ty {
                // Push only the values we need for this variant onto
                // the stack.
                let temp = flat_types(self.resolve, ty, None).unwrap();
                self.stack
                    .extend(block_inputs[..temp.len()].iter().cloned());

                // Cast all the types we have on the stack to the actual
                // types needed for this variant, if necessary.
                casts.truncate(0);
                for (actual, expected) in temp.iter().zip(&params[1..]) {
                    casts.push(cast(*expected, *actual));
                }
                if casts.iter().any(|c| *c != Bitcast::None) {
                    self.emit(&Instruction::Bitcasts { casts: &casts });
                }

                // Then recursively lift this variant's payload.
                iter(self, ty);
            }
            self.finish_block(if blocks_with_type_have_result {
                ty.is_some() as usize
            } else {
                0
            });
        }
    }

    fn write_to_memory(&mut self, ty: &Type, addr: B::Operand, offset: ArchitectureSize) {
        use Instruction::*;

        match *ty {
            // Builtin types need different flavors of storage instructions
            // depending on the size of the value written.
            Type::Bool | Type::U8 | Type::S8 => {
                self.lower_and_emit(ty, addr, &I32Store8 { offset })
            }
            Type::U16 | Type::S16 => self.lower_and_emit(ty, addr, &I32Store16 { offset }),
            Type::U32 | Type::S32 | Type::Char => {
                self.lower_and_emit(ty, addr, &I32Store { offset })
            }
            Type::U64 | Type::S64 => self.lower_and_emit(ty, addr, &I64Store { offset }),
            Type::F32 => self.lower_and_emit(ty, addr, &F32Store { offset }),
            Type::F64 => self.lower_and_emit(ty, addr, &F64Store { offset }),
            Type::String => self.write_list_to_memory(ty, addr, offset),
            Type::ErrorContext => self.lower_and_emit(ty, addr, &I32Store { offset }),

            Type::Id(id) => match &self.resolve.types[id].kind {
                TypeDefKind::Type(t) => self.write_to_memory(t, addr, offset),
                TypeDefKind::List(_) => self.write_list_to_memory(ty, addr, offset),

                TypeDefKind::Future(_) | TypeDefKind::Stream(_) | TypeDefKind::Handle(_) => {
                    self.lower_and_emit(ty, addr, &I32Store { offset })
                }

                // Decompose the record into its components and then write all
                // the components into memory one-by-one.
                TypeDefKind::Record(record) => {
                    self.emit(&RecordLower {
                        record,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                    self.write_fields_to_memory(record.fields.iter().map(|f| &f.ty), addr, offset);
                }
                TypeDefKind::Resource => {
                    todo!()
                }
                TypeDefKind::Tuple(tuple) => {
                    self.emit(&TupleLower { tuple, ty: id });
                    self.write_fields_to_memory(tuple.types.iter(), addr, offset);
                }

                TypeDefKind::Flags(f) => {
                    self.lower(ty);
                    match f.repr() {
                        FlagsRepr::U8 => {
                            self.stack.push(addr);
                            self.store_intrepr(offset, Int::U8);
                        }
                        FlagsRepr::U16 => {
                            self.stack.push(addr);
                            self.store_intrepr(offset, Int::U16);
                        }
                        FlagsRepr::U32(n) => {
                            for i in (0..n).rev() {
                                self.stack.push(addr.clone());
                                self.emit(&I32Store {
                                    offset: offset.add_bytes(i * 4),
                                });
                            }
                        }
                    }
                }

                // Each case will get its own block, and the first item in each
                // case is writing the discriminant. After that if we have a
                // payload we write the payload after the discriminant, aligned up
                // to the type's alignment.
                TypeDefKind::Variant(v) => {
                    self.write_variant_arms_to_memory(
                        offset,
                        addr,
                        v.tag(),
                        v.cases.iter().map(|c| c.ty.as_ref()),
                    );
                    self.emit(&VariantLower {
                        variant: v,
                        ty: id,
                        results: &[],
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }

                TypeDefKind::Option(t) => {
                    self.write_variant_arms_to_memory(offset, addr, Int::U8, [None, Some(t)]);
                    self.emit(&OptionLower {
                        payload: t,
                        ty: id,
                        results: &[],
                    });
                }

                TypeDefKind::Result(r) => {
                    self.write_variant_arms_to_memory(
                        offset,
                        addr,
                        Int::U8,
                        [r.ok.as_ref(), r.err.as_ref()],
                    );
                    self.emit(&ResultLower {
                        result: r,
                        ty: id,
                        results: &[],
                    });
                }

                TypeDefKind::Enum(e) => {
                    self.lower(ty);
                    self.stack.push(addr);
                    self.store_intrepr(offset, e.tag());
                }

                TypeDefKind::Unknown => unreachable!(),
                TypeDefKind::FixedSizeList(..) => todo!(),
                TypeDefKind::Map(..) => todo!(),
            },
        }
    }

    fn write_params_to_memory<'b>(
        &mut self,
        params: impl IntoIterator<Item = &'b Type, IntoIter: ExactSizeIterator>,
        addr: B::Operand,
        offset: ArchitectureSize,
    ) {
        self.write_fields_to_memory(params, addr, offset);
    }

    fn write_variant_arms_to_memory<'b>(
        &mut self,
        offset: ArchitectureSize,
        addr: B::Operand,
        tag: Int,
        cases: impl IntoIterator<Item = Option<&'b Type>> + Clone,
    ) {
        let payload_offset = offset + (self.bindgen.sizes().payload_offset(tag, cases.clone()));
        for (i, ty) in cases.into_iter().enumerate() {
            self.push_block();
            self.emit(&Instruction::VariantPayloadName);
            let payload_name = self.stack.pop().unwrap();
            self.emit(&Instruction::I32Const { val: i as i32 });
            self.stack.push(addr.clone());
            self.store_intrepr(offset, tag);
            if let Some(ty) = ty {
                self.stack.push(payload_name.clone());
                self.write_to_memory(ty, addr.clone(), payload_offset);
            }
            self.finish_block(0);
        }
    }

    fn write_list_to_memory(&mut self, ty: &Type, addr: B::Operand, offset: ArchitectureSize) {
        // After lowering the list there's two i32 values on the stack
        // which we write into memory, writing the pointer into the low address
        // and the length into the high address.
        self.lower(ty);
        self.stack.push(addr.clone());
        self.emit(&Instruction::LengthStore {
            offset: offset + self.bindgen.sizes().align(ty).into(),
        });
        self.stack.push(addr);
        self.emit(&Instruction::PointerStore { offset });
    }

    fn write_fields_to_memory<'b>(
        &mut self,
        tys: impl IntoIterator<Item = &'b Type, IntoIter: ExactSizeIterator>,
        addr: B::Operand,
        offset: ArchitectureSize,
    ) {
        let tys = tys.into_iter();
        let fields = self
            .stack
            .drain(self.stack.len() - tys.len()..)
            .collect::<Vec<_>>();
        for ((field_offset, ty), op) in self
            .bindgen
            .sizes()
            .field_offsets(tys)
            .into_iter()
            .zip(fields)
        {
            self.stack.push(op);
            self.write_to_memory(ty, addr.clone(), offset + (field_offset));
        }
    }

    fn lower_and_emit(&mut self, ty: &Type, addr: B::Operand, instr: &Instruction) {
        self.lower(ty);
        self.stack.push(addr);
        self.emit(instr);
    }

    fn read_from_memory(&mut self, ty: &Type, addr: B::Operand, offset: ArchitectureSize) {
        use Instruction::*;

        match *ty {
            Type::Bool => self.emit_and_lift(ty, addr, &I32Load8U { offset }),
            Type::U8 => self.emit_and_lift(ty, addr, &I32Load8U { offset }),
            Type::S8 => self.emit_and_lift(ty, addr, &I32Load8S { offset }),
            Type::U16 => self.emit_and_lift(ty, addr, &I32Load16U { offset }),
            Type::S16 => self.emit_and_lift(ty, addr, &I32Load16S { offset }),
            Type::U32 | Type::S32 | Type::Char => self.emit_and_lift(ty, addr, &I32Load { offset }),
            Type::U64 | Type::S64 => self.emit_and_lift(ty, addr, &I64Load { offset }),
            Type::F32 => self.emit_and_lift(ty, addr, &F32Load { offset }),
            Type::F64 => self.emit_and_lift(ty, addr, &F64Load { offset }),
            Type::String => self.read_list_from_memory(ty, addr, offset),
            Type::ErrorContext => self.emit_and_lift(ty, addr, &I32Load { offset }),

            Type::Id(id) => match &self.resolve.types[id].kind {
                TypeDefKind::Type(t) => self.read_from_memory(t, addr, offset),

                TypeDefKind::List(_) => self.read_list_from_memory(ty, addr, offset),

                TypeDefKind::Future(_) | TypeDefKind::Stream(_) | TypeDefKind::Handle(_) => {
                    self.emit_and_lift(ty, addr, &I32Load { offset })
                }

                TypeDefKind::Resource => {
                    todo!();
                }

                // Read and lift each field individually, adjusting the offset
                // as we go along, then aggregate all the fields into the
                // record.
                TypeDefKind::Record(record) => {
                    self.read_fields_from_memory(record.fields.iter().map(|f| &f.ty), addr, offset);
                    self.emit(&RecordLift {
                        record,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }

                TypeDefKind::Tuple(tuple) => {
                    self.read_fields_from_memory(&tuple.types, addr, offset);
                    self.emit(&TupleLift { tuple, ty: id });
                }

                TypeDefKind::Flags(f) => {
                    match f.repr() {
                        FlagsRepr::U8 => {
                            self.stack.push(addr);
                            self.load_intrepr(offset, Int::U8);
                        }
                        FlagsRepr::U16 => {
                            self.stack.push(addr);
                            self.load_intrepr(offset, Int::U16);
                        }
                        FlagsRepr::U32(n) => {
                            for i in 0..n {
                                self.stack.push(addr.clone());
                                self.emit(&I32Load {
                                    offset: offset.add_bytes(i * 4),
                                });
                            }
                        }
                    }
                    self.lift(ty);
                }

                // Each case will get its own block, and we'll dispatch to the
                // right block based on the `i32.load` we initially perform. Each
                // individual block is pretty simple and just reads the payload type
                // from the corresponding offset if one is available.
                TypeDefKind::Variant(variant) => {
                    self.read_variant_arms_from_memory(
                        offset,
                        addr,
                        variant.tag(),
                        variant.cases.iter().map(|c| c.ty.as_ref()),
                    );
                    self.emit(&VariantLift {
                        variant,
                        ty: id,
                        name: self.resolve.types[id].name.as_deref().unwrap(),
                    });
                }

                TypeDefKind::Option(t) => {
                    self.read_variant_arms_from_memory(offset, addr, Int::U8, [None, Some(t)]);
                    self.emit(&OptionLift { payload: t, ty: id });
                }

                TypeDefKind::Result(r) => {
                    self.read_variant_arms_from_memory(
                        offset,
                        addr,
                        Int::U8,
                        [r.ok.as_ref(), r.err.as_ref()],
                    );
                    self.emit(&ResultLift { result: r, ty: id });
                }

                TypeDefKind::Enum(e) => {
                    self.stack.push(addr.clone());
                    self.load_intrepr(offset, e.tag());
                    self.lift(ty);
                }

                TypeDefKind::Unknown => unreachable!(),
                TypeDefKind::FixedSizeList(..) => todo!(),
                TypeDefKind::Map(..) => todo!(),
            },
        }
    }

    fn read_results_from_memory(
        &mut self,
        result: &Option<Type>,
        addr: B::Operand,
        offset: ArchitectureSize,
    ) {
        self.read_fields_from_memory(result, addr, offset)
    }

    fn read_variant_arms_from_memory<'b>(
        &mut self,
        offset: ArchitectureSize,
        addr: B::Operand,
        tag: Int,
        cases: impl IntoIterator<Item = Option<&'b Type>> + Clone,
    ) {
        self.stack.push(addr.clone());
        self.load_intrepr(offset, tag);
        let payload_offset = offset + (self.bindgen.sizes().payload_offset(tag, cases.clone()));
        for ty in cases {
            self.push_block();
            if let Some(ty) = ty {
                self.read_from_memory(ty, addr.clone(), payload_offset);
            }
            self.finish_block(ty.is_some() as usize);
        }
    }

    fn read_list_from_memory(&mut self, ty: &Type, addr: B::Operand, offset: ArchitectureSize) {
        // Read the pointer/len and then perform the standard lifting
        // proceses.
        self.stack.push(addr.clone());
        self.emit(&Instruction::PointerLoad { offset });
        self.stack.push(addr);
        self.emit(&Instruction::LengthLoad {
            offset: offset + self.bindgen.sizes().align(ty).into(),
        });
        self.lift(ty);
    }

    fn read_fields_from_memory<'b>(
        &mut self,
        tys: impl IntoIterator<Item = &'b Type>,
        addr: B::Operand,
        offset: ArchitectureSize,
    ) {
        for (field_offset, ty) in self.bindgen.sizes().field_offsets(tys).iter() {
            self.read_from_memory(ty, addr.clone(), offset + (*field_offset));
        }
    }

    fn emit_and_lift(&mut self, ty: &Type, addr: B::Operand, instr: &Instruction) {
        self.stack.push(addr);
        self.emit(instr);
        self.lift(ty);
    }

    fn load_intrepr(&mut self, offset: ArchitectureSize, repr: Int) {
        self.emit(&match repr {
            Int::U64 => Instruction::I64Load { offset },
            Int::U32 => Instruction::I32Load { offset },
            Int::U16 => Instruction::I32Load16U { offset },
            Int::U8 => Instruction::I32Load8U { offset },
        });
    }

    fn store_intrepr(&mut self, offset: ArchitectureSize, repr: Int) {
        self.emit(&match repr {
            Int::U64 => Instruction::I64Store { offset },
            Int::U32 => Instruction::I32Store { offset },
            Int::U16 => Instruction::I32Store16 { offset },
            Int::U8 => Instruction::I32Store8 { offset },
        });
    }

    /// Runs the deallocation of `ty` for the operands currently on
    /// `self.stack`.
    ///
    /// This will pop the ABI items of `ty` from `self.stack`.
    fn deallocate(&mut self, ty: &Type, what: Deallocate) {
        use Instruction::*;

        match *ty {
            Type::String => {
                self.emit(&Instruction::GuestDeallocateString);
            }

            Type::Bool
            | Type::U8
            | Type::S8
            | Type::U16
            | Type::S16
            | Type::U32
            | Type::S32
            | Type::Char
            | Type::U64
            | Type::S64
            | Type::F32
            | Type::F64
            | Type::ErrorContext => {
                // No deallocation necessary, just discard the operand on the
                // stack.
                self.stack.pop().unwrap();
            }

            Type::Id(id) => match &self.resolve.types[id].kind {
                TypeDefKind::Type(t) => self.deallocate(t, what),

                TypeDefKind::List(element) => {
                    self.push_block();
                    self.emit(&IterBasePointer);
                    let elemaddr = self.stack.pop().unwrap();
                    self.deallocate_indirect(element, elemaddr, Default::default(), what);
                    self.finish_block(0);

                    self.emit(&Instruction::GuestDeallocateList { element });
                }

                TypeDefKind::Handle(Handle::Own(_))
                | TypeDefKind::Future(_)
                | TypeDefKind::Stream(_)
                    if what.handles() =>
                {
                    self.lift(ty);
                    self.emit(&DropHandle { ty });
                }

                TypeDefKind::Record(record) => {
                    self.flat_for_each_record_type(
                        ty,
                        record.fields.iter().map(|f| &f.ty),
                        |me, ty| me.deallocate(ty, what),
                    );
                }

                TypeDefKind::Tuple(tuple) => {
                    self.flat_for_each_record_type(ty, tuple.types.iter(), |me, ty| {
                        me.deallocate(ty, what)
                    });
                }

                TypeDefKind::Variant(variant) => {
                    self.flat_for_each_variant_arm(
                        ty,
                        false,
                        variant.cases.iter().map(|c| c.ty.as_ref()),
                        |me, ty| me.deallocate(ty, what),
                    );
                    self.emit(&GuestDeallocateVariant {
                        blocks: variant.cases.len(),
                    });
                }

                TypeDefKind::Option(t) => {
                    self.flat_for_each_variant_arm(ty, false, [None, Some(t)], |me, ty| {
                        me.deallocate(ty, what)
                    });
                    self.emit(&GuestDeallocateVariant { blocks: 2 });
                }

                TypeDefKind::Result(e) => {
                    self.flat_for_each_variant_arm(
                        ty,
                        false,
                        [e.ok.as_ref(), e.err.as_ref()],
                        |me, ty| me.deallocate(ty, what),
                    );
                    self.emit(&GuestDeallocateVariant { blocks: 2 });
                }

                // discard the operand on the stack, otherwise nothing to free.
                TypeDefKind::Flags(_)
                | TypeDefKind::Enum(_)
                | TypeDefKind::Future(_)
                | TypeDefKind::Stream(_)
                | TypeDefKind::Handle(Handle::Own(_))
                | TypeDefKind::Handle(Handle::Borrow(_)) => {
                    self.stack.pop().unwrap();
                }

                TypeDefKind::Resource => unreachable!(),
                TypeDefKind::Unknown => unreachable!(),

                TypeDefKind::FixedSizeList(..) => todo!(),
                TypeDefKind::Map(..) => todo!(),
            },
        }
    }

    fn deallocate_indirect(
        &mut self,
        ty: &Type,
        addr: B::Operand,
        offset: ArchitectureSize,
        what: Deallocate,
    ) {
        use Instruction::*;

        // No need to execute any instructions if this type itself doesn't
        // require any form of post-return.
        if !needs_deallocate(self.resolve, ty, what) {
            return;
        }

        match *ty {
            Type::String => {
                self.stack.push(addr.clone());
                self.emit(&Instruction::PointerLoad { offset });
                self.stack.push(addr);
                self.emit(&Instruction::LengthLoad {
                    offset: offset + self.bindgen.sizes().align(ty).into(),
                });
                self.deallocate(ty, what);
            }

            Type::Bool
            | Type::U8
            | Type::S8
            | Type::U16
            | Type::S16
            | Type::U32
            | Type::S32
            | Type::Char
            | Type::U64
            | Type::S64
            | Type::F32
            | Type::F64
            | Type::ErrorContext => {}

            Type::Id(id) => match &self.resolve.types[id].kind {
                TypeDefKind::Type(t) => self.deallocate_indirect(t, addr, offset, what),

                TypeDefKind::List(_) => {
                    self.stack.push(addr.clone());
                    self.emit(&Instruction::PointerLoad { offset });
                    self.stack.push(addr);
                    self.emit(&Instruction::LengthLoad {
                        offset: offset + self.bindgen.sizes().align(ty).into(),
                    });

                    self.deallocate(ty, what);
                }

                TypeDefKind::Handle(Handle::Own(_))
                | TypeDefKind::Future(_)
                | TypeDefKind::Stream(_)
                    if what.handles() =>
                {
                    self.read_from_memory(ty, addr, offset);
                    self.emit(&DropHandle { ty });
                }

                TypeDefKind::Handle(Handle::Own(_)) => unreachable!(),
                TypeDefKind::Handle(Handle::Borrow(_)) => unreachable!(),
                TypeDefKind::Resource => unreachable!(),

                TypeDefKind::Record(record) => {
                    self.deallocate_indirect_fields(
                        &record.fields.iter().map(|f| f.ty).collect::<Vec<_>>(),
                        addr,
                        offset,
                        what,
                    );
                }

                TypeDefKind::Tuple(tuple) => {
                    self.deallocate_indirect_fields(&tuple.types, addr, offset, what);
                }

                TypeDefKind::Flags(_) => {}

                TypeDefKind::Variant(variant) => {
                    self.deallocate_indirect_variant(
                        offset,
                        addr,
                        variant.tag(),
                        variant.cases.iter().map(|c| c.ty.as_ref()),
                        what,
                    );
                    self.emit(&GuestDeallocateVariant {
                        blocks: variant.cases.len(),
                    });
                }

                TypeDefKind::Option(t) => {
                    self.deallocate_indirect_variant(offset, addr, Int::U8, [None, Some(t)], what);
                    self.emit(&GuestDeallocateVariant { blocks: 2 });
                }

                TypeDefKind::Result(e) => {
                    self.deallocate_indirect_variant(
                        offset,
                        addr,
                        Int::U8,
                        [e.ok.as_ref(), e.err.as_ref()],
                        what,
                    );
                    self.emit(&GuestDeallocateVariant { blocks: 2 });
                }

                TypeDefKind::Enum(_) => {}

                TypeDefKind::Future(_) => unreachable!(),
                TypeDefKind::Stream(_) => unreachable!(),
                TypeDefKind::Unknown => unreachable!(),
                TypeDefKind::FixedSizeList(..) => todo!(),
                TypeDefKind::Map(..) => todo!(),
            },
        }
    }

    fn deallocate_indirect_variant<'b>(
        &mut self,
        offset: ArchitectureSize,
        addr: B::Operand,
        tag: Int,
        cases: impl IntoIterator<Item = Option<&'b Type>> + Clone,
        what: Deallocate,
    ) {
        self.stack.push(addr.clone());
        self.load_intrepr(offset, tag);
        let payload_offset = offset + (self.bindgen.sizes().payload_offset(tag, cases.clone()));
        for ty in cases {
            self.push_block();
            if let Some(ty) = ty {
                self.deallocate_indirect(ty, addr.clone(), payload_offset, what);
            }
            self.finish_block(0);
        }
    }

    fn deallocate_indirect_fields(
        &mut self,
        tys: &[Type],
        addr: B::Operand,
        offset: ArchitectureSize,
        what: Deallocate,
    ) {
        for (field_offset, ty) in self.bindgen.sizes().field_offsets(tys) {
            self.deallocate_indirect(ty, addr.clone(), offset + (field_offset), what);
        }
    }
}

fn cast(from: WasmType, to: WasmType) -> Bitcast {
    use WasmType::*;

    match (from, to) {
        (I32, I32)
        | (I64, I64)
        | (F32, F32)
        | (F64, F64)
        | (Pointer, Pointer)
        | (PointerOrI64, PointerOrI64)
        | (Length, Length) => Bitcast::None,

        (I32, I64) => Bitcast::I32ToI64,
        (F32, I32) => Bitcast::F32ToI32,
        (F64, I64) => Bitcast::F64ToI64,

        (I64, I32) => Bitcast::I64ToI32,
        (I32, F32) => Bitcast::I32ToF32,
        (I64, F64) => Bitcast::I64ToF64,

        (F32, I64) => Bitcast::F32ToI64,
        (I64, F32) => Bitcast::I64ToF32,

        (I64, PointerOrI64) => Bitcast::I64ToP64,
        (Pointer, PointerOrI64) => Bitcast::PToP64,
        (_, PointerOrI64) => {
            Bitcast::Sequence(Box::new([cast(from, I64), cast(I64, PointerOrI64)]))
        }

        (PointerOrI64, I64) => Bitcast::P64ToI64,
        (PointerOrI64, Pointer) => Bitcast::P64ToP,
        (PointerOrI64, _) => Bitcast::Sequence(Box::new([cast(PointerOrI64, I64), cast(I64, to)])),

        (I32, Pointer) => Bitcast::I32ToP,
        (Pointer, I32) => Bitcast::PToI32,
        (I32, Length) => Bitcast::I32ToL,
        (Length, I32) => Bitcast::LToI32,
        (I64, Length) => Bitcast::I64ToL,
        (Length, I64) => Bitcast::LToI64,
        (Pointer, Length) => Bitcast::PToL,
        (Length, Pointer) => Bitcast::LToP,

        (F32, Pointer | Length) => Bitcast::Sequence(Box::new([cast(F32, I32), cast(I32, to)])),
        (Pointer | Length, F32) => Bitcast::Sequence(Box::new([cast(from, I32), cast(I32, F32)])),

        (F32, F64)
        | (F64, F32)
        | (F64, I32)
        | (I32, F64)
        | (Pointer | Length, I64 | F64)
        | (I64 | F64, Pointer | Length) => {
            unreachable!("Don't know how to bitcast from {:?} to {:?}", from, to);
        }
    }
}

/// Flatten types in a given type
///
/// It is sometimes necessary to restrict the number of max parameters dynamically,
/// for example during an async guest import call (flat params are limited to 4)
fn flat_types(resolve: &Resolve, ty: &Type, max_params: Option<usize>) -> Option<Vec<WasmType>> {
    let max_params = max_params.unwrap_or(MAX_FLAT_PARAMS);
    let mut storage = iter::repeat_n(WasmType::I32, max_params).collect::<Vec<_>>();
    let mut flat = FlatTypes::new(storage.as_mut_slice());
    resolve.push_flat(ty, &mut flat).then_some(flat.to_vec())
}
