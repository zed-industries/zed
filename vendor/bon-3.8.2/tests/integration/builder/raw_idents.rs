//! This is based on the issue <https://github.com/elastio/bon/issues/8>
use crate::prelude::*;

#[test]
#[allow(non_camel_case_types)]
fn test_struct() {
    {
        #[derive(Builder)]
        struct r#Type {
            r#type: u32,

            #[builder(name = r#while)]
            other: u32,
        }

        let actual = r#Type::builder().r#type(42).r#while(100).build();

        assert_eq!(actual.r#type, 42);
        assert_eq!(actual.other, 100);

        #[derive(Builder)]
        #[builder(builder_type = r#type, state_mod = r#mod)]
        #[allow(clippy::items_after_statements, dead_code)]
        struct Sut {
            r#while: u32,
        }

        let _actual: r#type = Sut::builder();
        let _actual: r#type<r#mod::SetWhile> = Sut::builder().r#while(32);
    }
    // This is based on the issue https://github.com/elastio/bon/issues/174
    {
        {
            #[derive(Builder)]
            struct Sut {
                r#type: Option<u32>,
            }

            let value = Sut::builder().maybe_type(Some(2)).build();
            assert_eq!(value.r#type, Some(2));
        }
        {
            #[derive(Builder)]
            struct Sut {
                #[builder(default)]
                r#type: u32,
            }

            let value = Sut::builder().maybe_type(Some(2)).build();
            assert_eq!(value.r#type, 2);
        }
    }
}

#[test]
#[allow(non_camel_case_types)]
fn test_fn() {
    {
        #[builder]
        fn r#type(r#type: u32, #[builder(name = r#while)] other: u32) {
            let _ = (r#type, other);
        }

        r#type().r#type(42).r#while(100).call();
    }

    {
        #[builder(builder_type = r#type, state_mod = r#mod)]
        fn sut() {}

        let _: r#type = sut();
    }
    // This is based on the issue https://github.com/elastio/bon/issues/174
    {
        {
            #[builder]
            fn sut(r#type: Option<u32>) -> Option<u32> {
                r#type
            }

            let value = sut().maybe_type(Some(2)).call();
            assert_eq!(value, Some(2));
        }
        {
            #[builder]
            fn sut(#[builder(default)] r#type: u32) -> u32 {
                r#type
            }

            let value = sut().maybe_type(Some(2)).call();
            assert_eq!(value, 2);
        }
    }
}

// This is based on the issue https://github.com/elastio/bon/issues/237
#[test]
fn test_self_underscore_bug_237() {
    #[derive(Builder)]
    struct Sut {
        self_: u32,
    }

    let builder: SutBuilder<sut_builder::SetSelf_> = Sut::builder().self_(42);

    assert_eq!(builder.build().self_, 42);
}
