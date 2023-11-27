use gpui::{
    div, Div, Element, EventEmitter, InteractiveElement, IntoElement, ParentElement, Render,
    Stateful, StatefulInteractiveElement, StyledText, Subscription, ViewContext, WeakView,
};
use itertools::Itertools;
use theme::ActiveTheme;
use ui::{h_stack, Label};
use workspace::{
    item::{ItemEvent, ItemHandle},
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

pub enum Event {
    UpdateLocation,
}

pub struct Breadcrumbs {
    pane_focused: bool,
    active_item: Option<Box<dyn ItemHandle>>,
    subscription: Option<Subscription>,
    workspace: WeakView<Workspace>,
}

impl Breadcrumbs {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            pane_focused: false,
            active_item: Default::default(),
            subscription: Default::default(),
            workspace: workspace.weak_handle(),
        }
    }
}

impl EventEmitter<Event> for Breadcrumbs {}
impl EventEmitter<ToolbarItemEvent> for Breadcrumbs {}

impl Render for Breadcrumbs {
    type Element = Stateful<Div>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let id = "breadcrumbs";
        let default_style = cx.text_style();

        let active_item = match &self.active_item {
            Some(active_item) => active_item,
            None => return div().id(id),
        };
        let not_editor = active_item.downcast::<editor::Editor>().is_none();

        let breadcrumbs = match active_item.breadcrumbs(cx.theme(), cx) {
            Some(breadcrumbs) => breadcrumbs,
            None => return div().id(id),
        }
        .into_iter()
        .map(|breadcrumb| {
            StyledText::new(breadcrumb.text)
                .with_highlights(&default_style, breadcrumb.highlights.unwrap_or_default())
                .into_any()
        });

        let crumbs = h_stack().children(Itertools::intersperse_with(breadcrumbs, || {
            Label::new(" › ").into_any_element()
        }));

        if not_editor || !self.pane_focused {
            return crumbs.id(id);
        }

        let this = cx.view().downgrade();
        crumbs.id(id).on_click(move |_, cx| {
            this.update(cx, |this, cx| {
                if let Some(workspace) = this.workspace.upgrade() {
                    workspace.update(cx, |_workspace, _cx| {
                        todo!("outline::toggle");
                        // outline::toggle(workspace, &Default::default(), cx)
                    })
                }
            })
            .ok();
        })
    }
}

// impl View for Breadcrumbs {
//     fn ui_name() -> &'static str {
//         "Breadcrumbs"
//     }

//     fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
//         let active_item = match &self.active_item {
//             Some(active_item) => active_item,
//             None => return Empty::new().into_any(),
//         };
//         let not_editor = active_item.downcast::<editor::Editor>().is_none();

//         let theme = theme::current(cx).clone();
//         let style = &theme.workspace.toolbar.breadcrumbs;

//         let breadcrumbs = match active_item.breadcrumbs(&theme, cx) {
//             Some(breadcrumbs) => breadcrumbs,
//             None => return Empty::new().into_any(),
//         }
//         .into_iter()
//         .map(|breadcrumb| {
//             Text::new(
//                 breadcrumb.text,
//                 theme.workspace.toolbar.breadcrumbs.default.text.clone(),
//             )
//             .with_highlights(breadcrumb.highlights.unwrap_or_default())
//             .into_any()
//         });

//         let crumbs = Flex::row()
//             .with_children(Itertools::intersperse_with(breadcrumbs, || {
//                 Label::new(" › ", style.default.text.clone()).into_any()
//             }))
//             .constrained()
//             .with_height(theme.workspace.toolbar.breadcrumb_height)
//             .contained();

//         if not_editor || !self.pane_focused {
//             return crumbs
//                 .with_style(style.default.container)
//                 .aligned()
//                 .left()
//                 .into_any();
//         }

//         MouseEventHandler::new::<Breadcrumbs, _>(0, cx, |state, _| {
//             let style = style.style_for(state);
//             crumbs.with_style(style.container)
//         })
//         .on_click(MouseButton::Left, |_, this, cx| {
//             if let Some(workspace) = this.workspace.upgrade(cx) {
//                 workspace.update(cx, |workspace, cx| {
//                     outline::toggle(workspace, &Default::default(), cx)
//                 })
//             }
//         })
//         .with_tooltip::<Breadcrumbs>(
//             0,
//             "Show symbol outline".to_owned(),
//             Some(Box::new(outline::Toggle)),
//             theme.tooltip.clone(),
//             cx,
//         )
//         .aligned()
//         .left()
//         .into_any()
//     }
// }

impl ToolbarItemView for Breadcrumbs {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.active_item = None;
        if let Some(item) = active_pane_item {
            let this = cx.view().downgrade();
            self.subscription = Some(item.subscribe_to_item_events(
                cx,
                Box::new(move |event, cx| {
                    if let ItemEvent::UpdateBreadcrumbs = event {
                        this.update(cx, |_, cx| {
                            cx.emit(Event::UpdateLocation);
                            cx.notify();
                        })
                        .ok();
                    }
                }),
            ));
            self.active_item = Some(item.boxed_clone());
            item.breadcrumb_location(cx)
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    // fn location_for_event(
    //     &self,
    //     _: &Event,
    //     current_location: ToolbarItemLocation,
    //     cx: &AppContext,
    // ) -> ToolbarItemLocation {
    //     if let Some(active_item) = self.active_item.as_ref() {
    //         active_item.breadcrumb_location(cx)
    //     } else {
    //         current_location
    //     }
    // }

    fn pane_focus_update(&mut self, pane_focused: bool, _: &mut ViewContext<Self>) {
        self.pane_focused = pane_focused;
    }
}
