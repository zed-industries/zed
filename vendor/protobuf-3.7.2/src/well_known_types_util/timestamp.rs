use std::time::Duration;
use std::time::SystemTime;

use crate::well_known_types::timestamp::Timestamp;
use crate::SpecialFields;

impl Timestamp {
    /// Unix epoch value of timestamp.
    pub const UNIX_EPOCH: Timestamp = Timestamp {
        seconds: 0,
        nanos: 0,
        special_fields: SpecialFields::new(),
    };

    /// Return current time as `Timestamp`.
    pub fn now() -> Timestamp {
        Timestamp::from(SystemTime::now())
    }
}

/// Convert from [`Timestamp`].
///
/// # Panics
///
/// This function panics if given `SystemTime` is outside of `Timestamp` range.
impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        match time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(since_epoch) => Timestamp {
                seconds: since_epoch.as_secs() as i64,
                nanos: since_epoch.subsec_nanos() as i32,
                ..Default::default()
            },
            Err(e) => {
                let before_epoch = e.duration();
                Timestamp {
                    seconds: -(before_epoch.as_secs() as i64)
                        - (before_epoch.subsec_nanos() != 0) as i64,
                    nanos: (1_000_000_000 - before_epoch.subsec_nanos() as i32) % 1_000_000_000,
                    ..Default::default()
                }
            }
        }
    }
}

/// Convert into [`SystemTime`].
///
/// The conversion could be lossy if `SystemTime` precision is smaller than nanoseconds.
///
/// # Panics
///
/// This function panics:
/// * if given `Timestamp` is outside of `SystemTime` range
/// * if `Timestamp` is malformed
impl Into<SystemTime> for Timestamp {
    fn into(self) -> SystemTime {
        if self.seconds >= 0 {
            let duration =
                Duration::from_secs(self.seconds as u64) + Duration::from_nanos(self.nanos as u64);
            SystemTime::UNIX_EPOCH + duration
        } else {
            let duration =
                Duration::from_secs(-self.seconds as u64) - Duration::from_nanos(self.nanos as u64);
            SystemTime::UNIX_EPOCH - duration
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use std::time::SystemTime;

    use crate::well_known_types::timestamp::Timestamp;

    #[test]
    fn to_from_system_time() {
        fn to_from(timestamp: Timestamp, system_time: SystemTime) {
            assert_eq!(timestamp, Timestamp::from(system_time));
            assert_eq!(system_time, Into::<SystemTime>::into(timestamp));
        }

        to_from(Timestamp::UNIX_EPOCH, SystemTime::UNIX_EPOCH);
        to_from(
            Timestamp {
                seconds: 0,
                nanos: 200_000_000,
                ..Default::default()
            },
            SystemTime::UNIX_EPOCH + Duration::from_millis(200),
        );
        to_from(
            Timestamp {
                seconds: 3,
                nanos: 200_000_000,
                ..Default::default()
            },
            SystemTime::UNIX_EPOCH + Duration::from_millis(3_200),
        );
        to_from(
            Timestamp {
                seconds: -1,
                nanos: 800_000_000,
                ..Default::default()
            },
            SystemTime::UNIX_EPOCH - Duration::from_millis(200),
        );
        to_from(
            Timestamp {
                seconds: -4,
                nanos: 800_000_000,
                ..Default::default()
            },
            SystemTime::UNIX_EPOCH - Duration::from_millis(3_200),
        );
    }
}
