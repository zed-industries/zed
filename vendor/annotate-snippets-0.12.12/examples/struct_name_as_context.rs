use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Renderer, Snippet};
fn main() {
    let source = r#"struct S {
    field1: usize,
    field2: usize,
    field3: usize,
    field4: usize,
    fn foo() {},
    field6: usize,
}
"#;
    let report = &[Level::ERROR
        .primary_title("functions are not allowed in struct definitions")
        .element(
            Snippet::source(source)
                .path("$DIR/struct_name_as_context.rs")
                .annotation(AnnotationKind::Primary.span(91..102))
                .annotation(AnnotationKind::Visible.span(0..8)),
        )
        .element(
            Level::HELP
                .message("unlike in C++, Java, and C#, functions are declared in `impl` blocks"),
        )];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
