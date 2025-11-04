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
use regex::Regex;
use util::rel_path::RelPath;

#[derive(Debug)]
struct Projection {
    source: Regex,
    target: String,
}

impl Projection {
    fn new(source: &str, target: &str) -> Self {
        // Replace the `*` character in the source string, if such a character
        // is present, with a capture group, so we can then replace that value
        // when determining the target.
        // TODO!: Support for multiple `*` characters?
        // TODO!: Validation that the number of `{}` in the target matches the
        // number of `*` on the source.
        // TODO!: Avoid `unwrap` here by updating `new` to return
        // `Result<Self>`/`Option<Self>`.
        let source = Regex::new(&source.replace("*", "(.*)")).unwrap();
        let target = String::from(target);

        Self { source, target }
    }

    /// Determines whether the provided path matches this projection's source.
    /// TODO!: We'll likely want to update this to use `ProjectPath` instead of
    /// `&str`.
    fn matches(&self, path: &str) -> bool {
        self.source.is_match(path)
    }

    /// Returns the alternate path for the provided path.
    /// TODO!: Update to work with more than one capture group?
    fn alternate(&self, path: &str) -> String {
        // Determine the captures for the path.
        if let Some(capture) = self.source.captures_iter(path).next() {
            let (_, [name]) = capture.extract();
            self.target.replace("{}", name)
        } else {
            // TODO!: Can't find capture. Is this a regex without capture group?
            String::new()
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::Projection;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_matches(_cx: &mut TestAppContext) {
        let source = "lib/app/*.ex";
        let target = "test/app/{}_test.exs";
        let projection = Projection::new(source, target);

        let path = "lib/app/module.ex";
        assert_eq!(projection.matches(path), true);

        let path = "test/app/module_test.exs";
        assert_eq!(projection.matches(path), false);
    }

    #[gpui::test]
    async fn test_alternate(_cx: &mut TestAppContext) {
        let source = "lib/app/*.ex";
        let target = "test/app/{}_test.exs";
        let projection = Projection::new(source, target);

        let path = "lib/app/module.ex";
        assert_eq!(projection.alternate(path), "test/app/module_test.exs");
    }
}
