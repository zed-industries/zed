use cloud_llm_client::predict_edits_v3::Excerpt;
use edit_prediction_context::Line;
use language::{BufferSnapshot, Point};
use std::ops::Range;

pub fn merge_excerpts(
    buffer: &BufferSnapshot,
    sorted_line_ranges: impl IntoIterator<Item = Range<Line>>,
) -> Vec<Excerpt> {
    let mut output = Vec::new();
    let mut merged_ranges = Vec::<Range<Line>>::new();

    for line_range in sorted_line_ranges {
        if let Some(last_line_range) = merged_ranges.last_mut()
            && line_range.start <= last_line_range.end
        {
            last_line_range.end = last_line_range.end.max(line_range.end);
            continue;
        }
        merged_ranges.push(line_range);
    }

    let outline_items = buffer.outline_items_as_points_containing(0..buffer.len(), false, None);
    let mut outline_items = outline_items.into_iter().peekable();

    for range in merged_ranges {
        let point_range = Point::new(range.start.0, 0)..Point::new(range.end.0, 0);

        while let Some(outline_item) = outline_items.peek() {
            if outline_item.range.start >= point_range.start {
                break;
            }
            if outline_item.range.end > point_range.start {
                let mut point_range = outline_item.source_range_for_text.clone();
                point_range.start.column = 0;
                point_range.end.column = buffer.line_len(point_range.end.row);

                output.push(Excerpt {
                    start_line: Line(point_range.start.row),
                    text: buffer
                        .text_for_range(point_range.clone())
                        .collect::<String>()
                        .into(),
                })
            }
            outline_items.next();
        }

        output.push(Excerpt {
            start_line: Line(point_range.start.row),
            text: buffer
                .text_for_range(point_range.clone())
                .collect::<String>()
                .into(),
        })
    }

    output
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use cloud_llm_client::predict_edits_v3;
    use gpui::{TestAppContext, prelude::*};
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, OffsetRangeExt};
    use pretty_assertions::assert_eq;
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_rust(cx: &mut TestAppContext) {
        let table = [
            (
                indoc! {r#"
                    struct User {
                        first_name: String,
                    «    last_name: String,
                        ageˇ: u32,
                    »    email: String,
                        create_at: Instant,
                    }

                    impl User {
                        pub fn first_name(&self) -> String {
                            self.first_name.clone()
                        }

                        pub fn full_name(&self) -> String {
                    «        format!("{} {}", self.first_name, self.last_name)
                    »    }
                    }
                "#},
                indoc! {r#"
                    1|struct User {
                    …
                    3|    last_name: String,
                    4|    age<|cursor|>: u32,
                    …
                    9|impl User {
                    …
                    14|    pub fn full_name(&self) -> String {
                    15|        format!("{} {}", self.first_name, self.last_name)
                    …
                "#},
            ),
            (
                indoc! {r#"
                    struct User {
                        first_name: String,
                    «    last_name: String,
                        age: u32,
                    }
                    »"#
                },
                indoc! {r#"
                    1|struct User {
                    …
                    3|    last_name: String,
                    4|    age: u32,
                    5|}
                "#},
            ),
        ];

        for (input, expected_output) in table {
            let input_without_ranges = input.replace(['«', '»'], "");
            let input_without_caret = input.replace('ˇ', "");
            let cursor_offset = input_without_ranges.find('ˇ');
            let (input, ranges) = marked_text_ranges(&input_without_caret, false);
            let buffer =
                cx.new(|cx| Buffer::local(input, cx).with_language(Arc::new(rust_lang()), cx));
            buffer.read_with(cx, |buffer, _cx| {
                let insertions = cursor_offset
                    .map(|offset| {
                        let point = buffer.offset_to_point(offset);
                        vec![(
                            predict_edits_v3::Point {
                                line: Line(point.row),
                                column: point.column,
                            },
                            "<|cursor|>",
                        )]
                    })
                    .unwrap_or_default();
                let ranges: Vec<Range<Line>> = ranges
                    .into_iter()
                    .map(|range| {
                        let point_range = range.to_point(&buffer);
                        Line(point_range.start.row)..Line(point_range.end.row)
                    })
                    .collect();

                let mut output = String::new();
                cloud_zeta2_prompt::write_excerpts(
                    merge_excerpts(&buffer.snapshot(), ranges).iter(),
                    &insertions,
                    Line(buffer.max_point().row),
                    true,
                    &mut output,
                );
                assert_eq!(output, expected_output);
            });
        }
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(language::tree_sitter_rust::LANGUAGE.into()),
        )
        .with_outline_query(include_str!("../../languages/src/rust/outline.scm"))
        .unwrap()
    }
}
