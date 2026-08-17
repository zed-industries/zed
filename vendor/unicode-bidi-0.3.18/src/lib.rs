// Copyright 2015 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate implements the [Unicode Bidirectional Algorithm][tr9] for display of mixed
//! right-to-left and left-to-right text.  It is written in safe Rust, compatible with the
//! current stable release.
//!
//! ## Example
//!
//! ```rust
//! # #[cfg(feature = "hardcoded-data")] {
//! use unicode_bidi::BidiInfo;
//!
//! // This example text is defined using `concat!` because some browsers
//! // and text editors have trouble displaying bidi strings.
//! let text = concat![
//!   "א",
//!   "ב",
//!   "ג",
//!   "a",
//!   "b",
//!   "c",
//! ];
//!
//! // Resolve embedding levels within the text.  Pass `None` to detect the
//! // paragraph level automatically.
//! let bidi_info = BidiInfo::new(&text, None);
//!
//! // This paragraph has embedding level 1 because its first strong character is RTL.
//! assert_eq!(bidi_info.paragraphs.len(), 1);
//! let para = &bidi_info.paragraphs[0];
//! assert_eq!(para.level.number(), 1);
//! assert_eq!(para.level.is_rtl(), true);
//!
//! // Re-ordering is done after wrapping each paragraph into a sequence of
//! // lines. For this example, I'll just use a single line that spans the
//! // entire paragraph.
//! let line = para.range.clone();
//!
//! let display = bidi_info.reorder_line(para, line);
//! assert_eq!(display, concat![
//!   "a",
//!   "b",
//!   "c",
//!   "ג",
//!   "ב",
//!   "א",
//! ]);
//! # } // feature = "hardcoded-data"
//! ```
//!
//! # Features
//!
//! - `std`: Enabled by default, but can be disabled to make `unicode_bidi`
//!   `#![no_std]` + `alloc` compatible.
//! - `hardcoded-data`: Enabled by default. Includes hardcoded Unicode bidi data and more convenient APIs.
//! - `serde`: Adds [`serde::Serialize`] and [`serde::Deserialize`]
//!   implementations to relevant types.
//!
//! [tr9]: <http://www.unicode.org/reports/tr9/>

#![no_std]
// We need to link to std to make doc tests work on older Rust versions
#[cfg(feature = "std")]
extern crate std;
#[macro_use]
extern crate alloc;
#[cfg(feature = "smallvec")]
extern crate smallvec;

pub mod data_source;
pub mod deprecated;
pub mod format_chars;
pub mod level;
pub mod utf16;

mod char_data;
mod explicit;
mod implicit;
mod prepare;

pub use crate::char_data::{BidiClass, UNICODE_VERSION};
pub use crate::data_source::BidiDataSource;
pub use crate::level::{Level, LTR_LEVEL, RTL_LEVEL};
pub use crate::prepare::{LevelRun, LevelRunVec};

#[cfg(feature = "hardcoded-data")]
pub use crate::char_data::{bidi_class, HardcodedBidiData};

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use core::char;
use core::cmp;
use core::iter::repeat;
use core::ops::Range;
use core::str::CharIndices;
#[cfg(feature = "smallvec")]
use smallvec::SmallVec;

use crate::format_chars as chars;
use crate::BidiClass::*;

/// Trait that abstracts over a text source for use by the bidi algorithms.
/// We implement this for str (UTF-8) and for [u16] (UTF-16, native-endian).
/// (For internal unicode-bidi use; API may be unstable.)
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait TextSource<'text>: private::Sealed {
    type CharIter: Iterator<Item = char>;
    type CharIndexIter: Iterator<Item = (usize, char)>;
    type IndexLenIter: Iterator<Item = (usize, usize)>;

    /// Return the length of the text in code units.
    #[doc(hidden)]
    fn len(&self) -> usize;

    /// Get the character at a given code unit index, along with its length in code units.
    /// Returns None if index is out of range, or points inside a multi-code-unit character.
    /// Returns REPLACEMENT_CHARACTER for any unpaired surrogates in UTF-16.
    #[doc(hidden)]
    fn char_at(&self, index: usize) -> Option<(char, usize)>;

    /// Return a subrange of the text, indexed by code units.
    /// (We don't implement all of the Index trait, just the minimum we use.)
    #[doc(hidden)]
    fn subrange(&self, range: Range<usize>) -> &Self;

    /// An iterator over the text returning Unicode characters,
    /// REPLACEMENT_CHAR for invalid code units.
    #[doc(hidden)]
    fn chars(&'text self) -> Self::CharIter;

    /// An iterator over the text returning (index, char) tuples,
    /// where index is the starting code-unit index of the character,
    /// and char is its Unicode value (or REPLACEMENT_CHAR if invalid).
    #[doc(hidden)]
    fn char_indices(&'text self) -> Self::CharIndexIter;

    /// An iterator over the text returning (index, length) tuples,
    /// where index is the starting code-unit index of the character,
    /// and length is its length in code units.
    #[doc(hidden)]
    fn indices_lengths(&'text self) -> Self::IndexLenIter;

    /// Number of code units the given character uses.
    #[doc(hidden)]
    fn char_len(ch: char) -> usize;
}

mod private {
    pub trait Sealed {}

    // Implement for str and [u16] only.
    impl Sealed for str {}
    impl Sealed for [u16] {}
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Ltr,
    Rtl,
    Mixed,
}

/// Bidi information about a single paragraph
#[derive(Clone, Debug, PartialEq)]
pub struct ParagraphInfo {
    /// The paragraphs boundaries within the text, as byte indices.
    ///
    /// TODO: Shrink this to only include the starting index?
    pub range: Range<usize>,

    /// The paragraph embedding level.
    ///
    /// <http://www.unicode.org/reports/tr9/#BD4>
    pub level: Level,
}

impl ParagraphInfo {
    /// Gets the length of the paragraph in the source text.
    pub fn len(&self) -> usize {
        self.range.end - self.range.start
    }
}

/// Initial bidi information of the text.
///
/// Contains the text paragraphs and `BidiClass` of its characters.
#[derive(PartialEq, Debug)]
pub struct InitialInfo<'text> {
    /// The text
    pub text: &'text str,

    /// The BidiClass of the character at each byte in the text.
    /// If a character is multiple bytes, its class will appear multiple times in the vector.
    pub original_classes: Vec<BidiClass>,

    /// The boundaries and level of each paragraph within the text.
    pub paragraphs: Vec<ParagraphInfo>,
}

impl<'text> InitialInfo<'text> {
    /// Find the paragraphs and BidiClasses in a string of text.
    ///
    /// <http://www.unicode.org/reports/tr9/#The_Paragraph_Level>
    ///
    /// Also sets the class for each First Strong Isolate initiator (FSI) to LRI or RLI if a strong
    /// character is found before the matching PDI.  If no strong character is found, the class will
    /// remain FSI, and it's up to later stages to treat these as LRI when needed.
    ///
    /// The `hardcoded-data` Cargo feature (enabled by default) must be enabled to use this.
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    #[cfg(feature = "hardcoded-data")]
    pub fn new(text: &str, default_para_level: Option<Level>) -> InitialInfo<'_> {
        Self::new_with_data_source(&HardcodedBidiData, text, default_para_level)
    }

    /// Find the paragraphs and BidiClasses in a string of text, with a custom [`BidiDataSource`]
    /// for Bidi data. If you just wish to use the hardcoded Bidi data, please use [`InitialInfo::new()`]
    /// instead (enabled with tbe default `hardcoded-data` Cargo feature)
    ///
    /// <http://www.unicode.org/reports/tr9/#The_Paragraph_Level>
    ///
    /// Also sets the class for each First Strong Isolate initiator (FSI) to LRI or RLI if a strong
    /// character is found before the matching PDI.  If no strong character is found, the class will
    /// remain FSI, and it's up to later stages to treat these as LRI when needed.
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn new_with_data_source<'a, D: BidiDataSource>(
        data_source: &D,
        text: &'a str,
        default_para_level: Option<Level>,
    ) -> InitialInfo<'a> {
        InitialInfoExt::new_with_data_source(data_source, text, default_para_level).base
    }
}

/// Extended version of InitialInfo (not public API).
#[derive(PartialEq, Debug)]
struct InitialInfoExt<'text> {
    /// The base InitialInfo for the text, recording its paragraphs and bidi classes.
    base: InitialInfo<'text>,

    /// Parallel to base.paragraphs, records whether each paragraph is "pure LTR" that
    /// requires no further bidi processing (i.e. there are no RTL characters or bidi
    /// control codes present), and whether any bidi isolation controls are present.
    flags: Vec<ParagraphInfoFlags>,
}

#[derive(PartialEq, Debug)]
struct ParagraphInfoFlags {
    is_pure_ltr: bool,
    has_isolate_controls: bool,
}

impl<'text> InitialInfoExt<'text> {
    /// Find the paragraphs and BidiClasses in a string of text, with a custom [`BidiDataSource`]
    /// for Bidi data. If you just wish to use the hardcoded Bidi data, please use [`InitialInfo::new()`]
    /// instead (enabled with tbe default `hardcoded-data` Cargo feature)
    ///
    /// <http://www.unicode.org/reports/tr9/#The_Paragraph_Level>
    ///
    /// Also sets the class for each First Strong Isolate initiator (FSI) to LRI or RLI if a strong
    /// character is found before the matching PDI.  If no strong character is found, the class will
    /// remain FSI, and it's up to later stages to treat these as LRI when needed.
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn new_with_data_source<'a, D: BidiDataSource>(
        data_source: &D,
        text: &'a str,
        default_para_level: Option<Level>,
    ) -> InitialInfoExt<'a> {
        let mut paragraphs = Vec::<ParagraphInfo>::new();
        let mut flags = Vec::<ParagraphInfoFlags>::new();
        let (original_classes, _, _, _) = compute_initial_info(
            data_source,
            text,
            default_para_level,
            Some((&mut paragraphs, &mut flags)),
        );

        InitialInfoExt {
            base: InitialInfo {
                text,
                original_classes,
                paragraphs,
            },
            flags,
        }
    }
}

/// Implementation of initial-info computation for both BidiInfo and ParagraphBidiInfo.
/// To treat the text as (potentially) multiple paragraphs, the caller should pass the
/// pair of optional outparam arrays to receive the ParagraphInfo and pure-ltr flags
/// for each paragraph. Passing None for split_paragraphs will ignore any paragraph-
/// separator characters in the text, treating it just as a single paragraph.
/// Returns the array of BidiClass values for each code unit of the text, along with
/// the embedding level and pure-ltr flag for the *last* (or only) paragraph.
fn compute_initial_info<'a, D: BidiDataSource, T: TextSource<'a> + ?Sized>(
    data_source: &D,
    text: &'a T,
    default_para_level: Option<Level>,
    mut split_paragraphs: Option<(&mut Vec<ParagraphInfo>, &mut Vec<ParagraphInfoFlags>)>,
) -> (Vec<BidiClass>, Level, bool, bool) {
    let mut original_classes = Vec::with_capacity(text.len());

    // The stack contains the starting code unit index for each nested isolate we're inside.
    #[cfg(feature = "smallvec")]
    let mut isolate_stack = SmallVec::<[usize; 8]>::new();
    #[cfg(not(feature = "smallvec"))]
    let mut isolate_stack = Vec::new();

    debug_assert!(
        if let Some((ref paragraphs, ref flags)) = split_paragraphs {
            paragraphs.is_empty() && flags.is_empty()
        } else {
            true
        }
    );

    let mut para_start = 0;
    let mut para_level = default_para_level;

    // Per-paragraph flag: can subsequent processing be skipped? Set to false if any
    // RTL characters or bidi control characters are encountered in the paragraph.
    let mut is_pure_ltr = true;
    // Set to true if any bidi isolation controls are present in the paragraph.
    let mut has_isolate_controls = false;

    #[cfg(feature = "flame_it")]
    flame::start("compute_initial_info(): iter text.char_indices()");

    for (i, c) in text.char_indices() {
        let class = data_source.bidi_class(c);

        #[cfg(feature = "flame_it")]
        flame::start("original_classes.extend()");

        let len = T::char_len(c);
        original_classes.extend(repeat(class).take(len));

        #[cfg(feature = "flame_it")]
        flame::end("original_classes.extend()");

        match class {
            B => {
                if let Some((ref mut paragraphs, ref mut flags)) = split_paragraphs {
                    // P1. Split the text into separate paragraphs. The paragraph separator is kept
                    // with the previous paragraph.
                    let para_end = i + len;
                    paragraphs.push(ParagraphInfo {
                        range: para_start..para_end,
                        // P3. If no character is found in p2, set the paragraph level to zero.
                        level: para_level.unwrap_or(LTR_LEVEL),
                    });
                    flags.push(ParagraphInfoFlags {
                        is_pure_ltr,
                        has_isolate_controls,
                    });
                    // Reset state for the start of the next paragraph.
                    para_start = para_end;
                    // TODO: Support defaulting to direction of previous paragraph
                    //
                    // <http://www.unicode.org/reports/tr9/#HL1>
                    para_level = default_para_level;
                    is_pure_ltr = true;
                    has_isolate_controls = false;
                    isolate_stack.clear();
                }
            }

            L | R | AL => {
                if class != L {
                    is_pure_ltr = false;
                }
                match isolate_stack.last() {
                    Some(&start) => {
                        if original_classes[start] == FSI {
                            // X5c. If the first strong character between FSI and its matching
                            // PDI is R or AL, treat it as RLI. Otherwise, treat it as LRI.
                            for j in 0..T::char_len(chars::FSI) {
                                original_classes[start + j] = if class == L { LRI } else { RLI };
                            }
                        }
                    }

                    None => {
                        if para_level.is_none() {
                            // P2. Find the first character of type L, AL, or R, while skipping
                            // any characters between an isolate initiator and its matching
                            // PDI.
                            para_level = Some(if class != L { RTL_LEVEL } else { LTR_LEVEL });
                        }
                    }
                }
            }

            AN | LRE | RLE | LRO | RLO => {
                is_pure_ltr = false;
            }

            RLI | LRI | FSI => {
                is_pure_ltr = false;
                has_isolate_controls = true;
                isolate_stack.push(i);
            }

            PDI => {
                isolate_stack.pop();
            }

            _ => {}
        }
    }

    if let Some((paragraphs, flags)) = split_paragraphs {
        if para_start < text.len() {
            paragraphs.push(ParagraphInfo {
                range: para_start..text.len(),
                level: para_level.unwrap_or(LTR_LEVEL),
            });
            flags.push(ParagraphInfoFlags {
                is_pure_ltr,
                has_isolate_controls,
            });
        }
        debug_assert_eq!(paragraphs.len(), flags.len());
    }
    debug_assert_eq!(original_classes.len(), text.len());

    #[cfg(feature = "flame_it")]
    flame::end("compute_initial_info(): iter text.char_indices()");

    (
        original_classes,
        para_level.unwrap_or(LTR_LEVEL),
        is_pure_ltr,
        has_isolate_controls,
    )
}

/// Bidi information of the text.
///
/// The `original_classes` and `levels` vectors are indexed by byte offsets into the text.  If a
/// character is multiple bytes wide, then its class and level will appear multiple times in these
/// vectors.
// TODO: Impl `struct StringProperty<T> { values: Vec<T> }` and use instead of Vec<T>
#[derive(Debug, PartialEq)]
pub struct BidiInfo<'text> {
    /// The text
    pub text: &'text str,

    /// The BidiClass of the character at each byte in the text.
    pub original_classes: Vec<BidiClass>,

    /// The directional embedding level of each byte in the text.
    pub levels: Vec<Level>,

    /// The boundaries and paragraph embedding level of each paragraph within the text.
    ///
    /// TODO: Use SmallVec or similar to avoid overhead when there are only one or two paragraphs?
    /// Or just don't include the first paragraph, which always starts at 0?
    pub paragraphs: Vec<ParagraphInfo>,
}

impl<'text> BidiInfo<'text> {
    /// Split the text into paragraphs and determine the bidi embedding levels for each paragraph.
    ///
    ///
    /// The `hardcoded-data` Cargo feature (enabled by default) must be enabled to use this.
    ///
    /// TODO: In early steps, check for special cases that allow later steps to be skipped. like
    /// text that is entirely LTR.  See the `nsBidi` class from Gecko for comparison.
    ///
    /// TODO: Support auto-RTL base direction
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    #[cfg(feature = "hardcoded-data")]
    #[inline]
    pub fn new(text: &str, default_para_level: Option<Level>) -> BidiInfo<'_> {
        Self::new_with_data_source(&HardcodedBidiData, text, default_para_level)
    }

    /// Split the text into paragraphs and determine the bidi embedding levels for each paragraph, with a custom [`BidiDataSource`]
    /// for Bidi data. If you just wish to use the hardcoded Bidi data, please use [`BidiInfo::new()`]
    /// instead (enabled with tbe default `hardcoded-data` Cargo feature).
    ///
    /// TODO: In early steps, check for special cases that allow later steps to be skipped. like
    /// text that is entirely LTR.  See the `nsBidi` class from Gecko for comparison.
    ///
    /// TODO: Support auto-RTL base direction
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn new_with_data_source<'a, D: BidiDataSource>(
        data_source: &D,
        text: &'a str,
        default_para_level: Option<Level>,
    ) -> BidiInfo<'a> {
        let InitialInfoExt { base, flags, .. } =
            InitialInfoExt::new_with_data_source(data_source, text, default_para_level);

        let mut levels = Vec::<Level>::with_capacity(text.len());
        let mut processing_classes = base.original_classes.clone();

        for (para, flags) in base.paragraphs.iter().zip(flags.iter()) {
            let text = &text[para.range.clone()];
            let original_classes = &base.original_classes[para.range.clone()];

            compute_bidi_info_for_para(
                data_source,
                para,
                flags.is_pure_ltr,
                flags.has_isolate_controls,
                text,
                original_classes,
                &mut processing_classes,
                &mut levels,
            );
        }

        BidiInfo {
            text,
            original_classes: base.original_classes,
            paragraphs: base.paragraphs,
            levels,
        }
    }

    /// Produce the levels for this paragraph as needed for reordering, one level per *byte*
    /// in the paragraph. The returned vector includes bytes that are not included
    /// in the `line`, but will not adjust them.
    ///
    /// This runs [Rule L1], you can run
    /// [Rule L2] by calling [`Self::reorder_visual()`].
    /// If doing so, you may prefer to use [`Self::reordered_levels_per_char()`] instead
    /// to avoid non-byte indices.
    ///
    /// For an all-in-one reordering solution, consider using [`Self::reorder_visual()`].
    ///
    /// [Rule L1]: https://www.unicode.org/reports/tr9/#L1
    /// [Rule L2]: https://www.unicode.org/reports/tr9/#L2
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn reordered_levels(&self, para: &ParagraphInfo, line: Range<usize>) -> Vec<Level> {
        assert!(line.start <= self.levels.len());
        assert!(line.end <= self.levels.len());

        let mut levels = self.levels.clone();
        let line_classes = &self.original_classes[line.clone()];
        let line_levels = &mut levels[line.clone()];

        reorder_levels(
            line_classes,
            line_levels,
            self.text.subrange(line),
            para.level,
        );

        levels
    }

    /// Produce the levels for this paragraph as needed for reordering, one level per *character*
    /// in the paragraph. The returned vector includes characters that are not included
    /// in the `line`, but will not adjust them.
    ///
    /// This runs [Rule L1], you can run
    /// [Rule L2] by calling [`Self::reorder_visual()`].
    /// If doing so, you may prefer to use [`Self::reordered_levels_per_char()`] instead
    /// to avoid non-byte indices.
    ///
    /// For an all-in-one reordering solution, consider using [`Self::reorder_visual()`].
    ///
    /// [Rule L1]: https://www.unicode.org/reports/tr9/#L1
    /// [Rule L2]: https://www.unicode.org/reports/tr9/#L2
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn reordered_levels_per_char(
        &self,
        para: &ParagraphInfo,
        line: Range<usize>,
    ) -> Vec<Level> {
        let levels = self.reordered_levels(para, line);
        self.text.char_indices().map(|(i, _)| levels[i]).collect()
    }

    /// Re-order a line based on resolved levels and return the line in display order.
    ///
    /// This does not apply [Rule L3] or [Rule L4] around combining characters or mirroring.
    ///
    /// [Rule L3]: https://www.unicode.org/reports/tr9/#L3
    /// [Rule L4]: https://www.unicode.org/reports/tr9/#L4
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn reorder_line(&self, para: &ParagraphInfo, line: Range<usize>) -> Cow<'text, str> {
        if !level::has_rtl(&self.levels[line.clone()]) {
            return self.text[line].into();
        }
        let (levels, runs) = self.visual_runs(para, line.clone());
        reorder_line(self.text, line, levels, runs)
    }

    /// Reorders pre-calculated levels of a sequence of characters.
    ///
    /// NOTE: This is a convenience method that does not use a `Paragraph`  object. It is
    /// intended to be used when an application has determined the levels of the objects (character sequences)
    /// and just needs to have them reordered.
    ///
    /// the index map will result in `indexMap[visualIndex]==logicalIndex`.
    ///
    /// This only runs [Rule L2](http://www.unicode.org/reports/tr9/#L2) as it does not have
    /// information about the actual text.
    ///
    /// Furthermore, if `levels` is an array that is aligned with code units, bytes within a codepoint may be
    /// reversed. You may need to fix up the map to deal with this. Alternatively, only pass in arrays where each `Level`
    /// is for a single code point.
    ///
    ///
    ///   # # Example
    /// ```
    /// use unicode_bidi::BidiInfo;
    /// use unicode_bidi::Level;
    ///
    /// let l0 = Level::from(0);
    /// let l1 = Level::from(1);
    /// let l2 = Level::from(2);
    ///
    /// let levels = vec![l0, l0, l0, l0];
    /// let index_map = BidiInfo::reorder_visual(&levels);
    /// assert_eq!(levels.len(), index_map.len());
    /// assert_eq!(index_map, [0, 1, 2, 3]);
    ///
    /// let levels: Vec<Level> = vec![l0, l0, l0, l1, l1, l1, l2, l2];
    /// let index_map = BidiInfo::reorder_visual(&levels);
    /// assert_eq!(levels.len(), index_map.len());
    /// assert_eq!(index_map, [0, 1, 2, 6, 7, 5, 4, 3]);
    /// ```
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    #[inline]
    pub fn reorder_visual(levels: &[Level]) -> Vec<usize> {
        reorder_visual(levels)
    }

    /// Find the level runs within a line and return them in visual order.
    ///
    /// `line` is a range of bytes indices within `levels`.
    ///
    /// The first return value is a vector of levels used by the reordering algorithm,
    /// i.e. the result of [Rule L1]. The second return value is a vector of level runs,
    /// the result of [Rule L2], showing the visual order that each level run (a run of text with the
    /// same level) should be displayed. Within each run, the display order can be checked
    /// against the Level vector.
    ///
    /// This does not handle [Rule L3] (combining characters) or [Rule L4] (mirroring),
    /// as that should be handled by the engine using this API.
    ///
    /// Conceptually, this is the same as running [`Self::reordered_levels()`] followed by
    /// [`Self::reorder_visual()`], however it returns the result as a list of level runs instead
    /// of producing a level map, since one may wish to deal with the fact that this is operating on
    /// byte rather than character indices.
    ///
    /// <http://www.unicode.org/reports/tr9/#Reordering_Resolved_Levels>
    ///
    /// [Rule L1]: https://www.unicode.org/reports/tr9/#L1
    /// [Rule L2]: https://www.unicode.org/reports/tr9/#L2
    /// [Rule L3]: https://www.unicode.org/reports/tr9/#L3
    /// [Rule L4]: https://www.unicode.org/reports/tr9/#L4
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    #[inline]
    pub fn visual_runs(
        &self,
        para: &ParagraphInfo,
        line: Range<usize>,
    ) -> (Vec<Level>, Vec<LevelRun>) {
        let levels = self.reordered_levels(para, line.clone());
        visual_runs_for_line(levels, &line)
    }

    /// If processed text has any computed RTL levels
    ///
    /// This information is usually used to skip re-ordering of text when no RTL level is present
    #[inline]
    pub fn has_rtl(&self) -> bool {
        level::has_rtl(&self.levels)
    }
}

/// Bidi information of text treated as a single paragraph.
///
/// The `original_classes` and `levels` vectors are indexed by byte offsets into the text.  If a
/// character is multiple bytes wide, then its class and level will appear multiple times in these
/// vectors.
#[derive(Debug, PartialEq)]
pub struct ParagraphBidiInfo<'text> {
    /// The text
    pub text: &'text str,

    /// The BidiClass of the character at each byte in the text.
    pub original_classes: Vec<BidiClass>,

    /// The directional embedding level of each byte in the text.
    pub levels: Vec<Level>,

    /// The paragraph embedding level.
    pub paragraph_level: Level,

    /// Whether the paragraph is purely LTR.
    pub is_pure_ltr: bool,
}

impl<'text> ParagraphBidiInfo<'text> {
    /// Determine the bidi embedding level.
    ///
    ///
    /// The `hardcoded-data` Cargo feature (enabled by default) must be enabled to use this.
    ///
    /// TODO: In early steps, check for special cases that allow later steps to be skipped. like
    /// text that is entirely LTR.  See the `nsBidi` class from Gecko for comparison.
    ///
    /// TODO: Support auto-RTL base direction
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    #[cfg(feature = "hardcoded-data")]
    #[inline]
    pub fn new(text: &str, default_para_level: Option<Level>) -> ParagraphBidiInfo<'_> {
        Self::new_with_data_source(&HardcodedBidiData, text, default_para_level)
    }

    /// Determine the bidi embedding level, with a custom [`BidiDataSource`]
    /// for Bidi data. If you just wish to use the hardcoded Bidi data, please use [`BidiInfo::new()`]
    /// instead (enabled with tbe default `hardcoded-data` Cargo feature).
    ///
    /// (This is the single-paragraph equivalent of BidiInfo::new_with_data_source,
    /// and should be kept in sync with it.
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn new_with_data_source<'a, D: BidiDataSource>(
        data_source: &D,
        text: &'a str,
        default_para_level: Option<Level>,
    ) -> ParagraphBidiInfo<'a> {
        // Here we could create a ParagraphInitialInfo struct to parallel the one
        // used by BidiInfo, but there doesn't seem any compelling reason for it.
        let (original_classes, paragraph_level, is_pure_ltr, has_isolate_controls) =
            compute_initial_info(data_source, text, default_para_level, None);

        let mut levels = Vec::<Level>::with_capacity(text.len());
        let mut processing_classes = original_classes.clone();

        let para_info = ParagraphInfo {
            range: Range {
                start: 0,
                end: text.len(),
            },
            level: paragraph_level,
        };

        compute_bidi_info_for_para(
            data_source,
            &para_info,
            is_pure_ltr,
            has_isolate_controls,
            text,
            &original_classes,
            &mut processing_classes,
            &mut levels,
        );

        ParagraphBidiInfo {
            text,
            original_classes,
            levels,
            paragraph_level,
            is_pure_ltr,
        }
    }

    /// Produce the levels for this paragraph as needed for reordering, one level per *byte*
    /// in the paragraph. The returned vector includes bytes that are not included
    /// in the `line`, but will not adjust them.
    ///
    /// See BidiInfo::reordered_levels for details.
    ///
    /// (This should be kept in sync with BidiInfo::reordered_levels.)
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn reordered_levels(&self, line: Range<usize>) -> Vec<Level> {
        assert!(line.start <= self.levels.len());
        assert!(line.end <= self.levels.len());

        let mut levels = self.levels.clone();
        let line_classes = &self.original_classes[line.clone()];
        let line_levels = &mut levels[line.clone()];

        reorder_levels(
            line_classes,
            line_levels,
            self.text.subrange(line),
            self.paragraph_level,
        );

        levels
    }

    /// Produce the levels for this paragraph as needed for reordering, one level per *character*
    /// in the paragraph. The returned vector includes characters that are not included
    /// in the `line`, but will not adjust them.
    ///
    /// See BidiInfo::reordered_levels_per_char for details.
    ///
    /// (This should be kept in sync with BidiInfo::reordered_levels_per_char.)
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn reordered_levels_per_char(&self, line: Range<usize>) -> Vec<Level> {
        let levels = self.reordered_levels(line);
        self.text.char_indices().map(|(i, _)| levels[i]).collect()
    }

    /// Re-order a line based on resolved levels and return the line in display order.
    ///
    /// See BidiInfo::reorder_line for details.
    ///
    /// (This should be kept in sync with BidiInfo::reorder_line.)
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    pub fn reorder_line(&self, line: Range<usize>) -> Cow<'text, str> {
        if !level::has_rtl(&self.levels[line.clone()]) {
            return self.text[line].into();
        }

        let (levels, runs) = self.visual_runs(line.clone());

        reorder_line(self.text, line, levels, runs)
    }

    /// Reorders pre-calculated levels of a sequence of characters.
    ///
    /// See BidiInfo::reorder_visual for details.
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    #[inline]
    pub fn reorder_visual(levels: &[Level]) -> Vec<usize> {
        reorder_visual(levels)
    }

    /// Find the level runs within a line and return them in visual order.
    ///
    /// `line` is a range of bytes indices within `levels`.
    ///
    /// See BidiInfo::visual_runs for details.
    ///
    /// (This should be kept in sync with BidiInfo::visual_runs.)
    #[cfg_attr(feature = "flame_it", flamer::flame)]
    #[inline]
    pub fn visual_runs(&self, line: Range<usize>) -> (Vec<Level>, Vec<LevelRun>) {
        let levels = self.reordered_levels(line.clone());
        visual_runs_for_line(levels, &line)
    }

    /// If processed text has any computed RTL levels
    ///
    /// This information is usually used to skip re-ordering of text when no RTL level is present
    #[inline]
    pub fn has_rtl(&self) -> bool {
        !self.is_pure_ltr
    }

    /// Return the paragraph's Direction (Ltr, Rtl, or Mixed) based on its levels.
    #[inline]
    pub fn direction(&self) -> Direction {
        para_direction(&self.levels)
    }
}

/// Return a line of the text in display order based on resolved levels.
///
/// `text`   the full text passed to the `BidiInfo` or `ParagraphBidiInfo` for analysis
/// `line`   a range of byte indices within `text` corresponding to one line
/// `levels` array of `Level` values, with `line`'s levels reordered into visual order
/// `runs`   array of `LevelRun`s in visual order
///
/// (`levels` and `runs` are the result of calling `BidiInfo::visual_runs()` or
/// `ParagraphBidiInfo::visual_runs()` for the line of interest.)
///
/// Returns: the reordered text of the line.
///
/// This does not apply [Rule L3] or [Rule L4] around combining characters or mirroring.
///
/// [Rule L3]: https://www.unicode.org/reports/tr9/#L3
/// [Rule L4]: https://www.unicode.org/reports/tr9/#L4
fn reorder_line(
    text: &str,
    line: Range<usize>,
    levels: Vec<Level>,
    runs: Vec<LevelRun>,
) -> Cow<'_, str> {
    // If all isolating run sequences are LTR, no reordering is needed
    if runs.iter().all(|run| levels[run.start].is_ltr()) {
        return text[line].into();
    }

    let mut result = String::with_capacity(line.len());
    for run in runs {
        if levels[run.start].is_rtl() {
            result.extend(text[run].chars().rev());
        } else {
            result.push_str(&text[run]);
        }
    }
    result.into()
}

/// Find the level runs within a line and return them in visual order.
///
/// `line` is a range of code-unit indices within `levels`.
///
/// The first return value is a vector of levels used by the reordering algorithm,
/// i.e. the result of [Rule L1]. The second return value is a vector of level runs,
/// the result of [Rule L2], showing the visual order that each level run (a run of text with the
/// same level) should be displayed. Within each run, the display order can be checked
/// against the Level vector.
///
/// This does not handle [Rule L3] (combining characters) or [Rule L4] (mirroring),
/// as that should be handled by the engine using this API.
///
/// Conceptually, this is the same as running [`reordered_levels()`] followed by
/// [`reorder_visual()`], however it returns the result as a list of level runs instead
/// of producing a level map, since one may wish to deal with the fact that this is operating on
/// byte rather than character indices.
///
/// <http://www.unicode.org/reports/tr9/#Reordering_Resolved_Levels>
///
/// [Rule L1]: https://www.unicode.org/reports/tr9/#L1
/// [Rule L2]: https://www.unicode.org/reports/tr9/#L2
/// [Rule L3]: https://www.unicode.org/reports/tr9/#L3
/// [Rule L4]: https://www.unicode.org/reports/tr9/#L4
fn visual_runs_for_line(levels: Vec<Level>, line: &Range<usize>) -> (Vec<Level>, Vec<LevelRun>) {
    // Find consecutive level runs.
    let mut runs = Vec::new();
    let mut start = line.start;
    let mut run_level = levels[start];
    let mut min_level = run_level;
    let mut max_level = run_level;

    for (i, &new_level) in levels.iter().enumerate().take(line.end).skip(start + 1) {
        if new_level != run_level {
            // End of the previous run, start of a new one.
            runs.push(start..i);
            start = i;
            run_level = new_level;
            min_level = cmp::min(run_level, min_level);
            max_level = cmp::max(run_level, max_level);
        }
    }
    runs.push(start..line.end);

    let run_count = runs.len();

    // Re-order the odd runs.
    // <http://www.unicode.org/reports/tr9/#L2>

    // Stop at the lowest *odd* level.
    min_level = min_level.new_lowest_ge_rtl().expect("Level error");
    // This loop goes through contiguous chunks of level runs that have a level
    // ≥ max_level and reverses their contents, reducing max_level by 1 each time.
    while max_level >= min_level {
        // Look for the start of a sequence of consecutive runs of max_level or higher.
        let mut seq_start = 0;
        while seq_start < run_count {
            if levels[runs[seq_start].start] < max_level {
                seq_start += 1;
                continue;
            }

            // Found the start of a sequence. Now find the end.
            let mut seq_end = seq_start + 1;
            while seq_end < run_count {
                if levels[runs[seq_end].start] < max_level {
                    break;
                }
                seq_end += 1;
            }
            // Reverse the runs within this sequence.
            runs[seq_start..seq_end].reverse();

            seq_start = seq_end;
        }
        max_level
            .lower(1)
            .expect("Lowering embedding level below zero");
    }
    (levels, runs)
}

/// Reorders pre-calculated levels of a sequence of characters.
///
/// NOTE: This is a convenience method that does not use a `Paragraph`  object. It is
/// intended to be used when an application has determined the levels of the objects (character sequences)
/// and just needs to have them reordered.
///
/// the index map will result in `indexMap[visualIndex]==logicalIndex`.
///
/// This only runs [Rule L2](http://www.unicode.org/reports/tr9/#L2) as it does not have
/// information about the actual text.
///
/// Furthermore, if `levels` is an array that is aligned with code units, bytes within a codepoint may be
/// reversed. You may need to fix up the map to deal with this. Alternatively, only pass in arrays where each `Level`
/// is for a single code point.
fn reorder_visual(levels: &[Level]) -> Vec<usize> {
    // Gets the next range of characters after start_index with a level greater
    // than or equal to `max`
    fn next_range(levels: &[level::Level], mut start_index: usize, max: Level) -> Range<usize> {
        if levels.is_empty() || start_index >= levels.len() {
            return start_index..start_index;
        }
        while let Some(l) = levels.get(start_index) {
            if *l >= max {
                break;
            }
            start_index += 1;
        }

        if levels.get(start_index).is_none() {
            // If at the end of the array, adding one will
            // produce an out-of-range end element
            return start_index..start_index;
        }

        let mut end_index = start_index + 1;
        while let Some(l) = levels.get(end_index) {
            if *l < max {
                return start_index..end_index;
            }
            end_index += 1;
        }

        start_index..end_index
    }

    // This implementation is similar to the L2 implementation in `visual_runs()`
    // but it cannot benefit from a precalculated LevelRun vector so needs to be different.

    if levels.is_empty() {
        return vec![];
    }

    // Get the min and max levels
    let (mut min, mut max) = levels
        .iter()
        .fold((levels[0], levels[0]), |(min, max), &l| {
            (cmp::min(min, l), cmp::max(max, l))
        });

    // Initialize an index map
    let mut result: Vec<usize> = (0..levels.len()).collect();

    if min == max && min.is_ltr() {
        // Everything is LTR and at the same level, do nothing
        return result;
    }

    // Stop at the lowest *odd* level, since everything below that
    // is LTR and does not need further reordering
    min = min.new_lowest_ge_rtl().expect("Level error");

    // For each max level, take all contiguous chunks of
    // levels ≥ max and reverse them
    //
    // We can do this check with the original levels instead of checking reorderings because all
    // prior reorderings will have been for contiguous chunks of levels >> max, which will
    // be a subset of these chunks anyway.
    while min <= max {
        let mut range = 0..0;
        loop {
            range = next_range(levels, range.end, max);
            result[range.clone()].reverse();

            if range.end >= levels.len() {
                break;
            }
        }

        max.lower(1).expect("Level error");
    }

    result
}

/// The core of BidiInfo initialization, factored out into a function that both
/// the utf-8 and utf-16 versions of BidiInfo can use.
fn compute_bidi_info_for_para<'a, D: BidiDataSource, T: TextSource<'a> + ?Sized>(
    data_source: &D,
    para: &ParagraphInfo,
    is_pure_ltr: bool,
    has_isolate_controls: bool,
    text: &'a T,
    original_classes: &[BidiClass],
    processing_classes: &mut [BidiClass],
    levels: &mut Vec<Level>,
) {
    let new_len = levels.len() + para.range.len();
    levels.resize(new_len, para.level);
    if para.level == LTR_LEVEL && is_pure_ltr {
        return;
    }

    let processing_classes = &mut processing_classes[para.range.clone()];
    let levels = &mut levels[para.range.clone()];
    let mut level_runs = LevelRunVec::new();

    explicit::compute(
        text,
        para.level,
        original_classes,
        levels,
        processing_classes,
        &mut level_runs,
    );

    let mut sequences = prepare::IsolatingRunSequenceVec::new();
    prepare::isolating_run_sequences(
        para.level,
        original_classes,
        levels,
        level_runs,
        has_isolate_controls,
        &mut sequences,
    );
    for sequence in &sequences {
        implicit::resolve_weak(text, sequence, processing_classes);
        implicit::resolve_neutral(
            text,
            data_source,
            sequence,
            levels,
            original_classes,
            processing_classes,
        );
    }

    implicit::resolve_levels(processing_classes, levels);

    assign_levels_to_removed_chars(para.level, original_classes, levels);
}

/// Produce the levels for this paragraph as needed for reordering, one level per *code unit*
/// in the paragraph. The returned vector includes code units that are not included
/// in the `line`, but will not adjust them.
///
/// This runs [Rule L1]
///
/// [Rule L1]: https://www.unicode.org/reports/tr9/#L1
fn reorder_levels<'a, T: TextSource<'a> + ?Sized>(
    line_classes: &[BidiClass],
    line_levels: &mut [Level],
    line_text: &'a T,
    para_level: Level,
) {
    // Reset some whitespace chars to paragraph level.
    // <http://www.unicode.org/reports/tr9/#L1>
    let mut reset_from: Option<usize> = Some(0);
    let mut reset_to: Option<usize> = None;
    let mut prev_level = para_level;
    for ((i, c), (_, length)) in line_text.char_indices().zip(line_text.indices_lengths()) {
        match line_classes[i] {
            // Segment separator, Paragraph separator
            B | S => {
                assert_eq!(reset_to, None);
                reset_to = Some(i + T::char_len(c));
                if reset_from.is_none() {
                    reset_from = Some(i);
                }
            }
            // Whitespace, isolate formatting
            WS | FSI | LRI | RLI | PDI => {
                if reset_from.is_none() {
                    reset_from = Some(i);
                }
            }
            // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
            // same as above + set the level
            RLE | LRE | RLO | LRO | PDF | BN => {
                if reset_from.is_none() {
                    reset_from = Some(i);
                }
                // also set the level to previous
                for level in &mut line_levels[i..i + length] {
                    *level = prev_level;
                }
            }
            _ => {
                reset_from = None;
            }
        }
        if let (Some(from), Some(to)) = (reset_from, reset_to) {
            for level in &mut line_levels[from..to] {
                *level = para_level;
            }
            reset_from = None;
            reset_to = None;
        }
        prev_level = line_levels[i];
    }
    if let Some(from) = reset_from {
        for level in &mut line_levels[from..] {
            *level = para_level;
        }
    }
}

/// Contains a reference of `BidiInfo` and one of its `paragraphs`.
/// And it supports all operation in the `Paragraph` that needs also its
/// `BidiInfo` such as `direction`.
#[derive(Debug)]
pub struct Paragraph<'a, 'text> {
    pub info: &'a BidiInfo<'text>,
    pub para: &'a ParagraphInfo,
}

impl<'a, 'text> Paragraph<'a, 'text> {
    #[inline]
    pub fn new(info: &'a BidiInfo<'text>, para: &'a ParagraphInfo) -> Paragraph<'a, 'text> {
        Paragraph { info, para }
    }

    /// Returns if the paragraph is Left direction, right direction or mixed.
    #[inline]
    pub fn direction(&self) -> Direction {
        para_direction(&self.info.levels[self.para.range.clone()])
    }

    /// Returns the `Level` of a certain character in the paragraph.
    #[inline]
    pub fn level_at(&self, pos: usize) -> Level {
        let actual_position = self.para.range.start + pos;
        self.info.levels[actual_position]
    }
}

/// Return the directionality of the paragraph (Left, Right or Mixed) from its levels.
#[cfg_attr(feature = "flame_it", flamer::flame)]
fn para_direction(levels: &[Level]) -> Direction {
    let mut ltr = false;
    let mut rtl = false;
    for level in levels {
        if level.is_ltr() {
            ltr = true;
            if rtl {
                return Direction::Mixed;
            }
        }

        if level.is_rtl() {
            rtl = true;
            if ltr {
                return Direction::Mixed;
            }
        }
    }

    if ltr {
        return Direction::Ltr;
    }

    Direction::Rtl
}

/// Assign levels to characters removed by rule X9.
///
/// The levels assigned to these characters are not specified by the algorithm.  This function
/// assigns each one the level of the previous character, to avoid breaking level runs.
#[cfg_attr(feature = "flame_it", flamer::flame)]
fn assign_levels_to_removed_chars(para_level: Level, classes: &[BidiClass], levels: &mut [Level]) {
    for i in 0..levels.len() {
        if prepare::removed_by_x9(classes[i]) {
            levels[i] = if i > 0 { levels[i - 1] } else { para_level };
        }
    }
}

/// Get the base direction of the text provided according to the Unicode Bidirectional Algorithm.
///
/// See rules P2 and P3.
///
/// The base direction is derived from the first character in the string with bidi character type
/// L, R, or AL. If the first such character has type L, Direction::Ltr is returned. If the first
/// such character has type R or AL, Direction::Rtl is returned.
///
/// If the string does not contain any character of these types (outside of embedded isolate runs),
/// then Direction::Mixed is returned (but should be considered as meaning "neutral" or "unknown",
/// not in fact mixed directions).
///
/// This is a lightweight function for use when only the base direction is needed and no further
/// bidi processing of the text is needed.
///
/// If the text contains paragraph separators, this function considers only the first paragraph.
#[cfg(feature = "hardcoded-data")]
#[inline]
pub fn get_base_direction<'a, T: TextSource<'a> + ?Sized>(text: &'a T) -> Direction {
    get_base_direction_with_data_source(&HardcodedBidiData, text)
}

/// Get the base direction of the text provided according to the Unicode Bidirectional Algorithm,
/// considering the full text if the first paragraph is all-neutral.
///
/// This is the same as get_base_direction except that it does not stop at the first block
/// separator, but just resets the embedding level and continues to look for a strongly-
/// directional character. So the result will be the base direction of the first paragraph
/// that is not purely neutral characters.
#[cfg(feature = "hardcoded-data")]
#[inline]
pub fn get_base_direction_full<'a, T: TextSource<'a> + ?Sized>(text: &'a T) -> Direction {
    get_base_direction_full_with_data_source(&HardcodedBidiData, text)
}

#[inline]
pub fn get_base_direction_with_data_source<'a, D: BidiDataSource, T: TextSource<'a> + ?Sized>(
    data_source: &D,
    text: &'a T,
) -> Direction {
    get_base_direction_impl(data_source, text, false)
}

#[inline]
pub fn get_base_direction_full_with_data_source<
    'a,
    D: BidiDataSource,
    T: TextSource<'a> + ?Sized,
>(
    data_source: &D,
    text: &'a T,
) -> Direction {
    get_base_direction_impl(data_source, text, true)
}

fn get_base_direction_impl<'a, D: BidiDataSource, T: TextSource<'a> + ?Sized>(
    data_source: &D,
    text: &'a T,
    use_full_text: bool,
) -> Direction {
    let mut isolate_level = 0;
    for c in text.chars() {
        match data_source.bidi_class(c) {
            LRI | RLI | FSI => isolate_level += 1,
            PDI if isolate_level > 0 => isolate_level -= 1,
            L if isolate_level == 0 => return Direction::Ltr,
            R | AL if isolate_level == 0 => return Direction::Rtl,
            B if !use_full_text => break,
            B if use_full_text => isolate_level = 0,
            _ => (),
        }
    }
    // If no strong char was found, return Mixed. Normally this will be treated as Ltr by callers
    // (see rule P3), but we don't map this to Ltr here so that a caller that wants to apply other
    // heuristics to an all-neutral paragraph can tell the difference.
    Direction::Mixed
}

/// Implementation of TextSource for UTF-8 text (a string slice).
impl<'text> TextSource<'text> for str {
    type CharIter = core::str::Chars<'text>;
    type CharIndexIter = core::str::CharIndices<'text>;
    type IndexLenIter = Utf8IndexLenIter<'text>;

    #[inline]
    fn len(&self) -> usize {
        (self as &str).len()
    }
    #[inline]
    fn char_at(&self, index: usize) -> Option<(char, usize)> {
        if let Some(slice) = self.get(index..) {
            if let Some(ch) = slice.chars().next() {
                return Some((ch, ch.len_utf8()));
            }
        }
        None
    }
    #[inline]
    fn subrange(&self, range: Range<usize>) -> &Self {
        &(self as &str)[range]
    }
    #[inline]
    fn chars(&'text self) -> Self::CharIter {
        (self as &str).chars()
    }
    #[inline]
    fn char_indices(&'text self) -> Self::CharIndexIter {
        (self as &str).char_indices()
    }
    #[inline]
    fn indices_lengths(&'text self) -> Self::IndexLenIter {
        Utf8IndexLenIter::new(self)
    }
    #[inline]
    fn char_len(ch: char) -> usize {
        ch.len_utf8()
    }
}

/// Iterator over (UTF-8) string slices returning (index, char_len) tuple.
#[derive(Debug)]
pub struct Utf8IndexLenIter<'text> {
    iter: CharIndices<'text>,
}

impl<'text> Utf8IndexLenIter<'text> {
    #[inline]
    pub fn new(text: &'text str) -> Self {
        Utf8IndexLenIter {
            iter: text.char_indices(),
        }
    }
}

impl Iterator for Utf8IndexLenIter<'_> {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, ch)) = self.iter.next() {
            return Some((pos, ch.len_utf8()));
        }
        None
    }
}

#[cfg(test)]
fn to_utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[cfg(test)]
#[cfg(feature = "hardcoded-data")]
mod tests {
    use super::*;

    use utf16::{
        BidiInfo as BidiInfoU16, InitialInfo as InitialInfoU16, Paragraph as ParagraphU16,
        ParagraphBidiInfo as ParagraphBidiInfoU16,
    };

    #[test]
    fn test_utf16_text_source() {
        let text: &[u16] =
            &[0x41, 0xD801, 0xDC01, 0x20, 0xD800, 0x20, 0xDFFF, 0x20, 0xDC00, 0xD800];
        assert_eq!(text.char_at(0), Some(('A', 1)));
        assert_eq!(text.char_at(1), Some(('\u{10401}', 2)));
        assert_eq!(text.char_at(2), None);
        assert_eq!(text.char_at(3), Some((' ', 1)));
        assert_eq!(text.char_at(4), Some((char::REPLACEMENT_CHARACTER, 1)));
        assert_eq!(text.char_at(5), Some((' ', 1)));
        assert_eq!(text.char_at(6), Some((char::REPLACEMENT_CHARACTER, 1)));
        assert_eq!(text.char_at(7), Some((' ', 1)));
        assert_eq!(text.char_at(8), Some((char::REPLACEMENT_CHARACTER, 1)));
        assert_eq!(text.char_at(9), Some((char::REPLACEMENT_CHARACTER, 1)));
        assert_eq!(text.char_at(10), None);
    }

    #[test]
    fn test_utf16_char_iter() {
        let text: &[u16] =
            &[0x41, 0xD801, 0xDC01, 0x20, 0xD800, 0x20, 0xDFFF, 0x20, 0xDC00, 0xD800];
        assert_eq!(text.len(), 10);
        assert_eq!(text.chars().count(), 9);
        let mut chars = text.chars();
        assert_eq!(chars.next(), Some('A'));
        assert_eq!(chars.next(), Some('\u{10401}'));
        assert_eq!(chars.next(), Some(' '));
        assert_eq!(chars.next(), Some('\u{FFFD}'));
        assert_eq!(chars.next(), Some(' '));
        assert_eq!(chars.next(), Some('\u{FFFD}'));
        assert_eq!(chars.next(), Some(' '));
        assert_eq!(chars.next(), Some('\u{FFFD}'));
        assert_eq!(chars.next(), Some('\u{FFFD}'));
        assert_eq!(chars.next(), None);
    }

    #[test]
    fn test_initial_text_info() {
        let tests = vec![
            (
                // text
                "a1",
                // expected bidi classes per utf-8 byte
                vec![L, EN],
                // expected paragraph-info for utf-8
                vec![ParagraphInfo {
                    range: 0..2,
                    level: LTR_LEVEL,
                }],
                // expected bidi classes per utf-16 code unit
                vec![L, EN],
                // expected paragraph-info for utf-16
                vec![ParagraphInfo {
                    range: 0..2,
                    level: LTR_LEVEL,
                }],
            ),
            (
                // Arabic, space, Hebrew
                "\u{0639} \u{05D0}",
                vec![AL, AL, WS, R, R],
                vec![ParagraphInfo {
                    range: 0..5,
                    level: RTL_LEVEL,
                }],
                vec![AL, WS, R],
                vec![ParagraphInfo {
                    range: 0..3,
                    level: RTL_LEVEL,
                }],
            ),
            (
                // SMP characters from Kharoshthi, Cuneiform, Adlam:
                "\u{10A00}\u{12000}\u{1E900}",
                vec![R, R, R, R, L, L, L, L, R, R, R, R],
                vec![ParagraphInfo {
                    range: 0..12,
                    level: RTL_LEVEL,
                }],
                vec![R, R, L, L, R, R],
                vec![ParagraphInfo {
                    range: 0..6,
                    level: RTL_LEVEL,
                }],
            ),
            (
                "a\u{2029}b",
                vec![L, B, B, B, L],
                vec![
                    ParagraphInfo {
                        range: 0..4,
                        level: LTR_LEVEL,
                    },
                    ParagraphInfo {
                        range: 4..5,
                        level: LTR_LEVEL,
                    },
                ],
                vec![L, B, L],
                vec![
                    ParagraphInfo {
                        range: 0..2,
                        level: LTR_LEVEL,
                    },
                    ParagraphInfo {
                        range: 2..3,
                        level: LTR_LEVEL,
                    },
                ],
            ),
            (
                "\u{2068}א\u{2069}a", // U+2068 FSI, U+2069 PDI
                vec![RLI, RLI, RLI, R, R, PDI, PDI, PDI, L],
                vec![ParagraphInfo {
                    range: 0..9,
                    level: LTR_LEVEL,
                }],
                vec![RLI, R, PDI, L],
                vec![ParagraphInfo {
                    range: 0..4,
                    level: LTR_LEVEL,
                }],
            ),
        ];

        for t in tests {
            assert_eq!(
                InitialInfo::new(t.0, None),
                InitialInfo {
                    text: t.0,
                    original_classes: t.1,
                    paragraphs: t.2,
                }
            );
            let text = &to_utf16(t.0);
            assert_eq!(
                InitialInfoU16::new(text, None),
                InitialInfoU16 {
                    text,
                    original_classes: t.3,
                    paragraphs: t.4,
                }
            );
        }
    }

    #[test]
    #[cfg(feature = "hardcoded-data")]
    fn test_process_text() {
        let tests = vec![
            (
                // text
                "",
                // base level
                Some(RTL_LEVEL),
                // levels
                Level::vec(&[]),
                // original_classes
                vec![],
                // paragraphs
                vec![],
                // levels_u16
                Level::vec(&[]),
                // original_classes_u16
                vec![],
                // paragraphs_u16
                vec![],
            ),
            (
                // text
                "abc123",
                // base level
                Some(LTR_LEVEL),
                // levels
                Level::vec(&[0, 0, 0, 0, 0, 0]),
                // original_classes
                vec![L, L, L, EN, EN, EN],
                // paragraphs
                vec![ParagraphInfo {
                    range: 0..6,
                    level: LTR_LEVEL,
                }],
                // levels_u16
                Level::vec(&[0, 0, 0, 0, 0, 0]),
                // original_classes_u16
                vec![L, L, L, EN, EN, EN],
                // paragraphs_u16
                vec![ParagraphInfo {
                    range: 0..6,
                    level: LTR_LEVEL,
                }],
            ),
            (
                "abc \u{05D0}\u{05D1}\u{05D2}",
                Some(LTR_LEVEL),
                Level::vec(&[0, 0, 0, 0, 1, 1, 1, 1, 1, 1]),
                vec![L, L, L, WS, R, R, R, R, R, R],
                vec![ParagraphInfo {
                    range: 0..10,
                    level: LTR_LEVEL,
                }],
                Level::vec(&[0, 0, 0, 0, 1, 1, 1]),
                vec![L, L, L, WS, R, R, R],
                vec![ParagraphInfo {
                    range: 0..7,
                    level: LTR_LEVEL,
                }],
            ),
            (
                "abc \u{05D0}\u{05D1}\u{05D2}",
                Some(RTL_LEVEL),
                Level::vec(&[2, 2, 2, 1, 1, 1, 1, 1, 1, 1]),
                vec![L, L, L, WS, R, R, R, R, R, R],
                vec![ParagraphInfo {
                    range: 0..10,
                    level: RTL_LEVEL,
                }],
                Level::vec(&[2, 2, 2, 1, 1, 1, 1]),
                vec![L, L, L, WS, R, R, R],
                vec![ParagraphInfo {
                    range: 0..7,
                    level: RTL_LEVEL,
                }],
            ),
            (
                "\u{05D0}\u{05D1}\u{05D2} abc",
                Some(LTR_LEVEL),
                Level::vec(&[1, 1, 1, 1, 1, 1, 0, 0, 0, 0]),
                vec![R, R, R, R, R, R, WS, L, L, L],
                vec![ParagraphInfo {
                    range: 0..10,
                    level: LTR_LEVEL,
                }],
                Level::vec(&[1, 1, 1, 0, 0, 0, 0]),
                vec![R, R, R, WS, L, L, L],
                vec![ParagraphInfo {
                    range: 0..7,
                    level: LTR_LEVEL,
                }],
            ),
            (
                "\u{05D0}\u{05D1}\u{05D2} abc",
                None,
                Level::vec(&[1, 1, 1, 1, 1, 1, 1, 2, 2, 2]),
                vec![R, R, R, R, R, R, WS, L, L, L],
                vec![ParagraphInfo {
                    range: 0..10,
                    level: RTL_LEVEL,
                }],
                Level::vec(&[1, 1, 1, 1, 2, 2, 2]),
                vec![R, R, R, WS, L, L, L],
                vec![ParagraphInfo {
                    range: 0..7,
                    level: RTL_LEVEL,
                }],
            ),
            (
                "\u{063A}2\u{0638} \u{05D0}2\u{05D2}",
                Some(LTR_LEVEL),
                Level::vec(&[1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1]),
                vec![AL, AL, EN, AL, AL, WS, R, R, EN, R, R],
                vec![ParagraphInfo {
                    range: 0..11,
                    level: LTR_LEVEL,
                }],
                Level::vec(&[1, 2, 1, 1, 1, 2, 1]),
                vec![AL, EN, AL, WS, R, EN, R],
                vec![ParagraphInfo {
                    range: 0..7,
                    level: LTR_LEVEL,
                }],
            ),
            (
                "a א.\nג",
                None,
                Level::vec(&[0, 0, 1, 1, 0, 0, 1, 1]),
                vec![L, WS, R, R, CS, B, R, R],
                vec![
                    ParagraphInfo {
                        range: 0..6,
                        level: LTR_LEVEL,
                    },
                    ParagraphInfo {
                        range: 6..8,
                        level: RTL_LEVEL,
                    },
                ],
                Level::vec(&[0, 0, 1, 0, 0, 1]),
                vec![L, WS, R, CS, B, R],
                vec![
                    ParagraphInfo {
                        range: 0..5,
                        level: LTR_LEVEL,
                    },
                    ParagraphInfo {
                        range: 5..6,
                        level: RTL_LEVEL,
                    },
                ],
            ),
            // BidiTest:69635 (AL ET EN)
            (
                "\u{060B}\u{20CF}\u{06F9}",
                None,
                Level::vec(&[1, 1, 1, 1, 1, 2, 2]),
                vec![AL, AL, ET, ET, ET, EN, EN],
                vec![ParagraphInfo {
                    range: 0..7,
                    level: RTL_LEVEL,
                }],
                Level::vec(&[1, 1, 2]),
                vec![AL, ET, EN],
                vec![ParagraphInfo {
                    range: 0..3,
                    level: RTL_LEVEL,
                }],
            ),
        ];

        for t in tests {
            assert_eq!(
                BidiInfo::new(t.0, t.1),
                BidiInfo {
                    text: t.0,
                    levels: t.2.clone(),
                    original_classes: t.3.clone(),
                    paragraphs: t.4.clone(),
                }
            );
            // If it was empty, also test that ParagraphBidiInfo handles it safely.
            if t.4.len() == 0 {
                assert_eq!(
                    ParagraphBidiInfo::new(t.0, t.1),
                    ParagraphBidiInfo {
                        text: t.0,
                        original_classes: t.3.clone(),
                        levels: t.2.clone(),
                        paragraph_level: RTL_LEVEL,
                        is_pure_ltr: true,
                    }
                )
            }
            // If it was a single paragraph, also test ParagraphBidiInfo.
            if t.4.len() == 1 {
                assert_eq!(
                    ParagraphBidiInfo::new(t.0, t.1),
                    ParagraphBidiInfo {
                        text: t.0,
                        original_classes: t.3,
                        levels: t.2.clone(),
                        paragraph_level: t.4[0].level,
                        is_pure_ltr: !level::has_rtl(&t.2),
                    }
                )
            }
            let text = &to_utf16(t.0);
            assert_eq!(
                BidiInfoU16::new(text, t.1),
                BidiInfoU16 {
                    text,
                    levels: t.5.clone(),
                    original_classes: t.6.clone(),
                    paragraphs: t.7.clone(),
                }
            );
            if t.7.len() == 1 {
                assert_eq!(
                    ParagraphBidiInfoU16::new(text, t.1),
                    ParagraphBidiInfoU16 {
                        text: text,
                        original_classes: t.6.clone(),
                        levels: t.5.clone(),
                        paragraph_level: t.7[0].level,
                        is_pure_ltr: !level::has_rtl(&t.5),
                    }
                )
            }
        }
    }

    #[test]
    #[cfg(feature = "hardcoded-data")]
    fn test_paragraph_bidi_info() {
        // Passing text that includes a paragraph break to the ParagraphBidiInfo API:
        // this is a misuse of the API by the client, but our behavior is safe &
        // consistent. The embedded paragraph break acts like a separator (tab) would.
        let tests = vec![
            (
                "a א.\nג",
                None,
                // utf-8 results:
                vec![L, WS, R, R, CS, B, R, R],
                Level::vec(&[0, 0, 1, 1, 1, 1, 1, 1]),
                // utf-16 results:
                vec![L, WS, R, CS, B, R],
                Level::vec(&[0, 0, 1, 1, 1, 1]),
                // paragraph level; is_pure_ltr
                LTR_LEVEL,
                false,
            ),
            (
                "\u{5d1} a.\nb.",
                None,
                // utf-8 results:
                vec![R, R, WS, L, CS, B, L, CS],
                Level::vec(&[1, 1, 1, 2, 2, 2, 2, 1]),
                // utf-16 results:
                vec![R, WS, L, CS, B, L, CS],
                Level::vec(&[1, 1, 2, 2, 2, 2, 1]),
                // paragraph level; is_pure_ltr
                RTL_LEVEL,
                false,
            ),
            (
                "a א.\tג",
                None,
                // utf-8 results:
                vec![L, WS, R, R, CS, S, R, R],
                Level::vec(&[0, 0, 1, 1, 1, 1, 1, 1]),
                // utf-16 results:
                vec![L, WS, R, CS, S, R],
                Level::vec(&[0, 0, 1, 1, 1, 1]),
                // paragraph level; is_pure_ltr
                LTR_LEVEL,
                false,
            ),
            (
                "\u{5d1} a.\tb.",
                None,
                // utf-8 results:
                vec![R, R, WS, L, CS, S, L, CS],
                Level::vec(&[1, 1, 1, 2, 2, 2, 2, 1]),
                // utf-16 results:
                vec![R, WS, L, CS, S, L, CS],
                Level::vec(&[1, 1, 2, 2, 2, 2, 1]),
                // paragraph level; is_pure_ltr
                RTL_LEVEL,
                false,
            ),
        ];

        for t in tests {
            assert_eq!(
                ParagraphBidiInfo::new(t.0, t.1),
                ParagraphBidiInfo {
                    text: t.0,
                    original_classes: t.2,
                    levels: t.3,
                    paragraph_level: t.6,
                    is_pure_ltr: t.7,
                }
            );
            let text = &to_utf16(t.0);
            assert_eq!(
                ParagraphBidiInfoU16::new(text, t.1),
                ParagraphBidiInfoU16 {
                    text: text,
                    original_classes: t.4,
                    levels: t.5,
                    paragraph_level: t.6,
                    is_pure_ltr: t.7,
                }
            );
        }
    }

    #[test]
    #[cfg(feature = "hardcoded-data")]
    fn test_bidi_info_has_rtl() {
        let tests = vec![
            // ASCII only
            ("123", None, false),
            ("123", Some(LTR_LEVEL), false),
            ("123", Some(RTL_LEVEL), false),
            ("abc", None, false),
            ("abc", Some(LTR_LEVEL), false),
            ("abc", Some(RTL_LEVEL), false),
            ("abc 123", None, false),
            ("abc\n123", None, false),
            // With Hebrew
            ("\u{05D0}\u{05D1}\u{05BC}\u{05D2}", None, true),
            ("\u{05D0}\u{05D1}\u{05BC}\u{05D2}", Some(LTR_LEVEL), true),
            ("\u{05D0}\u{05D1}\u{05BC}\u{05D2}", Some(RTL_LEVEL), true),
            ("abc \u{05D0}\u{05D1}\u{05BC}\u{05D2}", None, true),
            ("abc\n\u{05D0}\u{05D1}\u{05BC}\u{05D2}", None, true),
            ("\u{05D0}\u{05D1}\u{05BC}\u{05D2} abc", None, true),
            ("\u{05D0}\u{05D1}\u{05BC}\u{05D2}\nabc", None, true),
            ("\u{05D0}\u{05D1}\u{05BC}\u{05D2} 123", None, true),
            ("\u{05D0}\u{05D1}\u{05BC}\u{05D2}\n123", None, true),
        ];

        for t in tests {
            assert_eq!(BidiInfo::new(t.0, t.1).has_rtl(), t.2);
            assert_eq!(BidiInfoU16::new(&to_utf16(t.0), t.1).has_rtl(), t.2);
        }
    }

    #[cfg(feature = "hardcoded-data")]
    fn reorder_paras(text: &str) -> Vec<Cow<'_, str>> {
        let bidi_info = BidiInfo::new(text, None);
        bidi_info
            .paragraphs
            .iter()
            .map(|para| bidi_info.reorder_line(para, para.range.clone()))
            .collect()
    }

    #[cfg(feature = "hardcoded-data")]
    fn reorder_paras_u16(text: &[u16]) -> Vec<Cow<'_, [u16]>> {
        let bidi_info = BidiInfoU16::new(text, None);
        bidi_info
            .paragraphs
            .iter()
            .map(|para| bidi_info.reorder_line(para, para.range.clone()))
            .collect()
    }

    #[test]
    #[cfg(feature = "hardcoded-data")]
    fn test_reorder_line() {
        let tests = vec![
            // Bidi_Class: L L L B L L L B L L L
            ("abc\ndef\nghi", vec!["abc\n", "def\n", "ghi"]),
            // Bidi_Class: L L EN B L L EN B L L EN
            ("ab1\nde2\ngh3", vec!["ab1\n", "de2\n", "gh3"]),
            // Bidi_Class: L L L B AL AL AL
            ("abc\nابج", vec!["abc\n", "جبا"]),
            // Bidi_Class: AL AL AL B L L L
            (
                "\u{0627}\u{0628}\u{062C}\nabc",
                vec!["\n\u{062C}\u{0628}\u{0627}", "abc"],
            ),
            ("1.-2", vec!["1.-2"]),
            ("1-.2", vec!["1-.2"]),
            ("abc אבג", vec!["abc גבא"]),
            // Numbers being weak LTR characters, cannot reorder strong RTL
            ("123 \u{05D0}\u{05D1}\u{05D2}", vec!["גבא 123"]),
            ("abc\u{202A}def", vec!["abc\u{202A}def"]),
            (
                "abc\u{202A}def\u{202C}ghi",
                vec!["abc\u{202A}def\u{202C}ghi"],
            ),
            (
                "abc\u{2066}def\u{2069}ghi",
                vec!["abc\u{2066}def\u{2069}ghi"],
            ),
            // Testing for RLE Character
            ("\u{202B}abc אבג\u{202C}", vec!["\u{202b}גבא abc\u{202c}"]),
            // Testing neutral characters
            ("\u{05D0}בג? אבג", vec!["גבא ?גבא"]),
            // Testing neutral characters with special case
            ("A אבג?", vec!["A גבא?"]),
            // Testing neutral characters with Implicit RTL Marker
            ("A אבג?\u{200F}", vec!["A \u{200F}?גבא"]),
            ("\u{05D0}בג abc", vec!["abc גבא"]),
            ("abc\u{2067}.-\u{2069}ghi", vec!["abc\u{2067}-.\u{2069}ghi"]),
            (
                "Hello, \u{2068}\u{202E}world\u{202C}\u{2069}!",
                vec!["Hello, \u{2068}\u{202E}\u{202C}dlrow\u{2069}!"],
            ),
            // With mirrorable characters in RTL run
            ("\u{05D0}(ב)ג.", vec![".ג)ב(א"]),
            // With mirrorable characters on level boundary
            ("\u{05D0}ב(גד[&ef].)gh", vec!["gh).]ef&[דג(בא"]),
        ];

        for t in tests {
            assert_eq!(reorder_paras(t.0), t.1);
            let expect_utf16 = t.1.iter().map(|v| to_utf16(v)).collect::<Vec<_>>();
            assert_eq!(reorder_paras_u16(&to_utf16(t.0)), expect_utf16);
        }
    }

    fn reordered_levels_for_paras(text: &str) -> Vec<Vec<Level>> {
        let bidi_info = BidiInfo::new(text, None);
        bidi_info
            .paragraphs
            .iter()
            .map(|para| bidi_info.reordered_levels(para, para.range.clone()))
            .collect()
    }

    fn reordered_levels_per_char_for_paras(text: &str) -> Vec<Vec<Level>> {
        let bidi_info = BidiInfo::new(text, None);
        bidi_info
            .paragraphs
            .iter()
            .map(|para| bidi_info.reordered_levels_per_char(para, para.range.clone()))
            .collect()
    }

    fn reordered_levels_for_paras_u16(text: &[u16]) -> Vec<Vec<Level>> {
        let bidi_info = BidiInfoU16::new(text, None);
        bidi_info
            .paragraphs
            .iter()
            .map(|para| bidi_info.reordered_levels(para, para.range.clone()))
            .collect()
    }

    fn reordered_levels_per_char_for_paras_u16(text: &[u16]) -> Vec<Vec<Level>> {
        let bidi_info = BidiInfoU16::new(text, None);
        bidi_info
            .paragraphs
            .iter()
            .map(|para| bidi_info.reordered_levels_per_char(para, para.range.clone()))
            .collect()
    }

    #[test]
    #[cfg(feature = "hardcoded-data")]
    // See issue #138
    fn test_reordered_levels_range() {
        //                |---------------|
        let s = "\u{202a}A\u{202c}\u{202a}A\u{202c}";
        let range = 4..11;
        assert!(s.get(range.clone()).is_some());

        let bidi = BidiInfo::new(s, None);
        let (_, runs) = bidi.visual_runs(&bidi.paragraphs[0], range);

        for run in runs {
            let _ = &s[run]; // should be valid slice of s
        }
    }

    #[test]
    #[cfg(feature = "hardcoded-data")]
    fn test_reordered_levels() {
        let tests = vec![
            // BidiTest:946 (LRI PDI)
            (
                "\u{2067}\u{2069}",
                vec![Level::vec(&[0, 0, 0, 0, 0, 0])],
                vec![Level::vec(&[0, 0])],
                vec![Level::vec(&[0, 0])],
            ),
            // BidiTest:69635 (AL ET EN)
            (
                "\u{060B}\u{20CF}\u{06F9}",
                vec![Level::vec(&[1, 1, 1, 1, 1, 2, 2])],
                vec![Level::vec(&[1, 1, 2])],
                vec![Level::vec(&[1, 1, 2])],
            ),
        ];

        for t in tests {
            assert_eq!(reordered_levels_for_paras(t.0), t.1);
            assert_eq!(reordered_levels_per_char_for_paras(t.0), t.2);
            let text = &to_utf16(t.0);
            assert_eq!(reordered_levels_for_paras_u16(text), t.3);
            assert_eq!(reordered_levels_per_char_for_paras_u16(text), t.2);
        }

        let tests = vec![
            // BidiTest:291284 (AN RLI PDF R)
            (
                "\u{0605}\u{2067}\u{202C}\u{0590}",
                vec![&["2", "2", "0", "0", "0", "x", "x", "x", "1", "1"]],
                vec![&["2", "0", "x", "1"]],
                vec![&["2", "0", "x", "1"]],
            ),
        ];

        for t in tests {
            assert_eq!(reordered_levels_for_paras(t.0), t.1);
            assert_eq!(reordered_levels_per_char_for_paras(t.0), t.2);
            let text = &to_utf16(t.0);
            assert_eq!(reordered_levels_for_paras_u16(text), t.3);
            assert_eq!(reordered_levels_per_char_for_paras_u16(text), t.2);
        }

        let text = "aa טֶ";
        let bidi_info = BidiInfo::new(text, None);
        assert_eq!(
            bidi_info.reordered_levels(&bidi_info.paragraphs[0], 3..7),
            Level::vec(&[0, 0, 0, 1, 1, 1, 1]),
        );

        let text = &to_utf16(text);
        let bidi_info = BidiInfoU16::new(text, None);
        assert_eq!(
            bidi_info.reordered_levels(&bidi_info.paragraphs[0], 1..4),
            Level::vec(&[0, 0, 0, 1, 1]),
        );
    }

    #[test]
    fn test_paragraph_info_len() {
        let text = "hello world";
        let bidi_info = BidiInfo::new(text, None);
        assert_eq!(bidi_info.paragraphs.len(), 1);
        assert_eq!(bidi_info.paragraphs[0].len(), text.len());

        let text2 = "How are you";
        let whole_text = format!("{}\n{}", text, text2);
        let bidi_info = BidiInfo::new(&whole_text, None);
        assert_eq!(bidi_info.paragraphs.len(), 2);

        // The first paragraph include the paragraph separator.
        // TODO: investigate if the paragraph separator character
        // should not be part of any paragraph.
        assert_eq!(bidi_info.paragraphs[0].len(), text.len() + 1);
        assert_eq!(bidi_info.paragraphs[1].len(), text2.len());

        let text = &to_utf16(text);
        let bidi_info = BidiInfoU16::new(text, None);
        assert_eq!(bidi_info.paragraphs.len(), 1);
        assert_eq!(bidi_info.paragraphs[0].len(), text.len());

        let text2 = &to_utf16(text2);
        let whole_text = &to_utf16(&whole_text);
        let bidi_info = BidiInfoU16::new(&whole_text, None);
        assert_eq!(bidi_info.paragraphs.len(), 2);

        assert_eq!(bidi_info.paragraphs[0].len(), text.len() + 1);
        assert_eq!(bidi_info.paragraphs[1].len(), text2.len());
    }

    #[test]
    fn test_direction() {
        let ltr_text = "hello world";
        let rtl_text = "أهلا بكم";
        let all_paragraphs = format!("{}\n{}\n{}{}", ltr_text, rtl_text, ltr_text, rtl_text);
        let bidi_info = BidiInfo::new(&all_paragraphs, None);
        assert_eq!(bidi_info.paragraphs.len(), 3);
        let p_ltr = Paragraph::new(&bidi_info, &bidi_info.paragraphs[0]);
        let p_rtl = Paragraph::new(&bidi_info, &bidi_info.paragraphs[1]);
        let p_mixed = Paragraph::new(&bidi_info, &bidi_info.paragraphs[2]);
        assert_eq!(p_ltr.direction(), Direction::Ltr);
        assert_eq!(p_rtl.direction(), Direction::Rtl);
        assert_eq!(p_mixed.direction(), Direction::Mixed);

        let all_paragraphs = &to_utf16(&all_paragraphs);
        let bidi_info = BidiInfoU16::new(&all_paragraphs, None);
        assert_eq!(bidi_info.paragraphs.len(), 3);
        let p_ltr = ParagraphU16::new(&bidi_info, &bidi_info.paragraphs[0]);
        let p_rtl = ParagraphU16::new(&bidi_info, &bidi_info.paragraphs[1]);
        let p_mixed = ParagraphU16::new(&bidi_info, &bidi_info.paragraphs[2]);
        assert_eq!(p_ltr.direction(), Direction::Ltr);
        assert_eq!(p_rtl.direction(), Direction::Rtl);
        assert_eq!(p_mixed.direction(), Direction::Mixed);
    }

    #[test]
    fn test_edge_cases_direction() {
        // No paragraphs for empty text.
        let empty = "";
        let bidi_info = BidiInfo::new(empty, Option::from(RTL_LEVEL));
        assert_eq!(bidi_info.paragraphs.len(), 0);

        let empty = &to_utf16(empty);
        let bidi_info = BidiInfoU16::new(empty, Option::from(RTL_LEVEL));
        assert_eq!(bidi_info.paragraphs.len(), 0);

        let tests = vec![
            // The paragraph separator will take the value of the default direction
            // which is left to right.
            ("\n", None, Direction::Ltr),
            // The paragraph separator will take the value of the given initial direction
            // which is left to right.
            ("\n", Option::from(LTR_LEVEL), Direction::Ltr),
            // The paragraph separator will take the value of the given initial direction
            // which is right to left.
            ("\n", Option::from(RTL_LEVEL), Direction::Rtl),
        ];

        for t in tests {
            let bidi_info = BidiInfo::new(t.0, t.1);
            assert_eq!(bidi_info.paragraphs.len(), 1);
            let p = Paragraph::new(&bidi_info, &bidi_info.paragraphs[0]);
            assert_eq!(p.direction(), t.2);
            let text = &to_utf16(t.0);
            let bidi_info = BidiInfoU16::new(text, t.1);
            let p = ParagraphU16::new(&bidi_info, &bidi_info.paragraphs[0]);
            assert_eq!(p.direction(), t.2);
        }
    }

    #[test]
    fn test_level_at() {
        let ltr_text = "hello world";
        let rtl_text = "أهلا بكم";
        let all_paragraphs = format!("{}\n{}\n{}{}", ltr_text, rtl_text, ltr_text, rtl_text);
        let bidi_info = BidiInfo::new(&all_paragraphs, None);
        assert_eq!(bidi_info.paragraphs.len(), 3);

        let p_ltr = Paragraph::new(&bidi_info, &bidi_info.paragraphs[0]);
        let p_rtl = Paragraph::new(&bidi_info, &bidi_info.paragraphs[1]);
        let p_mixed = Paragraph::new(&bidi_info, &bidi_info.paragraphs[2]);

        assert_eq!(p_ltr.level_at(0), LTR_LEVEL);
        assert_eq!(p_rtl.level_at(0), RTL_LEVEL);
        assert_eq!(p_mixed.level_at(0), LTR_LEVEL);
        assert_eq!(p_mixed.info.levels.len(), 54);
        assert_eq!(p_mixed.para.range.start, 28);
        assert_eq!(p_mixed.level_at(ltr_text.len()), RTL_LEVEL);

        let all_paragraphs = &to_utf16(&all_paragraphs);
        let bidi_info = BidiInfoU16::new(&all_paragraphs, None);
        assert_eq!(bidi_info.paragraphs.len(), 3);

        let p_ltr = ParagraphU16::new(&bidi_info, &bidi_info.paragraphs[0]);
        let p_rtl = ParagraphU16::new(&bidi_info, &bidi_info.paragraphs[1]);
        let p_mixed = ParagraphU16::new(&bidi_info, &bidi_info.paragraphs[2]);

        assert_eq!(p_ltr.level_at(0), LTR_LEVEL);
        assert_eq!(p_rtl.level_at(0), RTL_LEVEL);
        assert_eq!(p_mixed.level_at(0), LTR_LEVEL);
        assert_eq!(p_mixed.info.levels.len(), 40);
        assert_eq!(p_mixed.para.range.start, 21);
        assert_eq!(p_mixed.level_at(ltr_text.len()), RTL_LEVEL);
    }

    #[test]
    fn test_get_base_direction() {
        let tests = vec![
            ("", Direction::Mixed), // return Mixed if no strong character found
            ("123[]-+\u{2019}\u{2060}\u{00bf}?", Direction::Mixed),
            ("3.14\npi", Direction::Mixed), // only first paragraph is considered
            ("[123 'abc']", Direction::Ltr),
            ("[123 '\u{0628}' abc", Direction::Rtl),
            ("[123 '\u{2066}abc\u{2069}'\u{0628}]", Direction::Rtl), // embedded isolate is ignored
            ("[123 '\u{2066}abc\u{2068}'\u{0628}]", Direction::Mixed),
        ];

        for t in tests {
            assert_eq!(get_base_direction(t.0), t.1);
            let text = &to_utf16(t.0);
            assert_eq!(get_base_direction(text.as_slice()), t.1);
        }
    }

    #[test]
    fn test_get_base_direction_full() {
        let tests = vec![
            ("", Direction::Mixed), // return Mixed if no strong character found
            ("123[]-+\u{2019}\u{2060}\u{00bf}?", Direction::Mixed),
            ("3.14\npi", Direction::Ltr), // direction taken from the second paragraph
            ("3.14\n\u{05D0}", Direction::Rtl), // direction taken from the second paragraph
            ("[123 'abc']", Direction::Ltr),
            ("[123 '\u{0628}' abc", Direction::Rtl),
            ("[123 '\u{2066}abc\u{2069}'\u{0628}]", Direction::Rtl), // embedded isolate is ignored
            ("[123 '\u{2066}abc\u{2068}'\u{0628}]", Direction::Mixed),
            ("[123 '\u{2066}abc\u{2068}'\n\u{0628}]", Direction::Rtl), // \n resets embedding level
        ];

        for t in tests {
            assert_eq!(get_base_direction_full(t.0), t.1);
            let text = &to_utf16(t.0);
            assert_eq!(get_base_direction_full(text.as_slice()), t.1);
        }
    }
}

#[cfg(all(feature = "serde", feature = "hardcoded-data", test))]
mod serde_tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_levels() {
        let text = "abc אבג";
        let bidi_info = BidiInfo::new(text, None);
        let levels = bidi_info.levels;
        assert_eq!(text.as_bytes().len(), 10);
        assert_eq!(levels.len(), 10);
        assert_tokens(
            &levels,
            &[
                Token::Seq { len: Some(10) },
                Token::NewtypeStruct { name: "Level" },
                Token::U8(0),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(0),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(0),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(0),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(1),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(1),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(1),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(1),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(1),
                Token::NewtypeStruct { name: "Level" },
                Token::U8(1),
                Token::SeqEnd,
            ],
        );
    }
}
