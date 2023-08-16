use gpui::{elements::RenderElement, View, ViewContext};
use gpui_macros::Element;

#[test]
fn test_derive_render_element() {
    #[derive(Element)]
    struct TestElement {}

    impl RenderElement for TestElement {
        fn render<V: View>(&mut self, _: &mut V, _: &mut ViewContext<V>) -> gpui::AnyElement<V> {
            unimplemented!()
        }
    }
}
