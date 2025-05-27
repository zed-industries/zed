use crate::{Diagnostic, range_to_lsp};
use anyhow::Result;
use collections::HashMap;
use lsp::LanguageServerId;
use serde::Serialize;
use std::{
    cmp::{Ordering, Reverse},
    iter,
    ops::Range,
};
use sum_tree::{self, Bias, SumTree};
use text::{Anchor, FromAnchor, PointUtf16, ToOffset};

/// A set of diagnostics associated with a given buffer, provided
/// by a single language server.
///
/// The diagnostics are stored in a [`SumTree`], which allows this struct
/// to be cheaply copied, and allows for efficient retrieval of the
/// diagnostics that intersect a given range of the buffer.
#[derive(Clone, Debug)]
pub struct DiagnosticSet {
    diagnostics: SumTree<DiagnosticEntry<Anchor>>,
}

/// A single diagnostic in a set. Generic over its range type, because
/// the diagnostics are stored internally as [`Anchor`]s, but can be
/// resolved to different coordinates types like [`usize`] byte offsets or
/// [`Point`](gpui::Point)s.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DiagnosticEntry<T> {
    /// The range of the buffer where the diagnostic applies.
    pub range: Range<T>,
    /// The information about the diagnostic.
    pub diagnostic: Diagnostic,
}

/// A group of related diagnostics, ordered by their start position
/// in the buffer.
#[derive(Debug, Serialize)]
pub struct DiagnosticGroup<T> {
    /// The diagnostics.
    pub entries: Vec<DiagnosticEntry<T>>,
    /// The index into `entries` where the primary diagnostic is stored.
    pub primary_ix: usize,
}

impl DiagnosticGroup<Anchor> {
    /// Converts the entries in this [`DiagnosticGroup`] to a different buffer coordinate type.
    pub fn resolve<O: FromAnchor>(&self, buffer: &text::BufferSnapshot) -> DiagnosticGroup<O> {
        DiagnosticGroup {
            entries: self
                .entries
                .iter()
                .map(|entry| entry.resolve(buffer))
                .collect(),
            primary_ix: self.primary_ix,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Summary {
    start: Anchor,
    end: Anchor,
    min_start: Anchor,
    max_end: Anchor,
    count: usize,
}

impl DiagnosticEntry<PointUtf16> {
    /// Returns a raw LSP diagnostic used to provide diagnostic context to LSP
    /// codeAction request
    pub fn to_lsp_diagnostic_stub(&self) -> Result<lsp::Diagnostic> {
        let range = range_to_lsp(self.range.clone())?;

        Ok(lsp::Diagnostic {
            range,
            code: self.diagnostic.code.clone(),
            severity: Some(self.diagnostic.severity),
            source: self.diagnostic.source.clone(),
            message: self.diagnostic.message.clone(),
            data: self.diagnostic.data.clone(),
            ..Default::default()
        })
    }
}

impl DiagnosticSet {
    /// Constructs a [DiagnosticSet] from a sequence of entries, ordered by
    /// their position in the buffer.
    pub fn from_sorted_entries<I>(iter: I, buffer: &text::BufferSnapshot) -> Self
    where
        I: IntoIterator<Item = DiagnosticEntry<Anchor>>,
    {
        Self {
            diagnostics: SumTree::from_iter(iter, buffer),
        }
    }

    /// Constructs a [DiagnosticSet] from a sequence of entries in an arbitrary order.
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
                        ..buffer.anchor_before(entry.range.end),
                    diagnostic: entry.diagnostic,
                }),
                buffer,
            ),
        }
    }

    /// Returns the number of diagnostics in the set.
    pub fn len(&self) -> usize {
        self.diagnostics.summary().count
    }
    /// Returns true when there are no diagnostics in this diagnostic set
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the diagnostic entries in the set.
    pub fn iter(&self) -> impl Iterator<Item = &DiagnosticEntry<Anchor>> {
        self.diagnostics.iter()
    }

    /// Returns an iterator over the diagnostic entries that intersect the
    /// given range of the buffer.
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
        let mut cursor = self.diagnostics.filter::<_, ()>(buffer, {
            move |summary: &Summary| {
                let start_cmp = range.start.cmp(&summary.max_end, buffer);
                let end_cmp = range.end.cmp(&summary.min_start, buffer);
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

    /// Adds all of this set's diagnostic groups to the given output vector.
    pub fn groups(
        &self,
        language_server_id: LanguageServerId,
        output: &mut Vec<(LanguageServerId, DiagnosticGroup<Anchor>)>,
        buffer: &text::BufferSnapshot,
    ) {
        let mut groups = HashMap::default();
        for entry in self.diagnostics.iter() {
            groups
                .entry(entry.diagnostic.group_id)
                .or_insert(Vec::new())
                .push(entry.clone());
        }

        let start_ix = output.len();
        output.extend(groups.into_values().filter_map(|mut entries| {
            entries.sort_unstable_by(|a, b| a.range.start.cmp(&b.range.start, buffer));
            entries
                .iter()
                .position(|entry| entry.diagnostic.is_primary)
                .map(|primary_ix| {
                    (
                        language_server_id,
                        DiagnosticGroup {
                            entries,
                            primary_ix,
                        },
                    )
                })
        }));
        output[start_ix..].sort_unstable_by(|(id_a, group_a), (id_b, group_b)| {
            group_a.entries[group_a.primary_ix]
                .range
                .start
                .cmp(&group_b.entries[group_b.primary_ix].range.start, buffer)
                .then_with(|| id_a.cmp(id_b))
        });
    }

    /// Returns all of the diagnostics in a particular diagnostic group,
    /// in order of their position in the buffer.
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

impl sum_tree::Item for DiagnosticEntry<Anchor> {
    type Summary = Summary;

    fn summary(&self, _cx: &text::BufferSnapshot) -> Self::Summary {
        Summary {
            start: self.range.start,
            end: self.range.end,
            min_start: self.range.start,
            max_end: self.range.end,
            count: 1,
        }
    }
}

impl DiagnosticEntry<Anchor> {
    /// Converts the [DiagnosticEntry] to a different buffer coordinate type.
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
            start: Anchor::MIN,
            end: Anchor::MAX,
            min_start: Anchor::MAX,
            max_end: Anchor::MIN,
            count: 0,
        }
    }
}

impl sum_tree::Summary for Summary {
    type Context = text::BufferSnapshot;

    fn zero(_cx: &Self::Context) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, buffer: &Self::Context) {
        if other.min_start.cmp(&self.min_start, buffer).is_lt() {
            self.min_start = other.min_start;
        }
        if other.max_end.cmp(&self.max_end, buffer).is_gt() {
            self.max_end = other.max_end;
        }
        self.start = other.start;
        self.end = other.end;
        self.count += other.count;
    }
}
