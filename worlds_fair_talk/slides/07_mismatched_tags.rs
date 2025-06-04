// Another tricky case with XML: Tag mismatches

// Initial pass rate: 60%

// PROMPT FIX:
// Added: "Always close all tags properly"

// After prompt fix: 95% pass rate
// That last 5% wouldn't budge, so we made the parser forgiving:

#[gpui::test(iterations = 1000)]
fn test_mismatched_tags(mut rng: StdRng) {
    let mut parser = EditParser::new();
    assert_eq!(
        parse_random_chunks(
            // Reduced from an actual Sonnet 3.7 output
            indoc! {"
                <old_text>
                a
                b
                c
                </new_text>
                <new_text>
                a
                B
                c
                </old_text>
                <old_text>
                d
                e
                f
                </new_text>
                <new_text>
                D
                e
                F
                </old_text>
            "},
            &mut parser,
            &mut rng
        ),
        vec![
            Edit {
                old_text: "a\nb\nc".to_string(),
                new_text: "a\nB\nc".to_string(),
            },
            Edit {
                old_text: "d\ne\nf".to_string(),
                new_text: "D\ne\nF".to_string(),
            }
        ]
    );
    assert_eq!(
        parser.finish(),
        EditParserMetrics {
            tags: 4,
            mismatched_tags: 4
        }
    );

    let mut parser = EditParser::new();
    assert_eq!(
        parse_random_chunks(
            // Reduced from an actual Opus 4 output
            indoc! {"
                <edits>
                <old_text>
                Lorem
                </old_text>
                <new_text>
                LOREM
                </edits>
            "},
            &mut parser,
            &mut rng
        ),
        vec![Edit {
            old_text: "Lorem".to_string(),
            new_text: "LOREM".to_string(),
        },]
    );
    assert_eq!(
        parser.finish(),
        EditParserMetrics {
            tags: 2,
            mismatched_tags: 1
        }
    );
}

if &self.buffer[tag_range.clone()] != OLD_TEXT_END_TAG {
    self.metrics.mismatched_tags += 1;
    // Keep parsing anyway - don't let bad XML stop us
}

// We track mismatched tags across all evals and fail if > 5%:
let mismatched_tag_ratio =
    cumulative_parser_metrics.mismatched_tags as f32 / cumulative_parser_metrics.tags as f32;
if mismatched_tag_ratio > 0.05 {
    panic!("Too many mismatched tags: {:?}", cumulative_parser_metrics);
}
