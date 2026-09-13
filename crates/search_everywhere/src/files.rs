use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, Entity, SharedString, Task};
use project::{Project, ProjectPath, WorktreeId};

use crate::SearchEverywhereDelegate;
use crate::providers::{SearchResult, SearchResultCategory};

pub struct FileProvider {
    project: Entity<Project>,
}

impl FileProvider {
    pub fn new(project: Entity<Project>, _cx: &App) -> Self {
        Self { project }
    }

    pub fn search(
        &self,
        query: &str,
        cx: &mut gpui::Context<picker::Picker<SearchEverywhereDelegate>>,
    ) -> Task<Vec<(SearchResult, StringMatch)>> {
        if query.is_empty() {
            return Task::ready(Vec::new());
        }

        let project = self.project.clone();
        let query = query.to_string();

        cx.spawn(async move |_, cx| {
            let Some(candidates) = cx
                .update(|cx| {
                    let project = project.read(cx);
                    let worktrees = project.visible_worktrees(cx).collect::<Vec<_>>();

                    let mut candidates = Vec::new();
                    for worktree in worktrees {
                        let worktree = worktree.read(cx);
                        let worktree_id = worktree.id();

                        for entry in worktree.files(false, 0) {
                            let path = entry.path.as_unix_str().to_string();
                            candidates.push(FileCandidate {
                                path: path.clone(),
                                worktree_id,
                                project_path: ProjectPath {
                                    worktree_id,
                                    path: entry.path.clone(),
                                },
                            });
                        }
                    }
                    candidates
                })
                .ok()
            else {
                return Vec::new();
            };

            let string_candidates: Vec<StringMatchCandidate> = candidates
                .iter()
                .enumerate()
                .map(|(id, c)| StringMatchCandidate::new(id, &c.path))
                .collect();

            let matches = fuzzy::match_strings(
                &string_candidates,
                &query,
                true,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            matches
                .into_iter()
                .filter_map(|m| {
                    let candidate = candidates.get(m.candidate_id)?;
                    let file_name = candidate
                        .project_path
                        .path
                        .file_name()
                        .map(|n| n.to_string())
                        .unwrap_or_else(|| candidate.path.clone());

                    let result = SearchResult {
                        label: SharedString::from(file_name),
                        detail: Some(SharedString::from(candidate.path.clone())),
                        category: SearchResultCategory::File,
                        path: Some(candidate.project_path.clone()),
                        action: None,
                        symbol: None,
                        document_symbol: None,
                    };

                    Some((result, m))
                })
                .collect()
        })
    }
}

struct FileCandidate {
    path: String,
    #[allow(dead_code)]
    worktree_id: WorktreeId,
    project_path: ProjectPath,
}
