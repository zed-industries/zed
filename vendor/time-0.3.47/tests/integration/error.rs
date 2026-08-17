use std::error::Error as _;
use std::io;

use time::error::{
    ComponentRange, ConversionRange, DifferentVariant, Error, Format, IndeterminateOffset,
    InvalidFormatDescription, InvalidVariant, Parse, ParseFromDescription, TryFromParsed,
};
use time::macros::format_description;
use time::parsing::Parsed;
use time::{Date, Time, format_description};

macro_rules! assert_display_eq {
    ($a:expr, $b:expr $(,)?) => {
        assert_eq!($a.to_string(), $b.to_string())
    };
}

macro_rules! assert_dbg_reflexive {
    ($a:expr) => {
        assert_eq!(format!("{:?}", $a), format!("{:?}", $a))
    };
}

macro_rules! assert_source {
    ($err:expr,None $(,)?) => {
        assert!($err.source().is_none())
    };
    ($err:expr, $source:ty $(,)?) => {
        assert!($err.source().unwrap().is::<$source>())
    };
}

fn component_range() -> ComponentRange {
    Date::from_ordinal_date(0, 367).expect_err("367 is not a valid day")
}

fn insufficient_type_information() -> Format {
    Time::MIDNIGHT
        .format(format_description!("[year]"))
        .expect_err("missing date and UTC offset")
}

fn unexpected_trailing_characters() -> Parse {
    Time::parse("a", format_description!("")).expect_err("should fail to parse")
}

fn invalid_format_description() -> InvalidFormatDescription {
    format_description::parse("[").expect_err("format description is invalid")
}

fn io_error() -> io::Error {
    io::Error::last_os_error()
}

fn invalid_literal() -> ParseFromDescription {
    Parsed::parse_literal(b"a", b"b").expect_err("should fail to parse")
}

#[test]
fn debug() {
    assert_dbg_reflexive!(Parse::from(ParseFromDescription::InvalidComponent("a")));
    assert_dbg_reflexive!(invalid_format_description());
    assert_dbg_reflexive!(DifferentVariant);
    assert_dbg_reflexive!(InvalidVariant);
}

#[test]
fn display() {
    assert_display_eq!(ConversionRange, Error::from(ConversionRange));
    assert_display_eq!(component_range(), Error::from(component_range()));
    assert_display_eq!(component_range(), TryFromParsed::from(component_range()));
    assert_display_eq!(IndeterminateOffset, Error::from(IndeterminateOffset));
    assert_display_eq!(
        TryFromParsed::InsufficientInformation,
        Error::from(TryFromParsed::InsufficientInformation)
    );
    assert_display_eq!(
        insufficient_type_information(),
        Error::from(insufficient_type_information())
    );
    assert_display_eq!(
        Format::InvalidComponent("a"),
        Error::from(Format::InvalidComponent("a"))
    );
    assert_display_eq!(
        ParseFromDescription::InvalidComponent("a"),
        Error::from(Parse::from(ParseFromDescription::InvalidComponent("a")))
    );
    assert_display_eq!(invalid_literal(), Parse::from(invalid_literal()));
    assert_display_eq!(
        component_range(),
        Error::from(Parse::from(TryFromParsed::from(component_range())))
    );
    assert_display_eq!(
        ParseFromDescription::InvalidComponent("a"),
        Parse::from(ParseFromDescription::InvalidComponent("a"))
    );
    assert_display_eq!(
        component_range(),
        Parse::from(TryFromParsed::from(component_range()))
    );
    assert_display_eq!(
        unexpected_trailing_characters(),
        Error::from(unexpected_trailing_characters()),
    );
    assert_display_eq!(
        invalid_format_description(),
        Error::from(invalid_format_description())
    );
    assert_display_eq!(io_error(), Format::from(io_error()));
    assert_display_eq!(DifferentVariant, Error::from(DifferentVariant));
    assert_display_eq!(InvalidVariant, Error::from(InvalidVariant));
}

#[test]
fn source() {
    assert_source!(Error::from(ConversionRange), ConversionRange);
    assert_source!(Error::from(component_range()), ComponentRange);
    assert_source!(TryFromParsed::from(component_range()), ComponentRange);
    assert_source!(TryFromParsed::InsufficientInformation, None);
    assert_source!(insufficient_type_information(), None);
    assert_source!(Format::InvalidComponent("a"), None);
    assert_source!(Error::from(insufficient_type_information()), Format);
    assert_source!(Error::from(IndeterminateOffset), IndeterminateOffset);
    assert_source!(
        Parse::from(TryFromParsed::InsufficientInformation),
        TryFromParsed
    );
    assert_source!(
        Error::from(TryFromParsed::InsufficientInformation),
        TryFromParsed
    );
    assert_source!(
        Parse::from(ParseFromDescription::InvalidComponent("a")),
        ParseFromDescription
    );
    assert_source!(
        Error::from(ParseFromDescription::InvalidComponent("a")),
        ParseFromDescription
    );
    assert_source!(unexpected_trailing_characters(), ParseFromDescription);
    assert_source!(
        Error::from(unexpected_trailing_characters()),
        ParseFromDescription
    );
    assert_source!(
        Error::from(invalid_format_description()),
        InvalidFormatDescription
    );
    assert_source!(Format::from(io_error()), io::Error);
    assert_source!(Error::from(DifferentVariant), DifferentVariant);
    assert_source!(Error::from(InvalidVariant), InvalidVariant);
}

#[test]
fn component_name() {
    assert_eq!(component_range().name(), "ordinal");
}

#[expect(clippy::cognitive_complexity, reason = "all test the same thing")]
#[test]
fn conversion() {
    assert!(ComponentRange::try_from(Error::from(component_range())).is_ok());
    assert!(ConversionRange::try_from(Error::from(ConversionRange)).is_ok());
    assert!(Format::try_from(Error::from(insufficient_type_information())).is_ok());
    assert!(IndeterminateOffset::try_from(Error::from(IndeterminateOffset)).is_ok());
    assert!(InvalidFormatDescription::try_from(Error::from(invalid_format_description())).is_ok());
    assert!(ParseFromDescription::try_from(Error::from(invalid_literal())).is_ok());
    assert!(ParseFromDescription::try_from(Parse::from(invalid_literal())).is_ok());
    assert!(ParseFromDescription::try_from(unexpected_trailing_characters()).is_ok());
    assert!(Parse::try_from(Error::from(unexpected_trailing_characters())).is_ok());
    assert!(Parse::try_from(Error::from(invalid_literal())).is_ok());
    assert!(Parse::try_from(Error::from(TryFromParsed::InsufficientInformation)).is_ok());
    assert!(DifferentVariant::try_from(Error::from(DifferentVariant)).is_ok());
    assert!(InvalidVariant::try_from(Error::from(InvalidVariant)).is_ok());
    assert!(ComponentRange::try_from(TryFromParsed::ComponentRange(component_range())).is_ok());
    assert!(TryFromParsed::try_from(Error::from(TryFromParsed::InsufficientInformation)).is_ok());
    assert!(TryFromParsed::try_from(Parse::from(TryFromParsed::InsufficientInformation)).is_ok());
    assert!(io::Error::try_from(Format::from(io_error())).is_ok());

    assert!(ComponentRange::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(ConversionRange::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(Format::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(IndeterminateOffset::try_from(Error::from(ConversionRange)).is_err());
    assert!(InvalidFormatDescription::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(ParseFromDescription::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(Parse::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(DifferentVariant::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(InvalidVariant::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(ComponentRange::try_from(TryFromParsed::InsufficientInformation).is_err());
    assert!(TryFromParsed::try_from(Error::from(IndeterminateOffset)).is_err());
    assert!(TryFromParsed::try_from(unexpected_trailing_characters()).is_err());
    assert!(io::Error::try_from(insufficient_type_information()).is_err());
}
