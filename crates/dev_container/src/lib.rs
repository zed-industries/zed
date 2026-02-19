use gpui::AppContext;
use gpui::Entity;
use gpui::Task;
use http_client::anyhow;
use picker::Picker;
use picker::PickerDelegate;
use settings::RegisterSetting;
use settings::Settings;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;
use ui::ActiveTheme;
use ui::Button;
use ui::Clickable;
use ui::FluentBuilder;
use ui::KeyBinding;
use ui::StatefulInteractiveElement;
use ui::Switch;
use ui::ToggleState;
use ui::Tooltip;
use ui::h_flex;
use ui::rems_from_px;
use ui::v_flex;

use gpui::{Action, DismissEvent, EventEmitter, FocusHandle, Focusable, RenderOnce, WeakEntity};
use serde::Deserialize;
use ui::{
    AnyElement, App, Color, CommonAnimationExt, Context, Headline, HeadlineSize, Icon, IconName,
    InteractiveElement, IntoElement, Label, ListItem, ListSeparator, ModalHeader, Navigable,
    NavigableEntry, ParentElement, Render, Styled, StyledExt, Toggleable, Window, div, rems,
};
use util::ResultExt;
use util::rel_path::RelPath;
use workspace::{ModalView, Workspace, with_active_or_new_workspace};

use futures::AsyncReadExt;
use http::Request;
use http_client::{AsyncBody, HttpClient};

mod devcontainer_api;
mod model;

use devcontainer_api::read_default_devcontainer_configuration;

use crate::devcontainer_api::DevContainerError;
use crate::devcontainer_api::apply_dev_container_template;

pub use devcontainer_api::{
    DevContainerConfig, find_configs_in_snapshot, find_devcontainer_configs,
    start_dev_container_with_config,
};

#[derive(RegisterSetting)]
struct DevContainerSettings {
    use_podman: bool,
}

impl Settings for DevContainerSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        Self {
            use_podman: content.remote.use_podman.unwrap_or(false),
        }
    }
}

#[derive(PartialEq, Clone, Deserialize, Default, Action)]
#[action(namespace = projects)]
#[serde(deny_unknown_fields)]
struct InitializeDevContainer;

pub fn init(cx: &mut App) {
    cx.on_action(|_: &InitializeDevContainer, cx| {
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let weak_entity = cx.weak_entity();
            workspace.toggle_modal(window, cx, |window, cx| {
                DevContainerModal::new(weak_entity, window, cx)
            });
        });
    });
}

#[derive(Clone)]
struct TemplateEntry {
    template: DevContainerTemplate,
    options_selected: HashMap<String, String>,
    current_option_index: usize,
    current_option: Option<TemplateOptionSelection>,
    features_selected: HashSet<DevContainerFeature>,
}

#[derive(Clone)]
struct FeatureEntry {
    feature: DevContainerFeature,
    toggle_state: ToggleState,
}

#[derive(Clone)]
struct TemplateOptionSelection {
    option_name: String,
    description: String,
    navigable_options: Vec<(String, NavigableEntry)>,
}

impl Eq for TemplateEntry {}
impl PartialEq for TemplateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.template == other.template
    }
}
impl Debug for TemplateEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateEntry")
            .field("template", &self.template)
            .finish()
    }
}

impl Eq for FeatureEntry {}
impl PartialEq for FeatureEntry {
    fn eq(&self, other: &Self) -> bool {
        self.feature == other.feature
    }
}

impl Debug for FeatureEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureEntry")
            .field("feature", &self.feature)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DevContainerState {
    Initial,
    QueryingTemplates,
    TemplateQueryReturned(Result<Vec<TemplateEntry>, String>),
    QueryingFeatures(TemplateEntry),
    FeaturesQueryReturned(TemplateEntry),
    UserOptionsSpecifying(TemplateEntry),
    ConfirmingWriteDevContainer(TemplateEntry),
    TemplateWriteFailed(DevContainerError),
}

#[derive(Debug, Clone)]
enum DevContainerMessage {
    SearchTemplates,
    TemplatesRetrieved(Vec<DevContainerTemplate>),
    ErrorRetrievingTemplates(String),
    TemplateSelected(TemplateEntry),
    TemplateOptionsSpecified(TemplateEntry),
    TemplateOptionsCompleted(TemplateEntry),
    FeaturesRetrieved(Vec<DevContainerFeature>),
    FeaturesSelected(TemplateEntry),
    NeedConfirmWriteDevContainer(TemplateEntry),
    ConfirmWriteDevContainer(TemplateEntry),
    FailedToWriteTemplate(DevContainerError),
    GoBack,
}

struct DevContainerModal {
    workspace: WeakEntity<Workspace>,
    picker: Option<Entity<Picker<TemplatePickerDelegate>>>,
    features_picker: Option<Entity<Picker<FeaturePickerDelegate>>>,
    focus_handle: FocusHandle,
    confirm_entry: NavigableEntry,
    back_entry: NavigableEntry,
    state: DevContainerState,
}

struct TemplatePickerDelegate {
    selected_index: usize,
    placeholder_text: String,
    stateful_modal: WeakEntity<DevContainerModal>,
    candidate_templates: Vec<TemplateEntry>,
    matching_indices: Vec<usize>,
    on_confirm: Box<
        dyn FnMut(
            TemplateEntry,
            &mut DevContainerModal,
            &mut Window,
            &mut Context<DevContainerModal>,
        ),
    >,
}

impl TemplatePickerDelegate {
    fn new(
        placeholder_text: String,
        stateful_modal: WeakEntity<DevContainerModal>,
        elements: Vec<TemplateEntry>,
        on_confirm: Box<
            dyn FnMut(
                TemplateEntry,
                &mut DevContainerModal,
                &mut Window,
                &mut Context<DevContainerModal>,
            ),
        >,
    ) -> Self {
        Self {
            selected_index: 0,
            placeholder_text,
            stateful_modal,
            candidate_templates: elements,
            matching_indices: Vec::new(),
            on_confirm,
        }
    }
}

impl PickerDelegate for TemplatePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.matching_indices.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder_text.clone().into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        self.matching_indices = self
            .candidate_templates
            .iter()
            .enumerate()
            .filter(|(_, template_entry)| {
                template_entry
                    .template
                    .id
                    .to_lowercase()
                    .contains(&query.to_lowercase())
                    || template_entry
                        .template
                        .name
                        .to_lowercase()
                        .contains(&query.to_lowercase())
            })
            .map(|(ix, _)| ix)
            .collect();

        self.selected_index = std::cmp::min(
            self.selected_index,
            self.matching_indices.len().saturating_sub(1),
        );
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        let fun = &mut self.on_confirm;

        self.stateful_modal
            .update(cx, |modal, cx| {
                fun(
                    self.candidate_templates[self.matching_indices[self.selected_index]].clone(),
                    modal,
                    window,
                    cx,
                );
            })
            .ok();
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        self.stateful_modal
            .update(cx, |modal, cx| {
                modal.dismiss(&menu::Cancel, window, cx);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let Some(template_entry) = self.candidate_templates.get(self.matching_indices[ix]) else {
            return None;
        };
        Some(
            ListItem::new("li-template-match")
                .inset(true)
                .spacing(ui::ListItemSpacing::Sparse)
                .start_slot(Icon::new(IconName::Box))
                .toggle_state(selected)
                .child(Label::new(template_entry.template.name.clone()))
                .into_any_element(),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .justify_start()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("run-action", "Continue")
                        .key_binding(
                            KeyBinding::for_action(&menu::Confirm, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .into_any_element(),
        )
    }
}

struct FeaturePickerDelegate {
    selected_index: usize,
    placeholder_text: String,
    stateful_modal: WeakEntity<DevContainerModal>,
    candidate_features: Vec<FeatureEntry>,
    template_entry: TemplateEntry,
    matching_indices: Vec<usize>,
    on_confirm: Box<
        dyn FnMut(
            TemplateEntry,
            &mut DevContainerModal,
            &mut Window,
            &mut Context<DevContainerModal>,
        ),
    >,
}

impl FeaturePickerDelegate {
    fn new(
        placeholder_text: String,
        stateful_modal: WeakEntity<DevContainerModal>,
        candidate_features: Vec<FeatureEntry>,
        template_entry: TemplateEntry,
        on_confirm: Box<
            dyn FnMut(
                TemplateEntry,
                &mut DevContainerModal,
                &mut Window,
                &mut Context<DevContainerModal>,
            ),
        >,
    ) -> Self {
        Self {
            selected_index: 0,
            placeholder_text,
            stateful_modal,
            candidate_features,
            template_entry,
            matching_indices: Vec::new(),
            on_confirm,
        }
    }
}

impl PickerDelegate for FeaturePickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.matching_indices.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder_text.clone().into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.matching_indices = self
            .candidate_features
            .iter()
            .enumerate()
            .filter(|(_, feature_entry)| {
                feature_entry
                    .feature
                    .id
                    .to_lowercase()
                    .contains(&query.to_lowercase())
                    || feature_entry
                        .feature
                        .name
                        .to_lowercase()
                        .contains(&query.to_lowercase())
            })
            .map(|(ix, _)| ix)
            .collect();
        self.selected_index = std::cmp::min(
            self.selected_index,
            self.matching_indices.len().saturating_sub(1),
        );
        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if secondary {
            self.stateful_modal
                .update(cx, |modal, cx| {
                    (self.on_confirm)(self.template_entry.clone(), modal, window, cx)
                })
                .ok();
        } else {
            let current = &mut self.candidate_features[self.matching_indices[self.selected_index]];
            current.toggle_state = match current.toggle_state {
                ToggleState::Selected => {
                    self.template_entry
                        .features_selected
                        .remove(&current.feature);
                    ToggleState::Unselected
                }
                _ => {
                    self.template_entry
                        .features_selected
                        .insert(current.feature.clone());
                    ToggleState::Selected
                }
            };
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.stateful_modal
            .update(cx, |modal, cx| {
                modal.dismiss(&menu::Cancel, window, cx);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let feature_entry = self.candidate_features[self.matching_indices[ix]].clone();

        Some(
            ListItem::new("li-what")
                .inset(true)
                .toggle_state(selected)
                .start_slot(Switch::new(
                    feature_entry.feature.id.clone(),
                    feature_entry.toggle_state,
                ))
                .child(Label::new(feature_entry.feature.name))
                .into_any_element(),
        )
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_1()
                .justify_start()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("run-action", "Select Feature")
                        .key_binding(
                            KeyBinding::for_action(&menu::Confirm, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .child(
                    Button::new("run-action-secondary", "Confirm Selections")
                        .key_binding(
                            KeyBinding::for_action(&menu::SecondaryConfirm, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                        }),
                )
                .into_any_element(),
        )
    }
}

impl DevContainerModal {
    fn new(workspace: WeakEntity<Workspace>, _window: &mut Window, cx: &mut App) -> Self {
        DevContainerModal {
            workspace,
            picker: None,
            features_picker: None,
            state: DevContainerState::Initial,
            focus_handle: cx.focus_handle(),
            confirm_entry: NavigableEntry::focusable(cx),
            back_entry: NavigableEntry::focusable(cx),
        }
    }

    fn render_initial(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let mut view = Navigable::new(
            div()
                .p_1()
                .child(
                    div().track_focus(&self.focus_handle).child(
                        ModalHeader::new().child(
                            Headline::new("Create Dev Container").size(HeadlineSize::XSmall),
                        ),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div()
                        .track_focus(&self.confirm_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.accept_message(DevContainerMessage::SearchTemplates, window, cx);
                        }))
                        .child(
                            ListItem::new("li-search-containers")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(
                                    Icon::new(IconName::MagnifyingGlass).color(Color::Muted),
                                )
                                .toggle_state(
                                    self.confirm_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(
                                        DevContainerMessage::SearchTemplates,
                                        window,
                                        cx,
                                    );
                                    cx.notify();
                                }))
                                .child(Label::new("Search for Dev Container Templates")),
                        ),
                )
                .into_any_element(),
        );
        view = view.entry(self.confirm_entry.clone());
        view.render(window, cx).into_any_element()
    }

    fn render_error(
        &self,
        error_title: String,
        error: impl Display,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .p_1()
            .child(div().track_focus(&self.focus_handle).child(
                ModalHeader::new().child(Headline::new(error_title).size(HeadlineSize::XSmall)),
            ))
            .child(ListSeparator)
            .child(
                v_flex()
                    .child(Label::new(format!("{}", error)))
                    .whitespace_normal(),
            )
            .into_any_element()
    }

    fn render_retrieved_templates(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if let Some(picker) = &self.picker {
            let picker_element = div()
                .track_focus(&self.focus_handle(cx))
                .child(picker.clone().into_any_element())
                .into_any_element();
            picker.focus_handle(cx).focus(window, cx);
            picker_element
        } else {
            div().into_any_element()
        }
    }

    fn render_user_options_specifying(
        &self,
        template_entry: TemplateEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(next_option_entries) = &template_entry.current_option else {
            return div().into_any_element();
        };
        let mut view = Navigable::new(
            div()
                .child(
                    div()
                        .id("title")
                        .tooltip(Tooltip::text(next_option_entries.description.clone()))
                        .track_focus(&self.focus_handle)
                        .child(
                            ModalHeader::new()
                                .child(
                                    Headline::new("Template Option: ").size(HeadlineSize::XSmall),
                                )
                                .child(
                                    Headline::new(&next_option_entries.option_name)
                                        .size(HeadlineSize::XSmall),
                                ),
                        ),
                )
                .child(ListSeparator)
                .children(
                    next_option_entries
                        .navigable_options
                        .iter()
                        .map(|(option, entry)| {
                            div()
                                .id(format!("li-parent-{}", option))
                                .track_focus(&entry.focus_handle)
                                .on_action({
                                    let mut template = template_entry.clone();
                                    template.options_selected.insert(
                                        next_option_entries.option_name.clone(),
                                        option.clone(),
                                    );
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.accept_message(
                                            DevContainerMessage::TemplateOptionsSpecified(
                                                template.clone(),
                                            ),
                                            window,
                                            cx,
                                        );
                                    })
                                })
                                .child(
                                    ListItem::new(format!("li-option-{}", option))
                                        .inset(true)
                                        .spacing(ui::ListItemSpacing::Sparse)
                                        .toggle_state(
                                            entry.focus_handle.contains_focused(window, cx),
                                        )
                                        .on_click({
                                            let mut template = template_entry.clone();
                                            template.options_selected.insert(
                                                next_option_entries.option_name.clone(),
                                                option.clone(),
                                            );
                                            cx.listener(move |this, _, window, cx| {
                                                this.accept_message(
                                                    DevContainerMessage::TemplateOptionsSpecified(
                                                        template.clone(),
                                                    ),
                                                    window,
                                                    cx,
                                                );
                                                cx.notify();
                                            })
                                        })
                                        .child(Label::new(option)),
                                )
                        }),
                )
                .child(ListSeparator)
                .child(
                    div()
                        .track_focus(&self.back_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.accept_message(DevContainerMessage::GoBack, window, cx);
                        }))
                        .child(
                            ListItem::new("li-goback")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Return).color(Color::Muted))
                                .toggle_state(
                                    self.back_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(DevContainerMessage::GoBack, window, cx);
                                    cx.notify();
                                }))
                                .child(Label::new("Go Back")),
                        ),
                )
                .into_any_element(),
        );
        for (_, entry) in &next_option_entries.navigable_options {
            view = view.entry(entry.clone());
        }
        view = view.entry(self.back_entry.clone());
        view.render(window, cx).into_any_element()
    }

    fn render_features_query_returned(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if let Some(picker) = &self.features_picker {
            let picker_element = div()
                .track_focus(&self.focus_handle(cx))
                .child(picker.clone().into_any_element())
                .into_any_element();
            picker.focus_handle(cx).focus(window, cx);
            picker_element
        } else {
            div().into_any_element()
        }
    }

    fn render_confirming_write_dev_container(
        &self,
        template_entry: TemplateEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        Navigable::new(
            div()
                .child(
                    div().track_focus(&self.focus_handle).child(
                        ModalHeader::new()
                            .icon(Icon::new(IconName::Warning).color(Color::Warning))
                            .child(
                                Headline::new("Overwrite Existing Configuration?")
                                    .size(HeadlineSize::XSmall),
                            ),
                    ),
                )
                .child(
                    div()
                        .track_focus(&self.confirm_entry.focus_handle)
                        .on_action({
                            let template = template_entry.clone();
                            cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                this.accept_message(
                                    DevContainerMessage::ConfirmWriteDevContainer(template.clone()),
                                    window,
                                    cx,
                                );
                            })
                        })
                        .child(
                            ListItem::new("li-search-containers")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Check).color(Color::Muted))
                                .toggle_state(
                                    self.confirm_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.accept_message(
                                        DevContainerMessage::ConfirmWriteDevContainer(
                                            template_entry.clone(),
                                        ),
                                        window,
                                        cx,
                                    );
                                    cx.notify();
                                }))
                                .child(Label::new("Overwrite")),
                        ),
                )
                .child(
                    div()
                        .track_focus(&self.back_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.dismiss(&menu::Cancel, window, cx);
                        }))
                        .child(
                            ListItem::new("li-goback")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::XCircle).color(Color::Muted))
                                .toggle_state(
                                    self.back_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.dismiss(&menu::Cancel, window, cx);
                                    cx.notify();
                                }))
                                .child(Label::new("Cancel")),
                        ),
                )
                .into_any_element(),
        )
        .entry(self.confirm_entry.clone())
        .entry(self.back_entry.clone())
        .render(window, cx)
        .into_any_element()
    }

    fn render_querying_templates(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        Navigable::new(
            div()
                .child(
                    div().track_focus(&self.focus_handle).child(
                        ModalHeader::new().child(
                            Headline::new("Create Dev Container").size(HeadlineSize::XSmall),
                        ),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div().child(
                        ListItem::new("li-querying")
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(
                                Icon::new(IconName::ArrowCircle)
                                    .color(Color::Muted)
                                    .with_rotate_animation(2),
                            )
                            .child(Label::new("Querying template registry...")),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div()
                        .track_focus(&self.back_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.accept_message(DevContainerMessage::GoBack, window, cx);
                        }))
                        .child(
                            ListItem::new("li-goback")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                .toggle_state(
                                    self.back_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(DevContainerMessage::GoBack, window, cx);
                                    cx.notify();
                                }))
                                .child(Label::new("Go Back")),
                        ),
                )
                .into_any_element(),
        )
        .entry(self.back_entry.clone())
        .render(window, cx)
        .into_any_element()
    }
    fn render_querying_features(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        Navigable::new(
            div()
                .child(
                    div().track_focus(&self.focus_handle).child(
                        ModalHeader::new().child(
                            Headline::new("Create Dev Container").size(HeadlineSize::XSmall),
                        ),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div().child(
                        ListItem::new("li-querying")
                            .inset(true)
                            .spacing(ui::ListItemSpacing::Sparse)
                            .start_slot(
                                Icon::new(IconName::ArrowCircle)
                                    .color(Color::Muted)
                                    .with_rotate_animation(2),
                            )
                            .child(Label::new("Querying features...")),
                    ),
                )
                .child(ListSeparator)
                .child(
                    div()
                        .track_focus(&self.back_entry.focus_handle)
                        .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                            this.accept_message(DevContainerMessage::GoBack, window, cx);
                        }))
                        .child(
                            ListItem::new("li-goback")
                                .inset(true)
                                .spacing(ui::ListItemSpacing::Sparse)
                                .start_slot(Icon::new(IconName::Pencil).color(Color::Muted))
                                .toggle_state(
                                    self.back_entry.focus_handle.contains_focused(window, cx),
                                )
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.accept_message(DevContainerMessage::GoBack, window, cx);
                                    cx.notify();
                                }))
                                .child(Label::new("Go Back")),
                        ),
                )
                .into_any_element(),
        )
        .entry(self.back_entry.clone())
        .render(window, cx)
        .into_any_element()
    }
}

impl StatefulModal for DevContainerModal {
    type State = DevContainerState;
    type Message = DevContainerMessage;

    fn state(&self) -> Self::State {
        self.state.clone()
    }

    fn render_for_state(
        &self,
        state: Self::State,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match state {
            DevContainerState::Initial => self.render_initial(window, cx),
            DevContainerState::QueryingTemplates => self.render_querying_templates(window, cx),
            DevContainerState::TemplateQueryReturned(Ok(_)) => {
                self.render_retrieved_templates(window, cx)
            }
            DevContainerState::UserOptionsSpecifying(template_entry) => {
                self.render_user_options_specifying(template_entry, window, cx)
            }
            DevContainerState::QueryingFeatures(_) => self.render_querying_features(window, cx),
            DevContainerState::FeaturesQueryReturned(_) => {
                self.render_features_query_returned(window, cx)
            }
            DevContainerState::ConfirmingWriteDevContainer(template_entry) => {
                self.render_confirming_write_dev_container(template_entry, window, cx)
            }
            DevContainerState::TemplateWriteFailed(dev_container_error) => self.render_error(
                "Error Creating Dev Container Definition".to_string(),
                dev_container_error,
                window,
                cx,
            ),
            DevContainerState::TemplateQueryReturned(Err(e)) => {
                self.render_error("Error Retrieving Templates".to_string(), e, window, cx)
            }
        }
    }

    fn accept_message(
        &mut self,
        message: Self::Message,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let new_state = match message {
            DevContainerMessage::SearchTemplates => {
                cx.spawn_in(window, async move |this, cx| {
                    let Ok(client) = cx.update(|_, cx| cx.http_client()) else {
                        return;
                    };
                    match get_templates(client).await {
                        Ok(templates) => {
                            let message =
                                DevContainerMessage::TemplatesRetrieved(templates.templates);
                            this.update_in(cx, |this, window, cx| {
                                this.accept_message(message, window, cx);
                            })
                            .ok();
                        }
                        Err(e) => {
                            let message = DevContainerMessage::ErrorRetrievingTemplates(e);
                            this.update_in(cx, |this, window, cx| {
                                this.accept_message(message, window, cx);
                            })
                            .ok();
                        }
                    }
                })
                .detach();
                Some(DevContainerState::QueryingTemplates)
            }
            DevContainerMessage::ErrorRetrievingTemplates(message) => {
                Some(DevContainerState::TemplateQueryReturned(Err(message)))
            }
            DevContainerMessage::GoBack => match &self.state {
                DevContainerState::Initial => Some(DevContainerState::Initial),
                DevContainerState::QueryingTemplates => Some(DevContainerState::Initial),
                DevContainerState::UserOptionsSpecifying(template_entry) => {
                    if template_entry.current_option_index <= 1 {
                        self.accept_message(DevContainerMessage::SearchTemplates, window, cx);
                    } else {
                        let mut template_entry = template_entry.clone();
                        template_entry.current_option_index =
                            template_entry.current_option_index.saturating_sub(2);
                        self.accept_message(
                            DevContainerMessage::TemplateOptionsSpecified(template_entry),
                            window,
                            cx,
                        );
                    }
                    None
                }
                _ => Some(DevContainerState::Initial),
            },
            DevContainerMessage::TemplatesRetrieved(items) => {
                let items = items
                    .into_iter()
                    .map(|item| TemplateEntry {
                        template: item,
                        options_selected: HashMap::new(),
                        current_option_index: 0,
                        current_option: None,
                        features_selected: HashSet::new(),
                    })
                    .collect::<Vec<TemplateEntry>>();
                if self.state == DevContainerState::QueryingTemplates {
                    let delegate = TemplatePickerDelegate::new(
                        "Select a template".to_string(),
                        cx.weak_entity(),
                        items.clone(),
                        Box::new(|entry, this, window, cx| {
                            this.accept_message(
                                DevContainerMessage::TemplateSelected(entry),
                                window,
                                cx,
                            );
                        }),
                    );

                    let picker =
                        cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));
                    self.picker = Some(picker);
                    Some(DevContainerState::TemplateQueryReturned(Ok(items)))
                } else {
                    None
                }
            }
            DevContainerMessage::TemplateSelected(mut template_entry) => {
                let Some(options) = template_entry.template.clone().options else {
                    return self.accept_message(
                        DevContainerMessage::TemplateOptionsCompleted(template_entry),
                        window,
                        cx,
                    );
                };

                let options = options
                    .iter()
                    .collect::<Vec<(&String, &TemplateOptions)>>()
                    .clone();

                let Some((first_option_name, first_option)) =
                    options.get(template_entry.current_option_index)
                else {
                    return self.accept_message(
                        DevContainerMessage::TemplateOptionsCompleted(template_entry),
                        window,
                        cx,
                    );
                };

                let next_option_entries = first_option
                    .possible_values()
                    .into_iter()
                    .map(|option| (option, NavigableEntry::focusable(cx)))
                    .collect();

                template_entry.current_option_index += 1;
                template_entry.current_option = Some(TemplateOptionSelection {
                    option_name: (*first_option_name).clone(),
                    description: first_option
                        .description
                        .clone()
                        .unwrap_or_else(|| "".to_string()),
                    navigable_options: next_option_entries,
                });

                Some(DevContainerState::UserOptionsSpecifying(template_entry))
            }
            DevContainerMessage::TemplateOptionsSpecified(mut template_entry) => {
                let Some(options) = template_entry.template.clone().options else {
                    return self.accept_message(
                        DevContainerMessage::TemplateOptionsCompleted(template_entry),
                        window,
                        cx,
                    );
                };

                let options = options
                    .iter()
                    .collect::<Vec<(&String, &TemplateOptions)>>()
                    .clone();

                let Some((next_option_name, next_option)) =
                    options.get(template_entry.current_option_index)
                else {
                    return self.accept_message(
                        DevContainerMessage::TemplateOptionsCompleted(template_entry),
                        window,
                        cx,
                    );
                };

                let next_option_entries = next_option
                    .possible_values()
                    .into_iter()
                    .map(|option| (option, NavigableEntry::focusable(cx)))
                    .collect();

                template_entry.current_option_index += 1;
                template_entry.current_option = Some(TemplateOptionSelection {
                    option_name: (*next_option_name).clone(),
                    description: next_option
                        .description
                        .clone()
                        .unwrap_or_else(|| "".to_string()),
                    navigable_options: next_option_entries,
                });

                Some(DevContainerState::UserOptionsSpecifying(template_entry))
            }
            DevContainerMessage::TemplateOptionsCompleted(template_entry) => {
                cx.spawn_in(window, async move |this, cx| {
                    let Ok(client) = cx.update(|_, cx| cx.http_client()) else {
                        return;
                    };
                    let Some(features) = get_features(client).await.log_err() else {
                        return;
                    };
                    let message = DevContainerMessage::FeaturesRetrieved(features.features);
                    this.update_in(cx, |this, window, cx| {
                        this.accept_message(message, window, cx);
                    })
                    .ok();
                })
                .detach();
                Some(DevContainerState::QueryingFeatures(template_entry))
            }
            DevContainerMessage::FeaturesRetrieved(features) => {
                if let DevContainerState::QueryingFeatures(template_entry) = self.state.clone() {
                    let features = features
                        .iter()
                        .map(|feature| FeatureEntry {
                            feature: feature.clone(),
                            toggle_state: ToggleState::Unselected,
                        })
                        .collect::<Vec<FeatureEntry>>();
                    let delegate = FeaturePickerDelegate::new(
                        "Select features to add".to_string(),
                        cx.weak_entity(),
                        features,
                        template_entry.clone(),
                        Box::new(|entry, this, window, cx| {
                            this.accept_message(
                                DevContainerMessage::FeaturesSelected(entry),
                                window,
                                cx,
                            );
                        }),
                    );

                    let picker =
                        cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));
                    self.features_picker = Some(picker);
                    Some(DevContainerState::FeaturesQueryReturned(template_entry))
                } else {
                    None
                }
            }
            DevContainerMessage::FeaturesSelected(template_entry) => {
                if let Some(workspace) = self.workspace.upgrade() {
                    dispatch_apply_templates(template_entry, workspace, window, true, cx);
                }

                None
            }
            DevContainerMessage::NeedConfirmWriteDevContainer(template_entry) => Some(
                DevContainerState::ConfirmingWriteDevContainer(template_entry),
            ),
            DevContainerMessage::ConfirmWriteDevContainer(template_entry) => {
                if let Some(workspace) = self.workspace.upgrade() {
                    dispatch_apply_templates(template_entry, workspace, window, false, cx);
                }
                None
            }
            DevContainerMessage::FailedToWriteTemplate(error) => {
                Some(DevContainerState::TemplateWriteFailed(error))
            }
        };
        if let Some(state) = new_state {
            self.state = state;
            self.focus_handle.focus(window, cx);
        }
        cx.notify();
    }
}
impl EventEmitter<DismissEvent> for DevContainerModal {}
impl Focusable for DevContainerModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl ModalView for DevContainerModal {}

impl Render for DevContainerModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.render_inner(window, cx)
    }
}

trait StatefulModal: ModalView + EventEmitter<DismissEvent> + Render {
    type State;
    type Message;

    fn state(&self) -> Self::State;

    fn render_for_state(
        &self,
        state: Self::State,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement;

    fn accept_message(
        &mut self,
        message: Self::Message,
        window: &mut Window,
        cx: &mut Context<Self>,
    );

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn render_inner(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let element = self.render_for_state(self.state(), window, cx);
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ContainerModal")
            .on_action(cx.listener(Self::dismiss))
            .child(element)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GithubTokenResponse {
    pub(crate) token: String,
}

fn ghcr_url() -> &'static str {
    "https://ghcr.io"
}

fn ghcr_domain() -> &'static str {
    "ghcr.io"
}

fn devcontainer_templates_repository() -> &'static str {
    "devcontainers/templates"
}

fn devcontainer_features_repository() -> &'static str {
    "devcontainers/features"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ManifestLayer {
    pub(crate) digest: String,
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TemplateOptions {
    #[serde(rename = "type")]
    option_type: String,
    description: Option<String>,
    proposals: Option<Vec<String>>,
    #[serde(rename = "enum")]
    enum_values: Option<Vec<String>>,
    // Different repositories surface "default: 'true'" or "default: true",
    // so we need to be flexible in deserializing
    #[serde(deserialize_with = "deserialize_string_or_bool")]
    default: String,
}

fn deserialize_string_or_bool<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrBool {
        String(String),
        Bool(bool),
    }

    match StringOrBool::deserialize(deserializer)? {
        StringOrBool::String(s) => Ok(s),
        StringOrBool::Bool(b) => Ok(b.to_string()),
    }
}

impl TemplateOptions {
    fn possible_values(&self) -> Vec<String> {
        match self.option_type.as_str() {
            "string" => self
                .enum_values
                .clone()
                .or(self.proposals.clone().or(Some(vec![self.default.clone()])))
                .unwrap_or_default(),
            // If not string, must be boolean
            _ => {
                if self.default == "true" {
                    vec!["true".to_string(), "false".to_string()]
                } else {
                    vec!["false".to_string(), "true".to_string()]
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DockerManifestsResponse {
    pub(crate) layers: Vec<ManifestLayer>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
struct DevContainerFeature {
    id: String,
    version: String,
    name: String,
    source_repository: Option<String>,
}

impl DevContainerFeature {
    fn major_version(&self) -> String {
        let Some(mv) = self.version.get(..1) else {
            return "".to_string();
        };
        mv.to_string()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct DevContainerTemplate {
    id: String,
    name: String,
    options: Option<HashMap<String, TemplateOptions>>,
    source_repository: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerFeaturesResponse {
    features: Vec<DevContainerFeature>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevContainerTemplatesResponse {
    templates: Vec<DevContainerTemplate>,
}

fn dispatch_apply_templates(
    template_entry: TemplateEntry,
    workspace: Entity<Workspace>,
    window: &mut Window,
    check_for_existing: bool,
    cx: &mut Context<DevContainerModal>,
) {
    cx.spawn_in(window, async move |this, cx| {
        if let Some(tree_id) = workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            let worktree = project.read(cx).visible_worktrees(cx).find_map(|tree| {
                tree.read(cx)
                    .root_entry()?
                    .is_dir()
                    .then_some(tree.read(cx))
            });
            worktree.map(|w| w.id())
        }) {
            let node_runtime = workspace.read_with(cx, |workspace, _| {
                workspace.app_state().node_runtime.clone()
            });

            if check_for_existing && read_default_devcontainer_configuration(cx).await.is_ok() {
                this.update_in(cx, |this, window, cx| {
                    this.accept_message(
                        DevContainerMessage::NeedConfirmWriteDevContainer(template_entry),
                        window,
                        cx,
                    );
                })
                .ok();
                return;
            }

            let files = match apply_dev_container_template(
                &template_entry.template,
                &template_entry.options_selected,
                &template_entry.features_selected,
                cx,
                &node_runtime,
            )
            .await
            {
                Ok(files) => files,
                Err(e) => {
                    this.update_in(cx, |this, window, cx| {
                        this.accept_message(
                            DevContainerMessage::FailedToWriteTemplate(e),
                            window,
                            cx,
                        );
                    })
                    .ok();
                    return;
                }
            };

            if files
                .files
                .contains(&"./.devcontainer/devcontainer.json".to_string())
            {
                let Some(workspace_task) = workspace
                    .update_in(cx, |workspace, window, cx| {
                        let Ok(path) = RelPath::unix(".devcontainer/devcontainer.json") else {
                            return Task::ready(Err(anyhow!(
                                "Couldn't create path for .devcontainer/devcontainer.json"
                            )));
                        };
                        workspace.open_path((tree_id, path), None, true, window, cx)
                    })
                    .ok()
                else {
                    return;
                };

                workspace_task.await.log_err();
            }
            this.update_in(cx, |this, window, cx| {
                this.dismiss(&menu::Cancel, window, cx);
            })
            .ok();
        } else {
            return;
        }
    })
    .detach();
}

async fn get_templates(
    client: Arc<dyn HttpClient>,
) -> Result<DevContainerTemplatesResponse, String> {
    let token = get_ghcr_token(&client).await?;
    let manifest = get_latest_manifest(&token.token, &client).await?;

    let mut template_response =
        get_devcontainer_templates(&token.token, &manifest.layers[0].digest, &client).await?;

    for template in &mut template_response.templates {
        template.source_repository = Some(format!(
            "{}/{}",
            ghcr_domain(),
            devcontainer_templates_repository()
        ));
    }
    Ok(template_response)
}

async fn get_features(client: Arc<dyn HttpClient>) -> Result<DevContainerFeaturesResponse, String> {
    let token = get_ghcr_token(&client).await?;
    let manifest = get_latest_feature_manifest(&token.token, &client).await?;

    let mut features_response =
        get_devcontainer_features(&token.token, &manifest.layers[0].digest, &client).await?;

    for feature in &mut features_response.features {
        feature.source_repository = Some(format!(
            "{}/{}",
            ghcr_domain(),
            devcontainer_features_repository()
        ));
    }
    Ok(features_response)
}

async fn get_ghcr_token(client: &Arc<dyn HttpClient>) -> Result<GithubTokenResponse, String> {
    let url = format!(
        "{}/token?service=ghcr.io&scope=repository:{}:pull",
        ghcr_url(),
        devcontainer_templates_repository()
    );
    get_deserialized_response("", &url, client).await
}

async fn get_latest_feature_manifest(
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "{}/v2/{}/manifests/latest",
        ghcr_url(),
        devcontainer_features_repository()
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_latest_manifest(
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "{}/v2/{}/manifests/latest",
        ghcr_url(),
        devcontainer_templates_repository()
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_devcontainer_features(
    token: &str,
    blob_digest: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DevContainerFeaturesResponse, String> {
    let url = format!(
        "{}/v2/{}/blobs/{}",
        ghcr_url(),
        devcontainer_features_repository(),
        blob_digest
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_devcontainer_templates(
    token: &str,
    blob_digest: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DevContainerTemplatesResponse, String> {
    let url = format!(
        "{}/v2/{}/blobs/{}",
        ghcr_url(),
        devcontainer_templates_repository(),
        blob_digest
    );
    get_deserialized_response(token, &url, client).await
}

async fn get_deserialized_response<T>(
    token: &str,
    url: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let request = match Request::get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.oci.image.manifest.v1+json")
        .body(AsyncBody::default())
    {
        Ok(request) => request,
        Err(e) => return Err(format!("Failed to create request: {}", e)),
    };
    let response = match client.send(request).await {
        Ok(response) => response,
        Err(e) => {
            return Err(format!("Failed to send request to {}: {}", url, e));
        }
    };

    let status = response.status();
    let mut output = String::new();

    if let Err(e) = response.into_body().read_to_string(&mut output).await {
        return Err(format!("Failed to read response body from {}: {}", url, e));
    };

    log::info!(
        "OCI response from {}: status={}, body_len={}, body_preview={}",
        url,
        status.as_u16(),
        output.len(),
        &output[..output.len().min(200)],
    );

    if !status.is_success() {
        return Err(format!(
            "OCI request to {} returned HTTP {}: {}",
            url,
            status.as_u16(),
            &output[..output.len().min(500)],
        ));
    }

    match serde_json_lenient::from_str(&output) {
        Ok(response) => Ok(response),
        Err(e) => Err(format!(
            "Failed to deserialize response from {}: {} (body: {})",
            url,
            e,
            &output[..output.len().min(500)],
        )),
    }
}

/// Parsed components of an OCI feature reference such as
/// `ghcr.io/devcontainers/features/aws-cli:1`.
///
/// Mirrors the CLI's `OCIRef` in `containerCollectionsOCI.ts`.
#[derive(Debug, Clone)]
pub(crate) struct OciFeatureRef {
    /// Registry hostname, e.g. `ghcr.io`
    pub registry: String,
    /// Full repository path within the registry, e.g. `devcontainers/features/aws-cli`
    pub path: String,
    /// Short feature identifier, e.g. `aws-cli`
    pub id: String,
    /// Version tag, digest, or `latest`
    pub version: String,
}

/// Parses an OCI feature reference string into its components.
///
/// Handles formats like:
/// - `ghcr.io/devcontainers/features/aws-cli:1`
/// - `ghcr.io/user/repo/go`  (implicitly `:latest`)
/// - `ghcr.io/devcontainers/features/rust@sha256:abc123`
///
/// Returns `None` for local paths (`./…`) and direct tarball URIs (`https://…`).
pub(crate) fn parse_oci_feature_ref(input: &str) -> Option<OciFeatureRef> {
    if input.starts_with('.')
        || input.starts_with('/')
        || input.starts_with("https://")
        || input.starts_with("http://")
    {
        return None;
    }

    let input_lower = input.to_lowercase();

    let (resource, version) = if let Some(at_idx) = input_lower.rfind('@') {
        // Digest-based: ghcr.io/foo/bar@sha256:abc
        (
            input_lower[..at_idx].to_string(),
            input_lower[at_idx + 1..].to_string(),
        )
    } else {
        let last_slash = input_lower.rfind('/');
        let last_colon = input_lower.rfind(':');
        match (last_slash, last_colon) {
            (Some(slash), Some(colon)) if colon > slash => (
                input_lower[..colon].to_string(),
                input_lower[colon + 1..].to_string(),
            ),
            _ => (input_lower.clone(), "latest".to_string()),
        }
    };

    let parts: Vec<&str> = resource.split('/').collect();
    if parts.len() < 3 {
        return None;
    }

    let registry = parts[0].to_string();
    let id = parts[parts.len() - 1].to_string();
    let path = parts[1..].join("/");

    Some(OciFeatureRef {
        registry,
        path,
        id,
        version,
    })
}

/// Gets a bearer token for pulling from a container registry repository.
///
/// This uses the registry's `/token` endpoint directly, which works for
/// `ghcr.io` and other registries that follow the same convention.  For
/// registries that require a full `WWW-Authenticate` negotiation flow this
/// would need to be extended.
pub(crate) async fn get_oci_token_for_repo(
    registry: &str,
    repository_path: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<String, String> {
    let url = format!(
        "https://{registry}/token?service={registry}&scope=repository:{repository_path}:pull",
    );
    log::info!("Fetching OCI token from: {}", url);
    let token_response: GithubTokenResponse = get_deserialized_response("", &url, client)
        .await
        .map_err(|e| {
            log::error!("OCI token request failed for {}: {e}", url);
            e
        })?;
    log::info!(
        "Got OCI token for {}: {}...{}",
        repository_path,
        &token_response.token[..token_response.token.len().min(12)],
        &token_response.token[token_response.token.len().saturating_sub(4)..],
    );
    Ok(token_response.token)
}

/// Fetches the OCI manifest for a feature, returning its layer descriptors.
pub(crate) async fn fetch_oci_feature_manifest(
    feature_ref: &OciFeatureRef,
    token: &str,
    client: &Arc<dyn HttpClient>,
) -> Result<DockerManifestsResponse, String> {
    let url = format!(
        "https://{}/v2/{}/manifests/{}",
        feature_ref.registry, feature_ref.path, feature_ref.version,
    );
    log::info!("Fetching OCI manifest from: {}", url);
    let manifest: DockerManifestsResponse = get_deserialized_response(token, &url, client)
        .await
        .map_err(|e| {
            log::error!("OCI manifest request failed for {}: {e}", url);
            e
        })?;
    log::info!(
        "OCI manifest for '{}': {} layer(s), digests: {:?}",
        feature_ref.id,
        manifest.layers.len(),
        manifest
            .layers
            .iter()
            .map(|l| &l.digest)
            .collect::<Vec<_>>(),
    );
    Ok(manifest)
}

/// Downloads an OCI blob (feature tarball) and extracts it into `dest_dir`.
///
/// The blob is expected to be a gzip-compressed tar archive containing the
/// feature's `install.sh`, `devcontainer-feature.json`, and any other files.
pub(crate) async fn download_and_extract_oci_feature(
    feature_ref: &OciFeatureRef,
    layer_digest: &str,
    token: &str,
    dest_dir: &std::path::Path,
    client: &Arc<dyn HttpClient>,
) -> Result<(), String> {
    let url = format!(
        "https://{}/v2/{}/blobs/{}",
        feature_ref.registry, feature_ref.path, layer_digest,
    );
    log::info!(
        "Downloading OCI blob for feature '{}': {}",
        feature_ref.id,
        url
    );

    let request = Request::get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.devcontainers.layer.v1+tar")
        .body(AsyncBody::default())
        .map_err(|e| format!("Failed to create blob request: {e}"))?;

    let response = client
        .send(request)
        .await
        .map_err(|e| format!("Failed to download feature blob: {e}"))?;

    let status = response.status();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("<none>")
        .to_string();
    log::info!(
        "OCI blob response for '{}': status={}, content-type={}",
        feature_ref.id,
        status.as_u16(),
        content_type,
    );

    // Read the entire body into memory so we can inspect it before feeding
    // it to the gzip decoder.
    let mut body_bytes = Vec::new();
    response
        .into_body()
        .read_to_end(&mut body_bytes)
        .await
        .map_err(|e| format!("Failed to read feature blob body: {e}"))?;

    log::info!(
        "OCI blob body for '{}': {} bytes, first 16 bytes: {:02x?}",
        feature_ref.id,
        body_bytes.len(),
        &body_bytes[..body_bytes.len().min(16)],
    );

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(&body_bytes);
        return Err(format!(
            "Feature blob download returned HTTP {}: {}",
            status.as_u16(),
            body_text,
        ));
    }

    // Per the dev container features distribution spec, feature layers use
    // media type `application/vnd.devcontainers.layer.v1+tar` (plain tar).
    // https://containers.dev/implementors/features-distribution/#oci-registry
    let cursor = futures::io::Cursor::new(body_bytes);
    let archive = async_tar::Archive::new(cursor);
    archive
        .unpack(dest_dir)
        .await
        .map_err(|e| format!("Failed to extract feature tarball: {e}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, anyhow};

    use crate::{
        GithubTokenResponse, devcontainer_templates_repository, get_deserialized_response,
        get_devcontainer_templates, get_ghcr_token, get_latest_manifest,
    };

    #[gpui::test]
    async fn test_get_deserialized_response(_cx: &mut TestAppContext) {
        let client = FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response =
            get_deserialized_response::<GithubTokenResponse>("", "https://ghcr.io/token", &client)
                .await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string())
    }

    #[gpui::test]
    async fn test_get_ghcr_token() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path != "/token" {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            let query = request.uri().query();
            if query.is_none()
                || query.unwrap()
                    != format!(
                        "service=ghcr.io&scope=repository:{}:pull",
                        devcontainer_templates_repository()
                    )
            {
                return Err(anyhow!("Unexpected query: {}", query.unwrap_or_default()));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{ \"token\": \"thisisatoken\" }".into())
                .unwrap())
        });

        let response = get_ghcr_token(&client).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().token, "thisisatoken".to_string());
    }

    #[gpui::test]
    async fn test_get_latest_manifests() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path
                != format!(
                    "/v2/{}/manifests/latest",
                    devcontainer_templates_repository()
                )
            {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{
                    \"schemaVersion\": 2,
                    \"mediaType\": \"application/vnd.oci.image.manifest.v1+json\",
                    \"config\": {
                        \"mediaType\": \"application/vnd.devcontainers\",
                        \"digest\": \"sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a\",
                        \"size\": 2
                    },
                    \"layers\": [
                        {
                            \"mediaType\": \"application/vnd.devcontainers.collection.layer.v1+json\",
                            \"digest\": \"sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09\",
                            \"size\": 65235,
                            \"annotations\": {
                                \"org.opencontainers.image.title\": \"devcontainer-collection.json\"
                            }
                        }
                    ],
                    \"annotations\": {
                        \"com.github.package.type\": \"devcontainer_collection\"
                    }
                }".into())
                .unwrap())
        });

        let response = get_latest_manifest("", &client).await;
        assert!(response.is_ok());
        let response = response.unwrap();

        assert_eq!(response.layers.len(), 1);
        assert_eq!(
            response.layers[0].digest,
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09"
        );
    }

    #[gpui::test]
    async fn test_get_devcontainer_templates() {
        let client = FakeHttpClient::create(|request| async move {
            let host = request.uri().host();
            if host.is_none() || host.unwrap() != "ghcr.io" {
                return Err(anyhow!("Unexpected host: {}", host.unwrap_or_default()));
            }
            let path = request.uri().path();
            if path
                != format!(
                    "/v2/{}/blobs/sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09",
                    devcontainer_templates_repository()
                )
            {
                return Err(anyhow!("Unexpected path: {}", path));
            }
            Ok(http_client::Response::builder()
                .status(200)
                .body("{
                    \"sourceInformation\": {
                        \"source\": \"devcontainer-cli\"
                    },
                    \"templates\": [
                        {
                            \"id\": \"alpine\",
                            \"version\": \"3.4.0\",
                            \"name\": \"Alpine\",
                            \"description\": \"Simple Alpine container with Git installed.\",
                            \"documentationURL\": \"https://github.com/devcontainers/templates/tree/main/src/alpine\",
                            \"publisher\": \"Dev Container Spec Maintainers\",
                            \"licenseURL\": \"https://github.com/devcontainers/templates/blob/main/LICENSE\",
                            \"options\": {
                                \"imageVariant\": {
                                    \"type\": \"string\",
                                    \"description\": \"Alpine version:\",
                                    \"proposals\": [
                                        \"3.21\",
                                        \"3.20\",
                                        \"3.19\",
                                        \"3.18\"
                                    ],
                                    \"default\": \"3.20\"
                                }
                            },
                            \"platforms\": [
                                \"Any\"
                            ],
                            \"optionalPaths\": [
                                \".github/dependabot.yml\"
                            ],
                            \"type\": \"image\",
                            \"files\": [
                                \"NOTES.md\",
                                \"README.md\",
                                \"devcontainer-template.json\",
                                \".devcontainer/devcontainer.json\",
                                \".github/dependabot.yml\"
                            ],
                            \"fileCount\": 5,
                            \"featureIds\": []
                        }
                    ]
                }".into())
                .unwrap())
        });
        let response = get_devcontainer_templates(
            "",
            "sha256:035e9c9fd9bd61f6d3965fa4bf11f3ddfd2490a8cf324f152c13cc3724d67d09",
            &client,
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.templates.len(), 1);
        assert_eq!(response.templates[0].name, "Alpine");
    }
}
