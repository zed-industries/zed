use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};

fn main() {
    let source = r#"//@ compile-flags: -Ztreat-err-as-bug
//@ failure-status: 101
//@ error-pattern: aborting due to `-Z treat-err-as-bug=1`
//@ error-pattern: [eval_static_initializer] evaluating initializer of static `C`
//@ normalize-stderr: "note: .*\n\n" -> ""
//@ normalize-stderr: "thread 'rustc' panicked.*:\n.*\n" -> ""
//@ rustc-env:RUST_BACKTRACE=0

#![crate_type = "rlib"]

pub static C: u32 = 0 - 1;
//~^ ERROR could not evaluate static initializer
"#;
    let report = &[Level::ERROR
        .with_name(Some("error: internal compiler error"))
        .primary_title("could not evaluate static initializer")
        .id("E0080")
        .element(
            Snippet::source(source).path("$DIR/err.rs").annotation(
                AnnotationKind::Primary
                    .span(386..391)
                    .label("attempt to compute `0_u32 - 1_u32`, which would overflow"),
            ),
        )];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
