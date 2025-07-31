use gpui::{Menu, MenuItem};

pub fn app_menus() -> Vec<Menu> {
    use crate::actions::Quit;

    vec![Menu {
        name: "Storybook".into(),
        items: vec![MenuItem::action("Quit", Quit)],
    }]
}
