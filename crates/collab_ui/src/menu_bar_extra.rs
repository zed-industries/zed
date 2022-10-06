use crate::active_call_popover::{self, ActiveCallPopover};
use call::ActiveCall;
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f},
    Appearance, Entity, MouseButton, MutableAppContext, RenderContext, View, ViewContext,
    ViewHandle, WindowKind,
};

actions!(menu_bar_extra, [ToggleActiveCallPopover]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(MenuBarExtra::toggle_active_call_popover);

    let mut status_bar_item_id = None;
    cx.observe(&ActiveCall::global(cx), move |call, cx| {
        if let Some(status_bar_item_id) = status_bar_item_id.take() {
            cx.remove_status_bar_item(status_bar_item_id);
        }

        if call.read(cx).room().is_some() {
            let (id, _) = cx.add_status_bar_item(|_| MenuBarExtra::new());
            status_bar_item_id = Some(id);
        }
    })
    .detach();
}

struct MenuBarExtra {
    popover: Option<ViewHandle<ActiveCallPopover>>,
}

impl Entity for MenuBarExtra {
    type Event = ();
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
    fn new() -> Self {
        Self { popover: None }
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
                    |cx| ActiveCallPopover::new(cx),
                );
                cx.subscribe(&popover, Self::on_popover_event).detach();
                self.popover = Some(popover);
            }
        }
    }

    fn on_popover_event(
        &mut self,
        popover: ViewHandle<ActiveCallPopover>,
        event: &active_call_popover::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            active_call_popover::Event::Deactivated => {
                self.popover.take();
                cx.remove_window(popover.window_id());
            }
        }
    }
}
