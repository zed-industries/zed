use gpui::{
    black, div, red, Div, Fill, ParentElement, Render, SharedString, Styled, View, VisualContext,
    WindowContext,
};
use picker::{Picker, PickerDelegate};

pub struct PickerStory {
    candidates: Vec<SharedString>,
}

impl PickerDelegate for PickerStory {
    type ListItem = SharedString;

    fn match_count(&self, _picker_id: gpui::ElementId) -> usize {
        self.candidates.len()
    }

    fn render_match(
        &self,
        ix: usize,
        _active: bool,
        _hovered: bool,
        _selected: bool,
        _picker_id: gpui::ElementId,
        cx: &mut gpui::ViewContext<Self>,
    ) -> Self::ListItem {
        self.candidates[ix].clone()
    }
}

impl PickerStory {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.build_view(|cx| PickerStory {
            candidates: vec![
                "Pizza (Italy)".into(),
                "Sushi (Japan)".into(),
                "Paella (Spain)".into(),
                "Tacos (Mexico)".into(),
                "Peking Duck (China)".into(),
                "Fish and Chips (UK)".into(),
                "Croissant (France)".into(),
                "Bratwurst (Germany)".into(),
                "Poutine (Canada)".into(),
                "Chicken Tikka Masala (India)".into(),
                "Feijoada (Brazil)".into(),
                "Kimchi (Korea)".into(),
                "Borscht (Ukraine)".into(),
                "Falafel (Middle East)".into(),
                "Baklava (Turkey)".into(),
                "Shepherd's Pie (Ireland)".into(),
                "Rendang (Indonesia)".into(),
                "Kebab (Middle East)".into(),
                "Ceviche (Peru)".into(),
                "Pierogi (Poland)".into(),
                "Churrasco (Brazil)".into(),
                "Moussaka (Greece)".into(),
                "Lasagna (Italy)".into(),
                "Pad Thai (Thailand)".into(),
                "Pho (Vietnam)".into(),
            ],
        })
    }
}

impl Render for PickerStory {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        div()
            .text_color(red())
            .size_full()
            .child(Picker::new("picker_story"))
    }
}
