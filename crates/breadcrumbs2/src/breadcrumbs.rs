use gpui::{
    Div, Element, EventEmitter, IntoElement, ParentElement, Render, StyledText, Subscription,
    ViewContext, WeakView,
};
use itertools::Itertools;
use theme::ActiveTheme;
use ui::{prelude::*, ButtonLike, ButtonStyle, Label, Tooltip};
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
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let element = h_stack().text_ui();

        let Some(active_item) = &self
            .active_item
            .as_ref()
            .filter(|item| item.downcast::<editor::Editor>().is_some())
        else {
            return element;
        };

        let Some(segments) = active_item.breadcrumbs(cx.theme(), cx) else {
            return element;
        };

        let highlighted_segments = segments.into_iter().map(|segment| {
            StyledText::new(segment.text)
                .with_highlights(&cx.text_style(), segment.highlights.unwrap_or_default())
                .into_any()
        });
        let breadcrumbs = Itertools::intersperse_with(highlighted_segments, || {
            Label::new("›").into_any_element()
        });

        element.child(
            ButtonLike::new("toggle outline view")
                .style(ButtonStyle::Subtle)
                .child(h_stack().gap_1().children(breadcrumbs))
                // We disable the button when the containing pane is not focused:
                //    Because right now all the breadcrumb does is open the outline view, which is an
                //    action which operates on the active editor, clicking the breadcrumbs of another
                //    editor could cause weirdness. I remember that at one point it actually caused a
                //    panic weirdly.
                //
                //    It might be possible that with changes around how focus is managed that we
                //    might be able to update the active editor to the one with the breadcrumbs
                //    clicked on? That or we could just add a code path for being able to open the
                //    outline for a specific editor. Long term we'd like for it to be an actual
                //    breadcrumb bar so that problem goes away
                //
                //   — Julia (https://github.com/zed-industries/zed/pull/3505#pullrequestreview-1766198050)
                .disabled(!self.pane_focused)
                .on_click(cx.listener(|breadcrumbs, _, cx| {
                    if let Some(workspace) = breadcrumbs.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            outline::toggle(workspace, &outline::Toggle, cx)
                        })
                    }
                }))
                .tooltip(|cx| Tooltip::for_action("Show symbol outline", &outline::Toggle, cx)),
        )
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
