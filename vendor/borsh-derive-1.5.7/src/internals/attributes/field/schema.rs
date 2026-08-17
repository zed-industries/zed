use std::collections::BTreeMap;

use crate::internals::attributes::{
    parsing::{meta_get_by_symbol_keys, parse_lit_into_vec},
    schema_keys::{DECLARATION, DEFINITIONS, PARAMS, WITH_FUNCS},
    Symbol,
};
use once_cell::sync::Lazy;
use quote::ToTokens;
use syn::{
    meta::ParseNestedMeta,
    parse::{Parse, ParseStream},
    Ident, Token, Type,
};

use self::with_funcs::{WithFuncs, WITH_FUNCS_FIELD_PARSE_MAP};

pub mod with_funcs;

pub enum Variants {
    Params(Vec<ParameterOverride>),
    WithFuncs(WithFuncs),
}

type ParseFn = dyn Fn(Symbol, Symbol, &ParseNestedMeta) -> syn::Result<Variants> + Send + Sync;

pub static SCHEMA_FIELD_PARSE_MAP: Lazy<BTreeMap<Symbol, Box<ParseFn>>> = Lazy::new(|| {
    let mut m = BTreeMap::new();
    // assigning closure `let f = |args| {...};` and boxing closure `let f: Box<ParseFn> = Box::new(f);`
    // on 2 separate lines doesn't work
    let f_params: Box<ParseFn> = Box::new(|attr_name, meta_item_name, meta| {
        parse_lit_into_vec::<ParameterOverride>(attr_name, meta_item_name, meta)
            .map(Variants::Params)
    });

    let f_with_funcs: Box<ParseFn> = Box::new(|_attr_name, _meta_item_name, meta| {
        let map_result = meta_get_by_symbol_keys(WITH_FUNCS, meta, &WITH_FUNCS_FIELD_PARSE_MAP)?;
        let with_funcs: WithFuncs = map_result.into();
        if (with_funcs.declaration.is_some() && with_funcs.definitions.is_none())
            || (with_funcs.declaration.is_none() && with_funcs.definitions.is_some())
        {
            return Err(syn::Error::new_spanned(
                &meta.path,
                format!(
                    "both `{}` and `{}` have to be specified at the same time",
                    DECLARATION.1, DEFINITIONS.1,
                ),
            ));
        }
        Ok(Variants::WithFuncs(with_funcs))
    });
    m.insert(PARAMS, f_params);
    m.insert(WITH_FUNCS, f_with_funcs);
    m
});

/**
Struct describes an entry like `order_param => override_type`,  e.g. `K => <K as TraitName>::Associated`
*/
#[derive(Clone)]
pub struct ParameterOverride {
    pub order_param: Ident,
    arrow_token: Token![=>],
    pub override_type: Type,
}

impl Parse for ParameterOverride {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(ParameterOverride {
            order_param: input.parse()?,
            arrow_token: input.parse()?,
            override_type: input.parse()?,
        })
    }
}

impl ToTokens for ParameterOverride {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.order_param.to_tokens(tokens);
        self.arrow_token.to_tokens(tokens);
        self.override_type.to_tokens(tokens);
    }
}

#[allow(unused)]
#[derive(Default, Clone)]
pub(crate) struct Attributes {
    pub params: Option<Vec<ParameterOverride>>,
    pub with_funcs: Option<WithFuncs>,
}

impl From<BTreeMap<Symbol, Variants>> for Attributes {
    fn from(mut map: BTreeMap<Symbol, Variants>) -> Self {
        let params = map.remove(&PARAMS);
        let params = params.map(|variant| match variant {
            Variants::Params(params) => params,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });

        let with_funcs = map.remove(&WITH_FUNCS);
        let with_funcs = with_funcs.map(|variant| match variant {
            Variants::WithFuncs(with_funcs) => with_funcs,
            _ => unreachable!("only one enum variant is expected to correspond to given map key"),
        });
        Self { params, with_funcs }
    }
}
