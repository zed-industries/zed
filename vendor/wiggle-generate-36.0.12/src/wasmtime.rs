use crate::CodegenSettings;
use crate::config::Asyncness;
use crate::funcs::func_bounds;
use crate::names;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;

pub fn link_module(
    module: &witx::Module,
    target_path: Option<&syn::Path>,
    settings: &CodegenSettings,
) -> TokenStream {
    let module_ident = names::module(&module.name);

    let send_bound = if settings.async_.contains_async(module) {
        quote! { + Send, T: Send }
    } else {
        quote! {}
    };

    let mut bodies = Vec::new();
    let mut bounds = HashSet::new();
    for f in module.funcs() {
        let asyncness = settings.async_.get(module.name.as_str(), f.name.as_str());
        bodies.push(generate_func(&module, &f, target_path, asyncness));
        let bound = func_bounds(module, &f, settings);
        for b in bound {
            bounds.insert(b);
        }
    }

    let ctx_bound = if let Some(target_path) = target_path {
        let bounds = bounds
            .into_iter()
            .map(|b| quote!(#target_path::#module_ident::#b));
        quote!( #(#bounds)+* #send_bound )
    } else {
        let bounds = bounds.into_iter();
        quote!( #(#bounds)+* #send_bound )
    };

    let func_name = if target_path.is_none() {
        format_ident!("add_to_linker")
    } else {
        format_ident!("add_{}_to_linker", module_ident)
    };

    let u = if settings.mutable {
        quote!(&mut U)
    } else {
        quote!(&U)
    };
    quote! {
        /// Adds all instance items to the specified `Linker`.
        pub fn #func_name<T, U>(
            linker: &mut wiggle::wasmtime_crate::Linker<T>,
            get_cx: impl Fn(&mut T) -> #u + Send + Sync + Copy + 'static,
        ) -> wiggle::anyhow::Result<()>
            where
                T: 'static,
                U: #ctx_bound #send_bound
        {
            #(#bodies)*
            Ok(())
        }
    }
}

fn generate_func(
    module: &witx::Module,
    func: &witx::InterfaceFunc,
    target_path: Option<&syn::Path>,
    asyncness: Asyncness,
) -> TokenStream {
    let module_str = module.name.as_str();
    let module_ident = names::module(&module.name);

    let field_str = func.name.as_str();
    let field_ident = names::func(&func.name);

    let (params, results) = func.wasm_signature();

    let arg_names = (0..params.len())
        .map(|i| Ident::new(&format!("arg{i}"), Span::call_site()))
        .collect::<Vec<_>>();
    let arg_tys = params
        .iter()
        .map(|ty| names::wasm_type(*ty))
        .collect::<Vec<_>>();
    let arg_decls = arg_names
        .iter()
        .zip(arg_tys.iter())
        .map(|(name, ty)| {
            quote! { #name: #ty }
        })
        .collect::<Vec<_>>();

    let ret_ty = match results.len() {
        0 => quote!(()),
        1 => names::wasm_type(results[0]),
        _ => unimplemented!(),
    };

    let await_ = if asyncness.is_sync() {
        quote!()
    } else {
        quote!(.await)
    };

    let abi_func = if let Some(target_path) = target_path {
        quote!( #target_path::#module_ident::#field_ident )
    } else {
        quote!( #field_ident )
    };

    let body = quote! {
        let export = caller.get_export("memory");
        let fuel = wiggle::wasmtime_crate::AsContextMut::as_context_mut(&mut caller).hostcall_fuel();
        let (mut mem, ctx) = match &export {
            Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                let (mem, ctx) = m.data_and_store_mut(&mut caller);
                let ctx = get_cx(ctx);
                ctx.set_hostcall_fuel(fuel);
                (wiggle::GuestMemory::Unshared(mem), ctx)
            }
            Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                let ctx = get_cx(caller.data_mut());
                ctx.set_hostcall_fuel(fuel);
                (wiggle::GuestMemory::Shared(m.data()), ctx)
            }
            _ => wiggle::anyhow::bail!("missing required memory export"),
        };
        Ok(<#ret_ty>::from(#abi_func(ctx, &mut mem #(, #arg_names)*) #await_ ?))
    };

    match asyncness {
        Asyncness::Async => {
            let arg_decls = quote! { ( #(#arg_names,)* ) : ( #(#arg_tys,)* ) };
            quote! {
                linker.func_wrap_async(
                    #module_str,
                    #field_str,
                    move |mut caller: wiggle::wasmtime_crate::Caller<'_, T>, #arg_decls| {
                        Box::new(async move { #body })
                    },
                )?;
            }
        }

        Asyncness::Blocking { block_with } => {
            quote! {
                linker.func_wrap(
                    #module_str,
                    #field_str,
                    move |mut caller: wiggle::wasmtime_crate::Caller<'_, T> #(, #arg_decls)*| -> wiggle::anyhow::Result<#ret_ty> {
                        let result = async { #body };
                        #block_with(result)?
                    },
                )?;
            }
        }

        Asyncness::Sync => {
            quote! {
                linker.func_wrap(
                    #module_str,
                    #field_str,
                    move |mut caller: wiggle::wasmtime_crate::Caller<'_, T> #(, #arg_decls)*| -> wiggle::anyhow::Result<#ret_ty> {
                        #body
                    },
                )?;
            }
        }
    }
}
