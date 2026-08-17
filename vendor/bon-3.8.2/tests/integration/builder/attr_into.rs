use crate::prelude::*;
use core::fmt;
use core::num::NonZeroU32;

#[cfg(feature = "alloc")]
#[test]
fn into_attr_alloc() {
    use core::fmt;

    #[builder]
    fn sut(#[builder(into)] set: Option<BTreeSet<u32>>, no_into: String) -> impl fmt::Debug {
        (set, no_into)
    }

    assert_debug_eq(
        sut().set([32, 43]).no_into("disabled".into()).call(),
        expect![[r#"(Some({32, 43}), "disabled")"#]],
    );
}

#[test]
fn into_attr_no_std() {
    #[builder]
    fn sut(
        #[builder(into)] str_ref: &str,

        /// Some docs
        #[builder(into)]
        u32: u32,
    ) -> impl fmt::Debug + '_ {
        (str_ref, u32)
    }

    struct IntoStrRef<'a>(&'a str);

    impl<'a> From<IntoStrRef<'a>> for &'a str {
        fn from(val: IntoStrRef<'a>) -> Self {
            val.0
        }
    }

    assert_debug_eq(
        sut()
            .str_ref(IntoStrRef("vinyl-scratch"))
            .u32(NonZeroU32::new(32).unwrap())
            .call(),
        expect![[r#"("vinyl-scratch", 32)"#]],
    );
}

#[cfg(feature = "alloc")]
#[test]
fn into_string() {
    #[builder(on(String, into))]
    fn sut(arg1: String, arg2: Option<String>) -> impl fmt::Debug {
        (arg1, arg2)
    }

    assert_debug_eq(
        sut().arg1("blackjack").arg2("bruh").call(),
        expect![[r#"("blackjack", Some("bruh"))"#]],
    );

    assert_debug_eq(
        sut().arg1("blackjack").maybe_arg2(Some("bruh2")).call(),
        expect![[r#"("blackjack", Some("bruh2"))"#]],
    );
}
