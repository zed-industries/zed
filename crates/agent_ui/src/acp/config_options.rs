use std::{cmp::Reverse, rc::Rc, sync::Arc};

use acp_thread::AgentSessionConfigOptions;
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use agent_settings::AgentSettings;
use collections::HashSet;
use fs::Fs;
use fuzzy::StringMatchCandidate;
use gpui::{
    BackgroundExecutor, Context, DismissEvent, Entity, Subscription, Task, Window, prelude::*,
};
use ordered_float::OrderedFloat;
use picker::popover_menu::PickerPopoverMenu;
use picker::{Picker, PickerDelegate};
use settings::{Settings, SettingsStore};
use ui::{
    DocumentationSide, ElevationIndex, IconButton, ListItem, ListItemSpacing, PopoverMenuHandle,
    Tooltip, prelude::*,
};
use util::ResultExt as _;

use crate::ui::HoldForDefault;

const PICKER_THRESHOLD: usize = 5;

pub struct ConfigOptionsView {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    selectors: Vec<Entity<ConfigOptionSelector>>,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    config_option_ids: Vec<acp::SessionConfigId>,
    _refresh_task: Task<()>,
}

impl ConfigOptionsView {
    pub fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selectors = Self::build_selectors(&config_options, &agent_server, &fs, window, cx);
        let config_option_ids = Self::config_option_ids(&config_options);

        let rx = config_options.watch(cx);
        let refresh_task = cx.spawn_in(window, async move |this, cx| {
            if let Some(mut rx) = rx {
                while let Ok(()) = rx.recv().await {
                    this.update_in(cx, |this, window, cx| {
                        this.refresh_selectors_if_needed(window, cx);
                        cx.notify();
                    })
                    .log_err();
                }
            }
        });

        Self {
            config_options,
            selectors,
            agent_server,
            fs,
            config_option_ids,
            _refresh_task: refresh_task,
        }
    }

    fn config_option_ids(
        config_options: &Rc<dyn AgentSessionConfigOptions>,
    ) -> Vec<acp::SessionConfigId> {
        config_options
            .config_options()
            .into_iter()
            .map(|option| option.id)
            .collect()
    }

    fn refresh_selectors_if_needed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let current_ids = Self::config_option_ids(&self.config_options);
        if current_ids != self.config_option_ids {
            self.config_option_ids = current_ids;
            self.rebuild_selectors(window, cx);
        }
    }

    fn rebuild_selectors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.selectors = Self::build_selectors(
            &self.config_options,
            &self.agent_server,
            &self.fs,
            window,
            cx,
        );
        cx.notify();
    }

    fn build_selectors(
        config_options: &Rc<dyn AgentSessionConfigOptions>,
        agent_server: &Rc<dyn AgentServer>,
        fs: &Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Entity<ConfigOptionSelector>> {
        config_options
            .config_options()
            .into_iter()
            .map(|option| {
                let config_options = config_options.clone();
                let agent_server = agent_server.clone();
                let fs = fs.clone();
                cx.new(|cx| {
                    ConfigOptionSelector::new(
                        config_options,
                        option.id.clone(),
                        agent_server,
                        fs,
                        window,
                        cx,
                    )
                })
            })
            .collect()
    }
}

impl Render for ConfigOptionsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.selectors.is_empty() {
            return div().into_any_element();
        }

        h_flex()
            .gap_1()
            .children(self.selectors.iter().cloned())
            .into_any_element()
    }
}

struct ConfigOptionSelector {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    config_id: acp::SessionConfigId,
    picker_handle: PopoverMenuHandle<Picker<ConfigOptionPickerDelegate>>,
    picker: Entity<Picker<ConfigOptionPickerDelegate>>,
    setting_value: bool,
}

impl ConfigOptionSelector {
    pub fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        config_id: acp::SessionConfigId,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let option_count = config_options
            .config_options()
            .iter()
            .find(|opt| opt.id == config_id)
            .map(count_config_options)
            .unwrap_or(0);

        let is_searchable = option_count >= PICKER_THRESHOLD;

        let picker = {
            let config_options = config_options.clone();
            let config_id = config_id.clone();
            let agent_server = agent_server.clone();
            let fs = fs.clone();
            cx.new(move |picker_cx| {
                let delegate = ConfigOptionPickerDelegate::new(
                    config_options,
                    config_id,
                    agent_server,
                    fs,
                    window,
                    picker_cx,
                );

                if is_searchable {
                    Picker::list(delegate, window, picker_cx)
                } else {
                    Picker::nonsearchable_list(delegate, window, picker_cx)
                }
                .show_scrollbar(true)
                .width(rems(20.))
                .max_height(Some(rems(20.).into()))
            })
        };

        Self {
            config_options,
            config_id,
            picker_handle: PopoverMenuHandle::default(),
            picker,
            setting_value: false,
        }
    }

    fn current_option(&self) -> Option<acp::SessionConfigOption> {
        self.config_options
            .config_options()
            .into_iter()
            .find(|opt| opt.id == self.config_id)
    }

    fn current_value_name(&self) -> String {
        let Some(option) = self.current_option() else {
            return "Unknown".to_string();
        };

        match &option.kind {
            acp::SessionConfigKind::Select(select) => {
                find_option_name(&select.options, &select.current_value)
                    .unwrap_or_else(|| "Unknown".to_string())
            }
            _ => "Unknown".to_string(),
        }
    }

    fn render_trigger_button(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Button {
        let Some(option) = self.current_option() else {
            return Button::new("config-option-trigger", "Unknown")
                .label_size(LabelSize::Small)
                .color(Color::Muted)
                .disabled(true);
        };

        let icon = if self.picker_handle.is_deployed() {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        Button::new(
            ElementId::Name(format!("config-option-{}", option.id.0).into()),
            self.current_value_name(),
        )
        .label_size(LabelSize::Small)
        .color(Color::Muted)
        .icon(icon)
        .icon_size(IconSize::XSmall)
        .icon_position(IconPosition::End)
        .icon_color(Color::Muted)
        .disabled(self.setting_value)
    }
}

impl Render for ConfigOptionSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(option) = self.current_option() else {
            return div().into_any_element();
        };

        let trigger_button = self.render_trigger_button(window, cx);

        let option_name = option.name.clone();
        let option_description: Option<SharedString> = option.description.map(Into::into);

        let tooltip = Tooltip::element(move |_window, _cx| {
            let mut content = v_flex().gap_1().child(Label::new(option_name.clone()));
            if let Some(desc) = option_description.as_ref() {
                content = content.child(
                    Label::new(desc.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                );
            }
            content.into_any()
        });

        PickerPopoverMenu::new(
            self.picker.clone(),
            trigger_button,
            tooltip,
            gpui::Corner::BottomRight,
            cx,
        )
        .with_handle(self.picker_handle.clone())
        .render(window, cx)
        .into_any_element()
    }
}

#[derive(Clone)]
enum ConfigOptionPickerEntry {
    Separator(SharedString),
    Option(ConfigOptionValue),
}

#[derive(Clone)]
struct ConfigOptionValue {
    value: acp::SessionConfigValueId,
    name: String,
    description: Option<String>,
    group: Option<String>,
}

struct ConfigOptionPickerDelegate {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    config_id: acp::SessionConfigId,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    filtered_entries: Vec<ConfigOptionPickerEntry>,
    all_options: Vec<ConfigOptionValue>,
    selected_index: usize,
    selected_description: Option<(usize, SharedString, bool)>,
    favorites: HashSet<acp::SessionConfigValueId>,
    _settings_subscription: Subscription,
}

impl ConfigOptionPickerDelegate {
    fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        config_id: acp::SessionConfigId,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Self {
        let favorites = agent_server.favorite_config_option_value_ids(&config_id, cx);

        let all_options = extract_options(&config_options, &config_id);
        let filtered_entries = options_to_picker_entries(&all_options, &favorites);

        let current_value = get_current_value(&config_options, &config_id);
        let selected_index = current_value
            .and_then(|current| {
                filtered_entries.iter().position(|entry| {
                    matches!(entry, ConfigOptionPickerEntry::Option(opt) if opt.value == current)
                })
            })
            .unwrap_or(0);

        let agent_server_for_subscription = agent_server.clone();
        let config_id_for_subscription = config_id.clone();
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |picker, window, cx| {
                let new_favorites = agent_server_for_subscription
                    .favorite_config_option_value_ids(&config_id_for_subscription, cx);
                if new_favorites != picker.delegate.favorites {
                    picker.delegate.favorites = new_favorites;
                    picker.refresh(window, cx);
                }
            });

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
            favorites,
            _settings_subscription: settings_subscription,
        }
    }

    fn current_value(&self) -> Option<acp::SessionConfigValueId> {
        get_current_value(&self.config_options, &self.config_id)
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
                this.delegate.filtered_entries =
                    options_to_picker_entries(&filtered_options, &this.delegate.favorites);

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
                    .default_config_option(self.config_id.0.as_ref(), cx);
                let is_default = default_value.as_deref() == Some(&*option.value.0);

                self.agent_server.set_default_config_option(
                    self.config_id.0.as_ref(),
                    if is_default {
                        None
                    } else {
                        Some(option.value.0.as_ref())
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
                    .default_config_option(self.config_id.0.as_ref(), cx);
                let is_default = default_value.as_deref() == Some(&*option.value.0);

                let is_favorite = self.favorites.contains(&option.value);

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
                                .spacing(ListItemSpacing::Sparse)
                                .toggle_state(selected)
                                .child(h_flex().w_full().child(Label::new(option_name).truncate()))
                                .end_slot(div().pr_2().when(is_selected, |this| {
                                    this.child(Icon::new(IconName::Check).color(Color::Accent))
                                }))
                                .end_hover_slot(div().pr_1p5().child({
                                    let (icon, color, tooltip) = if is_favorite {
                                        (IconName::StarFilled, Color::Accent, "Unfavorite")
                                    } else {
                                        (IconName::Star, Color::Default, "Favorite")
                                    };

                                    let config_id = self.config_id.clone();
                                    let value_id = option.value.clone();
                                    let agent_server = self.agent_server.clone();
                                    let fs = self.fs.clone();

                                    IconButton::new(("toggle-favorite-config-option", ix), icon)
                                        .layer(ElevationIndex::ElevatedSurface)
                                        .icon_color(color)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text(tooltip))
                                        .on_click(move |_, _, cx| {
                                            agent_server.toggle_favorite_config_option_value(
                                                config_id.clone(),
                                                value_id.clone(),
                                                !is_favorite,
                                                fs.clone(),
                                                cx,
                                            );
                                        })
                                })),
                        )
                        .into_any_element(),
                )
            }
        }
    }

    fn documentation_aside(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<ui::DocumentationAside> {
        self.selected_description
            .as_ref()
            .map(|(_, description, is_default)| {
                let description = description.clone();
                let is_default = *is_default;

                let settings = AgentSettings::get_global(cx);
                let side = match settings.dock {
                    settings::DockPosition::Left => DocumentationSide::Right,
                    settings::DockPosition::Bottom | settings::DockPosition::Right => {
                        DocumentationSide::Left
                    }
                };

                ui::DocumentationAside::new(
                    side,
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

    fn documentation_aside_index(&self) -> Option<usize> {
        self.selected_description.as_ref().map(|(ix, _, _)| *ix)
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

fn options_to_picker_entries(
    options: &[ConfigOptionValue],
    favorites: &HashSet<acp::SessionConfigValueId>,
) -> Vec<ConfigOptionPickerEntry> {
    let mut entries = Vec::new();

    let mut favorite_options = Vec::new();

    for option in options {
        if favorites.contains(&option.value) {
            favorite_options.push(option.clone());
        }
    }

    if !favorite_options.is_empty() {
        entries.push(ConfigOptionPickerEntry::Separator("Favorites".into()));
        for option in favorite_options {
            entries.push(ConfigOptionPickerEntry::Option(option));
        }

        // If the remaining list would start ungrouped (group == None), insert a separator so
        // Favorites doesn't visually run into the main list.
        if let Some(option) = options.first()
            && option.group.is_none()
        {
            entries.push(ConfigOptionPickerEntry::Separator("All Options".into()));
        }
    }

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

fn find_option_name(
    options: &acp::SessionConfigSelectOptions,
    value_id: &acp::SessionConfigValueId,
) -> Option<String> {
    match options {
        acp::SessionConfigSelectOptions::Ungrouped(opts) => opts
            .iter()
            .find(|o| &o.value == value_id)
            .map(|o| o.name.clone()),
        acp::SessionConfigSelectOptions::Grouped(groups) => groups.iter().find_map(|group| {
            group
                .options
                .iter()
                .find(|o| &o.value == value_id)
                .map(|o| o.name.clone())
        }),
        _ => None,
    }
}

fn count_config_options(option: &acp::SessionConfigOption) -> usize {
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
