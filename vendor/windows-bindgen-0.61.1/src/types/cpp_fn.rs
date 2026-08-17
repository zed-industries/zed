use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppFn {
    pub namespace: &'static str,
    pub method: MethodDef,
}

impl Ord for CppFn {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.method.name(), self.method).cmp(&(other.method.name(), other.method))
    }
}

impl PartialOrd for CppFn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppFn {
    pub fn type_name(&self) -> TypeName {
        TypeName(self.namespace, self.method.name())
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write_link(&self, config: &Config<'_>, underlying_types: bool) -> TokenStream {
        let library = self.method.module_name().to_lowercase();
        let symbol = self.method.import_name();
        let name = to_ident(self.method.name());
        let abi = self.method.calling_convention();
        let signature = self.method.signature(self.namespace, &[]);

        let params = signature.params.iter().map(|param| {
            let name = param.write_ident();
            let ty = if underlying_types {
                param.underlying_type().write_abi(config)
            } else {
                param.write_abi(config)
            };
            quote! { #name: #ty }
        });

        let return_sig = config.write_return_sig(self.method, &signature, underlying_types);

        let vararg = if config.sys && signature.call_flags.contains(MethodCallAttributes::VARARG) {
            quote! { , ... }
        } else {
            quote! {}
        };

        let link = to_ident(config.link);

        link_fmt(quote! {
            #link::link!(#library #abi #symbol fn #name(#(#params),* #vararg) #return_sig);
        })
    }

    pub fn write_cfg(&self, config: &Config<'_>) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        Cfg::new(self.method, &self.dependencies(), config).write(config, false)
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        let name = to_ident(self.method.name());
        let signature = self.method.signature(self.namespace, &[]);

        let link = self.write_link(config, false);
        let arches = write_arches(self.method);
        let cfg = self.write_cfg(config);
        let cfg = quote! { #arches #cfg };
        let window_long = self.write_window_long();

        if config.sys {
            return quote! {
                #cfg
                #link
                #window_long
            };
        }

        let method = CppMethod::new(self.method, self.namespace);
        let args = method.write_args();
        let params = method.write_params(config);
        let generics = method.write_generics();
        let abi_return_type = method.write_return(config);

        let wrapper = match method.return_hint {
            ReturnHint::Query(..) => {
                let where_clause = method.write_where(config, true);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics T>(#params) -> windows_core::Result<T> #where_clause {
                        #link
                        let mut result__ = core::ptr::null_mut();
                        unsafe { #name(#args).and_then(||windows_core::Type::from_abi(result__)) }
                    }
                }
            }
            ReturnHint::QueryOptional(..) => {
                let where_clause = method.write_where(config, true);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics T>(#params result__: *mut Option<T>) -> windows_core::Result<()> #where_clause {
                        #link
                        unsafe { #name(#args).ok() }
                    }
                }
            }
            ReturnHint::ResultValue => {
                let where_clause = method.write_where(config, false);
                let return_type = signature.params[signature.params.len() - 1].deref();
                let map = return_type.write_result_map();
                let return_type = return_type.write_name(config);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                        #link
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            #name(#args).#map
                        }
                    }
                }
            }
            ReturnHint::ResultVoid => {
                let where_clause = method.write_where(config, false);

                quote! {
                    #cfg
                    #[inline]
                    pub unsafe fn #name<#generics>(#params) -> windows_core::Result<()> #where_clause {
                        #link
                        unsafe { #name(#args).ok() }
                    }
                }
            }
            ReturnHint::ReturnValue => {
                let where_clause = method.write_where(config, false);

                let return_type =
                    method.signature.params[method.signature.params.len() - 1].deref();

                if return_type.is_interface() {
                    let return_type = return_type.write_name(config);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                            #link
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                #name(#args);
                                windows_core::Type::from_abi(result__)
                            }
                        }
                    }
                } else {
                    let map = if return_type.is_copyable() {
                        quote! { result__ }
                    } else {
                        quote! { core::mem::transmute(result__) }
                    };

                    let where_clause = method.write_where(config, false);
                    let return_type = return_type.write_name(config);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> #return_type #where_clause {
                            #link
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                #name(#args);
                                #map
                            }
                        }
                    }
                }
            }
            ReturnHint::ReturnStruct | ReturnHint::None => {
                let where_clause = method.write_where(config, false);

                if method.handle_last_error() {
                    let return_type = signature.return_type.write_name(config);

                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) -> windows_core::Result<#return_type> #where_clause {
                            #link
                            let result__ = unsafe { #name(#args) };
                            (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
                        }
                    }
                } else {
                    quote! {
                        #cfg
                        #[inline]
                        pub unsafe fn #name<#generics>(#params) #abi_return_type #where_clause {
                            #link
                            unsafe { #name(#args) }
                        }
                    }
                }
            }
        };

        quote! {
            #wrapper
            #window_long
        }
    }

    fn write_window_long(&self) -> TokenStream {
        match self.method.name() {
            "GetWindowLongPtrA" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use GetWindowLongA as GetWindowLongPtrA;
            },
            "GetWindowLongPtrW" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use GetWindowLongW as GetWindowLongPtrW;
            },
            "SetWindowLongPtrA" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use SetWindowLongA as SetWindowLongPtrA;
            },
            "SetWindowLongPtrW" => quote! {
                #[cfg(target_pointer_width = "32")]
                pub use SetWindowLongW as SetWindowLongPtrW;
            },
            _ => quote! {},
        }
    }
}

impl Dependencies for CppFn {
    fn combine(&self, dependencies: &mut TypeMap) {
        self.method
            .signature(self.namespace, &[])
            .combine(dependencies);

        let dependency = match self.method.name() {
            "GetWindowLongPtrA" => Some("GetWindowLongA"),
            "GetWindowLongPtrW" => Some("GetWindowLongW"),
            "SetWindowLongPtrA" => Some("SetWindowLongA"),
            "SetWindowLongPtrW" => Some("SetWindowLongW"),
            _ => None,
        };

        if let Some(dependency) = dependency {
            self.method
                .reader()
                .unwrap_full_name(self.namespace, dependency)
                .combine(dependencies);
        }
    }
}

impl Config<'_> {
    pub fn write_return_sig(
        &self,
        method: MethodDef,
        signature: &Signature,
        underlying_types: bool,
    ) -> TokenStream {
        match &signature.return_type {
            Type::Void => {
                if method.has_attribute("DoesNotReturnAttribute") {
                    quote! { -> ! }
                } else {
                    quote! {}
                }
            }
            ty => {
                let ty = if underlying_types {
                    ty.underlying_type().write_default(self)
                } else {
                    ty.write_default(self)
                };

                quote! { -> #ty }
            }
        }
    }
}

fn link_fmt(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.0.replacen(" ! (  ", "!(", 1);
    tokens = tokens.replacen(" ( ", "(", 1);
    tokens = tokens.replace(" , ", ", ");
    tokens = tokens.replace(" )", ")");
    tokens.into()
}
