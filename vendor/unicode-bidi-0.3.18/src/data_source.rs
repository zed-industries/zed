// Copyright 2015 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::BidiClass;

/// This is the return value of [`BidiDataSource::bidi_matched_opening_bracket()`].
///
/// It represents the matching *normalized* opening bracket for a given bracket in a bracket pair,
/// and whether or not that bracket is opening.
#[derive(Debug, Copy, Clone)]
pub struct BidiMatchedOpeningBracket {
    /// The corresponding opening bracket in this bracket pair, normalized
    ///
    /// In case of opening brackets, this will be the bracket itself, except for when the bracket
    /// is not normalized, in which case it will be the normalized form.
    pub opening: char,
    /// Whether or not the requested bracket was an opening bracket. True for opening
    pub is_open: bool,
}

/// This trait abstracts over a data source that is able to produce the Unicode Bidi class for a given
/// character
pub trait BidiDataSource {
    fn bidi_class(&self, c: char) -> BidiClass;
    /// If this character is a bracket according to BidiBrackets.txt,
    /// return the corresponding *normalized* *opening bracket* of the pair,
    /// and whether or not it itself is an opening bracket.
    ///
    /// This effectively buckets brackets into equivalence classes keyed on the
    /// normalized opening bracket.
    ///
    /// The default implementation will pull in a small amount of hardcoded data,
    /// regardless of the `hardcoded-data` feature. This is in part for convenience
    /// (since this data is small and changes less often), and in part so that this method can be
    /// added without needing a breaking version bump.
    /// Override this method in your custom data source to prevent the use of hardcoded data.
    fn bidi_matched_opening_bracket(&self, c: char) -> Option<BidiMatchedOpeningBracket> {
        crate::char_data::bidi_matched_opening_bracket(c)
    }
}
