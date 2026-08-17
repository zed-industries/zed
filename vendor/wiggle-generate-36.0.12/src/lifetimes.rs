use proc_macro2::TokenStream;
use quote::quote;

pub trait LifetimeExt {
    fn needs_lifetime(&self) -> bool;
}

impl LifetimeExt for witx::TypeRef {
    fn needs_lifetime(&self) -> bool {
        self.type_().needs_lifetime()
    }
}

impl LifetimeExt for witx::Type {
    fn needs_lifetime(&self) -> bool {
        match self {
            witx::Type::Builtin(b) => b.needs_lifetime(),
            witx::Type::Record(s) => s.needs_lifetime(),
            witx::Type::Variant(u) => u.needs_lifetime(),
            witx::Type::Handle { .. } => false,
            witx::Type::Pointer { .. }
            | witx::Type::ConstPointer { .. }
            | witx::Type::List { .. } => true,
        }
    }
}

impl LifetimeExt for witx::BuiltinType {
    fn needs_lifetime(&self) -> bool {
        false
    }
}

impl LifetimeExt for witx::RecordDatatype {
    fn needs_lifetime(&self) -> bool {
        self.members.iter().any(|m| m.tref.needs_lifetime())
    }
}

impl LifetimeExt for witx::Variant {
    fn needs_lifetime(&self) -> bool {
        self.cases
            .iter()
            .any(|m| m.tref.as_ref().map(|t| t.needs_lifetime()).unwrap_or(false))
    }
}

pub fn anon_lifetime() -> TokenStream {
    quote!('_)
}
