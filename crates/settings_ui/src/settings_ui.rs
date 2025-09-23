//! # settings_ui
use std::{rc::Rc, sync::Arc};

use editor::Editor;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    App, AppContext as _, Context, Div, IntoElement, ReadGlobal, Render, Window, WindowHandle,
    actions, div, px, size,
};
use project::WorktreeId;
use settings::{SettingsContent, SettingsStore};
use std::path::Path;
use ui::{
    ActiveTheme as _, AnyElement, BorrowAppContext as _, Color, FluentBuilder as _,
    InteractiveElement as _, Label, LabelCommon as _, LabelSize, ParentElement, SharedString,
    StatefulInteractiveElement as _, Styled, Switch, v_flex,
};

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
        gpui::WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Zed Settings".into()),
                ..Default::default()
            }),
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
    current_page: usize, // Index into pages - should probably be (usize, Option<usize>) for section + page
}

#[derive(Clone)]
struct SettingsPage {
    title: &'static str,
    items: Vec<SettingsPageItem>,
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
    fn header(&self) -> Option<&'static str> {
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
}

impl Into<settings::SettingsFile> for SettingsFile {
    fn into(self) -> settings::SettingsFile {
        match self {
            SettingsFile::User => settings::SettingsFile::User,
            SettingsFile::Local(location) => settings::SettingsFile::Local(location),
            SettingsFile::Server(_ /*TODO*/) => settings::SettingsFile::Server,
        }
    }
}

fn user_settings_data() -> Vec<SettingsPage> {
    [
        SettingsPage {
            title: "General",
            items: vec![
                SettingsPageItem::SectionHeader("General"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Confirm Quit",
                    description: "Whether to confirm before quitting Zed",
                    render: Rc::new(|_, cx|
                        render_toggle_button("confirm_quit", SettingsFile::User, cx, |settings_content| {
                            &mut settings_content.workspace.confirm_quit
                        })),
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Update",
                    description: "Automatically update Zed (may be ignored on Linux if installed through a package manager)",
                    render: Rc::new(|_, cx| render_toggle_button("Auto Update", SettingsFile::User, cx, |settings_content| {
                        &mut settings_content.auto_update
                    })),
                }),
            ],
        },
        SettingsPage {
            title: "Project",
            items: vec![
                SettingsPageItem::SectionHeader("Worktree Settings Content"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Project Name",
                    description: "The displayed name of this project. If not set, the root directory name",
                    render: Rc::new(| window, cx| {                        render_text_field("project_name", window, cx, |settings_content| {
                            &mut settings_content.project.worktree.project_name
                        })
                    }),
                }),
            ],
        },
    ].iter().cloned().collect()
}

fn project_settings_data() -> Vec<SettingsPage> {
    vec![SettingsPage {
        title: "Project",
        items: vec![
            SettingsPageItem::SectionHeader("Worktree Settings Content"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Project Name",
                description: " The displayed name of this project. If not set, the root directory name",
                render: Rc::new(|_, _| todo!()),
            }),
        ],
    }]
}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_file = SettingsFile::User;
        let mut this = Self {
            files: vec![SettingsFile::User],
            current_file: current_file,
            pages: vec![],
            current_page: 0,
        };
        cx.observe_global_in::<SettingsStore>(window, move |_, _, cx| {
            cx.notify();
        })
        .detach();

        this.build_ui();
        this
    }

    fn build_ui(&mut self) {
        self.pages = self.current_file.pages();
    }

    fn render_files(&self, _window: &mut Window, _cx: &mut Context<SettingsWindow>) -> Div {
        todo!()
    }

    fn render_nav(&self, _window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        let mut nav = v_flex().p_4().gap_2();
        for (ix, page) in self.pages.iter().enumerate() {
            nav = nav.child(
                div()
                    .id(page.title)
                    .child(
                        Label::new(page.title)
                            .size(LabelSize::Large)
                            .when(self.is_page_selected(ix), |this| {
                                this.color(Color::Selected)
                            }),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.current_page = ix;
                        cx.notify();
                    })),
            );
        }
        nav
    }

    fn render_page(
        &self,
        page: &SettingsPage,
        window: &mut Window,
        cx: &mut Context<SettingsWindow>,
    ) -> Div {
        div()
            .child(Label::new(page.title))
            .children(page.items.iter().map(|item| item.render(window, cx)))
    }

    fn current_page(&self) -> &SettingsPage {
        &self.pages[self.current_page]
    }

    fn is_page_selected(&self, ix: usize) -> bool {
        ix == self.current_page
    }
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = self.render_nav(window, cx);
        let page = self.render_page(self.current_page(), window, cx);
        div()
            .flex()
            .flex_row()
            .child(sidebar.w(px(300.0)))
            .child(page.w_full())
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
    window: &mut Window,
    cx: &mut App,
    get_value: fn(&mut SettingsContent) -> &mut Option<String>,
) -> AnyElement {
    // TODO: Updating file does not cause the editor text to reload, suspicious it may be a missing global update/notify in SettingsStore
    let store = SettingsStore::global(cx);
    // let initial_text = store
    //     .get_value_from_file_mut(settings::SettingsFile, get_value)
    //     .unwrap_or_default();
    let mut defaults = store.raw_default_settings().clone();
    let mut user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    // TODO: Move this defaulting logic into a "Sources" concept
    let initial_text = get_value(user_settings.as_mut())
        .clone()
        // TODO: unwrap_or_default here because project name is null
        .unwrap_or_else(|| get_value(&mut defaults).clone().unwrap_or_default());

    let editor = window.use_keyed_state((id.into(), initial_text.clone()), cx, {
        move |window, cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_text(initial_text, window, cx);

            // TODO: is this redundant with the same logic in the SettingsWindow::new function?
            cx.observe_global_in::<SettingsStore>(window, move |editor, window, cx| cx.notify())
                .detach();

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
                dbg!("In project name confirm closure");
                let Some(editor) = weak_editor.upgrade() else {
                    // sanity check
                    dbg!("Failed to upgrade weak_editor (this shouldn't happen)");
                    return;
                };
                let new_value = editor.read_with(cx, |editor, cx| editor.text(cx));
                dbg!("Writing project name value");
                let new_value = (!new_value.is_empty()).then_some(new_value);
                dbg!(&new_value);
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
    file: SettingsFile,
    cx: &mut App,
    get_value: fn(&mut SettingsContent) -> &mut Option<bool>,
) -> AnyElement {
    let toggle_state = if cx.update_global(|store: &mut SettingsStore, _| {
        store
            .get_value_from_file_mut(file.into(), get_value)
            .unwrap_or_default()
    }) {
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
