//! # settings_ui
mod components;
use editor::Editor;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    App, AppContext as _, Context, Div, Entity, Global, IntoElement, ReadGlobal as _, Render,
    TitlebarOptions, UniformListScrollHandle, Window, WindowHandle, WindowOptions, actions, div,
    point, px, size, uniform_list,
};
use project::WorktreeId;
use settings::{CursorShape, SaturatingBool, SettingsContent, SettingsStore};
use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    collections::HashMap,
    ops::Range,
    rc::Rc,
    sync::Arc,
};
use ui::{Divider, DropdownMenu, ListItem, Switch, prelude::*};
use util::{paths::PathStyle, rel_path::RelPath};

use crate::components::SettingsEditor;

#[derive(Clone)]
struct SettingField<T: 'static> {
    pick: fn(&SettingsContent) -> &T,
    pick_mut: fn(&mut SettingsContent) -> &mut T,
}

trait AnySettingField {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
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
            Option<&SettingsFieldMetadata>,
            &mut Window,
            &mut App,
        ) -> AnyElement
        + 'static,
    ) -> &mut Self {
        let key = TypeId::of::<T>();
        let renderer = Box::new(
            move |any_setting_field: &dyn AnySettingField,
                  metadata: Option<&SettingsFieldMetadata>,
                  window: &mut Window,
                  cx: &mut App| {
                let field = any_setting_field
                    .as_any()
                    .downcast_ref::<SettingField<T>>()
                    .unwrap();
                renderer(field, metadata, window, cx)
            },
        );
        self.renderers.borrow_mut().insert(key, renderer);
        self
    }

    fn render(
        &self,
        any_setting_field: &dyn AnySettingField,
        metadata: Option<&SettingsFieldMetadata>,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let key = any_setting_field.type_id();
        if let Some(renderer) = self.renderers.borrow().get(&key) {
            renderer(any_setting_field, metadata, window, cx)
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
    cx.default_global::<SettingFieldRenderer>()
        .add_renderer::<Option<bool>>(|settings_field, _, _, cx| {
            render_toggle_button(settings_field.clone(), cx).into_any_element()
        })
        .add_renderer::<Option<String>>(|settings_field, metadata, _, cx| {
            render_text_field(settings_field.clone(), metadata, cx)
        })
        .add_renderer::<Option<SaturatingBool>>(|settings_field, _, _, cx| {
            render_toggle_button(settings_field.clone(), cx)
        })
        .add_renderer::<Option<CursorShape>>(|settings_field, _, window, cx| {
            render_dropdown(settings_field.clone(), window, cx)
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
    files: Vec<SettingsFile>,
    current_file: SettingsFile,
    pages: Vec<SettingsPage>,
    search: Entity<Editor>,
    navbar_entry: usize, // Index into pages - should probably be (usize, Option<usize>) for section + page
    navbar_entries: Vec<NavBarEntry>,
    list_handle: UniformListScrollHandle,
}

#[derive(PartialEq, Debug)]
struct NavBarEntry {
    title: &'static str,
    is_root: bool,
}

struct SettingsPage {
    title: &'static str,
    expanded: bool,
    items: Vec<SettingsPageItem>,
}

impl SettingsPage {
    fn section_headers(&self) -> impl Iterator<Item = &'static str> {
        self.items.iter().filter_map(|item| match item {
            SettingsPageItem::SectionHeader(header) => Some(*header),
            _ => None,
        })
    }
}

enum SettingsPageItem {
    SectionHeader(&'static str),
    SettingItem(SettingItem),
}

impl SettingsPageItem {
    fn render(&self, _file: SettingsFile, window: &mut Window, cx: &mut App) -> AnyElement {
        match self {
            SettingsPageItem::SectionHeader(header) => v_flex()
                .w_full()
                .gap_0p5()
                .child(Label::new(SharedString::new_static(header)).size(LabelSize::Large))
                .child(Divider::horizontal().color(ui::DividerColor::BorderVariant))
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => {
                let renderer = cx.default_global::<SettingFieldRenderer>().clone();
                h_flex()
                    .id(setting_item.title)
                    .w_full()
                    .gap_2()
                    .flex_wrap()
                    .justify_between()
                    .child(
                        v_flex()
                            .max_w_1_2()
                            .flex_shrink()
                            .child(
                                Label::new(SharedString::new_static(setting_item.title))
                                    .size(LabelSize::Default),
                            )
                            .child(
                                Label::new(SharedString::new_static(setting_item.description))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .child(renderer.render(
                        setting_item.field.as_ref(),
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

#[allow(unused)]
#[derive(Clone, PartialEq)]
enum SettingsFile {
    User,                              // Uses all settings.
    Local((WorktreeId, Arc<RelPath>)), // Has a special name, and special set of settings
    Server(&'static str),              // Uses a special name, and the user settings
}

impl SettingsFile {
    fn pages(&self) -> Vec<SettingsPage> {
        match self {
            SettingsFile::User => user_settings_data(),
            SettingsFile::Local(_) => project_settings_data(),
            SettingsFile::Server(_) => user_settings_data(),
        }
    }

    fn name(&self) -> SharedString {
        match self {
            SettingsFile::User => SharedString::new_static("User"),
            // TODO is PathStyle::local() ever not appropriate?
            SettingsFile::Local((_, path)) => {
                format!("Local ({})", path.display(PathStyle::local())).into()
            }
            SettingsFile::Server(file) => format!("Server ({})", file).into(),
        }
    }
}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_file = SettingsFile::User;
        let search = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search settings…", window, cx);
            editor
        });
        let mut this = Self {
            files: vec![],
            current_file: current_file,
            pages: vec![],
            navbar_entries: vec![],
            navbar_entry: 0,
            list_handle: UniformListScrollHandle::default(),
            search,
        };
        cx.observe_global_in::<SettingsStore>(window, move |this, _, cx| {
            this.fetch_files(cx);
            cx.notify();
        })
        .detach();
        this.fetch_files(cx);

        this.build_ui(cx);
        this
    }

    fn toggle_navbar_entry(&mut self, ix: usize) {
        if self.navbar_entries[ix].is_root {
            let expanded = &mut self.page_for_navbar_index(ix).expanded;
            *expanded = !*expanded;
            let current_page_index = self.page_index_from_navbar_index(self.navbar_entry);
            // if currently selected page is a child of the parent page we are folding,
            // set the current page to the parent page
            if current_page_index == ix {
                self.navbar_entry = ix;
            }
            self.build_navbar();
        }
    }

    fn build_navbar(&mut self) {
        self.navbar_entries = self
            .pages
            .iter()
            .flat_map(|page| {
                std::iter::once(NavBarEntry {
                    title: page.title,
                    is_root: true,
                })
                .chain(
                    page.expanded
                        .then(|| {
                            page.section_headers().map(|h| NavBarEntry {
                                title: h,
                                is_root: false,
                            })
                        })
                        .into_iter()
                        .flatten(),
                )
            })
            .collect();
    }

    fn build_ui(&mut self, cx: &mut Context<SettingsWindow>) {
        self.pages = self.current_file.pages();
        self.build_navbar();

        cx.notify();
    }

    fn fetch_files(&mut self, cx: &mut Context<SettingsWindow>) {
        let settings_store = cx.global::<SettingsStore>();
        let mut ui_files = vec![];
        let all_files = settings_store.get_all_files();
        for file in all_files {
            let settings_ui_file = match file {
                settings::SettingsFile::User => SettingsFile::User,
                settings::SettingsFile::Global => continue,
                settings::SettingsFile::Extension => continue,
                settings::SettingsFile::Server => SettingsFile::Server("todo: server name"),
                settings::SettingsFile::Default => continue,
                settings::SettingsFile::Local(location) => SettingsFile::Local(location),
            };
            ui_files.push(settings_ui_file);
        }
        ui_files.reverse();
        if !ui_files.contains(&self.current_file) {
            self.change_file(0, cx);
        }
        self.files = ui_files;
    }

    fn change_file(&mut self, ix: usize, cx: &mut Context<SettingsWindow>) {
        if ix >= self.files.len() {
            self.current_file = SettingsFile::User;
            return;
        }
        if self.files[ix] == self.current_file {
            return;
        }
        self.current_file = self.files[ix].clone();
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
            .child(self.search.clone())
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

                                h_flex()
                                    .id(("settings-ui-section", ix))
                                    .w_full()
                                    .pl_2p5()
                                    .py_0p5()
                                    .rounded_sm()
                                    .border_1()
                                    .border_color(cx.theme().colors().border_transparent)
                                    .text_color(cx.theme().colors().text_muted)
                                    .when(this.is_navbar_entry_selected(ix), |this| {
                                        this.text_color(cx.theme().colors().text)
                                            .bg(cx.theme().colors().element_selected.opacity(0.2))
                                            .border_color(cx.theme().colors().border)
                                    })
                                    .child(
                                        ListItem::new(("settings-ui-navbar-entry", ix))
                                            .selectable(true)
                                            .inset(true)
                                            .indent_step_size(px(1.))
                                            .indent_level(if entry.is_root { 1 } else { 3 })
                                            .when(entry.is_root, |item| {
                                                item.toggle(
                                                    this.pages
                                                        [this.page_index_from_navbar_index(ix)]
                                                    .expanded,
                                                )
                                                .always_show_disclosure_icon(true)
                                                .on_toggle(cx.listener(move |this, _, _, cx| {
                                                    this.toggle_navbar_entry(ix);
                                                    cx.notify();
                                                }))
                                            })
                                            .child(
                                                h_flex()
                                                    .text_ui(cx)
                                                    .truncate()
                                                    .hover(|s| {
                                                        s.bg(cx.theme().colors().element_hover)
                                                    })
                                                    .child(entry.title),
                                            ),
                                    )
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.navbar_entry = ix;
                                        cx.notify();
                                    }))
                            })
                            .collect()
                    }),
                )
                .track_scroll(self.list_handle.clone())
                .size_full()
                .flex_grow(),
            )
    }

    fn render_page(
        &self,
        page: &SettingsPage,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> Div {
        v_flex().gap_4().children(
            page.items
                .iter()
                .map(|item| item.render(self.current_file.clone(), window, cx)),
        )
    }

    fn current_page(&self) -> &SettingsPage {
        &self.pages[self.page_index_from_navbar_index(self.navbar_entry)]
    }

    fn page_index_from_navbar_index(&self, index: usize) -> usize {
        self.navbar_entries
            .iter()
            .take(index + 1)
            .map(|entry| entry.is_root as usize)
            .sum::<usize>()
            - 1
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
        div()
            .flex()
            .flex_row()
            .size_full()
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
                    .child(self.render_page(self.current_page(), window, cx)),
            )
    }
}

fn render_text_field(
    field: SettingField<Option<String>>,
    metadata: Option<&SettingsFieldMetadata>,
    cx: &mut App,
) -> AnyElement {
    // TODO: in settings window state
    let store = SettingsStore::global(cx);

    // TODO: This clone needs to go!!
    let defaults = store.raw_default_settings().clone();
    let user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    let initial_text = (field.pick)(&user_settings)
        .clone()
        .or_else(|| (field.pick)(&defaults).clone());

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
    field: SettingField<Option<B>>,
    cx: &mut App,
) -> AnyElement {
    // TODO: in settings window state
    let store = SettingsStore::global(cx);

    // TODO: This clone needs to go!!
    let defaults = store.raw_default_settings().clone();
    let user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    let toggle_state = if (field.pick)(&user_settings)
        .unwrap_or_else(|| (field.pick)(&defaults).unwrap())
        .into()
    {
        ui::ToggleState::Selected
    } else {
        ui::ToggleState::Unselected
    };

    Switch::new("toggle_button", toggle_state)
        .on_click({
            move |state, _window, cx| {
                let state = *state == ui::ToggleState::Selected;
                let field = field.clone();
                cx.update_global(move |store: &mut SettingsStore, cx| {
                    store.update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _cx| {
                        *(field.pick_mut)(settings) = Some(state.into());
                    });
                });
            }
        })
        .into_any_element()
}

fn render_dropdown<T>(
    field: SettingField<Option<T>>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + 'static,
{
    let variants = || -> &'static [T] { <T as strum::VariantArray>::VARIANTS };
    let labels = || -> &'static [&'static str] { <T as strum::VariantNames>::VARIANTS };

    let store = SettingsStore::global(cx);
    let defaults = store.raw_default_settings().clone();
    let user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    let current_value =
        (field.pick)(&user_settings).unwrap_or_else(|| (field.pick)(&defaults).unwrap());
    let current_value_label =
        labels()[variants().iter().position(|v| *v == current_value).unwrap()];

    DropdownMenu::new(
        "dropdown",
        current_value_label,
        ui::ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (value, label) in variants()
                .into_iter()
                .copied()
                .zip(labels().into_iter().copied())
            {
                menu = menu.toggleable_entry(
                    label,
                    value == current_value,
                    ui::IconPosition::Start,
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

        for (ix, mut line) in input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            if line.ends_with("*") {
                assert!(
                    selected_idx.is_none(),
                    "Can only have one selected navbar entry at a time"
                );
                selected_idx = Some(ix);
                line = &line[..line.len() - 1];
            }

            if line.starts_with("v") || line.starts_with(">") {
                if let Some(current_page) = current_page.take() {
                    pages.push(current_page);
                }

                let expanded = line.starts_with("v");

                current_page = Some(SettingsPage {
                    title: line.split_once(" ").unwrap().1,
                    expanded,
                    items: Vec::default(),
                });
            } else if line.starts_with("-") {
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

        let mut settings_window = SettingsWindow {
            files: Vec::default(),
            current_file: crate::SettingsFile::User,
            pages,
            search: cx.new(|cx| Editor::single_line(window, cx)),
            navbar_entry: selected_idx.unwrap(),
            navbar_entries: Vec::default(),
            list_handle: UniformListScrollHandle::default(),
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
        basic_open,
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
        basic_close,
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
        basic_second_root_entry_close,
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
}
