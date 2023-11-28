use gpui::{ClickEvent, WindowContext};

pub trait Clickable {
    fn on_click(
        &mut self,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> &mut Self;
}
