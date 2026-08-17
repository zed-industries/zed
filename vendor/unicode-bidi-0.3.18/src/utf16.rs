// Copyright 2023 The Mozilla Foundation. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::TextSource;

use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::char;
use core::ops::Range;

use crate::{
    compute_bidi_info_for_para, compute_initial_info, level, para_direction, reorder_levels,
    reorder_visual, visual_runs_for_line,
};
use crate::{
    BidiClass, BidiDataSource, Direction, Level, LevelRun, ParagraphInfo, ParagraphInfoFlags,
};

#[cfg(feature = "hardcoded-data")]
use crate::HardcodedBidiData;

/// Initial bidi information of the text (UTF-16 version).
///
/// Contains the text paragraphs and `BidiClass` of its characters.
#[derive(PartialEq, Debug)]
pub struct InitialInfo<'text> {
    /// The text
    pub text: &'text [u16],

    /// The BidiClass of the character at each code unit in the text.
    /// If a character is multiple code units, its class will appear multiple times in the vector.
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
    pub fn new(text: &[u16], default_para_level: Option<Level>) -> InitialInfo<'_> {
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
        text: &'a [u16],
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
    /// control codes present).
    flags: Vec<ParagraphInfoFlags>,
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
        text: &'a [u16],
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

/// Bidi information of the text (UTF-16 version).
///
/// The `original_classes` and `levels` vectors are indexed by code unit offsets into the text.  If a
/// character is multiple code units wide, then its class and level will appear multiple times in these
/// vectors.
// TODO: Impl `struct StringProperty<T> { values: Vec<T> }` and use instead of Vec<T>
#[derive(Debug, PartialEq)]
pub struct BidiInfo<'text> {
    /// The text
    pub text: &'text [u16],

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
    pub fn new(text: &[u16], default_para_level: Option<Level>) -> BidiInfo<'_> {
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
        text: &'a [u16],
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
        let line_str: &[u16] = &self.text[line.clone()];

        reorder_levels(line_classes, line_levels, line_str, para.level);

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
    pub fn reorder_line(&self, para: &ParagraphInfo, line: Range<usize>) -> Cow<'text, [u16]> {
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
/// The `original_classes` and `levels` vectors are indexed by code unit offsets into the text.  If a
/// character is multiple code units wide, then its class and level will appear multiple times in these
/// vectors.
#[derive(Debug, PartialEq)]
pub struct ParagraphBidiInfo<'text> {
    /// The text
    pub text: &'text [u16],

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
    pub fn new(text: &[u16], default_para_level: Option<Level>) -> ParagraphBidiInfo<'_> {
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
        text: &'a [u16],
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

    /// Produce the levels for this paragraph as needed for reordering, one level per *code unit*
    /// in the paragraph. The returned vector includes code units that are not included
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
    pub fn reorder_line(&self, line: Range<usize>) -> Cow<'text, [u16]> {
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
    /// `line` is a range of code-unit indices within `levels`.
    ///
    /// See `BidiInfo::visual_runs` for details.
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
    text: &[u16],
    line: Range<usize>,
    levels: Vec<Level>,
    runs: Vec<LevelRun>,
) -> Cow<'_, [u16]> {
    // If all isolating run sequences are LTR, no reordering is needed
    if runs.iter().all(|run| levels[run.start].is_ltr()) {
        return text[line].into();
    }

    let mut result = Vec::<u16>::with_capacity(line.len());
    for run in runs {
        if levels[run.start].is_rtl() {
            let mut buf = [0; 2];
            for c in text[run].chars().rev() {
                result.extend(c.encode_utf16(&mut buf).iter());
            }
        } else {
            result.extend(text[run].iter());
        }
    }
    result.into()
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

/// Implementation of TextSource for UTF-16 text in a [u16] array.
/// Note that there could be unpaired surrogates present!

// Convenience functions to check whether a UTF16 code unit is a surrogate.
#[inline]
fn is_high_surrogate(code: u16) -> bool {
    (code & 0xFC00) == 0xD800
}
#[inline]
fn is_low_surrogate(code: u16) -> bool {
    (code & 0xFC00) == 0xDC00
}

impl<'text> TextSource<'text> for [u16] {
    type CharIter = Utf16CharIter<'text>;
    type CharIndexIter = Utf16CharIndexIter<'text>;
    type IndexLenIter = Utf16IndexLenIter<'text>;

    #[inline]
    fn len(&self) -> usize {
        (self as &[u16]).len()
    }
    fn char_at(&self, index: usize) -> Option<(char, usize)> {
        if index >= self.len() {
            return None;
        }
        // Get the indicated code unit and try simply converting it to a char;
        // this will fail if it is half of a surrogate pair.
        let c = self[index];
        if let Some(ch) = char::from_u32(c.into()) {
            return Some((ch, 1));
        }
        // If it's a low surrogate, and was immediately preceded by a high surrogate,
        // then we're in the middle of a (valid) character, and should return None.
        if is_low_surrogate(c) && index > 0 && is_high_surrogate(self[index - 1]) {
            return None;
        }
        // Otherwise, try to decode, returning REPLACEMENT_CHARACTER for errors.
        if let Some(ch) = char::decode_utf16(self[index..].iter().cloned()).next() {
            if let Ok(ch) = ch {
                // This must be a surrogate pair, otherwise char::from_u32() above should
                // have succeeded!
                debug_assert!(ch.len_utf16() == 2, "BMP should have already been handled");
                return Some((ch, ch.len_utf16()));
            }
        } else {
            debug_assert!(
                false,
                "Why did decode_utf16 return None when we're not at the end?"
            );
            return None;
        }
        // Failed to decode UTF-16: we must have encountered an unpaired surrogate.
        // Return REPLACEMENT_CHARACTER (not None), to continue processing the following text
        // and keep indexing correct.
        Some((char::REPLACEMENT_CHARACTER, 1))
    }
    #[inline]
    fn subrange(&self, range: Range<usize>) -> &Self {
        &(self as &[u16])[range]
    }
    #[inline]
    fn chars(&'text self) -> Self::CharIter {
        Utf16CharIter::new(self)
    }
    #[inline]
    fn char_indices(&'text self) -> Self::CharIndexIter {
        Utf16CharIndexIter::new(self)
    }
    #[inline]
    fn indices_lengths(&'text self) -> Self::IndexLenIter {
        Utf16IndexLenIter::new(self)
    }
    #[inline]
    fn char_len(ch: char) -> usize {
        ch.len_utf16()
    }
}

/// Iterator over UTF-16 text in a [u16] slice, returning (index, char_len) tuple.
#[derive(Debug)]
pub struct Utf16IndexLenIter<'text> {
    text: &'text [u16],
    cur_pos: usize,
}

impl<'text> Utf16IndexLenIter<'text> {
    #[inline]
    pub fn new(text: &'text [u16]) -> Self {
        Utf16IndexLenIter { text, cur_pos: 0 }
    }
}

impl Iterator for Utf16IndexLenIter<'_> {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, char_len)) = self.text.char_at(self.cur_pos) {
            let result = (self.cur_pos, char_len);
            self.cur_pos += char_len;
            return Some(result);
        }
        None
    }
}

/// Iterator over UTF-16 text in a [u16] slice, returning (index, char) tuple.
#[derive(Debug)]
pub struct Utf16CharIndexIter<'text> {
    text: &'text [u16],
    cur_pos: usize,
}

impl<'text> Utf16CharIndexIter<'text> {
    pub fn new(text: &'text [u16]) -> Self {
        Utf16CharIndexIter { text, cur_pos: 0 }
    }
}

impl Iterator for Utf16CharIndexIter<'_> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((ch, char_len)) = self.text.char_at(self.cur_pos) {
            let result = (self.cur_pos, ch);
            self.cur_pos += char_len;
            return Some(result);
        }
        None
    }
}

/// Iterator over UTF-16 text in a [u16] slice, returning Unicode chars.
/// (Unlike the other iterators above, this also supports reverse iteration.)
#[derive(Debug)]
pub struct Utf16CharIter<'text> {
    text: &'text [u16],
    cur_pos: usize,
    end_pos: usize,
}

impl<'text> Utf16CharIter<'text> {
    pub fn new(text: &'text [u16]) -> Self {
        Utf16CharIter {
            text,
            cur_pos: 0,
            end_pos: text.len(),
        }
    }
}

impl Iterator for Utf16CharIter<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((ch, char_len)) = self.text.char_at(self.cur_pos) {
            self.cur_pos += char_len;
            return Some(ch);
        }
        None
    }
}

impl DoubleEndedIterator for Utf16CharIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end_pos <= self.cur_pos {
            return None;
        }
        self.end_pos -= 1;
        if let Some(ch) = char::from_u32(self.text[self.end_pos] as u32) {
            return Some(ch);
        }
        if self.end_pos > self.cur_pos {
            if let Some((ch, char_len)) = self.text.char_at(self.end_pos - 1) {
                if char_len == 2 {
                    self.end_pos -= 1;
                    return Some(ch);
                }
            }
        }
        Some(char::REPLACEMENT_CHARACTER)
    }
}
