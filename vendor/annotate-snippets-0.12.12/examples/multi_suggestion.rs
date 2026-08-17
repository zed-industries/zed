use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Patch, Renderer, Snippet};

fn main() {
    let source = r#"
#![allow(dead_code)]
struct U <T> {
    wtf: Option<Box<U<T>>>,
    x: T,
}
fn main() {
    U {
        wtf: Some(Box(U {
            wtf: None,
            x: (),
        })),
        x: ()
    };
    let _ = std::collections::HashMap(); 
    let _ = std::collections::HashMap {};
    let _ = Box {};
}
"#;

    let report = &[
        Level::ERROR
            .primary_title(
                "cannot construct `Box<_, _>` with struct literal syntax due to private fields",
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .annotation(AnnotationKind::Primary.span(295..298)),
            )
            .element(Level::NOTE.message("private fields `0` and `1` that were not provided")),
        Level::HELP
            .secondary_title(
                "you might have meant to use an associated function to build this type",
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new(_)")),
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new_uninit()")),
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new_zeroed()")),
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new_in(_, _)")),
            )
            .element(Level::NOTE.no_name().message("and 12 other candidates")),
        Level::HELP
            .secondary_title("consider using the `Default` trait")
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(295..295, "<"))
                    .patch(Patch::new(
                        298..301,
                        " as std::default::Default>::default()",
                    )),
            ),
    ];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
