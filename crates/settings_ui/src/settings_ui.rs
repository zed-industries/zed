//! # settings_ui
use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::Arc,
};

use editor::Editor;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    App, AppContext as _, Context, Div, Entity, Global, IntoElement, ReadGlobal as _, Render,
    Window, WindowHandle, WindowOptions, actions, div, px, size,
};
use project::WorktreeId;
use settings::{SettingsContent, SettingsStore};
use ui::{
    ActiveTheme as _, AnyElement, BorrowAppContext as _, Button, Clickable as _, Color,
    FluentBuilder as _, Icon, IconName, InteractiveElement as _, Label, LabelCommon as _,
    LabelSize, ParentElement, SharedString, StatefulInteractiveElement as _, Styled, Switch,
    v_flex,
};
use util::{paths::PathStyle, rel_path::RelPath};

#[derive(Clone)]
struct SettingField<T: 'static> {
    pick: Arc<dyn Send + Fn(&SettingsContent) -> &T>,
    pick_mut: Arc<dyn Send + Sync + Fn(&mut SettingsContent) -> &mut T>,
}

#[derive(Default, Clone)]
struct SettingFieldRenderer {
    renderers: Rc<RefCell<HashMap<TypeId, Box<dyn Fn(&dyn Any, &mut App) -> AnyElement>>>>,
}

impl Global for SettingFieldRenderer {}

impl SettingFieldRenderer {
    fn add_renderer<T: 'static>(
        &mut self,
        renderer: impl Fn(&SettingField<T>, &mut App) -> AnyElement + 'static,
    ) {
        let type_id = TypeId::of::<SettingField<T>>();
        let renderer = Box::new(move |any_setting_field: &dyn Any, cx: &mut App| {
            let field = any_setting_field.downcast_ref::<SettingField<T>>().unwrap();
            renderer(field, cx)
        });
        self.renderers.borrow_mut().insert(type_id, renderer);
    }

    fn render(&self, any_setting_field: &dyn Any, cx: &mut App) -> AnyElement {
        let type_id = any_setting_field.type_id();
        if let Some(renderer) = self.renderers.borrow().get(&type_id) {
            renderer(any_setting_field, cx)
        } else {
            panic!("No renderer found for type")
        }
    }
}

fn user_settings_data() -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "General Page",
            items: vec![
                SettingsPageItem::SectionHeader("General Section"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Confirm Quit",
                    description: "Whether to confirm before quitting Zed",
                    field: Box::new(SettingField {
                        pick: Arc::new(|settings_content| &settings_content.workspace.confirm_quit),
                        pick_mut: Arc::new(|settings_content| {
                            &mut settings_content.workspace.confirm_quit
                        }),
                    }),
                }),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Auto Update",
                //     description: "Automatically update Zed (may be ignored on Linux if installed through a package manager)",
                //     render: Rc::new(|_, cx| {
                //         render_toggle_button(
                //             "Auto Update",
                //             SettingsFile::User,
                //             cx,
                //             |settings_content| &mut settings_content.auto_update,
                //         )
                //     }),
                // }),
            ],
        },
        SettingsPage {
            title: "Project",
            items: vec![
                SettingsPageItem::SectionHeader("Worktree Settings Content"),
                // SettingsPageItem::SettingItem(SettingItem {
                //     title: "Project Name",
                //     description: "The displayed name of this project. If not set, the root directory name",
                //     render: Rc::new(|window, cx| {
                //         render_text_field(
                //             "project_name",
                //             SettingsFile::User,
                //             window,
                //             cx,
                //             |settings_content| &mut settings_content.project.worktree.project_name,
                //         )
                //     }),
                // }),
            ],
        },
    ]
}

// Derive Macro, on the new ProjectSettings struct

fn project_settings_data() -> Vec<SettingsPage> {
    vec![SettingsPage {
        title: "Project",
        items: vec![
            SettingsPageItem::SectionHeader("Worktree Settings Content"),
            // SettingsPageItem::SettingItem(SettingItem {
            //     title: "Project Name",
            //     description: " The displayed name of this project. If not set, the root directory name",
            //     render: Rc::new(|window, cx| {
            //         render_text_field(
            //             "project_name",
            //             SettingsFile::Local((
            //                 WorktreeId::from_usize(0),
            //                 Arc::from(RelPath::new("TODO: actually pass through file").unwrap()),
            //             )),
            //             window,
            //             cx,
            //             |settings_content| &mut settings_content.project.worktree.project_name,
            //         )
            //     }),
            // }),
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
    cx.default_global::<SettingFieldRenderer>()
        .add_renderer::<bool>(|settings_field, cx| {
            render_toggle_button(SettingsFile::User, settings_field.clone(), cx).into_any_element()
        });

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
}

struct SettingsPage {
    title: &'static str,
    items: Vec<SettingsPageItem>,
}

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
            SettingsPageItem::SettingItem(setting_item) => {
                let renderer = cx.default_global::<SettingFieldRenderer>().clone();
                div()
                    .id(setting_item.title) // This would be a formatted version of the field name,
                    .child(setting_item.title)
                    .child(setting_item.description)
                    .child(renderer.render(setting_item.field.as_ref(), cx))
                    .into_any_element()
            }
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

struct SettingItem {
    title: &'static str,
    description: &'static str,
    field: Box<dyn Any>,
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
            current_page: 0,
            search,
        };
        cx.observe_global_in::<SettingsStore>(window, move |this, _, cx| {
            this.fetch_files(cx);
            cx.notify();
        })
        .detach();
        this.fetch_files(cx);

        this.build_ui();
        this
    }

    fn build_ui(&mut self) {
        self.pages = self.current_file.pages();
    }

    fn fetch_files(&mut self, cx: &mut App) {
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
            self.change_file(0);
        }
        self.files = ui_files;
    }

    fn change_file(&mut self, ix: usize) {
        if ix >= self.files.len() {
            self.current_file = SettingsFile::User;
            return;
        }
        if self.files[ix] == self.current_file {
            return;
        }
        self.current_file = self.files[ix].clone();
        self.build_ui();
    }

    fn render_files(&self, _window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        div()
            .flex()
            .flex_row()
            .gap_1()
            .children(self.files.iter().enumerate().map(|(ix, file)| {
                Button::new(ix, file.name())
                    .on_click(cx.listener(move |this, _, _window, _cx| this.change_file(ix)))
            }))
    }

    fn render_search(&self, _window: &mut Window, _cx: &mut App) -> Div {
        div()
            .child(Icon::new(IconName::MagnifyingGlass))
            .child(self.search.clone())
    }

    fn render_nav(&self, window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
        let mut nav = v_flex()
            .p_4()
            .gap_2()
            .child(div().h_10()) // Files spacer;
            .child(self.render_search(window, cx));

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
            .child(self.render_files(window, cx))
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

// fn render_text_field(
//     id: &'static str,
//     _file: SettingsFile,
//     window: &mut Window,
//     cx: &mut App,
//     get_value: fn(&mut SettingsContent) -> &mut Option<String>,
// ) -> AnyElement {
//     // TODO: Updating file does not cause the editor text to reload, suspicious it may be a missing global update/notify in SettingsStore

//     // TODO: in settings window state
//     let store = SettingsStore::global(cx);

//     // TODO: This clone needs to go!!
//     let mut defaults = store.raw_default_settings().clone();
//     let mut user_settings = store
//         .raw_user_settings()
//         .cloned()
//         .unwrap_or_default()
//         .content;

//     // TODO: unwrap_or_default here because project name is null
//     let initial_text = get_value(user_settings.as_mut())
//         .clone()
//         .unwrap_or_else(|| get_value(&mut defaults).clone().unwrap_or_default());

//     let editor = window.use_keyed_state((id.into(), initial_text.clone()), cx, {
//         move |window, cx| {
//             let mut editor = Editor::single_line(window, cx);
//             editor.set_text(initial_text, window, cx);
//             editor
//         }
//     });

//     let weak_editor = editor.downgrade();
//     let theme_colors = cx.theme().colors();

//     div()
//         .child(editor)
//         .bg(theme_colors.editor_background)
//         .border_1()
//         .rounded_lg()
//         .border_color(theme_colors.border)
//         .on_action::<menu::Confirm>({
//             move |_, _, cx| {
//                 let Some(editor) = weak_editor.upgrade() else {
//                     return;
//                 };
//                 let new_value = editor.read_with(cx, |editor, cx| editor.text(cx));
//                 let new_value = (!new_value.is_empty()).then_some(new_value);
//                 write_setting_value(get_value, new_value, cx);
//                 editor.update(cx, |_, cx| {
//                     cx.notify();
//                 });
//             }
//         })
//         .into_any_element()
// }

// TODO: This should not be thinking about fallback logic
fn render_toggle_button(
    // id: &'static str,
    _: SettingsFile,
    field: SettingField<bool>,
    cx: &mut App,
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

    let toggle_state = if *(field.pick)(&user_settings) {
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
                        *(field.pick_mut)(settings) = state
                    });
                });
            }
        })
        .into_any_element()
}
