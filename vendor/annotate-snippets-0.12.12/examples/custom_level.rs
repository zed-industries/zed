use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet};

fn main() {
    let source = r#"// Regression test for issue #114529
// Tests that we do not ICE during const eval for a
// break-with-value in contexts where it is illegal

#[allow(while_true)]
fn main() {
    [(); {
        while true {
            break 9; //~ ERROR `break` with value from a `while` loop
        };
        51
    }];

    [(); {
        while let Some(v) = Some(9) {
            break v; //~ ERROR `break` with value from a `while` loop
        };
        51
    }];

    while true {
        break (|| { //~ ERROR `break` with value from a `while` loop
            let local = 9;
        });
    }
}
"#;
    let report = &[
        Level::ERROR
            .primary_title("`break` with value from a `while` loop")
            .id("E0571")
            .element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/issue-114529-illegal-break-with-value.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(483..581)
                            .label("can only break with a value inside `loop` or breakable block"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(462..472)
                            .label("you can't `break` with a value in a `while` loop"),
                    ),
            ),
        Level::HELP
            .with_name(Some("suggestion"))
            .secondary_title("use `break` on its own without a value inside this `while` loop")
            .element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/issue-114529-illegal-break-with-value.rs")
                    .patch(Patch::new(483..581, "break")),
            ),
    ];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
