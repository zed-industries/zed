//! # settings_ui
mod components;
use editor::{Editor, EditorEvent};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use fuzzy::StringMatchCandidate;
use gpui::{
    App, Div, Entity, Global, ReadGlobal as _, Task, TitlebarOptions, UniformListScrollHandle,
    Window, WindowHandle, WindowOptions, actions, div, point, px, size, uniform_list,
};
use project::WorktreeId;
use settings::{CursorShape, SaturatingBool, SettingsContent, SettingsStore};
use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashMap,
    ops::Range,
    rc::Rc,
    sync::{Arc, atomic::AtomicBool},
};
use ui::{
    ContextMenu, Divider, DropdownMenu, DropdownStyle, Switch, SwitchColor, TreeViewItem,
    prelude::*,
};
use util::{paths::PathStyle, rel_path::RelPath};

use crate::components::SettingsEditor;

#[derive(Clone, Copy)]
struct SettingField<T: 'static> {
    pick: fn(&SettingsContent) -> &Option<T>,
    pick_mut: fn(&mut SettingsContent) -> &mut Option<T>,
}

trait AnySettingField {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
    fn file_set_in(&self, file: SettingsUiFile, cx: &App) -> settings::SettingsFile;
}

impl<T> AnySettingField for SettingField<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn file_set_in(&self, file: SettingsUiFile, cx: &App) -> settings::SettingsFile {
        let (file, _) = cx
            .global::<SettingsStore>()
            .get_value_from_file(file.to_settings(), self.pick);
        return file;
    }
}

#[derive(Default, Clone)]
struct SettingFieldRenderer {
    renderers: Rc<
        RefCell<
            HashMap<
                TypeId,
                Box<
                    dyn Fn(
                        &dyn AnySettingField,
                        SettingsUiFile,
                        Option<&SettingsFieldMetadata>,
                        &mut Window,
                        &mut App,
                    ) -> AnyElement,
                >,
            >,
        >,
    >,
}

impl Global for SettingFieldRenderer {}

impl SettingFieldRenderer {
    fn add_renderer<T: 'static>(
        &mut self,
        renderer: impl Fn(
            &SettingField<T>,
            SettingsUiFile,
            Option<&SettingsFieldMetadata>,
            &mut Window,
            &mut App,
        ) -> AnyElement
        + 'static,
    ) -> &mut Self {
        let key = TypeId::of::<T>();
        let renderer = Box::new(
            move |any_setting_field: &dyn AnySettingField,
                  settings_file: SettingsUiFile,
                  metadata: Option<&SettingsFieldMetadata>,
                  window: &mut Window,
                  cx: &mut App| {
                let field = any_setting_field
                    .as_any()
                    .downcast_ref::<SettingField<T>>()
                    .unwrap();
                renderer(field, settings_file, metadata, window, cx)
            },
        );
        self.renderers.borrow_mut().insert(key, renderer);
        self
    }

    fn render(
        &self,
        any_setting_field: &dyn AnySettingField,
        settings_file: SettingsUiFile,
        metadata: Option<&SettingsFieldMetadata>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let key = any_setting_field.type_id();
        if let Some(renderer) = self.renderers.borrow().get(&key) {
            renderer(any_setting_field, settings_file, metadata, window, cx)
        } else {
            panic!(
                "No renderer found for type: {}",
                any_setting_field.type_name()
            )
        }
    }
}

struct SettingsFieldMetadata {
    placeholder: Option<&'static str>,
}

fn user_settings_data() -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "General Page",
            expanded: true,
            items: vec![
                SettingsPageItem::SectionHeader("General"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Confirm Quit",
                    description: "Whether to confirm before quitting Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.workspace.confirm_quit,
                        pick_mut: |settings_content| &mut settings_content.workspace.confirm_quit,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Update",
                    description: "Automatically update Zed (may be ignored on Linux if installed through a package manager)",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.auto_update,
                        pick_mut: |settings_content| &mut settings_content.auto_update,
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Privacy"),
            ],
        },
        SettingsPage {
            title: "Project",
            expanded: true,
            items: vec![
                SettingsPageItem::SectionHeader("Worktree Settings Content"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Name",
                    description: "The displayed name of this project. If not set, the root directory name",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.project.worktree.project_name,
                        pick_mut: |settings_content| {
                            &mut settings_content.project.worktree.project_name
                        },
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        placeholder: Some("A new name"),
                    })),
                }),
            ],
        },
        SettingsPage {
            title: "AI",
            expanded: true,
            items: vec![
                SettingsPageItem::SectionHeader("General"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Disable AI",
                    description: "Whether to disable all AI features in Zed",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.disable_ai,
                        pick_mut: |settings_content| &mut settings_content.disable_ai,
                    }),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "Appearance & Behavior",
            expanded: true,
            items: vec![
                SettingsPageItem::SectionHeader("Cursor"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Cursor Shape",
                    description: "Cursor shape for the editor",
                    field: Box::new(SettingField {
                        pick: |settings_content| &settings_content.editor.cursor_shape,
                        pick_mut: |settings_content| &mut settings_content.editor.cursor_shape,
                    }),
                    metadata: None,
                }),
            ],
        },
    ]
}

// Derive Macro, on the new ProjectSettings struct

fn project_settings_data() -> Vec<SettingsPage> {
    vec![SettingsPage {
        title: "Project",
        expanded: true,
        items: vec![
            SettingsPageItem::SectionHeader("Worktree Settings Content"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Project Name",
                description: "The displayed name of this project. If not set, the root directory name",
                field: Box::new(SettingField {
                    pick: |settings_content| &settings_content.project.worktree.project_name,
                    pick_mut: |settings_content| {
                        &mut settings_content.project.worktree.project_name
                    },
                }),
                metadata: Some(Box::new(SettingsFieldMetadata {
                    placeholder: Some("A new name"),
                })),
            }),
        ],
    }]
}

pub struct SettingsUiFeatureFlag;

impl FeatureFlag for SettingsUiFeatureFlag {
    const NAME: &'static str = "settings-ui";
}

actions!(
    zed,
    [
        /// Opens Settings Editor.
        OpenSettingsEditor
    ]
);

pub fn init(cx: &mut App) {
    init_renderers(cx);

    cx.observe_new(|workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action_renderer(|div, _, _, cx| {
            let settings_ui_actions = [std::any::TypeId::of::<OpenSettingsEditor>()];
            let has_flag = cx.has_flag::<SettingsUiFeatureFlag>();
            command_palette_hooks::CommandPaletteFilter::update_global(cx, |filter, _| {
                if has_flag {
                    filter.show_action_types(&settings_ui_actions);
                } else {
                    filter.hide_action_types(&settings_ui_actions);
                }
            });
            if has_flag {
                div.on_action(cx.listener(|_, _: &OpenSettingsEditor, _, cx| {
                    open_settings_editor(cx).ok();
                }))
            } else {
                div
            }
        });
    })
    .detach();
}

fn init_renderers(cx: &mut App) {
    // fn (field: SettingsField, current_file: SettingsFile, cx) -> (currently_set_in: SettingsFile, overridden_in: Vec<SettingsFile>)
    cx.default_global::<SettingFieldRenderer>()
        .add_renderer::<bool>(|settings_field, file, _, _, cx| {
            render_toggle_button(*settings_field, file, cx).into_any_element()
        })
        .add_renderer::<String>(|settings_field, file, metadata, _, cx| {
            render_text_field(settings_field.clone(), file, metadata, cx)
        })
        .add_renderer::<SaturatingBool>(|settings_field, file, _, _, cx| {
            render_toggle_button(*settings_field, file, cx)
        })
        .add_renderer::<CursorShape>(|settings_field, file, _, window, cx| {
            render_dropdown(*settings_field, file, window, cx)
        });
}

pub fn open_settings_editor(cx: &mut App) -> anyhow::Result<WindowHandle<SettingsWindow>> {
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Settings Window".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(12.0), px(12.0))),
            }),
            focus: true,
            show: true,
            kind: gpui::WindowKind::Normal,
            window_background: cx.theme().window_background_appearance(),
            window_min_size: Some(size(px(800.), px(600.))), // 4:3 Aspect Ratio
            ..Default::default()
        },
        |window, cx| cx.new(|cx| SettingsWindow::new(window, cx)),
    )
}

pub struct SettingsWindow {
    files: Vec<SettingsUiFile>,
    current_file: SettingsUiFile,
    pages: Vec<SettingsPage>,
    search_bar: Entity<Editor>,
    search_task: Option<Task<()>>,
    navbar_entry: usize, // Index into pages - should probably be (usize, Option<usize>) for section + page
    navbar_entries: Vec<NavBarEntry>,
    list_handle: UniformListScrollHandle,
    search_matches: Vec<Vec<bool>>,
}

#[derive(PartialEq, Debug)]
struct NavBarEntry {
    title: &'static str,
    is_root: bool,
    page_index: usize,
}

struct SettingsPage {
    title: &'static str,
    expanded: bool,
    items: Vec<SettingsPageItem>,
}

#[derive(PartialEq)]
enum SettingsPageItem {
    SectionHeader(&'static str),
    SettingItem(SettingItem),
}

impl std::fmt::Debug for SettingsPageItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsPageItem::SectionHeader(header) => write!(f, "SectionHeader({})", header),
            SettingsPageItem::SettingItem(setting_item) => {
                write!(f, "SettingItem({})", setting_item.title)
            }
        }
    }
}

impl SettingsPageItem {
    fn render(
        &self,
        file: SettingsUiFile,
        is_last: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        match self {
            SettingsPageItem::SectionHeader(header) => v_flex()
                .w_full()
                .gap_1()
                .child(
                    Label::new(SharedString::new_static(header))
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .buffer_font(cx),
                )
                .child(Divider::horizontal().color(ui::DividerColor::BorderVariant))
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => {
                let renderer = cx.default_global::<SettingFieldRenderer>().clone();
                let file_set_in =
                    SettingsUiFile::from_settings(setting_item.field.file_set_in(file.clone(), cx));

                h_flex()
                    .id(setting_item.title)
                    .w_full()
                    .gap_2()
                    .flex_wrap()
                    .justify_between()
                    .when(!is_last, |this| {
                        this.pb_4()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                    })
                    .child(
                        v_flex()
                            .max_w_1_2()
                            .flex_shrink()
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_4()
                                    .child(
                                        Label::new(SharedString::new_static(setting_item.title))
                                            .size(LabelSize::Default),
                                    )
                                    .when_some(
                                        file_set_in.filter(|file_set_in| file_set_in != &file),
                                        |elem, file_set_in| {
                                            elem.child(
                                                Label::new(format!(
                                                    "set in {}",
                                                    file_set_in.name()
                                                ))
                                                .color(Color::Muted),
                                            )
                                        },
                                    ),
                            )
                            .child(
                                Label::new(SharedString::new_static(setting_item.description))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(renderer.render(
                        setting_item.field.as_ref(),
                        file,
                        setting_item.metadata.as_deref(),
                        window,
                        cx,
                    ))
                    .into_any_element()
            }
        }
    }
}

struct SettingItem {
    title: &'static str,
    description: &'static str,
    field: Box<dyn AnySettingField>,
    metadata: Option<Box<SettingsFieldMetadata>>,
}

impl PartialEq for SettingItem {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && (match (&self.metadata, &other.metadata) {
                (None, None) => true,
                (Some(m1), Some(m2)) => m1.placeholder == m2.placeholder,
                _ => false,
            })
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq)]
enum SettingsUiFile {
    User,                              // Uses all settings.
    Local((WorktreeId, Arc<RelPath>)), // Has a special name, and special set of settings
    Server(&'static str),              // Uses a special name, and the user settings
}

impl SettingsUiFile {
    fn pages(&self) -> Vec<SettingsPage> {
        match self {
            SettingsUiFile::User => user_settings_data(),
            SettingsUiFile::Local(_) => project_settings_data(),
            SettingsUiFile::Server(_) => user_settings_data(),
        }
    }

    fn name(&self) -> SharedString {
        match self {
            SettingsUiFile::User => SharedString::new_static("User"),
            // TODO is PathStyle::local() ever not appropriate?
            SettingsUiFile::Local((_, path)) => {
                format!("Local ({})", path.display(PathStyle::local())).into()
            }
            SettingsUiFile::Server(file) => format!("Server ({})", file).into(),
        }
    }

    fn from_settings(file: settings::SettingsFile) -> Option<Self> {
        Some(match file {
            settings::SettingsFile::User => SettingsUiFile::User,
            settings::SettingsFile::Local(location) => SettingsUiFile::Local(location),
            settings::SettingsFile::Server => SettingsUiFile::Server("todo: server name"),
            settings::SettingsFile::Default => return None,
        })
    }

    fn to_settings(&self) -> settings::SettingsFile {
        match self {
            SettingsUiFile::User => settings::SettingsFile::User,
            SettingsUiFile::Local(location) => settings::SettingsFile::Local(location.clone()),
            SettingsUiFile::Server(_) => settings::SettingsFile::Server,
        }
    }
}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_file = SettingsUiFile::User;
        let search_bar = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search settings…", window, cx);
            editor
        });

        cx.subscribe(&search_bar, |this, _, event: &EditorEvent, cx| {
            let EditorEvent::Edited { transaction_id: _ } = event else {
                return;
            };

            this.update_matches(cx);
        })
        .detach();

        cx.observe_global_in::<SettingsStore>(window, move |this, _, cx| {
            this.fetch_files(cx);
            cx.notify();
        })
        .detach();

        let mut this = Self {
            files: vec![],
            current_file: current_file,
            pages: vec![],
            navbar_entries: vec![],
            navbar_entry: 0,
            list_handle: UniformListScrollHandle::default(),
            search_bar,
            search_task: None,
            search_matches: vec![],
        };

        this.fetch_files(cx);
        this.build_ui(cx);

        this
    }

    fn toggle_navbar_entry(&mut self, ix: usize) {
        // We can only toggle root entries
        if !self.navbar_entries[ix].is_root {
            return;
        }

        let toggle_page_index = self.page_index_from_navbar_index(ix);
        let selected_page_index = self.page_index_from_navbar_index(self.navbar_entry);

        let expanded = &mut self.page_for_navbar_index(ix).expanded;
        *expanded = !*expanded;
        let expanded = *expanded;
        // if currently selected page is a child of the parent page we are folding,
        // set the current page to the parent page
        if selected_page_index == toggle_page_index {
            self.navbar_entry = ix;
        } else if selected_page_index > toggle_page_index {
            let sub_items_count = self.pages[toggle_page_index]
                .items
                .iter()
                .filter(|item| matches!(item, SettingsPageItem::SectionHeader(_)))
                .count();
            if expanded {
                self.navbar_entry += sub_items_count;
            } else {
                self.navbar_entry -= sub_items_count;
            }
        }

        self.build_navbar();
    }

    fn build_navbar(&mut self) {
        let mut navbar_entries = Vec::with_capacity(self.navbar_entries.len());
        for (page_index, page) in self.pages.iter().enumerate() {
            if !self.search_matches[page_index]
                .iter()
                .any(|is_match| *is_match)
                && !self.search_matches[page_index].is_empty()
            {
                continue;
            }
            navbar_entries.push(NavBarEntry {
                title: page.title,
                is_root: true,
                page_index,
            });
            if !page.expanded {
                continue;
            }

            for (item_index, item) in page.items.iter().enumerate() {
                let SettingsPageItem::SectionHeader(title) = item else {
                    continue;
                };
                if !self.search_matches[page_index][item_index] {
                    continue;
                }

                navbar_entries.push(NavBarEntry {
                    title,
                    is_root: false,
                    page_index,
                });
            }
        }
        self.navbar_entries = navbar_entries;
    }

    fn update_matches(&mut self, cx: &mut Context<SettingsWindow>) {
        self.search_task.take();
        let query = self.search_bar.read(cx).text(cx);
        if query.is_empty() {
            for page in &mut self.search_matches {
                page.fill(true);
            }
            self.build_navbar();
            cx.notify();
            return;
        }

        struct ItemKey {
            page_index: usize,
            header_index: usize,
            item_index: usize,
        }
        let mut key_lut: Vec<ItemKey> = vec![];
        let mut candidates = Vec::default();

        for (page_index, page) in self.pages.iter().enumerate() {
            let mut header_index = 0;
            for (item_index, item) in page.items.iter().enumerate() {
                let key_index = key_lut.len();
                match item {
                    SettingsPageItem::SettingItem(item) => {
                        candidates.push(StringMatchCandidate::new(key_index, item.title));
                        candidates.push(StringMatchCandidate::new(key_index, item.description));
                    }
                    SettingsPageItem::SectionHeader(header) => {
                        candidates.push(StringMatchCandidate::new(key_index, header));
                        header_index = item_index;
                    }
                }
                key_lut.push(ItemKey {
                    page_index,
                    header_index,
                    item_index,
                });
            }
        }
        let atomic_bool = AtomicBool::new(false);

        self.search_task = Some(cx.spawn(async move |this, cx| {
            let string_matches = fuzzy::match_strings(
                candidates.as_slice(),
                &query,
                false,
                false,
                candidates.len(),
                &atomic_bool,
                cx.background_executor().clone(),
            );
            let string_matches = string_matches.await;

            this.update(cx, |this, cx| {
                for page in &mut this.search_matches {
                    page.fill(false);
                }

                for string_match in string_matches {
                    let ItemKey {
                        page_index,
                        header_index,
                        item_index,
                    } = key_lut[string_match.candidate_id];
                    let page = &mut this.search_matches[page_index];
                    page[header_index] = true;
                    page[item_index] = true;
                }
                this.build_navbar();
                this.navbar_entry = 0;
                cx.notify();
            })
            .ok();
        }));
    }

    fn build_ui(&mut self, cx: &mut Context<SettingsWindow>) {
        self.pages = self.current_file.pages();
        self.search_matches = self
            .pages
            .iter()
            .map(|page| vec![true; page.items.len()])
            .collect::<Vec<_>>();
        self.build_navbar();

        if !self.search_bar.read(cx).is_empty(cx) {
            self.update_matches(cx);
        }

        cx.notify();
    }

    fn fetch_files(&mut self, cx: &mut Context<SettingsWindow>) {
        let settings_store = cx.global::<SettingsStore>();
        let mut ui_files = vec![];
        let all_files = settings_store.get_all_files();
        for file in all_files {
            let Some(settings_ui_file) = SettingsUiFile::from_settings(file) else {
                continue;
            };
            ui_files.push(settings_ui_file);
        }
        ui_files.reverse();
        self.files = ui_files;
        if !self.files.contains(&self.current_file) {
            self.change_file(0, cx);
        }
    }

    fn change_file(&mut self, ix: usize, cx: &mut Context<SettingsWindow>) {
        if ix >= self.files.len() {
            self.current_file = SettingsUiFile::User;
            return;
        }
        if self.files[ix] == self.current_file {
            return;
        }
        self.current_file = self.files[ix].clone();
        self.navbar_entry = 0;
        self.build_ui(cx);
    }

    fn render_files(&self, _window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        h_flex()
            .gap_1()
            .children(self.files.iter().enumerate().map(|(ix, file)| {
                Button::new(ix, file.name())
                    .on_click(cx.listener(move |this, _, _window, cx| this.change_file(ix, cx)))
            }))
    }

    fn render_search(&self, _window: &mut Window, cx: &mut App) -> Div {
        h_flex()
            .pt_1()
            .px_1p5()
            .gap_1p5()
            .rounded_sm()
            .bg(cx.theme().colors().editor_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
            .child(self.search_bar.clone())
    }

    fn render_nav(&self, window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        v_flex()
            .w_64()
            .p_2p5()
            .pt_10()
            .gap_3()
            .flex_none()
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().panel_background)
            .child(self.render_search(window, cx).pb_1())
            .child(
                uniform_list(
                    "settings-ui-nav-bar",
                    self.navbar_entries.len(),
                    cx.processor(|this, range: Range<usize>, _, cx| {
                        range
                            .into_iter()
                            .map(|ix| {
                                let entry = &this.navbar_entries[ix];

                                TreeViewItem::new(("settings-ui-navbar-entry", ix), entry.title)
                                    .root_item(entry.is_root)
                                    .toggle_state(this.is_navbar_entry_selected(ix))
                                    .when(entry.is_root, |item| {
                                        item.toggle(
                                            this.pages[this.page_index_from_navbar_index(ix)]
                                                .expanded,
                                        )
                                        .on_toggle(
                                            cx.listener(move |this, _, _, cx| {
                                                this.toggle_navbar_entry(ix);
                                                cx.notify();
                                            }),
                                        )
                                    })
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.navbar_entry = ix;
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            })
                            .collect()
                    }),
                )
                .track_scroll(self.list_handle.clone())
                .size_full()
                .flex_grow(),
            )
    }

    fn page_items(&self) -> impl Iterator<Item = &SettingsPageItem> {
        let page_idx = self.current_page_index();

        self.current_page()
            .items
            .iter()
            .enumerate()
            .filter_map(move |(item_index, item)| {
                self.search_matches[page_idx][item_index].then_some(item)
            })
    }

    fn render_page(&self, window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        let items: Vec<_> = self.page_items().collect();
        let items_len = items.len();

        v_flex()
            .gap_4()
            .children(items.into_iter().enumerate().map(|(index, item)| {
                let is_last = index == items_len - 1;
                item.render(self.current_file.clone(), is_last, window, cx)
            }))
    }

    fn current_page_index(&self) -> usize {
        self.page_index_from_navbar_index(self.navbar_entry)
    }

    fn current_page(&self) -> &SettingsPage {
        &self.pages[self.current_page_index()]
    }

    fn page_index_from_navbar_index(&self, index: usize) -> usize {
        if self.navbar_entries.is_empty() {
            return 0;
        }

        self.navbar_entries[index].page_index
    }

    fn page_for_navbar_index(&mut self, index: usize) -> &mut SettingsPage {
        let index = self.page_index_from_navbar_index(index);
        &mut self.pages[index]
    }

    fn is_navbar_entry_selected(&self, ix: usize) -> bool {
        ix == self.navbar_entry
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme::setup_ui_font(window, cx);

        div()
            .flex()
            .flex_row()
            .size_full()
            .font(ui_font)
            .bg(cx.theme().colors().background)
            .text_color(cx.theme().colors().text)
            .child(self.render_nav(window, cx))
            .child(
                v_flex()
                    .w_full()
                    .pt_4()
                    .px_6()
                    .gap_4()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.render_files(window, cx))
                    .child(self.render_page(window, cx)),
            )
    }
}

// fn read_field<T>(pick: fn(&SettingsContent) -> &Option<T>, file: SettingsFile, cx: &App) -> Option<T> {
//     let (_, value) = cx.global::<SettingsStore>().get_value_from_file(file.to_settings(), (), pick);
// }

fn render_text_field(
    field: SettingField<String>,
    file: SettingsUiFile,
    metadata: Option<&SettingsFieldMetadata>,
    cx: &mut App,
) -> AnyElement {
    let (_, initial_text) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let initial_text = Some(initial_text.clone()).filter(|s| !s.is_empty());

    SettingsEditor::new()
        .when_some(initial_text, |editor, text| editor.with_initial_text(text))
        .when_some(
            metadata.and_then(|metadata| metadata.placeholder),
            |editor, placeholder| editor.with_placeholder(placeholder),
        )
        .on_confirm(move |new_text, cx: &mut App| {
            cx.update_global(move |store: &mut SettingsStore, cx| {
                store.update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _cx| {
                    *(field.pick_mut)(settings) = new_text;
                });
            });
        })
        .into_any_element()
}

fn render_toggle_button<B: Into<bool> + From<bool> + Copy>(
    field: SettingField<B>,
    file: SettingsUiFile,
    cx: &mut App,
) -> AnyElement {
    let (_, &value) = SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);

    let toggle_state = if value.into() {
        ToggleState::Selected
    } else {
        ToggleState::Unselected
    };

    Switch::new("toggle_button", toggle_state)
        .on_click({
            move |state, _window, cx| {
                let state = *state == ui::ToggleState::Selected;
                let field = field;
                cx.update_global(move |store: &mut SettingsStore, cx| {
                    store.update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _cx| {
                        *(field.pick_mut)(settings) = Some(state.into());
                    });
                });
            }
        })
        .color(SwitchColor::Accent)
        .into_any_element()
}

fn render_dropdown<T>(
    field: SettingField<T>,
    file: SettingsUiFile,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + 'static,
{
    let variants = || -> &'static [T] { <T as strum::VariantArray>::VARIANTS };
    let labels = || -> &'static [&'static str] { <T as strum::VariantNames>::VARIANTS };

    let (_, &current_value) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);

    let current_value_label =
        labels()[variants().iter().position(|v| *v == current_value).unwrap()];

    DropdownMenu::new(
        "dropdown",
        current_value_label,
        ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (value, label) in variants()
                .into_iter()
                .copied()
                .zip(labels().into_iter().copied())
            {
                menu = menu.toggleable_entry(
                    label,
                    value == current_value,
                    IconPosition::Start,
                    None,
                    move |_, cx| {
                        if value == current_value {
                            return;
                        }
                        cx.update_global(move |store: &mut SettingsStore, cx| {
                            store.update_settings_file(
                                <dyn fs::Fs>::global(cx),
                                move |settings, _cx| {
                                    *(field.pick_mut)(settings) = Some(value);
                                },
                            );
                        });
                    },
                );
            }
            menu
        }),
    )
    .style(DropdownStyle::Outlined)
    .into_any_element()
}

#[cfg(test)]
mod test {

    use super::*;

    impl SettingsWindow {
        fn navbar(&self) -> &[NavBarEntry] {
            self.navbar_entries.as_slice()
        }

        fn navbar_entry(&self) -> usize {
            self.navbar_entry
        }

        fn new_builder(window: &mut Window, cx: &mut Context<Self>) -> Self {
            let mut this = Self::new(window, cx);
            this.navbar_entries.clear();
            this.pages.clear();
            this
        }

        fn build(mut self) -> Self {
            self.build_navbar();
            self
        }

        fn add_page(
            mut self,
            title: &'static str,
            build_page: impl Fn(SettingsPage) -> SettingsPage,
        ) -> Self {
            let page = SettingsPage {
                title,
                expanded: false,
                items: Vec::default(),
            };

            self.pages.push(build_page(page));
            self
        }

        fn search(&mut self, search_query: &str, window: &mut Window, cx: &mut Context<Self>) {
            self.search_task.take();
            self.search_bar.update(cx, |editor, cx| {
                editor.set_text(search_query, window, cx);
            });
            self.update_matches(cx);
        }

        fn assert_search_results(&self, other: &Self) {
            // page index could be different because of filtered out pages
            assert!(
                self.navbar_entries
                    .iter()
                    .zip(other.navbar_entries.iter())
                    .all(|(entry, other)| {
                        entry.is_root == other.is_root && entry.title == other.title
                    })
            );
            assert_eq!(
                self.current_page().items.iter().collect::<Vec<_>>(),
                other.page_items().collect::<Vec<_>>()
            );
        }
    }

    impl SettingsPage {
        fn item(mut self, item: SettingsPageItem) -> Self {
            self.items.push(item);
            self
        }
    }

    impl SettingsPageItem {
        fn basic_item(title: &'static str, description: &'static str) -> Self {
            SettingsPageItem::SettingItem(SettingItem {
                title,
                description,
                field: Box::new(SettingField {
                    pick: |settings_content| &settings_content.auto_update,
                    pick_mut: |settings_content| &mut settings_content.auto_update,
                }),
                metadata: None,
            })
        }
    }

    fn register_settings(cx: &mut App) {
        settings::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        workspace::init_settings(cx);
        project::Project::init_settings(cx);
        language::init(cx);
        editor::init(cx);
        menu::init();
    }

    fn parse(input: &'static str, window: &mut Window, cx: &mut App) -> SettingsWindow {
        let mut pages: Vec<SettingsPage> = Vec::new();
        let mut current_page = None;
        let mut selected_idx = None;
        let mut ix = 0;
        let mut in_closed_subentry = false;

        for mut line in input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
        {
            let mut is_selected = false;
            if line.ends_with("*") {
                assert!(
                    selected_idx.is_none(),
                    "Can only have one selected navbar entry at a time"
                );
                selected_idx = Some(ix);
                line = &line[..line.len() - 1];
                is_selected = true;
            }

            if line.starts_with("v") || line.starts_with(">") {
                if let Some(current_page) = current_page.take() {
                    pages.push(current_page);
                }

                let expanded = line.starts_with("v");
                in_closed_subentry = !expanded;
                ix += 1;

                current_page = Some(SettingsPage {
                    title: line.split_once(" ").unwrap().1,
                    expanded,
                    items: Vec::default(),
                });
            } else if line.starts_with("-") {
                if !in_closed_subentry {
                    ix += 1;
                } else if is_selected && in_closed_subentry {
                    panic!("Can't select sub entry if it's parent is closed");
                }

                let Some(current_page) = current_page.as_mut() else {
                    panic!("Sub entries must be within a page");
                };

                current_page.items.push(SettingsPageItem::SectionHeader(
                    line.split_once(" ").unwrap().1,
                ));
            } else {
                panic!(
                    "Entries must start with one of 'v', '>', or '-'\n line: {}",
                    line
                );
            }
        }

        if let Some(current_page) = current_page.take() {
            pages.push(current_page);
        }

        let search_matches = pages
            .iter()
            .map(|page| vec![true; page.items.len()])
            .collect::<Vec<_>>();

        let mut settings_window = SettingsWindow {
            files: Vec::default(),
            current_file: crate::SettingsUiFile::User,
            pages,
            search_bar: cx.new(|cx| Editor::single_line(window, cx)),
            navbar_entry: selected_idx.expect("Must have a selected navbar entry"),
            navbar_entries: Vec::default(),
            list_handle: UniformListScrollHandle::default(),
            search_matches,
            search_task: None,
        };

        settings_window.build_navbar();
        settings_window
    }

    #[track_caller]
    fn check_navbar_toggle(
        before: &'static str,
        toggle_idx: usize,
        after: &'static str,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut settings_window = parse(before, window, cx);
        settings_window.toggle_navbar_entry(toggle_idx);

        let expected_settings_window = parse(after, window, cx);

        assert_eq!(settings_window.navbar(), expected_settings_window.navbar());
        assert_eq!(
            settings_window.navbar_entry(),
            expected_settings_window.navbar_entry()
        );
    }

    macro_rules! check_navbar_toggle {
        ($name:ident, before: $before:expr, toggle_idx: $toggle_idx:expr, after: $after:expr) => {
            #[gpui::test]
            fn $name(cx: &mut gpui::TestAppContext) {
                let window = cx.add_empty_window();
                window.update(|window, cx| {
                    register_settings(cx);
                    check_navbar_toggle($before, $toggle_idx, $after, window, cx);
                });
            }
        };
    }

    check_navbar_toggle!(
        navbar_basic_open,
        before: r"
        v General
        - General
        - Privacy*
        v Project
        - Project Settings
        ",
        toggle_idx: 0,
        after: r"
        > General*
        v Project
        - Project Settings
        "
    );

    check_navbar_toggle!(
        navbar_basic_close,
        before: r"
        > General*
        - General
        - Privacy
        v Project
        - Project Settings
        ",
        toggle_idx: 0,
        after: r"
        v General*
        - General
        - Privacy
        v Project
        - Project Settings
        "
    );

    check_navbar_toggle!(
        navbar_basic_second_root_entry_close,
        before: r"
        > General
        - General
        - Privacy
        v Project
        - Project Settings*
        ",
        toggle_idx: 1,
        after: r"
        > General
        > Project*
        "
    );

    check_navbar_toggle!(
        navbar_toggle_subroot,
        before: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content*
        v AI
        - General
        > Appearance & Behavior
        ",
        toggle_idx: 3,
        after: r"
        v General Page
        - General
        - Privacy
        > Project*
        v AI
        - General
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_close_propagates_selected_index,
        before: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_idx: 0,
        after: r"
        > General Page
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_expand_propagates_selected_index,
        before: r"
        > General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_idx: 0,
        after: r"
        v General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    check_navbar_toggle!(
        navbar_toggle_sub_entry_does_nothing,
        before: r"
        > General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        ",
        toggle_idx: 4,
        after: r"
        > General Page
        - General
        - Privacy
        v Project
        - Worktree Settings Content
        v AI
        - General*
        > Appearance & Behavior
        "
    );

    #[gpui::test]
    fn test_basic_search(cx: &mut gpui::TestAppContext) {
        let cx = cx.add_empty_window();
        let (actual, expected) = cx.update(|window, cx| {
            register_settings(cx);

            let expected = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item("test title", "General test"))
                    })
                    .build()
            });

            let actual = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item("test title", "General test"))
                    })
                    .add_page("Theme", |page| {
                        page.item(SettingsPageItem::SectionHeader("Theme settings"))
                    })
                    .build()
            });

            actual.update(cx, |settings, cx| settings.search("gen", window, cx));

            (actual, expected)
        });

        cx.cx.run_until_parked();

        cx.update(|_window, cx| {
            let expected = expected.read(cx);
            let actual = actual.read(cx);
            expected.assert_search_results(&actual);
        })
    }

    #[gpui::test]
    fn test_search_render_page_with_filtered_out_navbar_entries(cx: &mut gpui::TestAppContext) {
        let cx = cx.add_empty_window();
        let (actual, expected) = cx.update(|window, cx| {
            register_settings(cx);

            let actual = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("General", |page| {
                        page.item(SettingsPageItem::SectionHeader("General settings"))
                            .item(SettingsPageItem::basic_item(
                                "Confirm Quit",
                                "Whether to confirm before quitting Zed",
                            ))
                            .item(SettingsPageItem::basic_item(
                                "Auto Update",
                                "Automatically update Zed",
                            ))
                    })
                    .add_page("AI", |page| {
                        page.item(SettingsPageItem::basic_item(
                            "Disable AI",
                            "Whether to disable all AI features in Zed",
                        ))
                    })
                    .add_page("Appearance & Behavior", |page| {
                        page.item(SettingsPageItem::SectionHeader("Cursor")).item(
                            SettingsPageItem::basic_item(
                                "Cursor Shape",
                                "Cursor shape for the editor",
                            ),
                        )
                    })
                    .build()
            });

            let expected = cx.new(|cx| {
                SettingsWindow::new_builder(window, cx)
                    .add_page("Appearance & Behavior", |page| {
                        page.item(SettingsPageItem::SectionHeader("Cursor")).item(
                            SettingsPageItem::basic_item(
                                "Cursor Shape",
                                "Cursor shape for the editor",
                            ),
                        )
                    })
                    .build()
            });

            actual.update(cx, |settings, cx| settings.search("cursor", window, cx));

            (actual, expected)
        });

        cx.cx.run_until_parked();

        cx.update(|_window, cx| {
            let expected = expected.read(cx);
            let actual = actual.read(cx);
            expected.assert_search_results(&actual);
        })
    }
}
