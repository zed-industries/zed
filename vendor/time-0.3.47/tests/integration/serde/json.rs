use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_test::Configure;
use time::macros::{date, datetime, time};
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, Weekday};

enum Format {
    Compact,
    Readable,
}

use Format::*;

fn serialize<T: Serialize>(val: T) -> Result<String, Box<dyn Error>> {
    let mut buf: Vec<u8> = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut buf);
    val.serialize(&mut ser)?;
    let str = String::from_utf8(buf)?;
    Ok(str)
}

fn deserialize<'a, T: Deserialize<'a>>(from: &'a str, fmt: Format) -> serde_json::Result<T> {
    let mut de = serde_json::Deserializer::from_str(from);
    match fmt {
        Compact => T::deserialize((&mut de).compact()),
        Readable => T::deserialize((&mut de).readable()),
    }
}

#[test]
fn weekday_json() -> Result<(), Box<dyn Error>> {
    use Weekday::*;

    assert_eq!(serialize(Monday.compact())?, "1");
    assert_eq!(deserialize::<Weekday>("1", Compact)?, Monday);

    assert_eq!(serialize(Monday.readable())?, "\"Monday\"");
    assert_eq!(deserialize::<Weekday>("\"Monday\"", Readable)?, Monday);
    assert_eq!(deserialize::<Weekday>("1", Readable)?, Monday);

    Ok(())
}

#[test]
fn month_json() -> Result<(), Box<dyn Error>> {
    use Month::*;

    assert_eq!(serialize(March.compact())?, "3");
    assert_eq!(deserialize::<Month>("3", Compact)?, March);

    assert_eq!(serialize(March.readable())?, "\"March\"");
    assert_eq!(deserialize::<Month>("\"March\"", Readable)?, March);
    assert_eq!(deserialize::<Month>("3", Readable)?, March);

    Ok(())
}

#[test]
fn time_json() -> Result<(), Box<dyn Error>> {
    let time = time!(12:40:20);

    assert_eq!(serialize(time.compact())?, "[12,40,20,0]");
    assert_eq!(deserialize::<Time>("[12,40,20,0]", Compact)?, time);

    assert_eq!(serialize(time.readable())?, "\"12:40:20.0\"");
    assert_eq!(deserialize::<Time>("\"12:40:20.0\"", Readable)?, time);
    assert_eq!(deserialize::<Time>("[12,40,20,0]", Readable)?, time);

    Ok(())
}

#[test]
fn primitive_datetime_json() -> Result<(), Box<dyn Error>> {
    let dt = datetime!(2022-05-20 12:40:20);

    assert_eq!(serialize(dt.compact())?, "[2022,140,12,40,20,0]");
    assert_eq!(
        deserialize::<PrimitiveDateTime>("[2022,140,12,40,20,0]", Compact)?,
        dt
    );

    assert_eq!(serialize(dt.readable())?, "\"2022-05-20 12:40:20.0\"");
    assert_eq!(
        deserialize::<PrimitiveDateTime>("\"2022-05-20 12:40:20.0\"", Readable)?,
        dt
    );
    assert_eq!(
        deserialize::<PrimitiveDateTime>("[2022,140,12,40,20,0]", Readable)?,
        dt
    );

    Ok(())
}

#[test]
fn offset_datetime_json() -> Result<(), Box<dyn Error>> {
    let dt = datetime!(2022-05-20 12:40:20).assume_utc();

    assert_eq!(serialize(dt.compact())?, "[2022,140,12,40,20,0,0,0,0]");
    assert_eq!(
        deserialize::<OffsetDateTime>("[2022,140,12,40,20,0,0,0,0]", Compact)?,
        dt
    );

    assert_eq!(
        serialize(dt.readable())?,
        "\"2022-05-20 12:40:20.0 +00:00:00\""
    );
    assert_eq!(
        deserialize::<OffsetDateTime>("\"2022-05-20 12:40:20.0 +00:00:00\"", Readable)?,
        dt
    );
    assert_eq!(
        deserialize::<OffsetDateTime>("[2022,140,12,40,20,0,0,0,0]", Readable)?,
        dt
    );

    Ok(())
}

#[test]
fn duration_json() -> Result<(), Box<dyn Error>> {
    let dur = Duration::new(50, 0);

    assert_eq!(serialize(dur.compact())?, "[50,0]");
    assert_eq!(deserialize::<Duration>("[50,0]", Compact)?, dur);

    assert_eq!(serialize(dur.readable())?, "\"50.000000000\"");
    assert_eq!(deserialize::<Duration>("\"50.000000000\"", Readable)?, dur);
    assert_eq!(deserialize::<Duration>("[50,0]", Readable)?, dur);

    Ok(())
}

#[test]
fn date_json() -> Result<(), Box<dyn Error>> {
    let date = date!(2022-04-05);

    assert_eq!(serialize(date.compact())?, "[2022,95]");
    assert_eq!(deserialize::<Date>("[2022,95]", Compact)?, date);

    assert_eq!(serialize(date.readable())?, "\"2022-04-05\"");
    assert_eq!(deserialize::<Date>("\"2022-04-05\"", Readable)?, date);
    assert_eq!(deserialize::<Date>("[2022,95]", Readable)?, date);

    Ok(())
}
