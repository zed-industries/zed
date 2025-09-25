//! # settings_ui
use std::{ops::Range, rc::Rc, sync::Arc};

use editor::Editor;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    App, AppContext as _, Context, Div, Entity, IntoElement, ReadGlobal as _, Render, UniformList,
    UniformListScrollHandle, Window, WindowHandle, WindowOptions, actions, div, px, size,
    uniform_list,
};
use project::WorktreeId;
use settings::{SettingsContent, SettingsStore};
use std::path::Path;
use ui::{
    ActiveTheme as _, AnyElement, BorrowAppContext as _, Button, Clickable as _, Color,
    FluentBuilder as _, Icon, IconName, InteractiveElement as _, Label, LabelCommon as _,
    LabelSize, ListItem, ParentElement, SharedString, StatefulInteractiveElement as _, Styled,
    StyledTypography, Switch, Toggleable, h_flex, v_flex,
};

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
                    render: Rc::new(|_, cx| {
                        render_toggle_button(
                            "confirm_quit",
                            SettingsFile::User,
                            cx,
                            |settings_content| &mut settings_content.workspace.confirm_quit,
                        )
                    }),
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Update",
                    description: "Automatically update Zed (may be ignored on Linux if installed through a package manager)",
                    render: Rc::new(|_, cx| {
                        render_toggle_button(
                            "Auto Update",
                            SettingsFile::User,
                            cx,
                            |settings_content| &mut settings_content.auto_update,
                        )
                    }),
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
                    render: Rc::new(|window, cx| {
                        render_text_field(
                            "project_name",
                            SettingsFile::User,
                            window,
                            cx,
                            |settings_content| &mut settings_content.project.worktree.project_name,
                        )
                    }),
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
                render: Rc::new(|window, cx| {
                    render_text_field(
                        "project_name",
                        SettingsFile::Local((
                            WorktreeId::from_usize(0),
                            Arc::from(Path::new("TODO: actually pass through file")),
                        )),
                        window,
                        cx,
                        |settings_content| &mut settings_content.project.worktree.project_name,
                    )
                }),
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
    current_page: usize, // Index into pages - should probably be (usize, Option<usize>) for section + page
    navbar_entries: Vec<NavBarEntry>,
    list_handle: UniformListScrollHandle,
}

#[derive(Debug)]
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
    fn render(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        match self {
            SettingsPageItem::SectionHeader(header) => Label::new(SharedString::new_static(header))
                .size(LabelSize::Large)
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => div()
                .child(setting_item.title)
                .child(setting_item.description)
                .child((setting_item.render)(window, cx))
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
    render: std::rc::Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
}

#[allow(unused)]
#[derive(Clone)]
enum SettingsFile {
    User,                           // Uses all settings.
    Local((WorktreeId, Arc<Path>)), // Has a special name, and special set of settings
    Server(&'static str),           // Uses a special name, and the user settings
}

impl SettingsFile {
    fn pages(&self) -> Vec<SettingsPage> {
        match self {
            SettingsFile::User => user_settings_data(),
            SettingsFile::Local(_) => project_settings_data(),
            SettingsFile::Server(_) => user_settings_data(),
        }
    }

    fn name(&self) -> String {
        match self {
            SettingsFile::User => "User".to_string(),
            SettingsFile::Local((_, path)) => format!("Local ({})", path.display()),
            SettingsFile::Server(file) => format!("Server ({})", file),
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
            files: vec![
                SettingsFile::User,
                SettingsFile::Local((
                    WorktreeId::from_usize(0),
                    Arc::from(Path::new("/my-project/")),
                )),
            ],
            current_file: current_file,
            pages: vec![],
            navbar_entries: vec![],
            current_page: 0,
            list_handle: UniformListScrollHandle::default(),
            search,
        };
        cx.observe_global_in::<SettingsStore>(window, move |_, _, cx| {
            cx.notify();
        })
        .detach();

        this.build_ui(cx);
        this
    }

    fn toggle_navbar_entry(&mut self, ix: usize, cx: &mut Context<SettingsWindow>) {
        if self.navbar_entries[ix].is_root {
            let expanded = &mut self.page_for_navbar_index(ix).expanded;
            *expanded = !*expanded;
            let current_page_index = self.page_index_from_navbar_index(self.current_page);
            // if currently selected page is a child of the parent page we are folding,
            // set the current page to the parent page
            if current_page_index == ix {
                self.current_page = ix;
            }
            self.build_navbar(cx);
        }
    }

    fn build_navbar(&mut self, cx: &mut Context<SettingsWindow>) {
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

        cx.notify();
    }

    fn build_ui(&mut self, cx: &mut Context<SettingsWindow>) {
        self.pages = self.current_file.pages();
        self.build_navbar(cx);

        cx.notify();
    }

    fn change_file(&mut self, ix: usize, cx: &mut Context<SettingsWindow>) {
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
        div()
            .child(Icon::new(IconName::MagnifyingGlass))
            .child(self.search.clone())
    }

    fn render_nav(&self, window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        v_flex()
            .bg(cx.theme().colors().panel_background)
            .child(self.render_search(window, cx))
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
                                                    this.toggle_navbar_entry(ix, cx);
                                                }))
                                            })
                                            .child(
                                                div().text_ui(cx).w_full().child(entry.title).when(
                                                    this.is_page_selected(ix),
                                                    |this| {
                                                        this.text_color(Color::Selected.color(cx))
                                                    },
                                                ),
                                            ),
                                    )
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.current_page = ix;
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
        div()
            .child(self.render_files(window, cx))
            .child(Label::new(page.title))
            .children(page.items.iter().map(|item| item.render(window, cx)))
    }

    fn current_page(&self) -> &SettingsPage {
        &self.pages[self.page_index_from_navbar_index(self.current_page)]
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

    fn is_page_selected(&self, ix: usize) -> bool {
        ix == self.current_page
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
            .child(self.render_page(self.current_page(), window, cx).w_full())
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

fn render_toggle_button(
    id: &'static str,
    _: SettingsFile,
    cx: &mut App,
    get_value: fn(&mut SettingsContent) -> &mut Option<bool>,
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

    let toggle_state =
        if get_value(&mut user_settings).unwrap_or_else(|| get_value(&mut defaults).unwrap()) {
            ui::ToggleState::Selected
        } else {
            ui::ToggleState::Unselected
        };

    Switch::new(id, toggle_state)
        .on_click({
            move |state, _window, cx| {
                write_setting_value(get_value, Some(*state == ui::ToggleState::Selected), cx);
            }
        })
        .into_any_element()
}
