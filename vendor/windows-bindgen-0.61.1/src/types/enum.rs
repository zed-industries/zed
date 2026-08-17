use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Enum {
    pub def: TypeDef,
}

impl Enum {
    pub fn type_name(&self) -> TypeName {
        self.def.type_name()
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        self.type_name().write(config, &[])
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        let name = to_ident(self.def.name());
        let underlying_type = self.def.underlying_type();

        let mut derive = DeriveWriter::new(config, self.type_name());
        derive.extend(["Copy", "Clone"]);

        if !config.sys {
            derive.extend(["Default", "Debug", "PartialEq", "Eq"]);
        }

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

        let flags = if config.sys || underlying_type != Type::U32 {
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

        let underlying_type = underlying_type.write_name(config);

        let win_traits = if config.sys {
            quote! {}
        } else {
            let signature = Literal::byte_string(&self.runtime_signature());

            quote! {
                impl windows_core::TypeKind for #name {
                    type TypeKind = windows_core::CopyType;
                }
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#signature);
                }
            }
        };

        quote! {
            #[repr(transparent)]
            #derive
            pub struct #name(pub #underlying_type);
            impl #name {
                #(#fields)*
            }
            #win_traits
            #flags
        }
    }

    pub fn runtime_signature(&self) -> String {
        format!(
            "enum({};{})",
            self.def.type_name(),
            self.def.underlying_type().runtime_signature()
        )
    }
}
