use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppConst {
    pub namespace: &'static str,
    pub field: Field,
}

impl Ord for CppConst {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.field.name(), self).cmp(&(other.field.name(), other))
    }
}

impl PartialOrd for CppConst {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppConst {
    pub fn type_name(&self) -> TypeName {
        TypeName(self.namespace, self.field.name())
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write_cfg(&self, config: &Config<'_>) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        Cfg::new(self.field, &self.dependencies(), config).write(config, false)
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        let name = to_ident(self.field.name());

        if let Some(guid) = self.field.guid_attribute() {
            return config.write_cpp_const_guid(name, &guid);
        }

        let field_ty = self.field.ty(None).to_const_type();
        let cfg = self.write_cfg(config);

        if let Some(constant) = self.field.constant() {
            let constant_ty = constant.ty();

            if field_ty == constant_ty {
                if field_ty == Type::String {
                    let crate_name = config.write_core();
                    let value = constant.value().write();

                    // TODO: if config.no_core then write these literals out as byte strings?
                    if is_ansi_encoding(self.field) {
                        quote! {
                            #cfg
                            pub const #name: #crate_name PCSTR = #crate_name s!(#value);
                        }
                    } else {
                        quote! {
                            #cfg
                            pub const #name: #crate_name PCWSTR = #crate_name w!(#value);
                        }
                    }
                } else {
                    let ty = field_ty.write_name(config);
                    let value = constant.value().write();

                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                }
            } else {
                let underlying_ty = field_ty.underlying_type();
                let ty = field_ty.write_name(config);
                let mut value = constant.value().write();

                if underlying_ty == constant_ty {
                    if is_signed_error(&field_ty) {
                        if let Value::I32(signed) = constant.value() {
                            value = format!("0x{signed:X}_u32 as _").into();
                        }
                    }
                } else if field_ty == Type::Bool {
                    value = match constant.value() {
                        Value::U8(1) => quote! { true },
                        Value::U8(0) => quote! { false },
                        _ => panic!(),
                    };
                } else {
                    value = quote! { #value as _ };
                }

                if config.sys || field_ty == Type::Bool {
                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                } else {
                    quote! {
                        #cfg
                        pub const #name: #ty = #ty(#value);
                    }
                }
            }
        } else if let Some(attribute) = self.field.find_attribute("ConstantAttribute") {
            let args = attribute.args();
            let Some(&(_, Value::Str(mut input))) = args.first() else {
                panic!()
            };

            let Type::CppStruct(ty) = &field_ty else {
                panic!()
            };

            let mut tokens = quote! {};

            for field in ty.def.fields() {
                let (value, rest) = config.field_initializer(field, input);
                input = rest;
                tokens.combine(value);
            }

            let ty = field_ty.write_name(config);

            quote! {
                #cfg
                pub const #name: #ty = #ty { #tokens };
            }
        } else {
            panic!()
        }
    }
}

impl Dependencies for CppConst {
    fn combine(&self, dependencies: &mut TypeMap) {
        self.field.ty(None).to_const_type().combine(dependencies);
    }
}

fn is_ansi_encoding(row: Field) -> bool {
    row.find_attribute("NativeEncodingAttribute").is_some_and(|attribute| matches!(attribute.args().first(), Some((_, Value::Str(encoding))) if *encoding == "ansi"))
}

fn is_signed_error(ty: &Type) -> bool {
    match ty {
        Type::HRESULT => true,
        Type::CppStruct(ty) => ty.type_name() == TypeName::NTSTATUS,
        _ => false,
    }
}
