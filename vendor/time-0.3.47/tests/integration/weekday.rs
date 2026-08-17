use rstest::rstest;
use time::Weekday::{self, *};

#[rstest]
#[case(Sunday, Saturday)]
#[case(Monday, Sunday)]
#[case(Tuesday, Monday)]
#[case(Wednesday, Tuesday)]
#[case(Thursday, Wednesday)]
#[case(Friday, Thursday)]
#[case(Saturday, Friday)]
fn previous(#[case] current: Weekday, #[case] expected: Weekday) {
    assert_eq!(current.previous(), expected);
}

#[rstest]
#[case(Sunday, Monday)]
#[case(Monday, Tuesday)]
#[case(Tuesday, Wednesday)]
#[case(Wednesday, Thursday)]
#[case(Thursday, Friday)]
#[case(Friday, Saturday)]
#[case(Saturday, Sunday)]
fn next(#[case] current: Weekday, #[case] expected: Weekday) {
    assert_eq!(current.next(), expected);
}

#[rstest]
#[case(Sunday, 0, Sunday)]
#[case(Sunday, 1, Monday)]
#[case(Sunday, 2, Tuesday)]
#[case(Sunday, 3, Wednesday)]
#[case(Sunday, 4, Thursday)]
#[case(Sunday, 5, Friday)]
#[case(Sunday, 6, Saturday)]
#[case(Monday, 0, Monday)]
#[case(Monday, 1, Tuesday)]
#[case(Monday, 2, Wednesday)]
#[case(Monday, 3, Thursday)]
#[case(Monday, 4, Friday)]
#[case(Monday, 5, Saturday)]
#[case(Monday, 6, Sunday)]
#[case(Sunday, 7, Sunday)]
#[case(Sunday, u8::MAX, Wednesday)]
#[case(Monday, 7, Monday)]
#[case(Monday, u8::MAX, Thursday)]
fn nth_next(#[case] current: Weekday, #[case] n: u8, #[case] expected: Weekday) {
    assert_eq!(current.nth_next(n), expected);
}

#[rstest]
#[case(Sunday, 0, Sunday)]
#[case(Sunday, 1, Saturday)]
#[case(Sunday, 2, Friday)]
#[case(Sunday, 3, Thursday)]
#[case(Sunday, 4, Wednesday)]
#[case(Sunday, 5, Tuesday)]
#[case(Sunday, 6, Monday)]
#[case(Monday, 0, Monday)]
#[case(Monday, 1, Sunday)]
#[case(Monday, 2, Saturday)]
#[case(Monday, 3, Friday)]
#[case(Monday, 4, Thursday)]
#[case(Monday, 5, Wednesday)]
#[case(Monday, 6, Tuesday)]
#[case(Sunday, 7, Sunday)]
#[case(Sunday, u8::MAX, Thursday)]
#[case(Monday, 7, Monday)]
#[case(Monday, u8::MAX, Friday)]
fn nth_prev(#[case] current: Weekday, #[case] n: u8, #[case] expected: Weekday) {
    assert_eq!(current.nth_prev(n), expected);
}

#[rstest]
#[case(Monday, 1)]
#[case(Tuesday, 2)]
#[case(Wednesday, 3)]
#[case(Thursday, 4)]
#[case(Friday, 5)]
#[case(Saturday, 6)]
#[case(Sunday, 7)]
fn number_from_monday(#[case] weekday: Weekday, #[case] expected: u8) {
    assert_eq!(weekday.number_from_monday(), expected);
}

#[rstest]
#[case(Sunday, 1)]
#[case(Monday, 2)]
#[case(Tuesday, 3)]
#[case(Wednesday, 4)]
#[case(Thursday, 5)]
#[case(Friday, 6)]
#[case(Saturday, 7)]
fn number_from_sunday(#[case] weekday: Weekday, #[case] expected: u8) {
    assert_eq!(weekday.number_from_sunday(), expected);
}

#[rstest]
#[case(Monday, 0)]
#[case(Tuesday, 1)]
#[case(Wednesday, 2)]
#[case(Thursday, 3)]
#[case(Friday, 4)]
#[case(Saturday, 5)]
#[case(Sunday, 6)]
fn number_days_from_monday(#[case] weekday: Weekday, #[case] expected: u8) {
    assert_eq!(weekday.number_days_from_monday(), expected);
}

#[rstest]
#[case(Sunday, 0)]
#[case(Monday, 1)]
#[case(Tuesday, 2)]
#[case(Wednesday, 3)]
#[case(Thursday, 4)]
#[case(Friday, 5)]
#[case(Saturday, 6)]
fn number_days_from_sunday(#[case] weekday: Weekday, #[case] expected: u8) {
    assert_eq!(weekday.number_days_from_sunday(), expected);
}

#[rstest]
#[case(Monday, "Monday")]
#[case(Tuesday, "Tuesday")]
#[case(Wednesday, "Wednesday")]
#[case(Thursday, "Thursday")]
#[case(Friday, "Friday")]
#[case(Saturday, "Saturday")]
#[case(Sunday, "Sunday")]
fn display(#[case] weekday: Weekday, #[case] expected: &str) {
    assert_eq!(weekday.to_string(), expected);
}

#[rstest]
#[case("Monday", Ok(Monday))]
#[case("Tuesday", Ok(Tuesday))]
#[case("Wednesday", Ok(Wednesday))]
#[case("Thursday", Ok(Thursday))]
#[case("Friday", Ok(Friday))]
#[case("Saturday", Ok(Saturday))]
#[case("Sunday", Ok(Sunday))]
#[case("foo", Err(time::error::InvalidVariant))]
fn from_str(#[case] input: &str, #[case] expected: Result<Weekday, time::error::InvalidVariant>) {
    assert_eq!(input.parse::<Weekday>(), expected);
}
