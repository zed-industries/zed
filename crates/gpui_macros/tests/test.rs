use gpui::{elements::Empty, Element, ViewContext};
// use gpui_macros::Element;

#[test]
fn test_derive_render_element() {
    #[derive(Element)]
    struct TestElement {}

    impl TestElement {
        fn render<V: 'static>(&mut self, _: &mut V, _: &mut ViewContext<V>) -> impl Element<V> {
            Empty::new()
        }
    }
}
