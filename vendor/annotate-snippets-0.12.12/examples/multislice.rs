use annotate_snippets::{renderer::DecorStyle, Annotation, Level, Renderer, Snippet};

fn main() {
    let report = &[Level::ERROR
        .primary_title("mismatched types")
        .element(
            Snippet::<Annotation<'_>>::source("Foo")
                .line_start(51)
                .fold(false)
                .path("src/format.rs"),
        )
        .element(
            Snippet::<Annotation<'_>>::source("Faa")
                .line_start(129)
                .fold(false)
                .path("src/display.rs"),
        )];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
