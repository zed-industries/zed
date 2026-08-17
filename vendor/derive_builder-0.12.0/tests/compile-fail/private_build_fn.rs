#[macro_use]
extern crate derive_builder;

mod container {
    /// `LoremBuilder` should be accessible outside the module, but its
    /// build method should not be.
    #[derive(Debug, Default, Builder)]
    #[builder(default, public, build_fn(private))]
    pub struct Lorem {
        foo: usize,
        bar: String,
    }

    impl LoremBuilder {
        /// Create a `Lorem`
        pub fn my_build(&self) -> Lorem {
            self.build().expect("All good")
        }
    }
}

fn main() {
    use container::{Lorem, LoremBuilder};

    let lorem1 = LoremBuilder::default().my_build();

    let lorem2 = LoremBuilder::default().build().unwrap();

    println!("{:?} vs {:?}", lorem1, lorem2);
}
