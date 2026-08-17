use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};

fn main() {
    let source = r#"//@ compile-flags: -Zterminal-urls=yes
fn main() {
    let () = 4; //~ ERROR
}
"#;
    let report = &[Level::ERROR
        .primary_title("mismatched types")
        .id("E0308")
        .id_url("https://doc.rust-lang.org/error_codes/E0308.html")
        .element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/terminal_urls.rs")
                .annotation(
                    AnnotationKind::Primary
                        .span(59..61)
                        .label("expected integer, found `()`"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(64..65)
                        .label("this expression has type `{integer}`"),
                ),
        )];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
