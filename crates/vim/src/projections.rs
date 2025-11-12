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
// - [X] Implement actually updating the state from the `projections.json` file
// - [ ] Make this work with excerpts in multibuffers

use crate::Vim;
use anyhow::Result;
use editor::Editor;
use gpui::Context;
use gpui::Window;
use gpui::actions;
use project::Fs;
use project::ProjectItem;
use project::ProjectPath;
use regex::Regex;
use serde::Deserialize;
use settings::parse_json_with_comments;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use util::rel_path::RelPath;

#[derive(Debug)]
struct Projection {
    source: Regex,
    target: String,
}

#[derive(Deserialize, Debug)]
struct ProjectionValue {
    alternate: String,
}

type ProjectionsConfig = HashMap<String, ProjectionValue>;

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

async fn load_projections(root_path: &Path, fs: Arc<dyn Fs>) -> Result<ProjectionsConfig> {
    let projections_path = root_path.join(".zed").join("projections.json");

    let content = fs.load(&projections_path).await?;
    let config = parse_json_with_comments::<ProjectionsConfig>(&content)?;

    Ok(config)
}

impl Vim {
    pub fn open_projection(
        &mut self,
        _: &OpenProjection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |_vim, editor, cx| {
            let current_file_path = editor
                .buffer()
                .read(cx)
                .as_singleton()
                .and_then(|buffer| buffer.read(cx).project_path(cx));

            // User is editing an empty buffer, can't find a projection.
            let Some(current_file_path) = current_file_path else {
                return;
            };

            let Some(workspace) = editor.workspace() else {
                return;
            };

            let Some(project) = editor.project() else {
                return;
            };

            // Extract data we need before going async
            let worktree_id = current_file_path.worktree_id;
            let current_path = current_file_path.path.clone();
            let fs = project.read(cx).fs().clone();

            // Get the worktree to extract its root path
            let worktree = project.read(cx).worktree_for_id(worktree_id, cx);

            let Some(worktree) = worktree else {
                return;
            };

            let root_path = worktree.read(cx).abs_path();

            workspace.update(cx, |_workspace, cx| {
                cx.spawn_in(window, async move |workspace, cx| {
                    // Load the projections configuration
                    let config = match load_projections(&root_path, fs).await {
                        Ok(config) => {
                            log::info!("Loaded projections config: {:?}", config);
                            config
                        }
                        Err(err) => {
                            log::warn!("Failed to load projections: {:?}", err);
                            return;
                        }
                    };

                    // Convert config to Projection instances and find a match
                    let current_path_str = current_path.as_unix_str();
                    log::info!("Looking for projection for path: {}", current_path_str);
                    let mut alternate_path: Option<String> = None;

                    for (source_pattern, projection_value) in config.iter() {
                        log::debug!(
                            "Trying pattern '{}' -> '{}'",
                            source_pattern,
                            projection_value.alternate
                        );
                        let projection =
                            Projection::new(source_pattern, &projection_value.alternate);

                        if projection.matches(current_path_str) {
                            let alt = projection.alternate(current_path_str);
                            log::info!("Found match! Alternate path: {}", alt);
                            alternate_path = Some(alt);
                            break;
                        }
                    }

                    // If we found an alternate, open it
                    if let Some(alternate_path) = alternate_path {
                        let alternate_rel_path = match RelPath::unix(&alternate_path) {
                            Ok(path) => path,
                            Err(_) => return,
                        };

                        let alternate_project_path = ProjectPath {
                            worktree_id,
                            path: alternate_rel_path.into_arc(),
                        };

                        let result = workspace.update_in(cx, |workspace, window, cx| {
                            workspace.open_path(alternate_project_path, None, true, window, cx)
                        });

                        match result {
                            Ok(task) => {
                                task.detach();
                            }
                            Err(err) => {
                                log::error!("Failed to open alternate file: {:?}", err);
                            }
                        }
                    } else {
                        log::info!("No alternate projection found for: {}", current_path_str);
                    }
                })
                .detach();
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::Projection;
    use super::load_projections;
    use gpui::TestAppContext;
    use project::FakeFs;
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

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

    #[gpui::test]
    async fn test_load_projections(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                ".zed": {
                    "projections.json": r#"{
                        "src/main/java/*.java": {"alternate": "src/test/java/{}.java"},
                        "src/test/java/*.java": {"alternate": "src/main/java/{}.java"}
                    }"#
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree = project.read_with(cx, |project, _cx| project.worktrees(_cx).next().unwrap());

        let root_path = worktree.read_with(cx, |wt, _| wt.abs_path());
        let config = load_projections(&root_path, fs).await.unwrap();

        assert_eq!(config.len(), 2);
        assert_eq!(
            config.get("src/main/java/*.java").unwrap().alternate,
            "src/test/java/{}.java"
        );
        assert_eq!(
            config.get("src/test/java/*.java").unwrap().alternate,
            "src/main/java/{}.java"
        );
    }
}
