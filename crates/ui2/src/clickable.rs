use gpui::{ClickEvent, WindowContext};

pub trait Clickable {
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self;
}
