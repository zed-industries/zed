
use gpui::ViewContext;
use std::{any::TypeId, ops::Range};
use regex::Regex;
use language::BufferSnapshot;
use crate::Editor;
use unicode_normalization::UnicodeNormalization;

enum BadUniCodeCharacter {}


pub fn search(re: &Regex, buffer: &BufferSnapshot) -> Vec<Range<usize>> {
    let mut matches = Vec::new();
    let rope = buffer.as_rope().clone();
    let text = rope.to_string().nfc().collect::<String>();  // Normalize text to NFC
    let mut current_start = None;
    let mut current_end = None;

    for mat in re.find_iter(&text) {
        match (current_start, current_end) {
            (None, None) => {
                current_start = Some(mat.start());
                current_end = Some(mat.end());
            },
            (Some(start), Some(end)) => {
                if mat.start() <= end {
                    // Update the end range if there's an overlap or contiguous match
                    current_end = Some(mat.end().max(end));
                } else {
                    // Push the current match and reset the range
                    matches.push(start..end);
                    current_start = Some(mat.start());
                    current_end = Some(mat.end());
                }
            },
            _ => unreachable!(),  // Ensures we handle both cases explicitly
        }
    }

    // Push the last match after the loop ends
    if let (Some(start), Some(end)) = (current_start, current_end) {
        matches.push(start..end);
    }

    matches
}

pub fn refresh_invalid_character_highlight(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    let buffer = editor.buffer().read(cx).snapshot(cx);
    
    // Updated Regex to capture multiple invalid Unicode character sequences
    let re = Regex::new(r"[\u{200B}\u{200C}\u{200D}\u{200E}\u{200F}\u{0000}-\u{0009}\u{000B}-\u{000C}\u{000E}-\u{001F}\u{007F}-\u{009F}\u{20E3}\u{20DD}]").unwrap();
    
    let mut ranges = Vec::new();
    if let Some((_, _, excerpt_buffer)) = buffer.as_singleton() {
        ranges.extend(
            search(&re, excerpt_buffer).into_iter().map(|matched_range| {
                // Adjust the range for highlighting
                println!("{:?}", matched_range.clone());
                buffer.anchor_after(matched_range.start)
                    ..buffer.anchor_before(matched_range.end)
            }),
        )
    }

    // Apply highlights for all found matches
    editor.highlight_background::<BadUniCodeCharacter>(&ranges, |theme| theme.editor_invisible, cx);
}


// pub fn search(re: &Regex, buffer: &BufferSnapshot) -> Vec<Range<usize>> {
//     let mut matches = Vec::new();
//     let rope = buffer.as_rope().clone();
//     let text = rope.to_string().nfc().collect::<String>();  // Normalize text to NFC
//     let mut current_start = None;
//     let mut current_end = None;

//     for mat in re.find_iter(&text) {
//         match (current_start, current_end) {
//             (None, None) => {
//                 current_start = Some(mat.start());
//                 current_end = Some(mat.end());
//             },
//             (Some(start), Some(end)) => {
//                 if mat.start() <= end {
//                     current_end = Some(mat.end().max(end));
//                 } else {
//                     matches.push(start..end);
//                     current_start = Some(mat.start());
//                     current_end = Some(mat.end());
//                 }
//             },
//             _ => unreachable!(),
//         }
//     }
//     if let (Some(start), Some(end)) = (current_start, current_end) {
//         matches.push(start..end);
//     }

//     matches
// }
// pub fn refresh_invalid_character_highlight(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
//     let buffer = editor.buffer().read(cx).snapshot(cx);
//     let re = Regex::new(r"[\u{200B}\u{200C}\u{200D}\u{200E}\u{200F}\u{0000}-\u{0009}\u{000B}-\u{000C}\u{000E}-\u{001F}\u{007F}-\u{009F}\u{20E3}\u{20DD}]").unwrap();
//     let mut ranges = Vec::new();
//     if let Some((_, _, excerpt_buffer)) = buffer.as_singleton() {
//         ranges.extend(
//             search(&re, excerpt_buffer).into_iter().map(|matched_range| {
//                 // Adjust the range for highlighting
//                 buffer.anchor_after(matched_range.start)
//                     ..buffer.anchor_before(matched_range.end)
//             }),
//         )
//     }
//     editor.highlight_background::<BadUniCodeCharacter>(&ranges, |theme| theme.editor_document_highlight_read_background, cx);
// }

