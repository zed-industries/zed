//! # Component Preview
//!
//! A view for exploring Zed components.

#![allow(unused, dead_code)]
use component_system::{get_all_component_previews, ComponentPreview as _};
use gpui::{
    actions, hsla, Action, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, Hsla,
};
use strum::IntoEnumIterator;
use ui::{
    element_cell, prelude::*, string_cell, utils::calculate_contrast_ratio, AudioStatus,
    Availability, Avatar, AvatarAudioStatusIndicator, AvatarAvailabilityIndicator, ButtonLike,
    Checkbox, CheckboxWithLabel, ContentGroup, DecoratedIcon, ElevationIndex, Facepile,
    IconDecoration, Indicator, Table, TintColor, Tooltip,
};

actions!(component_preview, []);

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

// pub fn show_component_preview(
//     app_state: Arc<AppState>,
//     cx: &mut AppContext,
// ) -> Task<anyhow::Result<()>> {
//     open_new(Default::default(), app_state, cx, |workspace, cx| {
//         workspace.toggle_dock(DockPosition::Left, cx);
//         let welcome_page = WelcomePage::new(workspace, cx);
//         workspace.add_item_to_center(Box::new(welcome_page.clone()), cx);
//         cx.focus_view(&welcome_page);
//         cx.notify();

//         db::write_and_log(cx, || {
//             KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
//         });
//     })
// }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, strum::EnumIter)]
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
}

impl EventEmitter<ItemEvent> for ComponentPreview {}

impl FocusableView for ComponentPreview {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ComponentPreview {
    type Event = ItemEvent;

    fn tab_content_text(&self, cx: &WindowContext) -> Option<SharedString> {
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

const AVATAR_URL: &str = "https://avatars.githubusercontent.com/u/1714999?v=4";

impl ComponentPreview {
    fn preview_bg(cx: &WindowContext) -> Hsla {
        cx.theme().colors().editor_background
    }

    fn render_overview_page(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let all_previews = get_all_component_previews();

        v_flex()
            .id("component-preview-overview")
            .overflow_scroll()
            .size_full()
            .gap_2()
            .child(v_flex().child(Headline::new("Component Preview").size(HeadlineSize::Large)))
            .children(all_previews.into_iter().map(|preview_name| {
                let id = ElementId::Name(format!("{}-preview", preview_name).into());
                v_flex()
                    .gap_4()
                    .child(Headline::new(preview_name).size(HeadlineSize::Small))
                    .child(
                        div().id(id).child(match preview_name {
                            "Avatar" => Avatar::preview(cx),
                            // Add other component preview matches here
                            _ => div()
                                .child(format!("Preview not implemented for {}", preview_name))
                                .into_any_element(),
                        }),
                    )
            }))
    }

    fn render_page_nav(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .id("component-preview-nav")
            .items_center()
            .gap_4()
            .py_2()
            .bg(Self::preview_bg(cx))
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl ui::IntoElement {
        v_flex()
            .id("component-preview")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .max_h_full()
            .track_focus(&self.focus_handle)
            .px_2()
            .bg(Self::preview_bg(cx))
            .child(self.render_page_nav(cx))
            .child(self.view(self.current_page, cx))
    }
}
