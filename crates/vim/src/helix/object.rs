use std::{cmp::Ordering, ops::Range};

use editor::{
    DisplayPoint,
    display_map::DisplaySnapshot,
    movement::{self},
};
use text::Selection;

use crate::{helix::boundary::UnboundedErr, object::Object};

impl Object {
    /// Returns the range of the object the cursor is over.
    /// Follows helix convention.
    pub fn helix_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = selection.head();
        if let Ok(selection) = self.current_bounded_object(map, relative_to) {
            if around {
                selection.map(|s| self.surround(map, s).unwrap())
            } else {
                selection
            }
        } else {
            let head = selection.head();
            let range = self.range(map, selection, around, None)?;

            if range.start > head {
                None
            } else {
                Some(range)
            }
        }
    }

    /// Returns the range of the next object the cursor is not over.
    /// Follows helix convention.
    pub fn helix_next_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = selection.head();
        if let Ok(selection) = self.next_bounded_object(map, relative_to) {
            if around {
                selection.map(|s| self.surround(map, s).unwrap())
            } else {
                selection
            }
        } else {
            let head = selection.head();
            let range = self.range(map, selection, around, None)?;

            if range.start > head {
                Some(range)
            } else {
                None
            }
        }
    }

    /// Returns the range of the previous object the cursor is not over.
    /// Follows helix convention.
    pub fn helix_previous_range(
        self,
        map: &DisplaySnapshot,
        selection: Selection<DisplayPoint>,
        around: bool,
    ) -> Option<Range<DisplayPoint>> {
        let relative_to = selection.head();
        if let Ok(selection) = self.previous_bounded_object(map, relative_to) {
            if around {
                selection.map(|s| self.surround(map, s).unwrap())
            } else {
                selection
            }
        } else {
            None
        }
    }

    /// Returns the range of the object the cursor is over if it can be found with simple boundary checking. Potentially none. Follows helix convention.
    fn current_bounded_object(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<Range<DisplayPoint>>, UnboundedErr> {
        let maybe_prev_end = self.helix_previous_end(map, relative_to)?;
        let Some(prev_start) = self.helix_previous_start(map, relative_to)? else {
            return Ok(None);
        };
        let Some(next_end) = self.helix_next_end(map, movement::right(map, relative_to))? else {
            return Ok(None);
        };
        let maybe_next_start = self.helix_next_start(map, movement::right(map, relative_to))?;

        if let Some(next_start) = maybe_next_start {
            match next_start.cmp(&next_end) {
                Ordering::Less => return Ok(None),
                Ordering::Equal if self.can_be_zero_width() => return Ok(None),
                _ => (),
            }
        }
        if let Some(prev_end) = maybe_prev_end {
            if prev_start == prev_end && self.can_be_zero_width() {
                return Ok(None);
            }
            debug_assert!(prev_end <= prev_start)
        }

        Ok(Some(prev_start..next_end))
    }

    /// Returns the range of the next object the cursor is not over if it can be found with simple boundary checking. Potentially none. Follows helix convention.
    fn next_bounded_object(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<Range<DisplayPoint>>, UnboundedErr> {
        let Some(next_start) = self.helix_next_start(map, movement::right(map, relative_to))?
        else {
            return Ok(None);
        };
        let search_start = if self.can_be_zero_width() {
            next_start
        } else {
            movement::right(map, next_start)
        };
        let Some(end) = self.helix_next_end(map, search_start)? else {
            return Ok(None);
        };

        Ok(Some(next_start..end))
    }

    /// Returns the previous range of the object the cursor not is over if it can be found with simple boundary checking. Potentially none. Follows helix convention.
    fn previous_bounded_object(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<Range<DisplayPoint>>, UnboundedErr> {
        let Some(prev_end) = self.helix_previous_end(map, relative_to)? else {
            return Ok(None);
        };
        let search_start = if self.can_be_zero_width() {
            prev_end
        } else {
            movement::left(map, prev_end)
        };
        let Some(start) = self.helix_previous_start(map, search_start)? else {
            return Ok(None);
        };

        Ok(Some(start..prev_end))
    }

    /// Switches from an 'mi' range to an 'ma' range. Follows helix convention.
    fn surround(
        self,
        map: &DisplaySnapshot,
        selection: Range<DisplayPoint>,
    ) -> Result<Range<DisplayPoint>, UnboundedErr> {
        match self {
            Self::Word { .. } | Self::Subword { .. } => {
                let row = selection.end.row();
                let line_start = DisplayPoint::new(row, 0);
                let line_end = DisplayPoint::new(row, map.line_len(row));
                let next_start = self
                    .helix_next_start(map, selection.end)
                    .unwrap()
                    .unwrap()
                    .min(line_end);
                let prev_end = self
                    .helix_previous_end(map, selection.start)
                    .unwrap()
                    .unwrap()
                    .max(line_start);
                if next_start > selection.end {
                    Ok(selection.start..next_start)
                } else {
                    Ok(prev_end..selection.end)
                }
            }
            Self::AngleBrackets
            | Self::BackQuotes
            | Self::CurlyBrackets
            | Self::DoubleQuotes
            | Self::Parentheses
            | Self::SquareBrackets
            | Self::VerticalBars => {
                Ok(movement::left(map, selection.start)..movement::right(map, selection.end))
            }
            _ => Err(UnboundedErr),
        }
    }

    const fn can_be_zero_width(&self) -> bool {
        match self {
            Self::AngleBrackets
            | Self::AnyBrackets
            | Self::AnyQuotes
            | Self::BackQuotes
            | Self::CurlyBrackets
            | Self::DoubleQuotes
            | Self::EntireFile
            | Self::MiniBrackets
            | Self::MiniQuotes
            | Self::Parentheses
            | Self::Quotes
            | Self::SquareBrackets
            | Self::VerticalBars => true,
            _ => false,
        }
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
