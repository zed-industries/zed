use std::ops::Range;

use collections::HashMap;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

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

pub fn marked_text_ranges(
    marked_text: &str,
    range_markers: Vec<(char, char)>,
) -> (String, Vec<Range<usize>>) {
    let mut marker_chars = Vec::new();
    for (start, end) in range_markers.iter() {
        marker_chars.push(*start);
        marker_chars.push(*end);
    }
    let (unmarked_text, markers) = marked_text_by(marked_text, marker_chars);
    let ranges = range_markers
        .iter()
        .map(|(start_marker, end_marker)| {
            let start = markers.get(start_marker).unwrap()[0];
            let end = markers.get(end_marker).unwrap()[0];
            start..end
        })
        .collect();
    (unmarked_text, ranges)
}
