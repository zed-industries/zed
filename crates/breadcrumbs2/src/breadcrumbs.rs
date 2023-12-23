use editor::Editor;
use gpui::{
    Div, Element, EventEmitter, IntoElement, ParentElement, Render, StyledText, Subscription,
    ViewContext,
};
use itertools::Itertools;
use theme::ActiveTheme;
use ui::{prelude::*, ButtonLike, ButtonStyle, Label, Tooltip};
use workspace::{
    item::{ItemEvent, ItemHandle},
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView,
};

pub enum Event {
    UpdateLocation,
}

pub struct Breadcrumbs {
    pane_focused: bool,
    active_item: Option<Box<dyn ItemHandle>>,
    subscription: Option<Subscription>,
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
            let mut text_style = cx.text_style();
            text_style.color = Color::Muted.color(cx);

            StyledText::new(segment.text)
                .with_highlights(&text_style, segment.highlights.unwrap_or_default())
                .into_any()
        });
        let breadcrumbs = Itertools::intersperse_with(highlighted_segments, || {
            Label::new("›").color(Color::Muted).into_any_element()
        });

        let editor = active_item
            .downcast::<Editor>()
            .map(|editor| editor.downgrade());

        element.child(
            ButtonLike::new("toggle outline view")
                .style(ButtonStyle::Subtle)
                .child(h_stack().gap_1().children(breadcrumbs))
                .on_click(move |_, cx| {
                    if let Some(editor) = editor.as_ref().and_then(|editor| editor.upgrade()) {
                        outline::toggle(editor, &outline::Toggle, cx)
                    }
                })
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
