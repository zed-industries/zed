use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Renderer, Snippet};
use anstyle::AnsiColor;
use anstyle::Effects;
use anstyle::Style;

fn main() {
    let source = r#"// Make sure "highlighted" code is colored purple

//@ compile-flags: --error-format=human --color=always
//@ edition:2018

use core::pin::Pin;
use core::future::Future;
use core::any::Any;

fn query(_: fn(Box<(dyn Any + Send + '_)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>>) {}

fn wrapped_fn<'a>(_: Box<(dyn Any + Send)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>> {
    Box::pin(async { Err("nope".into()) })
}

fn main() {
    query(wrapped_fn);
}"#;

    const MAGENTA: Style = AnsiColor::Magenta.on_default().effects(Effects::BOLD);
    let message = format!(
        "expected fn pointer `{MAGENTA}for<'a>{MAGENTA:#} fn(Box<{MAGENTA}(dyn Any + Send + 'a){MAGENTA:#}>) -> Pin<_>`
      found fn item `fn(Box<{MAGENTA}(dyn Any + Send + 'static){MAGENTA:#}>) -> Pin<_> {MAGENTA}{{wrapped_fn}}{MAGENTA:#}`",
    );

    let report = &[
        Level::ERROR
            .primary_title("mismatched types")
            .id("E0308")
            .element(
                Snippet::source(source)
                    .path("$DIR/highlighting.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(553..563)
                            .label("one type is more general than the other"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(547..552)
                            .label("arguments to this function are incorrect"),
                    ),
            )
            .element(Level::NOTE.message(&message)),
        Level::NOTE
            .secondary_title("function defined here")
            .element(
                Snippet::source(source)
                    .path("$DIR/highlighting.rs")
                    .annotation(AnnotationKind::Context.span(200..333).label(""))
                    .annotation(AnnotationKind::Primary.span(194..199)),
            ),
    ];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
