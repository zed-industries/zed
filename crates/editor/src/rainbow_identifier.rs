use multi_buffer::MultiBufferSnapshot;
use std::ops::Range;

/// Extracts a complete identifier from the buffer at the given chunk position.
/// This function walks backward and forward from the chunk boundaries to find
/// the complete identifier, using efficient O(n) rope iteration.
#[inline]
pub fn extract_complete_identifier(
    buffer: &MultiBufferSnapshot,
    chunk_range: Range<usize>,
) -> Option<(Range<usize>, String)> {
    let total_len = buffer.len();
    
    if chunk_range.start >= total_len {
        return None;
    }
    
    // Find identifier start by walking backward
    let mut start = chunk_range.start;
    let chars_before = buffer.reversed_chars_at(start);
    
    for ch in chars_before {
        if !ch.is_alphanumeric() && ch != '_' {
            break;
        }
        start = start.saturating_sub(ch.len_utf8());
        if start == 0 {
            break;
        }
    }
    
    // Find identifier end by walking forward  
    let mut end = chunk_range.end.min(total_len);
    let chars_after = buffer.chars_at(end);
    
    for ch in chars_after {
        if !ch.is_alphanumeric() && ch != '_' {
            break;
        }
        end += ch.len_utf8();
        if end >= total_len {
            break;
        }
    }
    
    if start < end && end <= total_len {
        let identifier: String = buffer.text_for_range(start..end).collect();
        Some((start..end, identifier))
    } else {
        None
    }
}


/// Validates if a string is a complete, valid identifier.
/// Does NOT include language-specific keyword filtering - that should
/// be done by the Language layer.
#[inline]
pub fn is_valid_identifier(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    
    // Skip if contains whitespace or control characters
    if text.contains(char::is_whitespace) || text.contains(|c: char| c.is_control()) {
        return false;
    }
    
    // Skip underscore-only strings
    if text.chars().all(|c| c == '_') {
        return false;
    }
    
    // Must be valid identifier: starts with letter/_, rest alphanumeric/_
    let mut chars = text.chars();
    match chars.next() {
        Some(first) if first.is_alphabetic() || first == '_' => {
            chars.all(|c| c.is_alphanumeric() || c == '_')
        }
        _ => false,
    }
}
