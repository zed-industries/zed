use std::borrow::Cow;

use gpui::SharedString;

/// Replace GitHub-style `:shortcode:` tokens with the corresponding emoji.
///
/// Tokens that are not a known shortcode stay as written, so a time like
/// `10:30` or a URL is left alone. The stored commit message is unchanged.
pub(crate) fn render_commit_text(input: SharedString) -> SharedString {
    match expand_emoji_shortcodes(input.as_ref()) {
        Cow::Borrowed(_) => input,
        Cow::Owned(expanded) => expanded.into(),
    }
}

pub(crate) fn expand_emoji_shortcodes(input: &str) -> Cow<'_, str> {
    if !input.contains(':') {
        return Cow::Borrowed(input);
    }

    let bytes = input.as_bytes();
    let mut expanded = String::new();
    let mut last = 0;
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] != b':' {
            index += 1;
            continue;
        }

        let name_start = index + 1;
        let mut name_end = name_start;
        while name_end < bytes.len() && is_shortcode_byte(bytes[name_end]) {
            name_end += 1;
        }

        if name_end > name_start && name_end < bytes.len() && bytes[name_end] == b':' {
            let shortcode = &input[name_start..name_end];
            if let Some(emoji) = emojis::get_by_shortcode(shortcode) {
                if expanded.is_empty() {
                    expanded.reserve(input.len());
                }
                expanded.push_str(&input[last..index]);
                expanded.push_str(emoji.as_str());
                last = name_end + 1;
                index = last;
                continue;
            }
            // The closing colon may open another shortcode, as in `:nope:bug:`.
            index = name_end;
            continue;
        }

        index += 1;
    }

    if expanded.is_empty() {
        Cow::Borrowed(input)
    } else {
        expanded.push_str(&input[last..]);
        Cow::Owned(expanded)
    }
}

fn is_shortcode_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'+' | b'-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_github_shortcodes_used_by_commit_titles() {
        assert_eq!(expand_emoji_shortcodes(":bug: fix crash"), "🐛 fix crash");
        assert_eq!(expand_emoji_shortcodes(":sparkles: add"), "✨ add");
        assert_eq!(expand_emoji_shortcodes(":memo: docs"), "📝 docs");
        assert_eq!(expand_emoji_shortcodes(":lipstick: style"), "💄 style");
        assert_eq!(expand_emoji_shortcodes(":recycle: refactor"), "♻️ refactor");
        assert_eq!(expand_emoji_shortcodes(":zap: faster"), "⚡ faster");
        assert_eq!(
            expand_emoji_shortcodes(":white_check_mark: test"),
            "✅ test"
        );
        assert_eq!(expand_emoji_shortcodes(":package: build"), "📦 build");
        assert_eq!(expand_emoji_shortcodes(":ferris_wheel: ci"), "🎡 ci");
        assert_eq!(expand_emoji_shortcodes(":hammer: chore"), "🔨 chore");
        assert_eq!(expand_emoji_shortcodes(":rewind: revert"), "⏪ revert");
    }

    #[test]
    fn leaves_unknown_text_and_urls_unchanged() {
        let plain = "fix crash without a shortcode";
        assert!(matches!(expand_emoji_shortcodes(plain), Cow::Borrowed(_)));
        assert_eq!(
            expand_emoji_shortcodes(":not_a_real_emoji_zzz: still here"),
            ":not_a_real_emoji_zzz: still here"
        );
        assert_eq!(
            expand_emoji_shortcodes("see https://example.com/docs"),
            "see https://example.com/docs"
        );
        assert_eq!(expand_emoji_shortcodes("at 10:30"), "at 10:30");
        assert_eq!(expand_emoji_shortcodes("::"), "::");
    }

    #[test]
    fn expands_adjacent_shortcodes() {
        assert_eq!(
            expand_emoji_shortcodes(":bug: then :sparkles:"),
            "🐛 then ✨"
        );
        assert_eq!(expand_emoji_shortcodes(":nope:bug:"), ":nope🐛");
    }
}
