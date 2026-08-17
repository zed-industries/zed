#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(skip = "false"), default)]
struct SetterCustom {
    #[builder(setter(custom = "true"))]
    setter_custom_by_explicit_opt_in: u32,
    #[builder(setter(custom))]
    setter_custom_shorthand: u32,
    #[builder(setter(custom = "false"))]
    setter_custom_by_explicit_opt_out: u32,
    #[builder(setter(custom = "true"), default = "4")]
    setter_custom_with_explicit_default: u32,
    #[builder(setter(custom = "true", strip_option))]
    setter_custom_with_strip_option: Option<u32>,
}

// compile test
#[allow(dead_code)]
impl SetterCustomBuilder {
    // only possible if setter was skipped
    fn setter_custom_by_explicit_opt_in(&mut self) -> &mut Self {
        self.setter_custom_by_explicit_opt_in = Some(1);
        self
    }

    // only possible if setter was skipped
    fn setter_custom_shorthand(&mut self) -> &mut Self {
        self.setter_custom_shorthand = Some(2);
        self
    }

    // only possible if setter was skipped
    fn setter_custom_with_explicit_default(&mut self) -> &mut Self {
        self.setter_custom_with_explicit_default = Some(43);
        self
    }

    // only possible if setter was skipped
    fn setter_custom_with_strip_option(&mut self) -> &mut Self {
        self.setter_custom_with_strip_option = Some(Some(6));
        self
    }
}

#[test]
fn setter_custom_defaults() {
    let x: SetterCustom = SetterCustomBuilder::default().build().unwrap();

    assert_eq!(
        x,
        SetterCustom {
            setter_custom_by_explicit_opt_in: 0,
            setter_custom_shorthand: 0,
            setter_custom_by_explicit_opt_out: 0,
            setter_custom_with_explicit_default: 4,
            setter_custom_with_strip_option: None,
        }
    );
}

#[test]
fn setter_custom_setters_called() {
    let x: SetterCustom = SetterCustomBuilder::default()
        .setter_custom_by_explicit_opt_in() // set to 1
        .setter_custom_shorthand() // set to 2
        .setter_custom_by_explicit_opt_out(42)
        .setter_custom_with_explicit_default() // set to 43
        .setter_custom_with_strip_option() // set to 6
        .build()
        .unwrap();

    assert_eq!(
        x,
        SetterCustom {
            setter_custom_by_explicit_opt_in: 1,
            setter_custom_shorthand: 2,
            setter_custom_by_explicit_opt_out: 42,
            setter_custom_with_explicit_default: 43,
            setter_custom_with_strip_option: Some(6)
        }
    );
}
