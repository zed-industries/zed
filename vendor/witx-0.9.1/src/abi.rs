//! Definition of the ABI of witx functions
//!
//! This module is intended to assist with code generators which are binding or
//! implementing APIs defined by `*.witx` files. THis module contains all
//! details necessary to implement the actual ABI of these functions so wasm
//! modules and hosts can communicate with one another.
//!
//! Each interface types function (a function defined in `*.witx`) currently has
//! a well-known wasm signature associated with it. There's then also a standard
//! way to convert from interface-types values (whose representation is defined
//! per-language) into this wasm API. This module is intended to assist with
//! this definition.
//!
//! Contained within are two primary functions, [`InterfaceFunc::call_wasm`] and
//! [`InterfaceFunc::call_interface`]. These functions implement the two ways to
//! interact with an interface types function, namely calling the raw wasm
//! version and calling the high-level version with interface types. These two
//! functions are fed a structure that implements [`Bindgen`]. An instance of
//! [`Bindgen`] receives instructions which are low-level implementation details
//! of how to convert to and from wasm types and interface types. Code
//! generators will need to implement the various instructions to support APIs.

use crate::{
    BuiltinType, Id, IntRepr, InterfaceFunc, InterfaceFuncParam, NamedType, Type, TypeRef,
};

/// Enumerates wasm types used by interface types when lowering/lifting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    // NOTE: we don't lower interface types to any other Wasm type,
    // e.g. externref, so we don't need to define them here.
}

impl From<IntRepr> for WasmType {
    fn from(i: IntRepr) -> WasmType {
        match i {
            IntRepr::U8 | IntRepr::U16 | IntRepr::U32 => WasmType::I32,
            IntRepr::U64 => WasmType::I64,
        }
    }
}

/// Possible ABIs for interface functions to have.
///
/// Note that this is a stopgap until we have more of interface types. Interface
/// types functions do not have ABIs, they have APIs. For the meantime, however,
/// we mandate ABIs to ensure we can all talk to each other.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Abi {
    /// Only stable ABI currently, and is the historical WASI ABI since it was
    /// first created.
    ///
    /// Note that this ABI is limited notably in its return values where it can
    /// only return 0 results or one `Result<T, enum>` lookalike.
    Preview1,
}

// Helper macro for defining instructions without having to have tons of
// exhaustive `match` statements to update
macro_rules! def_instruction {
    (
        $( #[$enum_attr:meta] )*
        pub enum Instruction<'a> {
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
        pub enum Instruction<'a> {
            $(
                $( #[$attr] )*
                $variant $( {
                    $(
                        $field : $field_ty,
                    )*
                } )? ,
            )*
        }

        impl Instruction<'_> {
            /// How many operands does this instruction pop from the stack?
            #[allow(unused_variables)]
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
            #[allow(unused_variables)]
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
        /// Takes the value off the top of the stack and writes it into linear
        /// memory. Pushes the address in linear memory as an `i32`.
        AddrOf : [1] => [1],
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
        /// Converts a language-specific `usize` value to a wasm `i32`.
        I32FromUsize : [1] => [1],
        /// Converts an interface type `u16` value to a wasm `i32`.
        I32FromU16 : [1] => [1],
        /// Converts an interface type `s16` value to a wasm `i32`.
        I32FromS16 : [1] => [1],
        /// Converts an interface type `u8` value to a wasm `i32`.
        I32FromU8 : [1] => [1],
        /// Converts an interface type `s8` value to a wasm `i32`.
        I32FromS8 : [1] => [1],
        /// Converts a language-specific C `char` value to a wasm `i32`.
        I32FromChar8 : [1] => [1],
        /// Converts a language-specific pointer value to a wasm `i32`.
        I32FromPointer : [1] => [1],
        /// Converts a language-specific pointer value to a wasm `i32`.
        I32FromConstPointer : [1] => [1],
        /// Converts a language-specific handle value to a wasm `i32`.
        I32FromHandle { ty: &'a NamedType } : [1] => [1],
        /// Converts a language-specific record-of-bools to the packed
        /// representation as an `i32`.
        I32FromBitflags { ty: &'a NamedType } : [1] => [1],
        /// Converts a language-specific record-of-bools to the packed
        /// representation as an `i64`.
        I64FromBitflags { ty: &'a NamedType } : [1] => [1],
        /// Converts an interface type list into its pointer/length, pushing
        /// them both on the stack.
        ListPointerLength : [1] => [2],
        /// Pops two `i32` values from the stack and creates a list from them of
        /// the specified type. The first operand is the pointer in linear
        /// memory to the start of the list and the second operand is the
        /// length.
        ListFromPointerLength { ty: &'a TypeRef } : [2] => [1],
        /// Conversion an interface type `f32` value to a wasm `f32`.
        ///
        /// This may be a noop for some implementations, but it's here in case the
        /// native language representation of `f32` is different than the wasm
        /// representation of `f32`.
        F32FromIf32 : [1] => [1],
        /// Conversion an interface type `f64` value to a wasm `f64`.
        ///
        /// This may be a noop for some implementations, but it's here in case the
        /// native language representation of `f64` is different than the wasm
        /// representation of `f64`.
        F64FromIf64 : [1] => [1],

        /// Represents a call to a raw WebAssembly API. The module/name are
        /// provided inline as well as the types if necessary.
        CallWasm {
            module: &'a str,
            name: &'a str,
            params: &'a [WasmType],
            results: &'a [WasmType],
        } : [params.len()] => [results.len()],

        /// Same as `CallWasm`, except the dual where an interface is being
        /// called rather than a raw wasm function.
        CallInterface {
            module: &'a str,
            func: &'a InterfaceFunc,
        } : [func.params.len()] => [func.results.len()],

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
        /// Converts a native wasm `i32` to a language-specific C `char`.
        ///
        /// This will truncate the upper bits of the `i32`.
        Char8FromI32 : [1] => [1],
        /// Converts a native wasm `i32` to a language-specific `usize`.
        UsizeFromI32 : [1] => [1],
        /// Converts a native wasm `f32` to an interface type `f32`.
        If32FromF32 : [1] => [1],
        /// Converts a native wasm `f64` to an interface type `f64`.
        If64FromF64 : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `handle`.
        HandleFromI32 { ty: &'a NamedType } : [1] => [1],
        /// Converts a native wasm `i32` to a language-specific pointer.
        PointerFromI32 { ty: &'a TypeRef }: [1] => [1],
        /// Converts a native wasm `i32` to a language-specific pointer.
        ConstPointerFromI32 { ty: &'a TypeRef } : [1] => [1],
        /// Converts a native wasm `i32` to a language-specific record-of-bools.
        BitflagsFromI32 { ty: &'a NamedType } : [1] => [1],
        /// Converts a native wasm `i64` to a language-specific record-of-bools.
        BitflagsFromI64 { ty: &'a NamedType } : [1] => [1],
        /// Acquires the return pointer `n` and pushes an `i32` on the stack.
        ///
        /// Implementations of [`Bindgen`] may have [`Bindgen::allocate_space`]
        /// called to reserve space in memory for the result of a computation to
        /// get written. This instruction acquires a pointer to the space
        /// reserved in `allocate_space`.
        ReturnPointerGet { n: usize } : [0] => [1],
        /// Loads the interface types value from an `i32` pointer popped from
        /// the stack.
        Load { ty: &'a NamedType } : [1] => [1],
        /// Stores an interface types value into linear memory. The first
        /// operand is the value to store and the second operand is the pointer
        /// in linear memory to store it at.
        Store { ty: &'a NamedType } : [2] => [0],
        /// Pops a native wasm `i32` from the stack, as well as two blocks
        /// internally from the code generator.
        ///
        /// If the value is 0 then the first "ok" block value should be used.
        /// If the value is anything else then the second "err" block value
        /// should be used, and the value is used as the error enum.
        ///
        /// Note that this is a special instruction matching the current ABI of
        /// WASI and intentionally differs from the type-level grammar of
        /// interface types results.
        ResultLift : [1] => [1],
        /// Pops a native interface value from the stack as well as two blocks
        /// internally from the code generator.
        ///
        /// A `match` is performed on the value popped and the corresponding
        /// block for ok/err is used depending on value. This pushes a single
        /// `i32` onto the stack representing the error code for this result.
        ///
        /// Note that like `ResultLift` this is specialized to the current WASI
        /// ABI.
        ResultLower {
            ok: Option<&'a TypeRef>,
            err: Option<&'a TypeRef>,
        } : [1] => [1],
        /// Converts a native wasm `i32` to an interface type `enum` value.
        ///
        /// It's guaranteed that the interface type integer value is within
        /// range for this enum's type. Additionally `ty` is guaranteed to be
        /// enum-like as a `Variant` where all `case` arms have no associated
        /// type with them. The purpose of this instruction is to convert a
        /// native wasm integer into the enum type for the interface.
        EnumLift { ty: &'a NamedType } : [1] => [1],
        /// Converts an interface types enum value into a wasm `i32`.
        EnumLower { ty: &'a NamedType } : [1] => [1],
        /// Creates a tuple from the top `n` elements on the stack, pushing the
        /// tuple onto the stack.
        TupleLift { amt: usize } : [*amt] => [1],
        /// Splits a tuple at the top of the stack into its `n` components,
        /// pushing them all onto the stack.
        TupleLower { amt: usize } : [1] => [*amt],
        /// This is a special instruction specifically for the original ABI of
        /// WASI.  The raw return `i32` of a function is re-pushed onto the
        /// stack for reuse.
        ReuseReturn : [0] => [1],
        /// Returns `amt` values on the stack. This is always the last
        /// instruction.
        Return { amt: usize } : [*amt] => [0],
        /// This is a special instruction used at the entry of blocks used as
        /// part of `ResultLower`, representing that the payload of that variant
        /// being matched on should be pushed onto the stack.
        VariantPayload : [0] => [1],
    }
}

impl Abi {
    /// Validates the parameters/results are representable in this ABI.
    ///
    /// Returns an error string if they're not representable or returns `Ok` if
    /// they're indeed representable.
    pub fn validate(
        &self,
        _params: &[InterfaceFuncParam],
        results: &[InterfaceFuncParam],
    ) -> Result<(), String> {
        assert_eq!(*self, Abi::Preview1);
        match results.len() {
            0 => {}
            1 => match &**results[0].tref.type_() {
                Type::Handle(_) | Type::Builtin(_) | Type::ConstPointer(_) | Type::Pointer(_) => {}
                Type::Variant(v) => {
                    let (ok, err) = match v.as_expected() {
                        Some(pair) => pair,
                        None => return Err("invalid return type".to_string()),
                    };
                    if let Some(ty) = ok {
                        match &**ty.type_() {
                            Type::Record(r) if r.is_tuple() => {
                                for member in r.members.iter() {
                                    if !member.tref.named() {
                                        return Err(
                                            "only named types are allowed in results".to_string()
                                        );
                                    }
                                }
                            }
                            _ => {
                                if !ty.named() {
                                    return Err(
                                        "only named types are allowed in results".to_string()
                                    );
                                }
                            }
                        }
                    }
                    if let Some(ty) = err {
                        if !ty.named() {
                            return Err("only named types are allowed in results".to_string());
                        }
                        if let Type::Variant(v) = &**ty.type_() {
                            if v.is_enum() {
                                return Ok(());
                            }
                        }
                    }
                }
                Type::Record(r) if r.bitflags_repr().is_some() => {}
                Type::Record(_) | Type::List(_) => return Err("invalid return type".to_string()),
            },
            _ => return Err("more than one result".to_string()),
        }
        Ok(())
    }
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
    type Operand;

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
        inst: &Instruction<'_>,
        operands: &mut Vec<Self::Operand>,
        results: &mut Vec<Self::Operand>,
    );

    /// Allocates temporary space in linear memory indexed by `slot` with enough
    /// space to store `ty`.
    ///
    /// This is called when calling some wasm functions where a return pointer
    /// is needed.
    fn allocate_space(&mut self, slot: usize, ty: &NamedType);

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
    fn finish_block(&mut self, operand: Option<Self::Operand>);
}

impl InterfaceFunc {
    /// Get the WebAssembly type signature for this interface function
    ///
    /// The first entry returned is the list of parameters and the second entry
    /// is the list of results for the wasm function signature.
    pub fn wasm_signature(&self) -> (Vec<WasmType>, Vec<WasmType>) {
        assert_eq!(self.abi, Abi::Preview1);
        let mut params = Vec::new();
        let mut results = Vec::new();
        for param in self.params.iter() {
            match &**param.tref.type_() {
                Type::Builtin(BuiltinType::S8)
                | Type::Builtin(BuiltinType::U8 { .. })
                | Type::Builtin(BuiltinType::S16)
                | Type::Builtin(BuiltinType::U16)
                | Type::Builtin(BuiltinType::S32)
                | Type::Builtin(BuiltinType::U32 { .. })
                | Type::Builtin(BuiltinType::Char)
                | Type::Pointer(_)
                | Type::ConstPointer(_)
                | Type::Handle(_)
                | Type::Variant(_) => params.push(WasmType::I32),

                Type::Record(r) => match r.bitflags_repr() {
                    Some(repr) => params.push(WasmType::from(repr)),
                    None => params.push(WasmType::I32),
                },

                Type::Builtin(BuiltinType::S64) | Type::Builtin(BuiltinType::U64) => {
                    params.push(WasmType::I64)
                }

                Type::Builtin(BuiltinType::F32) => params.push(WasmType::F32),
                Type::Builtin(BuiltinType::F64) => params.push(WasmType::F64),

                Type::List(_) => {
                    params.push(WasmType::I32);
                    params.push(WasmType::I32);
                }
            }
        }

        for param in self.results.iter() {
            match &**param.tref.type_() {
                Type::Builtin(BuiltinType::S8)
                | Type::Builtin(BuiltinType::U8 { .. })
                | Type::Builtin(BuiltinType::S16)
                | Type::Builtin(BuiltinType::U16)
                | Type::Builtin(BuiltinType::S32)
                | Type::Builtin(BuiltinType::U32 { .. })
                | Type::Builtin(BuiltinType::Char)
                | Type::Pointer(_)
                | Type::ConstPointer(_)
                | Type::Handle(_) => results.push(WasmType::I32),

                Type::Builtin(BuiltinType::S64) | Type::Builtin(BuiltinType::U64) => {
                    results.push(WasmType::I64)
                }

                Type::Builtin(BuiltinType::F32) => results.push(WasmType::F32),
                Type::Builtin(BuiltinType::F64) => results.push(WasmType::F64),

                Type::Record(r) => match r.bitflags_repr() {
                    Some(repr) => results.push(WasmType::from(repr)),
                    None => unreachable!(),
                },
                Type::List(_) => unreachable!(),

                Type::Variant(v) => {
                    results.push(match v.tag_repr {
                        IntRepr::U64 => WasmType::I64,
                        IntRepr::U32 | IntRepr::U16 | IntRepr::U8 => WasmType::I32,
                    });
                    if v.is_enum() {
                        continue;
                    }
                    // return pointer
                    if let Some(ty) = &v.cases[0].tref {
                        match &**ty.type_() {
                            Type::Record(r) if r.is_tuple() => {
                                for _ in 0..r.members.len() {
                                    params.push(WasmType::I32);
                                }
                            }
                            _ => params.push(WasmType::I32),
                        }
                    }
                }
            }
        }
        (params, results)
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
    pub fn call_wasm(&self, module: &Id, bindgen: &mut impl Bindgen) {
        assert_eq!(self.abi, Abi::Preview1);
        Generator {
            bindgen,
            operands: vec![],
            results: vec![],
            stack: vec![],
        }
        .call_wasm(module, self);
    }

    /// This is the dual of [`InterfaceFunc::call_wasm`], except that instead of
    /// calling a wasm signature it generates code to come from a wasm signature
    /// and call an interface types signature.
    pub fn call_interface(&self, module: &Id, bindgen: &mut impl Bindgen) {
        assert_eq!(self.abi, Abi::Preview1);
        Generator {
            bindgen,
            operands: vec![],
            results: vec![],
            stack: vec![],
        }
        .call_interface(module, self);
    }
}

struct Generator<'a, B: Bindgen> {
    bindgen: &'a mut B,
    operands: Vec<B::Operand>,
    results: Vec<B::Operand>,
    stack: Vec<B::Operand>,
}

impl<B: Bindgen> Generator<'_, B> {
    fn call_wasm(&mut self, module: &Id, func: &InterfaceFunc) {
        // Translate all parameters which are interface values by lowering them
        // to their wasm types.
        for (nth, param) in func.params.iter().enumerate() {
            self.emit(&Instruction::GetArg { nth });
            self.lower(&param.tref, None);
        }

        // If necessary for our ABI, insert return pointers for any returned
        // values through a result.
        assert!(func.results.len() < 2);
        if let Some(result) = func.results.get(0) {
            self.prep_return_pointer(&result.tref.type_());
        }

        let (params, results) = func.wasm_signature();
        self.emit(&Instruction::CallWasm {
            module: module.as_str(),
            name: func.name.as_str(),
            params: &params,
            results: &results,
        });

        // Lift the return value if one is present.
        if let Some(result) = func.results.get(0) {
            self.lift(&result.tref, true);
        }

        self.emit(&Instruction::Return {
            amt: func.results.len(),
        });
    }

    fn call_interface(&mut self, module: &Id, func: &InterfaceFunc) {
        // Lift all wasm parameters into interface types first.
        //
        // Note that consuming arguments is somewhat janky right now by manually
        // giving lists a second argument for their length. In the future we'll
        // probably want to refactor the `lift` function to internally know how
        // to consume arguments.
        let mut nth = 0;
        for param in func.params.iter() {
            self.emit(&Instruction::GetArg { nth });
            nth += 1;
            if let Type::List(_) = &**param.tref.type_() {
                self.emit(&Instruction::GetArg { nth });
                nth += 1;
            }
            self.lift(&param.tref, false);
        }

        self.emit(&Instruction::CallInterface {
            module: module.as_str(),
            func,
        });

        // Like above the current ABI only has at most one result, so lower it
        // here if necessary.
        if let Some(result) = func.results.get(0) {
            self.lower(&result.tref, Some(&mut nth));
        }

        let (_params, results) = func.wasm_signature();
        self.emit(&Instruction::Return { amt: results.len() });
    }

    fn emit(&mut self, inst: &Instruction<'_>) {
        self.operands.clear();
        self.results.clear();

        let operands_len = inst.operands_len();
        assert!(
            self.stack.len() >= operands_len,
            "not enough operands on stack for {:?}",
            inst
        );
        self.operands
            .extend(self.stack.drain((self.stack.len() - operands_len)..));
        self.results.reserve(inst.results_len());

        self.bindgen
            .emit(inst, &mut self.operands, &mut self.results);

        assert_eq!(
            self.results.len(),
            inst.results_len(),
            "{:?} expected {} results, got {}",
            inst,
            inst.results_len(),
            self.results.len()
        );
        self.stack.extend(self.results.drain(..));
    }

    fn lower(&mut self, ty: &TypeRef, retptr: Option<&mut usize>) {
        use Instruction::*;
        match &**ty.type_() {
            Type::Builtin(BuiltinType::S8) => self.emit(&I32FromS8),
            Type::Builtin(BuiltinType::U8 { lang_c_char: true }) => self.emit(&I32FromChar8),
            Type::Builtin(BuiltinType::U8 { lang_c_char: false }) => self.emit(&I32FromU8),
            Type::Builtin(BuiltinType::S16) => self.emit(&I32FromS16),
            Type::Builtin(BuiltinType::U16) => self.emit(&I32FromU16),
            Type::Builtin(BuiltinType::S32) => self.emit(&I32FromS32),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: true,
            }) => self.emit(&I32FromUsize),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: false,
            }) => self.emit(&I32FromU32),
            Type::Builtin(BuiltinType::S64) => self.emit(&I64FromS64),
            Type::Builtin(BuiltinType::U64) => self.emit(&I64FromU64),
            Type::Builtin(BuiltinType::Char) => self.emit(&I32FromChar),
            Type::Pointer(_) => self.emit(&I32FromPointer),
            Type::ConstPointer(_) => self.emit(&I32FromConstPointer),
            Type::Handle(_) => self.emit(&I32FromHandle {
                ty: match ty {
                    TypeRef::Name(ty) => ty,
                    _ => unreachable!(),
                },
            }),
            Type::Record(r) => {
                let ty = match ty {
                    TypeRef::Name(ty) => ty,
                    _ => unreachable!(),
                };
                match r.bitflags_repr() {
                    Some(IntRepr::U64) => self.emit(&I64FromBitflags { ty }),
                    Some(_) => self.emit(&I32FromBitflags { ty }),
                    None => self.emit(&AddrOf),
                }
            }
            Type::Variant(v) => {
                // Enum-like variants are simply lowered to their discriminant.
                if v.is_enum() {
                    return self.emit(&EnumLower {
                        ty: match ty {
                            TypeRef::Name(n) => n,
                            _ => unreachable!(),
                        },
                    });
                }

                // If this variant is in the return position then it's special,
                // otherwise it's an argument and we just pass the address.
                let retptr = match retptr {
                    Some(ptr) => ptr,
                    None => return self.emit(&AddrOf),
                };

                // For the return position we emit some blocks to lower the
                // ok/err payloads which means that in the ok branch we're
                // storing to out-params and in the err branch we're simply
                // lowering the error enum.
                //
                // Note that this is all very specific to the current WASI ABI.
                let (ok, err) = v.as_expected().unwrap();
                self.bindgen.push_block();
                if let Some(ok) = ok {
                    self.emit(&VariantPayload);
                    let store = |me: &mut Self, ty: &TypeRef, n| {
                        me.emit(&GetArg { nth: *retptr + n });
                        match ty {
                            TypeRef::Name(ty) => me.emit(&Store { ty }),
                            _ => unreachable!(),
                        }
                    };
                    match &**ok.type_() {
                        Type::Record(r) if r.is_tuple() => {
                            self.emit(&TupleLower {
                                amt: r.members.len(),
                            });
                            // Note that `rev()` is used here due to the order
                            // that tuples are pushed onto the stack and how we
                            // consume the last item first from the stack.
                            for (i, member) in r.members.iter().enumerate().rev() {
                                store(self, &member.tref, i);
                            }
                        }
                        _ => store(self, ok, 0),
                    }
                };
                self.bindgen.finish_block(None);

                self.bindgen.push_block();
                let err_expr = if let Some(ty) = err {
                    self.emit(&VariantPayload);
                    self.lower(ty, None);
                    Some(self.stack.pop().unwrap())
                } else {
                    None
                };
                self.bindgen.finish_block(err_expr);

                self.emit(&ResultLower { ok, err });
            }
            Type::Builtin(BuiltinType::F32) => self.emit(&F32FromIf32),
            Type::Builtin(BuiltinType::F64) => self.emit(&F64FromIf64),
            Type::List(_) => self.emit(&ListPointerLength),
        }
    }

    fn prep_return_pointer(&mut self, ty: &Type) {
        // Return pointers are only needed for `Result<T, _>`...
        let variant = match ty {
            Type::Variant(v) => v,
            _ => return,
        };
        // ... and only if `T` is actually present in `Result<T, _>`
        let ok = match &variant.cases[0].tref {
            Some(t) => t,
            None => return,
        };

        // Tuples have each individual item in a separate return pointer while
        // all other types go through a singular return pointer.
        let mut n = 0;
        let mut prep = |ty: &TypeRef| {
            match ty {
                TypeRef::Name(ty) => self.bindgen.allocate_space(n, ty),
                _ => unreachable!(),
            }
            self.emit(&Instruction::ReturnPointerGet { n });
            n += 1;
        };
        match &**ok.type_() {
            Type::Record(r) if r.is_tuple() => {
                for member in r.members.iter() {
                    prep(&member.tref);
                }
            }
            _ => prep(ok),
        }
    }

    // Note that in general everything in this function is the opposite of the
    // `lower` function above. This is intentional and should be kept this way!
    fn lift(&mut self, ty: &TypeRef, is_return: bool) {
        use Instruction::*;
        match &**ty.type_() {
            Type::Builtin(BuiltinType::S8) => self.emit(&S8FromI32),
            Type::Builtin(BuiltinType::U8 { lang_c_char: true }) => self.emit(&Char8FromI32),
            Type::Builtin(BuiltinType::U8 { lang_c_char: false }) => self.emit(&U8FromI32),
            Type::Builtin(BuiltinType::S16) => self.emit(&S16FromI32),
            Type::Builtin(BuiltinType::U16) => self.emit(&U16FromI32),
            Type::Builtin(BuiltinType::S32) => self.emit(&S32FromI32),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: true,
            }) => self.emit(&UsizeFromI32),
            Type::Builtin(BuiltinType::U32 {
                lang_ptr_size: false,
            }) => self.emit(&U32FromI32),
            Type::Builtin(BuiltinType::S64) => self.emit(&S64FromI64),
            Type::Builtin(BuiltinType::U64) => self.emit(&U64FromI64),
            Type::Builtin(BuiltinType::Char) => self.emit(&CharFromI32),
            Type::Builtin(BuiltinType::F32) => self.emit(&If32FromF32),
            Type::Builtin(BuiltinType::F64) => self.emit(&If64FromF64),
            Type::Pointer(ty) => self.emit(&PointerFromI32 { ty }),
            Type::ConstPointer(ty) => self.emit(&ConstPointerFromI32 { ty }),
            Type::Handle(_) => self.emit(&HandleFromI32 {
                ty: match ty {
                    TypeRef::Name(ty) => ty,
                    _ => unreachable!(),
                },
            }),
            Type::Variant(v) => {
                if v.is_enum() {
                    return self.emit(&EnumLift {
                        ty: match ty {
                            TypeRef::Name(n) => n,
                            _ => unreachable!(),
                        },
                    });
                } else if !is_return {
                    return self.emit(&Load {
                        ty: match ty {
                            TypeRef::Name(n) => n,
                            _ => unreachable!(),
                        },
                    });
                }

                let (ok, err) = v.as_expected().unwrap();
                self.bindgen.push_block();
                let ok_expr = if let Some(ok) = ok {
                    let mut n = 0;
                    let mut load = |ty: &TypeRef| {
                        self.emit(&ReturnPointerGet { n });
                        n += 1;
                        match ty {
                            TypeRef::Name(ty) => self.emit(&Load { ty }),
                            _ => unreachable!(),
                        }
                    };
                    match &**ok.type_() {
                        Type::Record(r) if r.is_tuple() => {
                            for member in r.members.iter() {
                                load(&member.tref);
                            }
                            self.emit(&TupleLift {
                                amt: r.members.len(),
                            });
                        }
                        _ => load(ok),
                    }
                    Some(self.stack.pop().unwrap())
                } else {
                    None
                };
                self.bindgen.finish_block(ok_expr);

                self.bindgen.push_block();
                let err_expr = if let Some(ty) = err {
                    self.emit(&ReuseReturn);
                    self.lift(ty, false);
                    Some(self.stack.pop().unwrap())
                } else {
                    None
                };
                self.bindgen.finish_block(err_expr);

                self.emit(&ResultLift);
            }
            Type::Record(r) => {
                let ty = match ty {
                    TypeRef::Name(ty) => ty,
                    _ => unreachable!(),
                };
                match r.bitflags_repr() {
                    Some(IntRepr::U64) => self.emit(&BitflagsFromI64 { ty }),
                    Some(_) => self.emit(&BitflagsFromI32 { ty }),
                    None => self.emit(&Load { ty }),
                }
            }
            Type::List(ty) => self.emit(&ListFromPointerLength { ty }),
        }
    }
}
