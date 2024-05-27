use rope::Rope;
use std::{cmp::Ordering, ops::Range};

pub(crate) fn text_in_range_omitting_ranges(
    rope: &Rope,
    range: Range<usize>,
    omit_ranges: &[Range<usize>],
) -> String {
    let mut content = String::with_capacity(range.len());
    let mut omit_ranges = omit_ranges
        .iter()
        .skip_while(|omit_range| omit_range.end <= range.start)
        .peekable();
    let mut offset = range.start;
    let mut chunks = rope.chunks_in_range(range.clone());
    while let Some(chunk) = chunks.next() {
        if let Some(omit_range) = omit_ranges.peek() {
            match offset.cmp(&omit_range.start) {
                Ordering::Less => {
                    let max_len = omit_range.start - offset;
                    if chunk.len() < max_len {
                        content.push_str(chunk);
                        offset += chunk.len();
                    } else {
                        content.push_str(&chunk[..max_len]);
                        chunks.seek(omit_range.end.min(range.end));
                        offset = omit_range.end;
                        omit_ranges.next();
                    }
                }
                Ordering::Equal | Ordering::Greater => {
                    chunks.seek(omit_range.end.min(range.end));
                    offset = omit_range.end;
                    omit_ranges.next();
                }
            }
        } else {
            content.push_str(chunk);
            offset += chunk.len();
        }
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng as _};
    use util::RandomCharIter;

    #[gpui::test(iterations = 100)]
    fn test_text_in_range_omitting_ranges(mut rng: StdRng) {
        let text = RandomCharIter::new(&mut rng).take(1024).collect::<String>();
        let rope = Rope::from(text.as_str());

        let mut start = rng.gen_range(0..=text.len() / 2);
        let mut end = rng.gen_range(text.len() / 2..=text.len());
        while !text.is_char_boundary(start) {
            start -= 1;
        }
        while !text.is_char_boundary(end) {
            end += 1;
        }
        let range = start..end;

        let mut ix = 0;
        let mut omit_ranges = Vec::new();
        for _ in 0..rng.gen_range(0..10) {
            let mut start = rng.gen_range(ix..=text.len());
            while !text.is_char_boundary(start) {
                start += 1;
            }
            let mut end = rng.gen_range(start..=text.len());
            while !text.is_char_boundary(end) {
                end += 1;
            }
            omit_ranges.push(start..end);
            ix = end;
            if ix == text.len() {
                break;
            }
        }

        let mut expected_text = text[range.clone()].to_string();
        for omit_range in omit_ranges.iter().rev() {
            let start = omit_range
                .start
                .saturating_sub(range.start)
                .min(range.len());
            let end = omit_range.end.saturating_sub(range.start).min(range.len());
            expected_text.replace_range(start..end, "");
        }

        assert_eq!(
            text_in_range_omitting_ranges(&rope, range.clone(), &omit_ranges),
            expected_text,
            "text: {text:?}\nrange: {range:?}\nomit_ranges: {omit_ranges:?}"
        );
    }
}
