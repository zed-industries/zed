use super::SharedString;

const MAX_HIDE_LEN: usize = 1024;

pub(super) fn skip_partial_links(source: SharedString) -> SharedString {
    let source_str = source.as_ref();
    let mut tail_start = source_str.len().saturating_sub(MAX_HIDE_LEN);
    while tail_start < source_str.len() && !source_str.is_char_boundary(tail_start) {
        tail_start += 1;
    }
    let Some(start) = incomplete_link_start_in_tail(source_str, tail_start) else {
        return source;
    };

    source_str[..start].to_string().into()
}

fn incomplete_link_start(text: &str) -> Option<usize> {
    start_for_open_destination(text).or_else(|| start_for_open_label(text))
}

fn label_start_with_image(text: &str, label_start: usize) -> usize {
    if label_start > 0 && text.as_bytes()[label_start - 1] == b'!' {
        label_start - 1
    } else {
        label_start
    }
}

fn start_for_open_destination(text: &str) -> Option<usize> {
    let mut search_end = text.len();
    while let Some(link_start) = text[..search_end].rfind("](") {
        if text[link_start + 2..].contains(')') {
            search_end = link_start;
            continue;
        }

        let label_start = text[..link_start].rfind('[').unwrap_or(link_start);
        return Some(label_start_with_image(text, label_start));
    }

    None
}

fn start_for_open_label(text: &str) -> Option<usize> {
    let label_start = text.rfind('[')?;
    let after_label = &text[label_start + 1..];

    if let Some(close_offset) = after_label.find(']') {
        let close_index = label_start + 1 + close_offset;
        if text[close_index + 1..].trim().is_empty() {
            return Some(label_start_with_image(text, label_start));
        }
        return None;
    }

    Some(label_start_with_image(text, label_start))
}

fn incomplete_link_start_in_tail(text: &str, tail_start: usize) -> Option<usize> {
    let tail_text = &text[tail_start..];
    let start_in_tail = incomplete_link_start(tail_text)?;
    let tail_has_label_start = tail_text[..start_in_tail].contains('[');
    if tail_text.as_bytes().get(start_in_tail) == Some(&b']') && !tail_has_label_start {
        // Avoid hiding when the label start is outside the tail.
        return None;
    }

    let mut start = tail_start + start_in_tail;
    if start > 0 && text.as_bytes()[start - 1] == b'!' {
        start -= 1;
    }
    let hidden_len = text.len().saturating_sub(start);
    if hidden_len > MAX_HIDE_LEN {
        return None;
    }

    Some(start)
}

#[cfg(test)]
mod tests {
    use super::{MAX_HIDE_LEN, SharedString, skip_partial_links};

    fn run(input: &str) -> String {
        skip_partial_links(SharedString::new(input.to_string()))
            .as_ref()
            .to_string()
    }

    #[test]
    fn leaves_plain_text_intact() {
        assert_eq!(run("hello world"), "hello world");
    }

    #[test]
    fn leaves_complete_link_intact() {
        assert_eq!(run("see [link](dest)"), "see [link](dest)");
    }

    #[test]
    fn trims_incomplete_destination() {
        assert_eq!(run("see [link](dest"), "see ");
    }

    #[test]
    fn trims_incomplete_label() {
        assert_eq!(run("see [link"), "see ");
    }

    #[test]
    fn trims_closed_label_without_destination() {
        assert_eq!(run("see [link]"), "see ");
    }

    #[test]
    fn trims_closed_label_with_only_whitespace_after() {
        assert_eq!(run("see [link]   "), "see ");
    }

    #[test]
    fn keeps_closed_label_with_trailing_text() {
        assert_eq!(run("see [link] text"), "see [link] text");
    }

    #[test]
    fn trims_incomplete_image_destination() {
        assert_eq!(run("see ![alt](dest"), "see ");
    }

    #[test]
    fn does_not_trim_when_label_starts_before_tail() {
        let label_text = "a".repeat(MAX_HIDE_LEN);
        let input_text = format!("[{label_text}](");
        assert_eq!(run(&input_text), input_text);
    }

    #[test]
    fn does_not_trim_when_image_prefix_is_outside_tail() {
        let label_text = "a".repeat(MAX_HIDE_LEN - 1);
        let input_text = format!("![{label_text}");
        assert_eq!(run(&input_text), input_text);
    }
}
