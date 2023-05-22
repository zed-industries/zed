use gpui::{
    elements::*, platform::MouseButton, AppContext, Entity, Subscription, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
use itertools::Itertools;
use search::ProjectSearchView;
use workspace::{
    item::{ItemEvent, ItemHandle},
    ToolbarItemLocation, ToolbarItemView, Workspace,
};

pub enum Event {
    UpdateLocation,
}

pub struct Breadcrumbs {
    pane_focused: bool,
    active_item: Option<Box<dyn ItemHandle>>,
    project_search: Option<ViewHandle<ProjectSearchView>>,
    subscription: Option<Subscription>,
    workspace: WeakViewHandle<Workspace>,
}

impl Breadcrumbs {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            pane_focused: false,
            active_item: Default::default(),
            subscription: Default::default(),
            project_search: Default::default(),
            workspace: workspace.weak_handle(),
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let active_item = match &self.active_item {
            Some(active_item) => active_item,
            None => return Empty::new().into_any(),
        };
        let not_editor = active_item.downcast::<editor::Editor>().is_none();

        let theme = theme::current(cx).clone();
        let style = &theme.workspace.breadcrumbs;

        let breadcrumbs = match active_item.breadcrumbs(&theme, cx) {
            Some(breadcrumbs) => breadcrumbs,
            None => return Empty::new().into_any(),
        }
        .into_iter()
        .map(|breadcrumb| {
            Text::new(
                breadcrumb.text,
                theme.workspace.breadcrumbs.default.text.clone(),
            )
            .with_highlights(breadcrumb.highlights.unwrap_or_default())
            .into_any()
        });

        let crumbs = Flex::row()
            .with_children(Itertools::intersperse_with(breadcrumbs, || {
                Label::new(" 〉 ", style.default.text.clone()).into_any()
            }))
            .constrained()
            .with_height(theme.workspace.breadcrumb_height)
            .contained();

        if not_editor || !self.pane_focused {
            return crumbs
                .with_style(style.default.container)
                .aligned()
                .left()
                .into_any();
        }

        MouseEventHandler::<Breadcrumbs, Breadcrumbs>::new(0, cx, |state, _| {
            let style = style.style_for(state, false);
            crumbs.with_style(style.container)
        })
        .on_click(MouseButton::Left, |_, this, cx| {
            if let Some(workspace) = this.workspace.upgrade(cx) {
                workspace.update(cx, |workspace, cx| {
                    outline::toggle(workspace, &Default::default(), cx)
                })
            }
        })
        .with_tooltip::<Breadcrumbs>(
            0,
            "Show symbol outline".to_owned(),
            Some(Box::new(outline::Toggle)),
            theme.tooltip.clone(),
            cx,
        )
        .aligned()
        .left()
        .into_any()
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

    fn pane_focus_update(&mut self, pane_focused: bool, _: &mut ViewContext<Self>) {
        self.pane_focused = pane_focused;
    }
}
