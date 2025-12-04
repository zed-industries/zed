use crate::RelatedExcerpt;
use language::{BufferSnapshot, OffsetRangeExt as _, Point};
use std::ops::Range;

#[cfg(not(test))]
const MAX_OUTLINE_ITEM_BODY_SIZE: usize = 512;
#[cfg(test)]
const MAX_OUTLINE_ITEM_BODY_SIZE: usize = 24;

pub fn assemble_excerpts(
    buffer: &BufferSnapshot,
    mut input_ranges: Vec<Range<Point>>,
) -> Vec<RelatedExcerpt> {
    merge_ranges(&mut input_ranges);

    let mut outline_ranges = Vec::new();
    let outline_items = buffer.outline_items_as_points_containing(0..buffer.len(), false, None);
    let mut outline_ix = 0;
    for input_range in &mut input_ranges {
        *input_range = clip_range_to_lines(input_range, false, buffer);

        while let Some(outline_item) = outline_items.get(outline_ix) {
            let item_range = clip_range_to_lines(&outline_item.range, false, buffer);

            if item_range.start > input_range.start {
                break;
            }

            if item_range.end > input_range.start {
                let body_range = outline_item
                    .body_range(buffer)
                    .map(|body| clip_range_to_lines(&body, true, buffer))
                    .filter(|body_range| {
                        body_range.to_offset(buffer).len() > MAX_OUTLINE_ITEM_BODY_SIZE
                    });

                add_outline_item(
                    item_range.clone(),
                    body_range.clone(),
                    buffer,
                    &mut outline_ranges,
                );

                if let Some(body_range) = body_range
                    && input_range.start < body_range.start
                {
                    let mut child_outline_ix = outline_ix + 1;
                    while let Some(next_outline_item) = outline_items.get(child_outline_ix) {
                        if next_outline_item.range.end > body_range.end {
                            break;
                        }
                        if next_outline_item.depth == outline_item.depth + 1 {
                            let next_item_range =
                                clip_range_to_lines(&next_outline_item.range, false, buffer);

                            add_outline_item(
                                next_item_range,
                                next_outline_item
                                    .body_range(buffer)
                                    .map(|body| clip_range_to_lines(&body, true, buffer)),
                                buffer,
                                &mut outline_ranges,
                            );
                            child_outline_ix += 1;
                        }
                    }
                }
            }

            outline_ix += 1;
        }
    }

    input_ranges.extend_from_slice(&outline_ranges);
    merge_ranges(&mut input_ranges);

    input_ranges
        .into_iter()
        .map(|range| {
            let offset_range = range.to_offset(buffer);
            RelatedExcerpt {
                point_range: range,
                anchor_range: buffer.anchor_before(offset_range.start)
                    ..buffer.anchor_after(offset_range.end),
                text: buffer.as_rope().slice(offset_range),
            }
        })
        .collect()
}

fn clip_range_to_lines(
    range: &Range<Point>,
    inward: bool,
    buffer: &BufferSnapshot,
) -> Range<Point> {
    let mut range = range.clone();
    if inward {
        if range.start.column > 0 {
            range.start.column = buffer.line_len(range.start.row);
        }
        range.end.column = 0;
    } else {
        range.start.column = 0;
        if range.end.column > 0 {
            range.end.column = buffer.line_len(range.end.row);
        }
    }
    range
}

fn add_outline_item(
    mut item_range: Range<Point>,
    body_range: Option<Range<Point>>,
    buffer: &BufferSnapshot,
    outline_ranges: &mut Vec<Range<Point>>,
) {
    if let Some(mut body_range) = body_range {
        if body_range.start.column > 0 {
            body_range.start.column = buffer.line_len(body_range.start.row);
        }
        body_range.end.column = 0;

        let head_range = item_range.start..body_range.start;
        if head_range.start < head_range.end {
            outline_ranges.push(head_range);
        }

        let tail_range = body_range.end..item_range.end;
        if tail_range.start < tail_range.end {
            outline_ranges.push(tail_range);
        }
    } else {
        item_range.start.column = 0;
        item_range.end.column = buffer.line_len(item_range.end.row);
        outline_ranges.push(item_range);
    }
}

pub fn merge_ranges(ranges: &mut Vec<Range<Point>>) {
    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));

    let mut index = 1;
    while index < ranges.len() {
        let mut prev_range_end = ranges[index - 1].end;
        if prev_range_end.column > 0 {
            prev_range_end += Point::new(1, 0);
        }

        if (prev_range_end + Point::new(1, 0))
            .cmp(&ranges[index].start)
            .is_ge()
        {
            let removed = ranges.remove(index);
            if removed.end.cmp(&ranges[index - 1].end).is_gt() {
                ranges[index - 1].end = removed.end;
            }
        } else {
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, OffsetRangeExt};
    use pretty_assertions::assert_eq;
    use std::{fmt::Write as _, sync::Arc};
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_rust(cx: &mut TestAppContext) {
        let table = [
            (
                indoc! {r#"
                    struct User {
                        first_name: String,
                        «last_name»: String,
                        age: u32,
                        email: String,
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
                    struct User {
                        first_name: String,
                        last_name: String,
                    …
                    }

                    impl User {
                    …
                        pub fn full_name(&self) -> String {
                            format!("{} {}", self.first_name, self.last_name)
                        }
                    }
                "#},
            ),
            (
                indoc! {r#"
                    struct «User» {
                        first_name: String,
                        last_name: String,
                        age: u32,
                    }

                    impl User {
                        // methods
                    }
                    "#
                },
                indoc! {r#"
                    struct User {
                        first_name: String,
                        last_name: String,
                        age: u32,
                    }
                    …
                "#},
            ),
            (
                indoc! {r#"
                    trait «FooProvider» {
                        const NAME: &'static str;

                        fn provide_foo(&self, id: usize) -> Foo;

                        fn provide_foo_batched(&self, ids: &[usize]) -> Vec<Foo> {
                             ids.iter()
                                .map(|id| self.provide_foo(*id))
                                .collect()
                        }

                        fn sync(&self);
                    }
                    "#
                },
                indoc! {r#"
                    trait FooProvider {
                        const NAME: &'static str;

                        fn provide_foo(&self, id: usize) -> Foo;

                        fn provide_foo_batched(&self, ids: &[usize]) -> Vec<Foo> {
                    …
                        }

                        fn sync(&self);
                    }
                "#},
            ),
        ];

        for (input, expected_output) in table {
            let (input, ranges) = marked_text_ranges(&input, false);
            let buffer =
                cx.new(|cx| Buffer::local(input, cx).with_language(Arc::new(rust_lang()), cx));
            buffer.read_with(cx, |buffer, _cx| {
                let ranges: Vec<Range<Point>> = ranges
                    .into_iter()
                    .map(|range| range.to_point(&buffer))
                    .collect();

                let excerpts = assemble_excerpts(&buffer.snapshot(), ranges);

                let output = format_excerpts(buffer, &excerpts);
                assert_eq!(output, expected_output);
            });
        }
    }

    fn format_excerpts(buffer: &Buffer, excerpts: &[RelatedExcerpt]) -> String {
        let mut output = String::new();
        let file_line_count = buffer.max_point().row;
        let mut current_row = 0;
        for excerpt in excerpts {
            if excerpt.text.is_empty() {
                continue;
            }
            if current_row < excerpt.point_range.start.row {
                writeln!(&mut output, "…").unwrap();
            }
            current_row = excerpt.point_range.start.row;

            for line in excerpt.text.to_string().lines() {
                output.push_str(line);
                output.push('\n');
                current_row += 1;
            }
        }
        if current_row < file_line_count {
            writeln!(&mut output, "…").unwrap();
        }
        output
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
