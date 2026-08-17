use std::convert::TryFrom;
use std::fmt;

use crate::PgInterval;

impl fmt::Display for PgInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fields = match self {
            PgInterval::Year => "YEAR",
            PgInterval::Month => "MONTH",
            PgInterval::Day => "DAY",
            PgInterval::Hour => "HOUR",
            PgInterval::Minute => "MINUTE",
            PgInterval::Second => "SECOND",
            PgInterval::YearToMonth => "YEAR TO MONTH",
            PgInterval::DayToHour => "DAY TO HOUR",
            PgInterval::DayToMinute => "DAY TO MINUTE",
            PgInterval::DayToSecond => "DAY TO SECOND",
            PgInterval::HourToMinute => "HOUR TO MINUTE",
            PgInterval::HourToSecond => "HOUR TO SECOND",
            PgInterval::MinuteToSecond => "MINUTE TO SECOND",
        };
        write!(f, "{fields}")
    }
}

impl TryFrom<String> for PgInterval {
    type Error = String;

    fn try_from(field: String) -> Result<Self, Self::Error> {
        PgInterval::try_from(field.as_str())
    }
}

impl TryFrom<&String> for PgInterval {
    type Error = String;

    fn try_from(field: &String) -> Result<Self, Self::Error> {
        PgInterval::try_from(field.as_str())
    }
}

impl TryFrom<&str> for PgInterval {
    type Error = String;

    fn try_from(field: &str) -> Result<Self, Self::Error> {
        match field.trim_start().trim_end().to_uppercase().as_ref() {
            "YEAR" => Ok(PgInterval::Year),
            "MONTH" => Ok(PgInterval::Month),
            "DAY" => Ok(PgInterval::Day),
            "HOUR" => Ok(PgInterval::Hour),
            "MINUTE" => Ok(PgInterval::Minute),
            "SECOND" => Ok(PgInterval::Second),
            "YEAR TO MONTH" => Ok(PgInterval::YearToMonth),
            "DAY TO HOUR" => Ok(PgInterval::DayToHour),
            "DAY TO MINUTE" => Ok(PgInterval::DayToMinute),
            "DAY TO SECOND" => Ok(PgInterval::DayToSecond),
            "HOUR TO MINUTE" => Ok(PgInterval::HourToMinute),
            "HOUR TO SECOND" => Ok(PgInterval::HourToSecond),
            "MINUTE TO SECOND" => Ok(PgInterval::MinuteToSecond),
            field => Err(format!(
                "Cannot turn \"{field}\" into a Postgres interval field",
            )),
        }
    }
}
