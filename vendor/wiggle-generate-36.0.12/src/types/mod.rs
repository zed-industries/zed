// mod r#enum;
mod error;
mod flags;
mod handle;
mod record;
mod variant;

use crate::codegen_settings::ErrorType;
use crate::lifetimes::LifetimeExt;
use crate::names;

use proc_macro2::TokenStream;
use quote::quote;

pub fn define_datatype(namedtype: &witx::NamedType, error: Option<&ErrorType>) -> TokenStream {
    match &namedtype.tref {
        witx::TypeRef::Name(alias_to) => define_alias(&namedtype.name, &alias_to),
        witx::TypeRef::Value(v) => match &**v {
            witx::Type::Record(r) => match r.bitflags_repr() {
                Some(repr) => flags::define_flags(&namedtype.name, repr, &r),
                None => record::define_struct(&namedtype.name, &r),
            },
            witx::Type::Variant(v) => match error {
                Some(ErrorType::Generated(error)) => {
                    let d = variant::define_variant(&namedtype.name, &v, true);
                    let e = error::define_error(&namedtype.name, &v, error);
                    quote!( #d #e )
                }
                _ => variant::define_variant(&namedtype.name, &v, false),
            },
            witx::Type::Handle(h) => handle::define_handle(&namedtype.name, &h),
            witx::Type::Builtin(b) => define_builtin(&namedtype.name, *b),
            witx::Type::Pointer(p) => {
                define_witx_pointer(&namedtype.name, quote!(wiggle::GuestPtr), p)
            }
            witx::Type::ConstPointer(p) => {
                define_witx_pointer(&namedtype.name, quote!(wiggle::GuestPtr), p)
            }
            witx::Type::List(arr) => define_witx_list(&namedtype.name, &arr),
        },
    }
}

fn define_alias(name: &witx::Id, to: &witx::NamedType) -> TokenStream {
    let ident = names::type_(name);
    let rhs = names::type_(&to.name);
    if to.tref.needs_lifetime() {
        quote!(pub type #ident<'a> = #rhs<'a>;)
    } else {
        quote!(pub type #ident = #rhs;)
    }
}

fn define_builtin(name: &witx::Id, builtin: witx::BuiltinType) -> TokenStream {
    let ident = names::type_(name);
    let built = names::builtin_type(builtin);
    quote!(pub type #ident = #built;)
}

fn define_witx_pointer(
    name: &witx::Id,
    pointer_type: TokenStream,
    pointee: &witx::TypeRef,
) -> TokenStream {
    let ident = names::type_(name);
    let pointee_type = names::type_ref(pointee, quote!('a));

    quote!(pub type #ident<'a> = #pointer_type<'a, #pointee_type>;)
}

fn define_witx_list(name: &witx::Id, arr_raw: &witx::TypeRef) -> TokenStream {
    let ident = names::type_(name);
    let pointee_type = names::type_ref(arr_raw, quote!());
    quote!(pub type #ident = wiggle::GuestPtr<[#pointee_type]>;)
}

pub fn int_repr_tokens(int_repr: witx::IntRepr) -> TokenStream {
    match int_repr {
        witx::IntRepr::U8 => quote!(u8),
        witx::IntRepr::U16 => quote!(u16),
        witx::IntRepr::U32 => quote!(u32),
        witx::IntRepr::U64 => quote!(u64),
    }
}

pub trait WiggleType {
    fn impls_display(&self) -> bool;
}

impl WiggleType for witx::TypeRef {
    fn impls_display(&self) -> bool {
        match self {
            witx::TypeRef::Name(alias_to) => (&*alias_to).impls_display(),
            witx::TypeRef::Value(v) => (&*v).impls_display(),
        }
    }
}

impl WiggleType for witx::NamedType {
    fn impls_display(&self) -> bool {
        self.tref.impls_display()
    }
}

impl WiggleType for witx::Type {
    fn impls_display(&self) -> bool {
        match self {
            witx::Type::Record(x) => x.impls_display(),
            witx::Type::Variant(x) => x.impls_display(),
            witx::Type::Handle(x) => x.impls_display(),
            witx::Type::Builtin(x) => x.impls_display(),
            witx::Type::Pointer { .. }
            | witx::Type::ConstPointer { .. }
            | witx::Type::List { .. } => false,
        }
    }
}

impl WiggleType for witx::BuiltinType {
    fn impls_display(&self) -> bool {
        true
    }
}

impl WiggleType for witx::InterfaceFuncParam {
    fn impls_display(&self) -> bool {
        self.tref.impls_display()
    }
}
