use syn::{AttrStyle, Attribute, Expr, FnArg, ItemFn, Meta, PatType};

/// A parsed argument, with an optional custom strategy
pub struct Argument {
    pub pat_ty: PatType,
    pub strategy: Option<Expr>,
}

/// Convert a function to a zero-arg function, and return the args
///
/// Panics on any invalid function
pub fn strip_args(mut f: ItemFn) -> (ItemFn, Vec<Argument>) {
    let args = std::mem::take(&mut f.sig.inputs);
    let args = args
        .into_iter()
        .map(|arg| match arg {
            FnArg::Typed(arg) => strip_strategy(arg),
            FnArg::Receiver(_) => panic!(
                "receivers aren't allowed - should be filtered by `validate`"
            ),
        })
        .collect();

    (f, args)
}

fn strip_strategy(mut pat_ty: PatType) -> Argument {
    let (strategies, others) = pat_ty.attrs.into_iter().partition(is_strategy);

    pat_ty.attrs = others;

    let strategy = match &strategies[..] {
        [] => None,
        [s] => match &s.meta {
            Meta::NameValue(name_value) => Some(name_value.value.clone()),
            _ => panic!("invalid strategies should be filtered by validate"),
        },
        _ => panic!("multiple strategies should be filtered by validate"),
    };

    Argument { pat_ty, strategy }
}

/// Checks if an attribute counts as a "strategy" attribute
///
/// This means:
///  - it is an outer attribute (i.e. `#[...]` not `#![...]`)
///  - it contains `strategy = <expr>`
pub fn is_strategy(attr: &Attribute) -> bool {
    let path_correct = attr
        .path()
        .get_ident()
        .map(|ident| ident == "strategy")
        .unwrap_or(false);

    let has_equals = matches!(&attr.meta, Meta::NameValue(_));

    let is_outer = matches!(attr.style, AttrStyle::Outer);

    path_correct && has_equals && is_outer
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn strip_args_works() {
        let f = parse_quote! { fn foo(i: i32) {} };
        let (f, mut args) = strip_args(f);

        assert_eq!(f.to_token_stream().to_string(), "fn foo () { }");

        assert_eq!(args.len(), 1);
        let arg = args.pop().unwrap();
        assert_eq!(arg.pat_ty.to_token_stream().to_string(), "i : i32");
        assert!(arg.strategy.is_none());
    }

    #[test]
    #[should_panic]
    fn strip_args_panics_with_self() {
        let f = parse_quote! { fn foo(self) {} };
        strip_args(f);
    }

    #[test]
    fn is_strategy_works() {
        let attr = parse_quote! { #[strategy = 123] };
        assert!(is_strategy(&attr));

        let attr = parse_quote! { #![strategy = 123] };
        assert!(!is_strategy(&attr));

        let attr = parse_quote! { #[not_strategy = 123] };
        assert!(!is_strategy(&attr));

        let attr = parse_quote! { #[strategy(but, no, equals)] };
        assert!(!is_strategy(&attr));

        let attr = parse_quote! { #[strategy] };
        assert!(!is_strategy(&attr));
    }

    #[test]
    fn strip_strategy_works() {
        let f = parse_quote! {fn foo(#[strategy = 123] x: i32) {} };
        let Argument { pat_ty, strategy } = strip_args(f).1.pop().unwrap();
        // let Argument { pat_ty, strategy } = strip_strategy(parse_quote! {
        //     #[strategy] x: i32
        // });
        assert_eq!(pat_ty.to_token_stream().to_string(), "x : i32");
        assert_eq!(strategy.to_token_stream().to_string(), "123");
    }
}
