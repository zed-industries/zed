use crate::{
    adapter::AdapterElement,
    element::{AnyElement, Element},
};
use gpui::ViewContext;

pub fn view<F, E>(mut render: F) -> ViewFn
where
    F: 'static + FnMut(&mut ViewContext<ViewFn>) -> E,
    E: Element<ViewFn>,
{
    ViewFn(Box::new(move |cx| (render)(cx).into_any()))
}

pub struct ViewFn(Box<dyn FnMut(&mut ViewContext<ViewFn>) -> AnyElement<ViewFn>>);

impl gpui::Entity for ViewFn {
    type Event = ();
}

impl gpui::View for ViewFn {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> gpui::AnyElement<Self> {
        use gpui::Element as _;
        AdapterElement((self.0)(cx)).into_any()
    }
}
