#![deny(warnings)]
use bon::{bon, builder, Builder};

fn main() {
    // Test #[must_use] for setters
    {
        #[allow(dead_code)]
        #[derive(Builder)]
        struct Example {
            x: u32,
            y: u32,
        }

        #[bon]
        impl Example {
            #[builder]
            #[must_use]
            fn must_use() -> u32 {
                99
            }
        }

        #[builder]
        #[must_use]
        fn must_use() -> u32 {
            99
        }

        Example::builder();
        Example::must_use();
        must_use();

        Example::builder().x(1);
        Example::builder().x(1).y(2).build();

        Example::must_use().call();

        must_use().call();
        __orig_must_use();

        #[builder]
        #[cfg_attr(all(), must_use = "must use message")]
        fn must_use_under_cfg() -> u32 {
            99
        }

        must_use_under_cfg().call();

        #[builder]
        #[cfg_attr(any(), must_use = "unreachable must use")]
        fn must_use_compiled_out() -> u32 {
            99
        }

        must_use_compiled_out().call();
    }

    // Test #[must_use] for getters
    {
        #[derive(Builder)]
        struct Sut {
            #[builder(getter)]
            x1: u32,

            #[builder(getter, default)]
            x2: u32,

            #[builder(getter)]
            x3: Option<u32>,
        }

        let builder = Sut::builder().x1(1).x2(2).x3(3);

        // Make sure there are `#[must_use]` warnings
        builder.get_x1();
        builder.get_x2();
        builder.get_x3();
    }
}
