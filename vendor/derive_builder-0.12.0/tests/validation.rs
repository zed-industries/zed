#[macro_use]
extern crate derive_builder;

#[derive(Debug, Clone, Builder, PartialEq, Eq)]
#[builder(build_fn(validate = "LoremBuilder::validate"))]
pub struct Lorem {
    /// A percentile. Must be between 0 and 100.
    my_effort: u8,

    /// A percentile. Must be less than or equal to `Lorem::my_effort`.
    #[builder(default = "40")]
    their_effort: u8,

    /// A percentile. Must be between 0 and 100.
    rivals_effort: u8,
}

impl LoremBuilder {
    /// Performs bound checks.
    fn validate(&self) -> Result<(), String> {
        if let Some(ref my_effort) = self.my_effort {
            if *my_effort > 100 {
                return Err("Don't wear yourself out".to_string());
            }
        }

        if let Some(ref their_effort) = self.their_effort {
            if *their_effort > 100 {
                return Err("The game has changed".to_string());
            }
        }

        if let Some(ref rivals_effort) = self.rivals_effort {
            if *rivals_effort > 100 {
                return Err("Your rival is cheating".to_string());
            }
        }

        Ok(())
    }
}

#[test]
fn out_of_bounds() {
    assert_eq!(
        &LoremBuilder::default()
            .my_effort(120)
            .build()
            .unwrap_err()
            .to_string(),
        "Don't wear yourself out"
    );
    assert_eq!(
        &LoremBuilder::default()
            .rivals_effort(120)
            .build()
            .unwrap_err()
            .to_string(),
        "Your rival is cheating"
    );
}

#[test]
fn validation_pass() {
    let lorem = LoremBuilder::default()
        .my_effort(90)
        .rivals_effort(89)
        .build()
        .expect("All validations should be passing");

    assert_eq!(
        lorem,
        Lorem {
            my_effort: 90,
            rivals_effort: 89,
            their_effort: 40,
        }
    );
}
