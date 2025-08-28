mod appearance_settings_controls;

use std::any::TypeId;

use anyhow::Context as _;
use command_palette_hooks::CommandPaletteFilter;
use editor::EditorSettingsControls;
use feature_flags::{FeatureFlag, FeatureFlagViewExt};
use fs::Fs;
use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, ReadGlobal, actions};
use settings::{SettingsStore, SettingsUIItemSingle, SettingsValue};
use smallvec::SmallVec;
use ui::{NumericStepper, SwitchField, prelude::*};
use workspace::item::{Item, ItemEvent};
use workspace::{Workspace, with_active_or_new_workspace};

use crate::appearance_settings_controls::AppearanceSettingsControls;

pub struct SettingsUiFeatureFlag;

impl FeatureFlag for SettingsUiFeatureFlag {
    const NAME: &'static str = "settings-ui";
}

actions!(
    zed,
    [
        /// Opens the settings editor.
        OpenSettingsEditor
    ]
);

pub fn init(cx: &mut App) {
    cx.on_action(|_: &OpenSettingsEditor, cx| {
        with_active_or_new_workspace(cx, move |workspace, window, cx| {
            let existing = workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<SettingsPage>());

            if let Some(existing) = existing {
                workspace.activate_item(&existing, true, true, window, cx);
            } else {
                let settings_page = SettingsPage::new(workspace, cx);
                workspace.add_item_to_active_pane(Box::new(settings_page), None, true, window, cx)
            }
        });
    });

    cx.observe_new(|_workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let settings_ui_actions = [TypeId::of::<OpenSettingsEditor>()];

        CommandPaletteFilter::update_global(cx, |filter, _cx| {
            filter.hide_action_types(&settings_ui_actions);
        });

        cx.observe_flag::<SettingsUiFeatureFlag, _>(
            window,
            move |is_enabled, _workspace, _, cx| {
                if is_enabled {
                    CommandPaletteFilter::update_global(cx, |filter, _cx| {
                        filter.show_action_types(settings_ui_actions.iter());
                    });
                } else {
                    CommandPaletteFilter::update_global(cx, |filter, _cx| {
                        filter.hide_action_types(&settings_ui_actions);
                    });
                }
            },
        )
        .detach();
    })
    .detach();
}

pub struct SettingsPage {
    focus_handle: FocusHandle,
}

impl SettingsPage {
    pub fn new(_workspace: &Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }
}

impl EventEmitter<ItemEvent> for SettingsPage {}

impl Focusable for SettingsPage {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for SettingsPage {
    type Event = ItemEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Settings))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Settings".into()
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(ItemEvent)) {
        f(*event)
    }
}

fn element_id_from_path(path: &[&'static str]) -> ElementId {
    if path.len() == 0 {
        panic!("Path length must not be zero");
    } else if path.len() == 1 {
        ElementId::Name(SharedString::new_static(path[0]))
    } else {
        ElementId::from((
            ElementId::from(SharedString::new_static(path[path.len() - 2])),
            SharedString::new_static(path[path.len() - 1]),
        ))
    }
}

fn render_item_single(
    settings_value: SettingsValue<serde_json::Value>,
    item: &SettingsUIItemSingle,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    match item {
        SettingsUIItemSingle::Custom(_) => div()
            .child(format!("Item: {}", settings_value.path.join(".")))
            .into_any_element(),
        SettingsUIItemSingle::SwitchField => {
            render_any_item(settings_value, render_switch_field, window, cx)
        }
        SettingsUIItemSingle::NumericStepper => {
            render_any_item(settings_value, render_numeric_stepper, window, cx)
        }
        SettingsUIItemSingle::ToggleGroup => {
            todo!()
        }
    }
}

fn read_settings_value_from_path<'a>(
    settings_contents: &'a serde_json::Value,
    path: &[&'static str],
) -> Option<&'a serde_json::Value> {
    let Some((key, remaining)) = path.split_first() else {
        return Some(settings_contents);
    };
    let Some(value) = settings_contents.get(key) else {
        // let error = format!("Key not found: {}", key);
        // dbg!(error);
        return None;
    };

    read_settings_value_from_path(value, remaining)
}

fn render_any_item<T: serde::de::DeserializeOwned>(
    settings_value: SettingsValue<serde_json::Value>,
    render_fn: impl Fn(SettingsValue<T>, &mut Window, &mut App) -> AnyElement + 'static,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let value = settings_value
        .value
        .map(|value| serde_json::from_value::<T>(value).expect("value is not a T"));
    // todo! We have to make sure default.json has all default setting values now
    let default_value = serde_json::from_value::<T>(settings_value.default_value)
        .expect("default value is not an Option<T>");
    let deserialized_setting_value = SettingsValue {
        title: settings_value.title,
        path: settings_value.path,
        value,
        default_value,
    };
    render_fn(deserialized_setting_value, window, cx)
}

fn render_numeric_stepper(
    value: SettingsValue<u64>,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    let id = element_id_from_path(&value.path);
    let path = value.path.clone();
    let num = value.value.unwrap_or_else(|| value.default_value);

    NumericStepper::new(
        id,
        num.to_string(),
        {
            let path = value.path.clone();
            move |_, _, cx| {
                let Some(number) = serde_json::Number::from_u128(num.saturating_sub(1) as u128)
                else {
                    return;
                };
                let new_value = serde_json::Value::Number(number);

                let settings_store = SettingsStore::global(cx);
                let fs = <dyn Fs>::global(cx);

                let rx = settings_store.update_settings_file_at_path(
                    fs.clone(),
                    &path.as_slice(),
                    new_value,
                );
                cx.background_spawn(async move { rx.await?.context("Failed to update settings") })
                    .detach_and_log_err(cx);
            }
        },
        move |_, _, cx| {
            let Some(number) = serde_json::Number::from_u128(num.saturating_add(1) as u128) else {
                return;
            };

            let new_value = serde_json::Value::Number(number);

            let settings_store = SettingsStore::global(cx);
            let fs = <dyn Fs>::global(cx);

            let rx = settings_store.update_settings_file_at_path(
                fs.clone(),
                &path.as_slice(),
                new_value,
            );
            cx.background_spawn(async move { rx.await?.context("Failed to update settings") })
                .detach_and_log_err(cx);
        },
    )
    .style(ui::NumericStepperStyle::Outlined)
    .into_any_element()
}

fn render_switch_field(
    value: SettingsValue<bool>,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    let id = element_id_from_path(&value.path);
    let path = value.path.clone();
    SwitchField::new(
        id,
        SharedString::new_static(value.title),
        None,
        match value.read() {
            true => ToggleState::Selected,
            false => ToggleState::Unselected,
        },
        move |toggle_state, _, cx| {
            let new_value = serde_json::Value::Bool(match toggle_state {
                ToggleState::Indeterminate => {
                    return;
                }
                ToggleState::Selected => true,
                ToggleState::Unselected => false,
            });

            let settings_store = SettingsStore::global(cx);
            let fs = <dyn Fs>::global(cx);

            let rx = settings_store.update_settings_file_at_path(
                fs.clone(),
                &path.as_slice(),
                new_value,
            );
            cx.background_spawn(async move { rx.await?.context("Failed to update settings") })
                .detach_and_log_err(cx);
        },
    )
    .into_any_element()
}

fn settings_value_from_settings_and_path(
    path: SmallVec<[&'static str; 1]>,
    user_settings: &serde_json::Value,
    default_settings: &serde_json::Value,
) -> SettingsValue<serde_json::Value> {
    let default_value = read_settings_value_from_path(default_settings, &path)
        .with_context(|| format!("No default value for item at path {:?}", path.join(".")))
        .expect("Default value set for item")
        .clone();

    let value = read_settings_value_from_path(user_settings, &path).cloned();
    let settings_value = SettingsValue {
        default_value,
        value,
        path: path.clone(),
        title: path.last().expect("todo! pass path"),
    };
    return settings_value;
}

impl Render for SettingsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let user_settings;
        let default_settings;
        let items;
        // todo! this feels like it wants to be separated into 2 layers:
        // 1. Load settings, and ui_items tree. Construct intermediate representation of tree, that is more uniform, and possibly more caching-friendly
        // 2. Render from the intermediate representation
        // With this structure:
        // - changing how the tree is rendered completely should be easier (we don't know the final design yet)
        // - caching of IR is possible
        // IR can hold (and possibly cache):
        // - The settings values (i.e. don't store serde_json::Value and deserialize per frame, just downcast_ref)
        // - The structure of the tree (for panel)
        {
            let settings_store = SettingsStore::global(cx);
            // todo! remove clones somehow?
            user_settings = settings_store.raw_user_settings.clone();
            default_settings = settings_store.raw_default_settings.clone();
            items = settings_store
                .settings_ui_items()
                .into_iter()
                .collect::<Vec<_>>();
        }

        v_flex()
            .p_4()
            .size_full()
            .gap_4()
            .children(items.into_iter().flat_map(|item| {
                match item.item {
                    settings::SettingsUIItemVariant::Group {
                        title,
                        path: group_path,
                        group,
                    } => Some(
                        div()
                            .child(Label::new(title).size(LabelSize::Large))
                            .children(group.items.iter().map(|item| {
                                match &item.item {
                                    settings::SettingsUIItemVariant::Group {
                                        path,
                                        title,
                                        group,
                                    } => div()
                                        .child(format!("Subgroup: {}", title))
                                        .into_any_element(),
                                    settings::SettingsUIItemVariant::Item {
                                        path: item_path,
                                        item,
                                    } => {
                                        let path = smallvec::smallvec![group_path, *item_path];
                                        let settings_value = settings_value_from_settings_and_path(
                                            path,
                                            &user_settings,
                                            &default_settings,
                                        );
                                        render_item_single(settings_value, item, window, cx)
                                    }
                                    settings::SettingsUIItemVariant::None => {
                                        div().child("None").into_any_element()
                                    }
                                }
                            })),
                    ),

                    settings::SettingsUIItemVariant::Item { path, item } => todo!(),
                    settings::SettingsUIItemVariant::None => None,
                }
            }))
            .child(Label::new("Settings").size(LabelSize::Large))
            .child(
                v_flex().gap_1().child(Label::new("Appearance")).child(
                    v_flex()
                        .elevation_2(cx)
                        .child(AppearanceSettingsControls::new()),
                ),
            )
            .child(
                v_flex().gap_1().child(Label::new("Editor")).child(
                    v_flex()
                        .elevation_2(cx)
                        .child(EditorSettingsControls::new()),
                ),
            )
    }
}
