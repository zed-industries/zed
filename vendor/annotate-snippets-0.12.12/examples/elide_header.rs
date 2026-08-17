use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Group, Level, Renderer, Snippet};

fn main() {
    let source = r#"# Docstring followed by a newline

def foobar(door, bar={}):
    """
    """
"#;

    let report = &[Group::with_level(Level::NOTE)
        .element(
            Snippet::source(source)
                .fold(false)
                .annotation(AnnotationKind::Primary.span(56..58).label("B006")),
        )
        .element(Level::HELP.message("Replace with `None`; initialize within function"))];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
