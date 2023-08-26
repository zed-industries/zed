use gpui::elements::Text;
use gpui::geometry::vector::vec2f;
use gpui::LayoutContext;
use gpui::SizeConstraint;
use gpui::{elements::Empty, fonts, AnyElement, AppContext, Entity, View, ViewContext};
use gpui::Element;

#[crate::test(self)]
fn test_soft_wrapping_with_carriage_returns(cx: &mut AppContext) {
    cx.add_window(Default::default(), |cx| {
        let mut view = TestView;
        fonts::with_font_cache(cx.font_cache().clone(), || {
            let mut text = Text::new("Hello\r\n", Default::default()).with_soft_wrap(true);
            let mut new_parents = Default::default();
            let mut notify_views_if_parents_change = Default::default();
            let mut layout_cx = LayoutContext::new(
                cx,
                &mut new_parents,
                &mut notify_views_if_parents_change,
                false,
            );
            let (_, state) = text.layout(
                SizeConstraint::new(Default::default(), vec2f(f32::INFINITY, f32::INFINITY)),
                &mut view,
                &mut layout_cx,
            );
            assert_eq!(state.shaped_lines.len(), 2);
            assert_eq!(state.wrap_boundaries.len(), 2);
        });
        view
    });
}

struct TestView;

impl Entity for TestView {
    type Event = ();
}

impl View for TestView {
    fn ui_name() -> &'static str {
        "TestView"
    }

    fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
        Empty::new().into_any()
    }
}
