use gpui::{elements::*, Entity, RenderContext, View, ViewContext};
use settings::Settings;

pub enum Event {
    Deactivated,
}

pub struct ContactsPopover;

impl Entity for ContactsPopover {
    type Event = Event;
}

impl View for ContactsPopover {
    fn ui_name() -> &'static str {
        "ContactsPopover"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.contacts_popover;
        Empty::new()
            .contained()
            .with_background_color(theme.background)
            .boxed()
    }
}

impl ContactsPopover {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        cx.observe_window_activation(Self::window_activation_changed)
            .detach();
        Self
    }

    fn window_activation_changed(&mut self, is_active: bool, cx: &mut ViewContext<Self>) {
        if !is_active {
            cx.emit(Event::Deactivated);
        }
    }
}
