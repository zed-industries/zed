use crate::prelude::*;

#[test]
fn many_params_fn() {
    #[builder(builder_type = OverrideBuilder, finish_fn = finish)]
    fn sut(#[builder(default, name = renamed)] arg1: u32) -> u32 {
        arg1
    }

    let builder = || -> OverrideBuilder { sut() };
    assert_eq!(builder().finish(), 0);
    assert_eq!(builder().renamed(32).finish(), 32);
}

#[test]
fn many_attrs_struct() {
    #[derive(Builder)]
    #[builder(builder_type = OverrideBuilder)]
    #[builder(finish_fn = finish)]
    struct Sut {
        #[builder(default, name = renamed)]
        arg1: u32,

        #[builder(default)]
        #[builder(name = renamed2)]
        arg2: u32,
    }

    let builder = || -> OverrideBuilder { Sut::builder() };

    assert_eq!(builder().finish().arg1, 0);
    assert_eq!(builder().renamed(32).finish().arg1, 32);
    assert_eq!(builder().renamed2(32).finish().arg2, 32);
}

#[test]
fn many_params_in_one_attr_struct() {
    #[derive(Builder)]
    #[builder(builder_type = OverrideBuilder, finish_fn = finish)]
    #[builder(start_fn = start)]
    struct Sut {}

    let builder: OverrideBuilder = Sut::start();
    let _ = builder.finish();
}

#[test]
fn many_params_impl_block() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(builder_type = OverrideBuilder, finish_fn = finish)]
        fn method(#[builder(default, name = renamed)] arg1: u32) -> u32 {
            arg1
        }
    }

    let builder = || -> OverrideBuilder { Sut::method() };
    assert_eq!(builder().finish(), 0);
    assert_eq!(builder().renamed(32).finish(), 32);
}
