use language::{BufferSnapshot, Diff, Point, ToOffset};
use project::search::SearchQuery;
use std::iter;
use util::{ResultExt as _, paths::PathMatcher};

/// Performs an exact string replacement in a buffer, requiring precise character-for-character matching.
/// Uses the search functionality to locate the first occurrence of the exact string.
/// Returns None if no exact match is found in the buffer.
pub async fn replace_exact(old: &str, new: &str, snapshot: &BufferSnapshot) -> Option<Diff> {
    let query = SearchQuery::text(
        old,
        false,
        true,
        true,
        PathMatcher::new(iter::empty::<&str>()).ok()?,
        PathMatcher::new(iter::empty::<&str>()).ok()?,
        false,
        None,
    )
    .log_err()?;

    let matches = query.search(&snapshot, None).await;

    if matches.is_empty() {
        return None;
    }

    let edit_range = matches[0].clone();
    let diff = language::text_diff(&old, &new);

    let edits = diff
        .into_iter()
        .map(|(old_range, text)| {
            let start = edit_range.start + old_range.start;
            let end = edit_range.start + old_range.end;
            (start..end, text)
        })
        .collect::<Vec<_>>();

    let diff = language::Diff {
        base_version: snapshot.version().clone(),
        line_ending: snapshot.line_ending(),
        edits,
    };

    Some(diff)
}

/// Performs a replacement that's indentation-aware - matches text content ignoring leading whitespace differences.
/// When replacing, preserves the indentation level found in the buffer at each matching line.
/// Returns None if no match found or if indentation is offset inconsistently across matched lines.
pub fn replace_with_flexible_indent(old: &str, new: &str, buffer: &BufferSnapshot) -> Option<Diff> {
    let (old_lines, old_min_indent) = lines_with_min_indent(old);
    let (new_lines, new_min_indent) = lines_with_min_indent(new);
    let min_indent = old_min_indent.min(new_min_indent);

    let old_lines = drop_lines_prefix(&old_lines, min_indent);
    let new_lines = drop_lines_prefix(&new_lines, min_indent);

    let max_row = buffer.max_point().row;

    'windows: for start_row in 0..max_row.saturating_sub(old_lines.len() as u32 - 1) {
        let mut common_leading = None;

        let end_row = start_row + old_lines.len() as u32 - 1;

        if end_row > max_row {
            // The buffer ends before fully matching the pattern
            return None;
        }

        let start_point = Point::new(start_row, 0);
        let end_point = Point::new(end_row, buffer.line_len(end_row));
        let range = start_point.to_offset(buffer)..end_point.to_offset(buffer);

        let window_text = buffer.text_for_range(range.clone());
        let mut window_lines = window_text.lines();
        let mut old_lines_iter = old_lines.iter();

        while let (Some(window_line), Some(old_line)) = (window_lines.next(), old_lines_iter.next())
        {
            let line_trimmed = window_line.trim_start();

            if line_trimmed != old_line.trim_start() {
                continue 'windows;
            }

            if line_trimmed.is_empty() {
                continue;
            }

            let line_leading = &window_line[..window_line.len() - old_line.len()];

            match &common_leading {
                Some(common_leading) if common_leading != line_leading => {
                    continue 'windows;
                }
                Some(_) => (),
                None => common_leading = Some(line_leading.to_string()),
            }
        }

        if let Some(common_leading) = common_leading {
            let line_ending = buffer.line_ending();
            let replacement = new_lines
                .iter()
                .map(|new_line| {
                    if new_line.trim().is_empty() {
                        new_line.to_string()
                    } else {
                        common_leading.to_string() + new_line
                    }
                })
                .collect::<Vec<_>>()
                .join(line_ending.as_str());

            let diff = Diff {
                base_version: buffer.version().clone(),
                line_ending,
                edits: vec![(range, replacement.into())],
            };

            return Some(diff);
        }
    }

    None
}

fn drop_lines_prefix<'a>(lines: &'a [&str], prefix_len: usize) -> Vec<&'a str> {
    lines
        .iter()
        .map(|line| line.get(prefix_len..).unwrap_or(""))
        .collect()
}

fn lines_with_min_indent(input: &str) -> (Vec<&str>, usize) {
    let mut lines = Vec::new();
    let mut min_indent: Option<usize> = None;

    for line in input.lines() {
        lines.push(line);
        if !line.trim().is_empty() {
            let indent = line.len() - line.trim_start().len();
            min_indent = Some(min_indent.map_or(indent, |m| m.min(indent)));
        }
    }

    (lines, min_indent.unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui::prelude::*;
    use unindent::Unindent;

    #[gpui::test]
    fn test_replace_consistent_indentation(cx: &mut TestAppContext) {
        let whole = r#"
            fn test() {
                let x = 5;
                println!("x = {}", x);
                let y = 10;
            }
        "#
        .unindent();

        let old = r#"
            let x = 5;
            println!("x = {}", x);
        "#
        .unindent();

        let new = r#"
            let x = 42;
            println!("New value: {}", x);
        "#
        .unindent();

        let expected = r#"
            fn test() {
                let x = 42;
                println!("New value: {}", x);
                let y = 10;
            }
        "#
        .unindent();

        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            Some(expected.to_string())
        );
    }

    #[gpui::test]
    fn test_replace_inconsistent_indentation(cx: &mut TestAppContext) {
        let whole = r#"
            fn test() {
                if condition {
                    println!("{}", 43);
                }
            }
        "#
        .unindent();

        let old = r#"
            if condition {
            println!("{}", 43);
        "#
        .unindent();

        let new = r#"
            if condition {
            println!("{}", 42);
        "#
        .unindent();

        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            None
        );
    }

    #[gpui::test]
    fn test_replace_with_empty_lines(cx: &mut TestAppContext) {
        // Test with empty lines
        let whole = r#"
            fn test() {
                let x = 5;

                println!("x = {}", x);
            }
        "#
        .unindent();

        let old = r#"
            let x = 5;

            println!("x = {}", x);
        "#
        .unindent();

        let new = r#"
            let x = 10;

            println!("New x: {}", x);
        "#
        .unindent();

        let expected = r#"
            fn test() {
                let x = 10;

                println!("New x: {}", x);
            }
        "#
        .unindent();

        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            Some(expected.to_string())
        );
    }

    #[gpui::test]
    fn test_replace_no_match(cx: &mut TestAppContext) {
        // Test with no match
        let whole = r#"
            fn test() {
                let x = 5;
            }
        "#
        .unindent();

        let old = r#"
            let y = 10;
        "#
        .unindent();

        let new = r#"
            let y = 20;
        "#
        .unindent();

        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            None
        );
    }

    #[gpui::test]
    fn test_replace_whole_ends_before_matching_old(cx: &mut TestAppContext) {
        let whole = r#"
            fn test() {
                let x = 5;
        "#
        .unindent();

        let old = r#"
            let x = 5;
            println!("x = {}", x);
        "#
        .unindent();

        let new = r#"
            let x = 10;
            println!("x = {}", x);
        "#
        .unindent();

        // Should return None because whole doesn't fully contain the old text
        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            None
        );
    }

    #[test]
    fn test_lines_with_min_indent() {
        // Empty string
        assert_eq!(lines_with_min_indent(""), (vec![], 0));

        // Single line without indentation
        assert_eq!(lines_with_min_indent("hello"), (vec!["hello"], 0));

        // Multiple lines with no indentation
        assert_eq!(
            lines_with_min_indent("line1\nline2\nline3"),
            (vec!["line1", "line2", "line3"], 0)
        );

        // Multiple lines with consistent indentation
        assert_eq!(
            lines_with_min_indent("  line1\n  line2\n  line3"),
            (vec!["  line1", "  line2", "  line3"], 2)
        );

        // Multiple lines with varying indentation
        assert_eq!(
            lines_with_min_indent("    line1\n  line2\n      line3"),
            (vec!["    line1", "  line2", "      line3"], 2)
        );

        // Lines with mixed indentation and empty lines
        assert_eq!(
            lines_with_min_indent("    line1\n\n  line2"),
            (vec!["    line1", "", "  line2"], 2)
        );
    }

    #[gpui::test]
    fn test_replace_with_missing_indent_uneven_match(cx: &mut TestAppContext) {
        let whole = r#"
            fn test() {
                if true {
                        let x = 5;
                        println!("x = {}", x);
                }
            }
        "#
        .unindent();

        let old = r#"
            let x = 5;
            println!("x = {}", x);
        "#
        .unindent();

        let new = r#"
            let x = 42;
            println!("x = {}", x);
        "#
        .unindent();

        let expected = r#"
            fn test() {
                if true {
                        let x = 42;
                        println!("x = {}", x);
                }
            }
        "#
        .unindent();

        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            Some(expected.to_string())
        );
    }

    #[gpui::test]
    fn test_replace_big_example(cx: &mut TestAppContext) {
        let whole = r#"
            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn test_is_valid_age() {
                    assert!(is_valid_age(0));
                    assert!(!is_valid_age(151));
                }
            }
        "#
        .unindent();

        let old = r#"
            #[test]
            fn test_is_valid_age() {
                assert!(is_valid_age(0));
                assert!(!is_valid_age(151));
            }
        "#
        .unindent();

        let new = r#"
            #[test]
            fn test_is_valid_age() {
                assert!(is_valid_age(0));
                assert!(!is_valid_age(151));
            }

            #[test]
            fn test_group_people_by_age() {
                let people = vec![
                    Person::new("Young One", 5, "young@example.com").unwrap(),
                    Person::new("Teen One", 15, "teen@example.com").unwrap(),
                    Person::new("Teen Two", 18, "teen2@example.com").unwrap(),
                    Person::new("Adult One", 25, "adult@example.com").unwrap(),
                ];

                let groups = group_people_by_age(&people);

                assert_eq!(groups.get(&0).unwrap().len(), 1);  // One person in 0-9
                assert_eq!(groups.get(&10).unwrap().len(), 2); // Two people in 10-19
                assert_eq!(groups.get(&20).unwrap().len(), 1); // One person in 20-29
            }
        "#
        .unindent();
        let expected = r#"
            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn test_is_valid_age() {
                    assert!(is_valid_age(0));
                    assert!(!is_valid_age(151));
                }

                #[test]
                fn test_group_people_by_age() {
                    let people = vec![
                        Person::new("Young One", 5, "young@example.com").unwrap(),
                        Person::new("Teen One", 15, "teen@example.com").unwrap(),
                        Person::new("Teen Two", 18, "teen2@example.com").unwrap(),
                        Person::new("Adult One", 25, "adult@example.com").unwrap(),
                    ];

                    let groups = group_people_by_age(&people);

                    assert_eq!(groups.get(&0).unwrap().len(), 1);  // One person in 0-9
                    assert_eq!(groups.get(&10).unwrap().len(), 2); // Two people in 10-19
                    assert_eq!(groups.get(&20).unwrap().len(), 1); // One person in 20-29
                }
            }
        "#
        .unindent();
        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            Some(expected.to_string())
        );
    }

    #[test]
    fn test_drop_lines_prefix() {
        // Empty array
        assert_eq!(drop_lines_prefix(&[], 2), Vec::<&str>::new());

        // Zero prefix length
        assert_eq!(
            drop_lines_prefix(&["line1", "line2"], 0),
            vec!["line1", "line2"]
        );

        // Normal prefix drop
        assert_eq!(
            drop_lines_prefix(&["  line1", "  line2"], 2),
            vec!["line1", "line2"]
        );

        // Prefix longer than some lines
        assert_eq!(drop_lines_prefix(&["  line1", "a"], 2), vec!["line1", ""]);

        // Prefix longer than all lines
        assert_eq!(drop_lines_prefix(&["a", "b"], 5), vec!["", ""]);

        // Mixed length lines
        assert_eq!(
            drop_lines_prefix(&["    line1", "  line2", "      line3"], 2),
            vec!["  line1", "line2", "    line3"]
        );
    }

    fn test_replace_with_flexible_indent(
        cx: &mut TestAppContext,
        whole: &str,
        old: &str,
        new: &str,
    ) -> Option<String> {
        // Create a local buffer with the test content
        let buffer = cx.new(|cx| language::Buffer::local(whole, cx));

        // Get the buffer snapshot
        let buffer_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        // Call replace_flexible and transform the result
        replace_with_flexible_indent(old, new, &buffer_snapshot).map(|diff| {
            buffer.update(cx, |buffer, cx| {
                let _ = buffer.apply_diff(diff, cx);
                buffer.text()
            })
        })
    }
}
