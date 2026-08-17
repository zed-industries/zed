// Copyright Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! `chardetng` is a character encoding detector for legacy Web content.
//!
//! It is optimized for binary size in applications that already depend
//! on `encoding_rs` for other reasons.

#![no_std]

#[cfg(feature = "multithreading")]
use rayon::prelude::*;

#[cfg(feature = "multithreading")]
use arrayvec::ArrayVec;

#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
use core::arch::x86::__m128i;
#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
use core::arch::x86::_mm_movemask_epi8;

#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::arch::x86_64::__m128i;
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
use core::arch::x86_64::_mm_movemask_epi8;

use encoding_rs::Decoder;
use encoding_rs::DecoderResult;
use encoding_rs::Encoding;
use encoding_rs::BIG5;
use encoding_rs::EUC_JP;
use encoding_rs::EUC_KR;
use encoding_rs::GBK;
use encoding_rs::ISO_2022_JP;
use encoding_rs::ISO_8859_8;
use encoding_rs::SHIFT_JIS;
use encoding_rs::UTF_8;
use encoding_rs::WINDOWS_1255;

mod data;
mod tld;
use data::*;
use tld::classify_tld;
use tld::Tld;

const LATIN_ADJACENCY_PENALTY: i64 = -50;

const IMPLAUSIBILITY_PENALTY: i64 = -220;

const ORDINAL_BONUS: i64 = 300;

/// Must match the ISO-8859-2 score for " Š ". Note: There
/// are four Slovenian Wikipedia list page titles where the
/// list is split by letter so that Š stands alone for the
/// list part for Š. Let's assume that's a special case not
/// worth detecting even though the copyright sign detection
/// makes Slovenian title detection round to one percentage
/// point worse.
const COPYRIGHT_BONUS: i64 = 222;

const IMPLAUSIBLE_LATIN_CASE_TRANSITION_PENALTY: i64 = -180;

const NON_LATIN_CAPITALIZATION_BONUS: i64 = 40;

const NON_LATIN_ALL_CAPS_PENALTY: i64 = -40;

const NON_LATIN_MIXED_CASE_PENALTY: i64 = -20;

// Manually calibrated relative to windows-1256 Arabic
const CJK_BASE_SCORE: i64 = 41;

const CJK_SECONDARY_BASE_SCORE: i64 = 20; // Was 20

const SHIFT_JIS_SCORE_PER_KANA: i64 = 20;

const SHIFT_JIS_SCORE_PER_LEVEL_1_KANJI: i64 = CJK_BASE_SCORE;

const SHIFT_JIS_SCORE_PER_LEVEL_2_KANJI: i64 = CJK_SECONDARY_BASE_SCORE;

// Manually calibrated relative to windows-1256 Persian and Urdu
const SHIFT_JIS_INITIAL_HALF_WIDTH_KATAKANA_PENALTY: i64 = -75;

const HALF_WIDTH_KATAKANA_SCORE: i64 = 1;

// Unclear if this is a good idea; seems not harmful, but can't be sure.
const HALF_WIDTH_KATAKANA_VOICING_SCORE: i64 = 10;

const SHIFT_JIS_PUA_PENALTY: i64 = -(CJK_BASE_SCORE * 10); // Should this be larger?

const SHIFT_JIS_EXTENSION_PENALTY: i64 = SHIFT_JIS_PUA_PENALTY * 2;

const SHIFT_JIS_SINGLE_BYTE_EXTENSION_PENALTY: i64 = SHIFT_JIS_EXTENSION_PENALTY;

const EUC_JP_SCORE_PER_KANA: i64 = CJK_BASE_SCORE + (CJK_BASE_SCORE / 3); // Relative to Big5

const EUC_JP_SCORE_PER_NEAR_OBSOLETE_KANA: i64 = CJK_BASE_SCORE - 1;

const EUC_JP_SCORE_PER_LEVEL_1_KANJI: i64 = CJK_BASE_SCORE;

const EUC_JP_SCORE_PER_LEVEL_2_KANJI: i64 = CJK_SECONDARY_BASE_SCORE;

const EUC_JP_SCORE_PER_OTHER_KANJI: i64 = CJK_SECONDARY_BASE_SCORE / 4;

const EUC_JP_INITIAL_KANA_PENALTY: i64 = -((CJK_BASE_SCORE / 3) + 1);

const EUC_JP_EXTENSION_PENALTY: i64 = -(CJK_BASE_SCORE * 50); // Needs to be more severe than for Shift_JIS to avoid misdetecting EUC-KR!

const BIG5_SCORE_PER_LEVEL_1_HANZI: i64 = CJK_BASE_SCORE;

const BIG5_SCORE_PER_OTHER_HANZI: i64 = CJK_SECONDARY_BASE_SCORE;

const BIG5_PUA_PENALTY: i64 = -(CJK_BASE_SCORE * 30); // More severe than other PUA penalties to avoid misdetecting EUC-KR! (25 as the multiplier is too little)

const BIG5_SINGLE_BYTE_EXTENSION_PENALTY: i64 = -(CJK_BASE_SCORE * 40);

const EUC_KR_SCORE_PER_EUC_HANGUL: i64 = CJK_BASE_SCORE + 1;

const EUC_KR_SCORE_PER_NON_EUC_HANGUL: i64 = CJK_SECONDARY_BASE_SCORE / 5;

const EUC_KR_SCORE_PER_HANJA: i64 = CJK_SECONDARY_BASE_SCORE / 2;

const EUC_KR_HANJA_AFTER_HANGUL_PENALTY: i64 = -(CJK_BASE_SCORE * 10);

const EUC_KR_LONG_WORD_PENALTY: i64 = -6;

const EUC_KR_PUA_PENALTY: i64 = GBK_PUA_PENALTY - 1; // Break tie in favor of GBK

const EUC_KR_MAC_KOREAN_PENALTY: i64 = EUC_KR_PUA_PENALTY * 2;

const EUC_KR_SINGLE_BYTE_EXTENSION_PENALTY: i64 = EUC_KR_MAC_KOREAN_PENALTY;

const GBK_SCORE_PER_LEVEL_1: i64 = CJK_BASE_SCORE;

const GBK_SCORE_PER_LEVEL_2: i64 = CJK_SECONDARY_BASE_SCORE;

const GBK_SCORE_PER_NON_EUC: i64 = CJK_SECONDARY_BASE_SCORE / 4;

const GBK_PUA_PENALTY: i64 = -(CJK_BASE_SCORE * 10); // Factor should be at least 2, but should it be larger?

const GBK_SINGLE_BYTE_EXTENSION_PENALTY: i64 = GBK_PUA_PENALTY * 4;

const CJK_LATIN_ADJACENCY_PENALTY: i64 = -CJK_BASE_SCORE; // smaller penalty than LATIN_ADJACENCY_PENALTY

const CJ_PUNCTUATION: i64 = CJK_BASE_SCORE / 2;

const CJK_OTHER: i64 = CJK_SECONDARY_BASE_SCORE / 4;

/// Latin letter caseless class
const LATIN_LETTER: u8 = 1;

fn contains_upper_case_period_or_non_ascii(label: &[u8]) -> bool {
    for &b in label.into_iter() {
        if b >= 0x80 {
            return true;
        }
        if b == b'.' {
            return true;
        }
        if b >= b'A' && b <= b'Z' {
            return true;
        }
    }
    false
}

// For Latin, we only penalize pairwise bad transitions
// if one participant is non-ASCII. This avoids violating
// the principle that ASCII pairs never contribute to the
// score. (Maybe that's a bad principle, though!)
#[derive(PartialEq)]
enum LatinCaseState {
    Space,
    Upper,
    Lower,
    AllCaps,
}

// Fon non-Latin, we calculate case-related penalty
// or bonus on a per-non-Latin-word basis.
#[derive(PartialEq)]
enum NonLatinCaseState {
    Space,
    Upper,
    Lower,
    UpperLower,
    AllCaps,
    Mix,
}

struct NonLatinCasedCandidate {
    data: &'static SingleByteData,
    prev: u8,
    case_state: NonLatinCaseState,
    prev_ascii: bool,
    current_word_len: u64,
    longest_word: u64,
    ibm866: bool,
    prev_was_a0: bool, // Only used with IBM866
}

impl NonLatinCasedCandidate {
    fn new(data: &'static SingleByteData) -> Self {
        NonLatinCasedCandidate {
            data: data,
            prev: 0,
            case_state: NonLatinCaseState::Space,
            prev_ascii: true,
            current_word_len: 0,
            longest_word: 0,
            ibm866: data == &SINGLE_BYTE_DATA[IBM866_INDEX],
            prev_was_a0: false,
        }
    }

    fn feed(&mut self, buffer: &[u8]) -> Option<i64> {
        let mut score = 0i64;
        for &b in buffer {
            let class = self.data.classify(b);
            if class == 255 {
                return None;
            }
            let caseless_class = class & 0x7F;

            let ascii = b < 0x80;
            let ascii_pair = self.prev_ascii && ascii;

            let non_ascii_alphabetic = self.data.is_non_latin_alphabetic(caseless_class, false);

            // The purpose of this state machine is to avoid misdetecting Greek as
            // Cyrillic by:
            //
            // * Giving a small bonus to words that start with an upper-case letter
            //   and are lower-case for the rest.
            // * Giving a large penalty to start with one lower-case letter followed
            //   by all upper-case (obviously upper and lower case inverted, which
            //   unfortunately is possible due to KOI8-U).
            // * Giving a small per-word penalty to all-uppercase KOI8-U (to favor
            //   all-lowercase Greek over all-caps KOI8-U).
            // * Giving large penalties for mixed-case other than initial upper-case.
            //   This also helps relative to non-cased encodings.

            // ASCII doesn't participate in non-Latin casing.
            if caseless_class == LATIN_LETTER {
                // Latin
                // Mark this word as a mess. If there end up being non-Latin
                // letters in this word, the ASCII-adjacency penalty gets
                // applied to Latin/non-Latin pairs and the mix penalty
                // to non-Latin/non-Latin pairs.
                // XXX Apply penalty here
                self.case_state = NonLatinCaseState::Mix;
            } else if !non_ascii_alphabetic {
                // Space
                match self.case_state {
                    NonLatinCaseState::Space
                    | NonLatinCaseState::Upper
                    | NonLatinCaseState::Lower => {}
                    NonLatinCaseState::UpperLower => {
                        // Intentionally applied only once per word.
                        score += NON_LATIN_CAPITALIZATION_BONUS;
                    }
                    NonLatinCaseState::AllCaps => {
                        // Intentionally applied only once per word.
                        if self.data == &SINGLE_BYTE_DATA[KOI8_U_INDEX] {
                            // Apply only to KOI8-U.
                            score += NON_LATIN_ALL_CAPS_PENALTY;
                        }
                    }
                    NonLatinCaseState::Mix => {
                        // Per letter
                        score += NON_LATIN_MIXED_CASE_PENALTY * (self.current_word_len as i64);
                    }
                }
                self.case_state = NonLatinCaseState::Space;
            } else if (class >> 7) == 0 {
                // Lower case
                match self.case_state {
                    NonLatinCaseState::Space => {
                        self.case_state = NonLatinCaseState::Lower;
                    }
                    NonLatinCaseState::Upper => {
                        self.case_state = NonLatinCaseState::UpperLower;
                    }
                    NonLatinCaseState::Lower
                    | NonLatinCaseState::UpperLower
                    | NonLatinCaseState::Mix => {}
                    NonLatinCaseState::AllCaps => {
                        self.case_state = NonLatinCaseState::Mix;
                    }
                }
            } else {
                // Upper case
                match self.case_state {
                    NonLatinCaseState::Space => {
                        self.case_state = NonLatinCaseState::Upper;
                    }
                    NonLatinCaseState::Upper => {
                        self.case_state = NonLatinCaseState::AllCaps;
                    }
                    NonLatinCaseState::Lower | NonLatinCaseState::UpperLower => {
                        self.case_state = NonLatinCaseState::Mix;
                    }
                    NonLatinCaseState::AllCaps | NonLatinCaseState::Mix => {}
                }
            }

            // XXX Apply penalty if > 16
            if non_ascii_alphabetic {
                self.current_word_len += 1;
            } else {
                if self.current_word_len > self.longest_word {
                    self.longest_word = self.current_word_len;
                }
                self.current_word_len = 0;
            }

            let is_a0 = b == 0xA0;
            if !ascii_pair {
                // 0xA0 is no-break space in many other encodings, so avoid
                // assigning score to IBM866 when 0xA0 occurs next to itself
                // or a space-like byte.
                if !(self.ibm866
                    && ((is_a0 && (self.prev_was_a0 || self.prev == 0))
                        || caseless_class == 0 && self.prev_was_a0))
                {
                    score += self.data.score(caseless_class, self.prev, false);
                }

                if self.prev == LATIN_LETTER && non_ascii_alphabetic {
                    score += LATIN_ADJACENCY_PENALTY;
                } else if caseless_class == LATIN_LETTER
                    && self.data.is_non_latin_alphabetic(self.prev, false)
                {
                    score += LATIN_ADJACENCY_PENALTY;
                }
            }

            self.prev_ascii = ascii;
            self.prev = caseless_class;
            self.prev_was_a0 = is_a0;
        }
        Some(score)
    }
}

enum OrdinalState {
    Other,
    Space,
    PeriodAfterN,
    OrdinalExpectingSpace,
    OrdinalExpectingSpaceUndoImplausibility,
    OrdinalExpectingSpaceOrDigit,
    OrdinalExpectingSpaceOrDigitUndoImplausibily,
    UpperN,
    LowerN,
    FeminineAbbreviationStartLetter,
    Digit,
    Roman,
    Copyright,
}

struct LatinCandidate {
    data: &'static SingleByteData,
    prev: u8,
    case_state: LatinCaseState,
    prev_non_ascii: u32,
    ordinal_state: OrdinalState, // Used only when `windows1252 == true`
    windows1252: bool,
}

impl LatinCandidate {
    fn new(data: &'static SingleByteData) -> Self {
        LatinCandidate {
            data: data,
            prev: 0,
            case_state: LatinCaseState::Space,
            prev_non_ascii: 0,
            ordinal_state: OrdinalState::Space,
            windows1252: data == &SINGLE_BYTE_DATA[WINDOWS_1252_INDEX],
        }
    }

    fn feed(&mut self, buffer: &[u8]) -> Option<i64> {
        let mut score = 0i64;
        for &b in buffer {
            let class = self.data.classify(b);
            if class == 255 {
                return None;
            }
            let caseless_class = class & 0x7F;

            let ascii = b < 0x80;
            let ascii_pair = self.prev_non_ascii == 0 && ascii;

            let non_ascii_penalty = match self.prev_non_ascii {
                0 | 1 | 2 => 0,
                3 => -5,
                4 => -20,
                _ => -200,
            };
            score += non_ascii_penalty;
            // XXX if has Vietnamese-only characters and word length > 7,
            // apply penalty

            if !self.data.is_latin_alphabetic(caseless_class) {
                self.case_state = LatinCaseState::Space;
            } else if (class >> 7) == 0 {
                // Penalizing lower case after two upper case
                // is important for avoiding misdetecting
                // windows-1250 as windows-1252 (byte 0x9F).
                if self.case_state == LatinCaseState::AllCaps && !ascii_pair {
                    score += IMPLAUSIBLE_LATIN_CASE_TRANSITION_PENALTY;
                }
                self.case_state = LatinCaseState::Lower;
            } else {
                match self.case_state {
                    LatinCaseState::Space => {
                        self.case_state = LatinCaseState::Upper;
                    }
                    LatinCaseState::Upper | LatinCaseState::AllCaps => {
                        self.case_state = LatinCaseState::AllCaps;
                    }
                    LatinCaseState::Lower => {
                        if !ascii_pair {
                            // XXX How bad is this for Irish Gaelic?
                            score += IMPLAUSIBLE_LATIN_CASE_TRANSITION_PENALTY;
                        }
                        self.case_state = LatinCaseState::Upper;
                    }
                }
            }

            // Treat pairing space-like, which can be non-ASCII, with ASCII as
            // ASCIIish enough not to get a score in order to avoid giving
            // ASCII i and I in windows-1254 next to windows-125x apostrophe/quote
            // a score. This avoids detecting English I’ as Turkish.
            let ascii_ish_pair = ascii_pair
                || (ascii && self.prev == 0)
                || (caseless_class == 0 && self.prev_non_ascii == 0);

            if !ascii_ish_pair {
                score += self.data.score(caseless_class, self.prev, false);
            }

            if self.windows1252 {
                // This state machine assigns score to the sequences
                // * " º " (Spanish)
                // * " ª " (Spanish)
                // * ".ª " (Spanish)
                // * ".º " (Spanish)
                // * "n.º1" (Spanish)
                // * " Mª " (Spanish)
                // * " Dª " (Spanish)
                // * " Nª " (Spanish)
                // * " Sª " (Spanish)
                // * " 3º " (Italian, where 3 is an ASCII digit)
                // * " 3ª " (Italian, where 3 is an ASCII digit)
                // * " Xº " (Italian, where X is a small Roman numeral)
                // * " Xª " (Italian, where X is a small Roman numeral)
                // * " Nº1" (Italian, where 1 is an ASCII digit)
                // * " Nº " (Italian)
                // * " © " (otherwise ASCII-only)
                // which are problematic to deal with by pairwise scoring
                // without messing up Romanian detection.
                // Initial sc
                match self.ordinal_state {
                    OrdinalState::Other => {
                        if caseless_class == 0 {
                            self.ordinal_state = OrdinalState::Space;
                        }
                    }
                    OrdinalState::Space => {
                        if caseless_class == 0 {
                            // pass
                        } else if b == 0xAA || b == 0xBA {
                            self.ordinal_state = OrdinalState::OrdinalExpectingSpace;
                        } else if b == b'M' || b == b'D' || b == b'S' {
                            self.ordinal_state = OrdinalState::FeminineAbbreviationStartLetter;
                        } else if b == b'N' {
                            // numero or Nuestra
                            self.ordinal_state = OrdinalState::UpperN;
                        } else if b == b'n' {
                            // numero
                            self.ordinal_state = OrdinalState::LowerN;
                        } else if caseless_class == (ASCII_DIGIT as u8) {
                            self.ordinal_state = OrdinalState::Digit;
                        } else if caseless_class == 9 /* I */ || caseless_class == 22 /* V */ || caseless_class == 24
                        /* X */
                        {
                            self.ordinal_state = OrdinalState::Roman;
                        } else if b == 0xA9 {
                            self.ordinal_state = OrdinalState::Copyright;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::OrdinalExpectingSpace => {
                        if caseless_class == 0 {
                            score += ORDINAL_BONUS;
                            self.ordinal_state = OrdinalState::Space;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::OrdinalExpectingSpaceUndoImplausibility => {
                        if caseless_class == 0 {
                            score += ORDINAL_BONUS - IMPLAUSIBILITY_PENALTY;
                            self.ordinal_state = OrdinalState::Space;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::OrdinalExpectingSpaceOrDigit => {
                        if caseless_class == 0 {
                            score += ORDINAL_BONUS;
                            self.ordinal_state = OrdinalState::Space;
                        } else if caseless_class == (ASCII_DIGIT as u8) {
                            score += ORDINAL_BONUS;
                            // Deliberately set to `Other`
                            self.ordinal_state = OrdinalState::Other;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::OrdinalExpectingSpaceOrDigitUndoImplausibily => {
                        if caseless_class == 0 {
                            score += ORDINAL_BONUS - IMPLAUSIBILITY_PENALTY;
                            self.ordinal_state = OrdinalState::Space;
                        } else if caseless_class == (ASCII_DIGIT as u8) {
                            score += ORDINAL_BONUS - IMPLAUSIBILITY_PENALTY;
                            // Deliberately set to `Other`
                            self.ordinal_state = OrdinalState::Other;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::UpperN => {
                        if b == 0xAA {
                            self.ordinal_state =
                                OrdinalState::OrdinalExpectingSpaceUndoImplausibility;
                        } else if b == 0xBA {
                            self.ordinal_state =
                                OrdinalState::OrdinalExpectingSpaceOrDigitUndoImplausibily;
                        } else if b == b'.' {
                            self.ordinal_state = OrdinalState::PeriodAfterN;
                        } else if caseless_class == 0 {
                            self.ordinal_state = OrdinalState::Space;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::LowerN => {
                        if b == 0xBA {
                            self.ordinal_state =
                                OrdinalState::OrdinalExpectingSpaceOrDigitUndoImplausibily;
                        } else if b == b'.' {
                            self.ordinal_state = OrdinalState::PeriodAfterN;
                        } else if caseless_class == 0 {
                            self.ordinal_state = OrdinalState::Space;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::FeminineAbbreviationStartLetter => {
                        if b == 0xAA {
                            self.ordinal_state =
                                OrdinalState::OrdinalExpectingSpaceUndoImplausibility;
                        } else if caseless_class == 0 {
                            self.ordinal_state = OrdinalState::Space;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::Digit => {
                        if b == 0xAA || b == 0xBA {
                            self.ordinal_state = OrdinalState::OrdinalExpectingSpace;
                        } else if caseless_class == 0 {
                            self.ordinal_state = OrdinalState::Space;
                        } else if caseless_class == (ASCII_DIGIT as u8) {
                            // pass
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::Roman => {
                        if b == 0xAA || b == 0xBA {
                            self.ordinal_state =
                                OrdinalState::OrdinalExpectingSpaceUndoImplausibility;
                        } else if caseless_class == 0 {
                            self.ordinal_state = OrdinalState::Space;
                        } else if caseless_class == 9 /* I */ || caseless_class == 22 /* V */ || caseless_class == 24
                        /* X */
                        {
                            // pass
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::PeriodAfterN => {
                        if b == 0xBA {
                            self.ordinal_state = OrdinalState::OrdinalExpectingSpaceOrDigit;
                        } else if caseless_class == 0 {
                            self.ordinal_state = OrdinalState::Space;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                    OrdinalState::Copyright => {
                        if caseless_class == 0 {
                            score += COPYRIGHT_BONUS;
                            self.ordinal_state = OrdinalState::Space;
                        } else {
                            self.ordinal_state = OrdinalState::Other;
                        }
                    }
                }
            }

            if ascii {
                self.prev_non_ascii = 0;
            } else {
                self.prev_non_ascii += 1;
            }
            self.prev = caseless_class;
        }
        Some(score)
    }
}

struct ArabicFrenchCandidate {
    data: &'static SingleByteData,
    prev: u8,
    case_state: LatinCaseState,
    prev_ascii: bool,
    current_word_len: u64,
    longest_word: u64,
}

impl ArabicFrenchCandidate {
    fn new(data: &'static SingleByteData) -> Self {
        ArabicFrenchCandidate {
            data: data,
            prev: 0,
            case_state: LatinCaseState::Space,
            prev_ascii: true,
            current_word_len: 0,
            longest_word: 0,
        }
    }

    fn feed(&mut self, buffer: &[u8]) -> Option<i64> {
        let mut score = 0i64;
        for &b in buffer {
            let class = self.data.classify(b);
            if class == 255 {
                return None;
            }
            let caseless_class = class & 0x7F;

            let ascii = b < 0x80;
            let ascii_pair = self.prev_ascii && ascii;

            if caseless_class != LATIN_LETTER {
                // We compute case penalties for French only
                self.case_state = LatinCaseState::Space;
            } else if (class >> 7) == 0 {
                if self.case_state == LatinCaseState::AllCaps && !ascii_pair {
                    score += IMPLAUSIBLE_LATIN_CASE_TRANSITION_PENALTY;
                }
                self.case_state = LatinCaseState::Lower;
            } else {
                match self.case_state {
                    LatinCaseState::Space => {
                        self.case_state = LatinCaseState::Upper;
                    }
                    LatinCaseState::Upper | LatinCaseState::AllCaps => {
                        self.case_state = LatinCaseState::AllCaps;
                    }
                    LatinCaseState::Lower => {
                        if !ascii_pair {
                            score += IMPLAUSIBLE_LATIN_CASE_TRANSITION_PENALTY;
                        }
                        self.case_state = LatinCaseState::Upper;
                    }
                }
            }

            // Count only Arabic word length and ignore French
            let non_ascii_alphabetic = self.data.is_non_latin_alphabetic(caseless_class, true);
            // XXX apply penalty if > 23
            if non_ascii_alphabetic {
                self.current_word_len += 1;
            } else {
                if self.current_word_len > self.longest_word {
                    self.longest_word = self.current_word_len;
                }
                self.current_word_len = 0;
            }

            if !ascii_pair {
                score += self.data.score(caseless_class, self.prev, true);

                if self.prev == LATIN_LETTER && non_ascii_alphabetic {
                    score += LATIN_ADJACENCY_PENALTY;
                } else if caseless_class == LATIN_LETTER
                    && self.data.is_non_latin_alphabetic(self.prev, true)
                {
                    score += LATIN_ADJACENCY_PENALTY;
                }
            }

            self.prev_ascii = ascii;
            self.prev = caseless_class;
        }
        Some(score)
    }
}

struct CaselessCandidate {
    data: &'static SingleByteData,
    prev: u8,
    prev_ascii: bool,
    current_word_len: u64,
    longest_word: u64,
}

impl CaselessCandidate {
    fn new(data: &'static SingleByteData) -> Self {
        CaselessCandidate {
            data: data,
            prev: 0,
            prev_ascii: true,
            current_word_len: 0,
            longest_word: 0,
        }
    }

    fn feed(&mut self, buffer: &[u8]) -> Option<i64> {
        let mut score = 0i64;
        for &b in buffer {
            let class = self.data.classify(b);
            if class == 255 {
                return None;
            }
            let caseless_class = class & 0x7F;

            let ascii = b < 0x80;
            let ascii_pair = self.prev_ascii && ascii;

            let non_ascii_alphabetic = self.data.is_non_latin_alphabetic(caseless_class, false);
            // Apply penalty if > 23 and not Thai
            if non_ascii_alphabetic {
                self.current_word_len += 1;
            } else {
                if self.current_word_len > self.longest_word {
                    self.longest_word = self.current_word_len;
                }
                self.current_word_len = 0;
            }

            if !ascii_pair {
                score += self.data.score(caseless_class, self.prev, false);

                if self.prev == LATIN_LETTER && non_ascii_alphabetic {
                    score += LATIN_ADJACENCY_PENALTY;
                } else if caseless_class == LATIN_LETTER
                    && self.data.is_non_latin_alphabetic(self.prev, false)
                {
                    score += LATIN_ADJACENCY_PENALTY;
                }
            }

            self.prev_ascii = ascii;
            self.prev = caseless_class;
        }
        Some(score)
    }
}

fn is_ascii_punctuation(byte: u8) -> bool {
    match byte {
        b'.' | b',' | b':' | b';' | b'?' | b'!' => true,
        _ => false,
    }
}

struct LogicalCandidate {
    data: &'static SingleByteData,
    prev: u8,
    prev_ascii: bool,
    plausible_punctuation: u64,
    current_word_len: u64,
    longest_word: u64,
}

impl LogicalCandidate {
    fn new(data: &'static SingleByteData) -> Self {
        LogicalCandidate {
            data: data,
            prev: 0,
            prev_ascii: true,
            plausible_punctuation: 0,
            current_word_len: 0,
            longest_word: 0,
        }
    }

    fn feed(&mut self, buffer: &[u8]) -> Option<i64> {
        let mut score = 0i64;
        for &b in buffer {
            let class = self.data.classify(b);
            if class == 255 {
                return None;
            }
            let caseless_class = class & 0x7F;

            let ascii = b < 0x80;
            let ascii_pair = self.prev_ascii && ascii;

            let non_ascii_alphabetic = self.data.is_non_latin_alphabetic(caseless_class, false);
            // XXX apply penalty if > 22
            if non_ascii_alphabetic {
                self.current_word_len += 1;
            } else {
                if self.current_word_len > self.longest_word {
                    self.longest_word = self.current_word_len;
                }
                self.current_word_len = 0;
            }

            if !ascii_pair {
                score += self.data.score(caseless_class, self.prev, false);

                let prev_non_ascii_alphabetic = self.data.is_non_latin_alphabetic(self.prev, false);
                if caseless_class == 0 && prev_non_ascii_alphabetic && is_ascii_punctuation(b) {
                    self.plausible_punctuation += 1;
                }

                if self.prev == LATIN_LETTER && non_ascii_alphabetic {
                    score += LATIN_ADJACENCY_PENALTY;
                } else if caseless_class == LATIN_LETTER && prev_non_ascii_alphabetic {
                    score += LATIN_ADJACENCY_PENALTY;
                }
            }

            self.prev_ascii = ascii;
            self.prev = caseless_class;
        }
        Some(score)
    }
}

struct VisualCandidate {
    data: &'static SingleByteData,
    prev: u8,
    prev_ascii: bool,
    prev_punctuation: bool,
    plausible_punctuation: u64,
    current_word_len: u64,
    longest_word: u64,
}

impl VisualCandidate {
    fn new(data: &'static SingleByteData) -> Self {
        VisualCandidate {
            data: data,
            prev: 0,
            prev_ascii: true,
            prev_punctuation: false,
            plausible_punctuation: 0,
            current_word_len: 0,
            longest_word: 0,
        }
    }

    fn feed(&mut self, buffer: &[u8]) -> Option<i64> {
        let mut score = 0i64;
        for &b in buffer {
            let class = self.data.classify(b);
            if class == 255 {
                return None;
            }
            let caseless_class = class & 0x7F;

            let ascii = b < 0x80;
            let ascii_pair = self.prev_ascii && ascii;

            let non_ascii_alphabetic = self.data.is_non_latin_alphabetic(caseless_class, false);
            // XXX apply penalty if > 22
            if non_ascii_alphabetic {
                self.current_word_len += 1;
            } else {
                if self.current_word_len > self.longest_word {
                    self.longest_word = self.current_word_len;
                }
                self.current_word_len = 0;
            }

            if !ascii_pair {
                score += self.data.score(caseless_class, self.prev, false);

                if non_ascii_alphabetic && self.prev_punctuation {
                    self.plausible_punctuation += 1;
                }

                if self.prev == LATIN_LETTER && non_ascii_alphabetic {
                    score += LATIN_ADJACENCY_PENALTY;
                } else if caseless_class == LATIN_LETTER
                    && self.data.is_non_latin_alphabetic(self.prev, false)
                {
                    score += LATIN_ADJACENCY_PENALTY;
                }
            }

            self.prev_ascii = ascii;
            self.prev = caseless_class;
            self.prev_punctuation = caseless_class == 0 && is_ascii_punctuation(b);
        }
        Some(score)
    }
}

struct Utf8Candidate {
    decoder: Decoder,
}

impl Utf8Candidate {
    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        let mut dst = [0u8; 1024];
        let mut total_read = 0;
        loop {
            let (result, read, _) = self.decoder.decode_to_utf8_without_replacement(
                &buffer[total_read..],
                &mut dst,
                last,
            );
            total_read += read;
            match result {
                DecoderResult::InputEmpty => {
                    return Some(0);
                }
                DecoderResult::Malformed(_, _) => {
                    return None;
                }
                DecoderResult::OutputFull => {
                    continue;
                }
            }
        }
    }
}

struct Iso2022Candidate {
    decoder: Decoder,
}

impl Iso2022Candidate {
    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        let mut dst = [0u16; 1024];
        let mut total_read = 0;
        loop {
            let (result, read, _) = self.decoder.decode_to_utf16_without_replacement(
                &buffer[total_read..],
                &mut dst,
                last,
            );
            total_read += read;
            match result {
                DecoderResult::InputEmpty => {
                    return Some(0);
                }
                DecoderResult::Malformed(_, _) => {
                    return None;
                }
                DecoderResult::OutputFull => {
                    continue;
                }
            }
        }
    }
}

#[derive(PartialEq)]
enum LatinCj {
    AsciiLetter,
    Cj,
    Other,
}

#[derive(PartialEq, Copy, Clone)]
enum HalfWidthKatakana {
    DakutenForbidden,
    DakutenAllowed,
    DakutenOrHandakutenAllowed,
}

#[derive(PartialEq)]
enum LatinKorean {
    AsciiLetter,
    Hangul,
    Hanja,
    Other,
}

fn cjk_extra_score(u: u16, table: &'static [u16; 128]) -> i64 {
    if let Some(pos) = table.iter().position(|&x| x == u) {
        ((128 - pos) / 16) as i64
    } else {
        0
    }
}

struct GbkCandidate {
    decoder: Decoder,
    prev_byte: u8,
    prev: LatinCj,
    pending_score: Option<i64>,
}

impl GbkCandidate {
    fn maybe_set_as_pending(&mut self, s: i64) -> i64 {
        assert!(self.pending_score.is_none());
        if self.prev == LatinCj::Cj || !more_problematic_lead(self.prev_byte) {
            s
        } else {
            self.pending_score = Some(s);
            0
        }
    }

    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        let mut score = 0i64;
        let mut src = [0u8];
        let mut dst = [0u16; 2];
        for &b in buffer {
            src[0] = b;
            let (result, read, written) = self
                .decoder
                .decode_to_utf16_without_replacement(&src, &mut dst, false);
            if written == 1 {
                let u = dst[0];
                if (u >= u16::from(b'a') && u <= u16::from(b'z'))
                    || (u >= u16::from(b'A') && u <= u16::from(b'Z'))
                {
                    self.pending_score = None; // Discard pending score
                    if self.prev == LatinCj::Cj {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::AsciiLetter;
                } else if u == 0x20AC {
                    // euro sign
                    self.pending_score = None; // Discard pending score
                                               // Should there even be a penalty?
                    self.prev = LatinCj::Other;
                } else if u >= 0x4E00 && u <= 0x9FA5 {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    if b >= 0xA1 && b <= 0xFE {
                        match self.prev_byte {
                            0xA1..=0xD7 => {
                                score += GBK_SCORE_PER_LEVEL_1;
                                score +=
                                    cjk_extra_score(u, &data::DETECTOR_DATA.frequent_simplified);
                            }
                            0xD8..=0xFE => score += GBK_SCORE_PER_LEVEL_2,
                            _ => {
                                score += GBK_SCORE_PER_NON_EUC;
                            }
                        }
                    } else {
                        score += self.maybe_set_as_pending(GBK_SCORE_PER_NON_EUC);
                    }
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else if (u >= 0x3400 && u < 0xA000) || (u >= 0xF900 && u < 0xFB00) {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    // XXX score?
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else if u >= 0xE000 && u < 0xF900 {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    // Treat the GB18030-required PUA mappings as non-EUC ideographs.
                    match u {
                        0xE78D..=0xE796
                        | 0xE816..=0xE818
                        | 0xE81E
                        | 0xE826
                        | 0xE82B
                        | 0xE82C
                        | 0xE831
                        | 0xE832
                        | 0xE83B
                        | 0xE843
                        | 0xE854
                        | 0xE855
                        | 0xE864 => {
                            score += GBK_SCORE_PER_NON_EUC;
                            if self.prev == LatinCj::AsciiLetter {
                                score += CJK_LATIN_ADJACENCY_PENALTY;
                            }
                            self.prev = LatinCj::Cj;
                        }
                        _ => {
                            score += GBK_PUA_PENALTY;
                            self.prev = LatinCj::Other;
                        }
                    }
                } else {
                    match u {
                        0x3000 // Distinct from Korean, space
                        | 0x3001 // Distinct from Korean, enumeration comma
                        | 0x3002 // Distinct from Korean, full stop
                        | 0xFF08 // Distinct from Korean, parenthesis
                        | 0xFF09 // Distinct from Korean, parenthesis
                        | 0xFF01 // Distinct from Japanese, exclamation
                        | 0xFF0C // Distinct from Japanese, comma
                        | 0xFF1B // Distinct from Japanese, semicolon
                        | 0xFF1F // Distinct from Japanese, question
                        => {
                            if let Some(pending) = self.pending_score {
                                score += pending;
                                self.pending_score = None;
                            }
                            score += CJ_PUNCTUATION;
                        }
                        0..=0x7F => {
                            self.pending_score = None; // Discard pending score
                        }
                        _ => {
                            if let Some(pending) = self.pending_score {
                                score += pending;
                                self.pending_score = None;
                            }
                            score += CJK_OTHER;
                        }
                    }
                    self.prev = LatinCj::Other;
                }
            } else if written == 2 {
                if let Some(pending) = self.pending_score {
                    score += pending;
                    self.pending_score = None;
                }
                let u = dst[0];
                if u >= 0xDB80 && u <= 0xDBFF {
                    score += GBK_PUA_PENALTY;
                    self.prev = LatinCj::Other;
                } else if u >= 0xD480 && u < 0xD880 {
                    score += GBK_SCORE_PER_NON_EUC;
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else {
                    score += CJK_OTHER;
                    self.prev = LatinCj::Other;
                }
            }
            match result {
                DecoderResult::InputEmpty => {
                    assert_eq!(read, 1);
                }
                DecoderResult::Malformed(malformed_len, _) => {
                    if (self.prev_byte == 0xA0 || self.prev_byte == 0xFE || self.prev_byte == 0xFD)
                        && (b < 0x80 || b == 0xFF)
                    {
                        // Mac OS Chinese Simplified single-byte that conflicts with code page GBK lead byte
                        // followed by ASCII or a non-conflicting single-byte extension.
                        self.pending_score = None; // Just in case
                        score += GBK_SINGLE_BYTE_EXTENSION_PENALTY;
                        if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') {
                            self.prev = LatinCj::AsciiLetter;
                        } else if b == 0xFF {
                            score += GBK_SINGLE_BYTE_EXTENSION_PENALTY;
                            self.prev = LatinCj::Other;
                        } else {
                            self.prev = LatinCj::Other;
                        }
                        // The GBK decoder has the pending ASCII concept, which is
                        // a problem with this trickery, so let's reset the state.
                        self.decoder = GBK.new_decoder_without_bom_handling();
                    } else if malformed_len == 1 && b == 0xFF {
                        // Mac OS Chinese Simplified single-byte extension that doesn't conflict with lead bytes
                        self.pending_score = None; // Just in case
                        score += GBK_SINGLE_BYTE_EXTENSION_PENALTY;
                        self.prev = LatinCj::Other;
                        // The GBK decoder has the pending ASCII concept, which is
                        // a problem with this trickery, so let's reset the state.
                        self.decoder = GBK.new_decoder_without_bom_handling();
                    } else {
                        return None;
                    }
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
            self.prev_byte = b;
        }
        if last {
            let (result, _, _) = self
                .decoder
                .decode_to_utf16_without_replacement(b"", &mut dst, true);
            match result {
                DecoderResult::InputEmpty => {}
                DecoderResult::Malformed(_, _) => {
                    return None;
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
        }
        Some(score)
    }
}

// Shift_JIS and Big5
fn problematic_lead(b: u8) -> bool {
    match b {
        0x91..=0x97 | 0x9A | 0x8A | 0x9B | 0x8B | 0x9E | 0x8E | 0xB0 => true,
        _ => false,
    }
}

// GBK and EUC-KR
fn more_problematic_lead(b: u8) -> bool {
    problematic_lead(b) || b == 0x82 || b == 0x84 || b == 0x85 || b == 0xA0
}

struct ShiftJisCandidate {
    decoder: Decoder,
    half_width_katakana_seen: bool,
    half_width_katakana_state: HalfWidthKatakana,
    prev: LatinCj,
    prev_byte: u8,
    pending_score: Option<i64>,
}

impl ShiftJisCandidate {
    fn maybe_set_as_pending(&mut self, s: i64) -> i64 {
        assert!(self.pending_score.is_none());
        if self.prev == LatinCj::Cj || !problematic_lead(self.prev_byte) {
            s
        } else {
            self.pending_score = Some(s);
            0
        }
    }

    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        let mut score = 0i64;
        let mut src = [0u8];
        let mut dst = [0u16; 2];
        for &b in buffer {
            src[0] = b;
            let (result, read, written) = self
                .decoder
                .decode_to_utf16_without_replacement(&src, &mut dst, false);
            if written > 0 {
                let half_width_katakana_state = self.half_width_katakana_state;
                self.half_width_katakana_state = HalfWidthKatakana::DakutenForbidden;
                let u = dst[0];
                if (u >= u16::from(b'a') && u <= u16::from(b'z'))
                    || (u >= u16::from(b'A') && u <= u16::from(b'Z'))
                {
                    self.pending_score = None; // Discard pending score
                    if self.prev == LatinCj::Cj {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::AsciiLetter;
                } else if u >= 0xFF61 && u <= 0xFF9F {
                    if !self.half_width_katakana_seen {
                        self.half_width_katakana_seen = true;
                        // To avoid misdetecting title-length inputs
                        score += SHIFT_JIS_INITIAL_HALF_WIDTH_KATAKANA_PENALTY;
                    }
                    self.pending_score = None; // Discard pending score
                    score += HALF_WIDTH_KATAKANA_SCORE;

                    if (u >= 0xFF76 && u <= 0xFF84) || u == 0xFF73 {
                        self.half_width_katakana_state = HalfWidthKatakana::DakutenAllowed;
                    } else if u >= 0xFF8A && u <= 0xFF8E {
                        self.half_width_katakana_state =
                            HalfWidthKatakana::DakutenOrHandakutenAllowed;
                    } else if u == 0xFF9E {
                        if half_width_katakana_state == HalfWidthKatakana::DakutenForbidden {
                            score += IMPLAUSIBILITY_PENALTY;
                        } else {
                            score += HALF_WIDTH_KATAKANA_VOICING_SCORE;
                        }
                    } else if u == 0xFF9F {
                        if half_width_katakana_state
                            != HalfWidthKatakana::DakutenOrHandakutenAllowed
                        {
                            score += IMPLAUSIBILITY_PENALTY;
                        } else {
                            score += HALF_WIDTH_KATAKANA_VOICING_SCORE;
                        }
                    }

                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else if u >= 0x3040 && u < 0x3100 {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    score += SHIFT_JIS_SCORE_PER_KANA;
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else if (u >= 0x3400 && u < 0xA000) || (u >= 0xF900 && u < 0xFB00) {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    if self.prev_byte < 0x98 || (self.prev_byte == 0x98 && b < 0x73) {
                        score += self.maybe_set_as_pending(
                            SHIFT_JIS_SCORE_PER_LEVEL_1_KANJI
                                + cjk_extra_score(u, &data::DETECTOR_DATA.frequent_kanji),
                        );
                    } else {
                        score += self.maybe_set_as_pending(SHIFT_JIS_SCORE_PER_LEVEL_2_KANJI);
                    }
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else if u >= 0xE000 && u < 0xF900 {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    score += SHIFT_JIS_PUA_PENALTY;
                    self.prev = LatinCj::Other;
                } else {
                    match u {
                        0x3000 // Distinct from Korean, space
                        | 0x3001 // Distinct from Korean, enumeration comma
                        | 0x3002 // Distinct from Korean, full stop
                        | 0xFF08 // Distinct from Korean, parenthesis
                        | 0xFF09 // Distinct from Korean, parenthesis
                        => {
                            if let Some(pending) = self.pending_score {
                                score += pending;
                                self.pending_score = None;
                            }
                            // Not really needed for CJK distinction
                            // but let's give non-zero score for these
                            // common byte pairs anyway.
                            score += CJ_PUNCTUATION;
                        }
                        0..=0x7F => {
                            self.pending_score = None; // Discard pending score
                        }
                        0x80 => {
                            // This is a control character that overlaps euro
                            // in windows-1252 and happens to be a non-error
                            // is Shift_JIS.
                            self.pending_score = None; // Discard pending score
                            score += IMPLAUSIBILITY_PENALTY;
                        }
                        _ => {
                            if let Some(pending) = self.pending_score {
                                score += pending;
                                self.pending_score = None;
                            }
                            score += CJK_OTHER;
                        }
                    }
                    self.prev = LatinCj::Other;
                }
            }
            match result {
                DecoderResult::InputEmpty => {
                    assert_eq!(read, 1);
                }
                DecoderResult::Malformed(malformed_len, _) => {
                    if (((self.prev_byte >= 0x81 && self.prev_byte <= 0x9F)
                        || (self.prev_byte >= 0xE0 && self.prev_byte <= 0xFC))
                        && ((b >= 0x40 && b <= 0x7E) || (b >= 0x80 && b <= 0xFC)))
                        && !((self.prev_byte == 0x82 && b >= 0xFA)
                            || (self.prev_byte == 0x84 && ((b >= 0xDD && b <= 0xE4) || b >= 0xFB))
                            || (self.prev_byte == 0x86 && b >= 0xF2 && b <= 0xFA)
                            || (self.prev_byte == 0x87 && b >= 0x77 && b <= 0x7D)
                            || (self.prev_byte == 0xFC && b >= 0xF5))
                    {
                        // Shift_JIS2004 or MacJapanese
                        if let Some(pending) = self.pending_score {
                            score += pending;
                            self.pending_score = None;
                        }
                        score += SHIFT_JIS_EXTENSION_PENALTY;
                        // Approximate boundary
                        if self.prev_byte < 0x87 {
                            self.prev = LatinCj::Other;
                        } else {
                            if self.prev == LatinCj::AsciiLetter {
                                score += CJK_LATIN_ADJACENCY_PENALTY;
                            }
                            self.prev = LatinCj::Cj;
                        }
                    } else if malformed_len == 1 && (b == 0xA0 || b >= 0xFD) {
                        self.pending_score = None; // Just in case
                        score += SHIFT_JIS_SINGLE_BYTE_EXTENSION_PENALTY;
                        self.prev = LatinCj::Other;
                    } else {
                        return None;
                    }
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
            self.prev_byte = b;
        }
        if last {
            let (result, _, _) = self
                .decoder
                .decode_to_utf16_without_replacement(b"", &mut dst, true);
            match result {
                DecoderResult::InputEmpty => {}
                DecoderResult::Malformed(_, _) => {
                    return None;
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
        }
        Some(score)
    }
}

struct EucJpCandidate {
    decoder: Decoder,
    non_ascii_seen: bool,
    half_width_katakana_state: HalfWidthKatakana,
    prev: LatinCj,
    prev_byte: u8,
    prev_prev_byte: u8,
}

impl EucJpCandidate {
    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        let mut score = 0i64;
        let mut src = [0u8];
        let mut dst = [0u16; 2];
        for &b in buffer {
            src[0] = b;
            let (result, read, written) = self
                .decoder
                .decode_to_utf16_without_replacement(&src, &mut dst, false);
            if written > 0 {
                let half_width_katakana_state = self.half_width_katakana_state;
                self.half_width_katakana_state = HalfWidthKatakana::DakutenForbidden;
                let u = dst[0];
                if !self.non_ascii_seen && u >= 0x80 {
                    self.non_ascii_seen = true;
                    if u >= 0xFF61 && u <= 0xFF9F {
                        // return None;
                    }
                    if u >= 0x3040 && u < 0x3100 {
                        // Remove the kana advantage over initial Big5
                        // hanzi.
                        score += EUC_JP_INITIAL_KANA_PENALTY;
                    }
                }
                if (u >= u16::from(b'a') && u <= u16::from(b'z'))
                    || (u >= u16::from(b'A') && u <= u16::from(b'Z'))
                {
                    if self.prev == LatinCj::Cj {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::AsciiLetter;
                } else if u >= 0xFF61 && u <= 0xFF9F {
                    score += HALF_WIDTH_KATAKANA_SCORE;

                    if (u >= 0xFF76 && u <= 0xFF84) || u == 0xFF73 {
                        self.half_width_katakana_state = HalfWidthKatakana::DakutenAllowed;
                    } else if u >= 0xFF8A && u <= 0xFF8E {
                        self.half_width_katakana_state =
                            HalfWidthKatakana::DakutenOrHandakutenAllowed;
                    } else if u == 0xFF9E {
                        if half_width_katakana_state == HalfWidthKatakana::DakutenForbidden {
                            score += IMPLAUSIBILITY_PENALTY;
                        } else {
                            score += HALF_WIDTH_KATAKANA_VOICING_SCORE;
                        }
                    } else if u == 0xFF9F {
                        if half_width_katakana_state
                            != HalfWidthKatakana::DakutenOrHandakutenAllowed
                        {
                            score += IMPLAUSIBILITY_PENALTY;
                        } else {
                            score += HALF_WIDTH_KATAKANA_VOICING_SCORE;
                        }
                    }

                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Other;
                } else if (u >= 0x3041 && u <= 0x3093) || (u >= 0x30A1 && u <= 0x30F6) {
                    match u {
                        0x3090 // hiragana wi
                        | 0x3091 // hiragana we
                        | 0x30F0 // katakana wi
                        | 0x30F1 // katakana we
                        => {
                            // Remove advantage over Big5 Hanzi
                            score += EUC_JP_SCORE_PER_NEAR_OBSOLETE_KANA;
                        }
                        _ => {
                            score += EUC_JP_SCORE_PER_KANA;
                        }
                    }
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else if (u >= 0x3400 && u < 0xA000) || (u >= 0xF900 && u < 0xFB00) {
                    if self.prev_prev_byte == 0x8F {
                        score += EUC_JP_SCORE_PER_OTHER_KANJI;
                    } else if self.prev_byte < 0xD0 {
                        score += EUC_JP_SCORE_PER_LEVEL_1_KANJI;
                        score += cjk_extra_score(u, &data::DETECTOR_DATA.frequent_kanji);
                    } else {
                        score += EUC_JP_SCORE_PER_LEVEL_2_KANJI;
                    }
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else {
                    match u {
                        0x3000 // Distinct from Korean, space
                        | 0x3001 // Distinct from Korean, enumeration comma
                        | 0x3002 // Distinct from Korean, full stop
                        | 0xFF08 // Distinct from Korean, parenthesis
                        | 0xFF09 // Distinct from Korean, parenthesis
                        => {
                            score += CJ_PUNCTUATION;
                        }
                        0..=0x7F => {}
                        _ => {
                            score += CJK_OTHER;
                        }
                    }
                    self.prev = LatinCj::Other;
                }
            }
            match result {
                DecoderResult::InputEmpty => {
                    assert_eq!(read, 1);
                }
                DecoderResult::Malformed(_, _) => {
                    if b >= 0xA1
                        && b <= 0xFE
                        && self.prev_byte >= 0xA1
                        && self.prev_byte <= 0xFE
                        && ((self.prev_prev_byte != 0x8F
                            && !(self.prev_byte == 0xA8 && b >= 0xDF && b <= 0xE6)
                            && !(self.prev_byte == 0xAC && b >= 0xF4 && b <= 0xFC)
                            && !(self.prev_byte == 0xAD && b >= 0xD8 && b <= 0xDE))
                            || (self.prev_prev_byte == 0x8F
                                && self.prev_byte != 0xA2
                                && self.prev_byte != 0xA6
                                && self.prev_byte != 0xA7
                                && self.prev_byte != 0xA9
                                && self.prev_byte != 0xAA
                                && self.prev_byte != 0xAB
                                && self.prev_byte != 0xED
                                && !(self.prev_byte == 0xFE && b >= 0xF7)))
                    {
                        score += EUC_JP_EXTENSION_PENALTY;
                        if self.prev == LatinCj::AsciiLetter {
                            score += CJK_LATIN_ADJACENCY_PENALTY;
                        }
                        self.prev = LatinCj::Cj;
                    } else {
                        return None;
                    }
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
            self.prev_prev_byte = self.prev_byte;
            self.prev_byte = b;
        }
        if last {
            let (result, _, _) = self
                .decoder
                .decode_to_utf16_without_replacement(b"", &mut dst, true);
            match result {
                DecoderResult::InputEmpty => {}
                DecoderResult::Malformed(_, _) => {
                    return None;
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
        }
        Some(score)
    }
}

struct Big5Candidate {
    decoder: Decoder,
    prev: LatinCj,
    prev_byte: u8,
    pending_score: Option<i64>,
}

impl Big5Candidate {
    fn maybe_set_as_pending(&mut self, s: i64) -> i64 {
        assert!(self.pending_score.is_none());
        if self.prev == LatinCj::Cj || !problematic_lead(self.prev_byte) {
            s
        } else {
            self.pending_score = Some(s);
            0
        }
    }

    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        let mut score = 0i64;
        let mut src = [0u8];
        let mut dst = [0u16; 2];
        for &b in buffer {
            src[0] = b;
            let (result, read, written) = self
                .decoder
                .decode_to_utf16_without_replacement(&src, &mut dst, false);
            if written == 1 {
                let u = dst[0];
                if (u >= u16::from(b'a') && u <= u16::from(b'z'))
                    || (u >= u16::from(b'A') && u <= u16::from(b'Z'))
                {
                    self.pending_score = None; // Discard pending score
                    if self.prev == LatinCj::Cj {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::AsciiLetter;
                } else if (u >= 0x3400 && u < 0xA000) || (u >= 0xF900 && u < 0xFB00) {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    match self.prev_byte {
                        0xA4..=0xC6 => {
                            score += self.maybe_set_as_pending(BIG5_SCORE_PER_LEVEL_1_HANZI);
                            // score += cjk_extra_score(u, &data::DETECTOR_DATA.frequent_traditional);
                        }
                        _ => {
                            score += self.maybe_set_as_pending(BIG5_SCORE_PER_OTHER_HANZI);
                        }
                    }
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                } else {
                    match u {
                        0x3000 // Distinct from Korean, space
                        | 0x3001 // Distinct from Korean, enumeration comma
                        | 0x3002 // Distinct from Korean, full stop
                        | 0xFF08 // Distinct from Korean, parenthesis
                        | 0xFF09 // Distinct from Korean, parenthesis
                        | 0xFF01 // Distinct from Japanese, exclamation
                        | 0xFF0C // Distinct from Japanese, comma
                        | 0xFF1B // Distinct from Japanese, semicolon
                        | 0xFF1F // Distinct from Japanese, question
                        => {
                            if let Some(pending) = self.pending_score {
                                score += pending;
                                self.pending_score = None;
                            }
                            // Not really needed for CJK distinction
                            // but let's give non-zero score for these
                            // common byte pairs anyway.
                            score += CJ_PUNCTUATION;
                        }
                        0..=0x7F => {
                            self.pending_score = None; // Discard pending score
                        }
                        _ => {
                            if let Some(pending) = self.pending_score {
                                score += pending;
                                self.pending_score = None;
                            }
                            score += CJK_OTHER;
                        }
                    }
                    self.prev = LatinCj::Other;
                }
            } else if written == 2 {
                if let Some(pending) = self.pending_score {
                    score += pending;
                    self.pending_score = None;
                }
                if dst[0] == 0xCA || dst[0] == 0xEA {
                    score += CJK_OTHER;
                    self.prev = LatinCj::Other;
                } else {
                    debug_assert!(dst[0] >= 0xD480 && dst[0] < 0xD880);
                    score += self.maybe_set_as_pending(BIG5_SCORE_PER_OTHER_HANZI);
                    if self.prev == LatinCj::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinCj::Cj;
                }
            }
            match result {
                DecoderResult::InputEmpty => {
                    assert_eq!(read, 1);
                }
                DecoderResult::Malformed(malformed_len, _) => {
                    if self.prev_byte >= 0x81
                        && self.prev_byte <= 0xFE
                        && ((b >= 0x40 && b <= 0x7E) || (b >= 0xA1 && b <= 0xFE))
                    {
                        // The byte pair is in the Big5 range but unmapped.
                        // Treat as PUA to avoid rejecting Big5-UAO, etc.
                        // We don't reprocess `b` even if ASCII, since it's
                        // logically part of the pair.
                        if let Some(pending) = self.pending_score {
                            score += pending;
                            self.pending_score = None;
                        }
                        score += BIG5_PUA_PENALTY;
                        // Assume Hanzi semantics
                        if self.prev == LatinCj::AsciiLetter {
                            score += CJK_LATIN_ADJACENCY_PENALTY;
                        }
                        self.prev = LatinCj::Cj;
                    } else if (self.prev_byte == 0xA0
                        || self.prev_byte == 0xFD
                        || self.prev_byte == 0xFE)
                        && (b < 0x80 || b == 0xFF)
                    {
                        // Mac OS Chinese Traditional single-byte that conflicts with code page Big5 lead byte
                        // followed by ASCII or a non-conflicting single-byte extension.
                        self.pending_score = None; // Just in case
                        score += BIG5_SINGLE_BYTE_EXTENSION_PENALTY;
                        if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') {
                            self.prev = LatinCj::AsciiLetter;
                        } else if b == 0xFF {
                            score += BIG5_SINGLE_BYTE_EXTENSION_PENALTY;
                            self.prev = LatinCj::Other;
                        } else {
                            self.prev = LatinCj::Other;
                        }
                    } else if malformed_len == 1 && b == 0xFF {
                        // Mac OS Chinese Traditional single-byte extension that doesn't conflict with lead bytes
                        self.pending_score = None; // Just in case
                        score += BIG5_SINGLE_BYTE_EXTENSION_PENALTY;
                        self.prev = LatinCj::Other;
                    } else {
                        return None;
                    }
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
            self.prev_byte = b;
        }
        if last {
            let (result, _, _) = self
                .decoder
                .decode_to_utf16_without_replacement(b"", &mut dst, true);
            match result {
                DecoderResult::InputEmpty => {}
                DecoderResult::Malformed(_, _) => {
                    return None;
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
        }
        Some(score)
    }
}

struct EucKrCandidate {
    decoder: Decoder,
    prev_byte: u8,
    prev_was_euc_range: bool,
    prev: LatinKorean,
    current_word_len: u64,
    pending_score: Option<i64>,
}

impl EucKrCandidate {
    fn maybe_set_as_pending(&mut self, s: i64) -> i64 {
        assert!(self.pending_score.is_none());
        if self.prev == LatinKorean::Hangul || !more_problematic_lead(self.prev_byte) {
            s
        } else {
            self.pending_score = Some(s);
            0
        }
    }

    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        let mut score = 0i64;
        let mut src = [0u8];
        let mut dst = [0u16; 2];
        for &b in buffer {
            let in_euc_range = b >= 0xA1 && b <= 0xFE;
            src[0] = b;
            let (result, read, written) = self
                .decoder
                .decode_to_utf16_without_replacement(&src, &mut dst, false);
            if written > 0 {
                let u = dst[0];
                if (u >= u16::from(b'a') && u <= u16::from(b'z'))
                    || (u >= u16::from(b'A') && u <= u16::from(b'Z'))
                {
                    self.pending_score = None; // Discard pending score
                    match self.prev {
                        LatinKorean::Hangul | LatinKorean::Hanja => {
                            score += CJK_LATIN_ADJACENCY_PENALTY;
                        }
                        _ => {}
                    }
                    self.prev = LatinKorean::AsciiLetter;
                    self.current_word_len = 0;
                } else if u >= 0xAC00 && u <= 0xD7A3 {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    if self.prev_was_euc_range && in_euc_range {
                        score += EUC_KR_SCORE_PER_EUC_HANGUL;
                        score += cjk_extra_score(u, &data::DETECTOR_DATA.frequent_hangul);
                    } else {
                        score += self.maybe_set_as_pending(EUC_KR_SCORE_PER_NON_EUC_HANGUL);
                    }
                    if self.prev == LatinKorean::AsciiLetter {
                        score += CJK_LATIN_ADJACENCY_PENALTY;
                    }
                    self.prev = LatinKorean::Hangul;
                    self.current_word_len += 1;
                    if self.current_word_len > 5 {
                        score += EUC_KR_LONG_WORD_PENALTY;
                    }
                } else if (u >= 0x4E00 && u < 0xAC00) || (u >= 0xF900 && u <= 0xFA0B) {
                    if let Some(pending) = self.pending_score {
                        score += pending;
                        self.pending_score = None;
                    }
                    score += EUC_KR_SCORE_PER_HANJA;
                    match self.prev {
                        LatinKorean::AsciiLetter => {
                            score += CJK_LATIN_ADJACENCY_PENALTY;
                        }
                        LatinKorean::Hangul => {
                            score += EUC_KR_HANJA_AFTER_HANGUL_PENALTY;
                        }
                        _ => {}
                    }
                    self.prev = LatinKorean::Hanja;
                    self.current_word_len += 1;
                    if self.current_word_len > 5 {
                        score += EUC_KR_LONG_WORD_PENALTY;
                    }
                } else {
                    if u >= 0x80 {
                        if let Some(pending) = self.pending_score {
                            score += pending;
                            self.pending_score = None;
                        }
                        score += CJK_OTHER;
                    } else {
                        self.pending_score = None; // Discard pending score
                    }
                    self.prev = LatinKorean::Other;
                    self.current_word_len = 0;
                }
            }
            match result {
                DecoderResult::InputEmpty => {
                    assert_eq!(read, 1);
                }
                DecoderResult::Malformed(malformed_len, _) => {
                    if (self.prev_byte == 0xC9 || self.prev_byte == 0xFE) && b >= 0xA1 && b <= 0xFE
                    {
                        if let Some(pending) = self.pending_score {
                            score += pending;
                            self.pending_score = None;
                        }
                        // The byte pair is in code page 949 EUDC range
                        score += EUC_KR_PUA_PENALTY;
                        // Assume Hanja semantics
                        match self.prev {
                            LatinKorean::AsciiLetter => {
                                score += CJK_LATIN_ADJACENCY_PENALTY;
                            }
                            LatinKorean::Hangul => {
                                score += EUC_KR_HANJA_AFTER_HANGUL_PENALTY;
                            }
                            _ => {}
                        }
                        self.prev = LatinKorean::Hanja;
                        self.current_word_len += 1;
                        if self.current_word_len > 5 {
                            score += EUC_KR_LONG_WORD_PENALTY;
                        }
                    } else if (self.prev_byte == 0xA1
                        || (self.prev_byte >= 0xA3 && self.prev_byte <= 0xA8)
                        || (self.prev_byte >= 0xAA && self.prev_byte <= 0xAD))
                        && (b >= 0x7B && b <= 0x7D)
                    {
                        if let Some(pending) = self.pending_score {
                            score += pending;
                            self.pending_score = None;
                        }
                        // MacKorean symbols in range not part of code page 949
                        score += EUC_KR_MAC_KOREAN_PENALTY;
                        self.prev = LatinKorean::Other;
                        self.current_word_len = 0;
                    } else if (self.prev_byte >= 0x81 && self.prev_byte <= 0x84)
                        && (b <= 0x80 || b == 0xFF)
                    {
                        // MacKorean single-byte that conflicts with code page 949 lead byte
                        // followed by ASCII or a non-conflicting single-byte extension.
                        self.pending_score = None; // Just in case
                        score += EUC_KR_SINGLE_BYTE_EXTENSION_PENALTY;
                        if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') {
                            self.prev = LatinKorean::AsciiLetter;
                        } else if b == 0x80 || b == 0xFF {
                            score += EUC_KR_SINGLE_BYTE_EXTENSION_PENALTY;
                            self.prev = LatinKorean::Other;
                        } else {
                            self.prev = LatinKorean::Other;
                        }
                        self.current_word_len = 0;
                    } else if malformed_len == 1 && (b == 0x80 || b == 0xFF) {
                        // MacKorean single-byte extensions that don't conflict with lead bytes
                        self.pending_score = None; // Just in case
                        score += EUC_KR_SINGLE_BYTE_EXTENSION_PENALTY;
                        self.prev = LatinKorean::Other;
                        self.current_word_len = 0;
                    } else {
                        return None;
                    }
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
            self.prev_was_euc_range = in_euc_range;
            self.prev_byte = b;
        }
        if last {
            let (result, _, _) = self
                .decoder
                .decode_to_utf16_without_replacement(b"", &mut dst, true);
            match result {
                DecoderResult::InputEmpty => {}
                DecoderResult::Malformed(_, _) => {
                    return None;
                }
                DecoderResult::OutputFull => {
                    unreachable!();
                }
            }
        }
        Some(score)
    }
}

enum InnerCandidate {
    Latin(LatinCandidate),
    NonLatinCased(NonLatinCasedCandidate),
    Caseless(CaselessCandidate),
    ArabicFrench(ArabicFrenchCandidate),
    Logical(LogicalCandidate),
    Visual(VisualCandidate),
    Utf8(Utf8Candidate),
    Iso2022(Iso2022Candidate),
    Shift(ShiftJisCandidate),
    EucJp(EucJpCandidate),
    EucKr(EucKrCandidate),
    Big5(Big5Candidate),
    Gbk(GbkCandidate),
}

impl InnerCandidate {
    fn feed(&mut self, buffer: &[u8], last: bool) -> Option<i64> {
        match self {
            InnerCandidate::Latin(c) => {
                if let Some(new_score) = c.feed(buffer) {
                    if last {
                        // Treat EOF as space-like
                        if let Some(additional_score) = c.feed(b" ") {
                            Some(new_score + additional_score)
                        } else {
                            None
                        }
                    } else {
                        Some(new_score)
                    }
                } else {
                    None
                }
            }
            InnerCandidate::NonLatinCased(c) => {
                if let Some(new_score) = c.feed(buffer) {
                    if last {
                        // Treat EOF as space-like
                        if let Some(additional_score) = c.feed(b" ") {
                            Some(new_score + additional_score)
                        } else {
                            None
                        }
                    } else {
                        Some(new_score)
                    }
                } else {
                    None
                }
            }
            InnerCandidate::Caseless(c) => {
                if let Some(new_score) = c.feed(buffer) {
                    if last {
                        // Treat EOF as space-like
                        if let Some(additional_score) = c.feed(b" ") {
                            Some(new_score + additional_score)
                        } else {
                            None
                        }
                    } else {
                        Some(new_score)
                    }
                } else {
                    None
                }
            }
            InnerCandidate::ArabicFrench(c) => {
                if let Some(new_score) = c.feed(buffer) {
                    if last {
                        // Treat EOF as space-like
                        if let Some(additional_score) = c.feed(b" ") {
                            Some(new_score + additional_score)
                        } else {
                            None
                        }
                    } else {
                        Some(new_score)
                    }
                } else {
                    None
                }
            }
            InnerCandidate::Logical(c) => {
                if let Some(new_score) = c.feed(buffer) {
                    if last {
                        // Treat EOF as space-like
                        if let Some(additional_score) = c.feed(b" ") {
                            Some(new_score + additional_score)
                        } else {
                            None
                        }
                    } else {
                        Some(new_score)
                    }
                } else {
                    None
                }
            }
            InnerCandidate::Visual(c) => {
                if let Some(new_score) = c.feed(buffer) {
                    if last {
                        // Treat EOF as space-like
                        if let Some(additional_score) = c.feed(b" ") {
                            Some(new_score + additional_score)
                        } else {
                            None
                        }
                    } else {
                        Some(new_score)
                    }
                } else {
                    None
                }
            }
            InnerCandidate::Utf8(c) => c.feed(buffer, last),
            InnerCandidate::Iso2022(c) => c.feed(buffer, last),
            InnerCandidate::Shift(c) => c.feed(buffer, last),
            InnerCandidate::EucJp(c) => c.feed(buffer, last),
            InnerCandidate::EucKr(c) => c.feed(buffer, last),
            InnerCandidate::Big5(c) => c.feed(buffer, last),
            InnerCandidate::Gbk(c) => c.feed(buffer, last),
        }
    }
}

fn encoding_for_tld(tld: Tld) -> usize {
    match tld {
        Tld::CentralWindows | Tld::CentralCyrillic => EncodingDetector::CENTRAL_WINDOWS_INDEX,
        Tld::Cyrillic => EncodingDetector::CYRILLIC_WINDOWS_INDEX,
        Tld::Generic | Tld::Western | Tld::WesternCyrillic | Tld::WesternArabic | Tld::Eu => {
            EncodingDetector::WESTERN_INDEX
        }
        Tld::IcelandicFaroese => EncodingDetector::ICELANDIC_INDEX,
        Tld::Greek => EncodingDetector::GREEK_ISO_INDEX,
        Tld::TurkishAzeri => EncodingDetector::TURKISH_INDEX,
        Tld::Hebrew => EncodingDetector::LOGICAL_INDEX,
        Tld::Arabic => EncodingDetector::ARABIC_WINDOWS_INDEX,
        Tld::Baltic => EncodingDetector::BALTIC_WINDOWS_INDEX,
        Tld::Vietnamese => EncodingDetector::VIETNAMESE_INDEX,
        Tld::Thai => EncodingDetector::THAI_INDEX,
        Tld::Simplified | Tld::SimplifiedTraditional => EncodingDetector::GBK_INDEX,
        Tld::Traditional | Tld::TraditionalSimplified => EncodingDetector::BIG5_INDEX,
        Tld::Japanese => EncodingDetector::SHIFT_JIS_INDEX,
        Tld::Korean => EncodingDetector::EUC_KR_INDEX,
        Tld::CentralIso => EncodingDetector::CENTRAL_ISO_INDEX,
    }
}

fn encoding_is_native_to_tld(tld: Tld, encoding: usize) -> bool {
    match tld {
        Tld::CentralWindows => encoding == EncodingDetector::CENTRAL_WINDOWS_INDEX,
        Tld::Cyrillic => {
            encoding == EncodingDetector::CYRILLIC_WINDOWS_INDEX
                || encoding == EncodingDetector::CYRILLIC_KOI_INDEX
                || encoding == EncodingDetector::CYRILLIC_IBM_INDEX
                || encoding == EncodingDetector::CYRILLIC_ISO_INDEX
        }
        Tld::Western => encoding == EncodingDetector::WESTERN_INDEX,
        Tld::Greek => {
            encoding == EncodingDetector::GREEK_WINDOWS_INDEX
                || encoding == EncodingDetector::GREEK_ISO_INDEX
        }
        Tld::TurkishAzeri => encoding == EncodingDetector::TURKISH_INDEX,
        Tld::Hebrew => encoding == EncodingDetector::LOGICAL_INDEX,
        Tld::Arabic => {
            encoding == EncodingDetector::ARABIC_WINDOWS_INDEX
                || encoding == EncodingDetector::ARABIC_ISO_INDEX
        }
        Tld::Baltic => {
            encoding == EncodingDetector::BALTIC_WINDOWS_INDEX
                || encoding == EncodingDetector::BALTIC_ISO13_INDEX
                || encoding == EncodingDetector::BALTIC_ISO4_INDEX
        }
        Tld::Vietnamese => encoding == EncodingDetector::VIETNAMESE_INDEX,
        Tld::Thai => encoding == EncodingDetector::THAI_INDEX,
        Tld::Simplified => encoding == EncodingDetector::GBK_INDEX,
        Tld::Traditional => encoding == EncodingDetector::BIG5_INDEX,
        Tld::Japanese => {
            encoding == EncodingDetector::SHIFT_JIS_INDEX
                || encoding == EncodingDetector::EUC_JP_INDEX
        }
        Tld::Korean => encoding == EncodingDetector::EUC_KR_INDEX,
        Tld::SimplifiedTraditional | Tld::TraditionalSimplified => {
            encoding == EncodingDetector::GBK_INDEX || encoding == EncodingDetector::BIG5_INDEX
        }
        Tld::CentralIso => encoding == EncodingDetector::CENTRAL_ISO_INDEX,
        Tld::IcelandicFaroese => encoding == EncodingDetector::ICELANDIC_INDEX,
        Tld::WesternCyrillic => {
            encoding == EncodingDetector::WESTERN_INDEX
                || encoding == EncodingDetector::CYRILLIC_WINDOWS_INDEX
                || encoding == EncodingDetector::CYRILLIC_KOI_INDEX
                || encoding == EncodingDetector::CYRILLIC_IBM_INDEX
                || encoding == EncodingDetector::CYRILLIC_ISO_INDEX
        }
        Tld::CentralCyrillic => {
            encoding == EncodingDetector::CENTRAL_WINDOWS_INDEX
                || encoding == EncodingDetector::CENTRAL_ISO_INDEX
                || encoding == EncodingDetector::CYRILLIC_WINDOWS_INDEX
                || encoding == EncodingDetector::CYRILLIC_KOI_INDEX
                || encoding == EncodingDetector::CYRILLIC_IBM_INDEX
                || encoding == EncodingDetector::CYRILLIC_ISO_INDEX
        }
        Tld::WesternArabic => {
            encoding == EncodingDetector::WESTERN_INDEX
                || encoding == EncodingDetector::ARABIC_WINDOWS_INDEX
                || encoding == EncodingDetector::ARABIC_ISO_INDEX
        }
        Tld::Eu => {
            encoding == EncodingDetector::WESTERN_INDEX
                || encoding == EncodingDetector::ICELANDIC_INDEX
                || encoding == EncodingDetector::CENTRAL_WINDOWS_INDEX
                || encoding == EncodingDetector::CENTRAL_ISO_INDEX
                || encoding == EncodingDetector::CYRILLIC_WINDOWS_INDEX
                || encoding == EncodingDetector::CYRILLIC_KOI_INDEX
                || encoding == EncodingDetector::CYRILLIC_IBM_INDEX
                || encoding == EncodingDetector::CYRILLIC_ISO_INDEX
                || encoding == EncodingDetector::GREEK_WINDOWS_INDEX
                || encoding == EncodingDetector::GREEK_ISO_INDEX
                || encoding == EncodingDetector::BALTIC_WINDOWS_INDEX
                || encoding == EncodingDetector::BALTIC_ISO13_INDEX
                || encoding == EncodingDetector::BALTIC_ISO4_INDEX
        }
        Tld::Generic => false,
    }
}

fn score_adjustment(score: i64, encoding: usize, tld: Tld) -> i64 {
    if score < 1 {
        return 0;
    }
    // This is the most ad hoc part of this library.
    let (divisor, constant) = match tld {
        Tld::Generic => {
            unreachable!();
        }
        Tld::CentralWindows | Tld::CentralIso => {
            match encoding {
                EncodingDetector::WESTERN_INDEX
                | EncodingDetector::ICELANDIC_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::VIETNAMESE_INDEX
                | EncodingDetector::TURKISH_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Cyrillic => {
            match encoding {
                EncodingDetector::BIG5_INDEX
                | EncodingDetector::GBK_INDEX
                | EncodingDetector::EUC_JP_INDEX
                | EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::GREEK_WINDOWS_INDEX
                | EncodingDetector::GREEK_ISO_INDEX
                | EncodingDetector::VISUAL_INDEX
                | EncodingDetector::LOGICAL_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::TURKISH_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Western | Tld::WesternCyrillic | Tld::WesternArabic => {
            match encoding {
                EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::TURKISH_INDEX
                | EncodingDetector::VIETNAMESE_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Greek => {
            match encoding {
                EncodingDetector::BIG5_INDEX
                | EncodingDetector::GBK_INDEX
                | EncodingDetector::EUC_JP_INDEX
                | EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::CYRILLIC_WINDOWS_INDEX
                | EncodingDetector::CYRILLIC_ISO_INDEX
                | EncodingDetector::CYRILLIC_KOI_INDEX
                | EncodingDetector::CYRILLIC_IBM_INDEX
                | EncodingDetector::VISUAL_INDEX
                | EncodingDetector::LOGICAL_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::TURKISH_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::TurkishAzeri => {
            match encoding {
                EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::VIETNAMESE_INDEX
                | EncodingDetector::ICELANDIC_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Hebrew => {
            match encoding {
                EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::CYRILLIC_WINDOWS_INDEX
                | EncodingDetector::CYRILLIC_ISO_INDEX
                | EncodingDetector::CYRILLIC_KOI_INDEX
                | EncodingDetector::CYRILLIC_IBM_INDEX
                | EncodingDetector::GREEK_WINDOWS_INDEX
                | EncodingDetector::GREEK_ISO_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::VIETNAMESE_INDEX
                | EncodingDetector::TURKISH_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Arabic => {
            match encoding {
                EncodingDetector::BIG5_INDEX
                | EncodingDetector::GBK_INDEX
                | EncodingDetector::EUC_JP_INDEX
                | EncodingDetector::EUC_KR_INDEX
                | EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::CYRILLIC_WINDOWS_INDEX
                | EncodingDetector::CYRILLIC_ISO_INDEX
                | EncodingDetector::CYRILLIC_KOI_INDEX
                | EncodingDetector::CYRILLIC_IBM_INDEX
                | EncodingDetector::GREEK_WINDOWS_INDEX
                | EncodingDetector::GREEK_ISO_INDEX
                | EncodingDetector::VISUAL_INDEX
                | EncodingDetector::LOGICAL_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::VIETNAMESE_INDEX
                | EncodingDetector::TURKISH_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Baltic => {
            match encoding {
                EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::ICELANDIC_INDEX
                | EncodingDetector::TURKISH_INDEX
                | EncodingDetector::VIETNAMESE_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Vietnamese => {
            match encoding {
                EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::TURKISH_INDEX
                | EncodingDetector::ICELANDIC_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Thai => {
            match encoding {
                EncodingDetector::BIG5_INDEX
                | EncodingDetector::GBK_INDEX
                | EncodingDetector::EUC_JP_INDEX
                | EncodingDetector::EUC_KR_INDEX
                | EncodingDetector::SHIFT_JIS_INDEX
                | EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::CYRILLIC_WINDOWS_INDEX
                | EncodingDetector::CYRILLIC_ISO_INDEX
                | EncodingDetector::CYRILLIC_KOI_INDEX
                | EncodingDetector::CYRILLIC_IBM_INDEX
                | EncodingDetector::GREEK_WINDOWS_INDEX
                | EncodingDetector::GREEK_ISO_INDEX
                | EncodingDetector::ARABIC_WINDOWS_INDEX
                | EncodingDetector::ARABIC_ISO_INDEX
                | EncodingDetector::VISUAL_INDEX
                | EncodingDetector::LOGICAL_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::TURKISH_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Simplified
        | Tld::Traditional
        | Tld::TraditionalSimplified
        | Tld::SimplifiedTraditional
        | Tld::Japanese
        | Tld::Korean => {
            // If TLD default is valid, everything else scores zero
            return score;
        }
        Tld::IcelandicFaroese => {
            match encoding {
                EncodingDetector::CENTRAL_WINDOWS_INDEX
                | EncodingDetector::CENTRAL_ISO_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::TURKISH_INDEX
                | EncodingDetector::VIETNAMESE_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::CentralCyrillic => {
            match encoding {
                EncodingDetector::BIG5_INDEX
                | EncodingDetector::GBK_INDEX
                | EncodingDetector::EUC_JP_INDEX
                | EncodingDetector::GREEK_WINDOWS_INDEX
                | EncodingDetector::GREEK_ISO_INDEX
                | EncodingDetector::VISUAL_INDEX
                | EncodingDetector::LOGICAL_INDEX
                | EncodingDetector::BALTIC_WINDOWS_INDEX
                | EncodingDetector::BALTIC_ISO4_INDEX
                | EncodingDetector::BALTIC_ISO13_INDEX
                | EncodingDetector::TURKISH_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
        Tld::Eu => {
            match encoding {
                EncodingDetector::BIG5_INDEX
                | EncodingDetector::GBK_INDEX
                | EncodingDetector::EUC_JP_INDEX
                | EncodingDetector::TURKISH_INDEX
                | EncodingDetector::VIETNAMESE_INDEX => {
                    // XXX Tune this better instead of this kind of absolute.
                    return score;
                }
                _ => (50, 60),
            }
        }
    };
    (score / divisor) + constant
}

cfg_if::cfg_if! {
    if #[cfg(feature = "multithreading")] {
        #[repr(align(64))] // Align to cache lines to avoid false sharing in the Rayon case
        struct Candidate {
            inner: InnerCandidate,
            score: Option<i64>,
        }
    } else {
        struct Candidate {
            inner: InnerCandidate,
            score: Option<i64>,
        }
    }
}

impl Candidate {
    fn feed(&mut self, buffer: &[u8], last: bool) {
        if let Some(old_score) = self.score {
            if let Some(new_score) = self.inner.feed(buffer, last) {
                self.score = Some(old_score + new_score);
            } else {
                self.score = None;
            }
        }
    }

    #[cfg(feature = "multithreading")]
    fn qualified(&self) -> bool {
        !self.score.is_none()
    }

    fn new_latin(data: &'static SingleByteData) -> Self {
        Candidate {
            inner: InnerCandidate::Latin(LatinCandidate::new(data)),
            score: Some(0),
        }
    }

    fn new_non_latin_cased(data: &'static SingleByteData) -> Self {
        Candidate {
            inner: InnerCandidate::NonLatinCased(NonLatinCasedCandidate::new(data)),
            score: Some(0),
        }
    }

    fn new_caseless(data: &'static SingleByteData) -> Self {
        Candidate {
            inner: InnerCandidate::Caseless(CaselessCandidate::new(data)),
            score: Some(0),
        }
    }

    fn new_arabic_french(data: &'static SingleByteData) -> Self {
        Candidate {
            inner: InnerCandidate::ArabicFrench(ArabicFrenchCandidate::new(data)),
            score: Some(0),
        }
    }

    fn new_logical(data: &'static SingleByteData) -> Self {
        Candidate {
            inner: InnerCandidate::Logical(LogicalCandidate::new(data)),
            score: Some(0),
        }
    }

    fn new_visual(data: &'static SingleByteData) -> Self {
        Candidate {
            inner: InnerCandidate::Visual(VisualCandidate::new(data)),
            score: Some(0),
        }
    }

    fn new_utf_8() -> Self {
        Candidate {
            inner: InnerCandidate::Utf8(Utf8Candidate {
                decoder: UTF_8.new_decoder_without_bom_handling(),
            }),
            score: Some(0),
        }
    }

    fn new_iso_2022_jp() -> Self {
        Candidate {
            inner: InnerCandidate::Iso2022(Iso2022Candidate {
                decoder: ISO_2022_JP.new_decoder_without_bom_handling(),
            }),
            score: Some(0),
        }
    }

    fn new_shift_jis() -> Self {
        Candidate {
            inner: InnerCandidate::Shift(ShiftJisCandidate {
                decoder: SHIFT_JIS.new_decoder_without_bom_handling(),
                half_width_katakana_seen: false,
                half_width_katakana_state: HalfWidthKatakana::DakutenForbidden,
                prev: LatinCj::Other,
                prev_byte: 0,
                pending_score: None,
            }),
            score: Some(0),
        }
    }

    fn new_euc_jp() -> Self {
        Candidate {
            inner: InnerCandidate::EucJp(EucJpCandidate {
                decoder: EUC_JP.new_decoder_without_bom_handling(),
                non_ascii_seen: false,
                half_width_katakana_state: HalfWidthKatakana::DakutenForbidden,
                prev: LatinCj::Other,
                prev_byte: 0,
                prev_prev_byte: 0,
            }),
            score: Some(0),
        }
    }

    fn new_euc_kr() -> Self {
        Candidate {
            inner: InnerCandidate::EucKr(EucKrCandidate {
                decoder: EUC_KR.new_decoder_without_bom_handling(),
                prev_byte: 0,
                prev_was_euc_range: false,
                prev: LatinKorean::Other,
                current_word_len: 0,
                pending_score: None,
            }),
            score: Some(0),
        }
    }

    fn new_big5() -> Self {
        Candidate {
            inner: InnerCandidate::Big5(Big5Candidate {
                decoder: BIG5.new_decoder_without_bom_handling(),
                prev: LatinCj::Other,
                prev_byte: 0,
                pending_score: None,
            }),
            score: Some(0),
        }
    }

    fn new_gbk() -> Self {
        Candidate {
            inner: InnerCandidate::Gbk(GbkCandidate {
                decoder: GBK.new_decoder_without_bom_handling(),
                prev: LatinCj::Other,
                prev_byte: 0,
                pending_score: None,
            }),
            score: Some(0),
        }
    }

    fn score(&self, encoding: usize, tld: Tld, expectation_is_valid: bool) -> Option<i64> {
        match &self.inner {
            InnerCandidate::NonLatinCased(c) => {
                if c.longest_word < 2 {
                    return None;
                }
            }
            InnerCandidate::Caseless(c) => {
                if c.longest_word < 2 && !encoding_is_native_to_tld(tld, encoding) {
                    return None;
                }
            }
            InnerCandidate::ArabicFrench(c) => {
                if c.longest_word < 2 && !encoding_is_native_to_tld(tld, encoding) {
                    return None;
                }
            }
            InnerCandidate::Logical(c) => {
                if c.longest_word < 2 && !encoding_is_native_to_tld(tld, encoding) {
                    return None;
                }
            }
            InnerCandidate::Visual(c) => {
                if c.longest_word < 2 && !encoding_is_native_to_tld(tld, encoding) {
                    return None;
                }
            }
            _ => {}
        }
        if tld == Tld::Generic {
            return self.score;
        }
        if let Some(score) = self.score {
            if encoding == encoding_for_tld(tld) {
                return Some(score + 1);
            }
            if encoding_is_native_to_tld(tld, encoding) {
                return Some(score);
            }
            if expectation_is_valid {
                return Some(score - score_adjustment(score, encoding, tld));
            }
            // If expectation is no longer valid, fall back to
            // generic behavior.
            // XXX Flipped Chinese and Central
            return Some(score);
        }
        None
    }

    fn plausible_punctuation(&self) -> u64 {
        match &self.inner {
            InnerCandidate::Logical(c) => {
                return c.plausible_punctuation;
            }
            InnerCandidate::Visual(c) => {
                return c.plausible_punctuation;
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn encoding(&self) -> &'static Encoding {
        match &self.inner {
            InnerCandidate::Latin(c) => {
                return c.data.encoding;
            }
            InnerCandidate::NonLatinCased(c) => {
                return c.data.encoding;
            }
            InnerCandidate::Caseless(c) => {
                return c.data.encoding;
            }
            InnerCandidate::ArabicFrench(c) => {
                return c.data.encoding;
            }
            InnerCandidate::Logical(c) => {
                return c.data.encoding;
            }
            InnerCandidate::Visual(c) => {
                return c.data.encoding;
            }
            InnerCandidate::Shift(_) => {
                return SHIFT_JIS;
            }
            InnerCandidate::EucJp(_) => {
                return EUC_JP;
            }
            InnerCandidate::Big5(_) => {
                return BIG5;
            }
            InnerCandidate::EucKr(_) => {
                return EUC_KR;
            }
            InnerCandidate::Gbk(_) => {
                return GBK;
            }
            InnerCandidate::Utf8(_) => {
                return UTF_8;
            }
            InnerCandidate::Iso2022(_) => {
                return ISO_2022_JP;
            }
        }
    }
}

// LLVM doesn't autovectorize this properly for SSE2, so let's help manually.
cfg_if::cfg_if! {
    if #[cfg(target_feature = "sse2")] {
        fn count_non_ascii(buffer: &[u8]) -> u64 {
            let mut count = 0;
            let (prefix, simd, suffix) = unsafe { buffer.align_to::<__m128i>() };
            for &b in prefix {
                if b >= 0x80 {
                    count += 1;
                }
            }
            for &s in simd {
                count += unsafe {_mm_movemask_epi8(s)}.count_ones() as u64;
            }
            for &b in suffix {
                if b >= 0x80 {
                    count += 1;
                }
            }
            count
        }
    } else {
        fn count_non_ascii(buffer: &[u8]) -> u64 {
            let mut count = 0;
            for &b in buffer {
                if b >= 0x80 {
                    count += 1;
                }
            }
            count
        }
    }
}

#[derive(Clone, Copy)]
enum BeforeNonAscii {
    None,
    One([u8; 1]),
    Two([u8; 2]),
}

impl BeforeNonAscii {
    fn as_slice(&self) -> &[u8] {
        match self {
            BeforeNonAscii::None => b"",
            BeforeNonAscii::One(arr) => &arr[..],
            BeforeNonAscii::Two(arr) => &arr[..],
        }
    }

    fn push(&mut self, buffer: &[u8]) {
        let len = buffer.len();
        if len >= 2 {
            let arr = [buffer[len - 2], buffer[len - 1]];
            *self = BeforeNonAscii::Two(arr);
        } else if len == 1 {
            match self {
                BeforeNonAscii::None => {
                    let arr = [buffer[0]];
                    *self = BeforeNonAscii::One(arr);
                }
                BeforeNonAscii::One(first) => {
                    let arr = [first[0], buffer[0]];
                    *self = BeforeNonAscii::Two(arr);
                }
                BeforeNonAscii::Two(first) => {
                    let arr = [first[1], buffer[0]];
                    *self = BeforeNonAscii::Two(arr);
                }
            }
        }
    }
}

/// A Web browser-oriented detector for guessing what character
/// encoding a stream of bytes is encoded in.
///
/// The bytes are fed to the detector incrementally using the `feed`
/// method. The current guess of the detector can be queried using
/// the `guess` method. The guessing parameters are arguments to the
/// `guess` method rather than arguments to the constructor in order
/// to enable the application to check if the arguments affect the
/// guessing outcome. (The specific use case is to disable UI for
/// re-running the detector with UTF-8 allowed and the top-level
/// domain name ignored if those arguments don't change the guess.)
pub struct EncodingDetector {
    candidates: [Candidate; 27],
    non_ascii_seen: u64,
    // We need to feed up to two bytes of context before non-ASCII
    // thanks to Spanish n.º.
    last_before_non_ascii: BeforeNonAscii,
    esc_seen: bool,
    closed: bool,
}

impl EncodingDetector {
    cfg_if::cfg_if! {
        if #[cfg(feature = "multithreading")] {
            fn feed_impl(&mut self, buffer: &[u8], last: bool) {
                if buffer.len() < 10 {
                    self.candidates.iter_mut().for_each(|candidate| candidate.feed(buffer, last));
                    self.non_ascii_seen += count_non_ascii(buffer);
                    return;
                }
                // Collect only qualified candidates to avoid Rayon
                // performing thread synchronization only to bail
                // out immediately when trying a disqualified
                // candidate.
                let mut qualified = ArrayVec::<[_; 27]>::new();
                for candidate in self.candidates.iter_mut() {
                    if candidate.qualified() {
                        qualified.push(candidate);
                    }
                }
                let (_, non_ascii) = rayon::join(|| qualified.par_iter_mut().for_each(|candidate| candidate.feed(buffer, last)),
                                                 || count_non_ascii(buffer));
                self.non_ascii_seen += non_ascii;
            }
        } else {
            fn feed_impl(&mut self, buffer: &[u8], last: bool) {
                self.candidates.iter_mut().for_each(|candidate| candidate.feed(buffer, last));
                self.non_ascii_seen += count_non_ascii(buffer);
            }
        }
    }

    /// Inform the detector of a chunk of input.
    ///
    /// The byte stream is represented as a sequence of calls to this
    /// method such that the concatenation of the arguments to this
    /// method form the byte stream. It does not matter how the application
    /// chooses to chunk the stream. It is OK to call this method with
    /// a zero-length byte slice.
    ///
    /// The end of the stream is indicated by calling this method with
    /// `last` set to `true`. In that case, the end of the stream is
    /// considered to occur after the last byte of the `buffer` (which
    /// may be zero-length) passed in the same call. Once this method
    /// has been called with `last` set to `true` this method must not
    /// be called again.
    ///
    /// If you want to perform detection on just the prefix of a longer
    /// stream, do not pass `last=true` after the prefix if the stream
    /// actually still continues.
    ///
    /// Returns `true` if after processing `buffer` the stream has
    /// contained at least one non-ASCII byte and `false` if only
    /// ASCII has been seen so far.
    ///
    /// # Panics
    ///
    /// If this method has previously been called with `last` set to `true`.
    pub fn feed(&mut self, buffer: &[u8], last: bool) -> bool {
        assert!(
            !self.closed,
            "Must not feed again after feeding with last equaling true."
        );
        if last {
            self.closed = true;
        }
        let start = if self.non_ascii_seen == 0 && !self.esc_seen {
            let up_to = Encoding::ascii_valid_up_to(buffer);
            let start = if let Some(escape) = memchr::memchr(0x1B, &buffer[..up_to]) {
                self.esc_seen = true;
                escape
            } else {
                up_to
            };
            if start == buffer.len() {
                self.last_before_non_ascii.push(buffer);
                return self.non_ascii_seen != 0;
            }
            if start == 0 || start == 1 {
                let last_before = self.last_before_non_ascii;
                self.last_before_non_ascii = BeforeNonAscii::None;
                self.feed_impl(last_before.as_slice(), false);
                0
            } else {
                start - 2
            }
        } else {
            0
        };
        self.feed_impl(&buffer[start..], last);
        self.non_ascii_seen != 0
    }

    /// Guess the encoding given the bytes pushed to the detector so far
    /// (via `feed()`), the top-level domain name from which the bytes were
    /// loaded, and an indication of whether to consider UTF-8 as a permissible
    /// guess.
    ///
    /// The `tld` argument takes the rightmost DNS label of the hostname of the
    /// host the stream was loaded from in lower-case ASCII form. That is, if
    /// the label is an internationalized top-level domain name, it must be
    /// provided in its Punycode form. If the TLD that the stream was loaded
    /// from is unavalable, `None` may be passed instead, which is equivalent
    /// to passing `Some(b"com")`.
    ///
    /// If the `allow_utf8` argument is set to `false`, the return value of
    /// this method won't be `encoding_rs::UTF_8`. When performing detection
    /// on `text/html` on non-`file:` URLs, Web browsers must pass `false`,
    /// unless the user has taken a specific contextual action to request an
    /// override. This way, Web developers cannot start depending on UTF-8
    /// detection. Such reliance would make the Web Platform more brittle.
    ///
    /// Returns the guessed encoding.
    ///
    /// # Panics
    ///
    /// If `tld` contains non-ASCII, period, or upper-case letters. (The panic
    /// condition is intentionally limited to signs of failing to extract the
    /// label correctly, failing to provide it in its Punycode form, and failure
    /// to lower-case it. Full DNS label validation is intentionally not performed
    /// to avoid panics when the reality doesn't match the specs.)
    pub fn guess(&self, tld: Option<&[u8]>, allow_utf8: bool) -> &'static Encoding {
        self.guess_assess(tld, allow_utf8).0
    }

    /// Same as `guess()`, but also returns a Boolean indicating
    /// whether the guessed encoding had a higher score than at least
    /// one other candidate. If this method returns `false`, the
    /// guessed encoding is likely to be wrong.
    pub fn guess_assess(&self, tld: Option<&[u8]>, allow_utf8: bool) -> (&'static Encoding, bool) {
        let mut tld_type = tld.map_or(Tld::Generic, |tld| {
            assert!(!contains_upper_case_period_or_non_ascii(tld));
            classify_tld(tld)
        });

        if self.non_ascii_seen == 0
            && self.esc_seen
            && self.candidates[Self::ISO_2022_JP_INDEX].score.is_some()
        {
            return (ISO_2022_JP, true);
        }

        if self.candidates[Self::UTF_8_INDEX].score.is_some() {
            if allow_utf8 {
                return (UTF_8, true);
            }
            // Various test cases that prohibit UTF-8 detection want to
            // see windows-1252 specifically. These tests run on generic
            // domains. However, if we returned windows-1252 on
            // some non-generic domains, we'd cause reloads.
            return (self.candidates[encoding_for_tld(tld_type)].encoding(), true);
        }

        let mut encoding = self.candidates[encoding_for_tld(tld_type)].encoding();
        let mut max = 0i64;
        let mut expectation_is_valid = false;
        if tld_type != Tld::Generic {
            for (i, candidate) in self.candidates.iter().enumerate().skip(Self::FIRST_NORMAL) {
                if encoding_is_native_to_tld(tld_type, i) && candidate.score.is_some() {
                    expectation_is_valid = true;
                    break;
                }
            }
        }
        if !expectation_is_valid {
            // Flip Chinese and Central around
            match tld_type {
                Tld::Simplified => {
                    if self.candidates[Self::BIG5_INDEX].score.is_some() {
                        tld_type = Tld::Traditional;
                        expectation_is_valid = true;
                    }
                }
                Tld::Traditional => {
                    if self.candidates[Self::GBK_INDEX].score.is_some() {
                        tld_type = Tld::Simplified;
                        expectation_is_valid = true;
                    }
                }
                Tld::CentralWindows => {
                    if self.candidates[Self::CENTRAL_ISO_INDEX].score.is_some() {
                        tld_type = Tld::CentralIso;
                        expectation_is_valid = true;
                    }
                }
                Tld::CentralIso => {
                    if self.candidates[Self::CENTRAL_WINDOWS_INDEX].score.is_some() {
                        tld_type = Tld::CentralWindows;
                        expectation_is_valid = true;
                    }
                }
                _ => {}
            }
        }
        for (i, candidate) in self.candidates.iter().enumerate().skip(Self::FIRST_NORMAL) {
            if let Some(score) = candidate.score(i, tld_type, expectation_is_valid) {
                if score > max {
                    max = score;
                    encoding = candidate.encoding();
                }
            }
        }
        let visual = &self.candidates[Self::VISUAL_INDEX];
        if let Some(visual_score) = visual.score(Self::VISUAL_INDEX, tld_type, expectation_is_valid)
        {
            if (visual_score > max || encoding == WINDOWS_1255)
                && visual.plausible_punctuation()
                    > self.candidates[Self::LOGICAL_INDEX].plausible_punctuation()
            {
                // max = visual_score;
                encoding = ISO_8859_8;
            }
        }
        (encoding, max >= 0)
    }

    // XXX Test-only API
    #[cfg(feature = "testing-only-no-semver-guarantees-do-not-use")]
    pub fn find_score(&self, encoding: &'static Encoding) -> Option<i64> {
        let mut tld_type = Tld::Generic;
        let mut expectation_is_valid = false;
        if tld_type != Tld::Generic {
            for (i, candidate) in self.candidates.iter().enumerate().skip(Self::FIRST_NORMAL) {
                if encoding_is_native_to_tld(tld_type, i) && candidate.score.is_some() {
                    expectation_is_valid = true;
                    break;
                }
            }
        }
        if !expectation_is_valid {
            // Flip Chinese and Central around
            match tld_type {
                Tld::Simplified => {
                    if self.candidates[Self::BIG5_INDEX].score.is_some() {
                        tld_type = Tld::Traditional;
                        expectation_is_valid = true;
                    }
                }
                Tld::Traditional => {
                    if self.candidates[Self::GBK_INDEX].score.is_some() {
                        tld_type = Tld::Simplified;
                        expectation_is_valid = true;
                    }
                }
                Tld::CentralWindows => {
                    if self.candidates[Self::CENTRAL_ISO_INDEX].score.is_some() {
                        tld_type = Tld::CentralIso;
                        expectation_is_valid = true;
                    }
                }
                Tld::CentralIso => {
                    if self.candidates[Self::CENTRAL_WINDOWS_INDEX].score.is_some() {
                        tld_type = Tld::CentralWindows;
                        expectation_is_valid = true;
                    }
                }
                _ => {}
            }
        }
        for (i, candidate) in self.candidates.iter().enumerate() {
            if encoding == candidate.encoding() {
                return candidate.score(i, tld_type, expectation_is_valid);
            }
        }
        Some(0)
    }

    const FIRST_NORMAL: usize = 3;

    const UTF_8_INDEX: usize = 0;

    const ISO_2022_JP_INDEX: usize = 1;

    const VISUAL_INDEX: usize = 2;

    const GBK_INDEX: usize = 3;

    const EUC_JP_INDEX: usize = 4;

    const EUC_KR_INDEX: usize = 5;

    const SHIFT_JIS_INDEX: usize = 6;

    const BIG5_INDEX: usize = 7;

    const WESTERN_INDEX: usize = 8;

    const CYRILLIC_WINDOWS_INDEX: usize = 9;

    const CENTRAL_WINDOWS_INDEX: usize = 10;

    const CENTRAL_ISO_INDEX: usize = 11;

    const ARABIC_WINDOWS_INDEX: usize = 12;

    const ICELANDIC_INDEX: usize = 13;

    const TURKISH_INDEX: usize = 14;

    const THAI_INDEX: usize = 15;

    const LOGICAL_INDEX: usize = 16;

    const GREEK_WINDOWS_INDEX: usize = 17;

    const GREEK_ISO_INDEX: usize = 18;

    const BALTIC_WINDOWS_INDEX: usize = 19;

    const BALTIC_ISO13_INDEX: usize = 20;

    const CYRILLIC_KOI_INDEX: usize = 21;

    const CYRILLIC_IBM_INDEX: usize = 22;

    const ARABIC_ISO_INDEX: usize = 23;

    const VIETNAMESE_INDEX: usize = 24;

    const BALTIC_ISO4_INDEX: usize = 25;

    const CYRILLIC_ISO_INDEX: usize = 26;

    /// Creates a new instance of the detector.
    pub fn new() -> Self {
        EncodingDetector {
            candidates: [
                Candidate::new_utf_8(),                                                // 0
                Candidate::new_iso_2022_jp(),                                          // 1
                Candidate::new_visual(&SINGLE_BYTE_DATA[ISO_8859_8_INDEX]),            // 2
                Candidate::new_gbk(),                                                  // 3
                Candidate::new_euc_jp(),                                               // 4
                Candidate::new_euc_kr(),                                               // 5
                Candidate::new_shift_jis(),                                            // 6
                Candidate::new_big5(),                                                 // 7
                Candidate::new_latin(&SINGLE_BYTE_DATA[WINDOWS_1252_INDEX]),           // 8
                Candidate::new_non_latin_cased(&SINGLE_BYTE_DATA[WINDOWS_1251_INDEX]), // 9
                Candidate::new_latin(&SINGLE_BYTE_DATA[WINDOWS_1250_INDEX]),           // 10
                Candidate::new_latin(&SINGLE_BYTE_DATA[ISO_8859_2_INDEX]),             // 11
                Candidate::new_arabic_french(&SINGLE_BYTE_DATA[WINDOWS_1256_INDEX]),   // 12
                Candidate::new_latin(&SINGLE_BYTE_DATA[WINDOWS_1252_ICELANDIC_INDEX]), // 13
                Candidate::new_latin(&SINGLE_BYTE_DATA[WINDOWS_1254_INDEX]),           // 14
                Candidate::new_caseless(&SINGLE_BYTE_DATA[WINDOWS_874_INDEX]),         // 15
                Candidate::new_logical(&SINGLE_BYTE_DATA[WINDOWS_1255_INDEX]),         // 16
                Candidate::new_non_latin_cased(&SINGLE_BYTE_DATA[WINDOWS_1253_INDEX]), // 17
                Candidate::new_non_latin_cased(&SINGLE_BYTE_DATA[ISO_8859_7_INDEX]),   // 18
                Candidate::new_latin(&SINGLE_BYTE_DATA[WINDOWS_1257_INDEX]),           // 19
                Candidate::new_latin(&SINGLE_BYTE_DATA[ISO_8859_13_INDEX]),            // 20
                Candidate::new_non_latin_cased(&SINGLE_BYTE_DATA[KOI8_U_INDEX]),       // 21
                Candidate::new_non_latin_cased(&SINGLE_BYTE_DATA[IBM866_INDEX]),       // 22
                Candidate::new_caseless(&SINGLE_BYTE_DATA[ISO_8859_6_INDEX]),          // 23
                Candidate::new_latin(&SINGLE_BYTE_DATA[WINDOWS_1258_INDEX]),           // 24
                Candidate::new_latin(&SINGLE_BYTE_DATA[ISO_8859_4_INDEX]),             // 25
                Candidate::new_non_latin_cased(&SINGLE_BYTE_DATA[ISO_8859_5_INDEX]),   // 26
            ],
            non_ascii_seen: 0,
            last_before_non_ascii: BeforeNonAscii::None,
            esc_seen: false,
            closed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use detone::IterDecomposeVietnamese;
    use encoding_rs::IBM866;
    use encoding_rs::ISO_8859_2;
    use encoding_rs::ISO_8859_4;
    use encoding_rs::ISO_8859_5;
    use encoding_rs::ISO_8859_6;
    use encoding_rs::ISO_8859_7;
    use encoding_rs::KOI8_U;
    use encoding_rs::WINDOWS_1250;
    use encoding_rs::WINDOWS_1251;
    use encoding_rs::WINDOWS_1252;
    use encoding_rs::WINDOWS_1253;
    use encoding_rs::WINDOWS_1254;
    use encoding_rs::WINDOWS_1256;
    use encoding_rs::WINDOWS_1257;
    use encoding_rs::WINDOWS_1258;
    use encoding_rs::WINDOWS_874;

    fn check_bytes(bytes: &[u8], encoding: &'static Encoding) {
        let mut det = EncodingDetector::new();
        det.feed(bytes, true);
        let enc = det.guess(None, false);
        let (decoded, _) = enc.decode_without_bom_handling(bytes);
        println!("{:?}", decoded);
        assert_eq!(enc, encoding);
    }

    fn check(input: &str, encoding: &'static Encoding) {
        let orthographic;
        let (bytes, _, _) = if encoding == WINDOWS_1258 {
            orthographic = input
                .chars()
                .decompose_vietnamese_tones(true)
                .collect::<String>();
            encoding.encode(&orthographic)
        } else {
            encoding.encode(input)
        };
        check_bytes(&bytes, encoding);
    }

    #[test]
    fn test_i_apostrophe() {
        let mut det = EncodingDetector::new();
        det.feed(b"I\x92", true);
        let enc = det.guess(None, false);
        assert_eq!(enc, WINDOWS_1252);
    }

    #[test]
    fn test_streaming_numero_one_by_one() {
        let mut det = EncodingDetector::new();
        det.feed(b"n", false);
        det.feed(b".", false);
        det.feed(b"\xBA", false);
        det.feed(b"1", true);
        let enc = det.guess(None, false);
        assert_eq!(enc, WINDOWS_1252);
    }

    #[test]
    fn test_streaming_numero_two_together() {
        let mut det = EncodingDetector::new();
        det.feed(b"n.", false);
        det.feed(b"\xBA", false);
        det.feed(b"1", true);
        let enc = det.guess(None, false);
        assert_eq!(enc, WINDOWS_1252);
    }

    #[test]
    fn test_streaming_numero_one_by_one_extra_before() {
        let mut det = EncodingDetector::new();
        det.feed(b" n", false);
        det.feed(b".", false);
        det.feed(b"\xBA", false);
        det.feed(b"1", true);
        let enc = det.guess(None, false);
        assert_eq!(enc, WINDOWS_1252);
    }

    #[test]
    fn test_streaming_numero_one_before() {
        let mut det = EncodingDetector::new();
        det.feed(b"n", false);
        det.feed(b".\xBA", false);
        det.feed(b"1", true);
        let enc = det.guess(None, false);
        assert_eq!(enc, WINDOWS_1252);
    }

    #[test]
    fn test_streaming_numero_longer_first_buffer() {
        let mut det = EncodingDetector::new();
        det.feed(b"rrn.", false);
        det.feed(b"\xBA", false);
        det.feed(b"1", true);
        let enc = det.guess(None, false);
        assert_eq!(enc, WINDOWS_1252);
    }

    #[test]
    fn test_empty() {
        let mut det = EncodingDetector::new();
        let seen_non_ascii = det.feed(b"", true);
        let enc = det.guess(None, false);
        assert_eq!(enc, WINDOWS_1252);
        assert!(!seen_non_ascii);
    }

    #[test]
    fn test_fi() {
        check("Ääni", WINDOWS_1252);
    }

    #[test]
    fn test_fi_bis() {
        check("Tämä", WINDOWS_1252);
    }

    #[test]
    fn test_pt() {
        check(
            "Este é um teste de codificação de caracteres.",
            WINDOWS_1252,
        );
    }

    #[test]
    fn test_is() {
        check("Þetta er kóðunarpróf á staf. Fyrir sum tungumál sem nota latneska stafi þurfum við meira inntak til að taka ákvörðunina.", WINDOWS_1252);
    }

    #[test]
    fn test_ru_short() {
        check("Русский", WINDOWS_1251);
    }

    #[test]
    fn test_ru() {
        check("Это тест кодировки символов.", WINDOWS_1251);
    }

    #[test]
    fn test_ru_iso() {
        check("Это тест кодировки символов.", ISO_8859_5);
    }

    #[test]
    fn test_ru_ibm() {
        check("Это тест кодировки символов.", IBM866);
    }

    #[test]
    fn test_ru_koi() {
        check("Это тест кодировки символов.", KOI8_U);
    }

    #[test]
    fn test_uk() {
        check("Це тест на кодування символів.", WINDOWS_1251);
    }

    #[test]
    fn test_uk_koi() {
        check("Це тест на кодування символів.", KOI8_U);
    }

    #[test]
    fn test_el_short() {
        check("Ελληνικά", WINDOWS_1253);
    }

    #[test]
    fn test_el() {
        check(
            "Πρόκειται για δοκιμή κωδικοποίησης χαρακτήρων: Άρης",
            WINDOWS_1253,
        );
    }

    #[test]
    fn test_el_iso() {
        check(
            "Πρόκειται για δοκιμή κωδικοποίησης χαρακτήρων: Άρης",
            ISO_8859_7,
        );
    }

    #[test]
    fn test_de() {
        check("Straße", WINDOWS_1252);
    }

    #[test]
    fn test_en_windows1252() {
        // "Don't "
        check_bytes(&[68, 111, 110, 180, 116, 32], WINDOWS_1252);
    }

    #[test]
    fn test_he() {
        check("\u{5E2}\u{5D1}\u{5E8}\u{5D9}\u{5EA}", WINDOWS_1255);
    }

    #[test]
    fn test_2022() {
        check("日本語", ISO_2022_JP);
    }

    #[test]
    fn test_th() {
        check("นี่คือการทดสอบการเข้ารหัสอักขระ", WINDOWS_874);
    }

    #[test]
    fn test_vi() {
        check("Đây là một thử nghiệm mã hóa ký tự.", WINDOWS_1258);
    }

    #[test]
    fn test_tr() {
        check("Bu bir karakter kodlama testidir. Latince karakterleri kullanan bazı dillerde karar vermek için daha fazla girdiye ihtiyacımız var.", WINDOWS_1254);
    }

    #[test]
    fn test_simplified() {
        check("这是一个字符编码测试。", GBK);
    }

    #[test]
    fn test_traditional() {
        check("這是一個字符編碼測試。", BIG5);
    }

    #[test]
    fn test_ko() {
        check("이것은 문자 인코딩 테스트입니다.", EUC_KR);
    }

    #[test]
    fn test_shift() {
        check("これは文字実験です。", SHIFT_JIS);
    }

    #[test]
    fn test_euc() {
        check("これは文字実験です。", EUC_JP);
    }

    #[test]
    fn test_ar() {
        check("هذا هو اختبار ترميز الأحرف.", WINDOWS_1256);
    }

    #[test]
    fn test_ar_iso() {
        check("هذا هو اختبار ترميز الأحرف.", ISO_8859_6);
    }

    #[test]
    fn test_fa() {
        check("این یک تست رمزگذاری کاراکتر است.", WINDOWS_1256);
    }

    #[test]
    fn test_visual() {
        check(".םיוות דודיק ןחבמ והז", ISO_8859_8);
    }

    #[test]
    fn test_yi() {
        check("דאָס איז אַ טעסט פֿאַר קאָדירונג פון כאַראַקטער.", WINDOWS_1255);
    }

    #[test]
    fn test_it() {
        check("è", WINDOWS_1252);
    }

    #[test]
    fn test_en() {
        check("isn’t", WINDOWS_1252);
    }

    #[test]
    fn test_en_bis() {
        check("Rock ’n Roll", WINDOWS_1252);
    }

    #[test]
    fn test_ca() {
        check("Codificació de caràcters", WINDOWS_1252);
    }

    #[test]
    fn test_et() {
        check("või", WINDOWS_1252);
    }

    #[test]
    fn test_pl_iso() {
        check("To jest test kodowania znaków. W przypadku niektórych języków, które używają znaków łacińskich, potrzebujemy więcej danych, aby podjąć decyzję.", ISO_8859_2);
    }

    #[test]
    fn test_pl() {
        check("To jest test kodowania znaków. W przypadku niektórych języków, które używają znaków łacińskich, potrzebujemy więcej danych, aby podjąć decyzję.", WINDOWS_1250);
    }

    #[test]
    fn test_lt() {
        check("Tai simbolių kodavimo testas. Kai kurioms kalboms, naudojančioms lotyniškus rašmenis, mums reikia daugiau informacijos, kad galėtume priimti sprendimą.", WINDOWS_1257);
    }

    // TODO: Detected as ISO-8859-2.
    // #[test]
    // fn test_lt_windows_iso_8859_4() {
    //     check("Tai simbolių kodavimo testas. Kai kurioms kalboms, naudojančioms lotyniškus rašmenis, mums reikia daugiau informacijos, kad galėtume priimti sprendimą.", ISO_8859_4);
    // }

    #[test]
    fn test_lv() {
        check("Šis ir rakstzīmju kodēšanas tests. Dažās valodās, kurās tiek izmantotas latīņu valodas burti, lēmuma pieņemšanai mums ir nepieciešams vairāk ieguldījuma.", WINDOWS_1257);
    }

    #[test]
    fn test_lv_iso_8859_4() {
        check("Šis ir rakstzīmju kodēšanas tests. Dažās valodās, kurās tiek izmantotas latīņu valodas burti, lēmuma pieņemšanai mums ir nepieciešams vairāk ieguldījuma.", ISO_8859_4);
    }

    #[test]
    fn test_a0() {
        // Test that this isn't IBM866. TODO: What about GBK with fully paired 0xA0?
        check("\u{A0}\u{A0} \u{A0}", WINDOWS_1252);
    }

    #[test]
    fn test_a0a0() {
        // Test that this isn't GBK or EUC-KR.
        check("\u{A0}\u{A0}", WINDOWS_1252);
    }

    #[test]
    fn test_space_copyright_space() {
        check(" © ", WINDOWS_1252);
    }

    #[test]
    fn test_space_masculine_space() {
        check(" º ", WINDOWS_1252);
    }

    #[test]
    fn test_space_feminine_space() {
        check(" ª ", WINDOWS_1252);
    }

    #[test]
    fn test_period_masculine_space() {
        check(".º ", WINDOWS_1252);
    }

    #[test]
    fn test_period_feminine_space() {
        check(".ª ", WINDOWS_1252);
    }

    #[test]
    fn test_maria() {
        check(" Mª ", WINDOWS_1252);
    }

    #[test]
    fn test_dona() {
        check(" Dª ", WINDOWS_1252);
    }

    #[test]
    fn test_nuestra() {
        check(" Nª ", WINDOWS_1252);
    }

    #[test]
    fn test_senora() {
        check(" Sª ", WINDOWS_1252);
    }

    #[test]
    fn test_digit_feminine() {
        check(" 42ª ", WINDOWS_1252);
    }

    #[test]
    fn test_digit_masculine() {
        check(" 42º ", WINDOWS_1252);
    }

    #[test]
    fn test_roman_feminine() {
        check(" XIVª ", WINDOWS_1252);
    }

    #[test]
    fn test_roman_masculine() {
        check(" XIVº ", WINDOWS_1252);
    }

    #[test]
    fn test_numero_uno() {
        check("Nº1", WINDOWS_1252);
    }

    #[test]
    fn test_numero() {
        check("Nº", WINDOWS_1252);
    }

    #[test]
    fn test_euro() {
        check(" €9", WINDOWS_1252);
    }

    #[test]
    fn test_shift_jis_half_width_katakana() {
        check("ﾊｰﾄﾞｳｪｱﾊｰﾄﾞｳｪｱﾊｰﾄﾞｳｪｱﾊｰﾄﾞｳｪｱﾊｰﾄﾞｳｪｱ", SHIFT_JIS);
    }

    #[test]
    fn test_big5_pua() {
        let mut v = Vec::new();
        for _ in 0..40 {
            v.extend_from_slice(b"\xA4\x40");
        }
        v.extend_from_slice(b"\x81\x40\xA4\x40");
        check_bytes(&v, BIG5);
    }

    #[test]
    fn test_big5_single_byte_a0() {
        let mut v = Vec::new();
        for _ in 0..80 {
            v.extend_from_slice(b"\xA4\x40");
        }
        v.extend_from_slice(b"\x81\x40\xA0 ");
        check_bytes(&v, BIG5);
    }

    #[test]
    fn test_big5_single_byte_ff() {
        let mut v = Vec::new();
        for _ in 0..80 {
            v.extend_from_slice(b"\xA4\x40");
        }
        v.extend_from_slice(b"\x81\x40\xFF ");
        check_bytes(&v, BIG5);
    }

    #[test]
    fn test_not_big5() {
        let mut v = Vec::new();
        for _ in 0..40 {
            v.extend_from_slice(b"\xA4\x40");
        }
        v.extend_from_slice(b"\x81\x40\xA0\xA0");
        check_bytes(&v, IBM866);
    }

    #[test]
    fn test_euc_kr_pua() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xC9\xA1\xB0\xA1 ");
        for _ in 0..40 {
            v.extend_from_slice(b"\xC5\xD7\xBD\xBA\xC6\xAE. ");
        }
        check_bytes(&v, EUC_KR);
    }

    #[test]
    fn test_euc_kr_pua_bis() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xFE\xA1\xB0\xA1 ");
        for _ in 0..40 {
            v.extend_from_slice(b"\xC5\xD7\xBD\xBA\xC6\xAE. ");
        }
        check_bytes(&v, EUC_KR);
    }

    #[test]
    fn test_euc_kr_single_byte_ff() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xFF ");
        for _ in 0..40 {
            v.extend_from_slice(b"\xC5\xD7\xBD\xBA\xC6\xAE. ");
        }
        check_bytes(&v, EUC_KR);
    }

    #[test]
    fn test_euc_kr_single_byte_81() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\x81 ");
        for _ in 0..40 {
            v.extend_from_slice(b"\xC5\xD7\xBD\xBA\xC6\xAE. ");
        }
        check_bytes(&v, EUC_KR);
    }

    #[test]
    fn test_euc_kr_single_byte_84() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\x84 ");
        for _ in 0..40 {
            v.extend_from_slice(b"\xC5\xD7\xBD\xBA\xC6\xAE. ");
        }
        check_bytes(&v, EUC_KR);
    }

    #[test]
    fn test_not_euc_kr() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xC9\xA0\xB0\xA1 ");
        for _ in 0..40 {
            v.extend_from_slice(b"\xC5\xD7\xBD\xBA\xC6\xAE. ");
        }
        check_bytes(&v, GBK);
    }

    #[test]
    fn test_shift_jis_x0213() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\x87\xE5");
        for _ in 0..40 {
            v.extend_from_slice(b"\x82\xC9\x82\xD9\x82\xF1\x82\xB2");
        }
        check_bytes(&v, SHIFT_JIS);
    }

    #[test]
    fn test_shift_jis_single_byte_fd() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xFD");
        for _ in 0..40 {
            v.extend_from_slice(b"\x82\xC9\x82\xD9\x82\xF1\x82\xB2");
        }
        check_bytes(&v, SHIFT_JIS);
    }

    #[test]
    fn test_not_shift_jis() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\x84\xE0");
        for _ in 0..40 {
            v.extend_from_slice(b"\x82\xC9\x82\xD9\x82\xF1\x82\xB2");
        }
        check_bytes(&v, GBK);
    }

    #[test]
    fn test_not_shift_jis_bis() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\x87\x7D");
        for _ in 0..40 {
            v.extend_from_slice(b"\x82\xC9\x82\xD9\x82\xF1\x82\xB2");
        }
        check_bytes(&v, GBK);
    }

    #[test]
    fn test_euc_jp_x0213() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xAD\xBF");
        for _ in 0..80 {
            v.extend_from_slice(b"\xA4\xCB\xA4\xDB\xA4\xF3\xA4\xB4");
        }
        check_bytes(&v, EUC_JP);
    }

    #[test]
    fn test_euc_jp_x0213_other_plane() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\x8F\xFE\xF6");
        for _ in 0..80 {
            v.extend_from_slice(b"\xA4\xCB\xA4\xDB\xA4\xF3\xA4\xB4");
        }
        check_bytes(&v, EUC_JP);
    }

    #[test]
    fn test_not_euc_jp() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\x8F\xFE\xF7");
        for _ in 0..80 {
            v.extend_from_slice(b"\xA4\xCB\xA4\xDB\xA4\xF3\xA4\xB4");
        }
        check_bytes(&v, WINDOWS_1252);
    }

    #[test]
    fn test_not_euc_jp_bis() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xA8\xDF");
        for _ in 0..80 {
            v.extend_from_slice(b"\xA4\xCB\xA4\xDB\xA4\xF3\xA4\xB4");
        }
        check_bytes(&v, BIG5);
    }

    #[test]
    fn test_gbk_single_byte_ff() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xFF");
        for _ in 0..80 {
            v.extend_from_slice(b"\xB5\xC4");
        }
        check_bytes(&v, GBK);
    }

    #[test]
    fn test_gbk_single_byte_a0() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xA0 ");
        for _ in 0..80 {
            v.extend_from_slice(b"\xB5\xC4");
        }
        check_bytes(&v, GBK);
    }

    #[test]
    fn test_gbk_single_byte_fe() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xFE ");
        for _ in 0..80 {
            v.extend_from_slice(b"\xB5\xC4");
        }
        check_bytes(&v, GBK);
    }

    #[test]
    fn test_not_gbk_single_byte_fc() {
        let mut v = Vec::new();
        v.extend_from_slice(b"\xFC ");
        for _ in 0..80 {
            v.extend_from_slice(b"\xB5\xC4");
        }
        check_bytes(&v, ISO_8859_5);
    }
}
