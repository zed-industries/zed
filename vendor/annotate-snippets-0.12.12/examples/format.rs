use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Renderer, Snippet};

fn main() {
    let source = r#") -> Option<String> {
    for ann in annotations {
        match (ann.range.0, ann.range.1) {
            (None, None) => continue,
            (Some(start), Some(end)) if start > end_index => continue,
            (Some(start), Some(end)) if start >= start_index => {
                let label = if let Some(ref label) = ann.label {
                    format!(" {}", label)
                } else {
                    String::from("")
                };

                return Some(format!(
                    "{}{}{}",
                    " ".repeat(start - start_index),
                    "^".repeat(end - start),
                    label
                ));
            }
            _ => continue,
        }
    }"#;
    let report = &[Level::ERROR
        .primary_title("mismatched types")
        .id("E0308")
        .element(
            Snippet::source(source)
                .line_start(51)
                .path("src/format.rs")
                .fold(false)
                .annotation(
                    AnnotationKind::Context
                        .span(5..19)
                        .label("expected `Option<String>` because of return type"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(26..724)
                        .label("expected enum `std::option::Option`"),
                ),
        )];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
