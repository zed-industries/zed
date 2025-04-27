use anyhow::{anyhow, Result};
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use util::paths::PathMatcher;
use worktree::Snapshot as WorktreeSnapshot;

use crate::{
    templates::{GlobTemplate, Template, Templates},
    AgentTool,
};

#[derive(Deserialize, JsonSchema)]
struct GlobInput {
    glob: SharedString,
}

struct GlobTool {
    project: Entity<Project>,
    templates: Arc<Templates>,
}

impl AgentTool for GlobTool {
    type Input = GlobInput;

    fn name(&self) -> SharedString {
        "glob".into()
    }

    fn description(&self, cx: &mut App) -> SharedString {
        let project_roots = self
            .project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).root_name().into())
            .collect::<Vec<String>>()
            .join("\n");

        GlobTemplate { project_roots }
            .render(&self.templates)
            .expect("template failed to render")
            .into()
    }

    fn run(self: Arc<Self>, input: Self::Input, cx: &mut App) -> Task<Result<String>> {
        let path_matcher = match PathMatcher::new([&input.glob]) {
            Ok(matcher) => matcher,
            Err(error) => return Task::ready(Err(anyhow!(error))),
        };

        let snapshots: Vec<WorktreeSnapshot> = self
            .project
            .read(cx)
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).snapshot())
            .collect();

        cx.background_spawn(async move {
            let paths = snapshots.iter().flat_map(|snapshot| {
                let root_name = PathBuf::from(snapshot.root_name());
                snapshot
                    .entries(false, 0)
                    .map(move |entry| root_name.join(&entry.path))
                    .filter(|path| path_matcher.is_match(&path))
            });
            let output = paths
                .map(|path| format!("{}\n", path.display()))
                .collect::<String>();
            Ok(output)
        })
    }
}
