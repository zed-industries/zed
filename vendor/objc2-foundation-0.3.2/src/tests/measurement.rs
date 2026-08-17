#![cfg(all(feature = "NSMeasurement", feature = "NSUnit"))]

use crate::{NSMeasurement, NSUnitMass, NSUnitPower};

#[test]
fn create() {
    let mass = unsafe { NSMeasurement::<NSUnitMass>::new() };
    let _power = unsafe { mass.cast_unchecked::<NSUnitPower>() };
}
