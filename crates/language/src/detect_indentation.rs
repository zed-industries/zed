use std::collections::HashMap;
use std::num::NonZeroU32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DetectedIndentation {
    pub hard_tabs: bool,
    pub tab_size: Option<NonZeroU32>,
}

pub fn detect_indentation<'a>(lines: impl Iterator<Item = &'a str>) -> Option<DetectedIndentation> {
    const TEST_LINES: usize = 2000;
    const MAX_DELTA_SAMPLES: usize = 10;

    let mut lines_with_leading_tab = 0;
    let mut lines_with_leading_space = 0;

    let mut delta_histogram = HashMap::<usize, usize>::new();

    let mut previous_indent = None;

    for line in lines.take(TEST_LINES) {
        if line.trim().is_empty() {
            continue;
        }

        let tabs = line.chars().take_while(|&c| c == '\t').count();
        let spaces = line.chars().take_while(|&c| c == ' ').count();

        // Fow now, we are ignoring mixed indentation
        if tabs > 0 && spaces > 0 {
            continue;
        }

        if tabs > 0 {
            lines_with_leading_tab += 1;
            continue;
        }

        if spaces > 0 {
            lines_with_leading_space += 1;

            if let Some(previous_indent) = previous_indent {
                let delta = spaces.abs_diff(previous_indent);

                if delta > 0 {
                    *delta_histogram.entry(delta).or_default() += 1;

                    if *delta_histogram.get(&delta).unwrap() >= MAX_DELTA_SAMPLES {
                        break;
                    }
                }
            }

            previous_indent = Some(spaces);
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
                .and_then(|(size, _)| NonZeroU32::new(size as u32)),
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

    let result = detect_indentation(source.lines());

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

    let result = detect_indentation(source.lines());

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
        detect_indentation(source.lines()),
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
        detect_indentation(source.lines()),
        Some(DetectedIndentation {
            hard_tabs: false,
            tab_size: NonZeroU32::new(2),
        })
    );
}
