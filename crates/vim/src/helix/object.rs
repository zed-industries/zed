use std::{
    error::Error,
    fmt::{self, Display},
    ops::Range,
};

use editor::{DisplayPoint, display_map::DisplaySnapshot, movement};
use text::Selection;

use crate::{
    helix::boundary::{FuzzyBoundary, ImmediateBoundary},
    object::Object as VimObject,
};

/// A text object from helix or an extra one
pub trait HelixTextObject {
    fn range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>>;

    fn next_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>>;

    fn previous_range(
        &self,
        map: &DisplaySnapshot,
        relative_to: Range<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>>;
}

impl VimObject {
    /// Returns the range of the object the cursor is over.
    /// Follows helix convention.
    pub fn helix_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Result<Option<Range<DisplayPoint>>, VimToHelixError> {
        let cursor = cursor_range(&selection, map);
        if let Some(helix_object) = self.to_helix_object() {
            Ok(helix_object.range(map, cursor, around))
        } else {
            Err(VimToHelixError)
        }
    }
    /// Returns the range of the next object the cursor is not over.
    /// Follows helix convention.
    pub fn helix_next_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Result<Option<Range<DisplayPoint>>, VimToHelixError> {
        let cursor = cursor_range(&selection, map);
        if let Some(helix_object) = self.to_helix_object() {
            Ok(helix_object.next_range(map, cursor, around))
        } else {
            Err(VimToHelixError)
        }
    }
    /// Returns the range of the previous object the cursor is not over.
    /// Follows helix convention.
    pub fn helix_previous_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Result<Option<Range<DisplayPoint>>, VimToHelixError> {
        let cursor = cursor_range(&selection, map);
        if let Some(helix_object) = self.to_helix_object() {
            Ok(helix_object.previous_range(map, cursor, around))
        } else {
            Err(VimToHelixError)
        }
    }
}

#[derive(Debug)]
pub struct VimToHelixError;
impl Display for VimToHelixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Not all vim text objects have an implemented helix equivalent"
        )
    }
}
impl Error for VimToHelixError {}

impl VimObject {
    fn to_helix_object(self) -> Option<Box<dyn HelixTextObject>> {
        Some(match self {
            Self::AngleBrackets => Box::new(ImmediateBoundary::AngleBrackets),
            Self::BackQuotes => Box::new(ImmediateBoundary::BackQuotes),
            Self::CurlyBrackets => Box::new(ImmediateBoundary::CurlyBrackets),
            Self::DoubleQuotes => Box::new(ImmediateBoundary::DoubleQuotes),
            Self::Paragraph => Box::new(FuzzyBoundary::Paragraph),
            Self::Parentheses => Box::new(ImmediateBoundary::Parentheses),
            Self::Quotes => Box::new(ImmediateBoundary::SingleQuotes),
            Self::Sentence => Box::new(FuzzyBoundary::Sentence),
            Self::SquareBrackets => Box::new(ImmediateBoundary::SquareBrackets),
            Self::Subword { ignore_punctuation } => {
                Box::new(ImmediateBoundary::Subword { ignore_punctuation })
            }
            Self::VerticalBars => Box::new(ImmediateBoundary::VerticalBars),
            Self::Word { ignore_punctuation } => {
                Box::new(ImmediateBoundary::Word { ignore_punctuation })
            }
            _ => return None,
        })
    }
}

/// Returns the start of the cursor of a selection, whether that is collapsed or not.
pub(crate) fn cursor_range(
    selection: &Selection<DisplayPoint>,
    map: &DisplaySnapshot,
) -> Range<DisplayPoint> {
    if selection.is_empty() | selection.reversed {
        selection.head()..movement::right(map, selection.head())
    } else {
        movement::left(map, selection.head())..selection.head()
    }
}

#[cfg(test)]
mod test {
    use db::indoc;

    use crate::{state::Mode, test::VimTestContext};

    #[gpui::test]
    async fn test_select_word_object(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        let start = indoc! {"
                The quick brˇowˇnˇ
                fox «ˇjumps» ov«er
                the laˇ»zy dogˇ

                "
        };

        cx.set_state(start, Mode::HelixNormal);

        cx.simulate_keystrokes("m i w");

        cx.assert_state(
            indoc! {"
            The quick «brownˇ»
            fox «jumpsˇ» over
            the «lazyˇ» dogˇ

            "
            },
            Mode::HelixNormal,
        );

        cx.set_state(start, Mode::HelixNormal);

        cx.simulate_keystrokes("m a w");

        cx.assert_state(
            indoc! {"
            The quick« brownˇ»
            fox «jumps ˇ»over
            the «lazy ˇ»dogˇ

            "
            },
            Mode::HelixNormal,
        );
    }
}
