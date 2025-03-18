pub fn replace_flexible(whole: &str, old: &str, new: &str) -> Option<String> {
    let (old_lines, old_min_indent) = lines_with_min_indent(old);
    let (new_lines, new_min_indent) = lines_with_min_indent(new);
    let min_indent = old_min_indent.min(new_min_indent);

    let old_lines = drop_lines_prefix(&old_lines, min_indent);
    let new_lines = drop_lines_prefix(&new_lines, min_indent);

    let whole_lines = whole.lines().collect::<Vec<_>>();

    'windows: for (i, window) in whole_lines.windows(old_lines.len()).enumerate() {
        let mut common_leading = None;

        for (line, old_line) in window.iter().zip(old_lines.iter()) {
            let line_trimmed = line.trim_start();

            if line_trimmed != old_line.trim_start() {
                continue 'windows;
            }

            if line_trimmed.is_empty() {
                continue;
            }

            let line_leading = &line[..line.len() - old_line.len()];

            match common_leading {
                Some(common_leading) if common_leading != line_leading => {
                    // indent mismatch is not consistent
                    continue 'windows;
                }
                Some(_) => (),
                None => common_leading = Some(line_leading),
            }
        }

        if let Some(common_leading) = common_leading {
            return Some(
                whole_lines[..i]
                    .iter()
                    .map(|line| line.to_string())
                    .chain(new_lines.into_iter().map(|new_line| {
                        if new_line.trim().is_empty() {
                            new_line.to_string()
                        } else {
                            common_leading.to_string() + new_line
                        }
                    }))
                    .chain(
                        whole_lines[i + old_lines.len()..]
                            .iter()
                            .map(|line| line.to_string()),
                    )
                    .collect::<Vec<_>>()
                    .join("\n")
                    + "\n",
            );
        }
    }

    None
}

fn drop_lines_prefix<'a>(lines: &'a [&str], prefix_len: usize) -> Vec<&'a str> {
    lines
        .into_iter()
        .map(|line| {
            if line.len() > prefix_len {
                &line[prefix_len..]
            } else {
                ""
            }
        })
        .collect()
}

fn lines_with_min_indent(input: &str) -> (Vec<&str>, usize) {
    let mut min_indent: Option<usize> = None;

    let lines = input
        .lines()
        .map(|line| {
            if line.chars().any(|b| !b.is_whitespace()) {
                let indent = line.len() - line.trim_start().len();
                min_indent = Some(min_indent.map_or(indent, |m| m.min(indent)));
            }

            line
        })
        .collect::<Vec<_>>();

    (lines, min_indent.unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use unindent::Unindent;

    #[test]
    fn test_replace_consistent_indentation() {
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
            replace_flexible(&whole, &old, &new),
            Some(expected.to_string())
        );
    }

    #[test]
    fn test_replace_inconsistent_indentation() {
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

        assert_eq!(replace_flexible(&whole, &old, &new), None);
    }

    #[test]
    fn test_replace_with_empty_lines() {
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
            replace_flexible(&whole, &old, &new),
            Some(expected.to_string())
        );
    }

    #[test]
    fn test_replace_no_match() {
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

        assert_eq!(replace_flexible(&whole, &old, &new), None);
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
}
