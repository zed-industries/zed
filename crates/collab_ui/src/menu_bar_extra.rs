use crate::contacts_popover::{self, ContactsPopover};
use call::ActiveCall;
use client::UserStore;
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    Appearance, Entity, ModelHandle, MouseButton, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle, WindowKind,
};

actions!(menu_bar_extra, [ToggleActiveCallPopover]);

pub fn init(user_store: ModelHandle<UserStore>, cx: &mut MutableAppContext) {
    cx.add_action(MenuBarExtra::toggle_active_call_popover);

    let mut status_bar_item_id = None;
    cx.observe(&ActiveCall::global(cx), move |call, cx| {
        let had_room = status_bar_item_id.is_some();
        let has_room = call.read(cx).room().is_some();
        if had_room != has_room {
            if let Some(status_bar_item_id) = status_bar_item_id.take() {
                cx.remove_status_bar_item(status_bar_item_id);
            }

            if has_room {
                let (id, _) = cx.add_status_bar_item(|_| MenuBarExtra::new(user_store.clone()));
                status_bar_item_id = Some(id);
            }
        }
    })
    .detach();
}

struct MenuBarExtra {
    popover: Option<ViewHandle<ContactsPopover>>,
    user_store: ModelHandle<UserStore>,
}

impl Entity for MenuBarExtra {
    type Event = ();

    fn release(&mut self, cx: &mut MutableAppContext) {
        if let Some(popover) = self.popover.take() {
            cx.remove_window(popover.window_id());
        }
    }
}

impl View for MenuBarExtra {
    fn ui_name() -> &'static str {
        "MenuBarExtra"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let color = match cx.appearance {
            Appearance::Light | Appearance::VibrantLight => Color::black(),
            Appearance::Dark | Appearance::VibrantDark => Color::white(),
        };
        MouseEventHandler::<Self>::new(0, cx, |_, _| {
            Svg::new("icons/zed_22.svg")
                .with_color(color)
                .aligned()
                .boxed()
        })
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(ToggleActiveCallPopover);
        })
        .boxed()
    }
}

impl MenuBarExtra {
    fn new(user_store: ModelHandle<UserStore>) -> Self {
        Self {
            popover: None,
            user_store,
        }
    }

    fn toggle_active_call_popover(
        &mut self,
        _: &ToggleActiveCallPopover,
        cx: &mut ViewContext<Self>,
    ) {
        match self.popover.take() {
            Some(popover) => {
                cx.remove_window(popover.window_id());
            }
            None => {
                let window_bounds = cx.window_bounds();
                let size = vec2f(300., 350.);
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
                    |cx| ContactsPopover::new(true, None, self.user_store.clone(), cx),
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
            contacts_popover::Event::Dismissed => {
                self.popover.take();
                cx.remove_window(popover.window_id());
            }
        }
    }
}
