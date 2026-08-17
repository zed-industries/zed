use syn::{Attribute, Path};

pub mod field;
pub mod item;
pub mod parsing;

/// first field is attr name
/// second field is its expected value format representation for error printing
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Symbol(pub &'static str, pub &'static str);

/// borsh - top level prefix in nested meta attribute
pub const BORSH: Symbol = Symbol("borsh", "borsh(...)");
/// bound - sub-borsh nested meta, field-level only, `BorshSerialize` and `BorshDeserialize` contexts
pub const BOUND: Symbol = Symbol("bound", "bound(...)");
//  use_discriminant - sub-borsh nested meta, item-level only, enums only, `BorshSerialize` and `BorshDeserialize` contexts
pub const USE_DISCRIMINANT: Symbol = Symbol("use_discriminant", "use_discriminant = ...");
/// serialize - sub-bound nested meta attribute
pub const SERIALIZE: Symbol = Symbol("serialize", "serialize = ...");
/// deserialize - sub-bound nested meta attribute
pub const DESERIALIZE: Symbol = Symbol("deserialize", "deserialize = ...");
/// skip - sub-borsh nested meta, field-level only attribute, `BorshSerialize`, `BorshDeserialize`, `BorshSchema` contexts
pub const SKIP: Symbol = Symbol("skip", "skip");
/// init - sub-borsh nested meta, item-level only attribute  `BorshDeserialize` context
pub const INIT: Symbol = Symbol("init", "init = ...");
/// serialize_with - sub-borsh nested meta, field-level only, `BorshSerialize` context
pub const SERIALIZE_WITH: Symbol = Symbol("serialize_with", "serialize_with = ...");
/// deserialize_with - sub-borsh nested meta, field-level only, `BorshDeserialize` context
pub const DESERIALIZE_WITH: Symbol = Symbol("deserialize_with", "deserialize_with = ...");
/// crate - sub-borsh nested meta, item-level only, `BorshSerialize`, `BorshDeserialize`, `BorshSchema` contexts
pub const CRATE: Symbol = Symbol("crate", "crate = ...");

#[cfg(feature = "schema")]
pub mod schema_keys {
    use super::Symbol;

    /// schema - sub-borsh nested meta, `BorshSchema` context
    pub const SCHEMA: Symbol = Symbol("schema", "schema(...)");
    /// params - sub-schema nested meta, field-level only attribute
    pub const PARAMS: Symbol = Symbol("params", "params = ...");
    /// serialize_with - sub-borsh nested meta, field-level only, `BorshSerialize` context
    /// with_funcs - sub-schema nested meta, field-level only attribute
    pub const WITH_FUNCS: Symbol = Symbol("with_funcs", "with_funcs(...)");
    /// declaration - sub-with_funcs nested meta, field-level only attribute
    pub const DECLARATION: Symbol = Symbol("declaration", "declaration = ...");
    /// definitions - sub-with_funcs nested meta, field-level only attribute
    pub const DEFINITIONS: Symbol = Symbol("definitions", "definitions = ...");
}

#[derive(Clone, Copy)]
pub enum BoundType {
    Serialize,
    Deserialize,
}
impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

fn get_one_attribute(attrs: &[Attribute]) -> syn::Result<Option<&Attribute>> {
    let count = attrs.iter().filter(|attr| attr.path() == BORSH).count();
    let borsh = attrs.iter().find(|attr| attr.path() == BORSH);
    if count > 1 {
        return Err(syn::Error::new_spanned(
            borsh.unwrap(),
            format!("multiple `{}` attributes not allowed", BORSH.0),
        ));
    }
    Ok(borsh)
}
