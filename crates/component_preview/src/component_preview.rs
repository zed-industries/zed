//! # Component Preview
//!
//! A view for exploring Zed components.

use component::{components, ComponentMetadata};
use gpui::{list, prelude::*, uniform_list, App, EventEmitter, FocusHandle, Focusable, Window};
use gpui::{ListState, ScrollHandle, UniformListScrollHandle};
use ui::{prelude::*, ListItem};

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
    _view_scroll_handle: ScrollHandle,
    nav_scroll_handle: UniformListScrollHandle,
    components: Vec<ComponentMetadata>,
    component_list: ListState,
    selected_index: usize,
}

impl ComponentPreview {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let components = components().all_sorted();
        let initial_length = components.len();

        let component_list = ListState::new(initial_length, gpui::ListAlignment::Top, px(500.0), {
            let this = cx.entity().downgrade();
            move |ix, window: &mut Window, cx: &mut App| {
                this.update(cx, |this, cx| {
                    this.render_preview(ix, window, cx).into_any_element()
                })
                .unwrap()
            }
        });

        Self {
            focus_handle: cx.focus_handle(),
            _view_scroll_handle: ScrollHandle::new(),
            nav_scroll_handle: UniformListScrollHandle::new(),
            components,
            component_list,
            selected_index: 0,
        }
    }

    fn scroll_to_preview(&mut self, ix: usize, cx: &mut Context<Self>) {
        self.component_list.scroll_to_reveal_item(ix);
        self.selected_index = ix;
        cx.notify();
    }

    fn get_component(&self, ix: usize) -> ComponentMetadata {
        self.components[ix].clone()
    }

    fn render_sidebar_entry(
        &self,
        ix: usize,
        selected: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let component = self.get_component(ix);

        ListItem::new(ix)
            .child(Label::new(component.name().clone()).color(Color::Default))
            .selectable(true)
            .toggle_state(selected)
            .inset(true)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.scroll_to_preview(ix, cx);
            }))
    }

    fn render_preview(
        &self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let component = self.get_component(ix);

        let name = component.name();
        let scope = component.scope();

        let description = component.description();

        v_flex()
            .py_2()
            .child(
                v_flex()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .w_full()
                    .gap_4()
                    .py_4()
                    .px_6()
                    .flex_none()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .text_xl()
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
                    }),
            )
            .into_any_element()
    }
}

impl Render for ComponentPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        h_flex()
            .id("component-preview")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .track_focus(&self.focus_handle)
            .px_2()
            .bg(cx.theme().colors().editor_background)
            .child(
                uniform_list(
                    cx.entity().clone(),
                    "component-nav",
                    self.components.len(),
                    move |this, range, _window, cx| {
                        range
                            .map(|ix| this.render_sidebar_entry(ix, ix == this.selected_index, cx))
                            .collect()
                    },
                )
                .track_scroll(self.nav_scroll_handle.clone())
                .pt_4()
                .w(px(240.))
                .h_full()
                .flex_grow(),
            )
            .child(
                v_flex()
                    .id("component-list")
                    .px_8()
                    .pt_4()
                    .size_full()
                    .child(
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

// TODO: impl serializable item for component preview so it will restore with the workspace
// ref: https://github.com/zed-industries/zed/blob/32201ac70a501e63dfa2ade9c00f85aea2d4dd94/crates/image_viewer/src/image_viewer.rs#L199
// Use `ImageViewer` as a model for how to do it, except it'll be even simpler
