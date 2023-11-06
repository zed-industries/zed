use gpui::{div, Div, ParentElement, Render, View, VisualContext, WindowContext};
use picker::{Picker, PickerDelegate};

pub struct PickerStory {
    //     picker: View<Picker<PickerStoryDelegate>>,
}

impl PickerDelegate for PickerStory {
    type ListItem = Div<Self>;

    fn match_count(&self, picker_id: gpui::ElementId) -> usize {
        0
    }

    fn render_match(
        &self,
        ix: usize,
        active: bool,
        hovered: bool,
        selected: bool,
        picker_id: gpui::ElementId,
        cx: &mut gpui::ViewContext<Self>,
    ) -> Self::ListItem {
        todo!()
    }
}

impl PickerStory {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.build_view(|cx| PickerStory {})
    }
}

impl Render for PickerStory {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        div().child(Picker::new("picker_story"))
    }
}
