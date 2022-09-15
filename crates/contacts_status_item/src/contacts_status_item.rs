mod contacts_popover;

use contacts_popover::ContactsPopover;
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    Appearance, Entity, MouseButton, MutableAppContext, RenderContext, View, ViewContext,
    ViewHandle, WindowKind,
};

actions!(contacts_status_item, [ToggleContactsPopover]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ContactsStatusItem::toggle_contacts_popover);
}

pub struct ContactsStatusItem {
    popover: Option<ViewHandle<ContactsPopover>>,
}

impl Entity for ContactsStatusItem {
    type Event = ();
}

impl View for ContactsStatusItem {
    fn ui_name() -> &'static str {
        "ContactsStatusItem"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let color = match cx.appearance {
            Appearance::Light | Appearance::VibrantLight => Color::black(),
            Appearance::Dark | Appearance::VibrantDark => Color::white(),
        };
        MouseEventHandler::new::<Self, _, _>(0, cx, |_, _| {
            Svg::new("icons/zed_22.svg")
                .with_color(color)
                .aligned()
                .boxed()
        })
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(ToggleContactsPopover);
        })
        .boxed()
    }
}

impl ContactsStatusItem {
    pub fn new() -> Self {
        Self { popover: None }
    }

    fn toggle_contacts_popover(&mut self, _: &ToggleContactsPopover, cx: &mut ViewContext<Self>) {
        match self.popover.take() {
            Some(popover) => {
                cx.remove_window(popover.window_id());
            }
            None => {
                let window_bounds = cx.window_bounds();
                let size = vec2f(360., 460.);
                let origin = window_bounds.lower_left()
                    + vec2f(window_bounds.width() / 2. - size.x() / 2., 0.);
                let (_, popover) = cx.add_window(
                    gpui::WindowOptions {
                        bounds: gpui::WindowBounds::Fixed(RectF::new(origin, size)),
                        titlebar: None,
                        center: false,
                        kind: WindowKind::PopUp,
                        is_movable: false,
                    },
                    |cx| ContactsPopover::new(cx),
                );
                cx.subscribe(&popover, Self::on_popover_event).detach();
                self.popover = Some(popover);
            }
        }
    }

    fn on_popover_event(
        &mut self,
        popover: ViewHandle<ContactsPopover>,
        event: &contacts_popover::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            contacts_popover::Event::Deactivated => {
                self.popover.take();
                cx.remove_window(popover.window_id());
            }
        }
    }
}
