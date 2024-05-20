use super::SlashCommand;
use fuzzy::PathMatch;
use gpui::{AppContext, Model, Task};
use project::{PathMatchCandidateSet, Project};
use std::sync::{atomic::AtomicBool, Arc};

pub(crate) struct FileSlashCommand {
    project: Model<Project>,
}

impl FileSlashCommand {
    pub fn new(project: Model<Project>) -> Self {
        Self { project }
    }

    fn search_paths(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> Task<Vec<PathMatch>> {
        let worktrees = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                PathMatchCandidateSet {
                    snapshot: worktree.snapshot(),
                    include_ignored: worktree
                        .root_entry()
                        .map_or(false, |entry| entry.is_ignored),
                    include_root_name,
                    directories_only: false,
                }
            })
            .collect::<Vec<_>>();

        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.as_str(),
                None,
                false,
                100,
                &cancellation_flag,
                executor,
            )
            .await
        })
    }
}

impl SlashCommand for FileSlashCommand {
    fn name(&self) -> String {
        "file".into()
    }

    fn description(&self) -> String {
        "insert an entire file".into()
    }

    fn complete_argument(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> gpui::Task<http::Result<Vec<String>>> {
        let paths = self.search_paths(query, cancellation_flag, cx);
        cx.background_executor().spawn(async move {
            Ok(paths
                .await
                .into_iter()
                .map(|path_match| {
                    format!(
                        "{}{}",
                        path_match.path_prefix,
                        path_match.path.to_string_lossy()
                    )
                })
                .collect())
        })
    }
}
