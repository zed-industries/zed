use super::*;

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(&'static str),
    String(String),
    TypeName(TypeName),
}

impl Value {
    pub fn write(&self) -> TokenStream {
        match self {
            Self::Bool(value) => quote! { #value },
            Self::U8(value) => quote! { #value },
            Self::I8(value) => quote! { #value },
            Self::U16(value) => quote! { #value },
            Self::I16(value) => quote! { #value },
            Self::U32(value) => quote! { #value },
            Self::I32(value) => quote! { #value },
            Self::U64(value) => quote! { #value },
            Self::I64(value) => quote! { #value },
            Self::F32(value) => quote! { #value },
            Self::F64(value) => quote! { #value },
            Self::String(value) => {
                let mut tokens = "\"".to_string();

                for u in value.chars() {
                    write!(tokens, "{}", u.escape_default()).unwrap();
                }

                tokens.push('\"');
                tokens.into()
            }
            rest => panic!("{rest:?}"),
        }
    }
}
