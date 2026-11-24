use std::{rc::Rc, sync::Arc};

use acp_thread::AgentSessionConfigOptions;
use agent_client_protocol::v2::schema as acp;
use agent_servers::AgentServer;

use collections::HashSet;
use fs::Fs;
use fuzzy_nucleo::StringMatchCandidate;
use gpui::{
    App, BackgroundExecutor, Context, DismissEvent, Entity, Focusable as _, Subscription, Task,
    Window, prelude::*,
};
use menu::SelectParent;
use picker::popover_menu::PickerPopoverMenu;
use picker::{Picker, PickerDelegate};
use settings::{AgentConfigOptionValue, SettingsStore};
use ui::{
    ContextMenu, ContextMenuEntry, ElevationIndex, IconButton, KeyBinding, LabelLike, ListItem,
    ListItemSpacing, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};
use unicode_segmentation::UnicodeSegmentation;
use util::ResultExt as _;
use workspace::Workspace;
use zed_actions::agent::ToggleModelSelector;

use crate::model_search;
use crate::ui::{
    description_tooltip, documentation_aside_side, fast_mode_button, thinking_effort_trigger,
};
use crate::{
    CycleFavoriteModels, CycleModeSelector, CycleThinkingEffort, ToggleFastMode,
    ToggleProfileSelector, ToggleThinkingEffortMenu,
};

const PICKER_THRESHOLD: usize = 5;

fn toolbar_category(
    option: &acp::SessionConfigOption,
) -> Option<&acp::SessionConfigOptionCategory> {
    use acp::SessionConfigOptionCategory::*;

    let category = option.category.as_ref()?;
    match (&option.kind, category) {
        (acp::SessionConfigKind::Select(_), Model | ThoughtLevel | Mode)
            if count_config_options(option) > 0 =>
        {
            Some(category)
        }
        (acp::SessionConfigKind::Boolean(_), Speed) => Some(category),
        _ => None,
    }
}

fn split_config_options(
    options: Vec<acp::SessionConfigOption>,
    read_only: bool,
) -> (Vec<acp::SessionConfigOption>, Vec<acp::SessionConfigOption>) {
    use acp::SessionConfigOptionCategory::*;

    let mut options: Vec<_> = options
        .into_iter()
        .filter(|option| match &option.kind {
            acp::SessionConfigKind::Select(_) => read_only || count_config_options(option) > 0,
            acp::SessionConfigKind::Boolean(_) => true,
            _ => false,
        })
        .collect();
    if read_only {
        options.reverse();
        return (options, Vec::new());
    }

    let toolbar = [Speed, ThoughtLevel, Mode, Model]
        .iter()
        .filter_map(|category| {
            let index = options
                .iter()
                .position(|option| toolbar_category(option) == Some(category))?;
            Some(options.remove(index))
        })
        .collect();
    (toolbar, options)
}

pub struct ConfigOptionsView {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    selectors: Vec<Entity<ConfigOptionSelector>>,
    has_overflow: bool,
    overflow_menu: PopoverMenuHandle<ContextMenu>,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
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
        let (selectors, has_overflow) =
            Self::build_selectors(&config_options, &agent_server, &fs, window, cx);

        let rx = config_options.watch(cx);
        let refresh_task = cx.spawn_in(window, async move |this, cx| {
            if let Some(mut rx) = rx {
                while let Ok(()) = rx.recv().await {
                    this.update_in(cx, |this, window, cx| {
                        this.rebuild_selectors(window, cx);
                    })
                    .log_err();
                }
            }
        });

        Self {
            config_options,
            selectors,
            has_overflow,
            overflow_menu: PopoverMenuHandle::default(),
            agent_server,
            fs,
            _refresh_task: refresh_task,
        }
    }

    pub fn is_read_only(&self) -> bool {
        self.config_options.is_read_only()
    }

    pub(crate) fn response_controls<'a>(
        &'a self,
        cx: &'a App,
    ) -> impl Iterator<Item = AnyElement> + 'a {
        self.selectors
            .iter()
            .filter(|selector| selector.read(cx).is_response_control)
            .map(|selector| selector.clone().into_any_element())
    }

    pub fn toggle_category_picker(
        &mut self,
        category: acp::SessionConfigOptionCategory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(config_id) = self
            .config_options
            .config_options()
            .into_iter()
            .find(|option| {
                toolbar_category(option) == Some(&category)
                    && matches!(&option.kind, acp::SessionConfigKind::Select(_))
            })
            .map(|option| option.config_id)
        else {
            return false;
        };

        let Some(selector) = self.selector_for_config_id(&config_id, cx) else {
            return false;
        };

        selector.update(cx, |selector, cx| selector.toggle_picker(window, cx))
    }

    pub fn toggle_fast_mode(&self, window: &Window, cx: &mut App) {
        let Some(option) = self
            .config_options
            .config_options()
            .into_iter()
            .find(|option| {
                toolbar_category(option) == Some(&acp::SessionConfigOptionCategory::Speed)
            })
        else {
            return;
        };

        toggle_boolean_config_option(
            self.config_options.clone(),
            self.agent_server.clone(),
            option.config_id,
            self.fs.clone(),
            window,
            cx,
        );
    }

    pub fn cycle_category_option(
        &mut self,
        category: acp::SessionConfigOptionCategory,
        favorites_only: bool,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.config_options.is_read_only() {
            return false;
        }
        let Some(config_id) = self
            .config_options
            .config_options()
            .into_iter()
            .filter(|option| {
                option.category.as_ref() == Some(&category)
                    && match &option.kind {
                        acp::SessionConfigKind::Select(_) => count_config_options(option) > 0,
                        acp::SessionConfigKind::Boolean(_) => !favorites_only,
                        _ => false,
                    }
            })
            .min_by_key(|option| toolbar_category(option).is_none())
            .map(|option| option.config_id)
        else {
            return false;
        };

        let Some(next_value) = self.next_value_for_config(&config_id, favorites_only, cx) else {
            return false;
        };
        set_config_option_and_default(
            self.config_options.clone(),
            self.agent_server.clone(),
            config_id,
            next_value,
            self.fs.clone(),
            window,
            cx,
        )
        .detach_and_log_err(cx);

        true
    }

    fn selector_for_config_id(
        &self,
        config_id: &acp::SessionConfigId,
        cx: &App,
    ) -> Option<Entity<ConfigOptionSelector>> {
        self.selectors
            .iter()
            .find(|selector| selector.read(cx).config_id() == config_id)
            .cloned()
    }

    fn next_value_for_config(
        &self,
        config_id: &acp::SessionConfigId,
        favorites_only: bool,
        cx: &mut Context<Self>,
    ) -> Option<acp::SessionConfigOptionValue> {
        let option = self
            .config_options
            .config_options()
            .into_iter()
            .find(|option| &option.config_id == config_id)?;

        match &option.kind {
            acp::SessionConfigKind::Select(_) => {
                let mut options = extract_options(&self.config_options, config_id);
                if options.is_empty() {
                    return None;
                }

                if favorites_only {
                    let favorites = self
                        .agent_server
                        .favorite_config_option_value_ids(config_id, cx);
                    options.retain(|option| favorites.contains(&option.value));
                    if options.is_empty() {
                        return None;
                    }
                }

                let current_value = get_current_select_value(&self.config_options, config_id);
                let current_index = current_value
                    .as_ref()
                    .and_then(|current| options.iter().position(|option| &option.value == current))
                    .unwrap_or(usize::MAX);

                let next_index = if current_index == usize::MAX {
                    0
                } else {
                    (current_index + 1) % options.len()
                };

                Some(acp::SessionConfigOptionValue::id(
                    options[next_index].value.clone(),
                ))
            }
            acp::SessionConfigKind::Boolean(boolean) => {
                if favorites_only {
                    None
                } else {
                    Some(acp::SessionConfigOptionValue::boolean(
                        !boolean.current_value,
                    ))
                }
            }
            _ => None,
        }
    }

    fn rebuild_selectors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Config option updates can mutate option values for existing IDs (for example,
        // reasoning levels after a model switch). Rebuild to refresh cached picker entries.
        self.overflow_menu.hide(cx);
        for selector in &self.selectors {
            if let Some(handle) = &selector.read(cx).picker_handle {
                handle.clone().hide(cx);
            }
        }
        (self.selectors, self.has_overflow) = Self::build_selectors(
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
    ) -> (Vec<Entity<ConfigOptionSelector>>, bool) {
        let (toolbar, overflow) = split_config_options(
            config_options.config_options(),
            config_options.is_read_only(),
        );
        let selectors = toolbar
            .into_iter()
            .map(|option| {
                let config_options = config_options.clone();
                let agent_server = agent_server.clone();
                let fs = fs.clone();
                cx.new(|cx| {
                    ConfigOptionSelector::new(config_options, option, agent_server, fs, window, cx)
                })
            })
            .collect();
        (selectors, !overflow.is_empty())
    }

    fn build_overflow_menu(
        &self,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Entity<ContextMenu>> {
        let (_, overflow) =
            split_config_options(self.config_options.config_options(), self.is_read_only());
        if overflow.is_empty() {
            return None;
        }

        let format = self.config_options.description_format();
        Some(ContextMenu::build(window, cx, |mut menu, _, cx| {
            menu = menu.header("Agent Options");
            for option in overflow {
                let config_options = self.config_options.clone();
                let agent_server = self.agent_server.clone();
                let fs = self.fs.clone();
                let config_id = option.config_id.clone();

                match &option.kind {
                    acp::SessionConfigKind::Boolean(boolean) => {
                        let entry = ContextMenuEntry::new(option.name)
                            .toggleable(IconPosition::End, boolean.current_value)
                            .handler(move |window, cx| {
                                toggle_boolean_config_option(
                                    config_options.clone(),
                                    agent_server.clone(),
                                    config_id.clone(),
                                    fs.clone(),
                                    window,
                                    cx,
                                );
                            })
                            .when_some(option.description, |entry, description| {
                                entry.documentation_aside(documentation_aside_side(cx), move |_| {
                                    crate::ui::description(&description, format)
                                })
                            });
                        menu = menu.item(entry);
                    }
                    acp::SessionConfigKind::Select(select) => {
                        let label = find_option_name(&select.options, &select.current_value)
                            .map_or_else(
                                || option.name.clone(),
                                |value| format!("{}: {}", option.name, truncate_value_name(&value)),
                            );
                        menu = menu.submenu(label, move |menu, window, cx| {
                            let Some(option) =
                                config_options.config_options().into_iter().find(|option| {
                                    option.config_id == config_id
                                        && count_config_options(option) > 0
                                })
                            else {
                                return menu;
                            };
                            let picker = build_config_option_picker(
                                config_options.clone(),
                                &option,
                                agent_server.clone(),
                                fs.clone(),
                                true,
                                window,
                                cx,
                            );
                            cx.subscribe_in(
                                &picker,
                                window,
                                |menu, _, _: &DismissEvent, window, cx| {
                                    menu.select_submenu_parent(&SelectParent, window, cx);
                                },
                            )
                            .detach();
                            let focus_handle = picker.focus_handle(cx);
                            menu.embedded_view(picker).focus_content(focus_handle)
                        });
                    }
                    _ => {}
                }
            }
            menu
        }))
    }

    fn render_overflow_menu(&self, cx: &Context<Self>) -> PopoverMenu<ContextMenu> {
        let view = cx.weak_entity();
        PopoverMenu::new("agent-options-menu")
            .trigger_with_tooltip(
                IconButton::new("agent-options", IconName::Filter)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .selected_style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .aria_label("Agent Options"),
                Tooltip::text("Agent Options"),
            )
            .anchor(gpui::Anchor::BottomRight)
            .offset(gpui::point(px(0.), px(-2.)))
            .with_handle(self.overflow_menu.clone())
            .menu(move |window, cx| {
                view.upgrade()?
                    .update(cx, |view, cx| view.build_overflow_menu(window, cx))
            })
    }
}

impl Render for ConfigOptionsView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .min_w_0()
            .flex_wrap()
            .gap_1()
            .when(self.has_overflow, |this| {
                this.child(self.render_overflow_menu(cx))
            })
            .children(
                self.selectors
                    .iter()
                    .filter(|selector| !selector.read(cx).is_response_control)
                    .cloned(),
            )
    }
}

struct ConfigOptionSelector {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    config_id: acp::SessionConfigId,
    is_response_control: bool,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    picker_handle: Option<PopoverMenuHandle<Picker<ConfigOptionPickerDelegate>>>,
    picker: Option<Entity<Picker<ConfigOptionPickerDelegate>>>,
}

impl ConfigOptionSelector {
    fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        option: acp::SessionConfigOption,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let read_only = config_options.is_read_only();
        let is_response_control = !read_only
            && matches!(
                toolbar_category(&option),
                Some(
                    acp::SessionConfigOptionCategory::Speed
                        | acp::SessionConfigOptionCategory::ThoughtLevel
                )
            );
        let (picker_handle, picker) =
            if !read_only && matches!(&option.kind, acp::SessionConfigKind::Select(_)) {
                let picker = build_config_option_picker(
                    config_options.clone(),
                    &option,
                    agent_server.clone(),
                    fs.clone(),
                    false,
                    window,
                    cx,
                );
                (Some(PopoverMenuHandle::default()), Some(picker))
            } else {
                (None, None)
            };

        Self {
            config_options,
            config_id: option.config_id,
            is_response_control,
            agent_server,
            fs,
            picker_handle,
            picker,
        }
    }

    fn current_option(&self) -> Option<acp::SessionConfigOption> {
        self.config_options
            .config_options()
            .into_iter()
            .find(|opt| opt.config_id == self.config_id)
    }

    fn config_id(&self) -> &acp::SessionConfigId {
        &self.config_id
    }

    fn toggle_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.config_options.is_read_only()
            || !self
                .current_option()
                .is_some_and(|option| count_config_options(&option) > 0)
        {
            return false;
        }

        if let Some(picker_handle) = &self.picker_handle {
            picker_handle.toggle(window, cx);
            true
        } else {
            false
        }
    }
}

impl Render for ConfigOptionSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(option) = self.current_option() else {
            return div().into_any_element();
        };

        let format = self.config_options.description_format();
        if self.config_options.is_read_only() {
            let value = match &option.kind {
                acp::SessionConfigKind::Select(select) => {
                    find_option_name(&select.options, &select.current_value)
                        .unwrap_or_else(|| "Unknown".to_string())
                }
                acp::SessionConfigKind::Boolean(boolean) => {
                    format!(
                        "{}: {}",
                        option.name,
                        if boolean.current_value { "On" } else { "Off" }
                    )
                }
                _ => return div().into_any_element(),
            };
            return div()
                .id(ElementId::Name(
                    format!("config-option-{}", option.config_id.0).into(),
                ))
                .px_1()
                .child(Label::new(value).size(LabelSize::Small).color(Color::Muted))
                .map(|this| match option.description {
                    Some(text) => this.tooltip(description_tooltip(text.into(), format)),
                    None => this.tooltip(Tooltip::text(option.name)),
                })
                .into_any_element();
        }

        match &option.kind {
            acp::SessionConfigKind::Select(select) => {
                let (Some(picker), Some(picker_handle)) =
                    (self.picker.clone(), self.picker_handle.clone())
                else {
                    return div().into_any_element();
                };

                let id = ElementId::Name(format!("config-option-{}", option.config_id.0).into());
                let value_name = find_option_name(&select.options, &select.current_value);
                let value_name = value_name.as_deref().unwrap_or(&option.name);
                let aria_label = format!("{}: {value_name}", option.name);
                let label = truncate_value_name(value_name);
                let menu_open = picker_handle.is_deployed();

                let option_category = option.category.clone();
                let option_name = option.name;
                let option_description = option.description;

                let tooltip = Tooltip::element(move |_window, cx| {
                    let mut content = v_flex().gap_1().child(Label::new(option_name.clone()));
                    if let Some(desc) = option_description.as_ref() {
                        content = content.child(
                            LabelLike::new()
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .child(crate::ui::description(desc, format)),
                        );
                    }

                    let action_tooltip_container = |label: &str, keybinding: KeyBinding| {
                        h_flex()
                            .pt_1()
                            .gap_2()
                            .justify_between()
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(Label::new(label))
                            .child(keybinding)
                    };

                    if let Some(category) = &option_category {
                        match category {
                            acp::SessionConfigOptionCategory::Mode => {
                                content = content
                                    .child(action_tooltip_container(
                                        "Change Mode",
                                        KeyBinding::for_action(&ToggleProfileSelector, cx),
                                    ))
                                    .child(action_tooltip_container(
                                        "Cycle Through Modes",
                                        KeyBinding::for_action(&CycleModeSelector, cx),
                                    ));
                            }
                            acp::SessionConfigOptionCategory::Model => {
                                content = content
                                    .child(action_tooltip_container(
                                        "Change Model",
                                        KeyBinding::for_action(&ToggleModelSelector, cx),
                                    ))
                                    .child(action_tooltip_container(
                                        "Cycle Favorite Models",
                                        KeyBinding::for_action(&CycleFavoriteModels, cx),
                                    ));
                            }
                            acp::SessionConfigOptionCategory::ThoughtLevel => {
                                content = content
                                    .child(action_tooltip_container(
                                        "Change Thinking Effort",
                                        KeyBinding::for_action(&ToggleThinkingEffortMenu, cx),
                                    ))
                                    .child(action_tooltip_container(
                                        "Cycle Thinking Effort",
                                        KeyBinding::for_action(&CycleThinkingEffort, cx),
                                    ));
                            }
                            _ => {}
                        }
                    }
                    content.into_any()
                });

                if self.is_response_control {
                    return PickerPopoverMenu::new(
                        picker,
                        thinking_effort_trigger(id, label, menu_open, true).aria_label(aria_label),
                        tooltip,
                        gpui::Anchor::BottomLeft,
                        cx,
                    )
                    .with_handle(picker_handle)
                    .render(window, cx)
                    .into_any_element();
                }

                let (chevron, color) = if menu_open {
                    (IconName::ChevronUp, Color::Accent)
                } else {
                    (IconName::ChevronDown, Color::Muted)
                };
                let trigger_button = Button::new(id, label)
                    .label_size(LabelSize::Small)
                    .aria_label(aria_label)
                    .color(color)
                    .end_icon(Icon::new(chevron).size(IconSize::XSmall).color(color))
                    .selected_style(ButtonStyle::Tinted(ui::TintColor::Accent));
                PickerPopoverMenu::new(
                    picker,
                    trigger_button,
                    tooltip,
                    gpui::Anchor::BottomRight,
                    cx,
                )
                .with_handle(picker_handle)
                .render(window, cx)
                .into_any_element()
            }
            acp::SessionConfigKind::Boolean(boolean) => {
                let (action, state) = if boolean.current_value {
                    ("Disable", "On")
                } else {
                    ("Enable", "Off")
                };
                let label = format!("{}: {state}", option.name);
                let tooltip_label = format!("{action} {}", option.name);
                let option_description = option.description;

                fast_mode_button(
                    ElementId::Name(format!("config-option-{}", option.config_id.0).into()),
                    boolean.current_value,
                    cx,
                )
                .aria_label(label)
                .tooltip(Tooltip::element(move |_window, cx| {
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .justify_between()
                                .child(Label::new(tooltip_label.clone()))
                                .child(KeyBinding::for_action(&ToggleFastMode, cx)),
                        )
                        .when_some(option_description.as_ref(), |this, description| {
                            this.child(
                                LabelLike::new()
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .child(crate::ui::description(description, format)),
                            )
                        })
                        .into_any()
                }))
                .on_click(cx.listener(|this, _, window, cx| {
                    toggle_boolean_config_option(
                        this.config_options.clone(),
                        this.agent_server.clone(),
                        this.config_id.clone(),
                        this.fs.clone(),
                        window,
                        cx,
                    );
                }))
                .into_any_element()
            }
            _ => div().into_any_element(),
        }
    }
}

fn truncate_value_name(value_name: &str) -> String {
    let mut graphemes = value_name.graphemes(true);
    let mut truncated = graphemes.by_ref().take(32).collect::<String>();
    if graphemes.next().is_some() {
        truncated.push('…');
    }
    truncated
}

fn build_config_option_picker(
    config_options: Rc<dyn AgentSessionConfigOptions>,
    option: &acp::SessionConfigOption,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    embedded: bool,
    window: &mut Window,
    cx: &mut App,
) -> Entity<Picker<ConfigOptionPickerDelegate>> {
    let searchable = count_config_options(option) >= PICKER_THRESHOLD;
    cx.new(|cx| {
        let mut delegate = ConfigOptionPickerDelegate::new(
            config_options,
            option.config_id.clone(),
            agent_server,
            fs,
            window,
            cx,
        );
        // Overflow pickers have no toolbar trigger to show the option's description.
        if embedded {
            delegate.option_description = option.description.clone().map(Into::into);
        }
        let picker = if searchable {
            Picker::list(delegate, window, cx)
        } else {
            Picker::nonsearchable_list(delegate, window, cx)
        }
        .show_scrollbar(true)
        .initial_width(rems(20.));
        if embedded {
            picker.embedded().reopenable(false, cx)
        } else {
            picker
        }
    })
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
    option_description: Option<SharedString>,
    selected_index: usize,
    selected_description: Option<(usize, SharedString)>,
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

        let current_value = get_current_select_value(&config_options, &config_id);
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
            option_description: None,
            selected_index,
            selected_description: None,
            favorites,
            _settings_subscription: settings_subscription,
        }
    }

    fn current_value(&self) -> Option<acp::SessionConfigValueId> {
        get_current_select_value(&self.config_options, &self.config_id)
    }
}

impl PickerDelegate for ConfigOptionPickerDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "config options"
    }

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

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        match self.filtered_entries.get(ix) {
            Some(ConfigOptionPickerEntry::Option(option)) => {
                extract_options(&self.config_options, &self.config_id)
                    .iter()
                    .any(|current| current.value == option.value)
            }
            Some(ConfigOptionPickerEntry::Separator(_)) | None => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select an option…".into()
    }

    fn render_header(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let description = self.option_description.clone()?;
        let format = self.config_options.description_format();
        Some(
            div()
                .id("config-option-description")
                .debug_selector(|| "config-option-description".into())
                .px_2()
                .py_1()
                .tooltip(description_tooltip(description.clone(), format))
                .child(
                    LabelLike::new()
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                        .line_clamp(3)
                        .child(crate::ui::description(description, format)),
                )
                .into_any_element(),
        )
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
            // A deployed picker may outlive the option kind or choices that created it.
            if !extract_options(&self.config_options, &self.config_id)
                .iter()
                .any(|current| current.value == option.value)
            {
                return;
            }

            set_config_option_and_default(
                self.config_options.clone(),
                self.agent_server.clone(),
                self.config_id.clone(),
                acp::SessionConfigOptionValue::id(option.value.clone()),
                self.fs.clone(),
                window,
                cx,
            )
            .detach_and_log_err(cx);

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
                                    menu.delegate.selected_description = Some((ix, desc.clone()));
                                } else if matches!(menu.delegate.selected_description, Some((id, _)) if id == ix)
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
                                .end_slot_on_hover(div().pr_1p5().child({
                                    let (icon, color, tooltip) = if is_favorite {
                                        (IconName::StarFilled, Color::Accent, "Unfavorite")
                                    } else {
                                        (IconName::Star, Color::Default, "Favorite")
                                    };

                                    let config_id = self.config_id.clone();
                                    let value_id = option.value.clone();
                                    let agent_server = self.agent_server.clone();
                                    let fs = self.fs.clone();
                                    let config_options = self.config_options.clone();

                                    IconButton::new(("toggle-favorite-config-option", ix), icon)
                                        .layer(ElevationIndex::ElevatedSurface)
                                        .icon_color(color)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text(tooltip))
                                        .on_click(move |_, _, cx| {
                                            if !extract_options(&config_options, &config_id)
                                                .iter()
                                                .any(|current| current.value == value_id)
                                            {
                                                return;
                                            }

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
        self.selected_description.as_ref().map(|(_, description)| {
            let description = description.clone();
            let format = self.config_options.description_format();

            let side = documentation_aside_side(cx);

            ui::DocumentationAside::new(
                side,
                Rc::new(move |_| crate::ui::description(description.clone(), format)),
            )
        })
    }

    fn documentation_aside_index(&self) -> Option<usize> {
        self.selected_description.as_ref().map(|(ix, _)| *ix)
    }
}

fn extract_options(
    config_options: &Rc<dyn AgentSessionConfigOptions>,
    config_id: &acp::SessionConfigId,
) -> Vec<ConfigOptionValue> {
    let Some(option) = config_options
        .config_options()
        .into_iter()
        .find(|opt| &opt.config_id == config_id)
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

fn get_current_select_value(
    config_options: &Rc<dyn AgentSessionConfigOptions>,
    config_id: &acp::SessionConfigId,
) -> Option<acp::SessionConfigValueId> {
    config_options
        .config_options()
        .into_iter()
        .find(|opt| &opt.config_id == config_id)
        .and_then(|opt| match &opt.kind {
            acp::SessionConfigKind::Select(select) => Some(select.current_value.clone()),
            _ => None,
        })
}

fn set_config_option_and_default(
    config_options: Rc<dyn AgentSessionConfigOptions>,
    agent_server: Rc<dyn AgentServer>,
    config_id: acp::SessionConfigId,
    value: acp::SessionConfigOptionValue,
    fs: Arc<dyn Fs>,
    window: &Window,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    if config_options.is_read_only() {
        return Task::ready(Err(anyhow::anyhow!("Session configuration is read-only")));
    }
    let workspace = Workspace::for_window(window, cx).map(|workspace| workspace.downgrade());
    let response = config_options.set_config_option(config_id.clone(), value, cx);
    cx.spawn(async move |cx| {
        let options = match response.await {
            Ok(options) => options,
            Err(error) => {
                if let Some(workspace) = workspace.and_then(|workspace| workspace.upgrade()) {
                    workspace.update(cx, |workspace, cx| {
                        workspace.show_error(
                            anyhow::anyhow!("Could not change agent option: {error:#}"),
                            cx,
                        );
                    });
                }
                return Err(error);
            }
        };
        // Saving the request would make rejected or normalized choices future defaults.
        if let Some(value) = options
            .iter()
            .find(|option| option.config_id == config_id)
            .and_then(setting_value_for_config_option)
        {
            cx.update(|cx| {
                agent_server.set_default_config_option(config_id.0.as_ref(), Some(value), fs, cx);
            });
        }
        Ok(())
    })
}

fn toggle_boolean_config_option(
    config_options: Rc<dyn AgentSessionConfigOptions>,
    agent_server: Rc<dyn AgentServer>,
    config_id: acp::SessionConfigId,
    fs: Arc<dyn Fs>,
    window: &Window,
    cx: &mut App,
) {
    if config_options.is_read_only() {
        return;
    }
    let Some(acp::SessionConfigKind::Boolean(boolean)) = config_options
        .config_options()
        .into_iter()
        .find(|option| option.config_id == config_id)
        .map(|option| option.kind)
    else {
        return;
    };
    set_config_option_and_default(
        config_options,
        agent_server,
        config_id,
        acp::SessionConfigOptionValue::boolean(!boolean.current_value),
        fs,
        window,
        cx,
    )
    .detach_and_log_err(cx);
}

fn setting_value_for_config_option(
    option: &acp::SessionConfigOption,
) -> Option<AgentConfigOptionValue> {
    match &option.kind {
        acp::SessionConfigKind::Select(select)
            if find_option_name(&select.options, &select.current_value).is_some() =>
        {
            Some(AgentConfigOptionValue::ValueId(
                select.current_value.0.to_string(),
            ))
        }
        acp::SessionConfigKind::Boolean(boolean) => {
            Some(AgentConfigOptionValue::Boolean(boolean.current_value))
        }
        _ => None,
    }
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
    model_search::search(options, query, executor, |index, option| {
        (
            StringMatchCandidate::new(index, option.name.clone()),
            StringMatchCandidate::new(
                index,
                format!(
                    "{} {} {} {}",
                    option.name,
                    option.group.as_deref().unwrap_or_default(),
                    option.description.as_deref().unwrap_or_default(),
                    option.value.0,
                ),
            ),
        )
    })
    .await
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

#[cfg(test)]
mod tests {
    use super::*;
    use acp_thread::AgentConnection;
    use fs::FakeFs;
    use gpui::{Modifiers, TestAppContext, VisualTestContext};
    use parking_lot::Mutex;
    use project::{AgentId, Project};
    use std::{any::Any, cell::RefCell};

    #[test]
    fn composer_uses_the_first_matching_option_for_each_toolbar_control() {
        use acp::SessionConfigOptionCategory::*;

        let select = |id, category| {
            acp::SessionConfigOption::select(
                id,
                id,
                "value",
                vec![acp::SessionConfigSelectOption::new("value", "Value")],
            )
            .category(category)
        };
        let (toolbar, overflow) = split_config_options(
            vec![
                select("model", Model),
                acp::SessionConfigOption::boolean("extra-mode", "Extra Mode", true).category(Mode),
                select("effort", ThoughtLevel),
                select("another-model", Model),
                select("speed-select", Speed),
                acp::SessionConfigOption::boolean("fast-mode", "Fast Mode", true).category(Speed),
                acp::SessionConfigOption::boolean("tools", "Tools", true),
                acp::SessionConfigOption::select(
                    "empty-mode",
                    "Empty",
                    "none",
                    Vec::<acp::SessionConfigSelectOption>::new(),
                )
                .category(Mode),
                select("mode", Mode),
            ],
            false,
        );
        let ids = |options: Vec<acp::SessionConfigOption>| {
            options
                .into_iter()
                .map(|option| option.config_id.to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(ids(toolbar), ["fast-mode", "effort", "mode", "model"]);
        assert_eq!(
            ids(overflow),
            ["extra-mode", "another-model", "speed-select", "tools"]
        );
    }

    #[gpui::test]
    fn response_controls_render_separately_from_other_options(cx: &mut TestAppContext) {
        cx.update(init_test_ui);
        let config_options = Rc::new(TestSessionConfigOptions::new(Vec::new()));
        let agent_server = Rc::new(TestAgentServer::default());
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let (composer, cx) = cx.add_window_view({
            let config_options = config_options.clone();
            move |window, cx| {
                let options = cx
                    .new(|cx| ConfigOptionsView::new(config_options, agent_server, fs, window, cx));
                cx.observe(&options, |_, _, cx| cx.notify()).detach();
                TestConfigComposer { options }
            }
        });
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-FastForwardOff").is_none());

        config_options.replace_options(vec![
            acp::SessionConfigOption::boolean("fast", "Fast Mode", false)
                .category(acp::SessionConfigOptionCategory::Speed),
            acp::SessionConfigOption::select(
                "effort",
                "Effort",
                "medium",
                vec![
                    acp::SessionConfigSelectOption::new("medium", "Medium"),
                    acp::SessionConfigSelectOption::new("high", "High"),
                ],
            )
            .category(acp::SessionConfigOptionCategory::ThoughtLevel),
            acp::SessionConfigOption::boolean("tools", "Tools", false),
        ]);
        cx.run_until_parked();

        let response = cx
            .debug_bounds("ICON-FastForwardOff")
            .expect("fast mode control");
        let overflow = cx
            .debug_bounds("ICON-Filter")
            .expect("Agent Options button");
        assert!(response.right() < overflow.left());
        cx.simulate_click(response.center(), Modifiers::default());
        assert_eq!(
            config_options.set_values.borrow().as_slice(),
            &[("fast".into(), acp::SessionConfigOptionValue::boolean(true))]
        );

        let options = composer.read_with(cx, |composer, _| composer.options.clone());
        let effort_picker_deployed = options.update_in(cx, |options, window, cx| {
            options.toggle_category_picker(
                acp::SessionConfigOptionCategory::ThoughtLevel,
                window,
                cx,
            );
            options
                .selector_for_config_id(&"effort".into(), cx)
                .and_then(|selector| selector.read(cx).picker_handle.clone())
                .is_some_and(|handle| handle.is_deployed())
        });
        assert!(effort_picker_deployed);
    }

    #[gpui::test]
    fn overflow_options_stay_editable_after_a_config_update(cx: &mut TestAppContext) {
        cx.update(init_test_ui);
        let fast_mode = acp::SessionConfigOption::boolean("fast-mode", "Fast Mode", false)
            .category(acp::SessionConfigOptionCategory::Speed);
        let config_options = Rc::new(TestSessionConfigOptions::new(vec![fast_mode.clone()]));
        let agent_server = Rc::new(TestAgentServer::default());
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let (view, cx) = cx.add_window_view({
            let config_options = config_options.clone();
            let agent_server = agent_server.clone();
            move |window, cx| ConfigOptionsView::new(config_options, agent_server, fs, window, cx)
        });
        cx.run_until_parked();
        assert!(cx.debug_bounds("ICON-Filter").is_none());
        assert!(cx.debug_bounds("ICON-FastForwardOff").is_none());
        let initial_config_owners = Rc::strong_count(&config_options);

        config_options.replace_options(vec![
            fast_mode.clone(),
            acp::SessionConfigOption::boolean("web-search", "Web Search", false),
            acp::SessionConfigOption::select(
                "scope",
                "Scope",
                "choice-0",
                (0..PICKER_THRESHOLD)
                    .map(|index| {
                        acp::SessionConfigSelectOption::new(
                            format!("choice-{index}"),
                            format!("Choice {index}"),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
            .description("| Scope |\n| --- |\n| Folders the agent can see |"),
        ]);
        cx.run_until_parked();

        let overflow = cx
            .debug_bounds("ICON-Filter")
            .expect("Agent Options button");
        cx.simulate_click(overflow.center(), Modifiers::default());
        draw_menu_frames(cx);
        let boolean = cx
            .debug_bounds("MENU_ITEM-Web Search")
            .expect("overflow boolean row");
        cx.simulate_click(boolean.center(), Modifiers::default());
        cx.run_until_parked();

        cx.simulate_click(overflow.center(), Modifiers::default());
        draw_menu_frames(cx);
        assert!(cx.update(|window, cx| view.read(cx).overflow_menu.is_focused(window, cx)));
        cx.dispatch_action(menu::SelectLast);
        cx.dispatch_action(menu::Confirm);
        draw_menu_frames(cx);
        cx.run_until_parked();
        assert!(cx.debug_bounds("config-option-description").is_some());
        assert!(
            cx.debug_bounds("markdown_table").is_some(),
            "option description renders as Markdown"
        );
        cx.simulate_input("Choice 4");
        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();

        assert_eq!(
            config_options.set_values.borrow().as_slice(),
            &[
                (
                    "web-search".into(),
                    acp::SessionConfigOptionValue::boolean(true)
                ),
                (
                    "scope".into(),
                    acp::SessionConfigOptionValue::id("choice-4")
                ),
            ]
        );
        assert_eq!(
            agent_server.saved_defaults.lock().as_slice(),
            &[
                (
                    "web-search".into(),
                    Some(AgentConfigOptionValue::Boolean(true))
                ),
                (
                    "scope".into(),
                    Some(AgentConfigOptionValue::ValueId("choice-4".into()))
                ),
            ]
        );

        cx.simulate_click(overflow.center(), Modifiers::default());
        draw_menu_frames(cx);
        assert!(cx.update(|window, cx| view.read(cx).overflow_menu.is_focused(window, cx)));
        cx.dispatch_action(menu::SelectLast);
        cx.dispatch_action(menu::Confirm);
        draw_menu_frames(cx);
        assert!(cx.update(|window, cx| {
            let menu = &view.read(cx).overflow_menu;
            menu.is_deployed() && !menu.is_focused(window, cx) && window.focused(cx).is_some()
        }));
        config_options.replace_options(vec![fast_mode]);
        cx.run_until_parked();
        draw_menu_frames(cx);
        assert!(cx.debug_bounds("ICON-Filter").is_none());
        assert_eq!(
            Rc::strong_count(&config_options),
            initial_config_owners,
            "removing options must also release the open overflow menu and its picker"
        );
    }

    fn option_names(options: Vec<ConfigOptionValue>) -> Vec<String> {
        options.into_iter().map(|option| option.name).collect()
    }

    #[gpui::test]
    async fn config_option_search_matches_all_searchable_fields(executor: BackgroundExecutor) {
        let options = vec![
            ConfigOptionValue {
                value: acp::SessionConfigValueId::new("openai/gpt-5.6-sol"),
                name: "GPT-5.6 Sol".to_string(),
                description: Some("Flagship reasoning model".to_string()),
                group: Some("OpenAI".to_string()),
            },
            ConfigOptionValue {
                value: acp::SessionConfigValueId::new("anthropic/claude-sonnet-4"),
                name: "Claude Sonnet 4".to_string(),
                description: None,
                group: Some("Anthropic".to_string()),
            },
        ];

        for query in [
            "GPT 5.6",
            "GPT-5.6",
            "gpt5.6 sol",
            "openai",
            "openai/gpt-5.6-sol",
            "flagship",
            "openai reasoning",
        ] {
            assert_eq!(
                option_names(fuzzy_search_options(options.clone(), query, executor.clone()).await),
                ["GPT-5.6 Sol"],
                "query {query:?} should match the GPT model",
            );
        }
    }

    #[gpui::test]
    async fn config_option_search_ranks_names_before_metadata(executor: BackgroundExecutor) {
        let options = vec![
            ConfigOptionValue {
                value: acp::SessionConfigValueId::new("reasoner"),
                name: "Reasoner".to_string(),
                description: Some("High reasoning effort".to_string()),
                group: None,
            },
            ConfigOptionValue {
                value: acp::SessionConfigValueId::new("xhigh"),
                name: "Extra High".to_string(),
                description: None,
                group: None,
            },
            ConfigOptionValue {
                value: acp::SessionConfigValueId::new("high"),
                name: "High".to_string(),
                description: Some(
                    "High effort with advanced reasoning for complex tasks.".to_string(),
                ),
                group: None,
            },
        ];

        assert_eq!(
            option_names(fuzzy_search_options(options, "high", executor).await),
            ["High", "Extra High", "Reasoner"],
        );
    }

    #[gpui::test]
    async fn config_option_search_preserves_name_order_with_different_metadata(
        executor: BackgroundExecutor,
    ) {
        let options = vec![
            ConfigOptionValue {
                value: acp::SessionConfigValueId::new("model-a"),
                name: "Model A".to_string(),
                description: Some("Advanced reasoning for complex tasks".to_string()),
                group: Some("Provider A".to_string()),
            },
            ConfigOptionValue {
                value: acp::SessionConfigValueId::new("b"),
                name: "Model B".to_string(),
                description: None,
                group: None,
            },
        ];

        assert_eq!(
            option_names(fuzzy_search_options(options, "model", executor).await),
            ["Model A", "Model B"],
        );
    }

    #[gpui::test]
    async fn config_option_search_returns_more_than_one_hundred_matches(
        executor: BackgroundExecutor,
    ) {
        let options = (0..101)
            .map(|index| ConfigOptionValue {
                value: acp::SessionConfigValueId::new(format!("model-{index}")),
                name: format!("Model {index}"),
                description: None,
                group: None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            fuzzy_search_options(options, "model", executor).await.len(),
            101
        );
    }

    #[gpui::test]
    fn config_defaults_wait_for_authoritative_response(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        for boolean in [false, true] {
            let agent_server = Rc::new(TestAgentServer::default());
            let offered = if boolean {
                acp::SessionConfigOption::boolean("config", "Config", false)
            } else {
                acp::SessionConfigOption::select(
                    "config",
                    "Config",
                    "auto",
                    vec![
                        acp::SessionConfigSelectOption::new("auto", "Auto"),
                        acp::SessionConfigSelectOption::new("manual", "Manual"),
                    ],
                )
            }
            .category(acp::SessionConfigOptionCategory::ModelConfig);
            let config_options = Rc::new(TestSessionConfigOptions::new(vec![offered.clone()]));
            let _updates = config_options.updates.borrow().receiver();
            let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
            let view = cx.update(|window, cx| {
                cx.new(|cx| {
                    ConfigOptionsView::new(
                        config_options.clone(),
                        agent_server.clone(),
                        fs,
                        window,
                        cx,
                    )
                })
            });
            let rejected = config_options.defer_next_response();
            view.update_in(cx, |view, window, cx| {
                assert!(view.cycle_category_option(
                    acp::SessionConfigOptionCategory::ModelConfig,
                    false,
                    window,
                    cx
                ));
            });
            assert!(
                agent_server.saved_defaults.lock().is_empty(),
                "do not persist before acceptance"
            );
            rejected
                .send(Err(anyhow::anyhow!("rejected")))
                .expect("config response");
            cx.run_until_parked();
            assert!(
                agent_server.saved_defaults.lock().is_empty(),
                "rejection preserves preferences"
            );

            let accepted = config_options.defer_next_response();
            view.update_in(cx, |view, window, cx| {
                assert!(view.cycle_category_option(
                    acp::SessionConfigOptionCategory::ModelConfig,
                    false,
                    window,
                    cx
                ));
            });
            config_options.replace_options(vec![offered.clone()]);
            accepted
                .send(Ok(vec![offered]))
                .expect("authoritative response");
            cx.run_until_parked();
            let expected = if boolean {
                AgentConfigOptionValue::Boolean(false)
            } else {
                AgentConfigOptionValue::ValueId("auto".into())
            };
            assert_eq!(
                agent_server.saved_defaults.lock().as_slice(),
                &[("config".into(), Some(expected))]
            );

            let removed = config_options.defer_next_response();
            view.update_in(cx, |view, window, cx| {
                assert!(view.cycle_category_option(
                    acp::SessionConfigOptionCategory::ModelConfig,
                    false,
                    window,
                    cx
                ));
            });
            config_options.replace_options(Vec::new());
            removed.send(Ok(Vec::new())).expect("option removed");
            cx.run_until_parked();
            assert_eq!(
                agent_server.saved_defaults.lock().len(),
                1,
                "do not save an absent option"
            );
        }
    }

    #[gpui::test]
    fn cycling_preferred_config_option_saves_selected_value_as_default(cx: &mut TestAppContext) {
        cx.update(init_test_ui);
        let agent_server = Rc::new(TestAgentServer::default());
        let config_options = Rc::new(TestSessionConfigOptions::new(vec![
            acp::SessionConfigOption::boolean("extra_mode", "Extra Mode", false)
                .category(acp::SessionConfigOptionCategory::Mode),
            acp::SessionConfigOption::select(
                "mode",
                "Mode",
                "auto",
                vec![
                    acp::SessionConfigSelectOption::new("auto", "Auto"),
                    acp::SessionConfigSelectOption::new("manual", "Manual"),
                ],
            )
            .category(acp::SessionConfigOptionCategory::Mode),
        ]));
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let cx = cx.add_empty_window();

        cx.update(|window, cx| {
            let view = cx.new(|cx| {
                ConfigOptionsView::new(config_options.clone(), agent_server.clone(), fs, window, cx)
            });

            assert!(view.update(cx, |view, cx| {
                view.cycle_category_option(
                    acp::SessionConfigOptionCategory::Mode,
                    false,
                    window,
                    cx,
                )
            }));
        });
        cx.run_until_parked();

        assert_eq!(
            agent_server.saved_defaults.lock().as_slice(),
            &[(
                "mode".to_string(),
                Some(AgentConfigOptionValue::ValueId("manual".to_string()))
            )]
        );
        assert_eq!(
            config_options.set_values.borrow().as_slice(),
            &[(
                "mode".to_string(),
                acp::SessionConfigOptionValue::id("manual")
            )]
        );
    }

    #[gpui::test]
    fn cycling_category_falls_back_to_boolean_if_it_has_no_selector(cx: &mut TestAppContext) {
        let agent_server = Rc::new(TestAgentServer::default());
        let config_options = Rc::new(TestSessionConfigOptions::new(vec![
            acp::SessionConfigOption::boolean("web_search", "Web Search", false)
                .category(acp::SessionConfigOptionCategory::ThoughtLevel),
        ]));
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let cx = cx.add_empty_window();

        cx.update(|window, cx| {
            let view = cx.new(|cx| {
                ConfigOptionsView::new(config_options.clone(), agent_server.clone(), fs, window, cx)
            });

            assert!(view.update(cx, |view, cx| {
                view.cycle_category_option(
                    acp::SessionConfigOptionCategory::ThoughtLevel,
                    false,
                    window,
                    cx,
                )
            }));
        });
        cx.run_until_parked();

        assert_eq!(
            agent_server.saved_defaults.lock().as_slice(),
            &[(
                "web_search".to_string(),
                Some(AgentConfigOptionValue::Boolean(true))
            )]
        );
        assert_eq!(
            config_options.set_values.borrow().as_slice(),
            &[(
                "web_search".to_string(),
                acp::SessionConfigOptionValue::boolean(true)
            )]
        );
    }

    #[gpui::test]
    fn toggling_fast_mode_uses_the_boolean_speed_option(cx: &mut TestAppContext) {
        let agent_server = Rc::new(TestAgentServer::default());
        let config_options = Rc::new(TestSessionConfigOptions::new(vec![
            acp::SessionConfigOption::select(
                "speed",
                "Speed",
                "standard",
                vec![acp::SessionConfigSelectOption::new("standard", "Standard")],
            )
            .category(acp::SessionConfigOptionCategory::Speed),
            acp::SessionConfigOption::boolean("web_search", "Web Search", false)
                .category(acp::SessionConfigOptionCategory::ModelConfig),
            acp::SessionConfigOption::boolean("fast_mode", "Fast Mode", false)
                .category(acp::SessionConfigOptionCategory::Speed),
        ]));
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let cx = cx.add_empty_window();

        cx.update(|window, cx| {
            let view = cx.new(|cx| {
                ConfigOptionsView::new(config_options.clone(), agent_server.clone(), fs, window, cx)
            });
            view.update(cx, |view, cx| view.toggle_fast_mode(window, cx));
        });
        cx.run_until_parked();

        assert_eq!(
            agent_server.saved_defaults.lock().as_slice(),
            &[(
                "fast_mode".to_string(),
                Some(AgentConfigOptionValue::Boolean(true))
            )]
        );
        assert_eq!(
            config_options.set_values.borrow().as_slice(),
            &[(
                "fast_mode".to_string(),
                acp::SessionConfigOptionValue::boolean(true)
            )]
        );
    }

    #[gpui::test]
    fn toggling_category_picker_without_select_config_option_is_unhandled(cx: &mut TestAppContext) {
        let agent_server = Rc::new(TestAgentServer::default());
        let config_options = Rc::new(TestSessionConfigOptions::new(vec![
            acp::SessionConfigOption::boolean("web_search", "Web Search", false)
                .category(acp::SessionConfigOptionCategory::Model),
        ]));
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let cx = cx.add_empty_window();
        let view = cx.update({
            move |window, cx| {
                let config_options: Rc<dyn AgentSessionConfigOptions> = config_options;
                let agent_server: Rc<dyn AgentServer> = agent_server;
                cx.new(|cx| ConfigOptionsView::new(config_options, agent_server, fs, window, cx))
            }
        });

        let handled = cx.update(|window, cx| {
            view.update(cx, |view, cx| {
                view.toggle_category_picker(acp::SessionConfigOptionCategory::Model, window, cx)
            })
        });

        assert!(!handled);
    }

    #[gpui::test]
    fn unknown_config_options_survive_live_selector_updates(cx: &mut TestAppContext) {
        cx.update(init_test_ui);
        let category = acp::SessionConfigOptionCategory::Mode;
        let unknown = acp::SessionConfigOption::new(
            "mode",
            "Mode",
            acp::SessionConfigKind::Other(acp::OtherSessionConfigKind::new(
                "_future",
                agent_client_protocol::schema::IndexMap::from([(
                    "currentValue".to_string(),
                    serde_json::json!({"opaque": [1, true]}),
                )]),
            )),
        )
        .category(category.clone());
        let config_options = Rc::new(TestSessionConfigOptions::new(vec![unknown.clone()]));
        let agent_server = Rc::new(TestAgentServer::default());
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            let config_options: Rc<dyn AgentSessionConfigOptions> = config_options.clone();
            let agent_server: Rc<dyn AgentServer> = agent_server.clone();
            cx.new(|cx| ConfigOptionsView::new(config_options, agent_server, fs, window, cx))
        });
        cx.update(|window, cx| {
            view.update(cx, |view, cx| {
                assert!(view.selectors.is_empty());
                assert!(!view.has_overflow);
                assert!(!view.toggle_category_picker(category.clone(), window, cx));
                assert!(!view.cycle_category_option(category.clone(), false, window, cx));
            });
        });
        assert_eq!(config_options.config_options(), vec![unknown.clone()]);

        config_options.replace_options(vec![
            acp::SessionConfigOption::select(
                "mode",
                "Mode",
                "auto",
                vec![
                    acp::SessionConfigSelectOption::new("auto", "Auto"),
                    acp::SessionConfigSelectOption::new("manual", "Manual"),
                ],
            )
            .category(category.clone()),
        ]);
        cx.run_until_parked();
        let selector = cx.update(|window, cx| {
            view.update(cx, |view, cx| {
                assert_eq!(view.selectors.len(), 1);
                assert!(view.toggle_category_picker(category.clone(), window, cx));
                assert!(view.cycle_category_option(category.clone(), false, window, cx));
                view.selectors.first().expect("select control").clone()
            })
        });
        let picker = cx.update(|_, cx| selector.read(cx).picker.clone().expect("select picker"));

        config_options.replace_options(vec![unknown.clone()]);
        cx.run_until_parked();
        cx.update(|window, cx| {
            view.update(cx, |view, cx| {
                assert!(view.selectors.is_empty());
                assert!(!view.toggle_category_picker(category.clone(), window, cx));
                assert!(!view.cycle_category_option(category.clone(), false, window, cx));
            });
            assert!(!selector.update(cx, |selector, cx| selector.toggle_picker(window, cx)));
            picker.update(cx, |picker, cx| {
                assert!(!picker.delegate.can_select(0, window, cx));
                picker.delegate.confirm(false, window, cx);
            });
        });
        assert_eq!(config_options.config_options(), vec![unknown]);
        assert_eq!(
            config_options.set_values.borrow().as_slice(),
            &[(
                "mode".to_string(),
                acp::SessionConfigOptionValue::id("manual")
            )]
        );
        assert_eq!(
            agent_server.saved_defaults.lock().as_slice(),
            &[(
                "mode".to_string(),
                Some(AgentConfigOptionValue::ValueId("manual".to_string()))
            )]
        );
    }

    #[gpui::test]
    async fn read_only_config_options_do_not_set_values_or_defaults(cx: &mut TestAppContext) {
        let agent_server = Rc::new(TestAgentServer::default());
        let config_options = Rc::new(TestSessionConfigOptions {
            read_only: true,
            ..TestSessionConfigOptions::new(vec![
                acp::SessionConfigOption::select(
                    "model",
                    "Model",
                    "child",
                    vec![acp::SessionConfigSelectOption::new("child", "Child model")],
                )
                .category(acp::SessionConfigOptionCategory::Model),
                acp::SessionConfigOption::boolean("fast", "Fast", true)
                    .category(acp::SessionConfigOptionCategory::Speed),
            ])
        });
        let fs: Arc<dyn Fs> = FakeFs::new(cx.executor());
        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| {
            cx.new(|cx| {
                ConfigOptionsView::new(
                    config_options.clone(),
                    agent_server.clone(),
                    fs.clone(),
                    window,
                    cx,
                )
            })
        });
        cx.update(|window, cx| {
            view.update(cx, |view, cx| {
                assert_eq!(view.selectors.len(), 2);
                assert!(!view.has_overflow);
                assert!(!view.toggle_category_picker(
                    acp::SessionConfigOptionCategory::Model,
                    window,
                    cx,
                ));
                for category in [
                    acp::SessionConfigOptionCategory::Model,
                    acp::SessionConfigOptionCategory::Speed,
                ] {
                    assert!(!view.cycle_category_option(category, false, window, cx));
                }
                view.toggle_fast_mode(window, cx);
            });
        });
        let error = cx
            .update(|window, cx| {
                set_config_option_and_default(
                    config_options.clone(),
                    agent_server.clone(),
                    "fast".into(),
                    acp::SessionConfigOptionValue::boolean(false),
                    fs,
                    window,
                    cx,
                )
            })
            .await
            .expect_err("even a stale picker cannot set a read-only value");
        assert!(error.to_string().contains("read-only"));
        assert!(config_options.set_values.borrow().is_empty());
        assert!(agent_server.saved_defaults.lock().is_empty());

        config_options.replace_options(vec![acp::SessionConfigOption::boolean(
            "fast", "Fast", false,
        )]);
        cx.run_until_parked();
        view.read_with(cx, |view, cx| {
            assert_eq!(view.selectors.len(), 1);
            assert!(matches!(
                view.selectors.first().expect("updated selector").read(cx).current_option()
                    .expect("updated option").kind,
                acp::SessionConfigKind::Boolean(boolean) if !boolean.current_value
            ));
        });
    }

    fn init_test_ui(cx: &mut App) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    }

    fn draw_menu_frames(cx: &mut VisualTestContext) {
        for _ in 0..3 {
            cx.update(|window, cx| {
                window.simulate_next_frame(cx);
                window.draw(cx).clear(cx);
            });
        }
    }

    struct TestConfigComposer {
        options: Entity<ConfigOptionsView>,
    }

    impl Render for TestConfigComposer {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            h_flex()
                .w_full()
                .justify_between()
                .child(h_flex().children(self.options.read(cx).response_controls(cx)))
                .child(self.options.clone())
        }
    }

    #[derive(Default)]
    struct TestAgentServer {
        saved_defaults: Arc<Mutex<Vec<(String, Option<AgentConfigOptionValue>)>>>,
    }

    impl AgentServer for TestAgentServer {
        fn logo(&self) -> IconName {
            IconName::ZedAssistant
        }

        fn agent_id(&self) -> AgentId {
            AgentId::new("test-agent")
        }

        fn connect(
            &self,
            _delegate: agent_servers::AgentServerDelegate,
            _project: Entity<Project>,
            _cx: &mut App,
        ) -> Task<anyhow::Result<Rc<dyn AgentConnection>>> {
            Task::ready(Err(anyhow::anyhow!("test agent server cannot connect")))
        }

        fn into_any(self: Rc<Self>) -> Rc<dyn Any> {
            self
        }

        fn set_default_config_option(
            &self,
            config_id: &str,
            value: Option<AgentConfigOptionValue>,
            _fs: Arc<dyn Fs>,
            _cx: &mut App,
        ) {
            self.saved_defaults
                .lock()
                .push((config_id.to_string(), value));
        }
    }

    struct TestSessionConfigOptions {
        read_only: bool,
        options: RefCell<Vec<acp::SessionConfigOption>>,
        set_values: RefCell<Vec<(String, acp::SessionConfigOptionValue)>>,
        updates: RefCell<watch::Sender<()>>,
        next_response: RefCell<
            Option<
                futures::channel::oneshot::Receiver<anyhow::Result<Vec<acp::SessionConfigOption>>>,
            >,
        >,
    }

    impl TestSessionConfigOptions {
        fn new(options: Vec<acp::SessionConfigOption>) -> Self {
            Self {
                read_only: false,
                options: RefCell::new(options),
                set_values: RefCell::new(Vec::new()),
                updates: RefCell::new(watch::channel(()).0),
                next_response: RefCell::new(None),
            }
        }

        fn defer_next_response(
            &self,
        ) -> futures::channel::oneshot::Sender<anyhow::Result<Vec<acp::SessionConfigOption>>>
        {
            let (sender, receiver) = futures::channel::oneshot::channel();
            assert!(self.next_response.borrow_mut().replace(receiver).is_none());
            sender
        }

        fn replace_options(&self, options: Vec<acp::SessionConfigOption>) {
            *self.options.borrow_mut() = options;
            self.updates.borrow_mut().send(()).expect("config watcher");
        }
    }

    impl AgentSessionConfigOptions for TestSessionConfigOptions {
        fn description_format(&self) -> acp_thread::AgentDescriptionFormat {
            acp_thread::AgentDescriptionFormat::Markdown
        }

        fn config_options(&self) -> Vec<acp::SessionConfigOption> {
            self.options.borrow().clone()
        }

        fn is_read_only(&self) -> bool {
            self.read_only
        }

        fn set_config_option(
            &self,
            config_id: acp::SessionConfigId,
            value: acp::SessionConfigOptionValue,
            cx: &mut App,
        ) -> Task<anyhow::Result<Vec<acp::SessionConfigOption>>> {
            self.set_values
                .borrow_mut()
                .push((config_id.0.to_string(), value.clone()));

            if let Some(response) = self.next_response.borrow_mut().take() {
                return cx.spawn(async move |_| response.await?);
            }
            let options = {
                let mut options = self.options.borrow_mut();
                if let Some(option) = options
                    .iter_mut()
                    .find(|option| option.config_id == config_id)
                {
                    match (&mut option.kind, value) {
                        (
                            acp::SessionConfigKind::Select(select),
                            acp::SessionConfigOptionValue::Id { value },
                        ) => {
                            select.current_value = value;
                        }
                        (
                            acp::SessionConfigKind::Boolean(boolean),
                            acp::SessionConfigOptionValue::Boolean { value },
                        ) => {
                            boolean.current_value = value;
                        }
                        _ => {}
                    }
                }
                options.clone()
            };

            self.updates.borrow_mut().send(()).expect("config watcher");
            Task::ready(Ok(options))
        }

        fn watch(&self, _cx: &mut App) -> Option<watch::Receiver<()>> {
            Some(self.updates.borrow().receiver())
        }
    }
}
