use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::args::Args;
use crate::{attrs, fields, from, merge};

pub fn generate(original: &ItemStruct, args: Args) -> TokenStream {
    let mut opt_struct = original.clone();

    opt_struct.ident = args.item.name.clone();
    opt_struct.vis = args.item.final_visibility();

    opt_struct.attrs = attrs::generate(original, &args);
    opt_struct.fields = fields::generate(original, &args);

    let merge_impl = merge::generate(original, &opt_struct, &args);

    let from_impl = from::generate(original, &opt_struct, &args);

    quote! {
        #opt_struct

        #merge_impl

        #from_impl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_util::*;

    #[test]
    fn sets_name() {
        let (item, args) = parse_item_and_args(
            quote! {
                struct S;
            },
            quote! {
                Opt
            },
        );

        let generated = parse_item(generate(&item, args));

        assert_eq!(generated.ident, "Opt");
    }

    #[test]
    fn sets_generics() {
        let (item, args) = parse_item_and_args(
            quote! {
                struct S<'a, 'b, T, G> {
                    t: &'a T,
                    g: &'b G
                }
            },
            quote! {
                Opt
            },
        );

        let generated = parse_item(generate(&item, args));

        assert_eq!(item.generics, generated.generics);
    }

    #[test]
    fn sets_visibility() {
        let item = parse_item(quote! {
            struct S;
        });

        let cases = vec![
            (
                quote! {
                    Opt
                },
                quote!(),
            ),
            (
                quote! {
                    pub Opt
                },
                quote!(pub),
            ),
            (
                quote! {
                    pub(crate) Opt
                },
                quote!(pub(crate)),
            ),
            (
                quote! {
                    pub(in test::path) Opt
                },
                quote!(pub(in test::path)),
            ),
        ];

        for (args_tokens, vis_tokens) in cases {
            let args = parse_args(args_tokens);
            let vis = parse_visibility(vis_tokens);

            let generated = parse_item(generate(&item, args));

            assert_eq!(generated.vis, vis);
        }
    }
}
