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

pub fn marked_text_ranges_by(
    marked_text: &str,
    delimiters: Vec<(char, char)>,
) -> (String, HashMap<(char, char), Vec<Range<usize>>>) {
    let all_markers = delimiters
        .iter()
        .flat_map(|(start, end)| [*start, *end])
        .collect();
    let (unmarked_text, mut markers) = marked_text_by(marked_text, all_markers);
    let range_lookup = delimiters
        .into_iter()
        .map(|(start_marker, end_marker)| {
            let starts = markers.remove(&start_marker).unwrap_or_default();
            let ends = markers.remove(&end_marker).unwrap_or_default();
            assert_eq!(starts.len(), ends.len(), "marked ranges are unbalanced");

            let ranges = starts
                .into_iter()
                .zip(ends)
                .map(|(start, end)| {
                    assert!(end >= start, "marked ranges must be disjoint");
                    start..end
                })
                .collect::<Vec<Range<usize>>>();
            ((start_marker, end_marker), ranges)
        })
        .collect();

    (unmarked_text, range_lookup)
}

// Returns ranges delimited by (), [], and <> ranges. Ranges using the same markers
// must not be overlapping. May also include | for empty ranges
pub fn marked_text_ranges(full_marked_text: &str) -> (String, Vec<Range<usize>>) {
    let (range_marked_text, empty_offsets) = marked_text(full_marked_text);
    let (unmarked, range_lookup) =
        marked_text_ranges_by(&range_marked_text, vec![('[', ']'), ('(', ')'), ('<', '>')]);
    let mut combined_ranges: Vec<_> = range_lookup
        .into_values()
        .flatten()
        .chain(empty_offsets.into_iter().map(|offset| offset..offset))
        .collect();

    combined_ranges.sort_by_key(|range| range.start);
    (unmarked, combined_ranges)
}
