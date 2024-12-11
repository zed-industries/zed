//! # Component Preview
//!
//! A view for exploring Zed components.

use component_system::components;
use gpui::{prelude::*, AppContext, EventEmitter, FocusHandle, FocusableView};
use strum::{EnumIter, IntoEnumIterator};
use ui::{prelude::*, TintColor};

use workspace::{item::ItemEvent, Item, Workspace, WorkspaceId};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _cx| {
        workspace.register_action(|workspace, _: &workspace::ComponentPreview, cx| {
            let component_preview = cx.new_view(ComponentPreview::new);
            workspace.add_item_to_active_pane(Box::new(component_preview), None, true, cx)
        });
    })
    .detach();
}

struct ComponentPreview {
    focus_handle: FocusHandle,
}

impl ComponentPreview {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn render_sidebar(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        h_flex().children(components().all().iter().map(|component| {
            Button::new(component.name().clone(), component.name()).on_click(cx.listener(
                move |_this, _, _cx| {
                    // Handle button click
                },
            ))
        }))
    }

    fn render_preview(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .children(components().all_previews().iter().map(|component| {
                if let Some(preview) = component.preview() {
                    preview(cx)
                } else {
                    div().into_any_element()
                }
            }))
    }
}

impl Render for ComponentPreview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .id("component-preview")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .max_h_full()
            .track_focus(&self.focus_handle)
            .px_2()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_sidebar(cx))
            .child(self.render_preview(cx))
    }
}

impl EventEmitter<ItemEvent> for ComponentPreview {}

impl FocusableView for ComponentPreview {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ComponentPreview {
    type Event = ItemEvent;

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("Component Preview".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(Self::new))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
