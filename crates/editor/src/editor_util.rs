use std::{iter::Peekable, ops::Range};

use language::{Point, Selection};

use crate::display_map::DisplaySnapshot;

type RowIndex = u32;

pub fn end_row_for(selection: &Selection<Point>, display_map: &DisplaySnapshot) -> RowIndex {
    let mut end_row = if selection.end.column > 0 || selection.is_empty() {
        display_map.next_line_boundary(selection.end).0.row + 1
    } else {
        selection.end.row
    };
    end_row
}

struct ContiguousRowRanges<'snapshot, I: Iterator> {
    selections: Peekable<I>,
    display_map: &'snapshot DisplaySnapshot,
}

impl<'snapshot, I: Iterator> ContiguousRowRanges<'snapshot, I> {
    fn new(selections: I, display_map: &DisplaySnapshot) -> Self {
        Self {
            selections: selections.peekable(),
            display_map,
        }
    }
}

pub trait IteratorExtension {
    fn by_contiguous_rows<I>(self, display_map: &DisplaySnapshot) -> ContiguousRowRanges<Self>
    where
        Self: Sized + Iterator<Item = Selection<Point>>,
    {
        ContiguousRowRanges::new(self, display_map)
    }
}

impl<I> IteratorExtension for I where I: Iterator {}

impl<'snapshot, I: Iterator<Item = Selection<Point>>> Iterator
    for ContiguousRowRanges<'snapshot, I>
{
    type Item = (Range<u32>, Vec<Selection<Point>>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.selections.next();
        let selections = Vec::new();

        if let Some(selection) = next {
            selections.push(selection.clone());
            let start_row = selection.start.row;

            let end_row = end_row_for(&selection, self.display_map);

            while let Some(next_selection) = self.selections.peek() {
                if next_selection.start.row <= end_row {
                    end_row = end_row_for(next_selection, self.display_map);
                    selections.push(self.selections.next().unwrap().clone());
                } else {
                    break;
                }
            }
            Some((start_row..end_row, selections))
        } else {
            None
        }
    }
}
