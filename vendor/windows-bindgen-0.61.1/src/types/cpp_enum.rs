use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CppEnum {
    pub def: TypeDef,
}

impl Ord for CppEnum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.def.name().cmp(other.def.name())
    }
}

impl PartialOrd for CppEnum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CppEnum {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        let tn = self.def.type_name();
        let is_scoped = self.def.has_attribute("ScopedEnumAttribute");

        if !is_scoped && config.sys {
            return config.write_cpp_handle(self.def);
        }

        let name = to_ident(tn.name());
        let underlying_type = self.def.underlying_type().write_name(config);

        let mut derive = DeriveWriter::new(config, tn);
        derive.extend(["Copy", "Clone"]);

        if !config.sys {
            derive.extend(["Default", "Debug", "PartialEq", "Eq"]);
        }

        let fields = if is_scoped {
            let fields = self
                .def
                .fields()
                .filter(|field| field.flags().contains(FieldAttributes::Literal))
                .map(|field| {
                    let name = to_ident(field.name());
                    let value = field.constant().unwrap().value().write();

                    quote! {
                        pub const #name: Self = Self(#value);
                    }
                });

            quote! {
                impl #name {
                    #(#fields)*
                }
            }
        } else {
            quote! {}
        };

        let flags = if config.sys || !self.def.has_attribute("FlagsAttribute") {
            quote! {}
        } else {
            quote! {
                impl #name {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for #name {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for #name {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for #name {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for #name {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for #name {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }

            }
        };

        let must_use = if matches!(tn, TypeName::WIN32_ERROR | TypeName::RPC_STATUS) {
            quote! { #[must_use] }
        } else {
            quote! {}
        };

        quote! {
            #must_use
            #[repr(transparent)]
            #derive
            pub struct #name(pub #underlying_type);
            #fields
            #flags
        }
    }

    pub fn size(&self) -> usize {
        self.def.underlying_type().size()
    }

    pub fn align(&self) -> usize {
        self.def.underlying_type().align()
    }
}

impl Dependencies for CppEnum {
    fn combine(&self, dependencies: &mut TypeMap) {
        if let Some(attribute) = self.def.find_attribute("AlsoUsableForAttribute") {
            if let Some((_, Value::Str(type_name))) = attribute.args().first() {
                self.def
                    .reader()
                    .unwrap_full_name(self.def.namespace(), type_name)
                    .combine(dependencies);
            }
        }
    }
}
