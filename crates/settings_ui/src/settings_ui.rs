//! # settings_ui
use std::rc::Rc;

use feature_flags::FeatureFlag;
use gpui::{
    App, AppContext as _, Context, Div, IntoElement, ReadGlobal, Render, Window, WindowHandle,
    actions, div, px, size,
};
use settings::{SettingsContent, SettingsStore};
use ui::{
    AnyElement, BorrowAppContext, Color, FluentBuilder as _, InteractiveElement as _, Label,
    LabelCommon as _, LabelSize, ParentElement, SharedString, Styled, Switch, SwitchField, v_flex,
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
    fn render(&self, _window: &mut Window, cx: &mut App) -> AnyElement {
        match self {
            SettingsPageItem::SectionHeader(header) => Label::new(SharedString::new_static(header))
                .size(LabelSize::Large)
                .into_any_element(),
            SettingsPageItem::SettingItem(setting_item) => div()
                .child(setting_item.title)
                .child(setting_item.description)
                .child((setting_item.render)(cx))
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
    render: std::rc::Rc<dyn Fn(&mut App) -> AnyElement>,
}

enum SettingsFile {
    User,                  // Uses all settings.
    Project(&'static str), // Has a special name, and special set of settings
    Remote(&'static str),  // Uses a special name, and the user settings
}

impl SettingsFile {
    fn pages(&self) -> Vec<SettingsPage> {
        match self {
            SettingsFile::User => user_settings_data(),
            SettingsFile::Project(_) => project_settings_data(),
            SettingsFile::Remote(_) => user_settings_data(),
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
                    render: Rc::new(|cx|

                        render_toggle_button("confirm_quit", cx, |settings_content| {
                            settings_content.workspace.confirm_quit.as_mut()
                        })),
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Auto Update",
                    description: "Automatically update Zed (may be ignored on Linux if installed through a package manager)",
                    render: Rc::new(|cx| render_toggle_button("Auto Update", cx, |settings_content| {
                        settings_content.auto_update.as_mut()
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
                    render: Rc::new(|_| {


                        todo!()}),
                }),
            ],
        },
    ].iter().cloned().collect()
}

// 0. Make this turn on and look ok-ish
// 1. Do text input for the worktree settings content (might need to stash an editor in a use_state near the page)
// 2. Let's introduce settings files and settings source

fn project_settings_data() -> Vec<SettingsPage> {
    vec![SettingsPage {
        title: "Project",
        items: vec![
            SettingsPageItem::SectionHeader("Worktree Settings Content"),
            SettingsPageItem::SettingItem(SettingItem {
                title: "Project Name",
                description: " The displayed name of this project. If not set, the root directory name",
                render: Rc::new(|_| todo!()),
            }),
        ],
    }]
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

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_file = SettingsFile::User;
        let mut this = Self {
            files: vec![SettingsFile::User, SettingsFile::Project("zed.dev")],
            current_file: current_file,
            pages: vec![],
            current_page: 0,
        };

        this.build_ui();
        this
    }

    fn build_ui(&mut self) {
        self.pages = self.current_file.pages();
    }

    fn render_files(&self, _window: &mut Window, _cx: &mut Context<SettingsWindow>) -> Div {
        todo!()
    }

    fn render_nav(&self, _window: &mut Window, _cx: &mut Context<SettingsWindow>) -> Div {
        let mut nav = v_flex().p_4().gap_2();
        for (ix, page) in self.pages.iter().enumerate() {
            nav = nav.child(
                div().id(page.title).child(
                    Label::new(page.title)
                        .size(LabelSize::Large)
                        .when(self.is_page_selected(ix), |this| {
                            this.color(Color::Selected)
                        }),
                ),
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


fn render_toggle_button(id: &'static str, cx: &mut App, get_value: fn(&mut SettingsContent) -> Option<&mut bool>) -> AnyElement {
    // todo! in settings window state
    let store = SettingsStore::global(cx);

    // This clone needs to go!!
    let mut defaults = store.raw_default_settings().clone();
    let mut user_settings = store
        .raw_user_settings()
        .cloned()
        .unwrap_or_default()
        .content;

    // TODO: Move this defaulting logic into a "Sources" concept
    let toggle_state =
        if *get_value(&mut user_settings).unwrap_or_else(|| get_value(&mut defaults).unwrap()) {
            ui::ToggleState::Selected
        } else {
            ui::ToggleState::Unselected
        };

    Switch::new(id, toggle_state)
        .on_click({
            move |state, window, cx| {
                    cx.update_global(|store: &mut SettingsStore, cx| {
                        // source.update_file(cx, |settings| {
                        //   *get_value(settings).unwrap() = *state == ui::ToggleState::Selected;
                        // })

                        // TODO: Make seperate
                        store.update_user_settings(cx, |settings| {
                            *get_value(settings).unwrap() = *state == ui::ToggleState::Selected;
                        });
                    })
                },
        }).into_any_element()
}
