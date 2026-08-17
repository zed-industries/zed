use crate::prelude::*;
use macro_rules_attribute::apply;

#[test]
fn attributes_see_orig_fn_name() {
    macro_rules! return_fn_name {
        (
            $(#[$attr:meta])*
            fn $fn_name:ident() -> $ret:ty {}
        ) => {
            $(#[$attr])*
            fn $fn_name() -> $ret {
                stringify!($fn_name)
            }
        };
    }

    #[builder]
    #[apply(return_fn_name!)]
    fn attr_after_builder() -> &'static str {}

    #[apply(return_fn_name!)]
    #[builder]
    fn attr_before_builder() -> &'static str {}

    struct Sut;

    #[bon]
    impl Sut {
        #[builder]
        #[apply(return_fn_name!)]
        fn attr_after_builder() -> &'static str {}

        #[apply(return_fn_name!)]
        #[builder]
        fn attr_before_builder() -> &'static str {}
    }

    let actual = attr_after_builder().call();
    let expected = expect!["attr_after_builder"];

    expected.assert_eq(actual);

    let actual = attr_before_builder().call();
    let expected = expect!["attr_before_builder"];

    expected.assert_eq(actual);

    let actual = Sut::attr_after_builder().call();
    let expected = expect!["attr_after_builder"];

    expected.assert_eq(actual);

    let actual = Sut::attr_before_builder().call();
    let expected = expect!["attr_before_builder"];

    expected.assert_eq(actual);
}
