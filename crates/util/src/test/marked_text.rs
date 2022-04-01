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
    (unmarked_text, markers.remove(&'|').unwrap_or_else(Vec::new))
}

pub fn marked_text_ranges(marked_text: &str) -> (String, Vec<Range<usize>>) {
    let (unmarked_text, mut markers) = marked_text_by(marked_text, vec!['[', ']']);
    let opens = markers.remove(&'[').unwrap_or_default();
    let closes = markers.remove(&']').unwrap_or_default();
    assert_eq!(opens.len(), closes.len(), "marked ranges are unbalanced");

    let ranges = opens
        .into_iter()
        .zip(closes)
        .map(|(open, close)| {
            assert!(close >= open, "marked ranges must be disjoint");
            open..close
        })
        .collect();
    (unmarked_text, ranges)
}
