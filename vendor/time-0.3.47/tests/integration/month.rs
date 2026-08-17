use rstest::rstest;
use time::Month::{self, *};

#[rstest]
#[case(January, December)]
#[case(February, January)]
#[case(March, February)]
#[case(April, March)]
#[case(May, April)]
#[case(June, May)]
#[case(July, June)]
#[case(August, July)]
#[case(September, August)]
#[case(October, September)]
#[case(November, October)]
#[case(December, November)]
fn previous(#[case] month: Month, #[case] expected: Month) {
    assert_eq!(month.previous(), expected);
}

#[rstest]
#[case(January, February)]
#[case(February, March)]
#[case(March, April)]
#[case(April, May)]
#[case(May, June)]
#[case(June, July)]
#[case(July, August)]
#[case(August, September)]
#[case(September, October)]
#[case(October, November)]
#[case(November, December)]
#[case(December, January)]
fn next(#[case] month: Month, #[case] expected: Month) {
    assert_eq!(month.next(), expected);
}

#[rstest]
#[case(January, 0, January)]
#[case(January, 1, February)]
#[case(January, 2, March)]
#[case(January, 3, April)]
#[case(January, 4, May)]
#[case(January, 5, June)]
#[case(January, 6, July)]
#[case(January, 7, August)]
#[case(January, 8, September)]
#[case(January, 9, October)]
#[case(January, 10, November)]
#[case(January, 11, December)]
#[case(December, 0, December)]
#[case(December, 1, January)]
#[case(December, 2, February)]
#[case(December, 3, March)]
#[case(December, 4, April)]
#[case(December, 5, May)]
#[case(December, 6, June)]
#[case(December, 7, July)]
#[case(December, 8, August)]
#[case(December, 9, September)]
#[case(December, 10, October)]
#[case(December, 11, November)]
#[case(January, 12, January)]
#[case(January, u8::MAX, April)]
#[case(December, 12, December)]
#[case(December, u8::MAX, March)]
fn nth_next(#[case] month: Month, #[case] n: u8, #[case] expected: Month) {
    assert_eq!(month.nth_next(n), expected);
}

#[rstest]
#[case(January, 0, January)]
#[case(January, 1, December)]
#[case(January, 2, November)]
#[case(January, 3, October)]
#[case(January, 4, September)]
#[case(January, 5, August)]
#[case(January, 6, July)]
#[case(January, 7, June)]
#[case(January, 8, May)]
#[case(January, 9, April)]
#[case(January, 10, March)]
#[case(January, 11, February)]
#[case(December, 0, December)]
#[case(December, 1, November)]
#[case(December, 2, October)]
#[case(December, 3, September)]
#[case(December, 4, August)]
#[case(December, 5, July)]
#[case(December, 6, June)]
#[case(December, 7, May)]
#[case(December, 8, April)]
#[case(December, 9, March)]
#[case(December, 10, February)]
#[case(December, 11, January)]
#[case(January, 12, January)]
#[case(January, u8::MAX, October)]
#[case(December, 12, December)]
#[case(December, u8::MAX, September)]
fn nth_prev(#[case] month: Month, #[case] n: u8, #[case] expected: Month) {
    assert_eq!(month.nth_prev(n), expected);
}

#[rstest]
#[case(January, "January")]
#[case(February, "February")]
#[case(March, "March")]
#[case(April, "April")]
#[case(May, "May")]
#[case(June, "June")]
#[case(July, "July")]
#[case(August, "August")]
#[case(September, "September")]
#[case(October, "October")]
#[case(November, "November")]
#[case(December, "December")]
fn display(#[case] month: Month, #[case] expected: &str) {
    assert_eq!(month.to_string(), expected);
}

#[rstest]
#[case("January", Ok(January))]
#[case("February", Ok(February))]
#[case("March", Ok(March))]
#[case("April", Ok(April))]
#[case("May", Ok(May))]
#[case("June", Ok(June))]
#[case("July", Ok(July))]
#[case("August", Ok(August))]
#[case("September", Ok(September))]
#[case("October", Ok(October))]
#[case("November", Ok(November))]
#[case("December", Ok(December))]
#[case("foo", Err(time::error::InvalidVariant))]
fn from_str(#[case] s: &str, #[case] expected: Result<Month, time::error::InvalidVariant>) {
    assert_eq!(s.parse::<Month>(), expected);
}

#[rstest]
#[case(January, 1)]
#[case(February, 2)]
#[case(March, 3)]
#[case(April, 4)]
#[case(May, 5)]
#[case(June, 6)]
#[case(July, 7)]
#[case(August, 8)]
#[case(September, 9)]
#[case(October, 10)]
#[case(November, 11)]
#[case(December, 12)]
fn to_u8(#[case] month: Month, #[case] expected: u8) {
    assert_eq!(u8::from(month), expected);
}

#[rstest]
#[case(1, January)]
#[case(2, February)]
#[case(3, March)]
#[case(4, April)]
#[case(5, May)]
#[case(6, June)]
#[case(7, July)]
#[case(8, August)]
#[case(9, September)]
#[case(10, October)]
#[case(11, November)]
#[case(12, December)]
fn try_from_u8_success(#[case] input: u8, #[case] expected: Month) {
    assert_eq!(Month::try_from(input), Ok(expected));
}

#[rstest]
#[case(0)]
#[case(13)]
fn try_from_u8_error(#[case] input: u8) {
    assert!(matches!(Month::try_from(input), Err(err) if err.name() == "month"));
}
