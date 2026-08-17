//! A set of macros to generate Rust source for PHF data structures at compile time.
//! See [the `phf` crate's documentation][phf] for details.
//!
//! [phf]: https://docs.rs/phf

use phf_generator::HashState;
use phf_shared::PhfHash;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::hash::Hasher;
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, BinOp, Error, Expr, ExprLit, Lit, Token, UnOp};
#[cfg(feature = "uncased")]
use uncased_::Uncased;
#[cfg(feature = "unicase")]
use unicase_::{Ascii, UniCase};

#[derive(Hash, PartialEq, Eq, Clone)]
enum ParsedKey {
    Str(String),
    Binary(Vec<u8>),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Bool(bool),
    Tuple(Vec<ParsedKey>),
    #[cfg(feature = "unicase")]
    UniCase(UniCase<String>),
    #[cfg(feature = "unicase")]
    UniCaseAscii(Ascii<String>),
    #[cfg(feature = "uncased")]
    Uncased(Uncased<'static>),
}

impl PhfHash for ParsedKey {
    fn phf_hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            ParsedKey::Str(s) => s.phf_hash(state),
            ParsedKey::Binary(s) => s.phf_hash(state),
            ParsedKey::Char(s) => s.phf_hash(state),
            ParsedKey::I8(s) => s.phf_hash(state),
            ParsedKey::I16(s) => s.phf_hash(state),
            ParsedKey::I32(s) => s.phf_hash(state),
            ParsedKey::I64(s) => s.phf_hash(state),
            ParsedKey::I128(s) => s.phf_hash(state),
            ParsedKey::Isize(s) => s.phf_hash(state),
            ParsedKey::U8(s) => s.phf_hash(state),
            ParsedKey::U16(s) => s.phf_hash(state),
            ParsedKey::U32(s) => s.phf_hash(state),
            ParsedKey::U64(s) => s.phf_hash(state),
            ParsedKey::U128(s) => s.phf_hash(state),
            ParsedKey::Usize(s) => s.phf_hash(state),
            ParsedKey::Bool(s) => s.phf_hash(state),
            ParsedKey::Tuple(elements) => {
                for element in elements {
                    element.phf_hash(state);
                }
            }
            #[cfg(feature = "unicase")]
            ParsedKey::UniCase(s) => s.phf_hash(state),
            #[cfg(feature = "unicase")]
            ParsedKey::UniCaseAscii(s) => s.phf_hash(state),
            #[cfg(feature = "uncased")]
            ParsedKey::Uncased(s) => s.phf_hash(state),
        }
    }
}

impl ParsedKey {
    fn from_expr(expr: &Expr) -> Option<ParsedKey> {
        match expr {
            Expr::Lit(lit) => match &lit.lit {
                Lit::Str(s) => Some(ParsedKey::Str(s.value())),
                Lit::ByteStr(s) => Some(ParsedKey::Binary(s.value())),
                Lit::Byte(s) => Some(ParsedKey::U8(s.value())),
                Lit::Char(s) => Some(ParsedKey::Char(s.value())),
                Lit::Int(s) => match s.suffix() {
                    // we've lost the sign at this point, so `-128i8` looks like `128i8`,
                    // which doesn't fit in an `i8`; parse it as a `u8` and cast (to `0i8`),
                    // which is handled below, by `Unary`
                    "i8" => Some(ParsedKey::I8(s.base10_parse::<u8>().unwrap() as i8)),
                    "i16" => Some(ParsedKey::I16(s.base10_parse::<u16>().unwrap() as i16)),
                    "i32" => Some(ParsedKey::I32(s.base10_parse::<u32>().unwrap() as i32)),
                    "i64" => Some(ParsedKey::I64(s.base10_parse::<u64>().unwrap() as i64)),
                    "i128" => Some(ParsedKey::I128(s.base10_parse::<u128>().unwrap() as i128)),
                    "isize" => Some(ParsedKey::Isize(s.base10_parse::<usize>().unwrap() as isize)),
                    "u8" => Some(ParsedKey::U8(s.base10_parse::<u8>().unwrap())),
                    "u16" => Some(ParsedKey::U16(s.base10_parse::<u16>().unwrap())),
                    "u32" => Some(ParsedKey::U32(s.base10_parse::<u32>().unwrap())),
                    "u64" => Some(ParsedKey::U64(s.base10_parse::<u64>().unwrap())),
                    "u128" => Some(ParsedKey::U128(s.base10_parse::<u128>().unwrap())),
                    "usize" => Some(ParsedKey::Usize(s.base10_parse::<usize>().unwrap())),
                    // Handle unsuffixed integer literals, default to i32
                    "" => {
                        if let Ok(val) = s.base10_parse::<i32>() {
                            Some(ParsedKey::I32(val))
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                Lit::Bool(s) => Some(ParsedKey::Bool(s.value)),
                _ => None,
            },
            Expr::Array(array) => {
                let mut buf = vec![];
                for expr in &array.elems {
                    match expr {
                        Expr::Lit(lit) => match &lit.lit {
                            Lit::Int(s) => match s.suffix() {
                                "u8" | "" => buf.push(s.base10_parse::<u8>().unwrap()),
                                _ => return None,
                            },
                            _ => return None,
                        },
                        _ => return None,
                    }
                }
                Some(ParsedKey::Binary(buf))
            }
            Expr::Unary(unary) => {
                // Handle negation for signed integer types
                // If we received an integer literal (always unsigned) greater than i__::max_value()
                // then casting it to a signed integer type of the same width will negate it to
                // the same absolute value so we don't need to negate it here
                macro_rules! try_negate {
                    ($val:expr) => {
                        if $val < 0 {
                            $val
                        } else {
                            -$val
                        }
                    };
                }

                match unary.op {
                    UnOp::Neg(_) => match ParsedKey::from_expr(&unary.expr)? {
                        ParsedKey::I8(v) => Some(ParsedKey::I8(try_negate!(v))),
                        ParsedKey::I16(v) => Some(ParsedKey::I16(try_negate!(v))),
                        ParsedKey::I32(v) => Some(ParsedKey::I32(try_negate!(v))),
                        ParsedKey::I64(v) => Some(ParsedKey::I64(try_negate!(v))),
                        ParsedKey::I128(v) => Some(ParsedKey::I128(try_negate!(v))),
                        ParsedKey::Isize(v) => Some(ParsedKey::Isize(try_negate!(v))),
                        _ => None,
                    },
                    UnOp::Deref(_) => {
                        let mut expr = &*unary.expr;
                        while let Expr::Group(group) = expr {
                            expr = &*group.expr;
                        }
                        match expr {
                            Expr::Lit(ExprLit {
                                lit: Lit::ByteStr(s),
                                ..
                            }) => Some(ParsedKey::Binary(s.value())),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            Expr::Tuple(tuple) => {
                let mut elements = Vec::new();
                for elem in &tuple.elems {
                    if let Some(parsed_elem) = ParsedKey::from_expr(elem) {
                        elements.push(parsed_elem);
                    } else {
                        return None;
                    }
                }
                Some(ParsedKey::Tuple(elements))
            }
            Expr::Group(group) => ParsedKey::from_expr(&group.expr),
            Expr::Call(call) if call.args.len() == 1 => {
                let last;
                let last_ahead;

                if let Expr::Path(ep) = call.func.as_ref() {
                    let mut segments = ep.path.segments.iter();
                    last = segments.next_back()?.ident.to_string();
                    last_ahead = segments.next_back()?.ident.to_string();
                } else {
                    return None;
                }

                let mut arg = call.args.first().unwrap();

                while let Expr::Group(group) = arg {
                    arg = &group.expr;
                }

                let _value = match arg {
                    Expr::Lit(ExprLit {
                        attrs: _,
                        lit: Lit::Str(s),
                    }) => s.value(),
                    _ => {
                        return None;
                    }
                };

                match (&*last_ahead, &*last) {
                    #[cfg(feature = "unicase")]
                    ("UniCase", "unicode") => Some(ParsedKey::UniCase(UniCase::unicode(_value))),
                    #[cfg(feature = "unicase")]
                    ("UniCase", "ascii") => Some(ParsedKey::UniCase(UniCase::ascii(_value))),
                    #[cfg(feature = "unicase")]
                    ("Ascii", "new") => Some(ParsedKey::UniCaseAscii(Ascii::new(_value))),
                    #[cfg(feature = "uncased")]
                    ("UncasedStr", "new") => Some(ParsedKey::Uncased(Uncased::new(_value))),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Key {
    parsed: Vec<ParsedKey>,
    expr: Vec<Expr>,
    attrs: Vec<syn::Attribute>,
}

impl PhfHash for Key {
    fn phf_hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        // For OR patterns, we hash the first key (they should all hash to the same value)
        if let Some(first) = self.parsed.first() {
            first.phf_hash(state);
        }
    }
}

impl Parse for Key {
    fn parse(input: ParseStream<'_>) -> parse::Result<Key> {
        let attrs = input.call(syn::Attribute::parse_outer)?;

        // Parse the expression (which might contain OR patterns)
        let expr = input.parse::<Expr>()?;

        // Extract all keys from the expression (handling OR patterns)
        let (exprs, parsed_keys) = extract_keys_from_expr(&expr)?;

        Ok(Key {
            parsed: parsed_keys,
            expr: exprs,
            attrs,
        })
    }
}

/// Extract all keys from an expression, handling OR patterns
fn extract_keys_from_expr(expr: &Expr) -> parse::Result<(Vec<Expr>, Vec<ParsedKey>)> {
    match expr {
        Expr::Binary(binary) => {
            if let BinOp::BitOr(_) = binary.op {
                // Handle OR pattern: left | right
                let (left_exprs, left_keys) = extract_keys_from_expr(&binary.left)?;
                let (right_exprs, right_keys) = extract_keys_from_expr(&binary.right)?;

                let mut exprs = left_exprs;
                exprs.extend(right_exprs);

                let mut keys = left_keys;
                keys.extend(right_keys);

                Ok((exprs, keys))
            } else {
                // Single key
                let parsed = ParsedKey::from_expr(expr)
                    .ok_or_else(|| Error::new_spanned(expr, "unsupported key expression"))?;
                Ok((vec![expr.clone()], vec![parsed]))
            }
        }
        _ => {
            // Single key
            let parsed = ParsedKey::from_expr(expr)
                .ok_or_else(|| Error::new_spanned(expr, "unsupported key expression"))?;
            Ok((vec![expr.clone()], vec![parsed]))
        }
    }
}

#[derive(Clone)]
struct Entry {
    key: Key,
    value: Expr,
    attrs: Vec<syn::Attribute>,
}

impl PhfHash for Entry {
    fn phf_hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.key.phf_hash(state)
    }
}

impl Parse for Entry {
    fn parse(input: ParseStream<'_>) -> parse::Result<Entry> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let key = input.parse()?;
        input.parse::<Token![=>]>()?;
        let value = input.parse()?;
        Ok(Entry { key, value, attrs })
    }
}

struct Map(Vec<Entry>);

impl Parse for Map {
    fn parse(input: ParseStream<'_>) -> parse::Result<Map> {
        let parsed = Punctuated::<Entry, Token![,]>::parse_terminated(input)?;
        let mut expanded_entries = Vec::new();

        // Expand OR patterns into multiple entries
        for entry in parsed {
            for (i, (parsed_key, expr)) in entry
                .key
                .parsed
                .iter()
                .zip(entry.key.expr.iter())
                .enumerate()
            {
                let expanded_key = Key {
                    parsed: vec![parsed_key.clone()],
                    expr: vec![expr.clone()],
                    attrs: if i == 0 {
                        entry.key.attrs.clone()
                    } else {
                        Vec::new()
                    },
                };
                let expanded_entry = Entry {
                    key: expanded_key,
                    value: entry.value.clone(),
                    attrs: if i == 0 {
                        entry.attrs.clone()
                    } else {
                        Vec::new()
                    },
                };
                expanded_entries.push(expanded_entry);
            }
        }

        check_duplicates(&expanded_entries)?;
        Ok(Map(expanded_entries))
    }
}

struct Set(Vec<Entry>);

impl Parse for Set {
    fn parse(input: ParseStream<'_>) -> parse::Result<Set> {
        let parsed = Punctuated::<Key, Token![,]>::parse_terminated(input)?;
        let unit_value: Expr = syn::parse_str("()").expect("Failed to parse unit value");

        let mut expanded_entries = Vec::new();

        // Expand OR patterns into multiple entries
        for key in parsed {
            for (i, (parsed_key, expr)) in key.parsed.iter().zip(key.expr.iter()).enumerate() {
                let expanded_key = Key {
                    parsed: vec![parsed_key.clone()],
                    expr: vec![expr.clone()],
                    attrs: if i == 0 {
                        key.attrs.clone()
                    } else {
                        Vec::new()
                    },
                };
                let expanded_entry = Entry {
                    key: expanded_key,
                    value: unit_value.clone(),
                    attrs: if i == 0 {
                        key.attrs.clone()
                    } else {
                        Vec::new()
                    },
                };
                expanded_entries.push(expanded_entry);
            }
        }

        check_duplicates(&expanded_entries)?;
        Ok(Set(expanded_entries))
    }
}

fn check_duplicates(entries: &[Entry]) -> parse::Result<()> {
    let mut keys = HashSet::new();
    for entry in entries {
        if let Some(first) = entry.key.parsed.first() {
            if !keys.insert(first) {
                return Err(Error::new_spanned(&entry.key.expr[0], "duplicate key"));
            }
        }
    }
    Ok(())
}

fn build_map(entries: &[Entry], state: HashState) -> proc_macro2::TokenStream {
    let key = state.key;
    let disps = state.disps.iter().map(|&(d1, d2)| quote!((#d1, #d2)));
    let entries = state.map.iter().map(|&idx| {
        let entry = &entries[idx];
        let key = &entry.key.expr[0]; // Use the first expression
        let value = &entry.value;
        // Don't include attributes since we've filtered at macro expansion time
        quote!((#key, #value))
    });

    quote! {
        phf::Map {
            key: #key,
            disps: &[#(#disps),*],
            entries: &[#(#entries),*],
        }
    }
}

fn build_ordered_map(entries: &[Entry], state: HashState) -> proc_macro2::TokenStream {
    let key = state.key;
    let disps = state.disps.iter().map(|&(d1, d2)| quote!((#d1, #d2)));
    let idxs = state.map.iter().map(|idx| quote!(#idx));
    let entries = entries.iter().map(|entry| {
        let key = &entry.key.expr[0]; // Use the first expression
        let value = &entry.value;
        // Don't include attributes since we've filtered at macro expansion time
        quote!((#key, #value))
    });

    quote! {
        phf::OrderedMap {
            key: #key,
            disps: &[#(#disps),*],
            idxs: &[#(#idxs),*],
            entries: &[#(#entries),*],
        }
    }
}

#[proc_macro]
pub fn phf_map(input: TokenStream) -> TokenStream {
    let map = parse_macro_input!(input as Map);

    // Check if any entries have cfg attributes
    let has_cfg_attrs = map.0.iter().any(|entry| !entry.attrs.is_empty());

    if !has_cfg_attrs {
        // No cfg attributes - use the simple approach
        let state = phf_generator::generate_hash(&map.0);
        build_map(&map.0, state).into()
    } else {
        // Has cfg attributes - need to generate conditional map code
        build_conditional_phf_map(&map.0).into()
    }
}

/// Generate conditional cfg conditions for a given mask and conditional entries
fn build_cfg_conditions(mask: usize, conditional: &[&Entry]) -> Vec<proc_macro2::TokenStream> {
    let mut conditions = Vec::new();
    for (i, &entry) in conditional.iter().enumerate() {
        let include = (mask & (1 << i)) != 0;
        if let Some(attr) = entry.attrs.first() {
            if let Ok(meta) = attr.meta.require_list() {
                let tokens = &meta.tokens;
                if include {
                    conditions.push(quote!(cfg!(#tokens)));
                } else {
                    conditions.push(quote!(!cfg!(#tokens)));
                }
            }
        }
    }
    conditions
}

/// Combine multiple conditions into a single condition expression
fn combine_conditions(conditions: Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream {
    if conditions.is_empty() {
        quote!(true)
    } else if conditions.len() == 1 {
        conditions[0].clone()
    } else {
        quote!(#(#conditions)&&*)
    }
}

/// Generate nested if-else chain from variants
fn build_nested_conditional(
    variants: Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)>,
) -> proc_macro2::TokenStream {
    if variants.is_empty() {
        return quote!(compile_error!("No valid variants found"));
    }

    if variants.len() == 1 {
        return variants[0].1.clone();
    }

    let mut result = variants.last().unwrap().1.clone();
    for (condition, tokens) in variants.iter().rev().skip(1) {
        result = quote! {
            if #condition {
                #tokens
            } else {
                #result
            }
        };
    }
    quote! { { #result } }
}

/// Generic function to build conditional PHF structures
fn build_conditional_phf<F>(
    entries: &[Entry],
    simple_builder: F,
    empty_structure: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream
where
    F: Fn(&[Entry], HashState) -> proc_macro2::TokenStream,
{
    let unconditional: Vec<_> = entries.iter().filter(|e| e.attrs.is_empty()).collect();
    let conditional: Vec<_> = entries.iter().filter(|e| !e.attrs.is_empty()).collect();

    if conditional.is_empty() {
        let state = phf_generator::generate_hash(entries);
        return simple_builder(entries, state);
    }

    let mut variants = Vec::new();
    let num_conditional = conditional.len();

    for mask in 0..(1 << num_conditional) {
        let mut variant_entries = unconditional.clone();

        for (i, &entry) in conditional.iter().enumerate() {
            if (mask & (1 << i)) != 0 {
                variant_entries.push(entry);
            }
        }

        if variant_entries.is_empty() {
            continue;
        }

        let entries_vec: Vec<Entry> = variant_entries.into_iter().cloned().collect();
        let state = phf_generator::generate_hash(&entries_vec);
        let structure_tokens = simple_builder(&entries_vec, state);

        let conditions = build_cfg_conditions(mask, &conditional);
        let condition = combine_conditions(conditions);

        variants.push((condition, structure_tokens));
    }

    if variants.is_empty() {
        empty_structure
    } else {
        build_nested_conditional(variants)
    }
}

fn build_conditional_phf_map(entries: &[Entry]) -> proc_macro2::TokenStream {
    build_conditional_phf(
        entries,
        build_map,
        quote! {
            phf::Map {
                key: 0,
                disps: &[],
                entries: &[],
            }
        },
    )
}

#[proc_macro]
pub fn phf_set(input: TokenStream) -> TokenStream {
    let set = parse_macro_input!(input as Set);

    // Check if any entries have cfg attributes
    let has_cfg_attrs = set.0.iter().any(|entry| !entry.attrs.is_empty());

    if !has_cfg_attrs {
        // No cfg attributes - use the simple approach
        let state = phf_generator::generate_hash(&set.0);
        let map = build_map(&set.0, state);
        quote!(phf::Set { map: #map }).into()
    } else {
        // Has cfg attributes - need to generate conditional set code
        build_conditional_phf_set(&set.0).into()
    }
}

fn build_conditional_phf_set(entries: &[Entry]) -> proc_macro2::TokenStream {
    // Similar to conditional map but wraps in Set
    let map_tokens = build_conditional_phf_map(entries);
    quote!(phf::Set { map: #map_tokens })
}

#[proc_macro]
pub fn phf_ordered_map(input: TokenStream) -> TokenStream {
    let map = parse_macro_input!(input as Map);

    // Check if any entries have cfg attributes
    let has_cfg_attrs = map.0.iter().any(|entry| !entry.attrs.is_empty());

    if !has_cfg_attrs {
        // No cfg attributes - use the simple approach
        let state = phf_generator::generate_hash(&map.0);
        build_ordered_map(&map.0, state).into()
    } else {
        // Has cfg attributes - need to generate conditional ordered map code
        build_conditional_phf_ordered_map(&map.0).into()
    }
}

fn build_conditional_phf_ordered_map(entries: &[Entry]) -> proc_macro2::TokenStream {
    build_conditional_phf(
        entries,
        build_ordered_map,
        quote! {
            phf::OrderedMap {
                key: 0,
                disps: &[],
                idxs: &[],
                entries: &[],
            }
        },
    )
}

#[proc_macro]
pub fn phf_ordered_set(input: TokenStream) -> TokenStream {
    let set = parse_macro_input!(input as Set);

    let has_cfg_attrs = set.0.iter().any(|entry| !entry.attrs.is_empty());

    if !has_cfg_attrs {
        // No cfg attributes - use the simple approach
        let state = phf_generator::generate_hash(&set.0);
        let map = build_ordered_map(&set.0, state);
        quote!(phf::OrderedSet { map: #map }).into()
    } else {
        // Has cfg attributes - need to generate conditional ordered set code
        build_conditional_phf_ordered_set(&set.0).into()
    }
}

fn build_conditional_phf_ordered_set(entries: &[Entry]) -> proc_macro2::TokenStream {
    // Similar to conditional ordered map but wraps in OrderedSet
    let map_tokens = build_conditional_phf_ordered_map(entries);
    quote!(phf::OrderedSet { map: #map_tokens })
}
