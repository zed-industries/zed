use std::{cmp::Reverse, rc::Rc, sync::Arc};

use acp_thread::AgentSessionConfigOptions;
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use fs::Fs;
use fuzzy::StringMatchCandidate;
use gpui::{BackgroundExecutor, DismissEvent, Task};
use ordered_float::OrderedFloat;
use picker::{Picker, PickerDelegate};
use ui::{
    DocumentationAside, DocumentationEdge, DocumentationSide, IntoElement, ListItem, prelude::*,
};

use crate::ui::HoldForDefault;

pub type ConfigOptionPicker = Picker<ConfigOptionPickerDelegate>;

pub fn config_option_picker(
    config_options: Rc<dyn AgentSessionConfigOptions>,
    config_id: acp::SessionConfigId,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    window: &mut Window,
    cx: &mut Context<ConfigOptionPicker>,
) -> ConfigOptionPicker {
    let delegate = ConfigOptionPickerDelegate::new(config_options, config_id, agent_server, fs, cx);
    Picker::list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems(20.))
        .max_height(Some(rems(20.).into()))
}

#[derive(Clone)]
pub enum ConfigOptionPickerEntry {
    Separator(SharedString),
    Option(ConfigOptionValue),
}

#[derive(Clone)]
pub struct ConfigOptionValue {
    pub value: acp::SessionConfigValueId,
    pub name: String,
    pub description: Option<String>,
    pub group: Option<String>,
}

pub struct ConfigOptionPickerDelegate {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    config_id: acp::SessionConfigId,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    filtered_entries: Vec<ConfigOptionPickerEntry>,
    all_options: Vec<ConfigOptionValue>,
    selected_index: usize,
    selected_description: Option<(usize, SharedString, bool)>,
}

impl ConfigOptionPickerDelegate {
    fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        config_id: acp::SessionConfigId,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        cx: &mut Context<ConfigOptionPicker>,
    ) -> Self {
        let all_options = Self::extract_options(&config_options, &config_id);
        let filtered_entries = options_to_picker_entries(&all_options);

        let current_value = Self::get_current_value(&config_options, &config_id);
        let selected_index = current_value
            .and_then(|current| {
                filtered_entries.iter().position(|entry| {
                    matches!(entry, ConfigOptionPickerEntry::Option(opt) if opt.value == current)
                })
            })
            .unwrap_or(0);

        cx.notify();

        Self {
            config_options,
            config_id,
            agent_server,
            fs,
            filtered_entries,
            all_options,
            selected_index,
            selected_description: None,
        }
    }

    fn extract_options(
        config_options: &Rc<dyn AgentSessionConfigOptions>,
        config_id: &acp::SessionConfigId,
    ) -> Vec<ConfigOptionValue> {
        let Some(option) = config_options
            .config_options()
            .into_iter()
            .find(|opt| &opt.id == config_id)
        else {
            return Vec::new();
        };

        match &option.kind {
            acp::SessionConfigKind::Select(select) => match &select.options {
                acp::SessionConfigSelectOptions::Ungrouped(options) => options
                    .iter()
                    .map(|opt| ConfigOptionValue {
                        value: opt.value.clone(),
                        name: opt.name.clone(),
                        description: opt.description.clone(),
                        group: None,
                    })
                    .collect(),
                acp::SessionConfigSelectOptions::Grouped(groups) => groups
                    .iter()
                    .flat_map(|group| {
                        group.options.iter().map(|opt| ConfigOptionValue {
                            value: opt.value.clone(),
                            name: opt.name.clone(),
                            description: opt.description.clone(),
                            group: Some(group.name.clone()),
                        })
                    })
                    .collect(),
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }

    fn get_current_value(
        config_options: &Rc<dyn AgentSessionConfigOptions>,
        config_id: &acp::SessionConfigId,
    ) -> Option<acp::SessionConfigValueId> {
        config_options
            .config_options()
            .into_iter()
            .find(|opt| &opt.id == config_id)
            .and_then(|opt| match &opt.kind {
                acp::SessionConfigKind::Select(select) => Some(select.current_value.clone()),
                _ => None,
            })
    }

    fn current_value(&self) -> Option<acp::SessionConfigValueId> {
        Self::get_current_value(&self.config_options, &self.config_id)
    }
}

impl PickerDelegate for ConfigOptionPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_entries.len().saturating_sub(1));
        cx.notify();
    }

    fn can_select(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        match self.filtered_entries.get(ix) {
            Some(ConfigOptionPickerEntry::Option(_)) => true,
            Some(ConfigOptionPickerEntry::Separator(_)) | None => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select an option…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_options = self.all_options.clone();

        cx.spawn_in(window, async move |this, cx| {
            let filtered_options = match this
                .read_with(cx, |_, cx| {
                    if query.is_empty() {
                        None
                    } else {
                        Some((all_options.clone(), query.clone(), cx.background_executor().clone()))
                    }
                })
                .ok()
                .flatten()
            {
                Some((options, q, executor)) => fuzzy_search_options(options, &q, executor).await,
                None => all_options,
            };

            this.update_in(cx, |this, window, cx| {
                this.delegate.filtered_entries = options_to_picker_entries(&filtered_options);

                let current_value = this.delegate.current_value();
                let new_index = current_value
                    .and_then(|current| {
                        this.delegate.filtered_entries.iter().position(|entry| {
                            matches!(entry, ConfigOptionPickerEntry::Option(opt) if opt.value == current)
                        })
                    })
                    .unwrap_or(0);

                this.set_selected_index(new_index, Some(picker::Direction::Down), true, window, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(ConfigOptionPickerEntry::Option(option)) =
            self.filtered_entries.get(self.selected_index)
        {
            if window.modifiers().secondary() {
                let default_value = self
                    .agent_server
                    .default_config_option(&self.config_id.0, cx);
                let is_default = default_value.as_deref() == Some(&*option.value.0);

                self.agent_server.set_default_config_option(
                    &self.config_id.0,
                    if is_default {
                        None
                    } else {
                        Some(&option.value.0)
                    },
                    self.fs.clone(),
                    cx,
                );
            }

            let task = self.config_options.set_config_option(
                self.config_id.clone(),
                option.value.clone(),
                cx,
            );

            cx.spawn(async move |_, _| {
                if let Err(err) = task.await {
                    log::error!("Failed to set config option: {:?}", err);
                }
            })
            .detach();

            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.defer_in(window, |picker, window, cx| {
            picker.set_query("", window, cx);
        });
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.filtered_entries.get(ix)? {
            ConfigOptionPickerEntry::Separator(title) => Some(
                div()
                    .when(ix > 0, |this| this.mt_1())
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .text_xs()
                            .text_color(cx.theme().colors().text_muted)
                            .child(title.clone()),
                    )
                    .into_any_element(),
            ),
            ConfigOptionPickerEntry::Option(option) => {
                let current_value = self.current_value();
                let is_selected = current_value.as_ref() == Some(&option.value);
                let default_value = self
                    .agent_server
                    .default_config_option(&self.config_id.0, cx);
                let is_default = default_value.as_deref() == Some(&*option.value.0);

                let option_name = option.name.clone();
                let description = option.description.clone();

                Some(
                    div()
                        .id(("config-option-picker-item", ix))
                        .when_some(description, |this, desc| {
                            let desc: SharedString = desc.into();
                            this.on_hover(cx.listener(move |menu, hovered, _, cx| {
                                if *hovered {
                                    menu.delegate.selected_description =
                                        Some((ix, desc.clone(), is_default));
                                } else if matches!(menu.delegate.selected_description, Some((id, _, _)) if id == ix)
                                {
                                    menu.delegate.selected_description = None;
                                }
                                cx.notify();
                            }))
                        })
                        .child(
                            ListItem::new(ix)
                                .inset(true)
                                .toggle_state(selected)
                                .child(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .child(Label::new(option_name).size(LabelSize::Small))
                                        .when(is_selected, |this| {
                                            this.child(
                                                Icon::new(IconName::Check)
                                                    .size(IconSize::Small)
                                                    .color(Color::Accent),
                                            )
                                        }),
                                ),
                        )
                        .into_any_element(),
                )
            }
        }
    }

    fn documentation_aside(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<DocumentationAside> {
        self.selected_description
            .as_ref()
            .map(|(_, description, is_default)| {
                let description = description.clone();
                let is_default = *is_default;

                DocumentationAside::new(
                    DocumentationSide::Left,
                    DocumentationEdge::Top,
                    Rc::new(move |_| {
                        v_flex()
                            .gap_1()
                            .child(Label::new(description.clone()))
                            .child(HoldForDefault::new(is_default))
                            .into_any_element()
                    }),
                )
            })
    }
}

fn options_to_picker_entries(options: &[ConfigOptionValue]) -> Vec<ConfigOptionPickerEntry> {
    let mut entries = Vec::new();
    let mut current_group: Option<String> = None;

    for option in options {
        if option.group != current_group {
            if let Some(group_name) = &option.group {
                entries.push(ConfigOptionPickerEntry::Separator(
                    group_name.clone().into(),
                ));
            }
            current_group = option.group.clone();
        }
        entries.push(ConfigOptionPickerEntry::Option(option.clone()));
    }

    entries
}

async fn fuzzy_search_options(
    options: Vec<ConfigOptionValue>,
    query: &str,
    executor: BackgroundExecutor,
) -> Vec<ConfigOptionValue> {
    let candidates = options
        .iter()
        .enumerate()
        .map(|(ix, opt)| StringMatchCandidate::new(ix, &opt.name))
        .collect::<Vec<_>>();

    let mut matches = fuzzy::match_strings(
        &candidates,
        query,
        false,
        true,
        100,
        &Default::default(),
        executor,
    )
    .await;

    matches.sort_unstable_by_key(|mat| {
        let candidate = &candidates[mat.candidate_id];
        (Reverse(OrderedFloat(mat.score)), candidate.id)
    });

    matches
        .into_iter()
        .map(|mat| options[mat.candidate_id].clone())
        .collect()
}

/// Count the total number of selectable options for a config option.
pub fn count_config_options(option: &acp::SessionConfigOption) -> usize {
    match &option.kind {
        acp::SessionConfigKind::Select(select) => match &select.options {
            acp::SessionConfigSelectOptions::Ungrouped(options) => options.len(),
            acp::SessionConfigSelectOptions::Grouped(groups) => {
                groups.iter().map(|g| g.options.len()).sum()
            }
            _ => 0,
        },
        _ => 0,
    }
}
