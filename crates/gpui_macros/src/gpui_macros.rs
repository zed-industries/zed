mod derive_into_element;
mod derive_render;
mod register_action;
mod style_helpers;
mod test;

use proc_macro::TokenStream;

/// register_action! can be used to register an action with the GPUI runtime.
/// You should typically use `gpui::actions!` or `gpui::impl_actions!` instead,
/// but this can be used for fine grained customization.
#[proc_macro]
pub fn register_action(ident: TokenStream) -> TokenStream {
    register_action::register_action_macro(ident)
}

/// #[derive(IntoElement)] is used to create a Component out of anything that implements
/// the `RenderOnce` trait.
#[proc_macro_derive(IntoElement)]
pub fn derive_into_element(input: TokenStream) -> TokenStream {
    derive_into_element::derive_into_element(input)
}

#[proc_macro_derive(Render)]
#[doc(hidden)]
pub fn derive_render(input: TokenStream) -> TokenStream {
    derive_render::derive_render(input)
}

/// Used by gpui to generate the style helpers.
#[proc_macro]
#[doc(hidden)]
pub fn style_helpers(input: TokenStream) -> TokenStream {
    style_helpers::style_helpers(input)
}

/// #[gpui::test] can be used to annotate test functions that run with GPUI support.
/// it supports both synchronous and asynchronous tests, and can provide you with
/// as many `TestAppContext` instances as you need.
/// The output contains a `#[test]` annotation so this can be used with any existing
/// test harness (`cargo test` or `cargo-nextest`).
///
/// ```
/// #[gpui::test]
/// async fn test_foo(mut cx: &TestAppContext) { }
/// ```
///
/// In addition to passing a TestAppContext, you can also ask for a `StdRnd` instance.
/// this will be seeded with the `SEED` environment variable and is used internally by
/// the ForegroundExecutor and BackgroundExecutor to run tasks deterministically in tests.
/// Using the same `StdRng` for behaviour in your test will allow you to exercise a wide
/// variety of scenarios and interleavings just by changing the seed.
///
/// #[gpui::test] also takes three different arguments:
/// - `#[gpui::test(iterations=10)]` will run the test ten times with a different initial SEED.
/// - `#[gpui::test(retries=3)]` will run the test up to four times if it fails to try and make it pass.
/// - `#[gpui::test(on_failure="crate::test::report_failure")]` will call the specified function after the
///    tests fail so that you can write out more detail about the failure.
#[proc_macro_attribute]
pub fn test(args: TokenStream, function: TokenStream) -> TokenStream {
    test::test(args, function)
}
