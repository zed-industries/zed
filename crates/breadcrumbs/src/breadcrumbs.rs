use editor::render_breadcrumb_text;
use gpui::{Context, EventEmitter, IntoElement, Render, Subscription, Window};
use theme::ActiveTheme;
use ui::prelude::*;
use workspace::{
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
    item::{ItemEvent, ItemHandle},
};

pub struct Breadcrumbs {
    pane_focused: bool,
    active_item: Option<Box<dyn ItemHandle>>,
    subscription: Option<Subscription>,
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self::new()
    }
}

impl Breadcrumbs {
    pub fn new() -> Self {
        Self {
            pane_focused: false,
            active_item: Default::default(),
            subscription: Default::default(),
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for Breadcrumbs {}

impl Render for Breadcrumbs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let element = h_flex()
            .id("breadcrumb-container")
            .flex_grow()
            .h_8()
            .overflow_x_scroll()
            .text_ui(cx);

        let Some(active_item) = self.active_item.as_ref() else {
            return element.into_any_element();
        };

        let Some(segments) = active_item.breadcrumbs(cx.theme(), cx) else {
            return element.into_any_element();
        };

        let prefix_element = active_item.breadcrumb_prefix(window, cx);

        render_breadcrumb_text(
            segments,
            prefix_element,
            active_item.as_ref(),
            false,
            window,
            cx,
        )
        .into_any_element()
    }
}

impl ToolbarItemView for Breadcrumbs {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_item = None;

        let Some(item) = active_pane_item else {
            return ToolbarItemLocation::Hidden;
        };

        let this = cx.entity().downgrade();
        self.subscription = Some(item.subscribe_to_item_events(
            window,
            cx,
            Box::new(move |event, _, cx| {
                if let ItemEvent::UpdateBreadcrumbs = event {
                    this.update(cx, |this, cx| {
                        cx.notify();
                        if let Some(active_item) = this.active_item.as_ref() {
                            cx.emit(ToolbarItemEvent::ChangeLocation(
                                active_item.breadcrumb_location(cx),
                            ))
                        }
                    })
                    .ok();
                }
            }),
        ));
        self.active_item = Some(item.boxed_clone());
        item.breadcrumb_location(cx)
    }

    fn pane_focus_update(
        &mut self,
        pane_focused: bool,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        self.pane_focused = pane_focused;
    }
}
