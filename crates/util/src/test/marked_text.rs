use collections::HashMap;
use std::{cmp::Ordering, ops::Range};

/// Construct a string and a list of offsets within that string using a single
/// string containing embedded position markers.
pub fn marked_text_offsets_by(
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

/// Construct a string and a list of ranges within that string using a single
/// string containing embedded range markers, using arbitrary characters as
/// range markers. By using multiple different range markers, you can construct
/// ranges that overlap each other.
///
/// The returned ranges will be grouped by their range marking characters.
pub fn marked_text_ranges_by(
    marked_text: &str,
    markers: Vec<TextRangeMarker>,
) -> (String, HashMap<TextRangeMarker, Vec<Range<usize>>>) {
    let all_markers = markers.iter().flat_map(|m| m.markers()).collect();

    let (unmarked_text, mut marker_offsets) = marked_text_offsets_by(marked_text, all_markers);
    let range_lookup = markers
        .into_iter()
        .map(|marker| {
            (
                marker.clone(),
                match marker {
                    TextRangeMarker::Empty(empty_marker_char) => marker_offsets
                        .remove(&empty_marker_char)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|empty_index| empty_index..empty_index)
                        .collect::<Vec<Range<usize>>>(),
                    TextRangeMarker::Range(start_marker, end_marker) => {
                        let starts = marker_offsets.remove(&start_marker).unwrap_or_default();
                        let ends = marker_offsets.remove(&end_marker).unwrap_or_default();
                        assert_eq!(starts.len(), ends.len(), "marked ranges are unbalanced");
                        starts
                            .into_iter()
                            .zip(ends)
                            .map(|(start, end)| {
                                assert!(end >= start, "marked ranges must be disjoint");
                                start..end
                            })
                            .collect::<Vec<Range<usize>>>()
                    }
                    TextRangeMarker::ReverseRange(start_marker, end_marker) => {
                        let starts = marker_offsets.remove(&start_marker).unwrap_or_default();
                        let ends = marker_offsets.remove(&end_marker).unwrap_or_default();
                        assert_eq!(starts.len(), ends.len(), "marked ranges are unbalanced");
                        starts
                            .into_iter()
                            .zip(ends)
                            .map(|(start, end)| {
                                assert!(end >= start, "marked ranges must be disjoint");
                                end..start
                            })
                            .collect::<Vec<Range<usize>>>()
                    }
                },
            )
        })
        .collect();

    (unmarked_text, range_lookup)
}

/// Construct a string and a list of ranges within that string using a single
/// string containing embedded range markers. The characters used to mark the
/// ranges are as follows:
///
/// 1. To mark a range of text, surround it with the `«` and `»` angle brackets,
///    which can be typed on a US keyboard with the `alt-|` and `alt-shift-|` keys.
///
///    ```text
///    foo «selected text» bar
///    ```
///
/// 2. To mark a single position in the text, use the `ˇ` caron,
///    which can be typed on a US keyboard with the `alt-shift-t` key.
///
///    ```text
///    the cursors are hereˇ and hereˇ.
///    ```
///
/// 3. To mark a range whose direction is meaningful (like a selection),
///    put a caron character beside one of its bounds, on the inside:
///
///    ```text
///    one «ˇreversed» selection and one «forwardˇ» selection
///    ```
///
/// Any • characters in the input string will be replaced with spaces. This makes
/// it easier to test cases with trailing spaces, which tend to get trimmed from the
/// source code.
pub fn marked_text_ranges(
    marked_text: &str,
    ranges_are_directed: bool,
) -> (String, Vec<Range<usize>>) {
    let mut unmarked_text = String::with_capacity(marked_text.len());
    let mut ranges = Vec::new();
    let mut prev_marked_ix = 0;
    let mut current_range_start = None;
    let mut current_range_cursor = None;

    let marked_text = marked_text.replace("•", " ");
    for (marked_ix, marker) in marked_text.match_indices(&['«', '»', 'ˇ']) {
        unmarked_text.push_str(&marked_text[prev_marked_ix..marked_ix]);
        let unmarked_len = unmarked_text.len();
        let len = marker.len();
        prev_marked_ix = marked_ix + len;

        match marker {
            "ˇ" => {
                if current_range_start.is_some() {
                    if current_range_cursor.is_some() {
                        panic!("duplicate point marker 'ˇ' at index {marked_ix}");
                    } else {
                        current_range_cursor = Some(unmarked_len);
                    }
                } else {
                    ranges.push(unmarked_len..unmarked_len);
                }
            }
            "«" => {
                if current_range_start.is_some() {
                    panic!("unexpected range start marker '«' at index {marked_ix}");
                }
                current_range_start = Some(unmarked_len);
            }
            "»" => {
                let current_range_start = if let Some(start) = current_range_start.take() {
                    start
                } else {
                    panic!("unexpected range end marker '»' at index {marked_ix}");
                };

                let mut reversed = false;
                if let Some(current_range_cursor) = current_range_cursor.take() {
                    if current_range_cursor == current_range_start {
                        reversed = true;
                    } else if current_range_cursor != unmarked_len {
                        panic!("unexpected 'ˇ' marker in the middle of a range");
                    }
                } else if ranges_are_directed {
                    panic!("missing 'ˇ' marker to indicate range direction");
                }

                ranges.push(if reversed {
                    unmarked_len..current_range_start
                } else {
                    current_range_start..unmarked_len
                });
            }
            _ => unreachable!(),
        }
    }

    unmarked_text.push_str(&marked_text[prev_marked_ix..]);
    (unmarked_text, ranges)
}

pub fn marked_text_offsets(marked_text: &str) -> (String, Vec<usize>) {
    let (text, ranges) = marked_text_ranges(marked_text, false);
    (
        text,
        ranges
            .into_iter()
            .map(|range| {
                assert_eq!(range.start, range.end);
                range.start
            })
            .collect(),
    )
}

pub fn generate_marked_text(
    unmarked_text: &str,
    ranges: &[Range<usize>],
    indicate_cursors: bool,
    line_mode: bool,
) -> String {
    let ranges = if line_mode {
        let newline_positions = unmarked_text
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(pos, byte)| if *byte == b'\n' { Some(pos) } else { None })
            .collect::<Vec<_>>();

        ranges
            .iter()
            .map(|range| {
                let max_position = unmarked_text.len();
                let (start, end) = if range.start <= range.end {
                    (
                        find_previous_position(range.start, &newline_positions)
                            .map(|pos| std::cmp::min(pos + 1, range.end))
                            .unwrap_or(0),
                        find_next_position(range.end, &newline_positions).unwrap_or(max_position),
                    )
                } else {
                    (
                        find_next_position(range.start, &newline_positions).unwrap_or(max_position),
                        find_previous_position(range.end, &newline_positions)
                            .map(|pos| pos + 1)
                            .unwrap_or(0),
                    )
                };
                let cursor_pos = range.end;
                (start..end, cursor_pos)
            })
            .collect::<Vec<_>>()
    } else {
        ranges
            .iter()
            .map(|range| (range.clone(), range.end))
            .collect::<Vec<_>>()
    };

    let mut marked_text = unmarked_text.to_string();
    for (range, cursor) in ranges.iter().rev() {
        if indicate_cursors {
            match range.start.cmp(&range.end) {
                Ordering::Less => {
                    marked_text.insert(range.end, '»');
                    marked_text.insert(*cursor, 'ˇ');
                    marked_text.insert(range.start, '«');
                }
                Ordering::Equal => {
                    if line_mode {
                        marked_text.insert_str(range.start, "«ˇ»");
                    } else {
                        marked_text.insert(range.start, 'ˇ');
                    }
                }
                Ordering::Greater => {
                    marked_text.insert(range.start, '»');
                    marked_text.insert(*cursor, 'ˇ');
                    marked_text.insert(range.end, '«');
                }
            }
        } else {
            marked_text.insert(range.end, '»');
            marked_text.insert(range.start, '«');
        }
    }
    marked_text
}

fn find_previous_position(target: usize, positions: &[usize]) -> Option<usize> {
    positions
        .iter()
        .cloned()
        .take_while(|pos| *pos < target)
        .last()
}

fn find_next_position(target: usize, positions: &[usize]) -> Option<usize> {
    positions.iter().cloned().find(|pos| *pos >= target)
}

#[derive(Clone, Eq, PartialEq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::{generate_marked_text, marked_text_ranges};

    #[test]
    fn test_marked_text() {
        let (text, ranges) = marked_text_ranges("one «ˇtwo» «threeˇ» «ˇfour» fiveˇ six", true);

        assert_eq!(text, "one two three four five six");
        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0], 7..4);
        assert_eq!(ranges[1], 8..13);
        assert_eq!(ranges[2], 18..14);
        assert_eq!(ranges[3], 23..23);

        assert_eq!(
            generate_marked_text(&text, &ranges, true, false),
            "one «ˇtwo» «threeˇ» «ˇfour» fiveˇ six"
        );
    }

    #[test]
    fn test_marked_text_for_line_mode() {
        let (text, ranges) =
            marked_text_ranges("«The quick ˇ»brown fox jumps over the lazy dog.", true);
        assert_eq!(text, "The quick brown fox jumps over the lazy dog.");
        assert_eq!(ranges, vec![0..10]);
        assert_eq!(
            generate_marked_text(&text, &ranges, true, true),
            "«The quick ˇbrown fox jumps over the lazy dog.»"
        );

        let (text, ranges) = marked_text_ranges(
            "«The quick ˇ»\n brown «\nˇ» fox \n jump«ˇs »\n«ˇ»\n o«ˇver \n th»e lazy dog.",
            true,
        );
        assert_eq!(
            text,
            "The quick \n brown \n fox \n jumps \n\n over \n the lazy dog."
        );
        assert_eq!(ranges, vec![0..10, 18..19, 32..30, 33..33, 44..36]);
        assert_eq!(
            generate_marked_text(&text, &ranges, true, true),
            "«The quick ˇ»\n« brown \nˇ fox »\n« jumpˇs »\n«ˇ»\n« oˇver \n the lazy dog.»"
        );
    }
}
