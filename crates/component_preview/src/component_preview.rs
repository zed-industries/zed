//! # Component Preview
//!
//! A view for exploring Zed components.

use component_system::{components, AllComponents, ComponentPreview as _};
use gpui::{prelude::*, AppContext, EventEmitter, FocusHandle, FocusableView};
use strum::{EnumIter, IntoEnumIterator};
use ui::{prelude::*, Avatar, TintColor};

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, EnumIter)]
enum ComponentPreviewPage {
    Overview,
}

impl ComponentPreviewPage {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Overview => "Overview",
        }
    }
}

struct ComponentPreview {
    current_page: ComponentPreviewPage,
    focus_handle: FocusHandle,
}

impl ComponentPreview {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            current_page: ComponentPreviewPage::Overview,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(
        &self,
        page: ComponentPreviewPage,
        cx: &mut ViewContext<ComponentPreview>,
    ) -> impl IntoElement {
        match page {
            ComponentPreviewPage::Overview => self.render_overview_page(cx).into_any_element(),
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
        div()
    }

    fn render_overview_page(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        // let all_previews = get_all_component_previews();

        v_flex()
            .id("component-preview-overview")
            .overflow_scroll()
            .size_full()
            .gap_2()
            .child(v_flex().child(Headline::new("Component Preview").size(HeadlineSize::Large)))
            .child(self.render_sidebar(cx))
        // .children(all_previews.into_iter().map(|(name, preview)| {
        //     let id = ElementId::Name(format!("{}-preview", name).into());
        //     v_flex()
        //         .gap_4()
        //         .child(Headline::new(preview_name).size(HeadlineSize::Small))
        //         .child(
        //             // TODO: We should get preview functions from all_previews,
        //             // not just strings so we don't have to do this match
        //             div().id(id).child(match preview_name {
        //                 "Avatar" => Avatar::preview(cx),
        //                 _ => div()
        //                     .child(format!("Preview not implemented for {}", preview_name))
        //                     .into_any_element(),
        //             }),
        //         )
        // }))
    }

    fn render_page_nav(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .id("component-preview-nav")
            .items_center()
            .gap_4()
            .py_2()
            .bg(cx.theme().colors().editor_background)
            .children(ComponentPreviewPage::iter().map(|p| {
                Button::new(ElementId::Name(p.name().into()), p.name())
                    .on_click(cx.listener(move |this, _, cx| {
                        this.current_page = p;
                        cx.notify();
                    }))
                    .selected(p == self.current_page)
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            }))
    }
}

impl Render for ComponentPreview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("component-preview")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .max_h_full()
            .track_focus(&self.focus_handle)
            .px_2()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_page_nav(cx))
            .child(self.view(self.current_page, cx))
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
