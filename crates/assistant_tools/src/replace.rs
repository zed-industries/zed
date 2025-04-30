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

    'windows: for start_row in 0..max_row + 1 {
        let end_row = start_row + old_lines.len().saturating_sub(1) as u32;

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

        let mut common_mismatch = None;

        #[derive(Eq, PartialEq)]
        enum Mismatch {
            OverIndented(String),
            UnderIndented(String),
        }

        while let (Some(window_line), Some(old_line)) = (window_lines.next(), old_lines_iter.next())
        {
            let line_trimmed = window_line.trim_start();

            if line_trimmed != old_line.trim_start() {
                continue 'windows;
            }

            if line_trimmed.is_empty() {
                continue;
            }

            let line_mismatch = if window_line.len() > old_line.len() {
                let prefix = window_line[..window_line.len() - old_line.len()].to_string();
                Mismatch::UnderIndented(prefix)
            } else {
                let prefix = old_line[..old_line.len() - window_line.len()].to_string();
                Mismatch::OverIndented(prefix)
            };

            match &common_mismatch {
                Some(common_mismatch) if common_mismatch != &line_mismatch => {
                    continue 'windows;
                }
                Some(_) => (),
                None => common_mismatch = Some(line_mismatch),
            }
        }

        if let Some(common_mismatch) = &common_mismatch {
            let line_ending = buffer.line_ending();
            let replacement = new_lines
                .iter()
                .map(|new_line| {
                    if new_line.trim().is_empty() {
                        new_line.to_string()
                    } else {
                        match common_mismatch {
                            Mismatch::UnderIndented(prefix) => prefix.to_string() + new_line,
                            Mismatch::OverIndented(prefix) => new_line
                                .strip_prefix(prefix)
                                .unwrap_or(new_line)
                                .to_string(),
                        }
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
mod replace_exact_tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui::prelude::*;

    #[gpui::test]
    async fn basic(cx: &mut TestAppContext) {
        let result = test_replace_exact(cx, "let x = 41;", "let x = 41;", "let x = 42;").await;
        assert_eq!(result, Some("let x = 42;".to_string()));
    }

    #[gpui::test]
    async fn no_match(cx: &mut TestAppContext) {
        let result = test_replace_exact(cx, "let x = 41;", "let y = 42;", "let y = 43;").await;
        assert_eq!(result, None);
    }

    #[gpui::test]
    async fn multi_line(cx: &mut TestAppContext) {
        let whole = "fn example() {\n    let x = 41;\n    println!(\"x = {}\", x);\n}";
        let old_text = "    let x = 41;\n    println!(\"x = {}\", x);";
        let new_text = "    let x = 42;\n    println!(\"x = {}\", x);";
        let result = test_replace_exact(cx, whole, old_text, new_text).await;
        assert_eq!(
            result,
            Some("fn example() {\n    let x = 42;\n    println!(\"x = {}\", x);\n}".to_string())
        );
    }

    #[gpui::test]
    async fn multiple_occurrences(cx: &mut TestAppContext) {
        let whole = "let x = 41;\nlet y = 41;\nlet z = 41;";
        let result = test_replace_exact(cx, whole, "let x = 41;", "let x = 42;").await;
        assert_eq!(
            result,
            Some("let x = 42;\nlet y = 41;\nlet z = 41;".to_string())
        );
    }

    #[gpui::test]
    async fn empty_buffer(cx: &mut TestAppContext) {
        let result = test_replace_exact(cx, "", "let x = 41;", "let x = 42;").await;
        assert_eq!(result, None);
    }

    #[gpui::test]
    async fn partial_match(cx: &mut TestAppContext) {
        let whole = "let x = 41; let y = 42;";
        let result = test_replace_exact(cx, whole, "let x = 41", "let x = 42").await;
        assert_eq!(result, Some("let x = 42; let y = 42;".to_string()));
    }

    #[gpui::test]
    async fn whitespace_sensitive(cx: &mut TestAppContext) {
        let result = test_replace_exact(cx, "let x = 41;", " let x = 41;", "let x = 42;").await;
        assert_eq!(result, None);
    }

    #[gpui::test]
    async fn entire_buffer(cx: &mut TestAppContext) {
        let result = test_replace_exact(cx, "let x = 41;", "let x = 41;", "let x = 42;").await;
        assert_eq!(result, Some("let x = 42;".to_string()));
    }

    async fn test_replace_exact(
        cx: &mut TestAppContext,
        whole: &str,
        old: &str,
        new: &str,
    ) -> Option<String> {
        let buffer = cx.new(|cx| language::Buffer::local(whole, cx));

        let buffer_snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let diff = replace_exact(old, new, &buffer_snapshot).await;
        diff.map(|diff| {
            buffer.update(cx, |buffer, cx| {
                let _ = buffer.apply_diff(diff, cx);
                buffer.text()
            })
        })
    }
}

#[cfg(test)]
mod flexible_indent_tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui::prelude::*;
    use unindent::Unindent;

    #[gpui::test]
    fn test_underindented_single_line(cx: &mut TestAppContext) {
        let cur = "        let a = 41;".to_string();
        let old = "    let a = 41;".to_string();
        let new = "    let a = 42;".to_string();
        let exp = "        let a = 42;".to_string();

        let result = test_replace_with_flexible_indent(cx, &cur, &old, &new);

        assert_eq!(result, Some(exp.to_string()))
    }

    #[gpui::test]
    fn test_overindented_single_line(cx: &mut TestAppContext) {
        let cur = "    let a = 41;".to_string();
        let old = "        let a = 41;".to_string();
        let new = "        let a = 42;".to_string();
        let exp = "    let a = 42;".to_string();

        let result = test_replace_with_flexible_indent(cx, &cur, &old, &new);

        assert_eq!(result, Some(exp.to_string()))
    }

    #[gpui::test]
    fn test_underindented_multi_line(cx: &mut TestAppContext) {
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
    fn test_overindented_multi_line(cx: &mut TestAppContext) {
        let cur = r#"
            fn foo() {
                let a = 41;
                let b = 3.13;
            }
        "#
        .unindent();

        // 6 space indent instead of 4
        let old = "      let a = 41;\n      let b = 3.13;";
        let new = "      let a = 42;\n      let b = 3.14;";

        let expected = r#"
            fn foo() {
                let a = 42;
                let b = 3.14;
            }
        "#
        .unindent();

        let result = test_replace_with_flexible_indent(cx, &cur, &old, &new);

        assert_eq!(result, Some(expected.to_string()))
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

    #[gpui::test]
    fn test_replace_whole_is_shorter_than_old(cx: &mut TestAppContext) {
        let whole = r#"
            let x = 5;
        "#
        .unindent();

        let old = r#"
            let x = 5;
            let y = 10;
        "#
        .unindent();

        let new = r#"
            let x = 5;
            let y = 20;
        "#
        .unindent();

        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            None
        );
    }

    #[gpui::test]
    fn test_replace_old_is_empty(cx: &mut TestAppContext) {
        let whole = r#"
            fn test() {
                let x = 5;
            }
        "#
        .unindent();

        let old = "";
        let new = r#"
            let y = 10;
        "#
        .unindent();

        assert_eq!(
            test_replace_with_flexible_indent(cx, &whole, &old, &new),
            None
        );
    }

    #[gpui::test]
    fn test_replace_whole_is_empty(cx: &mut TestAppContext) {
        let whole = "";
        let old = r#"
            let x = 5;
        "#
        .unindent();

        let new = r#"
            let x = 10;
        "#
        .unindent();

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

    #[gpui::test]
    async fn test_replace_exact_basic(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| language::Buffer::local("let x = 41;", cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let diff = replace_exact("let x = 41;", "let x = 42;", &snapshot).await;
        assert!(diff.is_some());

        let diff = diff.unwrap();
        assert_eq!(diff.edits.len(), 1);

        let result = buffer.update(cx, |buffer, cx| {
            let _ = buffer.apply_diff(diff, cx);
            buffer.text()
        });

        assert_eq!(result, "let x = 42;");
    }

    #[gpui::test]
    async fn test_replace_exact_no_match(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| language::Buffer::local("let x = 41;", cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let diff = replace_exact("let y = 42;", "let y = 43;", &snapshot).await;
        assert!(diff.is_none());
    }

    #[gpui::test]
    async fn test_replace_exact_multi_line(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| {
            language::Buffer::local(
                "fn example() {\n    let x = 41;\n    println!(\"x = {}\", x);\n}",
                cx,
            )
        });
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let old_text = "    let x = 41;\n    println!(\"x = {}\", x);";
        let new_text = "    let x = 42;\n    println!(\"x = {}\", x);";
        let diff = replace_exact(old_text, new_text, &snapshot).await;
        assert!(diff.is_some());

        let diff = diff.unwrap();
        let result = buffer.update(cx, |buffer, cx| {
            let _ = buffer.apply_diff(diff, cx);
            buffer.text()
        });

        assert_eq!(
            result,
            "fn example() {\n    let x = 42;\n    println!(\"x = {}\", x);\n}"
        );
    }

    #[gpui::test]
    async fn test_replace_exact_multiple_occurrences(cx: &mut TestAppContext) {
        let buffer =
            cx.new(|cx| language::Buffer::local("let x = 41;\nlet y = 41;\nlet z = 41;", cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        // Should replace only the first occurrence
        let diff = replace_exact("let x = 41;", "let x = 42;", &snapshot).await;
        assert!(diff.is_some());

        let diff = diff.unwrap();
        let result = buffer.update(cx, |buffer, cx| {
            let _ = buffer.apply_diff(diff, cx);
            buffer.text()
        });

        assert_eq!(result, "let x = 42;\nlet y = 41;\nlet z = 41;");
    }

    #[gpui::test]
    async fn test_replace_exact_empty_buffer(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| language::Buffer::local("", cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let diff = replace_exact("let x = 41;", "let x = 42;", &snapshot).await;
        assert!(diff.is_none());
    }

    #[gpui::test]
    async fn test_replace_exact_partial_match(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| language::Buffer::local("let x = 41; let y = 42;", cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        // Verify substring replacement actually works
        let diff = replace_exact("let x = 41", "let x = 42", &snapshot).await;
        assert!(diff.is_some());

        let diff = diff.unwrap();
        let result = buffer.update(cx, |buffer, cx| {
            let _ = buffer.apply_diff(diff, cx);
            buffer.text()
        });

        assert_eq!(result, "let x = 42; let y = 42;");
    }

    #[gpui::test]
    async fn test_replace_exact_whitespace_sensitive(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| language::Buffer::local("let x = 41;", cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let diff = replace_exact(" let x = 41;", "let x = 42;", &snapshot).await;
        assert!(diff.is_none());
    }

    #[gpui::test]
    async fn test_replace_exact_entire_buffer(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| language::Buffer::local("let x = 41;", cx));
        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let diff = replace_exact("let x = 41;", "let x = 42;", &snapshot).await;
        assert!(diff.is_some());

        let diff = diff.unwrap();
        let result = buffer.update(cx, |buffer, cx| {
            let _ = buffer.apply_diff(diff, cx);
            buffer.text()
        });

        assert_eq!(result, "let x = 42;");
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
