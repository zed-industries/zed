use language_model::LanguageModelToolResultContent;
use std::borrow::Cow;

pub(crate) const TOOL_RESULT_TEXT_BYTE_BUDGET: usize = 16 * 1024;

pub(crate) const IMAGE_TOKEN_ESTIMATE: u64 = 1_500;

// Tokenizers vary by model and content. This favors early compaction for
// typical text and code; provider overflow recovery covers underestimates.
pub(crate) fn estimate_tokens(byte_len: usize) -> u64 {
    (byte_len as u64).div_ceil(3)
}

// Keep more of the tail because recent tool output usually contains the
// outcome of the operation; the head preserves identifying context.
pub(crate) fn truncate_text_to_budget(text: &str, budget: usize) -> Cow<'_, str> {
    if text.len() <= budget {
        return Cow::Borrowed(text);
    }

    let mut head_len = budget / 4;
    while !text.is_char_boundary(head_len) {
        head_len -= 1;
    }
    let mut tail_start = text.len() - (budget - budget / 4);
    while !text.is_char_boundary(tail_start) {
        tail_start += 1;
    }

    let kept_bytes = head_len + (text.len() - tail_start);
    let marker = format!("\n… [{} bytes elided] …\n", text.len() - kept_bytes);
    let mut truncated = String::with_capacity(kept_bytes + marker.len());
    truncated.push_str(&text[..head_len]);
    truncated.push_str(&marker);
    truncated.push_str(&text[tail_start..]);
    Cow::Owned(truncated)
}

pub(crate) fn truncate_tool_result(
    content: &mut Vec<LanguageModelToolResultContent>,
    budget: usize,
) {
    let text_bytes = content
        .iter()
        .map(|part| match part {
            LanguageModelToolResultContent::Text(text) => text.len(),
            LanguageModelToolResultContent::Image(_) => 0,
        })
        .sum::<usize>();
    if text_bytes <= budget {
        return;
    }

    let head_end = budget / 4;
    let tail_start = text_bytes - (budget - head_end);
    let mut offset = 0;
    let mut gap_marked = false;
    content.retain_mut(|part| {
        let LanguageModelToolResultContent::Text(text) = part else {
            return true;
        };
        let mut head_length = head_end.saturating_sub(offset).min(text.len());
        while !text.is_char_boundary(head_length) {
            head_length -= 1;
        }
        let mut tail_offset = tail_start.saturating_sub(offset).min(text.len());
        while !text.is_char_boundary(tail_offset) {
            tail_offset += 1;
        }
        offset += text.len();
        if head_length == text.len() || tail_offset == 0 {
            return true;
        }
        let mut kept = text[..head_length].to_string();
        if !gap_marked {
            kept.push_str("\n… [tool result text elided] …\n");
            gap_marked = true;
        }
        kept.push_str(&text[tail_offset..]);
        *text = kept.into();
        !text.is_empty()
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimate_tokens_rounds_up() {
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
        assert!(truncated.contains("bytes elided"));
        assert!(truncated.len() <= 1_000 + 40);
    }

    #[test]
    fn truncate_multibyte_does_not_panic() {
        let text = "ж".repeat(10_000);

        let truncated = truncate_text_to_budget(&text, 300).into_owned();

        assert!(truncated.starts_with('ж'));
        assert!(truncated.ends_with('ж'));
        assert!(truncated.contains("bytes elided"));
    }

    #[test]
    fn truncate_tiny_and_unlimited_budgets() {
        let text = "🙂жa";
        for budget in 0..=text.len() {
            let truncated = truncate_text_to_budget(text, budget);
            assert!(truncated.len() <= budget + 40);
        }
        assert!(matches!(
            truncate_text_to_budget(text, usize::MAX),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn multipart_truncation_preserves_images_and_order() {
        use language_model::{
            LanguageModelImage,
            LanguageModelToolResultContent::{Image, Text},
        };
        let first_image = LanguageModelImage {
            source: "first".into(),
        };
        let last_image = LanguageModelImage {
            source: "last".into(),
        };
        for budget in 0..=20 {
            let mut content = vec![
                Text("🙂".repeat(10).into()),
                Image(first_image.clone()),
                Text("ж".repeat(10).into()),
                Image(last_image.clone()),
                Text("z".repeat(10).into()),
            ];
            truncate_tool_result(&mut content, budget);
            let images = content
                .iter()
                .filter_map(|part| match part {
                    Image(image) => Some(image),
                    Text(_) => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(images, [&first_image, &last_image]);
            let text = content
                .iter()
                .filter_map(|part| match part {
                    Text(text) => Some(text.as_ref()),
                    Image(_) => None,
                })
                .collect::<String>();
            assert!(text.len() <= budget + 40);
            assert_eq!(text.matches("elided").count(), 1);
        }
    }
}
