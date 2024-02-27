use gpui::{Menu, MenuItem};

pub fn app_menus() -> Vec<Menu<'static>> {
    use crate::actions::Quit;

    vec![Menu {
        name: "Storybook",
        items: vec![MenuItem::action("Quit", Quit)],
    }]
}
