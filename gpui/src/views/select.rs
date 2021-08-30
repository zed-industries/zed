use crate::{elements::*, Entity, RenderContext, View};
use std::ops::Range;

pub struct Select {
    selected_ix: Option<usize>,
    render_selected_element: Box<dyn FnMut()>,
    render_elements: Box<dyn FnMut(Range<usize>, &mut RenderContext<Self>)>,
}

pub enum Event {}

impl Entity for Select {
    type Event = Event;
}

impl View for Select {
    fn ui_name() -> &'static str {
        "Select"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        todo!()
    }
}
