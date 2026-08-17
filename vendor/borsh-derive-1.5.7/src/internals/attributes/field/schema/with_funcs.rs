use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use syn::meta::ParseNestedMeta;

use crate::internals::attributes::{
    parsing::parse_lit_into,
    schema_keys::{DECLARATION, DEFINITIONS},
    Symbol,
};

pub enum Variants {
    Declaration(syn::ExprPath),
    Definitions(syn::ExprPath),
}

type ParseFn = dyn Fn(Symbol, Symbol, &ParseNestedMeta) -> syn::Result<Variants> + Send + Sync;

pub static WITH_FUNCS_FIELD_PARSE_MAP: Lazy<BTreeMap<Symbol, Box<ParseFn>>> = Lazy::new(|| {
    let mut m = BTreeMap::new();
    // assigning closure `let f = |args| {...};` and boxing closure `let f: Box<ParseFn> = Box::new(f);`
    // on 2 separate lines doesn't work
    let f_declaration: Box<ParseFn> = Box::new(|attr_name, meta_item_name, meta| {
        parse_lit_into::<syn::ExprPath>(attr_name, meta_item_name, meta).map(Variants::Declaration)
    });

    let f_definitions: Box<ParseFn> = Box::new(|attr_name, meta_item_name, meta| {
        parse_lit_into::<syn::ExprPath>(attr_name, meta_item_name, meta).map(Variants::Definitions)
    });

    m.insert(DECLARATION, f_declaration);
    m.insert(DEFINITIONS, f_definitions);
    m
});

#[derive(Clone)]
pub struct WithFuncs {
    pub declaration: Option<syn::ExprPath>,
    pub definitions: Option<syn::ExprPath>,
}

impl From<BTreeMap<Symbol, Variants>> for WithFuncs {
    fn from(mut map: BTreeMap<Symbol, Variants>) -> Self {
        let declaration = map.remove(&DECLARATION);
        let definitions = map.remove(&DEFINITIONS);
        let declaration = declaration.map(|variant| match variant {
            Variants::Declaration(declaration) => declaration,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });
        let definitions = definitions.map(|variant| match variant {
            Variants::Definitions(definitions) => definitions,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });
        Self {
            declaration,
            definitions,
        }
    }
}
