use imara_diff::{
    diff,
    intern::{InternedInput, Token},
    sources::lines_with_terminator,
    Algorithm,
};
use std::{iter, ops::Range, sync::Arc};

pub fn text_diff(old_text: &str, new_text: &str) -> Vec<(Range<usize>, Arc<str>)> {
    let empty: Arc<str> = Arc::default();
    let mut edits = Vec::new();
    let mut hunk_input = InternedInput::default();
    let input = InternedInput::new(
        lines_with_terminator(old_text),
        lines_with_terminator(new_text),
    );

    let mut old_offset = 0;
    let mut new_offset = 0;
    let mut old_row = 0;
    let mut new_row = 0;
    diff(
        Algorithm::Histogram,
        &input,
        |old_rows: Range<u32>, new_rows: Range<u32>| {
            let old_rows = old_rows.start as usize..old_rows.end as usize;
            let new_rows = new_rows.start as usize..new_rows.end as usize;

            old_offset += token_len(&input, &input.before[old_row..old_rows.start]);
            new_offset += token_len(&input, &input.after[new_row..new_rows.start]);
            let old_len = token_len(&input, &input.before[old_rows.start..old_rows.end]);
            let new_len = token_len(&input, &input.after[new_rows.start..new_rows.end]);
            old_row = old_rows.end;
            new_row = new_rows.end;

            let old_byte_range = old_offset..old_offset + old_len;
            let new_byte_range = new_offset..new_offset + new_len;

            if should_perform_word_diff_within_hunk(
                &old_rows,
                &old_byte_range,
                &new_rows,
                &new_byte_range,
            ) {
                let mut old_offset = old_offset;
                let mut new_offset = new_offset;
                let mut old_token = 0;
                let mut new_token = 0;
                let input = &mut hunk_input;

                input.clear();
                input.update_before(words(&old_text[old_byte_range.clone()]));
                input.update_after(words(&new_text[new_byte_range.clone()]));
                diff(
                    Algorithm::Histogram,
                    input,
                    |old_tokens: Range<u32>, new_tokens: Range<u32>| {
                        let old_tokens = old_tokens.start as usize..old_tokens.end as usize;
                        let new_tokens = new_tokens.start as usize..new_tokens.end as usize;
                        old_offset += token_len(&input, &input.before[old_token..old_tokens.start]);
                        new_offset += token_len(&input, &input.after[new_token..new_tokens.start]);
                        let old_len =
                            token_len(&input, &input.before[old_tokens.start..old_tokens.end]);
                        let new_len =
                            token_len(&input, &input.after[new_tokens.start..new_tokens.end]);
                        old_token = old_tokens.end;
                        new_token = new_tokens.end;

                        let old_byte_range = old_offset..old_offset + old_len;
                        let new_byte_range = new_offset..new_offset + new_len;

                        let replacement_text = if new_byte_range.is_empty() {
                            empty.clone()
                        } else {
                            new_text[new_byte_range.clone()].into()
                        };
                        edits.push((old_byte_range, replacement_text));
                        old_offset += old_len;
                        new_offset += new_len;
                    },
                );
            } else {
                let replacement_text = if new_byte_range.is_empty() {
                    empty.clone()
                } else {
                    new_text[new_byte_range.clone()].into()
                };
                edits.push((old_byte_range.clone(), replacement_text));
            }

            old_offset = old_byte_range.end;
            new_offset = new_byte_range.end;
        },
    );

    edits
}

const MAX_WORD_DIFF_LEN: usize = 512;

fn should_perform_word_diff_within_hunk(
    old_rows: &Range<usize>,
    old_byte_range: &Range<usize>,
    new_rows: &Range<usize>,
    new_byte_range: &Range<usize>,
) -> bool {
    old_byte_range.len() < MAX_WORD_DIFF_LEN
        && new_byte_range.len() < MAX_WORD_DIFF_LEN
        && old_rows.len() < 5
        && new_rows.len() < 5
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    let mut ix = 0;
    let mut in_whitespace = text.chars().next().map_or(false, |c| c.is_whitespace());
    iter::from_fn(move || {
        if ix == text.len() {
            return None;
        }
        let next_ix = text[ix..]
            .find(|c: char| c.is_whitespace() != in_whitespace)
            .map_or(text.len(), |i| i + ix);
        in_whitespace = !in_whitespace;
        let token = &text[ix..next_ix];
        ix = next_ix;
        Some(token)
    })
}

fn token_len(input: &InternedInput<&str>, tokens: &[Token]) -> usize {
    tokens
        .iter()
        .map(|token| input.interner[*token].len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words() {
        let text = "";
        assert_eq!(words(text).collect::<Vec<_>>(), Vec::<&str>::new());

        let text = " ";
        assert_eq!(words(text).collect::<Vec<_>>(), vec![" "]);

        let text = "one";
        assert_eq!(words(text).collect::<Vec<_>>(), vec!["one"]);

        let text = "one\n";
        assert_eq!(words(text).collect::<Vec<_>>(), vec!["one", "\n"]);

        let text = "one two three";
        assert_eq!(
            words(text).collect::<Vec<_>>(),
            vec!["one", " ", "two", " ", "three"]
        );

        let text = "   one\n two three";
        assert_eq!(
            words(text).collect::<Vec<_>>(),
            vec!["   ", "one", "\n ", "two", " ", "three"]
        );
    }

    #[test]
    fn test_text_diff() {
        let old_text = "one two three";
        let new_text = "one TWO three";
        assert_eq!(text_diff(old_text, new_text), [(4..7, "TWO".into()),]);

        let old_text = "one\ntwo\nthree\n";
        let new_text = "one\ntwo\nAND\nTHEN\nthree\n";
        assert_eq!(
            text_diff(old_text, new_text),
            [(8..8, "AND\nTHEN\n".into()),]
        );

        let old_text = "one two\nthree four five\nsix seven eight nine\nten\n";
        let new_text = "one two\nthree FOUR five\nsix SEVEN eight nine\nten\nELEVEN\n";
        assert_eq!(
            text_diff(old_text, new_text),
            [
                (14..18, "FOUR".into()),
                (28..33, "SEVEN".into()),
                (49..49, "ELEVEN\n".into())
            ]
        );
    }
}
