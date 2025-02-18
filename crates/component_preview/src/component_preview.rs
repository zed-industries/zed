//! # Component Preview
//!
//! A view for exploring Zed components.

use component::{components, ComponentMetadata};
use gpui::{list, prelude::*, uniform_list, App, EventEmitter, FocusHandle, Focusable, Window};
use gpui::{ListState, ScrollHandle, UniformListScrollHandle};
use ui::prelude::*;

use workspace::{item::ItemEvent, Item, Workspace, WorkspaceId};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(
            |workspace, _: &workspace::OpenComponentPreview, window, cx| {
                let component_preview = cx.new(|cx| ComponentPreview::new(window, cx));
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
    view_scroll_handle: ScrollHandle,
    nav_scroll_handle: UniformListScrollHandle,
    components: Vec<ComponentMetadata>,
    component_list: ListState,
}

impl ComponentPreview {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let components = components().all_sorted();
        let initial_length = components.len();
        println!("Initial length: {}", initial_length);

        Self {
            focus_handle: cx.focus_handle(),
            view_scroll_handle: ScrollHandle::new(),
            nav_scroll_handle: UniformListScrollHandle::new(),
            components,
            component_list: ListState::new(initial_length, gpui::ListAlignment::Top, px(500.0), {
                let this = cx.entity().downgrade();
                move |ix, window: &mut Window, cx: &mut App| {
                    this.update(cx, |this, cx| {
                        // div()
                        //     .child(this.components[ix].name().clone())
                        //     .into_any_element()
                        this.render_preview(ix, window, cx).into_any_element()
                    })
                    .unwrap()
                }
            }),
        }
    }

    // fn update_list(self, _window: &Window, _cx: &mut App) -> ListState {
    //     let length = components().all_sorted().len();

    //     ListState::new(
    //         length,
    //         gpui::ListAlignment::Top,
    //         px(500.0),
    //         move |ix, _, _| self.render_sidebar_entry(ix).into_any_element(),
    //     )
    // }

    // fn render_sidebar(&self, window: &Window, cx: &Context<Self>) -> impl IntoElement {
    //     let components = components().all_sorted();

    //     List::new("component-list")
    //         .track_scroll()
    //         .on_scroll(
    //             cx.listener(|this, event, cx| {
    //                 this.component_list.scroll_to(event.top_offset_y, cx)
    //             }),
    //         )
    //         .on_scroll_to_item(
    //             cx.listener(|this, index, cx| this.component_list.scroll_to_item(index, true, cx)),
    //         )
    //         .state(&self.component_list)
    //         .item_size(Pixels(24.0))
    //         .child(ListHeader::new("").child(Label::new("Components").size(LabelSize::Small)))
    //         .items(components.iter().enumerate().map(|(index, component)| {
    //             ListItem::new(index).child(self.render_sidebar_entry(component, cx))
    //         }))
    // }

    fn get_component(&self, ix: usize) -> ComponentMetadata {
        println!("Getting component at index {}", ix);
        println!("Got component name: {}", self.components[ix].name());
        self.components[ix].clone()
    }

    fn render_sidebar_entry(&self, ix: usize, selected: bool) -> impl IntoElement {
        let component = self.get_component(ix);

        Label::new(component.name().clone())
            .size(LabelSize::Small)
            .color(Color::Default)
    }

    fn render_preview(
        &self,
        ix: usize,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let component = self.get_component(ix);

        let name = component.name();
        let scope = component.scope();

        let description = component.description();

        v_flex()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .w_full()
            .gap_3()
            .py_6()
            .flex_none()
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

    // fn render_previews(&self, window: &mut Window, cx: &Context<Self>) -> impl IntoElement {
    //     v_flex()
    //         .id("component-previews")
    //         .size_full()
    //         .overflow_y_scroll()
    //         .p_4()
    //         .gap_4()
    //         .children(
    //             components()
    //                 .all_previews_sorted()
    //                 .iter()
    //                 .map(|component| self.render_preview(component, window, cx)),
    //         )
    // }
}

impl Render for ComponentPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        h_flex()
            .id("component-preview")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .track_focus(&self.focus_handle)
            .px_2()
            .bg(cx.theme().colors().editor_background)
            .debug_below()
            .child(
                uniform_list(
                    cx.entity().clone(),
                    "component-nav",
                    self.components.len(),
                    move |this, range, _window, _cx| {
                        range
                            .map(|ix| {
                                div().w_full().h_8().child(this.get_component(ix).name())
                                // this.render_sidebar_entry(ix, false)
                            })
                            .collect()
                    },
                )
                .track_scroll(self.nav_scroll_handle.clone())
                .flex_grow(),
            )
            .child(
                v_flex().id("component-list").size_full().child(
                    list(self.component_list.clone())
                        .flex_grow()
                        .with_sizing_behavior(gpui::ListSizingBehavior::Auto),
                ),
            )
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| Self::new(window, cx)))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}
