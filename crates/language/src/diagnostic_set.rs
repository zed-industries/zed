use crate::Diagnostic;
use collections::HashMap;
use std::{
    cmp::{Ordering, Reverse},
    iter,
    ops::Range,
};
use sum_tree::{self, Bias, SumTree};
use text::{Anchor, FromAnchor, PointUtf16, ToOffset};

#[derive(Clone, Debug)]
pub struct DiagnosticSet {
    diagnostics: SumTree<DiagnosticEntry<Anchor>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticEntry<T> {
    pub range: Range<T>,
    pub diagnostic: Diagnostic,
}

#[derive(Debug)]
pub struct DiagnosticGroup<T> {
    pub entries: Vec<DiagnosticEntry<T>>,
    pub primary_ix: usize,
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
    pub fn from_sorted_entries<I>(iter: I, buffer: &text::BufferSnapshot) -> Self
    where
        I: IntoIterator<Item = DiagnosticEntry<Anchor>>,
    {
        Self {
            diagnostics: SumTree::from_iter(iter, buffer),
        }
    }

    pub fn new<I>(iter: I, buffer: &text::BufferSnapshot) -> Self
    where
        I: IntoIterator<Item = DiagnosticEntry<PointUtf16>>,
    {
        let mut entries = iter.into_iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|entry| (entry.range.start, Reverse(entry.range.end)));
        Self {
            diagnostics: SumTree::from_iter(
                entries.into_iter().map(|entry| DiagnosticEntry {
                    range: buffer.anchor_before(entry.range.start)
                        ..buffer.anchor_after(entry.range.end),
                    diagnostic: entry.diagnostic,
                }),
                buffer,
            ),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &DiagnosticEntry<Anchor>> {
        self.diagnostics.iter()
    }

    pub fn range<'a, T, O>(
        &'a self,
        range: Range<T>,
        buffer: &'a text::BufferSnapshot,
        inclusive: bool,
        reversed: bool,
    ) -> impl 'a + Iterator<Item = DiagnosticEntry<O>>
    where
        T: 'a + ToOffset,
        O: FromAnchor,
    {
        let end_bias = if inclusive { Bias::Right } else { Bias::Left };
        let range = buffer.anchor_before(range.start)..buffer.anchor_at(range.end, end_bias);
        let mut cursor = self.diagnostics.filter::<_, ()>({
            move |summary: &Summary| {
                let start_cmp = range.start.cmp(&summary.max_end, buffer).unwrap();
                let end_cmp = range.end.cmp(&summary.min_start, buffer).unwrap();
                if inclusive {
                    start_cmp <= Ordering::Equal && end_cmp >= Ordering::Equal
                } else {
                    start_cmp == Ordering::Less && end_cmp == Ordering::Greater
                }
            }
        });

        if reversed {
            cursor.prev(buffer);
        } else {
            cursor.next(buffer);
        }
        iter::from_fn({
            move || {
                if let Some(diagnostic) = cursor.item() {
                    if reversed {
                        cursor.prev(buffer);
                    } else {
                        cursor.next(buffer);
                    }
                    Some(diagnostic.resolve(buffer))
                } else {
                    None
                }
            }
        })
    }

    pub fn groups(&self, output: &mut Vec<DiagnosticGroup<Anchor>>, buffer: &text::BufferSnapshot) {
        let mut groups = HashMap::default();
        for entry in self.diagnostics.iter() {
            groups
                .entry(entry.diagnostic.group_id)
                .or_insert(Vec::new())
                .push(entry.clone());
        }

        let start_ix = output.len();
        output.extend(groups.into_values().filter_map(|mut entries| {
            entries.sort_unstable_by(|a, b| a.range.start.cmp(&b.range.start, buffer).unwrap());
            entries
                .iter()
                .position(|entry| entry.diagnostic.is_primary)
                .map(|primary_ix| DiagnosticGroup {
                    entries,
                    primary_ix,
                })
        }));
        output[start_ix..].sort_unstable_by(|a, b| {
            a.entries[a.primary_ix]
                .range
                .start
                .cmp(&b.entries[b.primary_ix].range.start, buffer)
                .unwrap()
        });
    }

    pub fn group<'a, O: FromAnchor>(
        &'a self,
        group_id: usize,
        buffer: &'a text::BufferSnapshot,
    ) -> impl 'a + Iterator<Item = DiagnosticEntry<O>> {
        self.iter()
            .filter(move |entry| entry.diagnostic.group_id == group_id)
            .map(|entry| entry.resolve(buffer))
    }
}

impl Default for DiagnosticSet {
    fn default() -> Self {
        Self {
            diagnostics: Default::default(),
        }
    }
}

impl sum_tree::Item for DiagnosticEntry<Anchor> {
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

impl DiagnosticEntry<Anchor> {
    pub fn resolve<O: FromAnchor>(&self, buffer: &text::BufferSnapshot) -> DiagnosticEntry<O> {
        DiagnosticEntry {
            range: O::from_anchor(&self.range.start, buffer)
                ..O::from_anchor(&self.range.end, buffer),
            diagnostic: self.diagnostic.clone(),
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
    type Context = text::BufferSnapshot;

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
