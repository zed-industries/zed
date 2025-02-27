use gpui::{AnyView, Corner, Entity, ManagedView};
use ui::{
    px, App, ButtonCommon, IntoElement, PopoverMenu, PopoverMenuHandle, PopoverTrigger, RenderOnce,
    Window,
};

pub trait TriggerablePopover: ManagedView {
    fn menu_handle(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> PopoverMenuHandle<Self>;
}

// We want a button, that tells us what parameters to pass, and that "just works" after that
pub struct PopoverButton<T, B, F> {
    selector: Entity<T>,
    button: B,
    tooltip: F,
    corner: Corner,
}

impl<T, B, F> PopoverButton<T, B, F> {
    pub fn new(selector: Entity<T>, corner: Corner, button: B, tooltip: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyView + 'static,
    {
        Self {
            selector,
            button,
            tooltip,
            corner,
        }
    }
}

impl<T: TriggerablePopover, B: PopoverTrigger + ButtonCommon, F> RenderOnce
    for PopoverButton<T, B, F>
where
    F: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let menu_handle = self
            .selector
            .update(cx, |selector, cx| selector.menu_handle(window, cx));

        PopoverMenu::new("popover-button")
            .menu({
                let selector = self.selector.clone();
                move |_window, _cx| Some(selector.clone())
            })
            .trigger_with_tooltip(self.button, self.tooltip)
            .anchor(self.corner)
            .with_handle(menu_handle)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
    }
}
