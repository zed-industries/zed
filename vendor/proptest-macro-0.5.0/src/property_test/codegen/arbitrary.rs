use super::*;
use quote::quote_spanned;

/// Generate the arbitrary impl for the struct
pub(super) fn gen_arbitrary_impl(
    fn_name: &Ident,
    args: &[Argument],
    options: &Options,
) -> TokenStream {
    if args.iter().all(|arg| arg.strategy.is_none()) {
        no_custom_strategies(fn_name, args, options)
    } else {
        custom_strategies(fn_name, args, options)
    }
}

// we can avoid boxing strategies if there are no custom strategies, since we have types written
// out in function args
//
// If there are custom strategies, we can't write the type, because we're only provided the
// expression for the strategy (e.g. `#[strategy = my_custom_strategy()]` doesn't tell us the
// return type of `my_custom_strategy`). In these cases, we just use `BoxedStrategy<Self>`
fn no_custom_strategies(fn_name: &Ident, args: &[Argument], options: &Options) -> TokenStream {
    let proptest = options.true_proptest_path();
    let arg_types = args.iter().map(|arg| {
        let ty = &arg.pat_ty.ty;
        quote!(#ty,)
    });

    let arg_types = quote! { #(#arg_types)* };

    let arg_names = args.iter().enumerate().map(|(index, _arg)| {
        let name = nth_field_name(args, index);
        quote!(#name,)
    });

    let arg_names = quote! { #(#arg_names)* };

    let strategy_type = quote! {
        #proptest::strategy::Map<#proptest::arbitrary::StrategyFor<(#arg_types)>, fn((#arg_types)) -> Self>
    };

    let strategy_expr = quote! {
        use #proptest::strategy::Strategy;
        #proptest::prelude::any::<(#arg_types)>().prop_map(|(#arg_names)| Self { #arg_names })
    };

    arbitrary_shared(fn_name, strategy_type, strategy_expr, options)
}

// if we have `fn foo(#[strategy = x] a: i32, b: i32) {}`, we want to generate something like this:
// ```ignore
// impl Arbitrary for FooArgs {
//   type Parameters = ();
//   type Strategy = BoxedStrategy<Self>;
//
//   fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
//      (x, any::<i32>()).prop_map(|(a, b)| Self { a, b }).boxed()
//   }
// }
// ```
fn custom_strategies(fn_name: &Ident, args: &[Argument], options: &Options) -> TokenStream {
    let proptest = options.true_proptest_path();
    let arg_strategies: TokenStream =
        args.iter()
            .map(|arg| {
                arg.strategy.as_ref().map(|s| quote! {#s,}).unwrap_or_else(
                    || {
                        let ty = &arg.pat_ty.ty;
                        quote_spanned! {
                            ty.span() => #proptest::prelude::any::<#ty>(),
                        }
                    },
                )
            })
            .collect();

    let arg_names: TokenStream = args
        .iter()
        .enumerate()
        .map(|(index, _arg)| {
            let name = nth_field_name(args, index);
            quote!(#name,)
        })
        .collect();
    let arg_names = &arg_names;

    let strategy_expr = quote! {
        use #proptest::strategy::Strategy;
        (#arg_strategies).prop_map(|(#arg_names)| Self { #arg_names }).boxed()
    };

    let strategy_type = quote! {
        #proptest::strategy::BoxedStrategy<Self>
    };
    arbitrary_shared(fn_name, strategy_type, strategy_expr, options)
}

/// shared code between both boxed and unboxed paths
fn arbitrary_shared(
    fn_name: &Ident,
    strategy_type: TokenStream,
    strategy_expr: TokenStream,
    options: &Options,
) -> TokenStream {
    let proptest = options.true_proptest_path();
    let struct_name = struct_name(fn_name);

    quote! {
        impl #proptest::prelude::Arbitrary for #struct_name {
            type Parameters = ();
            type Strategy = #strategy_type;

            fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
                #strategy_expr
            }
        }
    }
}
