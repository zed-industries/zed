// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc::vec::Vec;
use unicode_bidi::{bidi_class, BidiClass, BidiInfo, ParagraphInfo};

/// An iterator over the paragraphs in the input text.
/// It is equivalent to [`core::str::Lines`] but follows `unicode-bidi` behaviour.
#[derive(Debug)]
pub struct BidiParagraphs<'text> {
    text: &'text str,
    info: alloc::vec::IntoIter<ParagraphInfo>,
}

impl<'text> BidiParagraphs<'text> {
    /// Create an iterator with optimized paragraph detection.
    /// This version avoids `BidiInfo` allocation for simple ASCII text.
    pub fn new(text: &'text str) -> Self {
        // Fast path for simple ASCII text - just split on newlines
        if text.is_ascii()
            && !text
                .chars()
                .any(|c| c.is_ascii_control() && c != '\n' && c != '\r' && c != '\t')
        {
            // For simple ASCII, we can avoid `BidiInfo` entirely
            // Create minimal ParagraphInfo entries for each line
            let mut paragraphs = Vec::new();
            let mut start = 0;

            for (i, c) in text.char_indices() {
                if c == '\n' {
                    paragraphs.push(ParagraphInfo {
                        range: start..i,
                        level: unicode_bidi::Level::ltr(),
                    });
                    start = i + 1;
                }
            }

            // Add final paragraph if text doesn't end with newline
            if start < text.len() {
                paragraphs.push(ParagraphInfo {
                    range: start..text.len(),
                    level: unicode_bidi::Level::ltr(),
                });
            }

            let info = paragraphs.into_iter();
            Self { text, info }
        } else {
            // Complex text - fall back to full `BidiInfo` analysis
            let info = BidiInfo::new(text, None);
            let info = info.paragraphs.into_iter();
            Self { text, info }
        }
    }
}

impl<'text> Iterator for BidiParagraphs<'text> {
    type Item = &'text str;

    fn next(&mut self) -> Option<Self::Item> {
        let para = self.info.next()?;
        let paragraph = &self.text[para.range];
        // `para.range` includes the newline that splits the line, so remove it if present
        let mut char_indices = paragraph.char_indices();
        char_indices
            .next_back()
            .and_then(|(i, c)| {
                // `BidiClass::B` is a Paragraph_Separator (various newline characters)
                (bidi_class(c) == BidiClass::B).then_some(i)
            })
            .map_or(Some(paragraph), |i| Some(&paragraph[0..i]))
    }
}
