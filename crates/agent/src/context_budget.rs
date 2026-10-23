//! Budget helpers for keeping model requests inside the context window:
//! conservative local token estimation and text truncation for oversized
//! tool results.

use std::borrow::Cow;

pub(crate) const TOOL_RESULT_TEXT_BYTE_BUDGET: usize = 16 * 1024;

pub(crate) const IMAGE_TOKEN_ESTIMATE: u64 = 1_500;

/// Estimates tokens from byte length at roughly 3 bytes per token, which
/// overestimates typical text and code so that budget checks err toward
/// compacting too early rather than overflowing the window.
pub(crate) fn estimate_tokens(byte_len: usize) -> u64 {
    (byte_len as u64).div_ceil(3)
}

/// Truncates `text` to at most `budget` bytes of content by keeping the
/// head quarter and the tail three quarters and marking the elided gap.
/// Cuts land on char boundaries so multi-byte text cannot panic.
pub(crate) fn truncate_text_to_budget(text: &str, budget: usize) -> Cow<'_, str> {
    if text.len() <= budget {
        return Cow::Borrowed(text);
    }

    let mut head_len = budget / 4;
    while !text.is_char_boundary(head_len) {
        head_len -= 1;
    }
    let mut tail_start = text.len() - budget * 3 / 4;
    while !text.is_char_boundary(tail_start) {
        tail_start += 1;
    }

    let kept_bytes = head_len + (text.len() - tail_start);
    let marker = format!("\n… [{} chars elided] …\n", text.len() - kept_bytes);
    let mut truncated = String::with_capacity(kept_bytes + marker.len());
    truncated.push_str(&text[..head_len]);
    truncated.push_str(&marker);
    truncated.push_str(&text[tail_start..]);
    Cow::Owned(truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimate_tokens_is_conservative() {
        assert_eq!(estimate_tokens(0), 0);
        assert_eq!(estimate_tokens(1), 1);
        assert_eq!(estimate_tokens(3), 1);
        assert_eq!(estimate_tokens(4), 2);
        assert!(estimate_tokens(40_000) >= 13_000);
    }

    #[test]
    fn truncate_noop_under_budget() {
        let text = "short text";
        assert_eq!(truncate_text_to_budget(text, 100), text);
    }

    #[test]
    fn truncate_keeps_head_tail_and_marks_gap() {
        let mut text = "a".repeat(10_000);
        text.push_str(&"b".repeat(1_000));

        let truncated = truncate_text_to_budget(&text, 1_000).into_owned();

        assert!(truncated.starts_with("aaa"));
        assert!(truncated.ends_with("bbb"));
        assert!(truncated.contains("chars elided"));
        assert!(truncated.len() <= 1_000 + 40);
    }

    #[test]
    fn truncate_multibyte_does_not_panic() {
        let text = "ж".repeat(10_000);

        let truncated = truncate_text_to_budget(&text, 300).into_owned();

        assert!(truncated.starts_with('ж'));
        assert!(truncated.ends_with('ж'));
        assert!(truncated.contains("chars elided"));
    }
}
