//! # Component Preview
//!
//! A view for exploring Zed components.

use std::iter::Iterator;
use std::sync::Arc;

use client::UserStore;
use component::{components, ComponentMetadata};
use gpui::{
    list, prelude::*, uniform_list, App, Entity, EventEmitter, FocusHandle, Focusable, Task,
    WeakEntity, Window,
};

use gpui::{ListState, ScrollHandle, UniformListScrollHandle};
use languages::LanguageRegistry;
use notifications::status_toast::{StatusToast, ToastIcon};
use project::Project;
use ui::{prelude::*, Divider, ListItem, ListSubHeader};

use workspace::{item::ItemEvent, Item, Workspace, WorkspaceId};
use workspace::{AppState, ItemId, SerializableItem};

pub fn init(app_state: Arc<AppState>, cx: &mut App) {
    let app_state = app_state.clone();

    cx.observe_new(move |workspace: &mut Workspace, _, cx| {
        let app_state = app_state.clone();
        let weak_workspace = cx.entity().downgrade();

        workspace.register_action(
            move |workspace, _: &workspace::OpenComponentPreview, window, cx| {
                let app_state = app_state.clone();

                let language_registry = app_state.languages.clone();
                let user_store = app_state.user_store.clone();

                let component_preview = cx.new(|cx| {
                    ComponentPreview::new(
                        weak_workspace.clone(),
                        language_registry,
                        user_store,
                        None,
                        cx,
                    )
                });

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

enum PreviewEntry {
    Component(ComponentMetadata),
    SectionHeader(SharedString),
}

impl From<ComponentMetadata> for PreviewEntry {
    fn from(component: ComponentMetadata) -> Self {
        PreviewEntry::Component(component)
    }
}

impl From<SharedString> for PreviewEntry {
    fn from(section_header: SharedString) -> Self {
        PreviewEntry::SectionHeader(section_header)
    }
}

struct ComponentPreview {
    focus_handle: FocusHandle,
    _view_scroll_handle: ScrollHandle,
    nav_scroll_handle: UniformListScrollHandle,
    components: Vec<ComponentMetadata>,
    component_list: ListState,
    selected_index: usize,
    language_registry: Arc<LanguageRegistry>,
    workspace: WeakEntity<Workspace>,
    user_store: Entity<UserStore>,
}

impl ComponentPreview {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        user_store: Entity<UserStore>,
        selected_index: impl Into<Option<usize>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let components = components().all_sorted();
        let initial_length = components.len();
        let selected_index = selected_index.into().unwrap_or(0);

        let component_list =
            ListState::new(initial_length, gpui::ListAlignment::Top, px(1500.0), {
                let this = cx.entity().downgrade();
                move |ix, window: &mut Window, cx: &mut App| {
                    this.update(cx, |this, cx| {
                        let component = this.get_component(ix);
                        this.render_preview(ix, &component, window, cx)
                            .into_any_element()
                    })
                    .unwrap()
                }
            });

        let mut component_preview = Self {
            focus_handle: cx.focus_handle(),
            _view_scroll_handle: ScrollHandle::new(),
            nav_scroll_handle: UniformListScrollHandle::new(),
            language_registry,
            user_store,
            workspace,
            components,
            component_list,
            selected_index,
        };

        if component_preview.selected_index > 0 {
            component_preview.scroll_to_preview(component_preview.selected_index, cx);
        }

        component_preview.update_component_list(cx);

        component_preview
    }

    fn scroll_to_preview(&mut self, ix: usize, cx: &mut Context<Self>) {
        self.component_list.scroll_to_reveal_item(ix);
        self.selected_index = ix;
        cx.notify();
    }

    fn get_component(&self, ix: usize) -> ComponentMetadata {
        self.components[ix].clone()
    }

    fn scope_ordered_entries(&self) -> Vec<PreviewEntry> {
        use std::collections::HashMap;

        // Group components by scope
        let mut scope_groups: HashMap<Option<ComponentScope>, Vec<ComponentMetadata>> =
            HashMap::default();

        for component in &self.components {
            scope_groups
                .entry(component.scope())
                .or_insert_with(Vec::new)
                .push(component.clone());
        }

        // Sort components within each scope by name
        for components in scope_groups.values_mut() {
            components.sort_by_key(|c| c.name().to_lowercase());
        }

        // Build entries with scopes in a defined order
        let mut entries = Vec::new();

        // Define scope order (we want Unknown at the end)
        let known_scopes = [
            ComponentScope::Layout,
            ComponentScope::Input,
            ComponentScope::Editor,
            ComponentScope::Notification,
            ComponentScope::Collaboration,
            ComponentScope::VersionControl,
        ];

        // First add components with known scopes
        for scope in known_scopes.iter() {
            let scope_key = Some(scope.clone());
            if let Some(components) = scope_groups.remove(&scope_key) {
                if !components.is_empty() {
                    // Add section header
                    entries.push(PreviewEntry::SectionHeader(scope.to_string().into()));

                    // Add all components under this scope
                    for component in components {
                        entries.push(PreviewEntry::Component(component));
                    }
                }
            }
        }

        // Handle components with Unknown scope
        for (scope, components) in &scope_groups {
            if let Some(ComponentScope::Unknown(_)) = scope {
                if !components.is_empty() {
                    // Add the unknown scope header
                    if let Some(scope_value) = scope {
                        entries.push(PreviewEntry::SectionHeader(scope_value.to_string().into()));
                    }

                    // Add all components under this unknown scope
                    for component in components {
                        entries.push(PreviewEntry::Component(component.clone()));
                    }
                }
            }
        }

        // Handle components with no scope
        if let Some(components) = scope_groups.get(&None) {
            if !components.is_empty() {
                entries.push(PreviewEntry::SectionHeader("Uncategorized".into()));

                for component in components {
                    entries.push(PreviewEntry::Component(component.clone()));
                }
            }
        }

        entries
    }

    fn render_sidebar_entry(
        &self,
        ix: usize,
        entry: &PreviewEntry,
        selected: bool,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        match entry {
            PreviewEntry::Component(component_metadata) => ListItem::new(ix)
                .child(Label::new(component_metadata.name().clone()).color(Color::Default))
                .selectable(true)
                .toggle_state(selected)
                .inset(true)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.scroll_to_preview(ix, cx);
                }))
                .into_any_element(),
            PreviewEntry::SectionHeader(shared_string) => ListSubHeader::new(shared_string)
                .inset(true)
                .into_any_element(),
        }
    }

    fn update_component_list(&mut self, cx: &mut Context<Self>) {
        let new_len = self.scope_ordered_entries().len();
        let entries = self.scope_ordered_entries();
        let weak_entity = cx.entity().downgrade();

        let new_list = ListState::new(
            new_len,
            gpui::ListAlignment::Top,
            px(1500.0),
            move |ix, window, cx| {
                let entry = &entries[ix];

                weak_entity
                    .update(cx, |this, cx| match entry {
                        PreviewEntry::Component(component) => this
                            .render_preview(ix, component, window, cx)
                            .into_any_element(),
                        PreviewEntry::SectionHeader(shared_string) => this
                            .render_scope_header(ix, shared_string.clone(), window, cx)
                            .into_any_element(),
                    })
                    .unwrap()
            },
        );

        self.component_list = new_list;
    }

    fn render_scope_header(
        &self,
        _ix: usize,
        title: SharedString,
        _window: &Window,
        _cx: &App,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .h_10()
            .items_center()
            .child(Headline::new(title).size(HeadlineSize::XSmall))
            .child(Divider::horizontal())
    }

    fn render_preview(
        &self,
        _ix: usize,
        component: &ComponentMetadata,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let name = component.name();
        let scope = component.scope();

        let description = component.description();

        v_flex()
            .py_2()
            .child(
                v_flex()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_sm()
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

    fn test_status_toast(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let status_toast = StatusToast::new(
                    "`zed/new-notification-system` created!",
                    window,
                    cx,
                    |this, _, cx| {
                        this.icon(ToastIcon::new(IconName::GitBranchSmall).color(Color::Muted))
                            .action(
                                "Open Pull Request",
                                cx.listener(|_, _, _, cx| cx.open_url("https://github.com/")),
                            )
                    },
                );
                workspace.toggle_status_toast(window, cx, status_toast)
            });
        }
    }
}

impl Render for ComponentPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let sidebar_entries = self.scope_ordered_entries();

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
                v_flex()
                    .h_full()
                    .child(
                        uniform_list(
                            cx.entity().clone(),
                            "component-nav",
                            sidebar_entries.len(),
                            move |this, range, _window, cx| {
                                range
                                    .map(|ix| {
                                        this.render_sidebar_entry(
                                            ix,
                                            &sidebar_entries[ix],
                                            ix == this.selected_index,
                                            cx,
                                        )
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(self.nav_scroll_handle.clone())
                        .pt_4()
                        .w(px(240.))
                        .h_full()
                        .flex_1(),
                    )
                    .child(
                        div().w_full().pb_4().child(
                            Button::new("toast-test", "Launch Toast")
                                .on_click(cx.listener({
                                    move |this, _, window, cx| {
                                        this.test_status_toast(window, cx);
                                        cx.notify();
                                    }
                                }))
                                .full_width(),
                        ),
                    ),
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
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Entity<Self>>
    where
        Self: Sized,
    {
        let language_registry = self.language_registry.clone();
        let user_store = self.user_store.clone();
        let weak_workspace = self.workspace.clone();
        let selected_index = self.selected_index;

        Some(cx.new(|cx| {
            Self::new(
                weak_workspace,
                language_registry,
                user_store,
                selected_index,
                cx,
            )
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl SerializableItem for ComponentPreview {
    fn serialized_item_kind() -> &'static str {
        "ComponentPreview"
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        _workspace_id: WorkspaceId,
        _item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        let user_store = project.read(cx).user_store().clone();
        let language_registry = project.read(cx).languages().clone();

        window.spawn(cx, |mut cx| async move {
            let user_store = user_store.clone();
            let language_registry = language_registry.clone();
            let weak_workspace = workspace.clone();
            cx.update(|_, cx| {
                Ok(cx.new(|cx| {
                    ComponentPreview::new(weak_workspace, language_registry, user_store, None, cx)
                }))
            })?
        })
    }

    fn cleanup(
        _workspace_id: WorkspaceId,
        _alive_items: Vec<ItemId>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        Task::ready(Ok(()))
        // window.spawn(cx, |_| {
        // ...
        // })
    }

    fn serialize(
        &mut self,
        _workspace: &mut Workspace,
        _item_id: ItemId,
        _closing: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        // TODO: Serialize the active index so we can re-open to the same place
        None
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}
