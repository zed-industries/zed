// Copyright 2015 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Accessor for `Bidi_Class` property from Unicode Character Database (UCD)

mod tables;

pub use self::tables::{BidiClass, UNICODE_VERSION};
#[cfg(feature = "hardcoded-data")]
use core::char;
#[cfg(feature = "hardcoded-data")]
use core::cmp::Ordering::{Equal, Greater, Less};

#[cfg(feature = "hardcoded-data")]
use self::tables::bidi_class_table;
use crate::data_source::BidiMatchedOpeningBracket;
use crate::BidiClass::*;
#[cfg(feature = "hardcoded-data")]
use crate::BidiDataSource;
/// Hardcoded Bidi data that ships with the unicode-bidi crate.
///
/// This can be enabled with the default `hardcoded-data` Cargo feature.
#[cfg(feature = "hardcoded-data")]
pub struct HardcodedBidiData;

#[cfg(feature = "hardcoded-data")]
impl BidiDataSource for HardcodedBidiData {
    fn bidi_class(&self, c: char) -> BidiClass {
        bsearch_range_value_table(c, bidi_class_table)
    }
}

/// Find the `BidiClass` of a single char.
#[cfg(feature = "hardcoded-data")]
pub fn bidi_class(c: char) -> BidiClass {
    bsearch_range_value_table(c, bidi_class_table)
}

/// If this character is a bracket according to BidiBrackets.txt,
/// return the corresponding *normalized* *opening bracket* of the pair,
/// and whether or not it itself is an opening bracket.
pub(crate) fn bidi_matched_opening_bracket(c: char) -> Option<BidiMatchedOpeningBracket> {
    for pair in self::tables::bidi_pairs_table {
        if pair.0 == c || pair.1 == c {
            let skeleton = pair.2.unwrap_or(pair.0);
            return Some(BidiMatchedOpeningBracket {
                opening: skeleton,
                is_open: pair.0 == c,
            });
        }
    }
    None
}

pub fn is_rtl(bidi_class: BidiClass) -> bool {
    matches!(bidi_class, RLE | RLO | RLI)
}

#[cfg(feature = "hardcoded-data")]
fn bsearch_range_value_table(c: char, r: &'static [(char, char, BidiClass)]) -> BidiClass {
    match r.binary_search_by(|&(lo, hi, _)| {
        if lo <= c && c <= hi {
            Equal
        } else if hi < c {
            Less
        } else {
            Greater
        }
    }) {
        Ok(idx) => {
            let (_, _, cat) = r[idx];
            cat
        }
        // UCD/extracted/DerivedBidiClass.txt: "All code points not explicitly listed
        // for Bidi_Class have the value Left_To_Right (L)."
        Err(_) => L,
    }
}

#[cfg(all(test, feature = "hardcoded-data"))]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        assert_eq!(bidi_class('\u{0000}'), BN);
        assert_eq!(bidi_class('\u{0040}'), ON);
        assert_eq!(bidi_class('\u{0041}'), L);
        assert_eq!(bidi_class('\u{0062}'), L);
        assert_eq!(bidi_class('\u{007F}'), BN);
    }

    #[test]
    fn test_bmp() {
        // Hebrew
        assert_eq!(bidi_class('\u{0590}'), R);
        assert_eq!(bidi_class('\u{05D0}'), R);
        assert_eq!(bidi_class('\u{05D1}'), R);
        assert_eq!(bidi_class('\u{05FF}'), R);

        // Arabic
        assert_eq!(bidi_class('\u{0600}'), AN);
        assert_eq!(bidi_class('\u{0627}'), AL);
        assert_eq!(bidi_class('\u{07BF}'), AL);

        // Default R + Arabic Extras
        assert_eq!(bidi_class('\u{07C0}'), R);
        assert_eq!(bidi_class('\u{085F}'), R);
        assert_eq!(bidi_class('\u{0860}'), AL);
        assert_eq!(bidi_class('\u{0870}'), AL);
        assert_eq!(bidi_class('\u{089F}'), NSM);
        assert_eq!(bidi_class('\u{08A0}'), AL);
        assert_eq!(bidi_class('\u{089F}'), NSM);
        assert_eq!(bidi_class('\u{08FF}'), NSM);

        // Default ET
        assert_eq!(bidi_class('\u{20A0}'), ET);
        assert_eq!(bidi_class('\u{20CF}'), ET);

        // Arabic Presentation Forms
        assert_eq!(bidi_class('\u{FB1D}'), R);
        assert_eq!(bidi_class('\u{FB4F}'), R);
        assert_eq!(bidi_class('\u{FB50}'), AL);
        assert_eq!(bidi_class('\u{FDCF}'), ON);
        assert_eq!(bidi_class('\u{FDF0}'), AL);
        assert_eq!(bidi_class('\u{FDFF}'), ON);
        assert_eq!(bidi_class('\u{FE70}'), AL);
        assert_eq!(bidi_class('\u{FEFE}'), AL);
        assert_eq!(bidi_class('\u{FEFF}'), BN);

        // noncharacters
        assert_eq!(bidi_class('\u{FDD0}'), L);
        assert_eq!(bidi_class('\u{FDD1}'), L);
        assert_eq!(bidi_class('\u{FDEE}'), L);
        assert_eq!(bidi_class('\u{FDEF}'), L);
        assert_eq!(bidi_class('\u{FFFE}'), L);
        assert_eq!(bidi_class('\u{FFFF}'), L);
    }

    #[test]
    fn test_smp() {
        // Default AL + R
        assert_eq!(bidi_class('\u{10800}'), R);
        assert_eq!(bidi_class('\u{10FFF}'), R);
        assert_eq!(bidi_class('\u{1E800}'), R);
        assert_eq!(bidi_class('\u{1EDFF}'), R);
        assert_eq!(bidi_class('\u{1EE00}'), AL);
        assert_eq!(bidi_class('\u{1EEFF}'), AL);
        assert_eq!(bidi_class('\u{1EF00}'), R);
        assert_eq!(bidi_class('\u{1EFFF}'), R);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(bidi_class('\u{30000}'), L);
        assert_eq!(bidi_class('\u{40000}'), L);
        assert_eq!(bidi_class('\u{50000}'), L);
        assert_eq!(bidi_class('\u{60000}'), L);
        assert_eq!(bidi_class('\u{70000}'), L);
        assert_eq!(bidi_class('\u{80000}'), L);
        assert_eq!(bidi_class('\u{90000}'), L);
        assert_eq!(bidi_class('\u{a0000}'), L);
    }
}
