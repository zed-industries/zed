extern crate rkyv_0_8 as rkyv;

use rkyv::{rancor::Error, Archive, Deserialize, Serialize};
use rust_decimal::prelude::{dec, Decimal};

/// The type containing a [`Decimal`] that will be de/serialized.
#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Root {
    #[rkyv(with = RkyvDecimal)]
    decimal: Decimal,
}

/// Archived layout of [`Decimal`]
#[derive(Archive, Serialize, Deserialize)]
#[rkyv(remote = Decimal)]
struct RkyvDecimal {
    #[rkyv(getter = Decimal::serialize)]
    bytes: [u8; 16],
}

impl From<RkyvDecimal> for Decimal {
    fn from(RkyvDecimal { bytes }: RkyvDecimal) -> Self {
        Self::deserialize(bytes)
    }
}

fn main() {
    let test_value = Root { decimal: dec!(123.456) };

    let bytes = rkyv::to_bytes::<Error>(&test_value).expect("Failed to serialize");

    let roundtrip_value = rkyv::from_bytes::<Root, Error>(&bytes).expect("Failed to deserialize");

    assert_eq!(test_value, roundtrip_value);
}
