//! This example illustrates the use of `validate` to add a pre-build validation
//! step.

#[macro_use]
extern crate derive_builder;

#[derive(Builder, Debug, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
struct Lorem {
    pub ipsum: u8,
}

impl LoremBuilder {
    /// Check that `Lorem` is putting in the right amount of effort.
    fn validate(&self) -> Result<(), String> {
        if let Some(ref ipsum) = self.ipsum {
            match *ipsum {
                i if i < 20 => Err("Try harder".to_string()),
                i if i > 100 => Err("You'll tire yourself out".to_string()),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

fn main() {
    // If we're trying too hard...
    let x = LoremBuilder::default().ipsum(120).build().unwrap_err();

    // .. the build will fail:
    assert_eq!(&x.to_string(), "You'll tire yourself out");
}
