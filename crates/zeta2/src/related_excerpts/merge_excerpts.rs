use edit_prediction_context::Line;
use language::{Bias, BufferSnapshot, Point};
use std::{fmt::Write, ops::Range};

pub fn write_merged_excerpts(
    buffer: &BufferSnapshot,
    sorted_line_ranges: impl IntoIterator<Item = Range<Line>>,
    merged: &mut String,
) {
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

    let mut position = Point::new(0, 0);
    for range in merged_ranges {
        let point_range = Point::new(range.start.0, 0)..Point::new(range.end.0, 0);

        while let Some(outline_item) = outline_items.peek() {
            if outline_item.range.start >= point_range.start {
                break;
            }
            if outline_item.range.end > point_range.start {
                let mut point_range = outline_item.source_range_for_text.clone();
                point_range.start.column = 0;
                if point_range.end.column != 0 {
                    point_range.end.row += 1;
                    point_range.end.column = 0;
                }

                write_numbered_lines(point_range, buffer, merged, &mut position);
            }
            outline_items.next();
        }

        write_numbered_lines(point_range, buffer, merged, &mut position);
    }

    write_numbered_lines(
        buffer.max_point()..buffer.max_point(),
        buffer,
        merged,
        &mut position,
    );
}

fn write_numbered_lines(
    range: Range<Point>,
    buffer: &BufferSnapshot,
    text: &mut String,
    position: &mut Point,
) {
    if range.start > *position {
        writeln!(text, "…").unwrap();
    }
    if range.is_empty() {
        return;
    }
    let mut range = range.start.max(*position)..range.end;
    *position = range.end;
    if range.end.column == 0 && range.end.row > range.start.row {
        range.end = Point::new(range.end.row - 1, u32::MAX);
    }
    let range =
        buffer.clip_point(range.start, Bias::Left)..buffer.clip_point(range.end, Bias::Right);
    let mut lines = buffer.text_for_range(range.clone()).lines();
    let mut line_number = range.start.row;
    while let Some(line) = lines.next() {
        line_number += 1;
        writeln!(text, "{line_number}|{line}").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
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
                        age: u32,
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
                    4|    age: u32,
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
            let (input, ranges) = marked_text_ranges(input, false);
            let buffer =
                cx.new(|cx| Buffer::local(input, cx).with_language(Arc::new(rust_lang()), cx));
            buffer.read_with(cx, |buffer, _cx| {
                let ranges: Vec<Range<Line>> = ranges
                    .into_iter()
                    .map(|range| {
                        let point_range = range.to_point(&buffer);
                        Line(point_range.start.row)..Line(point_range.end.row)
                    })
                    .collect();

                let mut output = String::new();
                write_merged_excerpts(&buffer.snapshot(), ranges, &mut output);
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
        .with_outline_query(include_str!("../../../languages/src/rust/outline.scm"))
        .unwrap()
    }
}
