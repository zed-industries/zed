pub mod assets;
pub mod file_finder;
pub mod language;
pub mod menus;
pub mod people_panel;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
pub mod theme_selector;

pub use buffer;
use buffer::LanguageRegistry;
use chat_panel::ChatPanel;
pub use client;
pub use editor;
use gpui::{
    action,
    geometry::{rect::RectF, vector::vec2f},
    keymap::Binding,
    platform::WindowOptions,
    ModelHandle, MutableAppContext, PathPromptOptions, Task, ViewContext,
};
use parking_lot::Mutex;
use people_panel::PeoplePanel;
use postage::watch;
pub use project::{self, fs};
use project_panel::ProjectPanel;
use std::{path::PathBuf, sync::Arc};
use theme::ThemeRegistry;
pub use workspace;
use workspace::{Settings, Workspace, WorkspaceParams};

action!(About);
action!(Open, Arc<AppState>);
action!(OpenPaths, OpenParams);
action!(Quit);
action!(AdjustBufferFontSize, f32);

const MIN_FONT_SIZE: f32 = 6.0;

pub struct AppState {
    pub settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
    pub settings: watch::Receiver<Settings>,
    pub languages: Arc<LanguageRegistry>,
    pub themes: Arc<ThemeRegistry>,
    pub client: Arc<client::Client>,
    pub user_store: ModelHandle<client::UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub channel_list: ModelHandle<client::ChannelList>,
}

#[derive(Clone)]
pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub app_state: Arc<AppState>,
}

pub fn init(app_state: &Arc<AppState>, cx: &mut gpui::MutableAppContext) {
    cx.add_global_action(open);
    cx.add_global_action(|action: &OpenPaths, cx: &mut MutableAppContext| {
        open_paths(action, cx).detach()
    });
    cx.add_global_action(open_new);
    cx.add_global_action(quit);

    cx.add_global_action({
        let settings_tx = app_state.settings_tx.clone();

        move |action: &AdjustBufferFontSize, cx| {
            let mut settings_tx = settings_tx.lock();
            let new_size = (settings_tx.borrow().buffer_font_size + action.0).max(MIN_FONT_SIZE);
            settings_tx.borrow_mut().buffer_font_size = new_size;
            cx.refresh_windows();
        }
    });

    cx.add_bindings(vec![
        Binding::new("cmd-=", AdjustBufferFontSize(1.), None),
        Binding::new("cmd--", AdjustBufferFontSize(-1.), None),
    ])
}

fn open(action: &Open, cx: &mut MutableAppContext) {
    let app_state = action.0.clone();
    cx.prompt_for_paths(
        PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        },
        move |paths, cx| {
            if let Some(paths) = paths {
                cx.dispatch_global_action(OpenPaths(OpenParams { paths, app_state }));
            }
        },
    );
}

fn open_paths(action: &OpenPaths, cx: &mut MutableAppContext) -> Task<()> {
    log::info!("open paths {:?}", action.0.paths);

    // Open paths in existing workspace if possible
    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(handle) = cx.root_view::<Workspace>(window_id) {
            let task = handle.update(cx, |view, cx| {
                if view.contains_paths(&action.0.paths, cx.as_ref()) {
                    log::info!("open paths on existing workspace");
                    Some(view.open_paths(&action.0.paths, cx))
                } else {
                    None
                }
            });

            if let Some(task) = task {
                return task;
            }
        }
    }

    log::info!("open new workspace");

    // Add a new workspace if necessary
    let app_state = &action.0.app_state;
    let (_, workspace) = cx.add_window(window_options(), |cx| {
        build_workspace(&WorkspaceParams::from(app_state.as_ref()), cx)
    });
    workspace.update(cx, |workspace, cx| {
        workspace.open_paths(&action.0.paths, cx)
    })
}

fn open_new(action: &workspace::OpenNew, cx: &mut MutableAppContext) {
    cx.add_window(window_options(), |cx| {
        let mut workspace = build_workspace(&action.0, cx);
        workspace.open_new_file(&action, cx);
        workspace
    });
}

fn build_workspace(params: &WorkspaceParams, cx: &mut ViewContext<Workspace>) -> Workspace {
    let mut workspace = Workspace::new(params, cx);
    let project = workspace.project().clone();
    workspace.left_sidebar_mut().add_item(
        "icons/folder-tree-16.svg",
        ProjectPanel::new(project, params.settings.clone(), cx).into(),
    );
    workspace.right_sidebar_mut().add_item(
        "icons/user-16.svg",
        cx.add_view(|cx| PeoplePanel::new(params.user_store.clone(), params.settings.clone(), cx))
            .into(),
    );
    workspace.right_sidebar_mut().add_item(
        "icons/comment-16.svg",
        cx.add_view(|cx| {
            ChatPanel::new(
                params.client.clone(),
                params.channel_list.clone(),
                params.settings.clone(),
                cx,
            )
        })
        .into(),
    );
    workspace
}

fn window_options() -> WindowOptions<'static> {
    WindowOptions {
        bounds: RectF::new(vec2f(0., 0.), vec2f(1024., 768.)),
        title: None,
        titlebar_appears_transparent: true,
        traffic_light_position: Some(vec2f(8., 8.)),
    }
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}

impl<'a> From<&'a AppState> for WorkspaceParams {
    fn from(state: &'a AppState) -> Self {
        Self {
            client: state.client.clone(),
            fs: state.fs.clone(),
            languages: state.languages.clone(),
            settings: state.settings.clone(),
            user_store: state.user_store.clone(),
            channel_list: state.channel_list.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use test::test_app_state;
    use theme::DEFAULT_THEME_NAME;
    use util::test::temp_tree;
    use workspace::ItemView;

    #[gpui::test]
    async fn test_open_paths_action(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        let dir = temp_tree(json!({
            "a": {
                "aa": null,
                "ab": null,
            },
            "b": {
                "ba": null,
                "bb": null,
            },
            "c": {
                "ca": null,
                "cb": null,
            },
        }));

        cx.update(|cx| {
            open_paths(
                &OpenPaths(OpenParams {
                    paths: vec![
                        dir.path().join("a").to_path_buf(),
                        dir.path().join("b").to_path_buf(),
                    ],
                    app_state: app_state.clone(),
                }),
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);

        cx.update(|cx| {
            open_paths(
                &OpenPaths(OpenParams {
                    paths: vec![dir.path().join("a").to_path_buf()],
                    app_state: app_state.clone(),
                }),
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);
        let workspace_1 = cx.root_view::<Workspace>(cx.window_ids()[0]).unwrap();
        workspace_1.read_with(&cx, |workspace, cx| {
            assert_eq!(workspace.worktrees(cx).len(), 2)
        });

        cx.update(|cx| {
            open_paths(
                &OpenPaths(OpenParams {
                    paths: vec![
                        dir.path().join("b").to_path_buf(),
                        dir.path().join("c").to_path_buf(),
                    ],
                    app_state: app_state.clone(),
                }),
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 2);
    }

    #[gpui::test]
    async fn test_new_empty_workspace(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        cx.update(|cx| init(&app_state, cx));
        cx.dispatch_global_action(workspace::OpenNew(app_state.as_ref().into()));
        let window_id = *cx.window_ids().first().unwrap();
        let workspace = cx.root_view::<Workspace>(window_id).unwrap();
        let editor = workspace.update(&mut cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<editor::Editor>()
                .unwrap()
        });

        editor.update(&mut cx, |editor, cx| {
            assert!(editor.text(cx).is_empty());
        });

        workspace.update(&mut cx, |workspace, cx| {
            workspace.save_active_item(&workspace::Save, cx)
        });

        app_state.fs.as_fake().insert_dir("/root").await.unwrap();
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name")));

        editor
            .condition(&cx, |editor, cx| editor.title(cx) == "the-new-name")
            .await;
        editor.update(&mut cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
        });
    }

    #[gpui::test]
    fn test_bundled_themes(cx: &mut MutableAppContext) {
        let app_state = test_app_state(cx);
        let mut has_default_theme = false;
        for theme_name in app_state.themes.list() {
            let theme = app_state.themes.get(&theme_name).unwrap();
            if theme.name == DEFAULT_THEME_NAME {
                has_default_theme = true;
            }
            assert_eq!(theme.name, theme_name);
        }
        assert!(has_default_theme);
    }
}
