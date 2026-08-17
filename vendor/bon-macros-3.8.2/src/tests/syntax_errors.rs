use super::snapshot;
use crate::util::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum MacroKind {
    #[allow(dead_code)]
    builder,
    bon,
}

#[track_caller]
fn assert_builder_codegen(
    macro_kind: MacroKind,
    test_name: &'static str,
    params: TokenStream,
    item: TokenStream,
) {
    let sut = match macro_kind {
        MacroKind::builder => crate::builder::generate_from_attr,
        MacroKind::bon => crate::bon::generate,
    };

    let actual = sut(params, item);
    let actual = syn::parse2(actual.clone())
        .map(|actual| prettyplease::unparse(&actual))
        // There is a syntax error, so we can't prettify it
        .unwrap_or_else(|_err| actual.to_string());

    snapshot(test_name).assert_eq(&actual);
}

macro_rules! test_codegen {
    (
        #[$test_name:ident]
        #[$kind:ident$( ( $( $params:tt )* ) )?]
        $( $item:tt )*
    ) => {
        #[test]
        fn $test_name() {
            assert_builder_codegen(
                MacroKind::$kind,
                stringify!($test_name),
                quote!( $( $( $params )* )?),
                quote!( $( $item )* ),
            );
        }
    }
}

test_codegen! {
    #[bon_incomplete_if]
    #[bon]
    impl Sut {
        #[builder]
        fn sut() {
            if 1
        }
    }
}
