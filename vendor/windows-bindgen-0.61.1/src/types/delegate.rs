use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Delegate {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}

impl Delegate {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_cfg(&self, config: &Config<'_>) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        Cfg::new(self.def, &self.dependencies(), config).write(config, false)
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        let name = self.write_name(config);
        let vtbl_name: TokenStream = format!("{}_Vtbl", self.def.name()).into();
        let boxed: TokenStream = format!("{}Box", self.def.name()).into();
        let generic_names = self.generics.iter().map(|ty| ty.write_name(config));
        let generic_names = quote! { #(#generic_names,)* };

        let constraints = config.write_generic_constraints(&self.generics);
        let named_phantoms = config.write_generic_named_phantoms(&self.generics);
        let method = self.method();
        let cfg = self.write_cfg(config);

        let invoke = method.write(
            config,
            None,
            InterfaceKind::Default,
            &mut MethodNames::new(),
            &mut MethodNames::new(),
        );

        let invoke_vtbl = method.write_abi(config, true);

        let definition = if self.generics.is_empty() {
            let guid = config.write_guid_u128(&self.def.guid_attribute().unwrap());

            quote! {
                #cfg
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                #cfg
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
            }
        } else {
            let phantoms = config.write_generic_phantoms(&self.generics);

            let guid = self.def.guid_attribute().unwrap();
            let pinterface = Literal::byte_string(&format!("pinterface({{{guid}}}"));

            let generics = self.generics.iter().map(|generic| {
                let name = generic.write_name(config);
                quote! {
                    .push_slice(b";").push_other(#name::SIGNATURE)
                }
            });

            quote! {
                #cfg
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                #cfg
                unsafe impl<#constraints> windows_core::Interface for #name {
                    type Vtable = #vtbl_name<#generic_names>;
                    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
                }
                #cfg
                impl<#constraints> windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(#pinterface)#(#generics)*.push_slice(b")");
                }
            }
        };

        let fn_constraint = {
            let signature = method.write_impl_signature(config, false, false);

            quote! { F: FnMut #signature + Send + 'static }
        };

        let invoke_upcall = method.write_upcall(quote! { (this.invoke) }, false);

        quote! {
            #definition
            #cfg
            impl<#constraints> #name {
                pub fn new<#fn_constraint>(invoke: F) -> Self {
                    let com = #boxed {
                        vtable: &#boxed::<#generic_names F>::VTABLE,
                        count: windows_core::imp::RefCount::new(1),
                        invoke,
                    };
                    unsafe {
                        core::mem::transmute(windows_core::imp::Box::new(com))
                    }
                }
                #invoke
            }
            #cfg
            #[repr(C)]
            #[doc(hidden)]
            pub struct #vtbl_name<#generic_names> where #constraints {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(#invoke_vtbl) -> windows_core::HRESULT,
                #named_phantoms
            }
            #cfg
            #[repr(C)]
            struct #boxed<#generic_names #fn_constraint> where #constraints {
                vtable: *const #vtbl_name<#generic_names>,
                invoke: F,
                count: windows_core::imp::RefCount,
            }
            #cfg
            impl<#constraints #fn_constraint> #boxed<#generic_names F> {
                const VTABLE: #vtbl_name<#generic_names> = #vtbl_name::<#generic_names>{
                    base__: windows_core::IUnknown_Vtbl{QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release},
                    Invoke: Self::Invoke,
                    #named_phantoms
                };
                unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
                    unsafe {
                        let this = this as *mut *mut core::ffi::c_void as *mut Self;

                        if iid.is_null() || interface.is_null() {
                            return windows_core::HRESULT(-2147467261); // E_POINTER
                        }

                        *interface = if *iid == <#name as windows_core::Interface>::IID ||
                            *iid == <windows_core::IUnknown as windows_core::Interface>::IID ||
                            *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID {
                                &mut (*this).vtable as *mut _ as _
                            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                                (*this).count.add_ref();
                                return windows_core::imp::marshaler(core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void), interface);
                            } else {
                                core::ptr::null_mut()
                            };

                        if (*interface).is_null() {
                            windows_core::HRESULT(-2147467262) // E_NOINTERFACE
                        } else {
                            (*this).count.add_ref();
                            windows_core::HRESULT(0)
                        }
                    }
                }
                unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
                    unsafe {
                        let this = this as *mut *mut core::ffi::c_void as *mut Self;
                        (*this).count.add_ref()
                    }
                }
                unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
                    unsafe {
                        let this = this as *mut *mut core::ffi::c_void as *mut Self;
                        let remaining = (*this).count.release();

                        if remaining == 0 {
                            let _ = windows_core::imp::Box::from_raw(this);
                        }

                        remaining
                    }
                }
                unsafe extern "system" fn Invoke(#invoke_vtbl) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                        #invoke_upcall
                    }
                }
            }
        }
    }

    pub fn method(&self) -> Method {
        Method::new(
            self.def
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap(),
            &self.generics,
        )
    }

    pub fn runtime_signature(&self) -> String {
        if self.generics.is_empty() {
            let guid = self.def.guid_attribute().unwrap();
            format!("delegate({{{guid}}})")
        } else {
            interface_signature(self.def, &self.generics)
        }
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &self.generics)
    }
}

impl Dependencies for Delegate {
    fn combine(&self, dependencies: &mut TypeMap) {
        dependencies.combine(&self.method().dependencies);

        for ty in &self.generics {
            ty.combine(dependencies);
        }
    }
}
