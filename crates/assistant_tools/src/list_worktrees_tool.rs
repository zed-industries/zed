use std::sync::Arc;

use anyhow::Result;
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListWorktreesToolInput {}

pub struct ListWorktreesTool;

impl Tool for ListWorktreesTool {
    fn name(&self) -> String {
        "list-worktrees".into()
    }

    fn description(&self) -> String {
        "Lists all worktrees in the current project. Use this tool when you need to find available worktrees and their IDs.".into()
    }

    fn input_schema(&self) -> serde_json::Value {
        serde_json::json!(
            {
                "type": "object",
                "properties": {},
                "required": []
            }
        )
    }

    fn run(
        self: Arc<Self>,
        _input: serde_json::Value,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        cx.spawn(|cx| async move {
            cx.update(|cx| {
                #[derive(Debug, Serialize)]
                struct WorktreeInfo {
                    id: usize,
                    root_name: String,
                    root_dir: Option<String>,
                }

                let worktrees = project.update(cx, |project, cx| {
                    project
                        .visible_worktrees(cx)
                        .map(|worktree| {
                            worktree.read_with(cx, |worktree, _cx| WorktreeInfo {
                                id: worktree.id().to_usize(),
                                root_dir: worktree
                                    .root_dir()
                                    .map(|root_dir| root_dir.to_string_lossy().to_string()),
                                root_name: worktree.root_name().to_string(),
                            })
                        })
                        .collect::<Vec<_>>()
                });

                if worktrees.is_empty() {
                    return Ok("No worktrees found in the current project.".to_string());
                }

                let mut result = String::from("Worktrees in the current project:\n\n");
                for worktree in worktrees {
                    result.push_str(&serde_json::to_string(&worktree)?);
                }

                Ok(result)
            })?
        })
    }
}
