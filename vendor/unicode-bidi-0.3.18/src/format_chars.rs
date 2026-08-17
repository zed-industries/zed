// Copyright 2017 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Directional Formatting Characters
//!
//! <http://www.unicode.org/reports/tr9/#Directional_Formatting_Characters>

// == Implicit ==
/// ARABIC LETTER MARK
pub const ALM: char = '\u{061C}';
/// LEFT-TO-RIGHT MARK
pub const LRM: char = '\u{200E}';
/// RIGHT-TO-LEFT MARK
pub const RLM: char = '\u{200F}';

// == Explicit Isolates ==
/// LEFT‑TO‑RIGHT ISOLATE
pub const LRI: char = '\u{2066}';
/// RIGHT‑TO‑LEFT ISOLATE
pub const RLI: char = '\u{2067}';
/// FIRST STRONG ISOLATE
pub const FSI: char = '\u{2068}';
/// POP DIRECTIONAL ISOLATE
pub const PDI: char = '\u{2069}';

// == Explicit Embeddings and Overrides ==
/// LEFT-TO-RIGHT EMBEDDING
pub const LRE: char = '\u{202A}';
/// RIGHT-TO-LEFT EMBEDDING
pub const RLE: char = '\u{202B}';
/// POP DIRECTIONAL FORMATTING
pub const PDF: char = '\u{202C}';
/// LEFT-TO-RIGHT OVERRIDE
pub const LRO: char = '\u{202D}';
/// RIGHT-TO-LEFT OVERRIDE
pub const RLO: char = '\u{202E}';
