//! # settings_ui
use std::fmt::Display;

use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    App, AppContext as _, Context, Div, IntoElement, ReadGlobal as _, Render, UpdateGlobal as _,
    Window, actions, div, px, size,
};
use settings::{SettingsContent, SettingsStore};
use ui::{
    ActiveTheme as _, Color, FluentBuilder as _, InteractiveElement as _, Label, LabelCommon as _,
    LabelSize, ParentElement, StatefulInteractiveElement as _, Styled as _, SwitchField,
    ToggleButton, v_flex,
};
use workspace::Workspace;

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

#[derive(PartialEq, Eq, Copy, Clone)]
enum SettingsPage {
    General,
    AppearanceAndBehavior,
    Editor,
    WorkbenchAndWindow,
    PanelsAndTools,
    AI,
    LanguageAndFrameworks,
    LanguageServer,
    VersionControl,
    Extensions,
    SystemAndNetwork,
    Collaboration,
}

static PAGES: &'static [SettingsPage] = &[
    SettingsPage::General,
    SettingsPage::AppearanceAndBehavior,
    SettingsPage::Editor,
    SettingsPage::WorkbenchAndWindow,
    SettingsPage::PanelsAndTools,
    SettingsPage::AI,
    SettingsPage::LanguageAndFrameworks,
    SettingsPage::LanguageServer,
    SettingsPage::VersionControl,
    SettingsPage::Extensions,
    SettingsPage::SystemAndNetwork,
    SettingsPage::Collaboration,
];

impl SettingsPage {
    fn title(&self) -> &'static str {
        match self {
            SettingsPage::General => "General",
            SettingsPage::AppearanceAndBehavior => "Appearance & Behavior",
            SettingsPage::Editor => "Editor",
            SettingsPage::WorkbenchAndWindow => "Workbench & Window",
            SettingsPage::PanelsAndTools => "Panels & Tools",
            SettingsPage::AI => "AI",
            SettingsPage::LanguageAndFrameworks => "Language & Frameworks",
            SettingsPage::LanguageServer => "Language Server",
            SettingsPage::VersionControl => "Version Control",
            SettingsPage::Extensions => "Extensions",
            SettingsPage::SystemAndNetwork => "System & Network",
            SettingsPage::Collaboration => "Collaboration",
        }
    }
}

impl Display for SettingsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = self.title();
        write!(f, "{title}")
    }
}

pub fn open_settings_editor(
    workspace: &mut Workspace,
    _: &OpenSettingsEditor,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    // todo(settings_ui) open in a local workspace if this is remote.
    cx.open_window(
        gpui::WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some("Zed Settings".into()),
                appears_transparent: true, // todo(settings_ui) will change in future, this looks cool for now
                ..Default::default()
            }),
            focus: true,
            show: true,
            kind: gpui::WindowKind::Normal,
            window_min_size: Some(size(px(300.), px(500.))), // todo(settings_ui): Does this min_size make sense?
            window_background: gpui::WindowBackgroundAppearance::Blurred, // todo(settings_ui) will change in future, this looks cool for now
            ..Default::default()
        },
        |window, cx| cx.new(|cx| SettingsWindow::new(window, cx)),
    )
    .ok();
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
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
                div.on_action(cx.listener(open_settings_editor))
            } else {
                div
            }
        });
    })
    .detach();
}

pub struct SettingsWindow {
    page: SettingsPage,
}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let page = SettingsPage::General;
        Self { page }
    }
}

fn render_nav(page: SettingsPage, _window: &mut Window, _cx: &mut Context<SettingsWindow>) -> Div {
    let mut nav = v_flex().p_4().gap_2();
    for &index in PAGES {
        nav = nav.child(
            div().id(index.title()).child(
                Label::new(index.title())
                    .size(LabelSize::Large)
                    .when(page == index, |this| this.color(Color::Selected)),
            ),
        );
    }
    nav
}

fn render_toggle_button(
    title: &'static str,
    description: &'static str,
    get_value: fn(&SettingsContent) -> &mut bool,
    cx: &mut App,
) -> impl IntoElement {
    // todo! in settings window state
    let store = SettingsStore::global(cx);
    let mut defaults = store
        .raw_user_settings()
        .map(|settings| &*settings.content)
        .unwrap_or_else(|| store.raw_default_settings())
        .clone();
    // todo! id?
    let toggle_state = if *get_value(&mut defaults) {
        ui::ToggleState::Selected
    } else {
        ui::ToggleState::Unselected
    };
    SwitchField::new(
        title,
        title,
        Some(description.into()),
        toggle_state,
        move |state, _, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    *get_value(settings) = *state == ui::ToggleState::Selected;
                });
            });
        },
    )
}

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().colors().background)
            .child(render_nav(self.page, window, cx))
    }
}
