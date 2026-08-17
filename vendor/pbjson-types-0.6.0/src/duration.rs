use crate::Duration;
use serde::de::Visitor;
use serde::Serialize;

impl TryFrom<Duration> for std::time::Duration {
    type Error = std::num::TryFromIntError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.seconds.try_into()?,
            value.nanos.try_into()?,
        ))
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self {
            seconds: value.as_secs() as _,
            nanos: value.subsec_nanos() as _,
        }
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.seconds != 0 && self.nanos != 0 && (self.nanos < 0) != (self.seconds < 0) {
            return Err(serde::ser::Error::custom("Duration has inconsistent signs"));
        }

        let mut s = if self.seconds == 0 {
            if self.nanos < 0 {
                "-0".to_string()
            } else {
                "0".to_string()
            }
        } else {
            self.seconds.to_string()
        };

        if self.nanos != 0 {
            s.push('.');
            let f = match split_nanos(self.nanos.unsigned_abs()) {
                (millis, 0, 0) => format!("{:03}", millis),
                (millis, micros, 0) => format!("{:03}{:03}", millis, micros),
                (millis, micros, nanos) => format!("{:03}{:03}{:03}", millis, micros, nanos),
            };
            s.push_str(&f);
        }

        s.push('s');
        serializer.serialize_str(&s)
    }
}

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a duration string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let s = s
            .strip_suffix('s')
            .ok_or_else(|| serde::de::Error::custom("missing 's' suffix"))?;

        let (negative, s) = match s.strip_prefix('-') {
            Some(s) => (true, s),
            None => (false, s),
        };

        let duration = match s.split_once('.') {
            Some((seconds_str, decimal_str)) => {
                let exp = 9_u32
                    .checked_sub(decimal_str.len() as u32)
                    .ok_or_else(|| serde::de::Error::custom("too many decimal places"))?;

                let pow = 10_u32.pow(exp);
                let seconds = seconds_str.parse().map_err(serde::de::Error::custom)?;
                let decimal: u32 = decimal_str.parse().map_err(serde::de::Error::custom)?;

                Duration {
                    seconds,
                    nanos: (decimal * pow) as i32,
                }
            }
            None => Duration {
                seconds: s.parse().map_err(serde::de::Error::custom)?,
                nanos: 0,
            },
        };

        Ok(match negative {
            true => Duration {
                seconds: -duration.seconds,
                nanos: -duration.nanos,
            },
            false => duration,
        })
    }
}

impl<'de> serde::Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DurationVisitor)
    }
}

/// Splits nanoseconds into whole milliseconds, microseconds, and nanoseconds
fn split_nanos(mut nanos: u32) -> (u32, u32, u32) {
    let millis = nanos / 1_000_000;
    nanos -= millis * 1_000_000;
    let micros = nanos / 1_000;
    nanos -= micros * 1_000;
    (millis, micros, nanos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        let verify = |duration: &Duration, expected: &str| {
            assert_eq!(serde_json::to_string(duration).unwrap().as_str(), expected);
            assert_eq!(
                &serde_json::from_str::<Duration>(expected).unwrap(),
                duration
            )
        };

        let duration = Duration {
            seconds: 0,
            nanos: 0,
        };
        verify(&duration, "\"0s\"");

        let duration = Duration {
            seconds: 0,
            nanos: 123,
        };
        verify(&duration, "\"0.000000123s\"");

        let duration = Duration {
            seconds: 0,
            nanos: 123456,
        };
        verify(&duration, "\"0.000123456s\"");

        let duration = Duration {
            seconds: 0,
            nanos: 123456789,
        };
        verify(&duration, "\"0.123456789s\"");

        let duration = Duration {
            seconds: 0,
            nanos: -67088,
        };
        verify(&duration, "\"-0.000067088s\"");

        let duration = Duration {
            seconds: 121,
            nanos: 3454,
        };
        verify(&duration, "\"121.000003454s\"");

        let duration = Duration {
            seconds: -90,
            nanos: -2456301,
        };
        verify(&duration, "\"-90.002456301s\"");

        let duration = Duration {
            seconds: -90,
            nanos: 234,
        };
        serde_json::to_string(&duration).unwrap_err();

        let duration = Duration {
            seconds: 90,
            nanos: -234,
        };
        serde_json::to_string(&duration).unwrap_err();

        serde_json::from_str::<Duration>("90.1234567891s").unwrap_err();
    }
}
