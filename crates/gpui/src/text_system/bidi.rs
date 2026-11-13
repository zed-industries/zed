use super::{DecorationRun, FontRun, LineLayout, LineWrapper, ShapedRun};
use crate::{Pixels, px};
use std::{collections::BTreeMap, ops::Range, sync::Arc};
use unicode_bidi::{BidiClass, BidiInfo, Level, bidi_class};
use unicode_segmentation::UnicodeSegmentation;

/// A visual row with its original UTF-8 offsets retained for selection and copying.
#[derive(Debug)]
pub struct BidiRow {
    /// This row's byte range in the original paragraph.
    pub range: Range<usize>,
    /// Glyph positions for this row, with row-local UTF-8 indices.
    pub layout: Arc<LineLayout>,
    carets: BidiCaretMap,
}

/// Grapheme caret positions in visual order, retaining logical UTF-8 offsets.
#[derive(Debug, Clone)]
pub struct BidiCaretMap {
    width: Pixels,
    /// Base direction used when moving across visual row edges.
    pub paragraph_is_rtl: bool,
    clusters: Vec<BidiCluster>,
}

#[derive(Debug, Clone)]
struct BidiCluster {
    range: Range<usize>,
    left: Pixels,
    right: Pixels,
    carets: Vec<(usize, Pixels)>,
}

pub(crate) fn contains_rtl(text: &str) -> bool {
    text.chars().any(|character| {
        matches!(
            bidi_class(character),
            BidiClass::R | BidiClass::AL | BidiClass::RLE | BidiClass::RLO | BidiClass::RLI
        )
    })
}

impl BidiRow {
    fn new(
        text: &str,
        range: Range<usize>,
        layout: Arc<LineLayout>,
        base_level: Option<Level>,
    ) -> Self {
        let carets = BidiCaretMap::new(text, &layout, base_level);
        Self {
            range,
            layout,
            carets,
        }
    }

    /// Maps a horizontal position to the paragraph's source offsets.
    pub fn index_for_x(&self, x: Pixels, closest: bool) -> Result<usize, usize> {
        self.carets
            .index_for_x(x, closest)
            .map(|index| self.range.start + index)
            .map_err(|index| self.range.start + index)
    }

    /// Returns the horizontal position of a source offset.
    pub fn x_for_index(&self, index: usize) -> Pixels {
        self.carets
            .x_for_index(index.saturating_sub(self.range.start))
    }

    /// Visual spans occupied by a logical selection in this row.
    pub fn selection_spans(&self, range: Range<usize>) -> Vec<Range<Pixels>> {
        self.carets.selection_spans(
            range.start.max(self.range.start) - self.range.start
                ..range
                    .end
                    .min(self.range.end)
                    .saturating_sub(self.range.start),
        )
    }

    pub(crate) fn paint_layout(
        &self,
        decorations: &[DecorationRun],
    ) -> (LineLayout, Vec<DecorationRun>) {
        paint_bidi_layout(&self.layout, self.range.start, decorations)
    }
}

pub(crate) fn paint_bidi_layout(
    layout: &LineLayout,
    source_start: usize,
    decorations: &[DecorationRun],
) -> (LineLayout, Vec<DecorationRun>) {
    // The existing painter advances styles in byte order. Give it a visual-only
    // index space while keeping source offsets in the row used for hit testing.
    let mut glyphs: Vec<_> = layout
        .runs
        .iter()
        .flat_map(|run| {
            run.glyphs
                .iter()
                .map(move |glyph| (run.font_id, glyph.clone()))
        })
        .collect();
    glyphs.sort_by(|(_, left), (_, right)| {
        f32::from(left.position.x).total_cmp(&f32::from(right.position.x))
    });
    let mut runs: Vec<ShapedRun> = Vec::new();
    let mut visual_decorations = Vec::new();
    for (visual_index, (font_id, mut glyph)) in glyphs.into_iter().enumerate() {
        let source_index = source_start + glyph.index;
        let mut offset = 0;
        let decoration = decorations.iter().find(|decoration| {
            offset += decoration.len as usize;
            source_index < offset
        });
        if let Some(decoration) = decoration {
            let mut decoration = decoration.clone();
            decoration.len = 1;
            visual_decorations.push(decoration);
        }
        glyph.index = visual_index;
        if let Some(run) = runs.last_mut().filter(|run| run.font_id == font_id) {
            run.glyphs.push(glyph);
        } else {
            runs.push(ShapedRun {
                font_id,
                glyphs: vec![glyph],
            });
        }
    }
    let len = runs.iter().map(|run| run.glyphs.len()).sum();
    (
        LineLayout {
            bidi_carets: None,
            font_size: layout.font_size,
            width: layout.width,
            ascent: layout.ascent,
            descent: layout.descent,
            runs,
            len,
        },
        visual_decorations,
    )
}

impl BidiCaretMap {
    pub(crate) fn new(text: &str, layout: &LineLayout, base_level: Option<Level>) -> Self {
        let bidi = BidiInfo::new(text, base_level);
        let mut cluster_positions: BTreeMap<usize, Pixels> = BTreeMap::new();
        for glyph in layout.runs.iter().flat_map(|run| &run.glyphs) {
            cluster_positions
                .entry(glyph.index)
                .and_modify(|left| *left = (*left).min(glyph.position.x))
                .or_insert(glyph.position.x);
        }
        let starts: Vec<_> = cluster_positions
            .keys()
            .copied()
            .chain([text.len()])
            .collect();
        let mut positions: Vec<_> = cluster_positions.values().copied().collect();
        positions.push(layout.width);
        positions.sort_by(|left, right| f32::from(*left).total_cmp(&f32::from(*right)));
        positions.dedup();
        let mut clusters = Vec::new();
        for indices in starts.windows(2) {
            let start = indices[0];
            let end = indices[1];
            let Some(cluster_text) = text.get(start..end) else {
                continue;
            };
            let Some(&left) = cluster_positions.get(&start) else {
                continue;
            };
            let right = positions
                .get(positions.partition_point(|position| *position <= left))
                .copied()
                .unwrap_or(layout.width)
                .max(left);
            let rtl = bidi.levels.get(start).is_some_and(|level| level.is_rtl());
            let mut boundaries: Vec<_> = cluster_text
                .grapheme_indices(true)
                .map(|(offset, _)| start + offset)
                .collect();
            boundaries.push(end);
            let count = boundaries.len().saturating_sub(1).max(1) as f32;
            let carets = boundaries
                .into_iter()
                .enumerate()
                .map(|(index, offset)| {
                    let advance = (right - left) * (index as f32 / count);
                    (offset, if rtl { right - advance } else { left + advance })
                })
                .collect();
            clusters.push(BidiCluster {
                range: start..end,
                left,
                right,
                carets,
            });
        }
        Self {
            width: layout.width,
            paragraph_is_rtl: bidi
                .paragraphs
                .first()
                .is_some_and(|paragraph| paragraph.level.is_rtl()),
            clusters,
        }
    }

    /// Maps a horizontal position to a source byte offset; out-of-row positions return `Err`.
    pub fn index_for_x(&self, x: Pixels, closest: bool) -> Result<usize, usize> {
        let cluster = self
            .clusters
            .iter()
            .find(|cluster| x >= cluster.left && x < cluster.right);
        if let Some(cluster) = cluster {
            let index = if closest {
                cluster
                    .carets
                    .iter()
                    .min_by(|(_, left), (_, right)| {
                        f32::from((*left - x).abs()).total_cmp(&f32::from((*right - x).abs()))
                    })
                    .map(|(index, _)| *index)
                    .unwrap_or(cluster.range.start)
            } else {
                cluster.range.start
            };
            return Ok(index);
        }
        let nearest = self
            .clusters
            .iter()
            .flat_map(|cluster| &cluster.carets)
            .min_by(|(_, left), (_, right)| {
                f32::from((*left - x).abs()).total_cmp(&f32::from((*right - x).abs()))
            })
            .map(|(index, _)| *index)
            .unwrap_or(0);
        Err(nearest)
    }

    /// Returns the row-local horizontal position of a source byte offset.
    pub fn x_for_index(&self, index: usize) -> Pixels {
        let local_index = index;
        self.clusters
            .iter()
            .flat_map(|cluster| &cluster.carets)
            .find(|(index, _)| *index >= local_index)
            .or_else(|| {
                self.clusters
                    .last()
                    .and_then(|cluster| cluster.carets.last())
            })
            .map(|(_, x)| *x)
            .unwrap_or(self.width)
    }

    /// Visual horizontal spans occupied by a logical selection in the paragraph.
    pub fn selection_spans(&self, range: Range<usize>) -> Vec<Range<Pixels>> {
        let start = range.start;
        let end = range.end;
        let mut spans = Vec::new();
        for cluster in &self.clusters {
            if cluster.range.start >= end || cluster.range.end <= start {
                continue;
            }
            let caret = |index| {
                cluster
                    .carets
                    .iter()
                    .find(|(offset, _)| *offset == index)
                    .map(|(_, x)| *x)
            };
            let first = caret(start.max(cluster.range.start)).unwrap_or(cluster.left);
            let last = caret(end.min(cluster.range.end)).unwrap_or(cluster.right);
            spans.push(first.min(last)..first.max(last));
        }
        spans.sort_by(|left, right| f32::from(left.start).total_cmp(&f32::from(right.start)));
        let mut merged: Vec<Range<Pixels>> = Vec::new();
        for span in spans {
            if let Some(previous) = merged.last_mut()
                && span.start <= previous.end + px(0.01)
            {
                previous.end = previous.end.max(span.end);
            } else {
                merged.push(span);
            }
        }
        merged
    }

    pub(crate) fn slice(&self, range: Range<usize>) -> (Self, Pixels) {
        let spans = self.selection_spans(range.clone());
        let left = spans.first().map_or(px(0.), |span| span.start);
        let right = spans.last().map_or(left, |span| span.end);
        let clusters = self
            .clusters
            .iter()
            .filter_map(|cluster| {
                if cluster.range.start >= range.end || cluster.range.end <= range.start {
                    return None;
                }
                Some(BidiCluster {
                    range: cluster.range.start.max(range.start) - range.start
                        ..cluster.range.end.min(range.end) - range.start,
                    left: cluster.left.max(left) - left,
                    right: cluster.right.min(right) - left,
                    carets: cluster
                        .carets
                        .iter()
                        .filter(|(index, _)| range.start <= *index && *index <= range.end)
                        .map(|(index, x)| (index - range.start, *x - left))
                        .collect(),
                })
            })
            .collect();
        (
            Self {
                width: right - left,
                paragraph_is_rtl: self.paragraph_is_rtl,
                clusters,
            },
            left,
        )
    }

    pub(crate) fn width(&self) -> Pixels {
        self.width
    }

    /// The neighboring caret in screen order, or `None` at a visual edge.
    pub fn visual_neighbor(&self, index: usize, right: bool) -> Option<usize> {
        let mut positions = BTreeMap::new();
        for (offset, x) in self.clusters.iter().flat_map(|cluster| &cluster.carets) {
            positions.entry(*offset).or_insert(*x);
        }
        let mut positions: Vec<_> = positions.into_iter().collect();
        positions.sort_by(|(left_index, left), (right_index, right)| {
            f32::from(*left)
                .total_cmp(&f32::from(*right))
                .then(left_index.cmp(right_index))
        });
        let position = positions.iter().position(|(offset, _)| *offset == index)?;
        let neighbor = if right {
            position.checked_add(1)?
        } else {
            position.checked_sub(1)?
        };
        positions.get(neighbor).map(|(offset, _)| *offset)
    }
}

pub(crate) fn layout_bidi_rows(
    text: &str,
    font_runs: &[FontRun],
    wrap_width: Option<Pixels>,
    max_lines: Option<usize>,
    mut shape: impl FnMut(&str, &[FontRun]) -> Arc<LineLayout>,
) -> Vec<BidiRow> {
    let base_level = BidiInfo::new(text, None)
        .paragraphs
        .first()
        .map(|paragraph| paragraph.level)
        .unwrap_or(Level::ltr());
    let direction_mark = if base_level.is_rtl() {
        '\u{200f}'
    } else {
        '\u{200e}'
    };
    let shape_range =
        |range: Range<usize>, shape: &mut dyn FnMut(&str, &[FontRun]) -> Arc<LineLayout>| {
            let mut offset = 0;
            let mut runs: Vec<_> = font_runs
                .iter()
                .filter_map(|run| {
                    let start = offset;
                    offset += run.len;
                    let length = offset.min(range.end).saturating_sub(start.max(range.start));
                    (length > 0).then_some(FontRun {
                        len: length,
                        font_id: run.font_id,
                    })
                })
                .collect();
            // Continuation rows must retain the paragraph direction, including
            // rows beginning with a URL or number. The mark is shaping-only;
            // it never enters the source text, copied text, or hit-test offsets.
            let Some(first_run) = runs.first_mut() else {
                return shape(&text[range], &runs);
            };
            let prefix_length = direction_mark.len_utf8();
            first_run.len += prefix_length;
            let marked_text = format!("{}{}", direction_mark, &text[range.clone()]);
            let shaped = shape(&marked_text, &runs);
            let runs = shaped
                .runs
                .iter()
                .filter_map(|run| {
                    let glyphs: Vec<_> = run
                        .glyphs
                        .iter()
                        .filter_map(|glyph| {
                            let index = glyph.index.checked_sub(prefix_length)?;
                            let mut glyph = glyph.clone();
                            glyph.index = index;
                            Some(glyph)
                        })
                        .collect();
                    (!glyphs.is_empty()).then_some(ShapedRun {
                        font_id: run.font_id,
                        glyphs,
                    })
                })
                .collect();
            Arc::new(LineLayout {
                bidi_carets: None,
                font_size: shaped.font_size,
                width: shaped.width,
                ascent: shaped.ascent,
                descent: shaped.descent,
                len: range.len(),
                runs,
            })
        };
    let mut breaks: Vec<_> = LineWrapper::break_indices(text).collect();
    breaks.push(text.len());
    breaks.sort_unstable();
    breaks.dedup();
    let graphemes: Vec<_> = text
        .grapheme_indices(true)
        .map(|(index, _)| index)
        .chain([text.len()])
        .collect();
    let mut rows = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let width = wrap_width.unwrap_or(Pixels::MAX).max(px(1.));
        let candidates = &breaks[breaks.partition_point(|index| *index <= start)..];
        let mut low = 0;
        let mut high = candidates.len();
        let mut selected = None;
        if max_lines.is_some_and(|limit| rows.len() + 1 >= limit) {
            selected = Some((text.len(), shape_range(start..text.len(), &mut shape)));
        } else {
            // Probe nearby words first. Searching the whole remaining paragraph
            // would repeatedly shape very long prefixes while an agent streams.
            high = high.min(1);
            while high < candidates.len() {
                let end = candidates[high - 1];
                let layout = shape_range(start..end, &mut shape);
                if layout.width > width {
                    break;
                }
                selected = Some((end, layout));
                low = high;
                high = (high * 2).min(candidates.len());
            }
            while low < high {
                let middle = low + (high - low) / 2;
                let end = candidates[middle];
                let layout = shape_range(start..end, &mut shape);
                if layout.width <= width {
                    selected = Some((end, layout));
                    low = middle + 1;
                } else {
                    high = middle;
                }
            }
        }
        let (end, layout) = if let Some(selected) = selected {
            selected
        } else {
            let limit = candidates.first().copied().unwrap_or(text.len());
            let first = graphemes.partition_point(|index| *index <= start);
            let last = graphemes.partition_point(|index| *index <= limit);
            let candidates = &graphemes[first..last];
            let mut low = 0;
            let mut high = candidates.len().min(1);
            let mut selected = None;
            while high < candidates.len() {
                let end = candidates[high - 1];
                let layout = shape_range(start..end, &mut shape);
                if layout.width > width {
                    break;
                }
                selected = Some((end, layout));
                low = high;
                high = (high * 2).min(candidates.len());
            }
            while low < high {
                let middle = low + (high - low) / 2;
                let end = candidates[middle];
                let layout = shape_range(start..end, &mut shape);
                if layout.width <= width || middle == 0 {
                    selected = Some((end, layout));
                    low = middle + 1;
                } else {
                    high = middle;
                }
            }
            selected.unwrap_or_else(|| (text.len(), shape_range(start..text.len(), &mut shape)))
        };
        rows.push(BidiRow::new(
            &text[start..end],
            start..end,
            layout,
            Some(base_level),
        ));
        start = end;
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontId, GlyphId, ShapedGlyph, black, point};

    fn rtl_layout(text: &str) -> Arc<LineLayout> {
        let graphemes: Vec<_> = text
            .grapheme_indices(true)
            .filter(|(_, grapheme)| *grapheme != "\u{200f}" && *grapheme != "\u{200e}")
            .collect();
        let width = px(graphemes.len() as f32 * 10.);
        Arc::new(LineLayout {
            bidi_carets: None,
            font_size: px(16.),
            width,
            ascent: px(12.),
            descent: px(4.),
            len: text.len(),
            runs: vec![ShapedRun {
                font_id: FontId(0),
                glyphs: graphemes
                    .iter()
                    .enumerate()
                    .map(|(index, (offset, _))| ShapedGlyph {
                        id: GlyphId(index as u32),
                        index: *offset,
                        is_emoji: false,
                        position: point(width - px((index + 1) as f32 * 10.), px(0.)),
                    })
                    .collect(),
            }],
        })
    }

    #[test]
    fn arabic_wrap_keeps_words_in_reading_order() {
        let text = "مرحبا بكم جميعا";
        let runs = [FontRun {
            len: text.len(),
            font_id: FontId(0),
        }];
        let rows = layout_bidi_rows(text, &runs, Some(px(65.)), None, |text, _| rtl_layout(text));
        let texts: Vec<_> = rows.iter().map(|row| &text[row.range.clone()]).collect();
        assert_eq!(texts, ["مرحبا ", "بكم ", "جميعا"]);
        assert_eq!(rows[1].index_for_x(px(35.), true), Ok("مرحبا ".len()));
        assert_eq!(
            rows[1].selection_spans("مرحبا ".len().."مرحبا بكم".len()),
            [px(10.)..px(40.)]
        );
    }

    #[test]
    fn arabic_wrap_shapes_only_nearby_words_in_long_paragraphs() {
        let text = "مرحبا ".repeat(256);
        let runs = [FontRun {
            len: text.len(),
            font_id: FontId(0),
        }];
        let mut shaped_bytes = 0;
        let rows = layout_bidi_rows(&text, &runs, Some(px(65.)), None, |text, _| {
            shaped_bytes += text.len();
            rtl_layout(text)
        });
        assert_eq!(rows.len(), 256);
        assert!(
            shaped_bytes < text.len() * 10,
            "shaped {shaped_bytes} bytes for {} source bytes",
            text.len()
        );
    }

    #[test]
    fn arabic_wrap_does_not_split_combining_marks() {
        let text = "عَرَبِيّ";
        let runs = [FontRun {
            len: text.len(),
            font_id: FontId(0),
        }];
        let rows = layout_bidi_rows(text, &runs, Some(px(15.)), None, |text, _| rtl_layout(text));
        let texts: Vec<_> = rows.iter().map(|row| &text[row.range.clone()]).collect();
        assert_eq!(texts, text.graphemes(true).collect::<Vec<_>>());
    }

    #[test]
    fn arabic_visual_painting_preserves_source_styles() {
        let text = "ابجد";
        let row = BidiRow::new(text, 0..text.len(), rtl_layout(text), None);
        let first = black();
        let second = crate::Hsla { a: 0.5, ..black() };
        let decorations: Vec<_> = [first, second]
            .into_iter()
            .map(|color| DecorationRun {
                len: 4,
                color,
                background_color: None,
                underline: None,
                strikethrough: None,
            })
            .collect();
        let (layout, decorations) = row.paint_layout(&decorations);
        assert_eq!(
            decorations
                .iter()
                .map(|decoration| decoration.color)
                .collect::<Vec<_>>(),
            [second, second, first, first]
        );
        assert!(
            layout
                .runs
                .iter()
                .flat_map(|run| &run.glyphs)
                .map(|glyph| glyph.position.x)
                .is_sorted()
        );
        assert!(
            layout
                .runs
                .iter()
                .flat_map(|run| &run.glyphs)
                .map(|glyph| glyph.index)
                .is_sorted()
        );
        assert_eq!(row.selection_spans(0..text.len()), [px(0.)..px(40.)]);
    }
}
