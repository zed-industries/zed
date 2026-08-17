//! Small entity decoding helpers used by a few render paths.
//!
//! This intentionally matches the current "minimal" decoding behavior and does not aim to be a
//! fully compliant HTML entity decoder.

use std::borrow::Cow;

/// Decodes a minimal subset of entities used by Mermaid labels.
///
/// This matches the historical replacement order used in this repo:
/// - `&lt;` / `&gt;` first
/// - `&amp;` next
/// - `&quot;` / `&#39;` last
///
/// The replacement order matters: for example `&amp;quot;` becomes `"` (two-step), while
/// `&amp;lt;` stays as `&lt;` (one-step).
pub(crate) fn decode_entities_minimal(text: &str) -> String {
    if !text.contains('&') {
        return text.to_string();
    }

    let stage1 = decode_stage1_lt_gt_amp(text);
    if !stage1.contains('&') {
        return stage1;
    }
    if !stage1.contains("&quot;") && !stage1.contains("&#39;") {
        return stage1;
    }
    decode_stage2_quot_apos(&stage1)
}

pub(crate) fn decode_entities_minimal_cow(text: &str) -> Cow<'_, str> {
    if !text.contains('&') {
        return Cow::Borrowed(text);
    }
    Cow::Owned(decode_entities_minimal(text))
}

fn decode_stage1_lt_gt_amp(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find('&') {
        out.push_str(&rest[..pos]);
        let tail = &rest[pos..];
        if let Some(stripped) = tail.strip_prefix("&lt;") {
            out.push('<');
            rest = stripped;
        } else if let Some(stripped) = tail.strip_prefix("&gt;") {
            out.push('>');
            rest = stripped;
        } else if let Some(stripped) = tail.strip_prefix("&amp;") {
            out.push('&');
            rest = stripped;
        } else {
            out.push('&');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
}

fn decode_stage2_quot_apos(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find('&') {
        out.push_str(&rest[..pos]);
        let tail = &rest[pos..];
        if let Some(stripped) = tail.strip_prefix("&quot;") {
            out.push('"');
            rest = stripped;
        } else if let Some(stripped) = tail.strip_prefix("&#39;") {
            out.push('\'');
            rest = stripped;
        } else {
            out.push('&');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::decode_entities_minimal;

    #[test]
    fn decode_entities_minimal_direct_entities() {
        assert_eq!(decode_entities_minimal("&lt;"), "<");
        assert_eq!(decode_entities_minimal("&gt;"), ">");
        assert_eq!(decode_entities_minimal("&amp;"), "&");
        assert_eq!(decode_entities_minimal("&quot;"), "\"");
        assert_eq!(decode_entities_minimal("&#39;"), "'");
    }

    #[test]
    fn decode_entities_minimal_preserves_unknown_entities() {
        assert_eq!(decode_entities_minimal("&unknown;"), "&unknown;");
        assert_eq!(decode_entities_minimal("a&b"), "a&b");
    }

    #[test]
    fn decode_entities_minimal_order_matters_like_replace_chain() {
        assert_eq!(decode_entities_minimal("&amp;quot;"), "\"");
        assert_eq!(decode_entities_minimal("&amp;#39;"), "'");
        assert_eq!(decode_entities_minimal("&amp;lt;"), "&lt;");
        assert_eq!(decode_entities_minimal("&amp;gt;"), "&gt;");
    }

    #[test]
    fn decode_entities_minimal_mixed_text() {
        assert_eq!(
            decode_entities_minimal("a &lt; b &amp;&amp; b &gt; c"),
            "a < b && b > c"
        );
    }
}
