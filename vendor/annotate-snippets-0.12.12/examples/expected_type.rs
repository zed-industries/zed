use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Renderer, Snippet};

fn main() {
    let source = r#"                annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    ,
                range: <22, 25>,"#;
    let report =
        &[Level::ERROR
            .primary_title("expected type, found `22`")
            .element(
                Snippet::source(source)
                    .line_start(26)
                    .path("examples/footer.rs")
                    .annotation(AnnotationKind::Primary.span(193..195).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    ))
                    .annotation(
                        AnnotationKind::Context
                            .span(34..50)
                            .label("while parsing this struct"),
                    ),
            )];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
