use std::ops::Range;

use collections::HashMap;
use language::{Bias, LanguageAwareStyling, Point};
use multi_buffer::{MBTextSummary, MultiBufferRow};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

use crate::display_map::{DisplaySnapshot, FoldPoint, Highlights, TabPoint, ToDisplayPoint as _};

pub(crate) struct ColumnarSelectionRows<'a> {
    snapshot: &'a DisplaySnapshot,
    rows: HashMap<u32, ColumnBoundaries<'a>>,
    buffer_rows: HashMap<u32, ColumnBoundaries<'a>>,
}

impl<'a> ColumnarSelectionRows<'a> {
    pub(crate) fn new(snapshot: &'a DisplaySnapshot) -> Self {
        Self {
            snapshot,
            rows: HashMap::default(),
            buffer_rows: HashMap::default(),
        }
    }

    pub(crate) fn columns_for_range(&mut self, range: Range<Point>) -> Range<u32> {
        let start = self.column_for_point(range.start);
        let end = self.column_for_point(range.end);
        start.min(end)..start.max(end)
    }

    pub(crate) fn points_for_row(
        &mut self,
        row: u32,
        columns: &Range<u32>,
    ) -> Option<(Point, Point)> {
        let tabs = self.snapshot.tab_snapshot();
        if row > tabs.max_point().row() {
            return None;
        }
        let is_empty = columns.start == columns.end;
        if !is_empty && columns.start >= tabs.line_len(row) {
            return None;
        }
        if !is_empty && self.row(row).byte_for_column(columns.start) == tabs.line_len(row) {
            return None;
        }
        let start = self.point_for_column(row, columns.start)?;
        let end = if is_empty {
            start
        } else {
            self.point_for_column(row, columns.end)?
        };
        Some((start.min(end), start.max(end)))
    }

    fn column_for_point(&mut self, point: Point) -> u32 {
        let point = self
            .snapshot
            .tab_snapshot()
            .point_to_tab_point(point, Bias::Left);
        self.row(point.row()).column_for_byte(point.column())
    }

    fn point_for_column(&mut self, row: u32, column: u32) -> Option<Point> {
        let mut byte = self.row(row).byte_for_column(column);
        loop {
            let tab_point = TabPoint::new(row, byte);
            let tabs = self.snapshot.tab_snapshot();
            let fold_point = tabs.tab_point_to_fold_point(tab_point, Bias::Left).0;
            if let Some(range) = self
                .snapshot
                .fold_snapshot()
                .placeholder_range_at(fold_point)
                && range.start < fold_point
            {
                let start = tabs.fold_point_to_tab_point(range.start);
                let column = self.row(row).column_for_byte(start.column());
                byte = self.row(row).byte_for_column(column);
                continue;
            }
            let point = tabs.tab_point_to_point(tab_point, Bias::Left);
            let canonical = tabs.point_to_tab_point(point, Bias::Left);
            if self.buffer_row(point.row).is_boundary(point.column)
                && canonical == tab_point
                && point
                    .to_display_point(self.snapshot)
                    .to_point(self.snapshot)
                    == point
            {
                return Some(point);
            }
            let preceding = if canonical < tab_point {
                canonical
            } else {
                let previous = if let Some(byte) = point.column.checked_sub(1) {
                    let boundaries = self.buffer_row(point.row);
                    let column = boundaries.column_for_byte(byte);
                    Point::new(point.row, boundaries.byte_for_column(column))
                } else {
                    let row = point.row.checked_sub(1)?;
                    Point::new(
                        row,
                        self.snapshot
                            .buffer_snapshot()
                            .line_len(MultiBufferRow(row)),
                    )
                };
                tabs.point_to_tab_point(previous, Bias::Left)
            };
            if preceding.row() != row || preceding >= tab_point {
                return None;
            }
            let column = self.row(row).column_for_byte(preceding.column());
            byte = self.row(row).byte_for_column(column);
        }
    }

    fn buffer_row(&mut self, row: u32) -> &mut ColumnBoundaries<'a> {
        let buffer = self.snapshot.buffer_snapshot();
        self.buffer_rows.entry(row).or_insert_with(|| {
            let len = buffer.line_len(MultiBufferRow(row));
            let range = Point::new(row, 0)..Point::new(row, len);
            let summary = buffer.text_summary_for_range::<MBTextSummary, _>(range.clone());
            if summary.len.0 == summary.chars {
                ColumnBoundaries::Ascii(len)
            } else {
                ColumnBoundaries::new(len, buffer.text_for_range(range))
            }
        })
    }

    fn row(&mut self, row: u32) -> &mut ColumnBoundaries<'a> {
        self.rows.entry(row).or_insert_with(|| {
            let tabs = self.snapshot.tab_snapshot();
            let folds = self.snapshot.fold_snapshot();
            let summary = folds.text_summary_for_range(
                FoldPoint::new(row, 0)..FoldPoint::new(row, folds.line_len(row)),
            );
            let len = tabs.line_len(row);
            if summary.len.0 == summary.chars {
                ColumnBoundaries::Ascii(len)
            } else {
                let chunks = tabs
                    .chunks(
                        TabPoint::new(row, 0)..TabPoint::new(row, len),
                        LanguageAwareStyling {
                            tree_sitter: false,
                            diagnostics: false,
                        },
                        Highlights::default(),
                    )
                    .map(|chunk| chunk.text);
                ColumnBoundaries::new(len, chunks)
            }
        })
    }
}

enum ColumnBoundaries<'a> {
    Ascii(u32),
    Unicode {
        chunks: Box<dyn Iterator<Item = &'a str> + 'a>,
        pending: &'a str,
        text: String,
        cursor: GraphemeCursor,
        boundaries: Vec<u32>,
    },
}

impl<'a> ColumnBoundaries<'a> {
    fn new(len: u32, chunks: impl Iterator<Item = &'a str> + 'a) -> Self {
        Self::Unicode {
            chunks: Box::new(chunks),
            pending: "",
            text: String::new(),
            cursor: GraphemeCursor::new(0, len as usize, true),
            boundaries: vec![0],
        }
    }

    #[cfg(test)]
    fn len(&mut self) -> u32 {
        self.column_for_byte(u32::MAX)
    }

    fn byte_for_column(&mut self, column: u32) -> u32 {
        self.extend_until(|boundaries| boundaries.len() > column as usize);
        match self {
            Self::Ascii(len) => column.min(*len),
            Self::Unicode { boundaries, .. } => boundaries
                .get(column as usize)
                .copied()
                .unwrap_or_else(|| boundaries.last().copied().unwrap_or(0)),
        }
    }

    fn column_for_byte(&mut self, byte: u32) -> u32 {
        self.extend_until(|boundaries| boundaries.last().is_some_and(|last| *last >= byte));
        match self {
            Self::Ascii(len) => byte.min(*len),
            Self::Unicode { boundaries, .. } => boundaries
                .partition_point(|boundary| *boundary <= byte)
                .saturating_sub(1) as u32,
        }
    }

    fn is_boundary(&mut self, byte: u32) -> bool {
        let column = self.column_for_byte(byte);
        self.byte_for_column(column) == byte
    }

    fn extend_until(&mut self, complete: impl Fn(&[u32]) -> bool) {
        if let Self::Unicode {
            chunks,
            pending,
            text,
            cursor,
            boundaries,
        } = self
        {
            while !complete(boundaries) {
                match cursor.next_boundary(text, 0) {
                    Ok(Some(boundary)) => boundaries.push(boundary as u32),
                    Ok(None) => break,
                    Err(GraphemeIncomplete::NextChunk) => {
                        if pending.is_empty() {
                            let Some(chunk) = chunks.next() else {
                                break;
                            };
                            *pending = chunk;
                        }
                        let end = pending
                            .char_indices()
                            .nth(128)
                            .map_or(pending.len(), |(index, _)| index);
                        text.push_str(&pending[..end]);
                        *pending = &pending[end..];
                    }
                    Err(error) => unreachable!("contiguous grapheme prefix: {error:?}"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::{
            BlockPlacement, BlockProperties, BlockStyle, Crease, DisplayMap, FoldPlaceholder,
        },
        inlays::Inlay,
        test::test_font,
    };
    use collections::HashSet;
    use gpui::{
        AppContext as _, BorrowAppContext as _, Element as _, Entity, Pixels, SharedString,
        TestAppContext, div, px,
    };
    use language::{Buffer, Capability};
    use multi_buffer::{Anchor, ExcerptRange, MultiBuffer, MultiBufferOffset, PathKey};
    use project::project_settings::DiagnosticSeverity;
    use rand::prelude::*;
    use settings::SettingsStore;
    use std::{env, iter, num::NonZeroU32, sync::Arc};
    use theme::LoadThemes;
    use unicode_segmentation::UnicodeSegmentation as _;

    #[test]
    fn test_column_boundaries_across_chunks() {
        for text in [
            "",
            "abc",
            "e\u{301}x",
            "\u{301}\u{301}x",
            "👩\u{200d}💻x",
            "🇦🇶🇦🇶🇦x",
            "क्\u{200d}षx",
        ] {
            let expected = grapheme_boundaries(text);
            for split in scalar_boundaries(text) {
                let mut query = ColumnBoundaries::new(
                    text.len() as u32,
                    ["", &text[..split], "", &text[split..], ""].into_iter(),
                );
                for byte in 0..=text.len() + 1 {
                    let column = expected.partition_point(|boundary| *boundary <= byte) - 1;
                    assert_eq!(
                        query.column_for_byte(byte as u32),
                        column as u32,
                        "{text:?}, split {split}, byte {byte}"
                    );
                    assert_eq!(
                        query.is_boundary(byte as u32),
                        expected.binary_search(&byte).is_ok()
                    );
                }
                for (column, byte) in expected.iter().enumerate().rev() {
                    assert_eq!(query.byte_for_column(column as u32), *byte as u32);
                }
                assert_eq!(query.byte_for_column(u32::MAX), text.len() as u32);
            }
        }
    }

    #[gpui::test]
    fn test_columnar_selection_lazy_unicode_prefix(cx: &mut TestAppContext) {
        init_test(cx);
        let line = format!("{}é", "a".repeat(1_048_576));
        let snapshot = plain_state(&format!("{line}\n{line}"), cx)
            .build_map(cx)
            .update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.columns_for_range(Point::zero()..Point::zero()), 0..0);
        assert_eq!(
            query.points_for_row(1, &(0..0)),
            Some((Point::new(1, 0), Point::new(1, 0)))
        );
        for row in query.rows.values().chain(query.buffer_rows.values()) {
            let ColumnBoundaries::Unicode {
                text, boundaries, ..
            } = row
            else {
                panic!("Unicode row")
            };
            assert_eq!(text.len(), 0);
            assert_eq!(boundaries, &[0]);
        }
        assert_eq!(
            query.points_for_row(1, &(1..1)),
            Some((Point::new(1, 1), Point::new(1, 1)))
        );
        for row in query.rows.values().chain(query.buffer_rows.values()) {
            let ColumnBoundaries::Unicode {
                text, boundaries, ..
            } = row
            else {
                panic!("Unicode row")
            };
            assert!(text.len() <= 512);
            assert!(boundaries.len() <= 2);
        }
        assert_eq!(
            query.points_for_row(1, &(1_048_576..1_048_576)),
            Some((Point::new(1, 1_048_576), Point::new(1, 1_048_576)))
        );
    }

    #[gpui::test]
    fn test_columnar_selection_skips_long_virtual_row(cx: &mut TestAppContext) {
        init_test(cx);
        let mut state = plain_state(&format!("x\n{}", "a".repeat(1_048_576)), cx);
        let buffer = state.buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx));
        state.inlays.push(Inlay::mock_hint(
            0,
            buffer.anchor_before(Point::new(1, 0)),
            &format!("{}\n", "a".repeat(1_048_576)),
        ));
        let snapshot = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.points_for_row(1, &(1_048_576..1_048_576)), None);
        assert_eq!(
            query.points_for_row(2, &(1_048_576..1_048_576)),
            Some((Point::new(1, 1_048_576), Point::new(1, 1_048_576)))
        );
    }

    #[gpui::test]
    fn test_columnar_selection_graphemes_and_edges(cx: &mut TestAppContext) {
        init_test(cx);
        let state = plain_state("e\u{301}\néé\n\n\t\u{301}x\n👩\u{200d}💻z\n", cx);
        let map = state.build_map(cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(
            query.columns_for_range(Point::new(0, 0)..Point::new(0, 3)),
            0..1
        );
        assert_eq!(
            query.columns_for_range(Point::new(1, 0)..Point::new(1, 4)),
            0..2
        );
        assert_eq!(
            query.columns_for_range(Point::new(1, 4)..Point::new(0, 0)),
            0..2
        );
        assert_eq!(
            query.points_for_row(0, &(0..1)),
            Some((Point::new(0, 0), Point::new(0, 3)))
        );
        assert_eq!(
            query.points_for_row(1, &(0..1)),
            Some((Point::new(1, 0), Point::new(1, 2)))
        );
        assert_eq!(query.points_for_row(0, &(1..2)), None);
        assert_eq!(
            query.points_for_row(0, &(u32::MAX..u32::MAX)),
            Some((Point::new(0, 3), Point::new(0, 3)))
        );
        assert_eq!(
            query.points_for_row(2, &(0..0)),
            Some((Point::new(2, 0), Point::new(2, 0)))
        );
        assert_eq!(query.points_for_row(2, &(0..1)), None);
        assert_eq!(
            query.points_for_row(3, &(3..4)),
            Some((Point::new(3, 0), Point::new(3, 3)))
        );
        assert_eq!(
            query.points_for_row(4, &(0..1)),
            Some((Point::new(4, 0), Point::new(4, 11)))
        );
        assert_eq!(
            query.points_for_row(5, &(u32::MAX..u32::MAX)),
            Some((Point::new(5, 0), Point::new(5, 0)))
        );
        assert_eq!(query.points_for_row(6, &(0..0)), None);
        assert_eq!(query.points_for_row(u32::MAX, &(0..0)), None);
        assert_plain_rows(&snapshot, 4);
        let empty = plain_state("", cx)
            .build_map(cx)
            .update(cx, |map, cx| map.snapshot(cx));
        assert_plain_rows(&empty, 4);
    }

    #[gpui::test]
    fn test_columnar_selection_ascii_fast_path(cx: &mut TestAppContext) {
        init_test(cx);
        let state = plain_state(&format!("{}\nx", "a".repeat(1_048_576)), cx);
        let map = state.build_map(cx);
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.points_for_row(1, &(2..3)), None);
        assert_eq!(query.rows.capacity(), 0);
        assert_eq!(query.buffer_rows.capacity(), 0);
        assert_eq!(
            query.columns_for_range(Point::zero()..Point::new(0, 1_048_576)),
            0..1_048_576
        );
        let ColumnBoundaries::Ascii(len) = query.row(0) else {
            panic!("ASCII row allocated Unicode boundaries")
        };
        assert_eq!(*len, 1_048_576);
        assert_eq!(
            query.points_for_row(0, &(1_048_575..u32::MAX)),
            Some((Point::new(0, 1_048_575), Point::new(0, 1_048_576)))
        );
        let Some(ColumnBoundaries::Ascii(len)) = query.buffer_rows.get(&0) else {
            panic!("ASCII buffer row allocated Unicode boundaries")
        };
        assert_eq!(*len, 1_048_576);
    }

    #[gpui::test]
    fn test_columnar_selection_tab_expansion_boundary(cx: &mut TestAppContext) {
        init_test(cx);
        let prefixes = (255..=257)
            .map(|column| format!("{}{}", "α".repeat(127), "a".repeat(column - 254)))
            .chain([
                format!("{}α", "a".repeat(255)),
                format!("{}e\u{301}", "a".repeat(254)),
            ])
            .collect::<Vec<_>>();
        let text = prefixes
            .iter()
            .map(|prefix| format!("{prefix}\tβ"))
            .collect::<Vec<_>>()
            .join("\n");
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let state = MapState::new(cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx)));
        let map = state.build_map(cx);
        for tab_size in [1, 3, 128] {
            set_tab_size(tab_size, cx);
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_plain_rows(&snapshot, tab_size);
            if tab_size == 128 {
                assert_eq!(
                    snapshot.tab_snapshot().text(),
                    format!(
                        "{}{}β\n{} β\n{} β\n{} β\n{} β",
                        prefixes[0],
                        " ".repeat(128),
                        prefixes[1],
                        prefixes[2],
                        prefixes[3],
                        prefixes[4]
                    )
                );
            }
            buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "a")], None, cx));
            assert_plain_rows(&map.update(cx, |map, cx| map.snapshot(cx)), tab_size);
            buffer.update(cx, |buffer, cx| buffer.edit([(0..1, "")], None, cx));
            let restored = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                restored.tab_snapshot().text(),
                snapshot.tab_snapshot().text()
            );
            assert_plain_rows(&restored, tab_size);
        }
    }

    #[gpui::test]
    fn test_columnar_selection_folds_and_inlays_without_frames(
        cx: &mut TestAppContext,
        mut rng: StdRng,
    ) {
        init_test(cx);
        let mut state = plain_state("e\u{301}x\nab\ncd", cx);
        let map = state.build_map(cx);
        map.update(cx, |map, cx| {
            map.fold(
                vec![Crease::simple(
                    Point::new(0, 1)..Point::new(0, 3),
                    FoldPlaceholder::test(),
                )],
                cx,
            )
        });
        let folded = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(folded.tab_snapshot().text(), "e⋯x\nab\ncd");
        assert_eq!(
            ColumnarSelectionRows::new(&folded).points_for_row(0, &(1..2)),
            Some((Point::new(0, 0), Point::new(0, 3)))
        );
        state.folds = folded
            .folds_in_range(Anchor::Min..Anchor::Max)
            .map(|fold| fold.range.0.clone())
            .collect();
        for (index, bias) in [Bias::Left, Bias::Right].into_iter().enumerate() {
            state.inlays.push(Inlay::mock_hint(
                index,
                folded.buffer_snapshot().anchor_at(Point::new(1, 1), bias),
                "\u{301}\n\tR",
            ));
        }
        map.update(cx, |map, cx| {
            map.splice_inlays(&[], state.inlays.clone(), cx)
        });
        let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        assert_eq!(
            snapshot.tab_snapshot().text(),
            "e⋯x\na\u{301}\n    R\u{301}\n    Rb\ncd"
        );
        let mut query = ColumnarSelectionRows::new(&snapshot);
        assert_eq!(query.points_for_row(2, &(0..0)), None);
        assert_eq!(query.points_for_row(2, &(0..u32::MAX)), None);
        let rebuilt = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
        check_snapshot(&snapshot, &rebuilt, None, 4, &mut rng, 0);
    }

    #[gpui::test]
    fn test_columnar_selection_long_fold_placeholders(cx: &mut TestAppContext) {
        init_test(cx);
        for placeholder in ["...", "long placeholder", "ééé", "👩\u{200d}💻xyz"] {
            for edited in [false, true] {
                let buffer = cx.new(|cx| Buffer::local(if edited { "a\nxb" } else { "axb" }, cx));
                let state = MapState::new(cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx)));
                let map = state.build_map(cx);
                map.update(cx, |map, cx| {
                    map.fold(
                        vec![Crease::simple(
                            Point::new(0, 1)..if edited {
                                Point::new(1, 1)
                            } else {
                                Point::new(0, 2)
                            },
                            FoldPlaceholder {
                                collapsed_text: Some(SharedString::from(placeholder)),
                                ..FoldPlaceholder::test()
                            },
                        )],
                        cx,
                    );
                });
                if edited {
                    buffer.update(cx, |buffer, cx| {
                        buffer.edit([(Point::new(0, 1)..Point::new(1, 0), "")], None, cx)
                    });
                }
                let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
                assert_eq!(snapshot.tab_snapshot().text(), format!("a{placeholder}b"));
                let mut query = ColumnarSelectionRows::new(&snapshot);
                let end = 1 + placeholder.graphemes(true).count() as u32;
                for column in 1..end {
                    assert_eq!(
                        query.points_for_row(0, &(column..column)),
                        Some((Point::new(0, 1), Point::new(0, 1))),
                        "{placeholder:?}, column {column}, edited {edited}"
                    );
                }
                assert_eq!(
                    query.points_for_row(0, &(end..end)),
                    Some((Point::new(0, 2), Point::new(0, 2)))
                );
            }
        }
    }

    #[gpui::test]
    fn test_columnar_selection_fold_grapheme_edges(cx: &mut TestAppContext) {
        init_test(cx);
        for (text, placeholder, column, expected) in
            [("axb", "\u{301}..", 1, 0), ("ax\u{301}b", "..", 3, 4)]
        {
            let state = plain_state(text, cx);
            let map = state.build_map(cx);
            map.update(cx, |map, cx| {
                map.fold(
                    vec![Crease::simple(
                        Point::new(0, 1)..Point::new(0, 2),
                        FoldPlaceholder {
                            collapsed_text: Some(SharedString::from(placeholder)),
                            ..FoldPlaceholder::test()
                        },
                    )],
                    cx,
                )
            });
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                ColumnarSelectionRows::new(&snapshot).points_for_row(0, &(column..column)),
                Some((Point::new(0, expected), Point::new(0, expected))),
                "{text:?}, {placeholder:?}"
            );
        }
        for merge in [false, true] {
            let map = plain_state("axyb", cx).build_map(cx);
            map.update(cx, |map, cx| {
                map.fold(
                    [(1, ".."), (2, "ZZ")]
                        .into_iter()
                        .map(|(start, placeholder)| {
                            Crease::simple(
                                Point::new(0, start)..Point::new(0, start + 1),
                                FoldPlaceholder {
                                    merge_adjacent: merge,
                                    collapsed_text: Some(SharedString::from(placeholder)),
                                    ..FoldPlaceholder::test()
                                },
                            )
                        })
                        .collect(),
                    cx,
                )
            });
            let snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            assert_eq!(
                snapshot.tab_snapshot().text(),
                if merge { "a..b" } else { "a..ZZb" }
            );
            let expected = if merge {
                vec![0, 1, 1, 3, 4]
            } else {
                vec![0, 1, 1, 2, 2, 3, 4]
            };
            let mut query = ColumnarSelectionRows::new(&snapshot);
            for (column, byte) in expected.into_iter().enumerate() {
                let column = column as u32;
                assert_eq!(
                    query.points_for_row(0, &(column..column)),
                    Some((Point::new(0, byte), Point::new(0, byte))),
                    "merge={merge}, column={column}"
                );
            }
        }
    }

    #[test]
    fn test_column_boundaries_large_grapheme() {
        let text = format!("e{}z", "\u{301}".repeat(1024));
        let mut query = ColumnBoundaries::new(text.len() as u32, iter::once(text.as_str()));
        assert_eq!(query.byte_for_column(0), 0);
        assert_eq!(query.byte_for_column(1), 2049);
        assert_eq!(query.byte_for_column(2), 2050);
        assert_eq!(query.column_for_byte(2048), 0);
        assert_eq!(query.column_for_byte(2049), 1);
    }

    #[gpui::test(iterations = 100)]
    fn test_random_columnar_selections(cx: &mut TestAppContext, mut rng: StdRng) {
        init_test(cx);
        cx.background_executor.set_block_on_ticks(0..=50);
        let operations = env::var("OPERATIONS")
            .map(|value| value.parse::<usize>().expect("invalid OPERATIONS"))
            .unwrap_or(15);
        let mut texts = (0..rng.random_range(1..=2))
            .map(|_| format!("a{}z", random_text(&mut rng, 12)))
            .collect::<Vec<_>>();
        let buffers = texts
            .iter()
            .map(|text| cx.new(|cx| Buffer::local(text.clone(), cx)))
            .collect::<Vec<_>>();
        let multibuffer = cx.new(|cx| {
            if buffers.len() == 1 {
                return MultiBuffer::singleton(buffers[0].clone(), cx);
            }
            let mut buffer = MultiBuffer::new(Capability::ReadWrite);
            for (index, source) in buffers.iter().enumerate() {
                let snapshot = source.read(cx).snapshot();
                buffer.set_excerpt_ranges_for_path(
                    PathKey::sorted(index as u64),
                    source.clone(),
                    &snapshot,
                    vec![ExcerptRange::new(Point::zero()..snapshot.max_point())],
                    cx,
                );
            }
            buffer
        });
        let mut state = MapState::new(multibuffer);
        let map = state.build_map(cx);
        let mut block_ids = HashSet::default();
        let mut kinds = [0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6];
        kinds.shuffle(&mut rng);
        let mut snapshot = map.update(cx, |map, cx| map.snapshot(cx));
        check_snapshot(
            &snapshot,
            &snapshot,
            Some(&texts.join("\n")),
            4,
            &mut rng,
            0,
        );
        for operation in 0..operations {
            let kind = kinds[operation % kinds.len()];
            log::info!(
                "operation {operation}: kind {kind}, texts {texts:?}, tabs {}, folds {:?}, inlays {:?}, block {:?}, wrap {:?}",
                state.tab_size,
                state.folds,
                state.inlays,
                state.block,
                state.wrap_width
            );
            match kind {
                0 => {
                    let index = rng.random_range(0..buffers.len());
                    let text = &mut texts[index];
                    let boundaries = text
                        .char_indices()
                        .map(|(offset, _)| offset)
                        .skip(1)
                        .collect::<Vec<_>>();
                    let start = rng.random_range(0..boundaries.len());
                    let mut range =
                        boundaries[start]..boundaries[rng.random_range(start..boundaries.len())];
                    let mut replacement = random_text(&mut rng, 4);
                    if text.get(range.clone()) == Some(replacement.as_str()) {
                        replacement.push('a');
                    }
                    let mut expected = text.clone();
                    expected.replace_range(range.clone(), &replacement);
                    if expected.len() > 256
                        || expected.matches('\t').count() > 4
                        || expected.matches('\n').count() > 12
                    {
                        range = 1..text.len() - 1;
                        replacement.clear();
                        expected = String::from("az");
                    }
                    assert_ne!(expected.as_str(), text.as_str());
                    log::info!("edit buffer {index}: {range:?} -> {replacement:?}");
                    buffers[index].update(cx, |buffer, cx| {
                        buffer.edit([(range, replacement)], None, cx)
                    });
                    *text = expected;
                }
                1 => {
                    let size = rng.random_range(1..128);
                    state.tab_size = size + u32::from(size >= state.tab_size);
                    set_tab_size(state.tab_size, cx);
                }
                2 => {
                    if state.folds.is_empty() {
                        let index = rng.random_range(0..texts.len());
                        let base = texts
                            .iter()
                            .take(index)
                            .map(|text| text.len() + 1)
                            .sum::<usize>();
                        let boundaries = scalar_boundaries(&texts[index]);
                        let start = rng.random_range(0..boundaries.len() - 1);
                        let end = rng.random_range(start + 1..boundaries.len());
                        let buffer = snapshot.buffer_snapshot();
                        let range = buffer.anchor_after(MultiBufferOffset(base + boundaries[start]))
                            ..buffer.anchor_before(MultiBufferOffset(base + boundaries[end]));
                        map.update(cx, |map, cx| {
                            map.fold(vec![Crease::simple(range, FoldPlaceholder::test())], cx)
                        });
                    } else {
                        map.update(cx, |map, cx| {
                            map.unfold_intersecting([Anchor::Min..Anchor::Max], true, cx)
                        });
                    }
                }
                3 | 6 => {
                    if state.inlays.is_empty() {
                        let text = snapshot.buffer_snapshot().text();
                        let boundaries = scalar_boundaries(&text);
                        let offset =
                            MultiBufferOffset(boundaries[rng.random_range(0..boundaries.len())]);
                        for (index, bias) in [Bias::Left, Bias::Right].into_iter().enumerate() {
                            let text = format!("\n{}\t", random_text(&mut rng, 3));
                            state.inlays.push(Inlay::mock_hint(
                                index,
                                snapshot.buffer_snapshot().anchor_at(offset, bias),
                                text.as_str(),
                            ));
                        }
                        map.update(cx, |map, cx| {
                            map.splice_inlays(&[], state.inlays.clone(), cx)
                        });
                    } else {
                        let ids = state
                            .inlays
                            .iter()
                            .map(|inlay| inlay.id)
                            .collect::<Vec<_>>();
                        map.update(cx, |map, cx| map.splice_inlays(&ids, Vec::new(), cx));
                        state.inlays.clear();
                    }
                }
                4 => {
                    state.wrap_width = if state.wrap_width.is_some() {
                        None
                    } else {
                        Some(px([0.0, 12.0, 48.0, 300.0][rng.random_range(0..4)]))
                    };
                    map.update(cx, |map, cx| map.set_wrap_width(state.wrap_width, cx));
                }
                5 => {
                    if state.block.is_some() {
                        map.update(cx, |map, cx| {
                            map.remove_blocks(std::mem::take(&mut block_ids), cx)
                        });
                        state.block = None;
                    } else {
                        let text = snapshot.buffer_snapshot().text();
                        let boundaries = scalar_boundaries(&text);
                        let offset =
                            MultiBufferOffset(boundaries[rng.random_range(0..boundaries.len())]);
                        state.block = Some((
                            snapshot.buffer_snapshot().anchor_after(offset),
                            rng.random(),
                            rng.random_range(1..=3),
                        ));
                        block_ids.extend(map.update(cx, |map, cx| {
                            map.insert_blocks([state.block_properties().expect("block")], cx)
                        }));
                    }
                }
                _ => unreachable!(),
            }
            let previous = snapshot;
            snapshot = map.update(cx, |map, cx| map.snapshot(cx));
            state.folds = snapshot
                .folds_in_range(Anchor::Min..Anchor::Max)
                .map(|fold| fold.range.0.clone())
                .collect();
            let text = texts.join("\n");
            assert_eq!(
                snapshot.buffer_snapshot().text(),
                text,
                "operation {operation}"
            );
            let plain = (state.folds.is_empty()
                && state
                    .inlays
                    .iter()
                    .all(|inlay| !inlay.position.is_valid(snapshot.buffer_snapshot())))
            .then_some(text.as_str());
            let rebuilt = state.build_map(cx).update(cx, |map, cx| map.snapshot(cx));
            check_snapshot(
                &snapshot,
                &rebuilt,
                plain,
                state.tab_size,
                &mut rng,
                operation,
            );
            if kind == 4 || kind == 5 {
                check_snapshot(
                    &snapshot,
                    &previous,
                    plain,
                    state.tab_size,
                    &mut rng,
                    operation,
                );
            }
            cx.run_until_parked();
            let settled = map.update(cx, |map, cx| map.snapshot(cx));
            check_snapshot(
                &settled,
                &rebuilt,
                plain,
                state.tab_size,
                &mut rng,
                operation,
            );
            snapshot = settled;
        }
    }

    struct MapState {
        buffer: Entity<MultiBuffer>,
        tab_size: u32,
        folds: Vec<Range<Anchor>>,
        inlays: Vec<Inlay>,
        block: Option<(Anchor, bool, u32)>,
        wrap_width: Option<Pixels>,
    }

    impl MapState {
        fn new(buffer: Entity<MultiBuffer>) -> Self {
            Self {
                buffer,
                tab_size: 4,
                folds: Vec::new(),
                inlays: Vec::new(),
                block: None,
                wrap_width: None,
            }
        }

        fn build_map(&self, cx: &mut TestAppContext) -> Entity<DisplayMap> {
            cx.new(|cx| {
                let mut map = DisplayMap::new(
                    self.buffer.clone(),
                    test_font(),
                    px(10.0),
                    self.wrap_width,
                    1,
                    1,
                    FoldPlaceholder::test(),
                    DiagnosticSeverity::Warning,
                    cx,
                );
                map.splice_inlays(&[], self.inlays.clone(), cx);
                if !self.folds.is_empty() {
                    map.fold(
                        self.folds
                            .iter()
                            .cloned()
                            .map(|range| Crease::simple(range, FoldPlaceholder::test()))
                            .collect(),
                        cx,
                    );
                }
                if let Some(block) = self.block_properties() {
                    map.insert_blocks([block], cx);
                }
                map
            })
        }

        fn block_properties(&self) -> Option<BlockProperties<Anchor>> {
            self.block.map(|(position, above, height)| BlockProperties {
                placement: if above {
                    BlockPlacement::Above(position)
                } else {
                    BlockPlacement::Below(position)
                },
                style: BlockStyle::Fixed,
                height: Some(height),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
            })
        }
    }

    struct PlainRow {
        text: String,
        len: u32,
        columns: Vec<(u32, u32)>,
        points: Vec<(u32, u32)>,
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            crate::init(cx);
            theme_settings::init(LoadThemes::JustBase, cx);
        });
        set_tab_size(4, cx);
    }

    fn set_tab_size(size: u32, cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.tab_size = NonZeroU32::new(size);
                })
            })
        });
    }

    fn plain_state(text: &str, cx: &mut TestAppContext) -> MapState {
        MapState::new(cx.update(|cx| MultiBuffer::build_simple(text, cx)))
    }

    fn random_text(rng: &mut StdRng, tokens: usize) -> String {
        let mut text = String::new();
        let mut tabs = 0;
        for _ in 0..rng.random_range(0..=tokens) {
            let mut token = *[
                "a",
                " ",
                "\t",
                "\n",
                "α",
                "界",
                "🏀",
                "é",
                "e\u{301}",
                "o\u{308}\u{301}",
                "\u{301}",
                "👩\u{200d}💻",
                "👍🏽",
                "🇦🇶",
                "✈\u{fe0f}",
            ]
            .choose(rng)
            .expect("tokens");
            if token == "\t" {
                if tabs == 2 {
                    token = "a";
                } else {
                    tabs += 1;
                }
            }
            text.push_str(token);
        }
        text
    }

    fn scalar_boundaries(text: &str) -> Vec<usize> {
        text.char_indices()
            .map(|(offset, _)| offset)
            .chain(iter::once(text.len()))
            .collect()
    }

    fn grapheme_boundaries(text: &str) -> Vec<usize> {
        text.grapheme_indices(true)
            .map(|(offset, _)| offset)
            .chain(iter::once(text.len()))
            .collect()
    }

    fn plain_row(line: &str, tab_size: u32) -> PlainRow {
        let mut text = String::new();
        let mut positions = Vec::new();
        let mut character_column = 0;
        for (byte, character) in line.char_indices() {
            positions.push((byte, text.len()));
            if character == '\t' {
                let width = if byte < 256 {
                    tab_size - character_column % tab_size
                } else {
                    1
                };
                text.extend(iter::repeat_n(' ', width as usize));
                character_column += width;
            } else {
                text.push(character);
                character_column += 1;
            }
        }
        positions.push((line.len(), text.len()));
        let raw = grapheme_boundaries(line);
        let expanded = grapheme_boundaries(&text);
        let columns = positions
            .iter()
            .map(|&(byte, offset)| {
                (
                    byte as u32,
                    expanded.partition_point(|boundary| *boundary <= offset) as u32 - 1,
                )
            })
            .collect();
        let points = positions
            .iter()
            .filter(|(byte, offset)| {
                raw.binary_search(byte).is_ok() && expanded.binary_search(offset).is_ok()
            })
            .map(|&(byte, offset)| {
                (
                    expanded.binary_search(&offset).expect("expanded boundary") as u32,
                    byte as u32,
                )
            })
            .collect();
        PlainRow {
            text,
            len: expanded.len() as u32 - 1,
            columns,
            points,
        }
    }

    fn expected_points(
        row: u32,
        columns: &Range<u32>,
        expected: &PlainRow,
    ) -> Option<(Point, Point)> {
        if columns.start != columns.end && columns.start >= expected.len {
            return None;
        }
        let point = |column| {
            Point::new(
                row,
                expected
                    .points
                    .iter()
                    .rev()
                    .find(|(candidate, _)| *candidate <= column)
                    .expect("row start")
                    .1,
            )
        };
        Some((point(columns.start), point(columns.end)))
    }

    fn assert_plain_rows(snapshot: &DisplaySnapshot, tab_size: u32) {
        let raw = snapshot.buffer_snapshot().text();
        let expected = raw
            .split('\n')
            .map(|line| plain_row(line, tab_size))
            .collect::<Vec<_>>();
        assert_eq!(
            snapshot.tab_snapshot().text(),
            expected
                .iter()
                .map(|row| row.text.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        );
        let mut query = ColumnarSelectionRows::new(snapshot);
        for (row, expected) in expected.iter().enumerate() {
            let row = row as u32;
            assert_eq!(query.row(row).len(), expected.len);
            for &(byte, column) in &expected.columns {
                assert_eq!(
                    query.columns_for_range(Point::new(row, byte)..Point::new(row, byte)),
                    column..column
                );
            }
            for column in 0..=expected.len + 1 {
                for columns in [column..column, column..column + 1, column..u32::MAX] {
                    assert_eq!(
                        query.points_for_row(row, &columns),
                        expected_points(row, &columns, expected),
                        "row {row}, columns {columns:?}, text {:?}",
                        expected.text
                    );
                }
            }
        }
    }

    fn check_snapshot(
        snapshot: &DisplaySnapshot,
        rebuilt: &DisplaySnapshot,
        plain: Option<&str>,
        tab_size: u32,
        rng: &mut StdRng,
        operation: usize,
    ) {
        let text = snapshot.tab_snapshot().text();
        assert_eq!(
            text,
            rebuilt.tab_snapshot().text(),
            "operation {operation}: tab text"
        );
        assert_eq!(
            snapshot.fold_snapshot().text(),
            rebuilt.fold_snapshot().text(),
            "operation {operation}: fold text"
        );
        assert_eq!(
            snapshot.inlay_snapshot().text(),
            rebuilt.inlay_snapshot().text(),
            "operation {operation}: inlay text"
        );
        let raw = snapshot.buffer_snapshot().text();
        assert_eq!(raw, rebuilt.buffer_snapshot().text());
        if let Some(plain) = plain {
            assert_eq!(raw, plain);
        }
        let expected = plain.map(|plain| {
            plain
                .split('\n')
                .map(|line| plain_row(line, tab_size))
                .collect::<Vec<_>>()
        });
        if let Some(expected) = &expected {
            assert_eq!(
                text,
                expected
                    .iter()
                    .map(|row| row.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n"),
                "operation {operation}: independent expansion"
            );
        }
        let lines = text.split('\n').collect::<Vec<_>>();
        let raw_lines = raw.split('\n').collect::<Vec<_>>();
        let raw_boundaries = grapheme_boundaries(&raw);
        let mut query = ColumnarSelectionRows::new(snapshot);
        let mut rebuilt_query = ColumnarSelectionRows::new(rebuilt);
        assert_eq!(
            snapshot.tab_snapshot().max_point().row() as usize + 1,
            lines.len()
        );
        let sources = scalar_boundaries(&raw);
        for (start, end) in [
            (0, raw.len()),
            (
                sources[rng.random_range(0..sources.len())],
                sources[rng.random_range(0..sources.len())],
            ),
        ] {
            let point = |offset| {
                Point::new(
                    raw[..offset].matches('\n').count() as u32,
                    raw[..offset].rsplit('\n').next().expect("line").len() as u32,
                )
            };
            let range = point(start)..point(end);
            let columns = query.columns_for_range(range.clone());
            assert_eq!(
                columns,
                rebuilt_query.columns_for_range(range.clone()),
                "operation {operation}: source {range:?}"
            );
            if let Some(expected) = &expected {
                let column = |point: Point| {
                    expected[point.row as usize]
                        .columns
                        .iter()
                        .find(|(byte, _)| *byte == point.column)
                        .expect("source byte")
                        .1
                };
                let (start, end) = (column(range.start), column(range.end));
                assert_eq!(
                    columns,
                    start.min(end)..start.max(end),
                    "operation {operation}: independent source {range:?}"
                );
            }
        }
        let mut rows = vec![
            0,
            lines.len() - 1,
            rng.random_range(0..lines.len()),
            rng.random_range(0..lines.len()),
        ];
        rows.sort_unstable();
        rows.dedup();
        for row in rows {
            let boundaries = grapheme_boundaries(lines[row]);
            let len = boundaries.len() as u32 - 1;
            assert_eq!(
                snapshot.tab_snapshot().line_len(row as u32),
                lines[row].len() as u32
            );
            assert_eq!(
                query.row(row as u32).len(),
                len,
                "operation {operation}: row {row}"
            );
            assert_eq!(rebuilt_query.row(row as u32).len(), len);
            let candidates = raw_lines
                .iter()
                .enumerate()
                .flat_map(|(row, line)| {
                    grapheme_boundaries(line)
                        .into_iter()
                        .map(move |byte| Point::new(row as u32, byte as u32))
                })
                .filter_map(|point| {
                    let tab = snapshot
                        .tab_snapshot()
                        .point_to_tab_point(point, Bias::Left);
                    if tab.row() as usize != row
                        || snapshot.tab_snapshot().tab_point_to_point(tab, Bias::Left) != point
                        || point.to_display_point(snapshot).to_point(snapshot) != point
                    {
                        return None;
                    }
                    boundaries
                        .binary_search(&(tab.column() as usize))
                        .ok()
                        .map(|column| (column as u32, point))
                })
                .collect::<Vec<_>>();
            let middle = rng.random_range(0..=len + 1);
            for columns in [
                0..0,
                0..u32::MAX,
                middle..middle,
                middle..middle + 1,
                middle..rng.random_range(middle..=len + 1),
                len..len,
                len..len + 1,
                u32::MAX..u32::MAX,
            ] {
                let points = query.points_for_row(row as u32, &columns);
                assert_eq!(
                    points,
                    rebuilt_query.points_for_row(row as u32, &columns),
                    "operation {operation}: row {row}, columns {columns:?}"
                );
                if let Some(expected) = &expected {
                    assert_eq!(
                        points,
                        expected_points(row as u32, &columns, &expected[row]),
                        "operation {operation}: independent row {row}, columns {columns:?}"
                    );
                }
                let expected_candidate = |column| {
                    candidates
                        .iter()
                        .filter(|(candidate, _)| *candidate <= column)
                        .max_by_key(|(candidate, _)| *candidate)
                        .map(|(_, point)| *point)
                };
                let expected = if columns.start != columns.end && columns.start >= len {
                    None
                } else {
                    expected_candidate(columns.start).zip(expected_candidate(columns.end))
                };
                assert_eq!(
                    points, expected,
                    "operation {operation}: enumerated raw boundaries, row {row}, columns {columns:?}, raw {raw:?}, tab {text:?}"
                );
                if let Some((start, end)) = points {
                    assert!(start <= end);
                    for point in [start, end] {
                        let line = raw_lines
                            .get(point.row as usize)
                            .expect("raw row in bounds");
                        assert!(point.column as usize <= line.len());
                        let offset = raw_lines
                            .iter()
                            .take(point.row as usize)
                            .map(|line| line.len() + 1)
                            .sum::<usize>()
                            + point.column as usize;
                        assert_eq!(
                            raw_boundaries
                                .get(raw_boundaries.partition_point(|boundary| *boundary < offset))
                                .copied(),
                            Some(offset),
                            "operation {operation}: full-buffer grapheme {point:?}, raw {raw:?}"
                        );
                        let forward = snapshot
                            .tab_snapshot()
                            .point_to_tab_point(point, Bias::Left);
                        assert_eq!(forward.row(), row as u32);
                        assert_eq!(
                            snapshot
                                .tab_snapshot()
                                .tab_point_to_point(forward, Bias::Left),
                            point
                        );
                        let column = boundaries
                            .binary_search(&(forward.column() as usize))
                            .expect("tab grapheme boundary")
                            as u32;
                        assert_eq!(
                            query.points_for_row(row as u32, &(column..column)),
                            Some((point, point))
                        );
                        let display = point.to_display_point(snapshot);
                        assert!(!snapshot.is_block_line(display.row()));
                        assert_eq!(display.to_point(snapshot), point);
                    }
                }
            }
        }
        assert_eq!(query.points_for_row(lines.len() as u32, &(0..0)), None);
    }
}
