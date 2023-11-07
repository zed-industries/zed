use gpui::{
    div, Component, Div, KeyBinding, ParentElement, Render, SharedString, StatelessInteractive,
    Styled, View, VisualContext, WindowContext,
};
use picker::{Picker, PickerDelegate};
use theme2::ActiveTheme;

pub struct PickerStory {
    picker: View<Picker<Delegate>>,
}

struct Delegate {
    candidates: Vec<SharedString>,
    selected_ix: usize,
}

impl PickerDelegate for Delegate {
    type ListItem = Div<Picker<Self>>;

    fn match_count(&self) -> usize {
        self.candidates.len()
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut gpui::ViewContext<Picker<Self>>,
    ) -> Self::ListItem {
        let colors = cx.theme().colors();

        div()
            .text_color(colors.text)
            .when(selected, |s| {
                s.border_l_10().border_color(colors.terminal_ansi_yellow)
            })
            .hover(|style| {
                style
                    .bg(colors.element_active)
                    .text_color(colors.text_accent)
            })
            .child(self.candidates[ix].clone())
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut gpui::ViewContext<Picker<Self>>) {
        self.selected_ix = ix;
        cx.notify();
    }

    fn confirm(&mut self, secondary: bool, cx: &mut gpui::ViewContext<Picker<Self>>) {
        if secondary {
            eprintln!("Secondary confirmed {}", self.candidates[self.selected_ix])
        } else {
            eprintln!("Confirmed {}", self.candidates[self.selected_ix])
        }
    }

    fn dismissed(&mut self, cx: &mut gpui::ViewContext<Picker<Self>>) {
        cx.quit();
    }
}

impl PickerStory {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.build_view(|cx| {
            cx.bind_keys([
                KeyBinding::new("up", menu::SelectPrev, Some("picker")),
                KeyBinding::new("pageup", menu::SelectFirst, Some("picker")),
                KeyBinding::new("shift-pageup", menu::SelectFirst, Some("picker")),
                KeyBinding::new("ctrl-p", menu::SelectPrev, Some("picker")),
                KeyBinding::new("down", menu::SelectNext, Some("picker")),
                KeyBinding::new("pagedown", menu::SelectLast, Some("picker")),
                KeyBinding::new("shift-pagedown", menu::SelectFirst, Some("picker")),
                KeyBinding::new("ctrl-n", menu::SelectNext, Some("picker")),
                KeyBinding::new("cmd-up", menu::SelectFirst, Some("picker")),
                KeyBinding::new("cmd-down", menu::SelectLast, Some("picker")),
                KeyBinding::new("enter", menu::Confirm, Some("picker")),
                KeyBinding::new("ctrl-enter", menu::ShowContextMenu, Some("picker")),
                KeyBinding::new("cmd-enter", menu::SecondaryConfirm, Some("picker")),
                KeyBinding::new("escape", menu::Cancel, Some("picker")),
                KeyBinding::new("ctrl-c", menu::Cancel, Some("picker")),
            ]);

            PickerStory {
                picker: cx.build_view(|cx| {
                    let picker = Picker::new(
                        Delegate {
                            candidates: vec![
                                "Baguette (France)".into(),
                                "Baklava (Turkey)".into(),
                                "Beef Wellington (UK)".into(),
                                "Biryani (India)".into(),
                                "Borscht (Ukraine)".into(),
                                "Bratwurst (Germany)".into(),
                                "Bulgogi (Korea)".into(),
                                "Burrito (USA)".into(),
                                "Ceviche (Peru)".into(),
                                "Chicken Tikka Masala (India)".into(),
                                "Churrasco (Brazil)".into(),
                                "Couscous (North Africa)".into(),
                                "Croissant (France)".into(),
                                "Dim Sum (China)".into(),
                                "Empanada (Argentina)".into(),
                                "Fajitas (Mexico)".into(),
                                "Falafel (Middle East)".into(),
                                "Feijoada (Brazil)".into(),
                                "Fish and Chips (UK)".into(),
                                "Fondue (Switzerland)".into(),
                                "Goulash (Hungary)".into(),
                                "Haggis (Scotland)".into(),
                                "Kebab (Middle East)".into(),
                                "Kimchi (Korea)".into(),
                                "Lasagna (Italy)".into(),
                                "Maple Syrup Pancakes (Canada)".into(),
                                "Moussaka (Greece)".into(),
                                "Pad Thai (Thailand)".into(),
                                "Paella (Spain)".into(),
                                "Pancakes (USA)".into(),
                                "Pasta Carbonara (Italy)".into(),
                                "Pavlova (Australia)".into(),
                                "Peking Duck (China)".into(),
                                "Pho (Vietnam)".into(),
                                "Pierogi (Poland)".into(),
                                "Pizza (Italy)".into(),
                                "Poutine (Canada)".into(),
                                "Pretzel (Germany)".into(),
                                "Ramen (Japan)".into(),
                                "Rendang (Indonesia)".into(),
                                "Sashimi (Japan)".into(),
                                "Satay (Indonesia)".into(),
                                "Shepherd's Pie (Ireland)".into(),
                                "Sushi (Japan)".into(),
                                "Tacos (Mexico)".into(),
                                "Tandoori Chicken (India)".into(),
                                "Tortilla (Spain)".into(),
                                "Tzatziki (Greece)".into(),
                                "Wiener Schnitzel (Austria)".into(),
                            ],
                            selected_ix: 0,
                        },
                        cx,
                    );
                    picker.focus(cx);
                    picker
                }),
            }
        })
    }
}

impl Render for PickerStory {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        div()
            .bg(cx.theme().styles.colors.background)
            .size_full()
            .child(self.picker.clone())
    }
}
