use std::collections::BTreeMap;

use syn::{meta::ParseNestedMeta, WherePredicate};

use crate::internals::attributes::{parsing::parse_lit_into_vec, Symbol, DESERIALIZE, SERIALIZE};
use once_cell::sync::Lazy;

pub enum Variants {
    Serialize(Vec<WherePredicate>),
    Deserialize(Vec<WherePredicate>),
}

type ParseFn = dyn Fn(Symbol, Symbol, &ParseNestedMeta) -> syn::Result<Variants> + Send + Sync;

pub static BOUNDS_FIELD_PARSE_MAP: Lazy<BTreeMap<Symbol, Box<ParseFn>>> = Lazy::new(|| {
    let mut m = BTreeMap::new();
    // assigning closure `let f = |args| {...};` and boxing closure `let f: Box<ParseFn> = Box::new(f);`
    // on 2 separate lines doesn't work
    let f_serialize: Box<ParseFn> = Box::new(|attr_name, meta_item_name, meta| {
        parse_lit_into_vec::<WherePredicate>(attr_name, meta_item_name, meta)
            .map(Variants::Serialize)
    });
    let f_deserialize: Box<ParseFn> = Box::new(|attr_name, meta_item_name, meta| {
        parse_lit_into_vec::<WherePredicate>(attr_name, meta_item_name, meta)
            .map(Variants::Deserialize)
    });
    m.insert(SERIALIZE, f_serialize);
    m.insert(DESERIALIZE, f_deserialize);
    m
});

#[derive(Default, Clone)]
pub struct Bounds {
    pub serialize: Option<Vec<WherePredicate>>,
    pub deserialize: Option<Vec<WherePredicate>>,
}

impl From<BTreeMap<Symbol, Variants>> for Bounds {
    fn from(mut map: BTreeMap<Symbol, Variants>) -> Self {
        let serialize = map.remove(&SERIALIZE);
        let deserialize = map.remove(&DESERIALIZE);
        let serialize = serialize.map(|variant| match variant {
            Variants::Serialize(ser) => ser,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });
        let deserialize = deserialize.map(|variant| match variant {
            Variants::Deserialize(de) => de,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });
        Self {
            serialize,
            deserialize,
        }
    }
}
