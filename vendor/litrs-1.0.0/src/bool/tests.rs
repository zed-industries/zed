use crate::{test_util::assert_parse_ok_eq, BoolLit, Literal};

macro_rules! assert_bool_parse {
    ($input:literal, $expected:expr) => {
        assert_parse_ok_eq(
            $input,
            Literal::parse($input),
            Literal::Bool($expected),
            "Literal::parse",
        );
        assert_parse_ok_eq($input, BoolLit::parse($input), $expected, "BoolLit::parse");
    };
}



#[test]
fn parse_ok() {
    assert_bool_parse!("false", BoolLit::False);
    assert_bool_parse!("true", BoolLit::True);
}

#[test]
fn parse_err() {
    assert!(Literal::parse("fa").is_err());
    assert!(Literal::parse("fal").is_err());
    assert!(Literal::parse("fals").is_err());
    assert!(Literal::parse(" false").is_err());
    assert!(Literal::parse("false ").is_err());
    assert!(Literal::parse("False").is_err());

    assert!(Literal::parse("tr").is_err());
    assert!(Literal::parse("tru").is_err());
    assert!(Literal::parse(" true").is_err());
    assert!(Literal::parse("true ").is_err());
    assert!(Literal::parse("True").is_err());
}

#[test]
fn value() {
    assert!(!BoolLit::False.value());
    assert!(BoolLit::True.value());
}

#[test]
fn as_str() {
    assert_eq!(BoolLit::False.as_str(), "false");
    assert_eq!(BoolLit::True.as_str(), "true");
}
