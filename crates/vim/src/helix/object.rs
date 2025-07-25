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
        let relative_to = cursor_start(&selection, map);
        if let Ok(selection) = self.current_bounded_object(map, relative_to) {
            if around {
                selection.map(|s| self.surround(map, s).unwrap())
            } else {
                selection
            }
        } else {
            let range = self.range(map, selection, around, None)?;

            if range.start > relative_to {
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
        let relative_to = cursor_start(&selection, map);
        if let Ok(selection) = self.next_bounded_object(map, relative_to) {
            if around {
                selection.map(|s| self.surround(map, s).unwrap())
            } else {
                selection
            }
        } else {
            let range = self.range(map, selection, around, None)?;

            if range.start > relative_to {
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
        let relative_to = cursor_start(&selection, map);
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

    /// Returns the range of the object the cursor is over if it can be found with simple boundary checking.
    /// Potentially none. Follows helix convention.
    fn current_bounded_object(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<Range<DisplayPoint>>, UnboundedErr> {
        let Some(start) = self.helix_previous_start(map, relative_to)? else {
            return Ok(None);
        };
        let Some(end) = self.close_at_end(start, map)? else {
            return Ok(None);
        };

        if end > relative_to {
            return Ok(Some(start..end));
        }

        let Some(end) = self.helix_next_end(map, movement::right(map, relative_to))? else {
            return Ok(None);
        };
        let Some(start) = self.close_at_start(end, map)? else {
            return Ok(None);
        };

        if start <= relative_to {
            return Ok(Some(start..end));
        }

        Ok(None)
    }

    /// Returns the range of the next object the cursor is not over if it can be found with simple boundary checking.
    /// Potentially none. Follows helix convention.
    fn next_bounded_object(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<Range<DisplayPoint>>, UnboundedErr> {
        let Some(next_start) = self.helix_next_start(map, movement::right(map, relative_to))?
        else {
            return Ok(None);
        };
        let Some(end) = self.close_at_end(next_start, map)? else {
            return Ok(None);
        };

        Ok(Some(next_start..end))
    }

    /// Returns the previous range of the object the cursor not is over if it can be found with simple boundary checking.
    /// Potentially none. Follows helix convention.
    fn previous_bounded_object(
        self,
        map: &DisplaySnapshot,
        relative_to: DisplayPoint,
    ) -> Result<Option<Range<DisplayPoint>>, UnboundedErr> {
        let Some(prev_end) = self.helix_previous_end(map, relative_to)? else {
            return Ok(None);
        };
        let Some(start) = self.close_at_start(prev_end, map)? else {
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

    fn close_at_end(
        self,
        start: DisplayPoint,
        map: &DisplaySnapshot,
    ) -> Result<Option<DisplayPoint>, UnboundedErr> {
        let mut last_start = movement::right(map, start);
        let mut opened = 1;
        while let Some(next_end) = self.helix_next_end(map, last_start)? {
            if !self.can_be_nested() {
                return Ok(Some(next_end));
            }
            if let Some(next_start) = self.helix_next_start(map, last_start)? {
                match next_start.cmp(&next_end) {
                    Ordering::Less => {
                        opened += 1;
                        last_start = movement::right(map, next_start);
                        continue;
                    }
                    Ordering::Equal if self.can_be_zero_width() => {
                        last_start = movement::right(map, next_start);
                        continue;
                    }
                    _ => (),
                }
            }
            // When this is reached one opened object can be closed.
            opened -= 1;
            if opened == 0 {
                return Ok(Some(next_end));
            }
            last_start = movement::right(map, next_end);
        }
        Ok(None)
    }

    fn close_at_start(
        self,
        end: DisplayPoint,
        map: &DisplaySnapshot,
    ) -> Result<Option<DisplayPoint>, UnboundedErr> {
        let mut last_end = movement::left(map, end);
        let mut opened = 1;
        while let Some(previous_start) = self.helix_previous_start(map, last_end)? {
            if !self.can_be_nested() {
                return Ok(Some(previous_start));
            }
            if let Some(previous_end) = self.helix_previous_end(map, last_end)? {
                if previous_end > previous_start
                    || previous_end == previous_start && self.can_be_zero_width()
                {
                    opened += 1;
                    last_end = movement::left(map, previous_end);
                    continue;
                }
            }
            // When this is reached one opened object can be closed.
            opened -= 1;
            if opened == 0 {
                return Ok(Some(previous_start));
            }
            last_end = movement::left(map, previous_start);
        }
        Ok(None)
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

    const fn can_be_nested(&self) -> bool {
        match self {
            Self::AngleBrackets
            | Self::AnyBrackets
            | Self::CurlyBrackets
            | Self::MiniBrackets
            | Self::Parentheses
            | Self::SquareBrackets
            | Self::AnyQuotes
            | Self::Class
            | Self::Method
            | Self::Tag
            | Self::Argument => true,
            _ => false,
        }
    }
}

/// Returns the start of the cursor of a selection, whether that is collapsed or not.
fn cursor_start(selection: &Selection<DisplayPoint>, map: &DisplaySnapshot) -> DisplayPoint {
    if selection.is_empty() | selection.reversed {
        selection.head()
    } else {
        movement::left(map, selection.head())
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
