use std::collections::HashMap;
use std::num::NonZeroU32;
use text::LineIndent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DetectedIndentation {
    pub hard_tabs: bool,
    pub tab_size: Option<NonZeroU32>,
}

pub fn detect_indentation(
    indents: impl Iterator<Item = LineIndent>,
) -> Option<DetectedIndentation> {
    const TEST_LINES: usize = 2000;
    const MAX_DELTA_SAMPLES: usize = 10;

    let mut lines_with_leading_tab = 0;
    let mut lines_with_leading_space = 0;
    let mut delta_histogram = HashMap::<u32, usize>::new();
    let mut previous_indent: Option<u32> = None;

    for indent in indents.take(TEST_LINES) {
        if indent.is_line_blank() {
            continue;
        }
        // Ignore mixed indentation lines for now, same as before.
        if indent.tabs > 0 && indent.spaces > 0 {
            continue;
        }
        if indent.tabs > 0 {
            lines_with_leading_tab += 1;
            continue;
        }
        if indent.spaces > 0 {
            lines_with_leading_space += 1;
            if let Some(prev) = previous_indent {
                let delta = indent.spaces.abs_diff(prev);
                if delta > 0 {
                    let count = delta_histogram.entry(delta).or_default();
                    *count += 1;
                    if *count >= MAX_DELTA_SAMPLES {
                        break;
                    }
                }
            }
            previous_indent = Some(indent.spaces);
        }
    }

    if lines_with_leading_tab == 0 && lines_with_leading_space == 0 {
        None
    } else if lines_with_leading_tab > lines_with_leading_space {
        Some(DetectedIndentation {
            hard_tabs: true,
            tab_size: None,
        })
    } else {
        Some(DetectedIndentation {
            hard_tabs: false,
            tab_size: delta_histogram
                .into_iter()
                .max_by_key(|(_, count)| *count)
                .and_then(|(size, _)| NonZeroU32::new(size)),
        })
    }
}

#[test]
fn test_detect_four_space_indentation() {
    let source = r#"
fn main() {
    let x = 10;

    if x > 5 {
        println!("hello");
    }
}
"#;

    let result = detect_indentation(source.lines().map(LineIndent::from));

    assert_eq!(
        result,
        Some(DetectedIndentation {
            hard_tabs: false,
            tab_size: NonZeroU32::new(4),
        })
    );
}

#[test]
fn test_detect_tab_indentation() {
    let source = "fn main() {\n\tlet x = 10;\n\tprintln!(\"hello\");\n}\n";

    let result = detect_indentation(source.lines().map(LineIndent::from));

    assert_eq!(
        result,
        Some(DetectedIndentation {
            hard_tabs: true,
            tab_size: None,
        })
    );
}

#[test]
fn test_detect_four_space_indentation_nested() {
    let source = r#"
fn main() {
    if true {
        println!("hello");
    }
}
"#;

    assert_eq!(
        detect_indentation(source.lines().map(LineIndent::from)),
        Some(DetectedIndentation {
            hard_tabs: false,
            tab_size: NonZeroU32::new(4),
        })
    );
}

#[test]
fn test_detect_two_space_indentation() {
    let source = r#"
fn main() {
  let x = 10;

  if x > 5 {
    println!("hello");
  }
}
"#;

    assert_eq!(
        detect_indentation(source.lines().map(LineIndent::from)),
        Some(DetectedIndentation {
            hard_tabs: false,
            tab_size: NonZeroU32::new(2),
        })
    );
}
