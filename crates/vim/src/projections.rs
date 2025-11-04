// Projections allow users to associate files within a project as projections of
// one another. Inspired by https://github.com/tpope/vim-projectionist .
//
// Take, for example, a newly generated Phoenix project. Among other files, one
// can find the page controller module and its corresponding test file in:
//
// - `lib/app_web/controllers/page_controller.ex`
// - `lib/app_web/controllers/page_controller_test.exs`
//
// From the point of view of the controller module, one can say that the test
// file is a projection of the controller module, and vice versa.
//
// TODO!:
// - [ ] Implement `:a` to open alternate file
// - [ ] Implement `:as` to open alternate file in split
// - [ ] Implement `:av` to open alternate file in vertical split
// - [ ] Implement actually updating the state from the `projections.json` file
// - [ ] Make this work with excerpts in multibuffers

use crate::Vim;
use editor::Editor;
use gpui::Context;
use gpui::Window;
use gpui::actions;
use project::ProjectItem;
use project::ProjectPath;
use util::rel_path::RelPath;

actions!(
    vim,
    [
        /// Opens a projection of the current file.
        OpenProjection,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, Vim::open_projection);
}

impl Vim {
    pub fn open_projection(
        &mut self,
        _: &OpenProjection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Implementation for opening a projection
        dbg!("[vim] attempting to open projection...");
        self.update_editor(cx, |_vim, editor, cx| {
            let project_path = editor
                .buffer()
                .read(cx)
                .as_singleton()
                .and_then(|buffer| buffer.read(cx).project_path(cx));

            // User is editing an empty buffer, can't even find a projection.
            if project_path.is_none() {
                return;
            }

            if let Some(project_path) = project_path
                && let Some(workspace) = editor.workspace()
            {
                dbg!(&project_path);
                if project_path.path.as_unix_str()
                    == "lib/phx_new_web/controllers/page_controller.ex"
                {
                    dbg!("[vim] opening projection...");
                    workspace
                        .update(cx, |workspace, cx| {
                            let worktree_id = project_path.worktree_id;
                            let mut project_path = ProjectPath::root_path(worktree_id);
                            project_path.path = RelPath::unix(
                                "test/phx_new_web/controllers/page_controller_test.exs",
                            )
                            .unwrap()
                            .into_arc();
                            dbg!(&project_path);

                            workspace.open_path(project_path, None, true, window, cx)
                        })
                        .detach();
                }
            }
        });
    }
}
