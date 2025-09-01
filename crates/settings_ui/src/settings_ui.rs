mod appearance_settings_controls;

use std::any::TypeId;
use std::ops::{Not, Range};

use anyhow::Context as _;
use command_palette_hooks::CommandPaletteFilter;
use editor::EditorSettingsControls;
use feature_flags::{FeatureFlag, FeatureFlagViewExt};
use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, ReadGlobal, actions};
use settings::{SettingsStore, SettingsUiEntryVariant, SettingsUiItemSingle, SettingsValue};
use smallvec::SmallVec;
use ui::{NumericStepper, SwitchField, ToggleButtonGroup, ToggleButtonSimple, prelude::*};
use workspace::{
    Workspace,
    item::{Item, ItemEvent},
    with_active_or_new_workspace,
};

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
    settings_tree: SettingsUiTree,
}

impl SettingsPage {
    pub fn new(_workspace: &Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            settings_tree: SettingsUiTree::new(cx),
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

// We want to iterate over the side bar with root groups
// - this is a loop over top level groups, and if any are expanded, recursively displaying their items
// - Should be able to get all items from a group (flatten a group)
// - Should be able to toggle/untoggle groups in UI (at least in sidebar)
// - Search should be available
//  - there should be an index of text -> item mappings, for using fuzzy::match
//   - Do we want to show the parent groups when a item is matched?

struct UIEntry {
    title: &'static str,
    path: &'static str,
    _depth: usize,
    // a
    //  b     < a descendant range < a total descendant range
    //    f   |                    |
    //    g   |                    |
    //  c     <                    |
    //    d                        |
    //    e                        <
    descendant_range: Range<usize>,
    total_descendant_range: Range<usize>,
    next_sibling: Option<usize>,
    // expanded: bool,
    render: Option<SettingsUiItemSingle>,
}

struct SettingsUiTree {
    root_entry_indices: Vec<usize>,
    entries: Vec<UIEntry>,
    active_entry_index: usize,
}

fn build_tree_item(
    tree: &mut Vec<UIEntry>,
    group: SettingsUiEntryVariant,
    depth: usize,
    prev_index: Option<usize>,
) {
    let index = tree.len();
    tree.push(UIEntry {
        title: "",
        path: "",
        _depth: depth,
        descendant_range: index + 1..index + 1,
        total_descendant_range: index + 1..index + 1,
        render: None,
        next_sibling: None,
    });
    if let Some(prev_index) = prev_index {
        tree[prev_index].next_sibling = Some(index);
    }
    match group {
        SettingsUiEntryVariant::Group {
            path,
            title,
            items: group_items,
        } => {
            tree[index].path = path;
            tree[index].title = title;
            for group_item in group_items {
                let prev_index = tree[index]
                    .descendant_range
                    .is_empty()
                    .not()
                    .then_some(tree[index].descendant_range.end - 1);
                tree[index].descendant_range.end = tree.len() + 1;
                build_tree_item(tree, group_item.item, depth + 1, prev_index);
                tree[index].total_descendant_range.end = tree.len();
            }
        }
        SettingsUiEntryVariant::Item { path, item } => {
            tree[index].path = path;
            // todo(settings_ui) create title from path in macro, and use here
            tree[index].title = path;
            tree[index].render = Some(item);
        }
        SettingsUiEntryVariant::None => {
            return;
        }
    }
}

impl SettingsUiTree {
    fn new(cx: &App) -> Self {
        let settings_store = SettingsStore::global(cx);
        let mut tree = vec![];
        let mut root_entry_indices = vec![];
        for item in settings_store.settings_ui_items() {
            if matches!(item.item, SettingsUiEntryVariant::None) {
                continue;
            }

            assert!(
                matches!(item.item, SettingsUiEntryVariant::Group { .. }),
                "top level items must be groups: {:?}",
                match item.item {
                    SettingsUiEntryVariant::Item { path, .. } => path,
                    _ => unreachable!(),
                }
            );
            let prev_root_entry_index = root_entry_indices.last().copied();
            root_entry_indices.push(tree.len());
            build_tree_item(&mut tree, item.item, 0, prev_root_entry_index);
        }

        root_entry_indices.sort_by_key(|i| tree[*i].title);

        let active_entry_index = root_entry_indices[0];
        Self {
            entries: tree,
            root_entry_indices,
            active_entry_index,
        }
    }
}

fn render_nav(tree: &SettingsUiTree, _window: &mut Window, cx: &mut Context<SettingsPage>) -> Div {
    let mut nav = v_flex().p_4().gap_2();
    for &index in &tree.root_entry_indices {
        nav = nav.child(
            div()
                .id(index)
                .on_click(cx.listener(move |settings, _, _, _| {
                    settings.settings_tree.active_entry_index = index;
                }))
                .child(
                    Label::new(SharedString::new_static(tree.entries[index].title))
                        .size(LabelSize::Large)
                        .when(tree.active_entry_index == index, |this| {
                            this.color(Color::Selected)
                        }),
                ),
        );
    }
    nav
}

fn render_content(
    tree: &SettingsUiTree,
    window: &mut Window,
    cx: &mut Context<SettingsPage>,
) -> impl IntoElement {
    let Some(entry) = tree.entries.get(tree.active_entry_index) else {
        return div()
            .size_full()
            .child(Label::new(SharedString::new_static("No settings found")).color(Color::Error));
    };
    let mut content = v_flex().size_full().gap_4();

    let mut child_index = entry
        .descendant_range
        .is_empty()
        .not()
        .then_some(entry.descendant_range.start);
    let mut path = smallvec::smallvec![entry.path];

    while let Some(index) = child_index {
        let child = &tree.entries[index];
        child_index = child.next_sibling;
        if child.render.is_none() {
            // todo(settings_ui): subgroups?
            continue;
        }
        path.push(child.path);
        let settings_value = settings_value_from_settings_and_path(
            path.clone(),
            // PERF: how to structure this better? There feels like there's a way to avoid the clone
            // and every value lookup
            SettingsStore::global(cx).raw_user_settings(),
            SettingsStore::global(cx).raw_default_settings(),
        );
        content = content.child(
            div()
                .child(
                    Label::new(SharedString::new_static(tree.entries[index].title))
                        .size(LabelSize::Large)
                        .when(tree.active_entry_index == index, |this| {
                            this.color(Color::Selected)
                        }),
                )
                .child(render_item_single(
                    settings_value,
                    child.render.as_ref().unwrap(),
                    window,
                    cx,
                )),
        );

        path.pop();
    }

    return content;
}

impl Render for SettingsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .grid()
            .grid_cols(16)
            .p_4()
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .child(
                div()
                    .col_span(2)
                    .h_full()
                    .child(render_nav(&self.settings_tree, window, cx)),
            )
            .child(div().col_span(4).h_full().child(render_content(
                &self.settings_tree,
                window,
                cx,
            )))
    }
}

// todo(settings_ui): remove, only here as inspiration
#[allow(dead_code)]
fn render_old_appearance_settings(cx: &mut App) -> impl IntoElement {
    v_flex()
        .p_4()
        .size_full()
        .gap_4()
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
    item: &SettingsUiItemSingle,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    match item {
        SettingsUiItemSingle::Custom(_) => div()
            .child(format!("Item: {}", settings_value.path.join(".")))
            .into_any_element(),
        SettingsUiItemSingle::SwitchField => {
            render_any_item(settings_value, render_switch_field, window, cx)
        }
        SettingsUiItemSingle::NumericStepper => {
            render_any_item(settings_value, render_numeric_stepper, window, cx)
        }
        SettingsUiItemSingle::ToggleGroup(variants) => {
            render_toggle_button_group(settings_value, variants, window, cx)
        }
        SettingsUiItemSingle::DropDown(_) => {
            unimplemented!("This")
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
        return None;
    };

    read_settings_value_from_path(value, remaining)
}

fn downcast_any_item<T: serde::de::DeserializeOwned>(
    settings_value: SettingsValue<serde_json::Value>,
) -> SettingsValue<T> {
    let value = settings_value
        .value
        .map(|value| serde_json::from_value::<T>(value).expect("value is not a T"));
    // todo(settings_ui) Create test that constructs UI tree, and asserts that all elements have default values
    let default_value = serde_json::from_value::<T>(settings_value.default_value)
        .expect("default value is not an Option<T>");
    let deserialized_setting_value = SettingsValue {
        title: settings_value.title,
        path: settings_value.path,
        value,
        default_value,
    };
    deserialized_setting_value
}

fn render_any_item<T: serde::de::DeserializeOwned>(
    settings_value: SettingsValue<serde_json::Value>,
    render_fn: impl Fn(SettingsValue<T>, &mut Window, &mut App) -> AnyElement + 'static,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let deserialized_setting_value = downcast_any_item(settings_value);
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
                SettingsValue::write_value(&path, new_value, cx);
            }
        },
        move |_, _, cx| {
            let Some(number) = serde_json::Number::from_u128(num.saturating_add(1) as u128) else {
                return;
            };

            let new_value = serde_json::Value::Number(number);

            SettingsValue::write_value(&path, new_value, cx);
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

            SettingsValue::write_value(&path, new_value, cx);
        },
    )
    .into_any_element()
}

fn render_toggle_button_group(
    value: SettingsValue<serde_json::Value>,
    variants: &'static [&'static str],
    _: &mut Window,
    _: &mut App,
) -> AnyElement {
    let value = downcast_any_item::<String>(value);

    fn make_toggle_group<const LEN: usize>(
        group_name: &'static str,
        value: SettingsValue<String>,
        variants: &'static [&'static str],
    ) -> AnyElement {
        let mut variants_array: [&'static str; LEN] = ["default"; LEN];
        variants_array.copy_from_slice(variants);
        let active_value = value.read();

        let selected_idx = variants_array
            .iter()
            .enumerate()
            .find_map(|(idx, variant)| {
                if variant == &active_value {
                    Some(idx)
                } else {
                    None
                }
            });

        ToggleButtonGroup::single_row(
            group_name,
            variants_array.map(|variant| {
                let path = value.path.clone();
                ToggleButtonSimple::new(variant, move |_, _, cx| {
                    SettingsValue::write_value(
                        &path,
                        serde_json::Value::String(variant.to_string()),
                        cx,
                    );
                })
            }),
        )
        .when_some(selected_idx, |this, ix| this.selected_index(ix))
        .style(ui::ToggleButtonGroupStyle::Filled)
        .into_any_element()
    }

    macro_rules! templ_toggl_with_const_param {
        ($len:expr) => {
            if variants.len() == $len {
                return make_toggle_group::<$len>(value.title, value, variants);
            }
        };
    }
    templ_toggl_with_const_param!(1);
    templ_toggl_with_const_param!(2);
    templ_toggl_with_const_param!(3);
    templ_toggl_with_const_param!(4);
    templ_toggl_with_const_param!(5);
    templ_toggl_with_const_param!(6);
    unreachable!("Too many variants");
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
        // todo(settings_ui) title for items
        title: path.last().expect("path non empty"),
    };
    return settings_value;
}
