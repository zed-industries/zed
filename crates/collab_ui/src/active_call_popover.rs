use gpui::{color::Color, elements::*, Entity, RenderContext, View, ViewContext};

pub enum Event {
    Deactivated,
}

pub struct ActiveCallPopover {
    _subscription: gpui::Subscription,
}

impl Entity for ActiveCallPopover {
    type Event = Event;
}

impl View for ActiveCallPopover {
    fn ui_name() -> &'static str {
        "ActiveCallPopover"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        Empty::new()
            .contained()
            .with_background_color(Color::red())
            .boxed()
    }
}

impl ActiveCallPopover {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            _subscription: cx.observe_window_activation(Self::window_activation_changed),
        }
    }

    fn window_activation_changed(&mut self, is_active: bool, cx: &mut ViewContext<Self>) {
        if !is_active {
            cx.emit(Event::Deactivated);
        }
    }
}
