// https://github.com/colin-kiegel/rust-derive-builder/issues/15
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate derive_builder;

#[derive(Debug, Clone, PartialEq)]
struct NotDefaultable(String);

fn new_notdefaultable() -> NotDefaultable {
    NotDefaultable("Lorem".to_string())
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(skip = "false"))]
struct SetterOptOut {
    setter_present_by_explicit_default: u32,
    #[builder(setter(skip = "true"))]
    setter_skipped_by_explicit_opt_out: u32,
    #[builder(setter(skip))]
    setter_skipped_by_shorthand_opt_out: u32,
    #[builder(setter(skip), default = "4")]
    setter_skipped_with_explicit_default: u32,
}

#[derive(Debug, PartialEq, Default, Builder, Clone)]
#[builder(setter(skip))]
struct SetterOptIn {
    setter_skipped_by_shorthand_default: u32,
    #[builder(setter(skip = false))] // Should be still OK without quotes
    setter_present_by_explicit_opt_in: u32,
    #[builder(setter)]
    setter_present_by_shorthand_opt_in: u32,
    #[builder(setter(prefix = "set"))]
    setter_present_by_shorthand_opt_in_2: u32,
}

#[derive(Debug, PartialEq, Builder, Clone)]
#[builder(default, setter(skip))]
struct SetterOptInStructDefault {
    setter_skipped_with_struct_default: NotDefaultable,
    setter_skipped_with_type_default: u32,
}

#[derive(Debug, PartialEq, Builder, Clone)]
#[builder(setter(into))]
struct SetterOptInFieldDefault {
    #[builder(setter(skip), default = "new_notdefaultable()")]
    setter_skipped_with_field_default: NotDefaultable,

    #[builder(default)]
    setter_present_by_default: u32,
}

// compile test
#[allow(dead_code)]
impl SetterOptOut {
    // only possible if setter was skipped
    fn setter_skipped_by_explicit_opt_out() {}
    // only possible if setter was skipped
    fn setter_skipped_by_shorthand_opt_out() {}
}

// compile test
#[allow(dead_code)]
impl SetterOptIn {
    // only possible if setter was skipped
    fn setter_skipped_by_shorthand_default() {}
}

impl Default for SetterOptInStructDefault {
    fn default() -> Self {
        SetterOptInStructDefault {
            setter_skipped_with_struct_default: new_notdefaultable(),
            setter_skipped_with_type_default: Default::default(),
        }
    }
}

#[test]
fn setter_opt_out() {
    let x: SetterOptOut = SetterOptOutBuilder::default()
        .setter_present_by_explicit_default(42u32)
        .build()
        .unwrap();

    assert_eq!(
        x,
        SetterOptOut {
            setter_present_by_explicit_default: 42,
            setter_skipped_by_explicit_opt_out: 0,
            setter_skipped_by_shorthand_opt_out: 0,
            setter_skipped_with_explicit_default: 4,
        }
    );
}

#[test]
fn setter_opt_in() {
    let x: SetterOptIn = SetterOptInBuilder::default()
        .setter_present_by_explicit_opt_in(47u32)
        .setter_present_by_shorthand_opt_in(11u32)
        .set_setter_present_by_shorthand_opt_in_2(815u32)
        .build()
        .unwrap();

    assert_eq!(
        x,
        SetterOptIn {
            setter_skipped_by_shorthand_default: 0,
            setter_present_by_explicit_opt_in: 47,
            setter_present_by_shorthand_opt_in: 11,
            setter_present_by_shorthand_opt_in_2: 815,
        }
    );
}

#[test]
fn setter_skipped_with_struct_default() {
    let x = SetterOptInStructDefaultBuilder::default().build().unwrap();

    assert_eq!(x, SetterOptInStructDefault::default());
}

#[test]
fn setter_skipped_with_field_default() {
    let x = SetterOptInFieldDefaultBuilder::default()
        .build()
        .expect("All fields were defaulted");

    assert_eq!(
        x,
        SetterOptInFieldDefault {
            setter_skipped_with_field_default: new_notdefaultable(),
            setter_present_by_default: Default::default(),
        }
    );
}
