pub mod pane;
pub mod pane_group;
pub mod workspace;
pub mod workspace_view;

pub use pane::*;
pub use pane_group::*;
pub use workspace::*;
pub use workspace_view::*;

use crate::{
    settings::Settings,
    watch::{self, Receiver},
};
use gpui::{MutableAppContext, PathPromptOptions};
use std::path::PathBuf;

pub fn init(app: &mut MutableAppContext) {
    app.add_global_action("workspace:open", open);
    app.add_global_action("workspace:open_paths", open_paths);
    app.add_global_action("app:quit", quit);
    pane::init(app);
    workspace_view::init(app);
}

pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub settings: watch::Receiver<Settings>,
}

fn open(settings: &Receiver<Settings>, ctx: &mut MutableAppContext) {
    let settings = settings.clone();
    ctx.prompt_for_paths(
        PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        },
        move |paths, ctx| {
            if let Some(paths) = paths {
                ctx.dispatch_global_action("workspace:open_paths", OpenParams { paths, settings });
            }
        },
    );
}

fn open_paths(params: &OpenParams, app: &mut MutableAppContext) {
    log::info!("open paths {:?}", params.paths);

    // Open paths in existing workspace if possible
    for window_id in app.window_ids().collect::<Vec<_>>() {
        if let Some(handle) = app.root_view::<WorkspaceView>(window_id) {
            if handle.update(app, |view, ctx| {
                if view.contains_paths(&params.paths, ctx.as_ref()) {
                    let open_paths = view.open_paths(&params.paths, ctx);
                    ctx.foreground().spawn(open_paths).detach();
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
    let workspace = app.add_model(|ctx| Workspace::new(vec![], ctx));
    app.add_window(|ctx| {
        let view = WorkspaceView::new(workspace, params.settings.clone(), ctx);
        let open_paths = view.open_paths(&params.paths, ctx);
        ctx.foreground().spawn(open_paths).detach();
        view
    });
}

fn quit(_: &(), app: &mut MutableAppContext) {
    app.platform().quit();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{settings, test::*};
    use gpui::App;
    use serde_json::json;

    #[test]
    fn test_open_paths_action() {
        App::test((), |app| {
            let settings = settings::channel(&app.font_cache()).unwrap().1;

            init(app);

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
            assert_eq!(app.window_ids().count(), 1);

            app.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths: vec![dir.path().join("a").to_path_buf()],
                    settings: settings.clone(),
                },
            );
            assert_eq!(app.window_ids().count(), 1);
            let workspace_view_1 = app
                .root_view::<WorkspaceView>(app.window_ids().next().unwrap())
                .unwrap();
            assert_eq!(
                workspace_view_1
                    .read(app)
                    .workspace
                    .read(app)
                    .worktrees()
                    .len(),
                2
            );

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
            assert_eq!(app.window_ids().count(), 2);
        });
    }
}
