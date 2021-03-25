pub mod pane;
pub mod pane_group;
pub mod workspace;
pub mod workspace_view;

pub use pane::*;
pub use pane_group::*;
pub use workspace::*;
pub use workspace_view::*;

use crate::{settings::Settings, watch};
use gpui::{App, MutableAppContext};
use std::path::PathBuf;

pub fn init(app: &mut App) {
    app.add_global_action("workspace:open_paths", open_paths);
    pane::init(app);
}

pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub settings: watch::Receiver<Settings>,
}

fn open_paths(params: &OpenParams, app: &mut MutableAppContext) {
    log::info!("open paths {:?}", params.paths);

    // Open paths in existing workspace if possible
    for window_id in app.window_ids().collect::<Vec<_>>() {
        if let Some(handle) = app.root_view::<WorkspaceView>(window_id) {
            if handle.update(app, |view, ctx| {
                if view.contains_paths(&params.paths, ctx.app()) {
                    view.open_paths(&params.paths, ctx.app_mut());
                    log::info!("open paths on existing workspace");
                    true
                } else {
                    false
                }
            }) {
                return;
            }
        }
    }

    log::info!("open new workspace");

    // Add a new workspace if necessary
    let workspace = app.add_model(|ctx| Workspace::new(params.paths.clone(), ctx));
    app.add_window(|ctx| WorkspaceView::new(workspace, params.settings.clone(), ctx));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{settings, test::*};
    use gpui::App;
    use serde_json::json;

    #[test]
    fn test_open_paths_action() {
        App::test((), |mut app| async move {
            let settings = settings::channel(&app.font_cache()).unwrap().1;

            init(&mut app);

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

            app.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths: vec![
                        dir.path().join("a").to_path_buf(),
                        dir.path().join("b").to_path_buf(),
                    ],
                    settings: settings.clone(),
                },
            );
            assert_eq!(app.window_ids().len(), 1);

            app.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths: vec![dir.path().join("a").to_path_buf()],
                    settings: settings.clone(),
                },
            );
            assert_eq!(app.window_ids().len(), 1);
            let workspace_view_1 = app.root_view::<WorkspaceView>(app.window_ids()[0]).unwrap();
            workspace_view_1.read(&app, |view, app| {
                assert_eq!(view.workspace.as_ref(app).worktrees().len(), 2);
            });

            app.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths: vec![
                        dir.path().join("b").to_path_buf(),
                        dir.path().join("c").to_path_buf(),
                    ],
                    settings: settings.clone(),
                },
            );
            assert_eq!(app.window_ids().len(), 2);
        });
    }
}
