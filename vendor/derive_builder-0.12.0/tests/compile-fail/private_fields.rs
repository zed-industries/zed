#[macro_use]
extern crate derive_builder;

/// This builder is in an inner module to make sure private fields aren't accessible
/// from the `main` function.
mod inner {
    /// The `LoremBuilder` struct will have private fields for `ipsum` and `dolor`, and
    /// a public `sit` field.
    #[derive(Debug, Builder)]
    #[builder(field(private), setter(into))]
    pub struct Lorem {
        ipsum: String,
        dolor: u16,
        #[builder(field(public))]
        sit: bool,
    }
}

fn main() {
    use inner::LoremBuilder;
    
    let mut lorem = LoremBuilder::default();
    lorem.dolor(15u16);
    lorem.sit = Some(true); // <-- public
    lorem.dolor = Some(0); // <-- private
}
