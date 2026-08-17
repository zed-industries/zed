use super::*;

impl Config<'_> {
    pub fn write_cpp_handle(&self, def: TypeDef) -> TokenStream {
        let tn = def.type_name();
        let name = to_ident(def.name());
        let ty = def.underlying_type();
        let ty_name = ty.write_name(self);

        if self.sys {
            quote! {
                pub type #name = #ty_name;
            }
        } else {
            let mut derive = quote! { Clone, Copy, Debug, PartialEq, Eq, };

            let default = if ty.is_pointer() {
                quote! {
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            } else {
                derive.combine(quote! { Default, });
                quote! {}
            };

            let invalid = def.invalid_values();

            let is_invalid = if ty.is_pointer() && (invalid.is_empty() || invalid == [0]) {
                quote! {
                    impl #name {
                        pub fn is_invalid(&self) -> bool {
                            self.0.is_null()
                        }
                    }
                }
            } else if invalid.is_empty() {
                quote! {}
            } else {
                let invalid = invalid.iter().map(|value| {
                    let literal = Literal::i64_unsuffixed(*value);

                    if ty.is_pointer() || (*value < 0 && ty.is_unsigned()) {
                        quote! { self.0 == #literal as _ }
                    } else {
                        quote! { self.0 == #literal }
                    }
                });
                quote! {
                    impl #name {
                        pub fn is_invalid(&self) -> bool {
                            #(#invalid)||*
                        }
                    }
                }
            };

            let free = if let Some(function) = def.free_function() {
                if is_invalid.is_empty() {
                    // TODO: https://github.com/microsoft/win32metadata/issues/1891
                    quote! {}
                } else {
                    let link = function.write_link(self, true);
                    let free = to_ident(function.method.name());
                    let signature = function.method.signature(def.namespace(), &[]);

                    // BCryptCloseAlgorithmProvider has an unused trailing parameter.
                    let tail = if signature.params.len() > 1 {
                        quote! { , 0 }
                    } else {
                        quote! {}
                    };

                    quote! {
                        impl windows_core::Free for #name {
                            #[inline]
                            unsafe fn free(&mut self) {
                                if !self.is_invalid() {
                                    #link
                                    unsafe { #free(self.0 #tail); }
                                }
                            }
                        }
                    }
                }
            } else {
                quote! {}
            };

            let must_use = if matches!(tn, TypeName::NTSTATUS | TypeName::RPC_STATUS) {
                quote! { #[must_use] }
            } else {
                quote! {}
            };

            let mut result = quote! {
                #must_use
                #[repr(transparent)]
                #[derive(#derive)]
                pub struct #name(pub #ty_name);
                #is_invalid
                #free
                #default
            };

            if let Some(attribute) = def.find_attribute("AlsoUsableForAttribute") {
                if let Some((_, Value::Str(type_name))) = attribute.args().first() {
                    let ty = def.reader().unwrap_full_name(def.namespace(), type_name);

                    let ty = ty.write_name(self);

                    result.combine(quote! {
                        impl windows_core::imp::CanInto<#ty> for #name {}
                        impl From<#name> for #ty {
                            fn from(value: #name) -> Self {
                                Self(value.0)
                            }
                        }
                    });
                }
            }

            result
        }
    }
}
