use crate::{
    ConstructorReturnType, Identifier, InterfaceGenerator, RustFlagsRepr,
    classify_constructor_return_type, int_repr, to_rust_ident,
};
use heck::*;
use std::fmt::Write as _;
use std::mem;
use wit_bindgen_core::abi::{Bindgen, Instruction, LiftLower, WasmType};
use wit_bindgen_core::{Source, dealias, uwrite, uwriteln, wit_parser::*};

pub(super) struct FunctionBindgen<'a, 'b> {
    pub r#gen: &'b mut InterfaceGenerator<'a>,
    params: Vec<String>,
    wasm_import_module: &'b str,
    pub src: Source,
    blocks: Vec<String>,
    block_storage: Vec<Source>,
    tmp: usize,
    pub needs_cleanup_list: bool,
    pub import_return_pointer_area_size: ArchitectureSize,
    pub import_return_pointer_area_align: Alignment,
    pub handle_decls: Vec<String>,
    always_owned: bool,
}

pub const POINTER_SIZE_EXPRESSION: &str = "::core::mem::size_of::<*const u8>()";

impl<'a, 'b> FunctionBindgen<'a, 'b> {
    pub(super) fn new(
        r#gen: &'b mut InterfaceGenerator<'a>,
        params: Vec<String>,
        wasm_import_module: &'b str,
        always_owned: bool,
    ) -> FunctionBindgen<'a, 'b> {
        FunctionBindgen {
            r#gen,
            params,
            wasm_import_module,
            src: Default::default(),
            blocks: Vec::new(),
            block_storage: Vec::new(),
            tmp: 0,
            needs_cleanup_list: false,
            import_return_pointer_area_size: Default::default(),
            import_return_pointer_area_align: Default::default(),
            handle_decls: Vec::new(),
            always_owned,
        }
    }

    fn cleanup(&mut self, cleanup_value: &str) {
        if self.block_storage.len() > 0 {
            self.needs_cleanup_list = true;
            uwriteln!(self.src, "cleanup_list.extend({cleanup_value});");
        }
    }

    fn declare_import(&mut self, name: &str, params: &[WasmType], results: &[WasmType]) -> String {
        let tmp = self.tmp();
        let rust_name = format!("wit_import{tmp}");
        self.src.push_str(&crate::declare_import(
            self.wasm_import_module,
            name,
            &rust_name,
            params,
            results,
        ));
        rust_name
    }

    fn let_results(&mut self, amt: usize, results: &mut Vec<String>) {
        match amt {
            0 => {}
            1 => {
                let tmp = self.tmp();
                let res = format!("result{tmp}");
                self.push_str("let ");
                self.push_str(&res);
                results.push(res);
                self.push_str(" = ");
            }
            n => {
                let tmp = self.tmp();
                self.push_str("let (");
                for i in 0..n {
                    let arg = format!("result{tmp}_{i}");
                    self.push_str(&arg);
                    self.push_str(",");
                    results.push(arg);
                }
                self.push_str(") = ");
            }
        }
    }

    fn record_lower(
        &mut self,
        id: TypeId,
        record: &Record,
        operand: &str,
        results: &mut Vec<String>,
    ) {
        let tmp = self.tmp();
        self.push_str("let ");
        let name = self.typename_lower(id);
        self.push_str(&name);
        self.push_str("{ ");
        for field in record.fields.iter() {
            let name = to_rust_ident(&field.name);
            let arg = format!("{name}{tmp}");
            self.push_str(&name);
            self.push_str(":");
            self.push_str(&arg);
            self.push_str(", ");
            results.push(arg);
        }
        self.push_str("} = ");
        self.push_str(operand);
        self.push_str(";\n");
    }

    fn record_lift(
        &mut self,
        id: TypeId,
        ty: &Record,
        operands: &[String],
        results: &mut Vec<String>,
    ) {
        let mut result = self.typename_lift(id);
        result.push_str("{\n");
        for (field, val) in ty.fields.iter().zip(operands) {
            result.push_str(&to_rust_ident(&field.name));
            result.push_str(": ");
            result.push_str(val);
            result.push_str(",\n");
        }
        result.push('}');
        results.push(result);
    }

    fn tuple_lower(&mut self, tuple: &Tuple, operand: &str, results: &mut Vec<String>) {
        let tmp = self.tmp();
        self.push_str("let (");
        for i in 0..tuple.types.len() {
            let arg = format!("t{tmp}_{i}");
            self.push_str(&arg);
            self.push_str(", ");
            results.push(arg);
        }
        self.push_str(") = ");
        self.push_str(operand);
        self.push_str(";\n");
    }

    fn tuple_lift(&mut self, operands: &[String], results: &mut Vec<String>) {
        if operands.len() == 1 {
            results.push(format!("({},)", operands[0]));
        } else {
            results.push(format!("({})", operands.join(", ")));
        }
    }

    fn typename_lower(&self, id: TypeId) -> String {
        let owned = self.always_owned
            || match self.lift_lower() {
                LiftLower::LowerArgsLiftResults => false,
                LiftLower::LiftArgsLowerResults => true,
            };
        self.r#gen.type_path(id, owned)
    }

    fn typename_lift(&self, id: TypeId) -> String {
        self.r#gen.type_path(id, true)
    }

    fn push_str(&mut self, s: &str) {
        self.src.push_str(s);
    }

    fn tmp(&mut self) -> usize {
        let ret = self.tmp;
        self.tmp += 1;
        ret
    }

    fn lift_lower(&self) -> LiftLower {
        if self.r#gen.in_import {
            LiftLower::LowerArgsLiftResults
        } else {
            LiftLower::LiftArgsLowerResults
        }
    }
}

impl Bindgen for FunctionBindgen<'_, '_> {
    type Operand = String;

    fn push_block(&mut self) {
        let prev_src = mem::take(&mut self.src);
        self.block_storage.push(prev_src);
    }

    fn finish_block(&mut self, operands: &mut Vec<String>) {
        let prev_src = self.block_storage.pop().unwrap();
        let src = mem::replace(&mut self.src, prev_src);
        let expr = match operands.len() {
            0 => "()".to_string(),
            1 => operands[0].clone(),
            _ => format!("({})", operands.join(", ")),
        };
        if src.is_empty() {
            self.blocks.push(expr);
        } else if operands.is_empty() {
            self.blocks.push(format!("{{\n{}}}", &src[..]));
        } else {
            self.blocks.push(format!("{{\n{}\n{}\n}}", &src[..], expr));
        }
    }

    fn return_pointer(&mut self, size: ArchitectureSize, align: Alignment) -> String {
        let tmp = self.tmp();

        // Imports get a per-function return area to facilitate using the
        // stack whereas exports use a per-module return area to cut down on
        // stack usage. Note that for imports this also facilitates "adapter
        // modules" for components to not have data segments.
        if size.is_empty() {
            // If the size requested is 0 then we know it won't be written to so
            // hand out a null pointer. This can happen with async for example
            // when the params or results are zero-sized.
            uwrite!(self.src, "let ptr{tmp} = core::ptr::null_mut::<u8>();");
        } else if self.r#gen.in_import {
            // Import return areas are stored on the stack since this stack
            self.import_return_pointer_area_size = self.import_return_pointer_area_size.max(size);
            self.import_return_pointer_area_align =
                self.import_return_pointer_area_align.max(align);
            uwrite!(
                self.src,
                "let ptr{tmp} = ret_area.0.as_mut_ptr().cast::<u8>();"
            );
        } else {
            // Export return areas are stored in `static` memory as they need to
            // persist beyond the function call itself (and are cleaned-up in
            // `post-return`).
            self.r#gen.return_pointer_area_size = self.r#gen.return_pointer_area_size.max(size);
            self.r#gen.return_pointer_area_align = self.r#gen.return_pointer_area_align.max(align);
            uwriteln!(
                self.src,
                "let ptr{tmp} = (&raw mut _RET_AREA.0).cast::<u8>();"
            );
        }
        format!("ptr{tmp}")
    }

    fn sizes(&self) -> &SizeAlign {
        &self.r#gen.sizes
    }

    fn is_list_canonical(&self, _resolve: &Resolve, ty: &Type) -> bool {
        self.r#gen.is_list_canonical(ty)
    }

    fn emit(
        &mut self,
        resolve: &Resolve,
        inst: &Instruction<'_>,
        operands: &mut Vec<String>,
        results: &mut Vec<String>,
    ) {
        let mut top_as = |cvt: &str| {
            let mut s = operands.pop().unwrap();
            s.push_str(" as ");
            s.push_str(cvt);
            results.push(s);
        };

        match inst {
            Instruction::GetArg { nth } => results.push(self.params[*nth].clone()),
            Instruction::I32Const { val } => results.push(format!("{val}i32")),
            Instruction::ConstZero { tys } => {
                for ty in tys.iter() {
                    match ty {
                        WasmType::I32 => results.push("0i32".to_string()),
                        WasmType::I64 => results.push("0i64".to_string()),
                        WasmType::F32 => results.push("0.0f32".to_string()),
                        WasmType::F64 => results.push("0.0f64".to_string()),
                        WasmType::Pointer => results.push("::core::ptr::null_mut()".to_string()),
                        WasmType::PointerOrI64 => {
                            results.push("::core::mem::MaybeUninit::<u64>::zeroed()".to_string())
                        }
                        WasmType::Length => results.push("0usize".to_string()),
                    }
                }
            }

            Instruction::I64FromU64 | Instruction::I64FromS64 => {
                let s = operands.pop().unwrap();
                results.push(format!("{}({s})", self.r#gen.path_to_as_i64()));
            }
            Instruction::I32FromChar
            | Instruction::I32FromU8
            | Instruction::I32FromS8
            | Instruction::I32FromU16
            | Instruction::I32FromS16
            | Instruction::I32FromU32
            | Instruction::I32FromS32 => {
                let s = operands.pop().unwrap();
                results.push(format!("{}({s})", self.r#gen.path_to_as_i32()));
            }

            Instruction::CoreF32FromF32 => {
                let s = operands.pop().unwrap();
                results.push(format!("{}({s})", self.r#gen.path_to_as_f32()));
            }
            Instruction::CoreF64FromF64 => {
                let s = operands.pop().unwrap();
                results.push(format!("{}({s})", self.r#gen.path_to_as_f64()));
            }
            Instruction::F32FromCoreF32
            | Instruction::F64FromCoreF64
            | Instruction::S32FromI32
            | Instruction::S64FromI64 => {
                results.push(operands.pop().unwrap());
            }
            Instruction::S8FromI32 => top_as("i8"),
            Instruction::U8FromI32 => top_as("u8"),
            Instruction::S16FromI32 => top_as("i16"),
            Instruction::U16FromI32 => top_as("u16"),
            Instruction::U32FromI32 => top_as("u32"),
            Instruction::U64FromI64 => top_as("u64"),
            Instruction::CharFromI32 => {
                results.push(format!(
                    "{}({} as u32)",
                    self.r#gen.path_to_char_lift(),
                    operands[0]
                ));
            }

            Instruction::Bitcasts { casts } => crate::bitcast(casts, operands, results),

            Instruction::I32FromBool => {
                results.push(format!("match {} {{ true => 1, false => 0 }}", operands[0]));
            }
            Instruction::BoolFromI32 => {
                results.push(format!(
                    "{}({} as u8)",
                    self.r#gen.path_to_bool_lift(),
                    operands[0]
                ));
            }

            Instruction::FlagsLower { flags, .. } => {
                let tmp = self.tmp();
                self.push_str(&format!("let flags{} = {};\n", tmp, operands[0]));
                for i in 0..flags.repr().count() {
                    results.push(format!("(flags{}.bits() >> {}) as i32", tmp, i * 32));
                }
            }
            Instruction::FlagsLift { flags, ty, .. } => {
                let repr = RustFlagsRepr::new(flags);
                let name = self.r#gen.type_path(*ty, true);
                let mut result = format!("{name}::empty()");
                for (i, op) in operands.iter().enumerate() {
                    result.push_str(&format!(
                        " | {name}::from_bits_retain((({op} as {repr}) << {}) as _)",
                        i * 32
                    ));
                }
                results.push(result);
            }

            Instruction::HandleLower {
                handle: Handle::Own(_),
                ..
            } => {
                let op = &operands[0];
                let result = format!("({op}).take_handle() as i32");
                results.push(result);
            }

            Instruction::HandleLower {
                handle: Handle::Borrow(_),
                ..
            } => {
                let op = &operands[0];
                results.push(format!("({op}).handle() as i32"))
            }

            Instruction::HandleLift { handle, .. } => {
                let op = &operands[0];
                let (is_own, resource) = match handle {
                    Handle::Borrow(resource) => (false, resource),
                    Handle::Own(resource) => (true, resource),
                };

                let dealiased_resource = dealias(resolve, *resource);

                let result = if is_own {
                    let name = self.r#gen.type_path(dealiased_resource, true);
                    format!("{name}::from_handle({op} as u32)")
                } else if self.r#gen.is_exported_resource(*resource) {
                    let name = resolve.types[*resource]
                        .name
                        .as_deref()
                        .unwrap()
                        .to_upper_camel_case();
                    format!("{name}Borrow::lift({op} as u32 as usize)")
                } else {
                    let tmp = format!("handle{}", self.tmp());
                    self.handle_decls.push(format!("let {tmp};"));
                    let name = self.r#gen.type_path(dealiased_resource, true);
                    format!(
                        "{{\n
                            {tmp} = {name}::from_handle({op} as u32);
                            &{tmp}
                        }}"
                    )
                };
                results.push(result);
            }

            Instruction::FutureLower { .. } => {
                let op = &operands[0];
                results.push(format!("({op}).take_handle() as i32"))
            }

            Instruction::FutureLift { payload, .. } => {
                let async_support = self.r#gen.r#gen.async_support_path();
                let op = &operands[0];
                let name = payload
                    .as_ref()
                    .map(|ty| {
                        self.r#gen
                            .type_name_owned_with_id(ty, Identifier::StreamOrFuturePayload)
                    })
                    .unwrap_or_else(|| "()".into());
                let ordinal = self
                    .r#gen
                    .r#gen
                    .future_payloads
                    .get_index_of(&name)
                    .unwrap();
                let path = self.r#gen.path_to_root();
                results.push(format!(
                    "{async_support}::FutureReader::new\
                     ({op} as u32, &{path}wit_future::vtable{ordinal}::VTABLE)"
                ))
            }

            Instruction::StreamLower { .. } => {
                let op = &operands[0];
                results.push(format!("({op}).take_handle() as i32"))
            }

            Instruction::StreamLift { payload, .. } => {
                let async_support = self.r#gen.r#gen.async_support_path();
                let op = &operands[0];
                let name = payload
                    .as_ref()
                    .map(|ty| {
                        self.r#gen
                            .type_name_owned_with_id(ty, Identifier::StreamOrFuturePayload)
                    })
                    .unwrap_or_else(|| "()".into());
                let ordinal = self
                    .r#gen
                    .r#gen
                    .stream_payloads
                    .get_index_of(&name)
                    .unwrap();
                let path = self.r#gen.path_to_root();
                results.push(format!(
                    "{async_support}::StreamReader::new\
                     ({op} as u32, &{path}wit_stream::vtable{ordinal}::VTABLE)"
                ))
            }

            Instruction::ErrorContextLower { .. } => {
                let op = &operands[0];
                results.push(format!("({op}).handle() as i32"))
            }

            Instruction::ErrorContextLift { .. } => {
                let async_support = self.r#gen.r#gen.async_support_path();
                let op = &operands[0];
                results.push(format!(
                    "{async_support}::ErrorContext::from_handle({op} as u32)"
                ))
            }

            Instruction::RecordLower { ty, record, .. } => {
                self.record_lower(*ty, record, &operands[0], results);
            }
            Instruction::RecordLift { ty, record, .. } => {
                self.record_lift(*ty, record, operands, results);
            }

            Instruction::TupleLower { tuple, .. } => {
                self.tuple_lower(tuple, &operands[0], results);
            }
            Instruction::TupleLift { .. } => {
                self.tuple_lift(operands, results);
            }

            Instruction::VariantPayloadName => results.push("e".to_string()),

            Instruction::VariantLower {
                variant,
                results: result_types,
                ty,
                ..
            } => {
                let blocks = self
                    .blocks
                    .drain(self.blocks.len() - variant.cases.len()..)
                    .collect::<Vec<_>>();
                let name = self.typename_lower(*ty);
                let name = if name.contains("::") {
                    let tmp = self.tmp();
                    uwriteln!(self.src, "use {name} as V{tmp};");
                    format!("V{tmp}")
                } else {
                    name
                };
                self.let_results(result_types.len(), results);
                let op0 = &operands[0];
                self.push_str(&format!("match {op0} {{\n"));
                for (case, block) in variant.cases.iter().zip(blocks) {
                    let case_name = case.name.to_upper_camel_case();
                    self.push_str(&format!("{name}::{case_name}"));
                    if case.ty.is_some() {
                        self.push_str(&format!("(e) => {block},\n"));
                    } else {
                        self.push_str(&format!(" => {{\n{block}\n}}\n"));
                    }
                }
                if results.is_empty() {
                    self.push_str("}\n");
                } else {
                    self.push_str("};\n");
                }
            }

            Instruction::VariantLift { variant, ty, .. } => {
                let blocks = self
                    .blocks
                    .drain(self.blocks.len() - variant.cases.len()..)
                    .collect::<Vec<_>>();
                let op0 = &operands[0];
                let tmp = self.tmp();
                let name = self.typename_lift(*ty);
                let name = if name.contains("::") {
                    uwriteln!(self.src, "use {name} as V{tmp};");
                    format!("V{tmp}")
                } else {
                    name
                };
                uwriteln!(self.src, "let v{tmp} = match {op0} {{");
                for (i, (case, block)) in variant.cases.iter().zip(blocks).enumerate() {
                    if i == variant.cases.len() - 1 {
                        uwriteln!(
                            self.src,
                            "n => {{
                                debug_assert_eq!(n, {i}, \"invalid enum discriminant\");\
                            "
                        );
                    } else {
                        uwriteln!(self.src, "{i} => {{");
                    }
                    let case_name = case.name.to_upper_camel_case();
                    if case.ty.is_none() {
                        uwriteln!(self.src, "{name}::{case_name}");
                    } else {
                        uwriteln!(self.src, "let e{tmp} = {block};");
                        uwriteln!(self.src, "{name}::{case_name}(e{tmp})");
                    }
                    uwriteln!(self.src, "}}");
                }
                uwriteln!(self.src, "}};");
                results.push(format!("v{tmp}"));
            }

            Instruction::OptionLower {
                results: result_types,
                ..
            } => {
                let some = self.blocks.pop().unwrap();
                let none = self.blocks.pop().unwrap();
                self.let_results(result_types.len(), results);
                let operand = &operands[0];
                self.push_str(&format!(
                    "match {operand} {{
                        Some(e) => {some},
                        None => {{\n{none}\n}},
                    }};"
                ));
            }

            Instruction::OptionLift { .. } => {
                let some = self.blocks.pop().unwrap();
                let none = self.blocks.pop().unwrap();
                assert_eq!(none, "()");
                let operand = &operands[0];
                results.push(format!(
                    "match {operand} {{
                        0 => None,
                        1 => {{
                            let e = {some};
                            Some(e)
                        }}
                        _ => {invalid}(),
                    }}",
                    invalid = self.r#gen.path_to_invalid_enum_discriminant(),
                ));
            }

            Instruction::ResultLower {
                results: result_types,
                result,
                ..
            } => {
                let err = self.blocks.pop().unwrap();
                let ok = self.blocks.pop().unwrap();
                self.let_results(result_types.len(), results);
                let operand = &operands[0];
                let ok_binding = if result.ok.is_some() { "e" } else { "_" };
                let err_binding = if result.err.is_some() { "e" } else { "_" };
                self.push_str(&format!(
                    "match {operand} {{
                        Ok({ok_binding}) => {{ {ok} }},
                        Err({err_binding}) => {{ {err} }},
                    }};"
                ));
            }

            Instruction::ResultLift { .. } => {
                let err = self.blocks.pop().unwrap();
                let ok = self.blocks.pop().unwrap();
                let operand = &operands[0];
                results.push(format!(
                    "match {operand} {{
                        0 => {{
                            let e = {ok};
                            Ok(e)
                        }}
                        1 => {{
                            let e = {err};
                            Err(e)
                        }}
                        _ => {invalid}(),
                    }}",
                    invalid = self.r#gen.path_to_invalid_enum_discriminant(),
                ));
            }

            Instruction::EnumLower { .. } => {
                results.push(format!("{}.clone() as i32", operands[0]));
            }

            Instruction::EnumLift { enum_, ty, .. } => {
                let name = self.r#gen.type_path(*ty, true);
                let repr = int_repr(enum_.tag());
                let op = &operands[0];
                let result = format!("{name}::_lift({op} as {repr})");
                results.push(result);
            }

            Instruction::ListCanonLower { realloc, .. } => {
                let tmp = self.tmp();
                let val = format!("vec{tmp}");
                let ptr = format!("ptr{tmp}");
                let len = format!("len{tmp}");
                let vec = self.r#gen.path_to_vec();
                if realloc.is_none() {
                    self.push_str(&format!("let {} = {};\n", val, operands[0]));
                } else {
                    let op0 = operands.pop().unwrap();
                    self.push_str(&format!(
                        "let {val} = <_ as Into<{vec}<_>>>::into({op0}).into_boxed_slice();\n"
                    ));
                }
                self.push_str(&format!("let {ptr} = {val}.as_ptr().cast::<u8>();\n"));
                self.push_str(&format!("let {len} = {val}.len();\n"));
                if realloc.is_some() {
                    self.push_str(&format!("::core::mem::forget({val});\n"));
                }
                results.push(format!("{ptr}.cast_mut()"));
                results.push(len);
            }

            Instruction::ListCanonLift { .. } => {
                let tmp = self.tmp();
                let len = format!("len{tmp}");
                self.push_str(&format!("let {} = {};\n", len, operands[1]));
                let vec = self.r#gen.path_to_vec();
                let result = format!(
                    "<_ as From<{vec}<_>>>::from({vec}::from_raw_parts({}.cast(), {1}, {1}))",
                    operands[0], len
                );
                results.push(result);
            }

            Instruction::StringLower { realloc } => {
                let tmp = self.tmp();
                let val = format!("vec{tmp}");
                let ptr = format!("ptr{tmp}");
                let len = format!("len{tmp}");
                if realloc.is_none() {
                    self.push_str(&format!("let {} = {};\n", val, operands[0]));
                } else {
                    let op0 = format!("{}.into_bytes()", operands[0]);
                    self.push_str(&format!("let {val} = ({op0}).into_boxed_slice();\n"));
                }
                self.push_str(&format!("let {ptr} = {val}.as_ptr().cast::<u8>();\n"));
                self.push_str(&format!("let {len} = {val}.len();\n"));
                if realloc.is_some() {
                    self.push_str(&format!("::core::mem::forget({val});\n"));
                }
                results.push(format!("{ptr}.cast_mut()"));
                results.push(len);
            }

            Instruction::StringLift => {
                let vec = self.r#gen.path_to_vec();
                let tmp = self.tmp();
                let len = format!("len{tmp}");
                uwriteln!(self.src, "let {len} = {};", operands[1]);
                uwriteln!(
                    self.src,
                    "let bytes{tmp} = {vec}::from_raw_parts({}.cast(), {len}, {len});",
                    operands[0],
                );
                if self.r#gen.r#gen.opts.raw_strings {
                    results.push(format!("bytes{tmp}"));
                } else {
                    results.push(format!("{}(bytes{tmp})", self.r#gen.path_to_string_lift()));
                }
            }

            Instruction::ListLower { element, realloc } => {
                let alloc = self.r#gen.path_to_std_alloc_module();
                let rt = self.r#gen.r#gen.runtime_path().to_string();
                let body = self.blocks.pop().unwrap();
                let tmp = self.tmp();
                let vec = format!("vec{tmp}");
                let result = format!("result{tmp}");
                let layout = format!("layout{tmp}");
                let len = format!("len{tmp}");
                let cleanup = format!("_cleanup{tmp}");
                self.push_str(&format!(
                    "let {vec} = {operand0};\n",
                    operand0 = operands[0]
                ));
                self.push_str(&format!("let {len} = {vec}.len();\n"));
                let size = self.r#gen.sizes.size(element);
                let align = self.r#gen.sizes.align(element);
                self.push_str(&format!(
                    "let {layout} = {alloc}::Layout::from_size_align({vec}.len() * {}, {}).unwrap();\n",
                    size.format(POINTER_SIZE_EXPRESSION), align.format(POINTER_SIZE_EXPRESSION),
                ));
                self.push_str(&format!(
                    "let ({result}, {cleanup}) = {rt}::Cleanup::new({layout});"
                ));
                if realloc.is_none() {
                    // If an allocator isn't requested then we must clean up the
                    // allocation ourselves since our callee isn't taking
                    // ownership.
                    self.cleanup(&cleanup);
                } else {
                    uwriteln!(
                        self.src,
                        "if let Some(cleanup) = {cleanup} {{ cleanup.forget(); }}"
                    );
                }
                self.push_str(&format!("for (i, e) in {vec}.into_iter().enumerate() {{\n",));
                self.push_str(&format!(
                    "let base = {result}.add(i * {});\n",
                    size.format(POINTER_SIZE_EXPRESSION)
                ));
                self.push_str(&body);
                self.push_str("\n}\n");
                results.push(format!("{result}"));
                results.push(len);
            }

            Instruction::ListLift { element, .. } => {
                let body = self.blocks.pop().unwrap();
                let tmp = self.tmp();
                let size = self.r#gen.sizes.size(element);
                let align = self.r#gen.sizes.align(element);
                let len = format!("len{tmp}");
                let base = format!("base{tmp}");
                let result = format!("result{tmp}");
                self.push_str(&format!(
                    "let {base} = {operand0};\n",
                    operand0 = operands[0]
                ));
                self.push_str(&format!(
                    "let {len} = {operand1};\n",
                    operand1 = operands[1]
                ));
                let vec = self.r#gen.path_to_vec();
                self.push_str(&format!(
                    "let mut {result} = {vec}::with_capacity({len});\n",
                ));

                uwriteln!(self.src, "for i in 0..{len} {{");
                uwriteln!(
                    self.src,
                    "let base = {base}.add(i * {size});",
                    size = size.format(POINTER_SIZE_EXPRESSION)
                );
                uwriteln!(self.src, "let e{tmp} = {body};");
                uwriteln!(self.src, "{result}.push(e{tmp});");
                uwriteln!(self.src, "}}");
                results.push(result);
                let dealloc = self.r#gen.path_to_cabi_dealloc();
                self.push_str(&format!(
                    "{dealloc}({base}, {len} * {size}, {align});\n",
                    size = size.format(POINTER_SIZE_EXPRESSION),
                    align = align.format(POINTER_SIZE_EXPRESSION)
                ));
            }

            Instruction::IterElem { .. } => results.push("e".to_string()),

            Instruction::IterBasePointer => results.push("base".to_string()),

            Instruction::CallWasm { name, sig, .. } => {
                let func = self.declare_import(name, &sig.params, &sig.results);

                // ... then call the function with all our operands
                if !sig.results.is_empty() {
                    self.push_str("let ret = ");
                    results.push("ret".to_string());
                }
                self.push_str(&func);
                self.push_str("(");
                self.push_str(&operands.join(", "));
                self.push_str(");\n");
            }

            Instruction::CallInterface { func, async_ } => {
                let prev_src = mem::replace(self.src.as_mut_string(), String::new());

                self.let_results(usize::from(func.result.is_some()), results);
                // If this function has a result and it's async, then after
                // this instruction the result is going to be lowered. This
                // lowering must happen in terms of `&T`, so force the result
                // of this expression to have `&` in front.
                if func.result.is_some() && *async_ {
                    self.push_str("&");
                }

                // Force evaluation of all arguments to happen in a sub-block
                // for this call. This is where `handle_decls` are pushed to
                // ensure that they're scoped to just this block. Additionally
                // this is where `prev_src` is pushed, just before the call.
                //
                // The purpose of this is to force `borrow<T>` arguments to get
                // dropped before the call to `task.return` that will happen
                // after an async call. For normal calls this happens because
                // the result of this function is returned directly, but for
                // async calls the return happens as part of this function as
                // a call to `task.return` via the `AsyncTaskReturn`
                // instruction.
                self.push_str("{\n");
                for decl in self.handle_decls.drain(..) {
                    self.src.push_str(&decl);
                }
                self.push_str(&prev_src);
                let constructor_type = match &func.kind {
                    FunctionKind::Freestanding | FunctionKind::AsyncFreestanding => {
                        self.push_str(&format!("T::{}", to_rust_ident(func.item_name())));
                        None
                    }
                    FunctionKind::Method(_)
                    | FunctionKind::Static(_)
                    | FunctionKind::AsyncMethod(_)
                    | FunctionKind::AsyncStatic(_) => {
                        self.push_str(&format!("T::{}", to_rust_ident(func.item_name())));
                        None
                    }
                    FunctionKind::Constructor(ty) => {
                        let return_type =
                            classify_constructor_return_type(resolve, *ty, &func.result);
                        let ty = resolve.types[*ty]
                            .name
                            .as_deref()
                            .unwrap()
                            .to_upper_camel_case();

                        match return_type {
                            ConstructorReturnType::Self_ => {
                                self.push_str(&format!("{ty}::new(T::new"));
                            }
                            ConstructorReturnType::Result { .. } => {
                                self.push_str(&format!("T::new"));
                            }
                        }

                        Some((ty, return_type))
                    }
                };
                self.push_str("(");
                for (i, operand) in operands.iter().enumerate() {
                    if i > 0 {
                        self.push_str(", ");
                    }

                    self.push_str(operand);

                    // Automatically convert `Borrow<'_, AResource>` to
                    // `&Self` since traits have `&self` as their
                    // first arguments.
                    if i == 0
                        && (matches!(func.kind, FunctionKind::Method(_))
                            || matches!(func.kind, FunctionKind::AsyncMethod(_)))
                    {
                        self.push_str(".get()")
                    }
                }
                self.push_str(")");
                if *async_ {
                    self.push_str(".await");
                }
                match constructor_type {
                    Some((_, ConstructorReturnType::Self_)) => self.push_str(")"),
                    Some((ty, ConstructorReturnType::Result { .. })) => {
                        self.push_str(&format!(".map({ty}::new)"))
                    }
                    None => {}
                }
                self.push_str("\n};\n");
            }

            Instruction::AsyncTaskReturn { name, params } => {
                let func = self.declare_import(name, params, &[]);

                uwriteln!(self.src, "_task_cancel.forget();");
                uwriteln!(self.src, "{func}({});", operands.join(", "));
            }

            Instruction::Flush { amt } => {
                for i in 0..*amt {
                    let tmp = self.tmp();
                    let result = format!("result{tmp}");
                    uwriteln!(self.src, "let {result} = {};", operands[i]);
                    results.push(result);
                }
            }

            Instruction::Return { amt, .. } => match amt {
                0 => {}
                1 => {
                    self.push_str(&operands[0]);
                    self.push_str("\n");
                }
                _ => {
                    self.push_str("(");
                    self.push_str(&operands.join(", "));
                    self.push_str(")\n");
                }
            },

            Instruction::I32Load { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = *{}.add({offset}).cast::<i32>();",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::I32Load8U { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = i32::from(*{0}.add({offset}).cast::<u8>());",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::I32Load8S { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = i32::from(*{}.add({offset}).cast::<i8>());",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::I32Load16U { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = i32::from(*{}.add({offset}).cast::<u16>());",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::I32Load16S { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = i32::from(*{}.add({offset}).cast::<i16>());",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::I64Load { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = *{}.add({offset}).cast::<i64>();",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::F32Load { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = *{}.add({offset}).cast::<f32>();",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::F64Load { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = *{}.add({offset}).cast::<f64>();",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }

            Instruction::PointerLoad { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = *{}.add({offset}).cast::<*mut u8>();",
                    operands[0],
                    offset = offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }
            Instruction::LengthLoad { offset } => {
                let tmp = self.tmp();
                uwriteln!(
                    self.src,
                    "let l{tmp} = *{}.add({}).cast::<usize>();",
                    operands[0],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true)
                );
                results.push(format!("l{tmp}"));
            }

            Instruction::I32Store { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<i32>() = {};\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }
            Instruction::I32Store8 { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<u8>() = ({}) as u8;\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }
            Instruction::I32Store16 { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<u16>() = ({}) as u16;\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }
            Instruction::I64Store { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<i64>() = {};\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }
            Instruction::F32Store { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<f32>() = {};\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }
            Instruction::F64Store { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<f64>() = {};\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }

            Instruction::PointerStore { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<*mut u8>() = {};\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }
            Instruction::LengthStore { offset } => {
                self.push_str(&format!(
                    "*{}.add({}).cast::<usize>() = {};\n",
                    operands[1],
                    offset.format_term(POINTER_SIZE_EXPRESSION, true),
                    operands[0]
                ));
            }

            Instruction::Malloc { .. } => unimplemented!(),

            Instruction::GuestDeallocate { size, align } => {
                let dealloc = self.r#gen.path_to_cabi_dealloc();
                self.push_str(&format!(
                    "{dealloc}({op}, {size}, {align});\n",
                    op = operands[0],
                    size = size.format_term(POINTER_SIZE_EXPRESSION, true),
                    align = align.format(POINTER_SIZE_EXPRESSION)
                ));
            }

            Instruction::GuestDeallocateString => {
                let dealloc = self.r#gen.path_to_cabi_dealloc();
                self.push_str(&format!(
                    "{dealloc}({op0}, {op1}, 1);\n",
                    op0 = operands[0],
                    op1 = operands[1],
                ));
            }

            Instruction::GuestDeallocateVariant { blocks } => {
                let max = blocks - 1;
                let blocks = self
                    .blocks
                    .drain(self.blocks.len() - blocks..)
                    .collect::<Vec<_>>();
                let op0 = &operands[0];
                self.src.push_str(&format!("match {op0} {{\n"));
                for (i, block) in blocks.into_iter().enumerate() {
                    let pat = if i == max {
                        String::from("_")
                    } else {
                        i.to_string()
                    };
                    self.src.push_str(&format!("{pat} => {block},\n"));
                }
                self.src.push_str("}\n");
            }

            Instruction::GuestDeallocateList { element } => {
                let body = self.blocks.pop().unwrap();
                let tmp = self.tmp();
                let size = self.r#gen.sizes.size(element);
                let align = self.r#gen.sizes.align(element);
                let len = format!("len{tmp}");
                let base = format!("base{tmp}");
                self.push_str(&format!(
                    "let {base} = {operand0};\n",
                    operand0 = operands[0]
                ));
                self.push_str(&format!(
                    "let {len} = {operand1};\n",
                    operand1 = operands[1]
                ));

                if body != "()" {
                    self.push_str("for i in 0..");
                    self.push_str(&len);
                    self.push_str(" {\n");
                    self.push_str("let base = ");
                    self.push_str(&base);
                    self.push_str(".add(i * ");
                    self.push_str(&size.format(POINTER_SIZE_EXPRESSION));
                    self.push_str(");\n");
                    self.push_str(&body);
                    self.push_str("\n}\n");
                }
                let dealloc = self.r#gen.path_to_cabi_dealloc();
                self.push_str(&format!(
                    "{dealloc}({base}, {len} * {size}, {align});\n",
                    size = size.format(POINTER_SIZE_EXPRESSION),
                    align = align.format(POINTER_SIZE_EXPRESSION)
                ));
            }

            Instruction::DropHandle { .. } => {
                uwriteln!(self.src, "let _ = {};", operands[0]);
            }
        }
    }
}
