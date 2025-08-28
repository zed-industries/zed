mod appearance_settings_controls;

use std::any::{Any, TypeId};
use std::ops::Range;

use anyhow::Context as _;
use command_palette_hooks::CommandPaletteFilter;
use editor::EditorSettingsControls;
use feature_flags::{FeatureFlag, FeatureFlagViewExt};
use fs::Fs;
use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, ReadGlobal, actions};
use settings::{
    SettingsStore, SettingsUIItemGroup, SettingsUIItemSingle, SettingsUIItemVariant, SettingsValue,
};
use smallvec::SmallVec;
use ui::{SwitchField, prelude::*};
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
    depth: usize,
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
    // todo! rename SettingsUIItemSingle
    render: Option<SettingsUIItemSingle>,
}

struct SettingsUITree {
    user_settings: serde_json::Value,
    default_settings: serde_json::Value,
    root_entry_indices: Vec<usize>,
    tree: Vec<UIEntry>,
    active_entry_index: usize,
}

fn build_tree_item(
    tree: &mut Vec<UIEntry>,
    group: SettingsUIItemVariant,
    depth: usize,
    prev_index: Option<usize>,
) {
    let index = tree.len();
    tree.push(UIEntry {
        title: "",
        path: "",
        depth,
        descendant_range: index + 1..index + 1,
        total_descendant_range: index + 1..index + 1,
        render: None,
        next_sibling: None,
    });
    if let Some(prev_index) = prev_index {
        tree[prev_index].next_sibling = Some(index);
    }
    match group {
        SettingsUIItemVariant::Group { path, title, group } => {
            tree[index].path = path;
            tree[index].title = title;
            for group_item in group.items {
                let prev_index = Some(tree[index].descendant_range.end - 1)
                    .filter(|_| !tree[index].descendant_range.is_empty());
                tree[index].descendant_range.end = tree.len() + 1;
                build_tree_item(tree, group_item.item, depth + 1, prev_index);
                tree[index].total_descendant_range.end = tree.len();
            }
        }
        SettingsUIItemVariant::Item { path, item } => {
            tree[index].path = path;
            tree[index].title = path; // todo! title
            tree[index].render = Some(item);
        }
        SettingsUIItemVariant::None => {
            return;
        }
    }
}

impl SettingsUITree {
    fn new(cx: &App) -> Self {
        let settings_store = SettingsStore::global(cx);
        // todo! remove clones somehow?
        let user_settings = settings_store.raw_user_settings.clone();
        let default_settings = settings_store.raw_default_settings.clone();
        let mut tree = vec![];
        let mut root_entry_indices = vec![];
        for item in settings_store.settings_ui_items() {
            assert!(
                matches!(
                    item.item,
                    SettingsUIItemVariant::Group { .. } | SettingsUIItemVariant::None
                ),
                "top level items must be groups"
            );
            let prev_root_entry_index = root_entry_indices.last().copied();
            root_entry_indices.push(tree.len());
            build_tree_item(&mut tree, item.item, 0, prev_root_entry_index);
        }

        Self {
            tree,
            root_entry_indices,
            user_settings,
            default_settings,
            active_entry_index: 0,
        }
    }

    fn render_nav(&self, _window: &mut Window, cx: &mut Context<SettingsPage>) -> impl IntoElement {
        let mut nav = v_flex()
            .items_start()
            .p_4()
            .bg(cx.theme().colors().background)
            .size_full()
            .gap_4();
        for index in &self.root_entry_indices {
            nav = nav.child(
                Label::new(SharedString::new_static(self.tree[*index].title))
                    .size(LabelSize::Large)
                    .when(self.active_entry_index == *index, |this| {
                        this.color(Color::Selected)
                    }),
            );
        }
        nav
    }

    fn render_content(
        &self,
        _window: &mut Window,
        _cx: &mut Context<SettingsPage>,
    ) -> impl IntoElement {
        let Some(entry) = self.tree.get(self.active_entry_index) else {
            return div().size_full().child(
                Label::new(SharedString::new_static("No settings found")).color(Color::Error),
            );
        };
        return div()
            .size_full()
            .child(Label::new(SharedString::new_static("Content goes here")));
    }
}

impl Render for SettingsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings_tree = SettingsUITree::new(cx);

        h_flex()
            .p_4()
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .gap_4()
            .child(
                div()
                    .w_1_4()
                    .h_full()
                    .child(settings_tree.render_nav(window, cx)),
            )
            .child(
                div()
                    .w_3_4()
                    .h_full()
                    .child(settings_tree.render_content(window, cx)),
            )
    }
}

// TODO: remove, only here as inspiration
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
