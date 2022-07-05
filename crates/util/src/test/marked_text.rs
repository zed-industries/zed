use std::{collections::HashMap, ops::Range};

pub fn marked_text_by(
    marked_text: &str,
    markers: Vec<char>,
) -> (String, HashMap<char, Vec<usize>>) {
    let mut extracted_markers: HashMap<char, Vec<usize>> = Default::default();
    let mut unmarked_text = String::new();

    for char in marked_text.chars() {
        if markers.contains(&char) {
            let char_offsets = extracted_markers.entry(char).or_insert(Vec::new());
            char_offsets.push(unmarked_text.len());
        } else {
            unmarked_text.push(char);
        }
    }

    (unmarked_text, extracted_markers)
}

pub fn marked_text(marked_text: &str) -> (String, Vec<usize>) {
    let (unmarked_text, mut markers) = marked_text_by(marked_text, vec!['|']);
    (unmarked_text, markers.remove(&'|').unwrap_or_default())
}

#[derive(Eq, PartialEq, Hash)]
pub enum TextRangeMarker {
    Empty(char),
    Range(char, char),
    ReverseRange(char, char),
}

impl TextRangeMarker {
    fn markers(&self) -> Vec<char> {
        match self {
            Self::Empty(m) => vec![*m],
            Self::Range(l, r) => vec![*l, *r],
            Self::ReverseRange(l, r) => vec![*l, *r],
        }
    }
}

impl From<char> for TextRangeMarker {
    fn from(marker: char) -> Self {
        Self::Empty(marker)
    }
}

impl From<(char, char)> for TextRangeMarker {
    fn from((left_marker, right_marker): (char, char)) -> Self {
        Self::Range(left_marker, right_marker)
    }
}

pub fn marked_text_ranges_by(
    marked_text: &str,
    markers: Vec<TextRangeMarker>,
) -> (String, HashMap<TextRangeMarker, Vec<Range<usize>>>) {
    let all_markers = markers.iter().flat_map(|m| m.markers()).collect();

    let (unmarked_text, mut marker_offsets) = marked_text_by(marked_text, all_markers);
    let range_lookup = markers
        .into_iter()
        .map(|marker| match marker {
            TextRangeMarker::Empty(empty_marker_char) => {
                let ranges = marker_offsets
                    .remove(&empty_marker_char)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|empty_index| empty_index..empty_index)
                    .collect::<Vec<Range<usize>>>();
                (marker, ranges)
            }
            TextRangeMarker::Range(start_marker, end_marker) => {
                let starts = marker_offsets.remove(&start_marker).unwrap_or_default();
                let ends = marker_offsets.remove(&end_marker).unwrap_or_default();
                assert_eq!(starts.len(), ends.len(), "marked ranges are unbalanced");

                let ranges = starts
                    .into_iter()
                    .zip(ends)
                    .map(|(start, end)| {
                        assert!(end >= start, "marked ranges must be disjoint");
                        start..end
                    })
                    .collect::<Vec<Range<usize>>>();
                (marker, ranges)
            }
            TextRangeMarker::ReverseRange(start_marker, end_marker) => {
                let starts = marker_offsets.remove(&start_marker).unwrap_or_default();
                let ends = marker_offsets.remove(&end_marker).unwrap_or_default();
                assert_eq!(starts.len(), ends.len(), "marked ranges are unbalanced");

                let ranges = starts
                    .into_iter()
                    .zip(ends)
                    .map(|(start, end)| {
                        assert!(start >= end, "marked ranges must be disjoint");
                        end..start
                    })
                    .collect::<Vec<Range<usize>>>();
                (marker, ranges)
            }
        })
        .collect();

    (unmarked_text, range_lookup)
}

// Returns ranges delimited by (), [], and <> ranges. Ranges using the same markers
// must not be overlapping. May also include | for empty ranges
pub fn marked_text_ranges(full_marked_text: &str) -> (String, Vec<Range<usize>>) {
    let (unmarked, range_lookup) = marked_text_ranges_by(
        &full_marked_text,
        vec![
            '|'.into(),
            ('[', ']').into(),
            ('(', ')').into(),
            ('<', '>').into(),
        ],
    );
    let mut combined_ranges: Vec<_> = range_lookup.into_values().flatten().collect();

    combined_ranges.sort_by_key(|range| range.start);
    (unmarked, combined_ranges)
}
