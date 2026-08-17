/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn _cssparser_internal_max_len(input: TokenStream) -> TokenStream {
    struct Input {
        max_length: usize,
    }

    impl syn::parse::Parse for Input {
        fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
            let mut max_length = 0;
            while !input.is_empty() {
                if input.peek(syn::Token![_]) {
                    input.parse::<syn::Token![_]>().unwrap();
                    continue;
                }
                let lit: syn::LitStr = input.parse()?;
                let value = lit.value();
                if value.to_ascii_lowercase() != value {
                    return Err(syn::Error::new(lit.span(), "must be ASCII-lowercase"));
                }
                max_length = max_length.max(value.len());
            }
            Ok(Input { max_length })
        }
    }

    let Input { max_length } = syn::parse_macro_input!(input);
    quote::quote!(
        pub(super) const MAX_LENGTH: usize = #max_length;
    )
    .into()
}

fn get_byte_from_lit(lit: &syn::Lit) -> u8 {
    if let syn::Lit::Byte(ref byte) = *lit {
        byte.value()
    } else {
        panic!("Found a pattern that wasn't a byte")
    }
}

fn get_byte_from_expr_lit(expr: &syn::Expr) -> u8 {
    match *expr {
        syn::Expr::Lit(syn::ExprLit { ref lit, .. }) => {
            get_byte_from_lit(lit)
        }
        _ => unreachable!(),
    }
}

/// Parse a pattern and fill the table accordingly
fn parse_pat_to_table<'a>(
    pat: &'a syn::Pat,
    case_id: u8,
    wildcard: &mut Option<&'a syn::Ident>,
    table: &mut [u8; 256],
) {
    match pat {
        &syn::Pat::Lit(syn::PatLit { ref lit, .. }) => {
            let value = get_byte_from_lit(lit);
            if table[value as usize] == 0 {
                table[value as usize] = case_id;
            }
        }
        &syn::Pat::Range(syn::PatRange { ref start, ref end, .. }) => {
            let lo = get_byte_from_expr_lit(&start.as_ref().unwrap());
            let hi = get_byte_from_expr_lit(&end.as_ref().unwrap());
            for value in lo..hi {
                if table[value as usize] == 0 {
                    table[value as usize] = case_id;
                }
            }
            if table[hi as usize] == 0 {
                table[hi as usize] = case_id;
            }
        }
        &syn::Pat::Wild(_) => {
            for byte in table.iter_mut() {
                if *byte == 0 {
                    *byte = case_id;
                }
            }
        }
        &syn::Pat::Ident(syn::PatIdent { ref ident, .. }) => {
            assert_eq!(*wildcard, None);
            *wildcard = Some(ident);
            for byte in table.iter_mut() {
                if *byte == 0 {
                    *byte = case_id;
                }
            }
        }
        &syn::Pat::Or(syn::PatOr { ref cases, .. }) => {
            for case in cases {
                parse_pat_to_table(case, case_id, wildcard, table);
            }
        }
        _ => {
            panic!("Unexpected pattern: {:?}. Buggy code ?", pat);
        }
    }
}

/// Expand a TokenStream corresponding to the `match_byte` macro.
///
/// ## Example
///
/// ```rust
/// match_byte! { tokenizer.next_byte_unchecked(),
///     b'a'..b'z' => { ... }
///     b'0'..b'9' => { ... }
///     b'\n' | b'\\' => { ... }
///     foo => { ... }
///  }
///  ```
///
#[proc_macro]
pub fn match_byte(input: TokenStream) -> TokenStream {
    use syn::spanned::Spanned;
    struct MatchByte {
        expr: syn::Expr,
        arms: Vec<syn::Arm>,
    }

    impl syn::parse::Parse for MatchByte {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(MatchByte {
                expr: {
                    let expr = input.parse()?;
                    input.parse::<syn::Token![,]>()?;
                    expr
                },
                arms: {
                    let mut arms = Vec::new();
                    while !input.is_empty() {
                        let arm = input.call(syn::Arm::parse)?;
                        assert!(arm.guard.is_none(), "match_byte doesn't support guards");
                        assert!(
                            arm.attrs.is_empty(),
                            "match_byte doesn't support attributes"
                        );
                        arms.push(arm);
                    }
                    arms
                },
            })
        }
    }
    let MatchByte { expr, arms } = syn::parse_macro_input!(input);

    let mut cases = Vec::new();
    let mut table = [0u8; 256];
    let mut match_body = Vec::new();
    let mut wildcard = None;
    for (i, ref arm) in arms.iter().enumerate() {
        let case_id = i + 1;
        let index = case_id as isize;
        let name = syn::Ident::new(&format!("Case{}", case_id), arm.span());
        let pat = &arm.pat;
        parse_pat_to_table(pat, case_id as u8, &mut wildcard, &mut table);

        cases.push(quote::quote!(#name = #index));
        let body = &arm.body;
        match_body.push(quote::quote!(Case::#name => { #body }))
    }

    let en = quote::quote!(enum Case {
        #(#cases),*
    });

    let mut table_content = Vec::new();
    for entry in table.iter() {
        let name: syn::Path = syn::parse_str(&format!("Case::Case{}", entry)).unwrap();
        table_content.push(name);
    }
    let table = quote::quote!(static __CASES: [Case; 256] = [#(#table_content),*];);

    if let Some(binding) = wildcard {
        quote::quote!({ #en #table let #binding = #expr; match __CASES[#binding as usize] { #(#match_body),* }})
    } else {
        quote::quote!({ #en #table match __CASES[#expr as usize] { #(#match_body),* }})
    }.into()
}
