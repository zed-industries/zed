use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterfaceKind {
    None,
    Default,
    Static,
    Composable,
    Base,
}

#[derive(Clone, Debug)]
pub enum MethodOrName {
    Method(Method),
    Name(MethodDef),
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub def: TypeDef,
    pub generics: Vec<Type>,
    pub kind: InterfaceKind,
}

impl PartialEq for Interface {
    fn eq(&self, other: &Self) -> bool {
        self.def == other.def
    }
}

impl Eq for Interface {}

impl std::hash::Hash for Interface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.def.hash(state);
    }
}

impl Ord for Interface {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.def.name(), self.def).cmp(&(other.def.name(), other.def))
    }
}

impl PartialOrd for Interface {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Interface {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn get_methods(&self, config: &Config<'_>) -> Vec<MethodOrName> {
        self.def
            .methods()
            .map(|def| {
                let method = Method::new(def, &self.generics);
                if method.dependencies.included(config) {
                    MethodOrName::Method(method)
                } else {
                    config
                        .warnings
                        .skip_method(method.def, &method.dependencies, config);
                    MethodOrName::Name(method.def)
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
        let type_name = self.def.type_name();
        let methods = self.get_methods(config);

        let required_interfaces = self.required_interfaces();
        let name = self.write_name(config);

        let vtbl_name = self.write_vtbl_name(config);
        let is_exclusive = self.is_exclusive();
        let constraints = config.write_generic_constraints(&self.generics);
        let phantoms = config.write_generic_phantoms(&self.generics);
        let named_phantoms = config.write_generic_named_phantoms(&self.generics);
        let (class_cfg, cfg) = self.write_cfg(config);

        let vtbl = {
            let virtual_names = &mut MethodNames::new();
            let core = config.write_core();

            let vtbl_methods = methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let name = virtual_names.add(method.def);
                    let vtbl = method.write_abi(config, false);

                    let method_cfg = class_cfg.difference(method.def, &method.dependencies, config);
                    let yes = method_cfg.write(config, false);

                    if yes.is_empty() {
                        quote! {
                            pub #name: unsafe extern "system" fn(#vtbl) -> #core HRESULT,
                        }
                    } else {
                        let no = method_cfg.write(config, true);

                        quote! {
                            #yes
                            pub #name: unsafe extern "system" fn(#vtbl) -> #core HRESULT,
                            #no
                            #name: usize,
                        }
                    }
                }
                MethodOrName::Name(method) => {
                    let name = virtual_names.add(*method);
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
                pub struct #vtbl_name where #constraints {
                    pub base__: #core IInspectable_Vtbl,
                    #(#vtbl_methods)*
                    #named_phantoms
                }
            }
        };

        if config.sys {
            let mut result = quote! {};

            if !config.package {
                if let Some(guid) = self.def.guid_attribute() {
                    let name: TokenStream = format!("IID_{}", self.def.name()).into();
                    result.combine(config.write_cpp_const_guid(name, &guid));
                }

                result.combine(vtbl);
            }

            result
        } else {
            let mut result = if self.generics.is_empty() {
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
                let guid = self.def.guid_attribute().unwrap();
                let pinterface = Literal::byte_string(&format!("pinterface({{{guid}}}"));

                let generics = self.generics.iter().map(|generic| {
                    let name = generic.write_name(config);

                    quote! {
                        .push_slice(b";").push_other(#name::SIGNATURE)
                    }
                });

                quote! {
                    #[repr(transparent)]
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                    impl<#constraints> windows_core::imp::CanInto<windows_core::IUnknown> for #name {}
                    impl<#constraints> windows_core::imp::CanInto<windows_core::IInspectable> for #name {}
                    unsafe impl<#constraints> windows_core::Interface for #name {
                        type Vtable = #vtbl_name;
                        const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
                    }
                    impl<#constraints> windows_core::RuntimeType for #name {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(#pinterface)#(#generics)*.push_slice(b")");
                    }
                }
            };

            if !is_exclusive && self.generics.is_empty() {
                result.combine(quote! {
                #cfg
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
            });
            }

            if !is_exclusive && !required_interfaces.is_empty() {
                if self.generics.is_empty() {
                    let interfaces = required_interfaces.iter().map(|ty| ty.write_name(config));

                    result.combine(quote! {
                        #cfg
                        windows_core::imp::required_hierarchy!(#name, #(#interfaces),*);
                    });
                } else {
                    let interfaces = required_interfaces.iter().map(|ty| {
                    let ty = ty.write_name(config);
                    quote!{
                        impl<#constraints> windows_core::imp::CanInto<#ty> for #name { const QUERY: bool = true; }
                    }
                });

                    result.combine(quote! {
                        #(#interfaces)*
                    });
                }
            }

            if !is_exclusive {
                let method_names = &mut MethodNames::new();
                let virtual_names = &mut MethodNames::new();
                let mut method_tokens = TokenStream::new();

                for method in methods.iter().filter_map(|method| match &method {
                    MethodOrName::Method(method) => Some(method),
                    _ => None,
                }) {
                    let cfg = method.write_cfg(config, &class_cfg, false);

                    let method = method.write(
                        config,
                        Some(self),
                        InterfaceKind::Default,
                        method_names,
                        virtual_names,
                    );

                    method_tokens.combine(quote! {
                        #cfg
                        #method
                    });
                }

                for interface in &required_interfaces {
                    let virtual_names = &mut MethodNames::new();

                    for method in
                        interface
                            .get_methods(config)
                            .iter()
                            .filter_map(|method| match &method {
                                MethodOrName::Method(method) => Some(method),
                                _ => None,
                            })
                    {
                        let cfg = method.write_cfg(config, &class_cfg, false);

                        let method = method.write(
                            config,
                            Some(interface),
                            interface.kind,
                            method_names,
                            virtual_names,
                        );

                        method_tokens.combine(quote! {
                            #cfg
                            #method
                        });
                    }
                }

                if !method_tokens.is_empty() {
                    result.combine(quote! {
                        #cfg
                        impl<#constraints> #name {
                            #method_tokens
                        }
                    });
                }

                if self.def.is_agile() {
                    result.combine(quote! {
                        #cfg
                        unsafe impl<#constraints> Send for #name {}
                        #cfg
                        unsafe impl<#constraints> Sync for #name {}
                    });
                }

                if let Some(into_iterator) = required_interfaces
                    .iter()
                    .find(|interface| interface.type_name() == TypeName::IIterable)
                    .map(|interface| {
                        let ty = interface.generics[0].write_name(config);
                        let namespace = config.write_namespace(TypeName::IIterator);

                        quote! {
                            #cfg
                            impl<#constraints> IntoIterator for #name {
                                type Item = #ty;
                                type IntoIter = #namespace IIterator<Self::Item>;

                                fn into_iter(self) -> Self::IntoIter {
                                    IntoIterator::into_iter(&self)
                                }
                            }
                            #cfg
                            impl<#constraints> IntoIterator for &#name {
                                type Item = #ty;
                                type IntoIter = #namespace IIterator<Self::Item>;

                                fn into_iter(self) -> Self::IntoIter {
                                    self.First().unwrap()
                                }
                            }

                        }
                    })
                {
                    result.combine(into_iterator);
                }
            }

            if config.implement || !is_exclusive {
                let impl_name: TokenStream = format!("{}_Impl", self.def.name()).into();

                let generics: Vec<_> = self
                    .generics
                    .iter()
                    .map(|ty| ty.write_name(config))
                    .collect();

                let runtime_name = format!("{type_name}");

                let cfg = if config.package {
                    fn combine(
                        interface: &Interface,
                        dependencies: &mut TypeMap,
                        config: &Config<'_>,
                    ) {
                        for method in interface.get_methods(config).iter() {
                            if let MethodOrName::Method(method) = method {
                                dependencies.combine(&method.dependencies);
                            }
                        }
                    }

                    let mut dependencies = self.dependencies();
                    combine(self, &mut dependencies, config);

                    required_interfaces
                        .iter()
                        .for_each(|interface| combine(interface, &mut dependencies, config));

                    Cfg::new(self.def, &dependencies, config).write(config, false)
                } else {
                    quote! {}
                };

                result.combine(quote! {
                    #cfg
                    impl<#constraints> windows_core::RuntimeName for #name {
                        const NAME: &'static str = #runtime_name;
                    }
                });

                let mut names = MethodNames::new();

                let field_methods: Vec<_> = methods
                    .iter()
                    .map(|method| match method {
                        MethodOrName::Method(method) => {
                            let name = names.add(method.def);
                            quote! { #name: #name::<#(#generics,)* Identity, OFFSET>, }
                        }
                        MethodOrName::Name(method) => {
                            let name = names.add(*method);
                            quote! { #name: 0, }
                        }
                    })
                    .collect();

                let mut names = MethodNames::new();

                let impl_methods: Vec<_> = methods.iter().map(|method| match method {
                MethodOrName::Method(method) => {
                    let name = names.add(method.def);
                    let signature = method.write_abi(config, true);
                    let call = quote! { #impl_name::#name };
                    let upcall = method.write_upcall(call, true);

                    quote! {
                        unsafe extern "system" fn #name<#constraints Identity: #impl_name <#(#generics,)*>, const OFFSET: isize> (#signature) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                #upcall
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
                        MethodOrName::Method(method) => {
                            let name = names.add(method.def);
                            let signature = method.write_impl_signature(config, true, true);
                            quote! { fn #name #signature; }
                        }
                        _ => quote! {},
                    })
                    .collect();

                let requires = if required_interfaces.is_empty() {
                    quote! { windows_core::IUnknownImpl }
                } else {
                    let interfaces = required_interfaces
                        .iter()
                        .map(|ty| ty.write_impl_name(config));

                    quote! {  #(#interfaces)+* }
                };

                result.combine(quote! {
                #cfg
                pub trait #impl_name <#(#generics),*> : #requires where #constraints {
                    #(#trait_methods)*
                }
                #cfg
                impl<#constraints> #vtbl_name {
                    pub const fn new<Identity: #impl_name <#(#generics,)*>, const OFFSET: isize>() -> Self {
                        #(#impl_methods)*
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, #name, OFFSET>(),
                            #(#field_methods)*
                            #named_phantoms
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<#name as windows_core::Interface>::IID
                    }
                }
            });
            }

            result.combine(vtbl);
            result.combine(self.write_extensions());
            result
        }
    }

    fn write_extensions(&self) -> TokenStream {
        match self.type_name() {
            TypeName::IIterator => {
                quote! {
                    impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
                        type Item = T;

                        fn next(&mut self) -> Option<Self::Item> {
                            let result = if self.HasCurrent().unwrap_or(false) {
                                self.Current().ok()
                            } else {
                                None
                            };

                            if result.is_some() {
                                self.MoveNext().ok()?;
                            }

                            result
                        }
                    }
                }
            }
            TypeName::IIterable => {
                quote! {
                    impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
                        type Item = T;
                        type IntoIter = IIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            IntoIterator::into_iter(&self)
                        }
                    }
                    impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
                        type Item = T;
                        type IntoIter = IIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            self.First().unwrap()
                        }
                    }

                }
            }
            _ => quote! {},
        }
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &self.generics)
    }

    fn write_vtbl_name(&self, config: &Config<'_>) -> TokenStream {
        let name: TokenStream = format!("{}_Vtbl", self.def.name()).into();

        if self.generics.is_empty() {
            name
        } else {
            let generics = self.generics.iter().map(|ty| ty.write_name(config));
            quote! { #name < #(#generics,)* > }
        }
    }

    pub fn write_impl_name(&self, config: &Config<'_>) -> TokenStream {
        let name: TokenStream = format!("{}_Impl", self.def.name()).into();
        let namespace = config.write_namespace(self.def.type_name());

        if self.generics.is_empty() {
            quote! { #namespace #name }
        } else {
            let generics = self.generics.iter().map(|ty| ty.write_name(config));
            quote! { #namespace #name < #(#generics),* > }
        }
    }

    pub fn is_exclusive(&self) -> bool {
        self.def.has_attribute("ExclusiveToAttribute")
    }

    pub fn runtime_signature(&self) -> String {
        interface_signature(self.def, &self.generics)
    }

    pub fn required_interfaces(&self) -> Vec<Self> {
        fn walk(interface: &Interface, set: &mut Vec<Interface>) {
            for ty in interface
                .def
                .interface_impls()
                .map(|imp| imp.ty(&interface.generics))
            {
                let Type::Interface(interface) = ty else {
                    panic!();
                };

                if !set.iter().any(|existing| existing.def == interface.def) {
                    walk(&interface, set);
                    set.push(interface);
                }
            }
        }
        let mut set = vec![];
        walk(self, &mut set);

        set.sort();
        set.dedup();
        set
    }
}

impl Dependencies for Interface {
    fn combine(&self, dependencies: &mut TypeMap) {
        Type::Object.combine(dependencies);

        for interface in self.required_interfaces() {
            Type::Interface(interface).combine(dependencies);
        }

        // Different specializations of Interface may have different generics...
        for ty in &self.generics {
            ty.combine(dependencies);
        }

        let is_iterable = self.type_name() == TypeName::IIterable;

        for method in self.def.methods() {
            for ty in method
                .signature(self.def.namespace(), &self.generics)
                .types()
            {
                if is_iterable || ty.is_core() {
                    ty.combine(dependencies);
                }
            }
        }
    }
}
