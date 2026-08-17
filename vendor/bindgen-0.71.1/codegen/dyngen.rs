use crate::codegen;
use crate::ir::context::BindgenContext;
use crate::ir::function::ClangAbi;
use proc_macro2::{Ident, TokenStream};

/// Used to build the output tokens for dynamic bindings.
#[derive(Default)]
pub(crate) struct DynamicItems {
    /// Tracks the tokens that will appears inside the library struct -- e.g.:
    /// ```ignore
    /// struct Lib {
    ///    __library: ::libloading::Library,
    ///    pub x: Result<unsafe extern ..., ::libloading::Error>, // <- tracks these
    ///    ...
    /// }
    /// ```
    struct_members: Vec<TokenStream>,

    /// Tracks the tokens that will appear inside the library struct's implementation, e.g.:
    ///
    /// ```ignore
    /// impl Lib {
    ///     ...
    ///     pub unsafe fn foo(&self, ...) { // <- tracks these
    ///         ...
    ///     }
    /// }
    /// ```
    struct_implementation: Vec<TokenStream>,

    /// Tracks the initialization of the fields inside the `::new` constructor of the library
    /// struct, e.g.:
    /// ```ignore
    /// impl Lib {
    ///
    ///     pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    ///     where
    ///         P: AsRef<::std::ffi::OsStr>,
    ///     {
    ///         ...
    ///         let foo = __library.get(...) ...; // <- tracks these
    ///         ...
    ///     }
    ///
    ///     ...
    /// }
    /// ```
    constructor_inits: Vec<TokenStream>,

    /// Tracks the information that is passed to the library struct at the end of the `::new`
    /// constructor, e.g.:
    /// ```ignore
    /// impl LibFoo {
    ///     pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    ///     where
    ///         P: AsRef<::std::ffi::OsStr>,
    ///     {
    ///         ...
    ///         Ok(LibFoo {
    ///             __library: __library,
    ///             foo,
    ///             bar, // <- tracks these
    ///             ...
    ///         })
    ///     }
    /// }
    /// ```
    init_fields: Vec<TokenStream>,
}

impl DynamicItems {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn get_tokens(
        &self,
        lib_ident: Ident,
        ctx: &BindgenContext,
    ) -> TokenStream {
        let struct_members = &self.struct_members;
        let constructor_inits = &self.constructor_inits;
        let init_fields = &self.init_fields;
        let struct_implementation = &self.struct_implementation;

        let library_new = if ctx.options().wrap_unsafe_ops {
            quote!(unsafe { ::libloading::Library::new(path) })
        } else {
            quote!(::libloading::Library::new(path))
        };

        let from_library = if ctx.options().wrap_unsafe_ops {
            quote!(unsafe { Self::from_library(library) })
        } else {
            quote!(Self::from_library(library))
        };

        quote! {
            pub struct #lib_ident {
                __library: ::libloading::Library,
                #(#struct_members)*
            }

            impl #lib_ident {
                pub unsafe fn new<P>(
                    path: P
                ) -> Result<Self, ::libloading::Error>
                where P: AsRef<::std::ffi::OsStr> {
                    let library = #library_new?;
                    #from_library
                }

                pub unsafe fn from_library<L>(
                    library: L
                ) -> Result<Self, ::libloading::Error>
                where L: Into<::libloading::Library> {
                    let __library = library.into();
                    #( #constructor_inits )*
                    Ok(#lib_ident {
                        __library,
                        #( #init_fields ),*
                    })
                }

                #( #struct_implementation )*
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn push_func(
        &mut self,
        ident: Ident,
        abi: ClangAbi,
        is_variadic: bool,
        is_required: bool,
        args: Vec<TokenStream>,
        args_identifiers: Vec<TokenStream>,
        ret: TokenStream,
        ret_ty: TokenStream,
        attributes: Vec<TokenStream>,
        ctx: &BindgenContext,
    ) {
        if !is_variadic {
            assert_eq!(args.len(), args_identifiers.len());
        }

        let signature = quote! { unsafe extern #abi fn ( #( #args),* ) #ret };
        let member = if is_required {
            signature
        } else {
            quote! { Result<#signature, ::libloading::Error> }
        };

        self.struct_members.push(quote! {
            pub #ident: #member,
        });

        // N.B: If the signature was required, it won't be wrapped in a Result<...>
        //      and we can simply call it directly.
        let fn_ = if is_required {
            quote! { self.#ident }
        } else {
            quote! { self.#ident.as_ref().expect("Expected function, got error.") }
        };
        let call_body = if ctx.options().wrap_unsafe_ops {
            quote!(unsafe { (#fn_)(#( #args_identifiers ),*) })
        } else {
            quote!((#fn_)(#( #args_identifiers ),*) )
        };

        // We can't implement variadic functions from C easily, so we allow to
        // access the function pointer so that the user can call it just fine.
        if !is_variadic {
            self.struct_implementation.push(quote! {
                #(#attributes)*
                pub unsafe fn #ident ( &self, #( #args ),* ) #ret_ty {
                    #call_body
                }
            });
        }

        // N.B: Unwrap the signature upon construction if it is required to be resolved.
        let ident_str = codegen::helpers::ast_ty::cstr_expr(ident.to_string());
        let library_get = if ctx.options().wrap_unsafe_ops {
            quote!(unsafe { __library.get(#ident_str) })
        } else {
            quote!(__library.get(#ident_str))
        };

        self.constructor_inits.push(if is_required {
            quote! {
                let #ident = #library_get.map(|sym| *sym)?;
            }
        } else {
            quote! {
                let #ident = #library_get.map(|sym| *sym);
            }
        });

        self.init_fields.push(quote! {
            #ident
        });
    }

    pub fn push_var(
        &mut self,
        ident: Ident,
        ty: TokenStream,
        is_required: bool,
        wrap_unsafe_ops: bool,
    ) {
        let member = if is_required {
            quote! { *mut #ty }
        } else {
            quote! { Result<*mut #ty, ::libloading::Error> }
        };

        self.struct_members.push(quote! {
            pub #ident: #member,
        });

        let deref = if is_required {
            quote! { self.#ident }
        } else {
            quote! { *self.#ident.as_ref().expect("Expected variable, got error.") }
        };
        self.struct_implementation.push(quote! {
            pub unsafe fn #ident (&self) -> *mut #ty {
                #deref
            }
        });

        let ident_str = codegen::helpers::ast_ty::cstr_expr(ident.to_string());

        let library_get = if wrap_unsafe_ops {
            quote!(unsafe { __library.get::<*mut #ty>(#ident_str) })
        } else {
            quote!(__library.get::<*mut #ty>(#ident_str))
        };

        let qmark = if is_required { quote!(?) } else { quote!() };

        let var_get = quote! {
            let #ident = #library_get.map(|sym| *sym)#qmark;
        };

        self.constructor_inits.push(var_get);

        self.init_fields.push(quote! {
            #ident
        });
    }
}
