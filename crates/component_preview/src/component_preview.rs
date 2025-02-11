//! # Component Preview
//!
//! A view for exploring Zed components.

use component::{components, ComponentMetadata};
use gpui::{prelude::*, App, EventEmitter, FocusHandle, Focusable, Window};
use ui::prelude::*;

use workspace::{item::ItemEvent, Item, Workspace, WorkspaceId};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(
            |workspace, _: &workspace::OpenComponentPreview, window, cx| {
                let component_preview = cx.new(ComponentPreview::new);
                workspace.add_item_to_active_pane(
                    Box::new(component_preview),
                    None,
                    true,
                    window,
                    cx,
                )
            },
        );
    })
    .detach();
}

struct ComponentPreview {
    focus_handle: FocusHandle,
}

impl ComponentPreview {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn render_sidebar(&self, _window: &Window, _cx: &Context<Self>) -> impl IntoElement {
        let components = components().all_sorted();
        let sorted_components = components.clone();

        v_flex()
            .max_w_48()
            .gap_px()
            .p_1()
            .children(
                sorted_components
                    .into_iter()
                    .map(|component| self.render_sidebar_entry(&component, _cx)),
            )
            .child(
                Label::new("These will be clickable once the layout is moved to a gpui::List.")
                    .color(Color::Muted)
                    .size(LabelSize::XSmall)
                    .italic(),
            )
    }

    fn render_sidebar_entry(
        &self,
        component: &ComponentMetadata,
        _cx: &Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .w_40()
            .px_1p5()
            .py_0p5()
            .text_sm()
            .child(component.name().clone())
    }

    fn render_preview(
        &self,
        component: &ComponentMetadata,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let name = component.name();
        let scope = component.scope();

        let description = component.description();

        v_flex()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .w_full()
            .gap_3()
            .py_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        h_flex()
                            .gap_1()
                            .text_2xl()
                            .child(div().child(name))
                            .when_some(scope, |this, scope| {
                                this.child(div().opacity(0.5).child(format!("({})", scope)))
                            }),
                    )
                    .when_some(description, |this, description| {
                        this.child(
                            div()
                                .text_ui_sm(cx)
                                .text_color(cx.theme().colors().text_muted)
                                .max_w(px(600.0))
                                .child(description),
                        )
                    }),
            )
            .when_some(component.preview(), |this, preview| {
                this.child(preview(window, cx))
            })
            .into_any_element()
    }

    fn render_previews(&self, window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .id("component-previews")
            .size_full()
            .overflow_y_scroll()
            .p_4()
            .gap_4()
            .children(
                components()
                    .all_previews_sorted()
                    .iter()
                    .map(|component| self.render_preview(component, window, cx)),
            )
    }
}

impl Render for ComponentPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
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
            .child(self.render_sidebar(window, cx))
            .child(self.render_previews(window, cx))
    }
}

impl EventEmitter<ItemEvent> for ComponentPreview {}

impl Focusable for ComponentPreview {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ComponentPreview {
    type Event = ItemEvent;

    fn tab_content_text(&self, _window: &Window, _cx: &App) -> Option<SharedString> {
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
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(Self::new))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
