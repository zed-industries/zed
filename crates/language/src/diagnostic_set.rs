use crate::Diagnostic;
use std::{
    cmp::{Ordering, Reverse},
    iter,
    ops::Range,
};
use sum_tree::{self, Bias, SumTree};
use text::{Anchor, PointUtf16, ToOffset};

#[derive(Clone, Default)]
pub struct DiagnosticSet {
    diagnostics: SumTree<DiagnosticEntry>,
}

#[derive(Clone, Debug)]
pub struct DiagnosticEntry {
    pub range: Range<Anchor>,
    pub diagnostic: Diagnostic,
}

#[derive(Clone, Debug)]
pub struct Summary {
    start: Anchor,
    end: Anchor,
    min_start: Anchor,
    max_end: Anchor,
    count: usize,
}

impl DiagnosticSet {
    pub fn from_sorted_entries<I>(iter: I, buffer: &text::Snapshot) -> Self
    where
        I: IntoIterator<Item = DiagnosticEntry>,
    {
        Self {
            diagnostics: SumTree::from_iter(iter, buffer),
        }
    }

    pub fn reset<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (Range<PointUtf16>, Diagnostic)>,
    {
        let mut entries = iter.into_iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|(range, _)| (range.start, Reverse(range.end)));
    }

    pub fn iter(&self) -> impl Iterator<Item = &DiagnosticEntry> {
        self.diagnostics.iter()
    }

    pub fn range<'a, T>(
        &'a self,
        range: Range<T>,
        buffer: &'a text::Snapshot,
        inclusive: bool,
    ) -> impl Iterator<Item = &'a DiagnosticEntry>
    where
        T: 'a + ToOffset,
    {
        let end_bias = if inclusive { Bias::Right } else { Bias::Left };
        let range = buffer.anchor_before(range.start)..buffer.anchor_at(range.end, end_bias);
        let mut cursor = self.diagnostics.filter::<_, ()>(
            {
                move |summary: &Summary| {
                    let start_cmp = range.start.cmp(&summary.max_end, buffer).unwrap();
                    let end_cmp = range.end.cmp(&summary.min_start, buffer).unwrap();
                    if inclusive {
                        start_cmp <= Ordering::Equal && end_cmp >= Ordering::Equal
                    } else {
                        start_cmp == Ordering::Less && end_cmp == Ordering::Greater
                    }
                }
            },
            buffer,
        );

        iter::from_fn({
            move || {
                if let Some(diagnostic) = cursor.item() {
                    cursor.next(buffer);
                    Some(diagnostic)
                } else {
                    None
                }
            }
        })
    }

    pub fn group(&self, group_id: usize) -> impl Iterator<Item = &DiagnosticEntry> {
        self.iter()
            .filter(move |entry| entry.diagnostic.group_id == group_id)
    }
}

impl sum_tree::Item for DiagnosticEntry {
    type Summary = Summary;

    fn summary(&self) -> Self::Summary {
        Summary {
            start: self.range.start.clone(),
            end: self.range.end.clone(),
            min_start: self.range.start.clone(),
            max_end: self.range.end.clone(),
            count: 1,
        }
    }
}

impl Default for Summary {
    fn default() -> Self {
        Self {
            start: Anchor::min(),
            end: Anchor::max(),
            min_start: Anchor::max(),
            max_end: Anchor::min(),
            count: 0,
        }
    }
}

impl sum_tree::Summary for Summary {
    type Context = text::Snapshot;

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        if other
            .min_start
            .cmp(&self.min_start, buffer)
            .unwrap()
            .is_lt()
        {
            self.min_start = other.min_start.clone();
        }
        if other.max_end.cmp(&self.max_end, buffer).unwrap().is_gt() {
            self.max_end = other.max_end.clone();
        }
        self.start = other.start.clone();
        self.end = other.end.clone();
        self.count += other.count;
    }
}
