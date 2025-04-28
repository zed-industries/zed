//! # Component Preview
//!
//! A view for exploring Zed components.

mod persistence;

use std::iter::Iterator;
use std::sync::Arc;

use client::UserStore;
use component::{ComponentId, ComponentMetadata, components};
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity, Window, list, prelude::*,
};

use collections::HashMap;

use gpui::{ListState, ScrollHandle, ScrollStrategy, UniformListScrollHandle};
use languages::LanguageRegistry;
use notifications::status_toast::{StatusToast, ToastIcon};
use persistence::COMPONENT_PREVIEW_DB;
use project::Project;
use ui::{Divider, HighlightedLabel, ListItem, ListSubHeader, prelude::*};

use ui_input::SingleLineInput;
use workspace::{AppState, ItemId, SerializableItem, delete_unloaded_items};
use workspace::{Item, Workspace, WorkspaceId, item::ItemEvent};

pub fn init(app_state: Arc<AppState>, cx: &mut App) {
    workspace::register_serializable_item::<ComponentPreview>(cx);

    let app_state = app_state.clone();

    cx.observe_new(move |workspace: &mut Workspace, _window, cx| {
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
                        None,
                        window,
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
    AllComponents,
    Separator,
    Component(ComponentMetadata, Option<Vec<usize>>),
    SectionHeader(SharedString),
}

impl From<ComponentMetadata> for PreviewEntry {
    fn from(component: ComponentMetadata) -> Self {
        PreviewEntry::Component(component, None)
    }
}

impl From<SharedString> for PreviewEntry {
    fn from(section_header: SharedString) -> Self {
        PreviewEntry::SectionHeader(section_header)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
enum PreviewPage {
    #[default]
    AllComponents,
    Component(ComponentId),
}

struct ComponentPreview {
    workspace_id: Option<WorkspaceId>,
    focus_handle: FocusHandle,
    _view_scroll_handle: ScrollHandle,
    nav_scroll_handle: UniformListScrollHandle,
    component_map: HashMap<ComponentId, ComponentMetadata>,
    active_page: PreviewPage,
    components: Vec<ComponentMetadata>,
    component_list: ListState,
    cursor_index: usize,
    language_registry: Arc<LanguageRegistry>,
    workspace: WeakEntity<Workspace>,
    user_store: Entity<UserStore>,
    filter_editor: Entity<SingleLineInput>,
    filter_text: String,
}

impl ComponentPreview {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        user_store: Entity<UserStore>,
        selected_index: impl Into<Option<usize>>,
        active_page: Option<PreviewPage>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let sorted_components = components().all_sorted();
        let selected_index = selected_index.into().unwrap_or(0);
        let active_page = active_page.unwrap_or(PreviewPage::AllComponents);
        let filter_editor =
            cx.new(|cx| SingleLineInput::new(window, cx, "Find components or usages…"));

        let component_list = ListState::new(
            sorted_components.len(),
            gpui::ListAlignment::Top,
            px(1500.0),
            {
                let this = cx.entity().downgrade();
                move |ix, window: &mut Window, cx: &mut App| {
                    this.update(cx, |this, cx| {
                        let component = this.get_component(ix);
                        this.render_preview(&component, window, cx)
                            .into_any_element()
                    })
                    .unwrap()
                }
            },
        );

        let mut component_preview = Self {
            workspace_id: None,
            focus_handle: cx.focus_handle(),
            _view_scroll_handle: ScrollHandle::new(),
            nav_scroll_handle: UniformListScrollHandle::new(),
            language_registry,
            user_store,
            workspace,
            active_page,
            component_map: components().0,
            components: sorted_components,
            component_list,
            cursor_index: selected_index,
            filter_editor,
            filter_text: String::new(),
        };

        if component_preview.cursor_index > 0 {
            component_preview.scroll_to_preview(component_preview.cursor_index, cx);
        }

        component_preview.update_component_list(cx);

        let focus_handle = component_preview.filter_editor.read(cx).focus_handle(cx);
        window.focus(&focus_handle);

        component_preview
    }

    pub fn active_page_id(&self, _cx: &App) -> ActivePageId {
        match &self.active_page {
            PreviewPage::AllComponents => ActivePageId::default(),
            PreviewPage::Component(component_id) => ActivePageId(component_id.0.to_string()),
        }
    }

    fn scroll_to_preview(&mut self, ix: usize, cx: &mut Context<Self>) {
        self.component_list.scroll_to_reveal_item(ix);
        self.cursor_index = ix;
        cx.notify();
    }

    fn set_active_page(&mut self, page: PreviewPage, cx: &mut Context<Self>) {
        self.active_page = page;
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    fn get_component(&self, ix: usize) -> ComponentMetadata {
        self.components[ix].clone()
    }

    fn filtered_components(&self) -> Vec<ComponentMetadata> {
        if self.filter_text.is_empty() {
            return self.components.clone();
        }

        let filter = self.filter_text.to_lowercase();
        self.components
            .iter()
            .filter(|component| {
                let component_name = component.name().to_lowercase();
                let scope_name = component.scope().to_string().to_lowercase();
                let description = component
                    .description()
                    .map(|d| d.to_lowercase())
                    .unwrap_or_default();

                component_name.contains(&filter)
                    || scope_name.contains(&filter)
                    || description.contains(&filter)
            })
            .cloned()
            .collect()
    }

    fn scope_ordered_entries(&self) -> Vec<PreviewEntry> {
        use std::collections::HashMap;

        let mut scope_groups: HashMap<
            ComponentScope,
            Vec<(ComponentMetadata, Option<Vec<usize>>)>,
        > = HashMap::default();
        let lowercase_filter = self.filter_text.to_lowercase();

        for component in &self.components {
            if self.filter_text.is_empty() {
                scope_groups
                    .entry(component.scope())
                    .or_insert_with(Vec::new)
                    .push((component.clone(), None));
                continue;
            }

            // let full_component_name = component.name();
            let scopeless_name = component.scopeless_name();
            let scope_name = component.scope().to_string();
            let description = component.description().unwrap_or_default();

            let lowercase_scopeless = scopeless_name.to_lowercase();
            let lowercase_scope = scope_name.to_lowercase();
            let lowercase_desc = description.to_lowercase();

            if lowercase_scopeless.contains(&lowercase_filter) {
                if let Some(index) = lowercase_scopeless.find(&lowercase_filter) {
                    let end = index + lowercase_filter.len();

                    if end <= scopeless_name.len() {
                        let mut positions = Vec::new();
                        for i in index..end {
                            if scopeless_name.is_char_boundary(i) {
                                positions.push(i);
                            }
                        }

                        if !positions.is_empty() {
                            scope_groups
                                .entry(component.scope())
                                .or_insert_with(Vec::new)
                                .push((component.clone(), Some(positions)));
                            continue;
                        }
                    }
                }
            }

            if lowercase_scopeless.contains(&lowercase_filter)
                || lowercase_scope.contains(&lowercase_filter)
                || lowercase_desc.contains(&lowercase_filter)
            {
                scope_groups
                    .entry(component.scope())
                    .or_insert_with(Vec::new)
                    .push((component.clone(), None));
            }
        }

        // Sort the components in each group
        for components in scope_groups.values_mut() {
            components.sort_by_key(|(c, _)| c.sort_name());
        }

        let mut entries = Vec::new();

        // Always show all components first
        entries.push(PreviewEntry::AllComponents);
        entries.push(PreviewEntry::Separator);

        let mut scopes: Vec<_> = scope_groups
            .keys()
            .filter(|scope| !matches!(**scope, ComponentScope::None))
            .cloned()
            .collect();

        scopes.sort_by_key(|s| s.to_string());

        for scope in scopes {
            if let Some(components) = scope_groups.remove(&scope) {
                if !components.is_empty() {
                    entries.push(PreviewEntry::SectionHeader(scope.to_string().into()));
                    let mut sorted_components = components;
                    sorted_components.sort_by_key(|(component, _)| component.sort_name());

                    for (component, positions) in sorted_components {
                        entries.push(PreviewEntry::Component(component, positions));
                    }
                }
            }
        }

        // Add uncategorized components last
        if let Some(components) = scope_groups.get(&ComponentScope::None) {
            if !components.is_empty() {
                entries.push(PreviewEntry::Separator);
                entries.push(PreviewEntry::SectionHeader("Uncategorized".into()));
                let mut sorted_components = components.clone();
                sorted_components.sort_by_key(|(c, _)| c.sort_name());

                for (component, positions) in sorted_components {
                    entries.push(PreviewEntry::Component(component, positions));
                }
            }
        }

        entries
    }

    fn render_sidebar_entry(
        &self,
        ix: usize,
        entry: &PreviewEntry,
        cx: &Context<Self>,
    ) -> impl IntoElement + use<> {
        match entry {
            PreviewEntry::Component(component_metadata, highlight_positions) => {
                let id = component_metadata.id();
                let selected = self.active_page == PreviewPage::Component(id.clone());
                let name = component_metadata.scopeless_name();

                ListItem::new(ix)
                    .child(if let Some(_positions) = highlight_positions {
                        let name_lower = name.to_lowercase();
                        let filter_lower = self.filter_text.to_lowercase();
                        let valid_positions = if let Some(start) = name_lower.find(&filter_lower) {
                            let end = start + filter_lower.len();
                            (start..end).collect()
                        } else {
                            Vec::new()
                        };
                        if valid_positions.is_empty() {
                            Label::new(name.clone())
                                .color(Color::Default)
                                .into_any_element()
                        } else {
                            HighlightedLabel::new(name.clone(), valid_positions).into_any_element()
                        }
                    } else {
                        Label::new(name.clone())
                            .color(Color::Default)
                            .into_any_element()
                    })
                    .selectable(true)
                    .toggle_state(selected)
                    .inset(true)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        let id = id.clone();
                        this.set_active_page(PreviewPage::Component(id), cx);
                    }))
                    .into_any_element()
            }
            PreviewEntry::SectionHeader(shared_string) => ListSubHeader::new(shared_string)
                .inset(true)
                .into_any_element(),
            PreviewEntry::AllComponents => {
                let selected = self.active_page == PreviewPage::AllComponents;

                ListItem::new(ix)
                    .child(Label::new("All Components").color(Color::Default))
                    .selectable(true)
                    .toggle_state(selected)
                    .inset(true)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.set_active_page(PreviewPage::AllComponents, cx);
                    }))
                    .into_any_element()
            }
            PreviewEntry::Separator => ListItem::new(ix)
                .child(
                    h_flex()
                        .occlude()
                        .pt_3()
                        .child(Divider::horizontal_dashed()),
                )
                .into_any_element(),
        }
    }

    fn update_component_list(&mut self, cx: &mut Context<Self>) {
        let entries = self.scope_ordered_entries();
        let new_len = entries.len();
        let weak_entity = cx.entity().downgrade();

        if new_len > 0 {
            self.nav_scroll_handle
                .scroll_to_item(0, ScrollStrategy::Top);
        }

        let filtered_components = self.filtered_components();

        if !self.filter_text.is_empty() && !matches!(self.active_page, PreviewPage::AllComponents) {
            if let PreviewPage::Component(ref component_id) = self.active_page {
                let component_still_visible = filtered_components
                    .iter()
                    .any(|component| component.id() == *component_id);

                if !component_still_visible {
                    if !filtered_components.is_empty() {
                        let first_component = &filtered_components[0];
                        self.set_active_page(PreviewPage::Component(first_component.id()), cx);
                    } else {
                        self.set_active_page(PreviewPage::AllComponents, cx);
                    }
                }
            }
        }

        self.component_list = ListState::new(
            filtered_components.len(),
            gpui::ListAlignment::Top,
            px(1500.0),
            {
                let components = filtered_components.clone();
                let this = cx.entity().downgrade();
                move |ix, window: &mut Window, cx: &mut App| {
                    if ix >= components.len() {
                        return div().w_full().h_0().into_any_element();
                    }

                    this.update(cx, |this, cx| {
                        let component = &components[ix];
                        this.render_preview(component, window, cx)
                            .into_any_element()
                    })
                    .unwrap()
                }
            },
        );

        let new_list = ListState::new(
            new_len,
            gpui::ListAlignment::Top,
            px(1500.0),
            move |ix, window, cx| {
                if ix >= entries.len() {
                    return div().w_full().h_0().into_any_element();
                }

                let entry = &entries[ix];

                weak_entity
                    .update(cx, |this, cx| match entry {
                        PreviewEntry::Component(component, _) => this
                            .render_preview(component, window, cx)
                            .into_any_element(),
                        PreviewEntry::SectionHeader(shared_string) => this
                            .render_scope_header(ix, shared_string.clone(), window, cx)
                            .into_any_element(),
                        PreviewEntry::AllComponents => div().w_full().h_0().into_any_element(),
                        PreviewEntry::Separator => div().w_full().h_0().into_any_element(),
                    })
                    .unwrap()
            },
        );

        self.component_list = new_list;
        cx.emit(ItemEvent::UpdateTab);
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
        component: &ComponentMetadata,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let name = component.scopeless_name();
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
                                h_flex().gap_1().text_xl().child(div().child(name)).when(
                                    !matches!(scope, ComponentScope::None),
                                    |this| {
                                        this.child(div().opacity(0.5).child(format!("({})", scope)))
                                    },
                                ),
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
                        this.children(preview(window, cx))
                    }),
            )
            .into_any_element()
    }

    fn render_all_components(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .id("component-list")
            .px_8()
            .pt_4()
            .size_full()
            .child(
                if self.filtered_components().is_empty() && !self.filter_text.is_empty() {
                    div()
                        .size_full()
                        .items_center()
                        .justify_center()
                        .text_color(cx.theme().colors().text_muted)
                        .child(format!("No components matching '{}'.", self.filter_text))
                        .into_any_element()
                } else {
                    list(self.component_list.clone())
                        .flex_grow()
                        .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                        .into_any_element()
                },
            )
    }

    fn render_component_page(
        &mut self,
        component_id: &ComponentId,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let component = self.component_map.get(&component_id);

        if let Some(component) = component {
            v_flex()
                .id("render-component-page")
                .size_full()
                .child(ComponentPreviewPage::new(component.clone()))
                .into_any_element()
        } else {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child("Component not found")
                .into_any_element()
        }
    }

    fn test_status_toast(&self, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                let status_toast =
                    StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                        this.icon(ToastIcon::new(IconName::GitBranchSmall).color(Color::Muted))
                            .action("Open Pull Request", |_, cx| {
                                cx.open_url("https://github.com/")
                            })
                    });
                workspace.toggle_status_toast(status_toast, cx)
            });
        }
    }
}

impl Render for ComponentPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: move this into the struct
        let current_filter = self.filter_editor.update(cx, |input, cx| {
            if input.is_empty(cx) {
                String::new()
            } else {
                input.editor().read(cx).text(cx).to_string()
            }
        });

        if current_filter != self.filter_text {
            self.filter_text = current_filter;
            self.update_component_list(cx);
        }
        let sidebar_entries = self.scope_ordered_entries();
        let active_page = self.active_page.clone();

        h_flex()
            .id("component-preview")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .track_focus(&self.focus_handle)
            .bg(cx.theme().colors().editor_background)
            .child(
                v_flex()
                    .border_r_1()
                    .border_color(cx.theme().colors().border)
                    .h_full()
                    .child(
                        gpui::uniform_list(
                            cx.entity().clone(),
                            "component-nav",
                            sidebar_entries.len(),
                            move |this, range, _window, cx| {
                                range
                                    .filter_map(|ix| {
                                        if ix < sidebar_entries.len() {
                                            Some(this.render_sidebar_entry(
                                                ix,
                                                &sidebar_entries[ix],
                                                cx,
                                            ))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(self.nav_scroll_handle.clone())
                        .pt_4()
                        .px_4()
                        .w(px(240.))
                        .h_full()
                        .flex_1(),
                    )
                    .child(
                        div().w_full().pb_4().child(
                            Button::new("toast-test", "Launch Toast")
                                .on_click(cx.listener({
                                    move |this, _, _window, cx| {
                                        this.test_status_toast(cx);
                                        cx.notify();
                                    }
                                }))
                                .full_width(),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .id("content-area")
                    .flex_1()
                    .size_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .p_2()
                            .w_full()
                            .border_b_1()
                            .border_color(cx.theme().colors().border)
                            .child(self.filter_editor.clone()),
                    )
                    .child(match active_page {
                        PreviewPage::AllComponents => {
                            self.render_all_components(cx).into_any_element()
                        }
                        PreviewPage::Component(id) => self
                            .render_component_page(&id, window, cx)
                            .into_any_element(),
                    }),
            )
    }
}

impl EventEmitter<ItemEvent> for ComponentPreview {}

impl Focusable for ComponentPreview {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActivePageId(pub String);

impl Default for ActivePageId {
    fn default() -> Self {
        ActivePageId("AllComponents".to_string())
    }
}

impl From<ComponentId> for ActivePageId {
    fn from(id: ComponentId) -> Self {
        ActivePageId(id.0.to_string())
    }
}

impl Item for ComponentPreview {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Component Preview".into()
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
        let language_registry = self.language_registry.clone();
        let user_store = self.user_store.clone();
        let weak_workspace = self.workspace.clone();
        let selected_index = self.cursor_index;
        let active_page = self.active_page.clone();

        Some(cx.new(|cx| {
            Self::new(
                weak_workspace,
                language_registry,
                user_store,
                selected_index,
                Some(active_page),
                window,
                cx,
            )
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace_id = workspace.database_id();

        let focus_handle = self.filter_editor.read(cx).focus_handle(cx);
        window.focus(&focus_handle);
    }
}

impl SerializableItem for ComponentPreview {
    fn serialized_item_kind() -> &'static str {
        "ComponentPreview"
    }

    fn deserialize(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<Entity<Self>>> {
        let deserialized_active_page =
            match COMPONENT_PREVIEW_DB.get_active_page(item_id, workspace_id) {
                Ok(page) => {
                    if let Some(page) = page {
                        ActivePageId(page)
                    } else {
                        ActivePageId::default()
                    }
                }
                Err(_) => ActivePageId::default(),
            };

        let user_store = project.read(cx).user_store().clone();
        let language_registry = project.read(cx).languages().clone();
        let preview_page = if deserialized_active_page.0 == ActivePageId::default().0 {
            Some(PreviewPage::default())
        } else {
            let component_str = deserialized_active_page.0;
            let component_registry = components();
            let all_components = component_registry.all();
            let found_component = all_components.iter().find(|c| c.id().0 == component_str);

            if let Some(component) = found_component {
                Some(PreviewPage::Component(component.id().clone()))
            } else {
                Some(PreviewPage::default())
            }
        };

        window.spawn(cx, async move |cx| {
            let user_store = user_store.clone();
            let language_registry = language_registry.clone();
            let weak_workspace = workspace.clone();
            cx.update(move |window, cx| {
                Ok(cx.new(|cx| {
                    ComponentPreview::new(
                        weak_workspace,
                        language_registry,
                        user_store,
                        None,
                        preview_page,
                        window,
                        cx,
                    )
                }))
            })?
        })
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<gpui::Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "component_previews",
            &COMPONENT_PREVIEW_DB,
            cx,
        )
    }

    fn serialize(
        &mut self,
        _workspace: &mut Workspace,
        item_id: ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<gpui::Result<()>>> {
        let active_page = self.active_page_id(cx);
        let workspace_id = self.workspace_id?;
        Some(cx.background_spawn(async move {
            COMPONENT_PREVIEW_DB
                .save_active_page(item_id, workspace_id, active_page.0)
                .await
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        matches!(event, ItemEvent::UpdateTab)
    }
}

// TODO: use language registry to allow rendering markdown
#[derive(IntoElement)]
pub struct ComponentPreviewPage {
    // languages: Arc<LanguageRegistry>,
    component: ComponentMetadata,
}

impl ComponentPreviewPage {
    pub fn new(
        component: ComponentMetadata,
        // languages: Arc<LanguageRegistry>
    ) -> Self {
        Self {
            // languages,
            component,
        }
    }

    fn render_header(&self, _: &Window, cx: &App) -> impl IntoElement {
        v_flex()
            .px_12()
            .pt_16()
            .pb_12()
            .gap_6()
            .bg(cx.theme().colors().surface_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                v_flex()
                    .gap_0p5()
                    .child(
                        Label::new(self.component.scope().to_string())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Headline::new(self.component.scopeless_name()).size(HeadlineSize::XLarge),
                    ),
            )
            .when_some(self.component.description(), |this, description| {
                this.child(div().text_sm().child(description))
            })
    }

    fn render_preview(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .flex_1()
            .px_12()
            .py_6()
            .bg(cx.theme().colors().editor_background)
            .child(if let Some(preview) = self.component.preview() {
                preview(window, cx).unwrap_or_else(|| {
                    div()
                        .child("Failed to load preview. This path should be unreachable")
                        .into_any_element()
                })
            } else {
                div().child("No preview available").into_any_element()
            })
    }
}

impl RenderOnce for ComponentPreviewPage {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .id("component-preview-page")
            .overflow_y_scroll()
            .overflow_x_hidden()
            .w_full()
            .child(self.render_header(window, cx))
            .child(self.render_preview(window, cx))
    }
}
