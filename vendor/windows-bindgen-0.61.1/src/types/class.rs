use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Class {
    pub def: TypeDef,
}

impl Class {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
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
        let required_interfaces = self.required_interfaces();
        let type_name = self.def.type_name();
        let name = to_ident(type_name.name());
        let (class_cfg, cfg) = self.write_cfg(config);
        let runtime_name = format!("{type_name}");

        let runtime_name = quote! {
            #cfg
            impl windows_core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
        };

        let mut methods = quote! {};
        let mut method_names = MethodNames::new();

        for interface in &required_interfaces {
            let mut virtual_names = MethodNames::new();

            for method in interface
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
                    &mut method_names,
                    &mut virtual_names,
                );

                methods.combine(quote! {
                    #cfg
                    #method
                });
            }
        }

        let new = self.has_default_constructor().then(||
            quote! {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<#name, windows_core::imp::IGenericFactory> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
        );

        let factories = required_interfaces.iter().filter_map(|interface| match interface.kind {
            InterfaceKind::Static | InterfaceKind::Composable => {
                if interface.def.methods().next().is_none() {
                    None
                } else {
                        let method_name = to_ident(interface.def.name());
                        let interface_type = interface.write_name(config);

                        let cfg = if config.package {
                            class_cfg.difference(interface.def, &interface.dependencies(), config).write(config, false)
                        } else {
                            quote! {}
                        };

                        Some(quote! {
                            #cfg
                            fn #method_name<R, F: FnOnce(&#interface_type) -> windows_core::Result<R>>(
                                callback: F,
                            ) -> windows_core::Result<R> {
                                static SHARED: windows_core::imp::FactoryCache<#name, #interface_type> =
                                    windows_core::imp::FactoryCache::new();
                                SHARED.call(callback)
                            }
                        })
                    }
                }
                _ => None,
            });

        if let Some(default_interface) = self.default_interface() {
            if default_interface.is_async() {
                let default_interface = default_interface.write_name(config);

                return quote! {
                    #cfg
                    pub type #name = #default_interface;
                };
            }

            let is_exclusive = default_interface.is_exclusive();
            let default_interface = default_interface.write_name(config);

            let interface_hierarchy = if is_exclusive {
                quote! { windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable); }
            } else {
                quote! { windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable, #default_interface); }
            };

            let required_hierarchy = {
                let mut interfaces: Vec<_> = required_interfaces
                    .iter()
                    .filter(|ty| !ty.is_exclusive() && ty.kind != InterfaceKind::Default)
                    .map(|ty| ty.write_name(config))
                    .collect();

                interfaces.extend(self.bases().iter().map(|ty| ty.write_name(config)));

                if interfaces.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        #cfg
                        windows_core::imp::required_hierarchy!(#name, #(#interfaces),*);
                    }
                }
            };

            let agile = self.def.is_agile().then(|| {
                quote! {
                    #cfg
                    unsafe impl Send for #name {}
                    #cfg
                    unsafe impl Sync for #name {}
                }
            });

            let into_iterator = required_interfaces
                .iter()
                .find(|interface| interface.def.type_name() == TypeName::IIterable)
                .map(|interface| {
                    let ty = interface.generics[0].write_name(config);
                    let namespace = config.write_namespace(TypeName::IIterator);

                    quote! {
                        #cfg
                        impl IntoIterator for #name {
                            type Item = #ty;
                            type IntoIter = #namespace IIterator<Self::Item>;

                            fn into_iter(self) -> Self::IntoIter {
                                IntoIterator::into_iter(&self)
                            }
                        }
                        #cfg
                        impl IntoIterator for &#name {
                            type Item = #ty;
                            type IntoIter = #namespace IIterator<Self::Item>;

                            fn into_iter(self) -> Self::IntoIter {
                                self.First().unwrap()
                            }
                        }

                    }
                });

            quote! {
                #cfg
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #name(windows_core::IUnknown);
                #cfg
                #interface_hierarchy
                #required_hierarchy
                #cfg
                impl #name {
                    #new
                    #methods
                    #(#factories)*
                }
                #cfg
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, #default_interface>();
                }
                #cfg
                unsafe impl windows_core::Interface for #name {
                    type Vtable = <#default_interface as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <#default_interface as windows_core::Interface>::IID;
                }
                #runtime_name
                #agile
                #into_iterator
            }
        } else {
            quote! {
                #cfg
                pub struct #name;
                #cfg
                impl #name {
                    #methods
                    #(#factories)*
                }
                #runtime_name
            }
        }
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &[])
    }

    fn default_interface(&self) -> Option<Type> {
        self.def
            .interface_impls()
            .find(|imp| imp.has_attribute("DefaultAttribute"))
            .map(|imp| imp.ty(&[]))
    }

    pub fn runtime_signature(&self) -> String {
        format!(
            "rc({};{})",
            self.type_name(),
            self.default_interface().unwrap().runtime_signature()
        )
    }

    fn bases(&self) -> Vec<Self> {
        let mut bases = Vec::new();
        let mut def = self.def;
        let reader = def.reader();

        loop {
            let extends = def.extends().unwrap();

            if extends == TypeName::Object {
                break;
            }

            let Type::Class(base) = reader.unwrap_full_name(extends.namespace(), extends.name())
            else {
                panic!("type not found: {extends}");
            };

            def = base.def;
            bases.push(base);
        }

        bases
    }

    pub fn required_interfaces(&self) -> Vec<Interface> {
        fn walk(def: TypeDef, generics: &[Type], is_base: bool, set: &mut Vec<Interface>) {
            for imp in def.interface_impls() {
                let Type::Interface(mut interface) = imp.ty(generics) else {
                    panic!();
                };

                interface.kind = if !is_base && imp.has_attribute("DefaultAttribute") {
                    InterfaceKind::Default
                } else if is_base {
                    InterfaceKind::Base
                } else {
                    InterfaceKind::None
                };

                if let Some(pos) = set
                    .iter()
                    .position(|existing| existing.def == interface.def)
                {
                    if interface.kind == InterfaceKind::Default {
                        set[pos].kind = interface.kind;
                    }
                } else {
                    walk(interface.def, &interface.generics, is_base, set);
                    set.push(interface);
                }
            }
        }
        let mut set = vec![];
        walk(self.def, &[], false, &mut set);

        for base in self.bases() {
            walk(base.def, &[], true, &mut set);
        }

        for attribute in self.def.attributes() {
            let kind = match attribute.name() {
                "StaticAttribute" | "ActivatableAttribute" => InterfaceKind::Static,
                "ComposableAttribute" => InterfaceKind::Composable,
                _ => continue,
            };

            for (_, arg) in attribute.args() {
                if let Value::TypeName(tn) = arg {
                    let Type::Interface(mut interface) = self
                        .def
                        .reader()
                        .unwrap_full_name(tn.namespace(), tn.name())
                    else {
                        panic!("type not found: {tn}");
                    };

                    interface.kind = kind;
                    set.push(interface);
                    break;
                }
            }
        }

        set.sort();
        set.dedup();
        set
    }

    fn has_default_constructor(&self) -> bool {
        self.def
            .attributes()
            .filter(|attribute| attribute.name() == "ActivatableAttribute")
            .any(|attribute| {
                !attribute
                    .args()
                    .iter()
                    .any(|arg| matches!(arg.1, Value::TypeName(_)))
            })
    }
}

impl Dependencies for Class {
    fn combine(&self, dependencies: &mut TypeMap) {
        for interface in self.required_interfaces() {
            Type::Interface(interface).combine(dependencies);
        }
    }
}
