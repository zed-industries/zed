use std::ops::Range;

use editor::{
    DisplayPoint,
    display_map::DisplaySnapshot,
    movement::{self, FindRange},
};
use language::CharKind;
use text::{Bias, Selection};

use crate::{
    motion::right,
    object::{Object, expand_to_include_whitespace},
};

impl Object {
    /// Returns
    /// Follows helix convention.
    pub fn helix_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = selection.head();
        match self {
            Object::Word { ignore_punctuation } => {
                if around {
                    helix_around_word(map, relative_to, ignore_punctuation)
                } else {
                    helix_in_word(map, relative_to, ignore_punctuation)
                }
            }
            _ => self.range(map, selection, around, None),
        }
    }
}

/// Returns a range that surrounds the word `relative_to` is in.
///
/// If `relative_to` is between words, return `None`.
fn helix_in_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Option<Range<DisplayPoint>> {
    // Use motion::right so that we consider the character under the cursor when looking for the start
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(relative_to.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    let char = map
        .buffer_chars_at(relative_to.to_offset(map, Bias::Left))
        .next()?
        .0;

    if classifier.kind(char) == CharKind::Whitespace {
        return None;
    }

    let start = movement::find_preceding_boundary_display_point(
        map,
        right(map, relative_to, 1),
        movement::FindRange::SingleLine,
        |left, right| classifier.kind(left) != classifier.kind(right),
    );

    let end = movement::find_boundary(map, relative_to, FindRange::SingleLine, |left, right| {
        classifier.kind(left) != classifier.kind(right)
    });

    Some(start..end)
}

/// Returns the range of the word the cursor is over and all the whitespace on one side.
/// If there is whitespace after that is included, otherwise it's whitespace before the word if any.
fn helix_around_word(
    map: &DisplaySnapshot,
    relative_to: DisplayPoint,
    ignore_punctuation: bool,
) -> Option<Range<DisplayPoint>> {
    let word_range = helix_in_word(map, relative_to, ignore_punctuation)?;

    Some(expand_to_include_whitespace(map, word_range, true))
}

#[cfg(test)]
mod test {
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
