use gpui::{
    elements::*, AppContext, Entity, RenderContext, Subscription, View, ViewContext, ViewHandle,
};
use itertools::Itertools;
use search::ProjectSearchView;
use settings::Settings;
use workspace::{
    item::{ItemEvent, ItemHandle},
    ToolbarItemLocation, ToolbarItemView,
};

pub enum Event {
    UpdateLocation,
}

pub struct Breadcrumbs {
    active_item: Option<Box<dyn ItemHandle>>,
    project_search: Option<ViewHandle<ProjectSearchView>>,
    subscription: Option<Subscription>,
}

impl Breadcrumbs {
    pub fn new() -> Self {
        Self {
            active_item: Default::default(),
            subscription: Default::default(),
            project_search: Default::default(),
        }
    }
}

impl Entity for Breadcrumbs {
    type Event = Event;
}

impl View for Breadcrumbs {
    fn ui_name() -> &'static str {
        "Breadcrumbs"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        if let Some(breadcrumbs) = self
            .active_item
            .as_ref()
            .and_then(|item| item.breadcrumbs(&theme, cx))
        {
            Flex::row()
                .with_children(Itertools::intersperse_with(breadcrumbs.into_iter(), || {
                    Label::new(" 〉 ".to_string(), theme.breadcrumbs.text.clone()).boxed()
                }))
                .contained()
                .with_style(theme.breadcrumbs.container)
                .aligned()
                .left()
                .boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl ToolbarItemView for Breadcrumbs {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_item = None;
        self.project_search = None;
        if let Some(item) = active_pane_item {
            let this = cx.weak_handle();
            self.subscription = Some(item.subscribe_to_item_events(
                cx,
                Box::new(move |event, cx| {
                    if let Some(this) = this.upgrade(cx) {
                        if let ItemEvent::UpdateBreadcrumbs = event {
                            this.update(cx, |_, cx| {
                                cx.emit(Event::UpdateLocation);
                                cx.notify();
                            });
                        }
                    }
                }),
            ));
            self.active_item = Some(item.boxed_clone());
            item.breadcrumb_location(cx)
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn location_for_event(
        &self,
        _: &Event,
        current_location: ToolbarItemLocation,
        cx: &AppContext,
    ) -> ToolbarItemLocation {
        if let Some(active_item) = self.active_item.as_ref() {
            active_item.breadcrumb_location(cx)
        } else {
            current_location
        }
    }
}
