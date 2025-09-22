//! # settings_ui
use std::{fmt::Display, sync::Arc};

use feature_flags::FeatureFlag;
use gpui::{
    App, AppContext as _, Context, IntoElement, ReadGlobal, Render, Window, WindowHandle, actions,
    div, px, size,
};
use settings::{
    CargoWorkspaceCommandSettings, SettingsContent, SettingsStore, SlashCommandSettings,
};
use ui::{BorrowAppContext, SwitchField};
// use settings::{SettingsContent, SettingsStore};
// use ui::{
//     ActiveTheme as _, Color, FluentBuilder as _, InteractiveElement as _, Label, LabelCommon as _,
//     LabelSize, ParentElement, StatefulInteractiveElement as _, Styled as _, SwitchField,
//     ToggleButton, v_flex,
// };
// use workspace::Workspace;

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

pub fn open_settings_editor(cx: &mut App) -> anyhow::Result<WindowHandle<SettingsWindow>> {
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
}

pub struct SettingsWindow {}

impl SettingsWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

// fn render_nav(page: SettingsPage, _window: &mut Window, _cx: &mut Context<SettingsWindow>) -> Div {
//     let mut nav = v_flex().p_4().gap_2();
//     for &index in PAGES {
//         nav = nav.child(
//             div().id(index.title()).child(
//                 Label::new(index.title())
//                     .size(LabelSize::Large)
//                     .when(page == index, |this| this.color(Color::Selected)),
//             ),
//         );
//     }
//     nav
// }

// fn render_page(page: SettingsPage, _window: &mut Window, cx: &mut Context<SettingsWindow>) -> Div {
//     let store = SettingsStore::global(cx);
//     let mut defaults = store
//         .raw_user_settings()
//         .map(|settings| &*settings.content)
//         .unwrap_or_else(|| store.raw_default_settings())
//         .clone();
//     div()
// }

// slash_commands.cargo_workspace.enabled Is a boolean, that shows up in multiple files.
//
// "slash_commands": {
//   "cargo_workspace": {
//     "enabled": true
//   }
// },
//
// This shows up in server, user, and project settings
// It can be overridden in different places,

fn render_toggle_button(
    title: &'static str,
    description: &'static str,
    // source: SettingsSource,
    cx: &mut App,
    get_value: fn(&mut SettingsContent) -> Option<&mut bool>,
) {
    // div()
    // .child(div().child(title).children(link))
    // .child(description
    // .child(settingUIStuffhere)
    // )
    
    // todo! in settings window state
    let store = SettingsStore::global(cx);

    // This clone needs to go!!
    let mut defaults = store.raw_default_settings().clone();

    // Hey settings store, here's my current file, get me the next correct value AND the file associated with that value

    // What do we render, when a setting has been modified in the user settings, but not project settings, and we are viewing project settings?
    //  - What VSCode does is render the _setting value's default_ (confusing!) and then adds a link to the user settings value
    // When a project setting has been modified, that takes precedence over the user setting

    let toggle_state = if *get_value(&mut store.raw_default_settings().clone())
        .unwrap_or_else(|| get_value(&mut defaults).unwrap())
    {
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
    );
}

// fn render_project_settings(cx: &mut App, get_value: fn(&mut SettingsContent) -> Option<&mut SlashCommandSettings>,
// ) {
//     // todo! in settings window state
//     let store = SettingsStore::global(cx);

//     // This clone needs to go!!
//     let mut defaults = store.raw_default_settings().clone();
//     render_slash_commands(defaults.project.get_or_insert_default())
// }

// fn render_slash_commands(
//     cx: &mut App,
//     get_value: fn(&mut SettingsContent) -> Option<&mut SlashCommandSettings>,
// ) {
//     // todo! in settings window state
//     let store = SettingsStore::global(cx);

//     // This clone needs to go!!
//     let mut defaults = store.raw_default_settings().clone();

//     let value = get_value(defaults).get_or_insert_default();
//     render_cargo_workspace(value)
// }

// fn render_cargo_workspace
//     cx: &mut App,
//     get_value: fn(&mut SlashCommandSettings) -> Option<&mut CargoWorkspaceCommandSettings>,
// ) {
//     // todo! in settings window state
//     let store = SettingsStore::global(cx);
//     // This clone needs to go!!
//     let mut defaults = store.raw_default_settings().clone();

//     let value = get_value(defaults);
//     render_toggle_button(title, description, source, cx, |content| {
//         get_value(content).enabled.as_mut()
//     });
//     // ^ we could just use the get_value passed in to get the parent, then get the child in the callback we pass to the ui component
// }
//
// ------------------------------------------------------------------------------------------

// This should hold the write(&self) method
// enum SettingsSource {
//     User,
//     Project(Arc<std::path::Path>),
// }

// Good exploration:
// All of this should go in settings.
// struct SettingsSources {
//     defaults: SettingsContent,
//     user: SettingsContent,
//     project: SettingsContent,
//     // server: SettingsContent
// }

// enum WhereWasSettingFound {
//     InRequestedSource,
//     In(SettingsSource),
// }

// fn read_setting_value<T>(
//     sources: &mut SettingsSources,
//     source: SettingsSource,
//     get_value: fn(&mut SettingsContent) -> Option<&mut T>,
// ) -> (Option<&mut T>, WhereWasSettingFound) {
//     match source {
//         SettingsSource::User => {
//             get_value(&mut sources.user).or_else(|| get_value(&mut sources.defaults))
//         }
//         SettingsSource::Project(path) => get_value(&mut sources.project)
//             .or_else(|| get_value(&mut sources.user))
//             .or_else(|| get_value(&mut sources.defaults)),
//     }
// }

// fn update_settings_source<T>(
//     source: SettingsSource,
//     update_fn: fn(&mut SettingsContent) -> &mut T,
//     value: T,
//     cx: &mut App,
// ) {
//     cx.update_global::<SettingsStore, _>(|store, cx| match source {
//         SettingsSource::User => store.update_user_settings(cx, |settings| {
//             *update_fn(settings) = value;
//         }),
//         SettingsSource::Project(path) => {
//             store.update_settings_file_at_path(cx, &[path], |settings| {
//                 *update_fn(settings) = value;
//             })
//         }
//     });
// }

impl Render for SettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
