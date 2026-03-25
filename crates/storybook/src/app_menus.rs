use gpui::{Menu, MenuItem};

pub fn app_menus() -> Vec<Menu> {
    use crate::actions::Quit;

    vec![Menu::new("Storybook").items([MenuItem::action("Quit", Quit)])]
}
