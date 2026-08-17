use super::*;

#[derive(Clone, Debug)]
pub enum CppMethodOrName {
    Method(CppMethod),
    Name(MethodDef),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CppInterface {
    pub def: TypeDef,
}

impl Ord for CppInterface {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.name().cmp(other.def.name())
    }
}

impl PartialOrd for CppInterface {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppInterface {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn get_methods(&self, config: &Config<'_>) -> Vec<CppMethodOrName> {
        let namespace = self.def.namespace();

        self.def
            .methods()
            .map(|def| {
                let method = CppMethod::new(def, namespace);
                if method.dependencies.included(config) {
                    CppMethodOrName::Method(method)
                } else {
                    config
                        .warnings
                        .skip_method(method.def, &method.dependencies, config);
                    CppMethodOrName::Name(method.def)
                }
            })
            .collect()
    }

    fn write_cfg(&self, config: &Config<'_>) -> (Cfg, TokenStream) {
        if !config.package {
            return (Cfg::default(), quote! {});
        }

        let cfg = Cfg::new(self.def, &self.dependencies(), config);
        let tokens = cfg.write(config, false);
        (cfg, tokens)
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        let methods = self.get_methods(config);

        let base_interfaces = self.base_interfaces();
        let has_unknown_base = matches!(base_interfaces.first(), Some(Type::IUnknown));
        let (class_cfg, cfg) = self.write_cfg(config);
        let vtbl_name = self.write_vtbl_name(config);

        let vtbl = {
            let core = config.write_core();

            let base = match base_interfaces.last() {
                Some(Type::IUnknown) => {
                    quote! { pub base__: #core IUnknown_Vtbl, }
                }
                Some(Type::Object) => {
                    quote! { pub base__: #core IInspectable_Vtbl, }
                }
                Some(Type::CppInterface(ty)) => {
                    let name = ty.write_vtbl_name(config);
                    quote! { pub base__: #name, }
                }
                _ => quote! {},
            };

            let mut names = MethodNames::new();

            let methods = methods.iter().map(|method| match method {
                CppMethodOrName::Method(method) => {
                    let method_cfg = class_cfg.difference(method.def, &method.dependencies, config);
                    let yes = method_cfg.write(config, false);

                    let name = names.add(method.def);
                    let abi = method.write_abi(config, false);

                    if yes.is_empty() {
                        quote! {
                            pub #name: unsafe extern "system" fn #abi,
                        }
                    } else {
                        let no = method_cfg.write(config, true);

                        quote! {
                            #yes
                            pub #name: unsafe extern "system" fn #abi,
                            #no
                            #name: usize,
                        }
                    }
                }
                CppMethodOrName::Name(method) => {
                    let name = names.add(*method);
                    quote! { #name: usize, }
                }
            });

            let hide_vtbl = if config.sys {
                quote! {}
            } else {
                quote! { #[doc(hidden)] }
            };

            quote! {
                #cfg
                #[repr(C)]
                #hide_vtbl
                pub struct #vtbl_name {
                    #base
                    #(#methods)*
                }
            }
        };

        if config.sys {
            let mut result = quote! {};

            if !config.package {
                if has_unknown_base {
                    if let Some(guid) = self.def.guid_attribute() {
                        let name: TokenStream = format!("IID_{}", self.def.name()).into();
                        result.combine(config.write_cpp_const_guid(name, &guid));
                    }
                }

                result.combine(vtbl);
            }

            result
        } else {
            let name = to_ident(self.def.name());

            let mut result = if has_unknown_base {
                if let Some(guid) = self.def.guid_attribute() {
                    let guid = config.write_guid_u128(&guid);

                    quote! {
                        #cfg
                        windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                    }
                } else {
                    quote! {
                        #cfg
                        windows_core::imp::define_interface!(#name, #vtbl_name, 0);
                    }
                }
            } else {
                quote! {
                    #cfg
                    windows_core::imp::define_interface!(#name, #vtbl_name);
                }
            };

            if let Some(Type::CppInterface(base)) = base_interfaces.last() {
                let base = base.write_name(config);

                result.combine(quote! {
                    #cfg
                    impl core::ops::Deref for #name {
                        type Target = #base;
                        fn deref(&self) -> &Self::Target {
                            unsafe { core::mem::transmute(self) }
                        }
                    }
                });
            }

            if !base_interfaces.is_empty() {
                let bases = base_interfaces.iter().map(|ty| ty.write_name(config));
                result.combine(quote! {
                    #cfg
                    windows_core::imp::interface_hierarchy!(#name, #(#bases),*);
                })
            }

            let method_names = &mut MethodNames::new();
            let virtual_names = &mut MethodNames::new();
            let mut methods_tokens = quote! {};

            for method in methods.iter().filter_map(|method| match &method {
                CppMethodOrName::Method(method) => Some(method),
                _ => None,
            }) {
                let cfg = method.write_cfg(config, &class_cfg, false);
                let method = method.write(config, method_names, virtual_names);

                methods_tokens.combine(quote! {
                    #cfg
                    #method
                });
            }

            if !methods_tokens.is_empty() {
                result.combine(quote! {
                    #cfg
                    impl #name {
                        #methods_tokens
                    }
                });
            }

            result.combine(vtbl);

            if self.def.is_agile() {
                result.combine(quote! {
                    #cfg
                    unsafe impl Send for #name {}
                    #cfg
                    unsafe impl Sync for #name {}
                });
            }

            let impl_name: TokenStream = format!("{}_Impl", self.def.name()).into();

            let cfg = if config.package {
                fn combine(
                    interface: &CppInterface,
                    dependencies: &mut TypeMap,
                    config: &Config<'_>,
                ) {
                    for method in interface.get_methods(config).iter() {
                        if let CppMethodOrName::Method(method) = method {
                            dependencies.combine(&method.dependencies);
                        }
                    }
                }

                let mut dependencies = self.dependencies();
                combine(self, &mut dependencies, config);

                base_interfaces.iter().for_each(|interface| {
                    if let Type::CppInterface(ty) = interface {
                        combine(ty, &mut dependencies, config);
                    }
                });

                Cfg::new(self.def, &dependencies, config).write(config, false)
            } else {
                quote! {}
            };

            let mut names = MethodNames::new();

            let field_methods: Vec<_> = methods
                .iter()
                .map(|method| match method {
                    CppMethodOrName::Method(method) => {
                        let name = names.add(method.def);
                        if has_unknown_base {
                            quote! { #name: #name::<Identity, OFFSET>, }
                        } else {
                            quote! { #name: #name::<Identity>, }
                        }
                    }
                    CppMethodOrName::Name(method) => {
                        let name = names.add(*method);
                        quote! { #name: 0, }
                    }
                })
                .collect();

            let mut names = MethodNames::new();

            let impl_methods: Vec<_> = methods.iter().map(|method| match method {
                CppMethodOrName::Method(method) => {
                    let name = names.add(method.def);
                    let signature = method.write_abi(config, true);
                    let upcall = method.write_upcall(&impl_name, &name);

                    if has_unknown_base {
                    quote! {
                        unsafe extern "system" fn #name<Identity: #impl_name, const OFFSET: isize> #signature {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                #upcall
                            }
                        }
                    }
                } else {
                    quote! {
                        unsafe extern "system" fn #name<Identity: #impl_name> #signature {
                            unsafe {
                                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                                let this = &*((*this).this as *const Identity);
                                #upcall
                            }
                        }
                    }
                }
                }
                _ => quote! {},
            }).collect();

            let mut names = MethodNames::new();

            let trait_methods: Vec<_> = methods
                .iter()
                .map(|method| match method {
                    CppMethodOrName::Method(method) => {
                        let name = names.add(method.def);
                        let signature = method.write_impl_signature(config, true);
                        quote! { fn #name #signature; }
                    }
                    _ => quote! {},
                })
                .collect();

            let impl_base = base_interfaces.last().map(|ty| ty.write_impl_name(config));

            let field_base = base_interfaces.last().map(|ty|{
                match ty {
                    Type::IUnknown => quote! { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), },
                    Type::Object => quote! { base__: windows_core::IInspectable_Vtbl::new::<Identity, #name, OFFSET>(), },
                    Type::CppInterface(ty) => {
                        let ty = ty.write_vtbl_name(config);
                        if has_unknown_base {
                            quote! { base__: #ty::new::<Identity, OFFSET>(), }
                        } else {
                            quote! { base__: #ty::new::<Identity>(), }
                        }
                    }
                    rest => panic!("{rest:?}"),
                }
            });

            result.combine( if has_unknown_base {
                let matches = base_interfaces.iter().filter_map(|ty|{
                    match ty {
                        Type::CppInterface(ty) => {
                            let name = ty.write_name(config);
                            Some(quote! { || iid == &<#name as windows_core::Interface>::IID })
                        }
                        _ => None,
                    }
                });

                quote! {
                    #cfg
                    pub trait #impl_name: #impl_base {
                        #(#trait_methods)*
                    }
                    #cfg
                    impl #vtbl_name {
                        pub const fn new<Identity: #impl_name, const OFFSET: isize>() -> Self {
                            #(#impl_methods)*
                            Self {
                                #field_base
                                #(#field_methods)*
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<#name as windows_core::Interface>::IID #(#matches)*
                        }
                    }
                    #cfg
                    impl windows_core::RuntimeName for #name {}
                }
            } else {
                let implvtbl_ident = impl_name.join("Vtbl");

                quote! {
                    #cfg
                    pub trait #impl_name : #impl_base {
                        #(#trait_methods)*
                    }
                    #cfg
                    impl #vtbl_name {
                        pub const fn new<Identity: #impl_name>() -> Self {
                            #(#impl_methods)*
                            Self{
                                #field_base
                                #(#field_methods)*
                            }
                        }
                    }
                    #cfg
                    struct #implvtbl_ident<T: #impl_name> (core::marker::PhantomData<T>);
                    #cfg
                    impl<T: #impl_name> #implvtbl_ident<T> {
                        const VTABLE: #vtbl_name = #vtbl_name::new::<T>();
                    }
                    #cfg
                    impl #name {
                        pub fn new<'a, T: #impl_name>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
                            let this = windows_core::ScopedHeap { vtable: &#implvtbl_ident::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                            let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
                            unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
                        }
                    }
                }
            });

            result
        }
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &[])
    }

    fn write_vtbl_name(&self, config: &Config<'_>) -> TokenStream {
        let name: TokenStream = format!("{}_Vtbl", self.def.name()).into();
        let namespace = config.write_namespace(self.def.type_name());
        quote! { #namespace #name }
    }

    pub fn write_impl_name(&self, config: &Config<'_>) -> TokenStream {
        let name: TokenStream = format!("{}_Impl", self.def.name()).into();
        let namespace = config.write_namespace(self.def.type_name());
        quote! { #namespace #name }
    }

    pub fn base_interfaces(&self) -> Vec<Type> {
        let mut bases = vec![];
        let mut def = self.def;

        while let Some(base) = def.interface_impls().map(move |imp| imp.ty(&[])).next() {
            match base {
                Type::CppInterface(ref ty) => {
                    def = ty.def;
                    bases.insert(0, base);
                }
                Type::Object => {
                    bases.insert(0, Type::IUnknown);
                    bases.insert(1, Type::Object);
                    break;
                }
                Type::IUnknown => {
                    bases.insert(0, Type::IUnknown);
                    break;
                }
                rest => panic!("{rest:?}"),
            }
        }

        bases
    }
}

impl Dependencies for CppInterface {
    fn combine(&self, dependencies: &mut TypeMap) {
        let base_interfaces = self.base_interfaces();

        for interface in &base_interfaces {
            interface.combine(dependencies);
        }

        for method in self.def.methods() {
            for ty in method.signature(self.def.namespace(), &[]).types() {
                if ty.is_core() {
                    ty.combine(dependencies);
                }
            }
        }
    }
}
