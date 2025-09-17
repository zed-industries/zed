mod appearance_settings_controls;

use std::{
    num::NonZeroU32,
    ops::{Not, Range},
    rc::Rc,
};

use anyhow::Context as _;
use editor::{Editor, EditorSettingsControls};
use feature_flags::{FeatureFlag, FeatureFlagAppExt};
use gpui::{App, Entity, EventEmitter, FocusHandle, Focusable, ReadGlobal, ScrollHandle, actions};
use settings::{
    NumType, SettingsStore, SettingsUiEntry, SettingsUiEntryMetaData, SettingsUiItem,
    SettingsUiItemDynamicMap, SettingsUiItemGroup, SettingsUiItemSingle, SettingsUiItemUnion,
    SettingsValue,
};
use smallvec::SmallVec;
use ui::{
    ContextMenu, DropdownMenu, NumericStepper, SwitchField, ToggleButtonGroup, ToggleButtonSimple,
    prelude::*,
};
use workspace::{
    Workspace,
    item::{Item, ItemEvent},
};

use crate::appearance_settings_controls::AppearanceSettingsControls;

pub struct SettingsUiFeatureFlag;

impl FeatureFlag for SettingsUiFeatureFlag {
    const NAME: &'static str = "settings-ui";
}

actions!(
    zed,
    [
        /// Opens settings UI.
        OpenSettingsUi
    ]
);

pub fn open_settings_editor(
    workspace: &mut Workspace,
    _: &OpenSettingsUi,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    // todo(settings_ui) open in a local workspace if this is remote.
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
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action_renderer(|div, _, _, cx| {
            let settings_ui_actions = [std::any::TypeId::of::<OpenSettingsUi>()];
            let has_flag = cx.has_flag::<SettingsUiFeatureFlag>();
            command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _| {
                if has_flag {
                    filter.show_action_types(&settings_ui_actions);
                } else {
                    filter.hide_action_types(&settings_ui_actions);
                }
            });
            if has_flag {
                div.on_action(cx.listener(open_settings_editor))
            } else {
                div
            }
        });
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

struct UiEntry {
    title: SharedString,
    path: Option<SharedString>,
    documentation: Option<SharedString>,
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
    dynamic_render: Option<SettingsUiItemUnion>,
    generate_items: Option<(
        SettingsUiItem,
        fn(&serde_json::Value, &App) -> Vec<SettingsUiEntryMetaData>,
        SmallVec<[SharedString; 1]>,
    )>,
}

impl UiEntry {
    fn first_descendant_index(&self) -> Option<usize> {
        return self
            .descendant_range
            .is_empty()
            .not()
            .then_some(self.descendant_range.start);
    }

    fn nth_descendant_index(&self, tree: &[UiEntry], n: usize) -> Option<usize> {
        let first_descendant_index = self.first_descendant_index()?;
        let mut current_index = 0;
        let mut current_descendant_index = Some(first_descendant_index);
        while let Some(descendant_index) = current_descendant_index
            && current_index < n
        {
            current_index += 1;
            current_descendant_index = tree[descendant_index].next_sibling;
        }
        current_descendant_index
    }
}

pub struct SettingsUiTree {
    root_entry_indices: Vec<usize>,
    entries: Vec<UiEntry>,
    active_entry_index: usize,
}

fn build_tree_item(
    tree: &mut Vec<UiEntry>,
    entry: SettingsUiEntry,
    depth: usize,
    prev_index: Option<usize>,
) {
    // let tree: HashMap<Path, UiEntry>;
    let index = tree.len();
    tree.push(UiEntry {
        title: entry.title.into(),
        path: entry.path.map(SharedString::new_static),
        documentation: entry.documentation.map(SharedString::new_static),
        _depth: depth,
        descendant_range: index + 1..index + 1,
        total_descendant_range: index + 1..index + 1,
        render: None,
        next_sibling: None,
        dynamic_render: None,
        generate_items: None,
    });
    if let Some(prev_index) = prev_index {
        tree[prev_index].next_sibling = Some(index);
    }
    match entry.item {
        SettingsUiItem::Group(SettingsUiItemGroup { items: group_items }) => {
            for group_item in group_items {
                let prev_index = tree[index]
                    .descendant_range
                    .is_empty()
                    .not()
                    .then_some(tree[index].descendant_range.end - 1);
                tree[index].descendant_range.end = tree.len() + 1;
                build_tree_item(tree, group_item, depth + 1, prev_index);
                tree[index].total_descendant_range.end = tree.len();
            }
        }
        SettingsUiItem::Single(item) => {
            tree[index].render = Some(item);
        }
        SettingsUiItem::Union(dynamic_render) => {
            // todo(settings_ui) take from item and store other fields instead of clone
            // will also require replacing usage in render_recursive so it can know
            // which options were actually rendered
            let options = dynamic_render.options.clone();
            tree[index].dynamic_render = Some(dynamic_render);
            for option in options {
                let Some(option) = option else { continue };
                let prev_index = tree[index]
                    .descendant_range
                    .is_empty()
                    .not()
                    .then_some(tree[index].descendant_range.end - 1);
                tree[index].descendant_range.end = tree.len() + 1;
                build_tree_item(tree, option, depth + 1, prev_index);
                tree[index].total_descendant_range.end = tree.len();
            }
        }
        SettingsUiItem::DynamicMap(SettingsUiItemDynamicMap {
            item: generate_settings_ui_item,
            determine_items,
            defaults_path,
        }) => {
            tree[index].generate_items = Some((
                generate_settings_ui_item(),
                determine_items,
                defaults_path
                    .into_iter()
                    .copied()
                    .map(SharedString::new_static)
                    .collect(),
            ));
        }
        SettingsUiItem::None => {
            return;
        }
    }
}

impl SettingsUiTree {
    pub fn new(cx: &App) -> Self {
        let settings_store = SettingsStore::global(cx);
        let mut tree = vec![];
        let mut root_entry_indices = vec![];
        for item in settings_store.settings_ui_items() {
            if matches!(item.item, SettingsUiItem::None)
            // todo(settings_ui): How to handle top level single items? BaseKeymap is in this category. Probably need a way to
            // link them to other groups
            || matches!(item.item, SettingsUiItem::Single(_))
            {
                continue;
            }

            let prev_root_entry_index = root_entry_indices.last().copied();
            root_entry_indices.push(tree.len());
            build_tree_item(&mut tree, item, 0, prev_root_entry_index);
        }

        root_entry_indices.sort_by_key(|i| &tree[*i].title);

        let active_entry_index = root_entry_indices[0];
        Self {
            entries: tree,
            root_entry_indices,
            active_entry_index,
        }
    }

    // todo(settings_ui): Make sure `Item::None` paths are added to the paths tree,
    // so that we can keep none/skip and still test in CI that all settings have
    #[cfg(feature = "test-support")]
    pub fn all_paths(&self, cx: &App) -> Vec<Vec<SharedString>> {
        fn all_paths_rec(
            tree: &[UiEntry],
            paths: &mut Vec<Vec<SharedString>>,
            current_path: &mut Vec<SharedString>,
            idx: usize,
            cx: &App,
        ) {
            let child = &tree[idx];
            let mut pushed_path = false;
            if let Some(path) = child.path.as_ref() {
                current_path.push(path.clone());
                paths.push(current_path.clone());
                pushed_path = true;
            }
            // todo(settings_ui): handle dynamic nodes here
            let selected_descendant_index = child
                .dynamic_render
                .as_ref()
                .map(|dynamic_render| {
                    read_settings_value_from_path(
                        SettingsStore::global(cx).raw_default_settings(),
                        &current_path,
                    )
                    .map(|value| (dynamic_render.determine_option)(value, cx))
                })
                .and_then(|selected_descendant_index| {
                    selected_descendant_index.map(|index| child.nth_descendant_index(tree, index))
                });

            if let Some(selected_descendant_index) = selected_descendant_index {
                // just silently fail if we didn't find a setting value for the path
                if let Some(descendant_index) = selected_descendant_index {
                    all_paths_rec(tree, paths, current_path, descendant_index, cx);
                }
            } else if let Some(desc_idx) = child.first_descendant_index() {
                let mut desc_idx = Some(desc_idx);
                while let Some(descendant_index) = desc_idx {
                    all_paths_rec(&tree, paths, current_path, descendant_index, cx);
                    desc_idx = tree[descendant_index].next_sibling;
                }
            }
            if pushed_path {
                current_path.pop();
            }
        }

        let mut paths = Vec::new();
        for &index in &self.root_entry_indices {
            all_paths_rec(&self.entries, &mut paths, &mut Vec::new(), index, cx);
        }
        paths
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
                    Label::new(tree.entries[index].title.clone())
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
) -> Div {
    let content = v_flex().size_full().gap_4();

    let mut path = smallvec::smallvec![];

    return render_recursive(
        &tree.entries,
        tree.active_entry_index,
        &mut path,
        content,
        &mut None,
        true,
        window,
        cx,
    );
}

fn render_recursive(
    tree: &[UiEntry],
    index: usize,
    path: &mut SmallVec<[SharedString; 1]>,
    mut element: Div,
    fallback_path: &mut Option<SmallVec<[SharedString; 1]>>,
    render_next_title: bool,
    window: &mut Window,
    cx: &mut App,
) -> Div {
    let Some(child) = tree.get(index) else {
        return element
            .child(Label::new(SharedString::new_static("No settings found")).color(Color::Error));
    };

    if render_next_title {
        element = element.child(Label::new(child.title.clone()).size(LabelSize::Large));
    }

    // todo(settings_ui): subgroups?
    let mut pushed_path = false;
    if let Some(child_path) = child.path.as_ref() {
        path.push(child_path.clone());
        if let Some(fallback_path) = fallback_path.as_mut() {
            fallback_path.push(child_path.clone());
        }
        pushed_path = true;
    }
    let settings_value = settings_value_from_settings_and_path(
        path.clone(),
        fallback_path.as_ref().map(|path| path.as_slice()),
        child.title.clone(),
        child.documentation.clone(),
        // PERF: how to structure this better? There feels like there's a way to avoid the clone
        // and every value lookup
        SettingsStore::global(cx).raw_user_settings(),
        SettingsStore::global(cx).raw_default_settings(),
    );
    if let Some(dynamic_render) = child.dynamic_render.as_ref() {
        let value = settings_value.read();
        let selected_index = (dynamic_render.determine_option)(value, cx);
        element = element.child(div().child(render_toggle_button_group_inner(
            settings_value.title.clone(),
            dynamic_render.labels,
            Some(selected_index),
            {
                let path = settings_value.path.clone();
                let defaults = dynamic_render.defaults.clone();
                move |idx, cx| {
                    if idx == selected_index {
                        return;
                    }
                    let default = defaults.get(idx).cloned().unwrap_or_default();
                    SettingsValue::write_value(&path, default, cx);
                }
            },
        )));
        // we don't add descendants for unit options, so we adjust the selected index
        // by the number of options we didn't add descendants for, to get the descendant index
        let selected_descendant_index = selected_index
            - dynamic_render.options[..selected_index]
                .iter()
                .filter(|option| option.is_none())
                .count();
        if dynamic_render.options[selected_index].is_some()
            && let Some(descendant_index) =
                child.nth_descendant_index(tree, selected_descendant_index)
        {
            element = render_recursive(
                tree,
                descendant_index,
                path,
                element,
                fallback_path,
                false,
                window,
                cx,
            );
        }
    } else if let Some((settings_ui_item, generate_items, defaults_path)) =
        child.generate_items.as_ref()
    {
        let generated_items = generate_items(settings_value.read(), cx);
        let mut ui_items = Vec::with_capacity(generated_items.len());
        for item in generated_items {
            let settings_ui_entry = SettingsUiEntry {
                path: None,
                title: "",
                documentation: None,
                item: settings_ui_item.clone(),
            };
            let prev_index = if ui_items.is_empty() {
                None
            } else {
                Some(ui_items.len() - 1)
            };
            let item_index = ui_items.len();
            build_tree_item(
                &mut ui_items,
                settings_ui_entry,
                child._depth + 1,
                prev_index,
            );
            if item_index < ui_items.len() {
                ui_items[item_index].path = None;
                ui_items[item_index].title = item.title.clone();
                ui_items[item_index].documentation = item.documentation.clone();

                // push path instead of setting path on ui item so that the path isn't pushed to default_path as well
                // when we recurse
                path.push(item.path.clone());
                element = render_recursive(
                    &ui_items,
                    item_index,
                    path,
                    element,
                    &mut Some(defaults_path.clone()),
                    true,
                    window,
                    cx,
                );
                path.pop();
            }
        }
    } else if let Some(child_render) = child.render.as_ref() {
        element = element.child(div().child(render_item_single(
            settings_value,
            child_render,
            window,
            cx,
        )));
    } else if let Some(child_index) = child.first_descendant_index() {
        let mut index = Some(child_index);
        while let Some(sub_child_index) = index {
            element = render_recursive(
                tree,
                sub_child_index,
                path,
                element,
                fallback_path,
                true,
                window,
                cx,
            );
            index = tree[sub_child_index].next_sibling;
        }
    } else {
        element = element.child(div().child(Label::new("// skipped (for now)").color(Color::Muted)))
    }

    if pushed_path {
        path.pop();
        if let Some(fallback_path) = fallback_path.as_mut() {
            fallback_path.pop();
        }
    }
    return element;
}

impl Render for SettingsPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scroll_handle = window.use_state(cx, |_, _| ScrollHandle::new());
        div()
            .grid()
            .grid_cols(16)
            .p_4()
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .child(
                div()
                    .id("settings-ui-nav")
                    .col_span(2)
                    .h_full()
                    .child(render_nav(&self.settings_tree, window, cx)),
            )
            .child(
                div().col_span(6).h_full().child(
                    render_content(&self.settings_tree, window, cx)
                        .id("settings-ui-content")
                        .track_scroll(scroll_handle.read(cx))
                        .overflow_y_scroll(),
                ),
            )
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

fn element_id_from_path(path: &[SharedString]) -> ElementId {
    if path.len() == 0 {
        panic!("Path length must not be zero");
    } else if path.len() == 1 {
        ElementId::Name(path[0].clone())
    } else {
        ElementId::from((
            ElementId::from(path[path.len() - 2].clone()),
            path[path.len() - 1].clone(),
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
        SettingsUiItemSingle::NumericStepper(num_type) => {
            render_any_numeric_stepper(settings_value, *num_type, window, cx)
        }
        SettingsUiItemSingle::ToggleGroup {
            variants: values,
            labels: titles,
        } => render_toggle_button_group(settings_value, values, titles, window, cx),
        SettingsUiItemSingle::DropDown { variants, labels } => {
            render_dropdown(settings_value, variants, labels, window, cx)
        }
        SettingsUiItemSingle::TextField => render_text_field(settings_value, window, cx),
    }
}

pub fn read_settings_value_from_path<'a>(
    settings_contents: &'a serde_json::Value,
    path: &[impl AsRef<str>],
) -> Option<&'a serde_json::Value> {
    // todo(settings_ui) make non recursive, and move to `settings` alongside SettingsValue, and add method to SettingsValue to get nested
    let Some((key, remaining)) = path.split_first() else {
        return Some(settings_contents);
    };
    let Some(value) = settings_contents.get(key.as_ref()) else {
        return None;
    };

    read_settings_value_from_path(value, remaining)
}

fn downcast_any_item<T: serde::de::DeserializeOwned>(
    settings_value: SettingsValue<serde_json::Value>,
) -> SettingsValue<T> {
    let value = settings_value.value.map(|value| {
        serde_json::from_value::<T>(value.clone())
            .with_context(|| format!("path: {:?}", settings_value.path.join(".")))
            .with_context(|| format!("value is not a {}: {}", std::any::type_name::<T>(), value))
            .unwrap()
    });
    // todo(settings_ui) Create test that constructs UI tree, and asserts that all elements have default values
    let default_value = serde_json::from_value::<T>(settings_value.default_value)
        .with_context(|| format!("path: {:?}", settings_value.path.join(".")))
        .with_context(|| format!("value is not a {}", std::any::type_name::<T>()))
        .unwrap();
    let deserialized_setting_value = SettingsValue {
        title: settings_value.title,
        path: settings_value.path,
        documentation: settings_value.documentation,
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

fn render_any_numeric_stepper(
    settings_value: SettingsValue<serde_json::Value>,
    num_type: NumType,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    match num_type {
        NumType::U64 => render_numeric_stepper::<u64>(
            downcast_any_item(settings_value),
            |n| u64::saturating_sub(n, 1),
            |n| u64::saturating_add(n, 1),
            |n| {
                serde_json::Number::try_from(n)
                    .context("Failed to convert u64 to serde_json::Number")
            },
            window,
            cx,
        ),
        NumType::U32 => render_numeric_stepper::<u32>(
            downcast_any_item(settings_value),
            |n| u32::saturating_sub(n, 1),
            |n| u32::saturating_add(n, 1),
            |n| {
                serde_json::Number::try_from(n)
                    .context("Failed to convert u32 to serde_json::Number")
            },
            window,
            cx,
        ),
        NumType::F32 => render_numeric_stepper::<f32>(
            downcast_any_item(settings_value),
            |a| a - 1.0,
            |a| a + 1.0,
            |n| {
                serde_json::Number::from_f64(n as f64)
                    .context("Failed to convert f32 to serde_json::Number")
            },
            window,
            cx,
        ),
        NumType::USIZE => render_numeric_stepper::<usize>(
            downcast_any_item(settings_value),
            |n| usize::saturating_sub(n, 1),
            |n| usize::saturating_add(n, 1),
            |n| {
                serde_json::Number::try_from(n)
                    .context("Failed to convert usize to serde_json::Number")
            },
            window,
            cx,
        ),
        NumType::U32NONZERO => render_numeric_stepper::<NonZeroU32>(
            downcast_any_item(settings_value),
            |a| NonZeroU32::new(u32::saturating_sub(a.get(), 1)).unwrap_or(NonZeroU32::MIN),
            |a| NonZeroU32::new(u32::saturating_add(a.get(), 1)).unwrap_or(NonZeroU32::MAX),
            |n| {
                serde_json::Number::try_from(n.get())
                    .context("Failed to convert usize to serde_json::Number")
            },
            window,
            cx,
        ),
    }
}

fn render_numeric_stepper<T: serde::de::DeserializeOwned + std::fmt::Display + Copy + 'static>(
    value: SettingsValue<T>,
    saturating_sub_1: fn(T) -> T,
    saturating_add_1: fn(T) -> T,
    to_serde_number: fn(T) -> anyhow::Result<serde_json::Number>,
    _window: &mut Window,
    _cx: &mut App,
) -> AnyElement {
    let id = element_id_from_path(&value.path);
    let path = value.path.clone();
    let num = *value.read();

    NumericStepper::new(
        id,
        num.to_string(),
        {
            let path = value.path;
            move |_, _, cx| {
                let Some(number) = to_serde_number(saturating_sub_1(num)).ok() else {
                    return;
                };
                let new_value = serde_json::Value::Number(number);
                SettingsValue::write_value(&path, new_value, cx);
            }
        },
        move |_, _, cx| {
            let Some(number) = to_serde_number(saturating_add_1(num)).ok() else {
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
        value.title.clone(),
        value.documentation.clone(),
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

fn render_text_field(
    value: SettingsValue<serde_json::Value>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let value = downcast_any_item::<String>(value);
    let path = value.path.clone();
    let editor = window.use_state(cx, {
        let path = path.clone();
        move |window, cx| {
            let mut editor = Editor::single_line(window, cx);

            cx.observe_global_in::<SettingsStore>(window, move |editor, window, cx| {
                let user_settings = SettingsStore::global(cx).raw_user_settings();
                if let Some(value) = read_settings_value_from_path(&user_settings, &path).cloned()
                    && let Some(value) = value.as_str()
                {
                    editor.set_text(value, window, cx);
                }
            })
            .detach();

            editor.set_text(value.read().clone(), window, cx);
            editor
        }
    });

    let weak_editor = editor.downgrade();
    let theme_colors = cx.theme().colors();

    div()
        .child(editor)
        .bg(theme_colors.editor_background)
        .border_1()
        .rounded_lg()
        .border_color(theme_colors.border)
        .on_action::<menu::Confirm>({
            move |_, _, cx| {
                let new_value = weak_editor.read_with(cx, |editor, cx| editor.text(cx)).ok();

                if let Some(new_value) = new_value {
                    SettingsValue::write_value(&path, serde_json::Value::String(new_value), cx);
                }
            }
        })
        .into_any_element()
}

fn render_toggle_button_group(
    value: SettingsValue<serde_json::Value>,
    variants: &'static [&'static str],
    labels: &'static [&'static str],
    _: &mut Window,
    _: &mut App,
) -> AnyElement {
    let value = downcast_any_item::<String>(value);
    let active_value = value.read();
    let selected_idx = variants.iter().position(|v| v == &active_value);

    return render_toggle_button_group_inner(value.title, labels, selected_idx, {
        let path = value.path.clone();
        move |variant_index, cx| {
            SettingsValue::write_value(
                &path,
                serde_json::Value::String(variants[variant_index].to_string()),
                cx,
            );
        }
    });
}

fn render_dropdown(
    value: SettingsValue<serde_json::Value>,
    variants: &'static [&'static str],
    labels: &'static [&'static str],
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let value = downcast_any_item::<String>(value);
    let id = element_id_from_path(&value.path);

    let menu = window.use_keyed_state(id.clone(), cx, |window, cx| {
        let path = value.path.clone();
        let handler = Rc::new(move |variant: &'static str, cx: &mut App| {
            SettingsValue::write_value(&path, serde_json::Value::String(variant.to_string()), cx);
        });

        ContextMenu::build(window, cx, |mut menu, _, _| {
            for (label, variant) in labels.iter().zip(variants) {
                menu = menu.entry(*label, None, {
                    let handler = handler.clone();
                    move |_, cx| {
                        handler(variant, cx);
                    }
                });
            }

            menu
        })
    });

    DropdownMenu::new(id, value.read(), menu.read(cx).clone())
        .style(ui::DropdownStyle::Outlined)
        .into_any_element()
}

fn render_toggle_button_group_inner(
    title: SharedString,
    labels: &'static [&'static str],
    selected_idx: Option<usize>,
    on_write: impl Fn(usize, &mut App) + 'static,
) -> AnyElement {
    fn make_toggle_group<const LEN: usize>(
        title: SharedString,
        selected_idx: Option<usize>,
        on_write: Rc<dyn Fn(usize, &mut App)>,
        labels: &'static [&'static str],
    ) -> AnyElement {
        let labels_array: [&'static str; LEN] = {
            let mut arr = ["unused"; LEN];
            arr.copy_from_slice(labels);
            arr
        };

        let mut idx = 0;
        ToggleButtonGroup::single_row(
            title,
            labels_array.map(|label| {
                idx += 1;
                let on_write = on_write.clone();
                ToggleButtonSimple::new(label, move |_, _, cx| {
                    on_write(idx - 1, cx);
                })
            }),
        )
        .when_some(selected_idx, |this, ix| this.selected_index(ix))
        .style(ui::ToggleButtonGroupStyle::Filled)
        .into_any_element()
    }

    let on_write = Rc::new(on_write);

    macro_rules! templ_toggl_with_const_param {
        ($len:expr) => {
            if labels.len() == $len {
                return make_toggle_group::<$len>(title.clone(), selected_idx, on_write, labels);
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
    path: SmallVec<[SharedString; 1]>,
    fallback_path: Option<&[SharedString]>,
    title: SharedString,
    documentation: Option<SharedString>,
    user_settings: &serde_json::Value,
    default_settings: &serde_json::Value,
) -> SettingsValue<serde_json::Value> {
    let default_value = read_settings_value_from_path(default_settings, &path)
        .or_else(|| {
            fallback_path.and_then(|fallback_path| {
                read_settings_value_from_path(default_settings, fallback_path)
            })
        })
        .with_context(|| format!("No default value for item at path {:?}", path.join(".")))
        .expect("Default value set for item")
        .clone();

    let value = read_settings_value_from_path(user_settings, &path).cloned();
    let settings_value = SettingsValue {
        default_value,
        value,
        documentation,
        path,
        // todo(settings_ui) is title required inside SettingsValue?
        title,
    };
    return settings_value;
}
