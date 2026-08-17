use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppDelegate {
    pub def: TypeDef,
}

impl Ord for CppDelegate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.def.name(), self.def).cmp(&(other.def.name(), other.def))
    }
}

impl PartialOrd for CppDelegate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppDelegate {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn method(&self) -> MethodDef {
        self.def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap()
    }

    pub fn write_cfg(&self, config: &Config<'_>) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        Cfg::new(self.def, &self.dependencies(), config).write(config, false)
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        let type_name = self.def.type_name();
        let name = to_ident(type_name.name());
        let method = self.method();
        let signature = method.signature(type_name.namespace(), &[]);

        let mut params = quote! {};

        for param in &signature.params {
            params.combine(write_param(config, param));
        }

        let return_sig = config.write_return_sig(method, &signature, false);
        let arches = write_arches(self.def);
        let cfg = self.write_cfg(config);

        quote! {
            #arches
            #cfg
            pub type #name = Option<unsafe extern "system" fn(#params) #return_sig>;
        }
    }
}

impl Dependencies for CppDelegate {
    fn combine(&self, dependencies: &mut TypeMap) {
        self.method()
            .signature(self.def.namespace(), &[])
            .combine(dependencies);
    }
}

fn write_param(config: &Config<'_>, param: &Param) -> TokenStream {
    let name = param.write_ident();
    let type_name = param.write_name(config);

    if config.sys {
        return quote! { #name: #type_name, };
    }

    if param.is_input() {
        if param.is_copyable() {
            return quote! { #name: #type_name, };
        } else {
            return quote! { #name: windows_core::Ref<'_, #type_name>, };
        }
    }

    let deref = param.deref();

    if deref.is_interface() {
        let type_name = deref.write_name(config);
        quote! { #name: windows_core::OutRef<'_, #type_name>, }
    } else {
        quote! { #name: #type_name, }
    }
}
