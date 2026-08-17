use crate::codegen_settings::{CodegenSettings, ErrorType};
use crate::lifetimes::anon_lifetime;
use crate::module_trait::passed_by_reference;
use crate::names;
use crate::types::WiggleType;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::mem;
use witx::Instruction;

pub fn define_func(
    module: &witx::Module,
    func: &witx::InterfaceFunc,
    settings: &CodegenSettings,
) -> TokenStream {
    let (ts, _bounds) = _define_func(module, func, settings);
    ts
}

pub fn func_bounds(
    module: &witx::Module,
    func: &witx::InterfaceFunc,
    settings: &CodegenSettings,
) -> Vec<Ident> {
    let (_ts, bounds) = _define_func(module, func, settings);
    bounds
}

fn _define_func(
    module: &witx::Module,
    func: &witx::InterfaceFunc,
    settings: &CodegenSettings,
) -> (TokenStream, Vec<Ident>) {
    let ident = names::func(&func.name);

    let (wasm_params, wasm_results) = func.wasm_signature();
    let param_names = (0..wasm_params.len())
        .map(|i| Ident::new(&format!("arg{i}"), Span::call_site()))
        .collect::<Vec<_>>();
    let abi_params = wasm_params.iter().zip(&param_names).map(|(arg, name)| {
        let wasm = names::wasm_type(*arg);
        quote!(#name : #wasm)
    });

    let abi_ret = match wasm_results.len() {
        0 => quote!(()),
        1 => {
            let ty = names::wasm_type(wasm_results[0]);
            quote!(#ty)
        }
        _ => unimplemented!(),
    };

    let mut body = TokenStream::new();
    let mut bounds = vec![names::trait_name(&module.name)];
    func.call_interface(
        &module.name,
        &mut Rust {
            src: &mut body,
            params: &param_names,
            block_storage: Vec::new(),
            blocks: Vec::new(),
            module,
            funcname: func.name.as_str(),
            settings,
            bounds: &mut bounds,
        },
    );

    let mod_name = &module.name.as_str();
    let func_name = &func.name.as_str();
    let mk_span = quote!(
        let _span = wiggle::tracing::span!(
            wiggle::tracing::Level::TRACE,
            "wiggle abi",
            module = #mod_name,
            function = #func_name
        );
    );
    let ctx_type = if settings.mutable {
        quote!(&mut)
    } else {
        quote!(&)
    };
    if settings.get_async(&module, &func).is_sync() {
        let traced_body = if settings.tracing.enabled_for(&mod_name, &func_name) {
            quote!(
                #mk_span
                _span.in_scope(|| {
                  #body
                })
            )
        } else {
            quote!(#body)
        };
        (
            quote!(
                #[allow(unreachable_code)] // deals with warnings in noreturn functions
                pub fn #ident(
                    ctx: #ctx_type (impl #(#bounds)+*),
                    memory: &mut wiggle::GuestMemory<'_>,
                    #(#abi_params),*
                ) -> wiggle::anyhow::Result<#abi_ret> {
                    use std::convert::TryFrom as _;
                    #traced_body
                }
            ),
            bounds,
        )
    } else {
        let traced_body = if settings.tracing.enabled_for(&mod_name, &func_name) {
            quote!(
                use wiggle::tracing::Instrument as _;
                #mk_span
                async move {
                    #body
                }.instrument(_span).await
            )
        } else {
            quote!(
                #body
            )
        };
        (
            quote!(
                #[allow(unreachable_code)] // deals with warnings in noreturn functions
                pub async fn #ident(
                    ctx: #ctx_type (impl #(#bounds)+*),
                    memory: &mut wiggle::GuestMemory<'_>,
                    #(#abi_params),*
                ) -> wiggle::anyhow::Result<#abi_ret> {
                    use std::convert::TryFrom as _;
                    #traced_body
                }
            ),
            bounds,
        )
    }
}

struct Rust<'a> {
    src: &'a mut TokenStream,
    params: &'a [Ident],
    block_storage: Vec<TokenStream>,
    blocks: Vec<TokenStream>,
    module: &'a witx::Module,
    funcname: &'a str,
    settings: &'a CodegenSettings,
    bounds: &'a mut Vec<Ident>,
}

impl Rust<'_> {
    fn bound(&mut self, i: Ident) {
        if !self.bounds.contains(&i) {
            self.bounds.push(i);
        }
    }
}

impl witx::Bindgen for Rust<'_> {
    type Operand = TokenStream;

    fn push_block(&mut self) {
        let prev = mem::replace(self.src, TokenStream::new());
        self.block_storage.push(prev);
    }

    fn finish_block(&mut self, operand: Option<TokenStream>) {
        let to_restore = self.block_storage.pop().unwrap();
        let src = mem::replace(self.src, to_restore);
        match operand {
            None => self.blocks.push(src),
            Some(s) => {
                if src.is_empty() {
                    self.blocks.push(s);
                } else {
                    self.blocks.push(quote!({ #src; #s }));
                }
            }
        }
    }

    // This is only used for `call_wasm` at this time.
    fn allocate_space(&mut self, _: usize, _: &witx::NamedType) {
        unimplemented!()
    }

    fn emit(
        &mut self,
        inst: &Instruction<'_>,
        operands: &mut Vec<TokenStream>,
        results: &mut Vec<TokenStream>,
    ) {
        let wrap_err = |location: &str| {
            let modulename = self.module.name.as_str();
            let funcname = self.funcname;
            quote! {
                |e| {
                    wiggle::GuestError::InFunc {
                        modulename: #modulename,
                        funcname: #funcname,
                        location: #location,
                        err: Box::new(wiggle::GuestError::from(e)),
                    }
                }
            }
        };

        let mut try_from = |ty: TokenStream| {
            let val = operands.pop().unwrap();
            let wrap_err = wrap_err(&format!("convert {ty}"));
            results.push(quote!(#ty::try_from(#val).map_err(#wrap_err)?));
        };

        match inst {
            Instruction::GetArg { nth } => {
                let param = &self.params[*nth];
                results.push(quote!(#param));
            }

            Instruction::PointerFromI32 { ty } | Instruction::ConstPointerFromI32 { ty } => {
                let val = operands.pop().unwrap();
                let pointee_type = names::type_ref(ty, anon_lifetime());
                results.push(quote! {
                    wiggle::GuestPtr::<#pointee_type>::new(#val as u32)
                });
            }

            Instruction::ListFromPointerLength { ty } => {
                let ptr = &operands[0];
                let len = &operands[1];
                let ty = match &**ty.type_() {
                    witx::Type::Builtin(witx::BuiltinType::Char) => quote!(str),
                    _ => {
                        let ty = names::type_ref(ty, anon_lifetime());
                        quote!([#ty])
                    }
                };
                results.push(quote! {
                    wiggle::GuestPtr::<#ty>::new((#ptr as u32, #len as u32));
                })
            }

            Instruction::CallInterface { func, .. } => {
                // Use the `tracing` crate to log all arguments that are going
                // out, and afterwards we call the function with those bindings.
                let mut args = Vec::new();
                for (i, param) in func.params.iter().enumerate() {
                    let name = names::func_param(&param.name);
                    let val = &operands[i];
                    self.src.extend(quote!(let #name = #val;));
                    if passed_by_reference(param.tref.type_()) {
                        args.push(quote!(&#name));
                    } else {
                        args.push(quote!(#name));
                    }
                }
                if self
                    .settings
                    .tracing
                    .enabled_for(self.module.name.as_str(), self.funcname)
                    && func.params.len() > 0
                {
                    let args = func
                        .params
                        .iter()
                        .map(|param| {
                            let name = names::func_param(&param.name);
                            if param.impls_display() {
                                quote!( #name = wiggle::tracing::field::display(&#name) )
                            } else {
                                quote!( #name = wiggle::tracing::field::debug(&#name) )
                            }
                        })
                        .collect::<Vec<_>>();
                    self.src.extend(quote! {
                        wiggle::tracing::event!(wiggle::tracing::Level::TRACE, #(#args),*);
                    });
                }

                let trait_name = names::trait_name(&self.module.name);
                let ident = names::func(&func.name);
                if self.settings.get_async(&self.module, &func).is_sync() {
                    self.src.extend(quote! {
                        let ret = #trait_name::#ident(ctx, memory, #(#args),*);
                    })
                } else {
                    self.src.extend(quote! {
                        let ret = #trait_name::#ident(ctx, memory, #(#args),*).await;
                    })
                };
                if self
                    .settings
                    .tracing
                    .enabled_for(self.module.name.as_str(), self.funcname)
                {
                    self.src.extend(quote! {
                        wiggle::tracing::event!(
                            wiggle::tracing::Level::TRACE,
                            result = wiggle::tracing::field::debug(&ret),
                        );
                    });
                }

                if func.results.len() > 0 {
                    results.push(quote!(ret));
                } else if func.noreturn {
                    self.src.extend(quote!(return Err(ret);));
                }
            }

            // Lowering an enum is typically simple but if we have an error
            // transformation registered for this enum then what we're actually
            // doing is lowering from a user-defined error type to the error
            // enum, and *then* we lower to an i32.
            Instruction::EnumLower { ty } => {
                let val = operands.pop().unwrap();
                let val = match self.settings.errors.for_name(ty) {
                    Some(ErrorType::User(custom)) => {
                        let method = names::user_error_conversion_method(&custom);
                        self.bound(quote::format_ident!("UserErrorConversion"));
                        quote!(UserErrorConversion::#method(ctx, #val)?)
                    }
                    Some(ErrorType::Generated(_)) => quote!(#val.downcast()?),
                    None => val,
                };
                results.push(quote!(#val as i32));
            }

            Instruction::ResultLower { err: err_ty, .. } => {
                let err = self.blocks.pop().unwrap();
                let ok = self.blocks.pop().unwrap();
                let val = operands.pop().unwrap();
                let err_typename = names::type_ref(err_ty.unwrap(), anon_lifetime());
                results.push(quote! {
                    match #val {
                        Ok(e) => { #ok; <#err_typename as wiggle::GuestErrorType>::success() as i32 }
                        Err(e) => { #err }
                    }
                });
            }

            Instruction::VariantPayload => results.push(quote!(e)),

            Instruction::Return { amt: 0 } => {
                self.src.extend(quote!(return Ok(())));
            }
            Instruction::Return { amt: 1 } => {
                let val = operands.pop().unwrap();
                self.src.extend(quote!(return Ok(#val)));
            }
            Instruction::Return { .. } => unimplemented!(),

            Instruction::TupleLower { amt } => {
                let names = (0..*amt)
                    .map(|i| Ident::new(&format!("t{i}"), Span::call_site()))
                    .collect::<Vec<_>>();
                let val = operands.pop().unwrap();
                self.src.extend(quote!( let (#(#names,)*) = #val;));
                results.extend(names.iter().map(|i| quote!(#i)));
            }

            Instruction::Store { ty } => {
                let ptr = operands.pop().unwrap();
                let val = operands.pop().unwrap();
                let wrap_err = wrap_err(&format!("write {}", ty.name.as_str()));
                let pointee_type = names::type_(&ty.name);
                self.src.extend(quote! {
                    memory.write(
                        wiggle::GuestPtr::<#pointee_type>::new(#ptr as u32),
                        #val,
                    )
                    .map_err(#wrap_err)?;
                });
            }

            Instruction::Load { ty } => {
                let ptr = operands.pop().unwrap();
                let wrap_err = wrap_err(&format!("read {}", ty.name.as_str()));
                let pointee_type = names::type_(&ty.name);
                results.push(quote! {
                    memory.read(wiggle::GuestPtr::<#pointee_type>::new(#ptr as u32))
                        .map_err(#wrap_err)?
                });
            }

            Instruction::HandleFromI32 { ty } => {
                let val = operands.pop().unwrap();
                let ty = names::type_(&ty.name);
                results.push(quote!(#ty::from(#val)));
            }

            // Smaller-than-32 numerical conversions are done with `TryFrom` to
            // ensure we're not losing bits.
            Instruction::U8FromI32 => try_from(quote!(u8)),
            Instruction::S8FromI32 => try_from(quote!(i8)),
            Instruction::Char8FromI32 => try_from(quote!(u8)),
            Instruction::U16FromI32 => try_from(quote!(u16)),
            Instruction::S16FromI32 => try_from(quote!(i16)),

            // Conversions with matching bit-widths but different signededness
            // use `as` since we're basically just reinterpreting the bits.
            Instruction::U32FromI32 | Instruction::UsizeFromI32 => {
                let val = operands.pop().unwrap();
                results.push(quote!(#val as u32));
            }
            Instruction::U64FromI64 => {
                let val = operands.pop().unwrap();
                results.push(quote!(#val as u64));
            }

            // Conversions to enums/bitflags use `TryFrom` to ensure that the
            // values are valid coming in.
            Instruction::EnumLift { ty }
            | Instruction::BitflagsFromI64 { ty }
            | Instruction::BitflagsFromI32 { ty } => {
                let ty = names::type_(&ty.name);
                try_from(quote!(#ty))
            }

            // No conversions necessary for these, the native wasm type matches
            // our own representation.
            Instruction::If32FromF32
            | Instruction::If64FromF64
            | Instruction::S32FromI32
            | Instruction::S64FromI64 => results.push(operands.pop().unwrap()),

            // There's a number of other instructions we could implement but
            // they're not exercised by WASI at this time. As necessary we can
            // add code to implement them.
            other => panic!("no implementation for {other:?}"),
        }
    }
}
