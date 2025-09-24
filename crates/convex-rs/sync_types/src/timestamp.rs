use std::{
    fmt,
    ops::Sub,
    time::{
        Duration,
        SystemTime,
    },
};

use anyhow::Context;
use derive_more::FromStr;
use serde::Serialize;
use serde_json::json;

/// Database transaction timestamp.
/// This is unique across all transactions.
/// Units are nanoseconds since epoch.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, FromStr, Hash, Serialize, Default)]
pub struct Timestamp(u64);

impl Timestamp {
    // Some SQL and serialization don't support timestamps > i64::MAX,
    // which is fine to use as an upper bound because real timestamps aren't that
    // high.
    pub const MAX: Self = Self(i64::MAX as u64);
    pub const MIN: Self = Self(0);

    pub fn succ(&self) -> anyhow::Result<Self> {
        if *self >= Self::MAX {
            anyhow::bail!("timestamp {self} already at max");
        }
        Ok(Self(self.0 + 1))
    }

    pub fn pred(&self) -> anyhow::Result<Self> {
        if *self <= Self::MIN {
            anyhow::bail!("timestamp {self} already at min");
        }
        Ok(Self(self.0 - 1))
    }

    pub fn add(&self, duration: Duration) -> anyhow::Result<Self> {
        let nanos = self
            .0
            .checked_add(duration.as_nanos() as u64)
            .with_context(|| format!("timestamp {self} + {duration:?} overflow u64"))?;

        anyhow::ensure!(
            nanos <= u64::from(Self::MAX),
            "timestamp {self} + {duration:?} overflow i64"
        );

        Ok(Self(nanos))
    }

    pub fn sub(&self, duration: Duration) -> anyhow::Result<Self> {
        let nanos = duration.as_nanos() as u64;
        if self.0 <= nanos {
            anyhow::bail!("timestamp {self} already greater than {duration:?}");
        }
        Ok(Self(self.0 - nanos))
    }

    // This is similar to `self - base` but it works if `self` is before `base`.
    // Since Duration is always positive, `self - base` can overflow.
    pub fn secs_since_f64(self, base: Timestamp) -> f64 {
        if self >= base {
            (self - base).as_secs_f64()
        } else {
            -(base - self).as_secs_f64()
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn must(value: i32) -> Self {
        if value < Self::MIN.0 as i32 || value as u64 > Self::MAX.0 {
            panic!("timestamp {value} out of bounds");
        }
        Self(value as u64)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Timestamp> for u64 {
    fn from(ts: Timestamp) -> Self {
        ts.0
    }
}

impl From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> Self {
        // This cast is safe because Timestamp checks bounds on construction.
        ts.0 as i64
    }
}

impl TryFrom<i64> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Timestamp(u64::try_from(value)?))
    }
}

impl TryFrom<u64> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > Self::MAX.0 {
            anyhow::bail!("ts {value} too large");
        }
        Ok(Timestamp(value))
    }
}

impl TryFrom<SystemTime> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: SystemTime) -> Result<Self, Self::Error> {
        let system_ns: u64 = value
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("SystemTime before 1970")?
            .as_nanos()
            .try_into()
            .context("SystemTime past 2262")?;
        Self::try_from(system_ns)
    }
}

impl From<Timestamp> for SystemTime {
    fn from(ts: Timestamp) -> Self {
        SystemTime::UNIX_EPOCH + Duration::from_nanos(ts.0)
    }
}

impl TryFrom<serde_json::Value> for Timestamp {
    type Error = anyhow::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let ts = value
            .as_i64()
            .ok_or_else(|| anyhow::anyhow!("value is not timestamp"))?;
        Timestamp::try_from(ts)
    }
}

impl From<Timestamp> for serde_json::Value {
    fn from(ts: Timestamp) -> Self {
        json!(i64::from(ts))
    }
}

#[cfg(any(test, feature = "testing"))]
impl proptest::arbitrary::Arbitrary for Timestamp {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;
        (Timestamp::MIN.0..=Timestamp::MAX.0)
            .prop_map(Timestamp)
            .boxed()
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration::from_nanos(self.0 - rhs.0)
    }
}

#[test]
fn test_secs_since_f64_positive_zero() {
    let ts = Timestamp::must(1234);
    let zero = ts.secs_since_f64(ts);
    // should be positive zero, not negative zero
    assert!(zero.total_cmp(&0.0).is_eq(), "{zero:?}");
}
