//! # settings_ui
use std::{ops::Range, sync::Arc};

use editor::Editor;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    App, AppContext as _, Context, Div, Entity, IntoElement, ReadGlobal as _, Render,
    UniformListScrollHandle, Window, WindowHandle, WindowOptions, actions, div, px, size,
    uniform_list,
};
use project::WorktreeId;
use settings::{SettingsContent, SettingsStore};
use ui::{
    ActiveTheme as _, AnyElement, BorrowAppContext as _, Button, Clickable as _, Color, Divider,
    DropdownMenu, FluentBuilder as _, Icon, IconName, InteractiveElement as _, Label,
    LabelCommon as _, LabelSize, ListItem, ParentElement, SharedString,
    StatefulInteractiveElement as _, Styled, StyledTypography, Switch, h_flex, v_flex,
};
use util::{paths::PathStyle, rel_path::RelPath};

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
                    render: |file, _, cx| {
                        render_toggle_button("confirm_quit", file, cx, |settings_content| {
                            &mut settings_content.workspace.confirm_quit
                        })
                    },
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Update",
                    description: "Automatically update Zed (may be ignored on Linux if installed through a package manager)",
                    render: |file, _, cx| {
                        render_toggle_button("Auto Update", file, cx, |settings_content| {
                            &mut settings_content.auto_update
                        })
                    },
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
                    render: |file, window, cx| {
                        render_text_field("project_name", file, window, cx, |settings_content| {
                            &mut settings_content.project.worktree.project_name
                        })
                    },
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
                    render: |file, _, cx| {
                        render_toggle_button("disable_AI", file, cx, |settings_content| {
                            &mut settings_content.disable_ai
                        })
                    },
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
                    render: |file, window, cx| {
                        render_dropdown::<settings::CursorShape>(
                            "cursor_shape",
                            file,
                            window,
                            cx,
                            |settings_content| &mut settings_content.editor.cursor_shape,
                        )
                    },
                }),
            ],
        },
    ]
}

fn project_settings_data() -> Vec<SettingsPage> {
    vec![SettingsPage {
        title: "Project",
        expanded: true,
        items: vec![
            SettingsPageItem::SectionHeader("Worktree Settings Content"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Project Name",
                description: " The displayed name of this project. If not set, the root directory name",
                render: |file, window, cx| {
                    render_text_field("project_name", file, window, cx, |settings_content| {
                        &mut settings_content.project.worktree.project_name
                    })
                },
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

pub fn open_settings_editor(cx: &mut App) -> anyhow::Result<WindowHandle<SettingsWindow>> {
    cx.open_window(
        WindowOptions {
            titlebar: None,
            focus: true,
            show: true,
            kind: gpui::WindowKind::Normal,
            window_min_size: Some(size(px(300.), px(500.))), // todo(settings_ui): Does this min_size make sense?
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

#[derive(Clone)]
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

#[derive(Clone)]
enum SettingsPageItem {
    SectionHeader(&'static str),
    SettingItem(SettingItem),
}

impl SettingsPageItem {
    fn render(&self, file: SettingsFile, window: &mut Window, cx: &mut App) -> AnyElement {
        match self {
            SettingsPageItem::SectionHeader(header) => div()
                .w_full()
                .child(Label::new(SharedString::new_static(header)).size(LabelSize::Large))
                .child(Divider::horizontal().color(ui::DividerColor::BorderVariant))
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => div()
                .child(
                    Label::new(SharedString::new_static(setting_item.title))
                        .size(LabelSize::Default),
                )
                .child(
                    h_flex()
                        .justify_between()
                        .child(
                            div()
                                .child(
                                    Label::new(SharedString::new_static(setting_item.description))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .max_w_1_2(),
                        )
                        .child((setting_item.render)(file, window, cx)),
                )
                .into_any_element(),
        }
    }
}

impl SettingsPageItem {
    fn _header(&self) -> Option<&'static str> {
        match self {
            SettingsPageItem::SectionHeader(header) => Some(header),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct SettingItem {
    title: &'static str,
    description: &'static str,
    render: fn(file: SettingsFile, &mut Window, &mut App) -> AnyElement,
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
            editor.set_placeholder_text("Search Settings", window, cx);
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
        div()
            .flex()
            .flex_row()
            .gap_1()
            .children(self.files.iter().enumerate().map(|(ix, file)| {
                Button::new(ix, file.name())
                    .on_click(cx.listener(move |this, _, _window, cx| this.change_file(ix, cx)))
            }))
    }

    fn render_search(&self, _window: &mut Window, _cx: &mut App) -> Div {
        h_flex()
            .child(Icon::new(IconName::MagnifyingGlass))
            .child(self.search.clone())
    }

    fn render_nav(&self, window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        v_flex()
            .bg(cx.theme().colors().panel_background)
            .p_3()
            .child(div().h_10()) // Files spacer;
            .child(self.render_search(window, cx).pb_1())
            .gap_3()
            .child(
                uniform_list(
                    "settings-ui-nav-bar",
                    self.navbar_entries.len(),
                    cx.processor(|this, range: Range<usize>, _, cx| {
                        range
                            .into_iter()
                            .map(|ix| {
                                let entry = &this.navbar_entries[ix];

                                div()
                                    .id(("settings-ui-section", ix))
                                    .child(
                                        ListItem::new(("settings-ui-navbar-entry", ix))
                                            .selectable(true)
                                            .indent_step_size(px(10.))
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
                                                div()
                                                    .text_ui(cx)
                                                    .size_full()
                                                    .child(entry.title)
                                                    .hover(|style| {
                                                        style.bg(cx.theme().colors().element_hover)
                                                    })
                                                    .when(!entry.is_root, |this| {
                                                        this.text_color(
                                                            cx.theme().colors().text_muted,
                                                        )
                                                    })
                                                    .when(
                                                        this.is_navbar_entry_selected(ix),
                                                        |this| {
                                                            this.text_color(
                                                                Color::Selected.color(cx),
                                                            )
                                                        },
                                                    ),
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
                .gap_1_5()
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
        v_flex().gap_4().py_4().children(
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
            .size_full()
            .bg(cx.theme().colors().background)
            .flex()
            .flex_row()
            .text_color(cx.theme().colors().text)
            .child(self.render_nav(window, cx).w(px(300.0)))
            .child(Divider::vertical().color(ui::DividerColor::BorderVariant))
            .child(
                v_flex()
                    .bg(cx.theme().colors().editor_background)
                    .px_6()
                    .py_2()
                    .child(self.render_files(window, cx))
                    .child(self.render_page(self.current_page(), window, cx))
                    .w_full(),
            )
    }
}

fn write_setting_value<T: Send + 'static>(
    get_value: fn(&mut SettingsContent) -> &mut Option<T>,
    value: Option<T>,
    cx: &mut App,
) {
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_settings_file(<dyn fs::Fs>::global(cx), move |settings, _cx| {
            *get_value(settings) = value;
        });
    });
}

fn render_text_field(
    id: &'static str,
    _file: SettingsFile,
    window: &mut Window,
    cx: &mut App,
    get_value: fn(&mut SettingsContent) -> &mut Option<String>,
) -> AnyElement {
    // TODO: Updating file does not cause the editor text to reload, suspicious it may be a missing global update/notify in SettingsStore

    // TODO: in settings window state
    let store = SettingsStore::global(cx);

    // TODO: This clone needs to go!!
    let mut defaults = store.raw_default_settings().clone();
    let mut user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    // TODO: unwrap_or_default here because project name is null
    let initial_text = get_value(user_settings.as_mut())
        .clone()
        .unwrap_or_else(|| get_value(&mut defaults).clone().unwrap_or_default());

    let editor = window.use_keyed_state((id.into(), initial_text.clone()), cx, {
        move |window, cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(initial_text, window, cx);
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
                let Some(editor) = weak_editor.upgrade() else {
                    return;
                };
                let new_value = editor.read_with(cx, |editor, cx| editor.text(cx));
                let new_value = (!new_value.is_empty()).then_some(new_value);
                write_setting_value(get_value, new_value, cx);
                editor.update(cx, |_, cx| {
                    cx.notify();
                });
            }
        })
        .into_any_element()
}

fn render_toggle_button<B: Into<bool> + From<bool> + Copy + Send + 'static>(
    id: &'static str,
    _: SettingsFile,
    cx: &mut App,
    get_value: fn(&mut SettingsContent) -> &mut Option<B>,
) -> AnyElement {
    // TODO: in settings window state
    let store = SettingsStore::global(cx);

    // TODO: This clone needs to go!!
    let mut defaults = store.raw_default_settings().clone();
    let mut user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    let toggle_state = if get_value(&mut user_settings)
        .unwrap_or_else(|| get_value(&mut defaults).unwrap())
        .into()
    {
        ui::ToggleState::Selected
    } else {
        ui::ToggleState::Unselected
    };

    Switch::new(id, toggle_state)
        .on_click({
            move |state, _window, cx| {
                write_setting_value(
                    get_value,
                    Some((*state == ui::ToggleState::Selected).into()),
                    cx,
                );
            }
        })
        .into_any_element()
}

fn render_dropdown<T>(
    id: &'static str,
    _: SettingsFile,
    window: &mut Window,
    cx: &mut App,
    get_value: fn(&mut SettingsContent) -> &mut Option<T>,
) -> AnyElement
where
    T: strum::VariantArray + strum::VariantNames + Copy + PartialEq + Send + 'static,
{
    let variants = || -> &'static [T] { <T as strum::VariantArray>::VARIANTS };
    let labels = || -> &'static [&'static str] { <T as strum::VariantNames>::VARIANTS };

    let store = SettingsStore::global(cx);
    let mut defaults = store.raw_default_settings().clone();
    let mut user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    let current_value =
        get_value(&mut user_settings).unwrap_or_else(|| get_value(&mut defaults).unwrap());
    let current_value_label =
        labels()[variants().iter().position(|v| *v == current_value).unwrap()];

    DropdownMenu::new(
        id,
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
                        write_setting_value(get_value, Some(value), cx);
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
